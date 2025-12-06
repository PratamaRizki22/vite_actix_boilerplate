use actix_web::{HttpRequest, HttpResponse, Result, web};
use serde_json;
use sqlx::PgPool;

use crate::middleware::rate_limiter::RateLimiter;
use crate::models::auth::{PasswordResetRequest, PasswordResetConfirm, PasswordResetResponse};
use crate::models::user::User;
use crate::services::email_service::EmailService;
use crate::services::audit_logger::AuditLogger;
use crate::utils::auth::AuthUtils;
use totp_rs::{TOTP, Algorithm};
use base32;

pub async fn request_password_reset(
    pool: web::Data<PgPool>,
    req_http: HttpRequest,
    reset_data: web::Json<PasswordResetRequest>,
) -> Result<HttpResponse> {
    // Rate limiting: 3 attempts per hour per IP
    let (is_allowed, _, reset_seconds) = 
        RateLimiter::check_limit(&req_http, "password_reset_request", 3, 60);

    if !is_allowed {
        return Ok(HttpResponse::TooManyRequests().json(PasswordResetResponse {
            success: false,
            message: format!("Too many password reset requests. Please try again in {} seconds.", reset_seconds),
        }));
    }

    // Check if user exists with this email
    let user = sqlx::query_as!(
        User,
        "SELECT id, username, email, password, role, wallet_address, email_verified, totp_enabled, recovery_codes, is_banned, banned_until, last_login, created_at, updated_at
         FROM users WHERE email = $1",
        reset_data.email
    )
    .fetch_optional(pool.get_ref())
    .await
    .map_err(|_| actix_web::error::ErrorInternalServerError("Database error"))?;

    // If user doesn't exist, return success anyway (don't leak user existence)
    if user.is_none() {
        return Ok(HttpResponse::Ok().json(PasswordResetResponse {
            success: true,
            message: "If an account with this email exists, a password reset link has been sent.".to_string(),
        }));
    }

    let user = user.unwrap();

    // Check if user is a traditional user (not Web3 only)
    if user.password == "web3_auth" {
        return Ok(HttpResponse::Ok().json(PasswordResetResponse {
            success: true,
            message: "If an account with this email exists, a password reset link has been sent.".to_string(),
        }));
    }

    // Generate reset token
    let reset_token = EmailService::generate_password_reset_token();
    EmailService::store_password_reset_token(&reset_data.email, &reset_token);

    // Send password reset email
    let email_client = match EmailService::new() {
        Ok(client) => {
            println!("Email client initialized successfully for password reset");
            Some(client)
        },
        Err(e) => {
            println!("Failed to initialize email client for password reset: {}", e);
            eprintln!("Failed to initialize email client: {}", e);
            None
        }
    };

    if let Some(client) = email_client {
        println!("Attempting to send password reset email to: {}", reset_data.email);
        if let Err(e) = client.send_password_reset_email(&reset_data.email, &reset_token).await {
            println!("Failed to send password reset email: {}", e);
            eprintln!("Failed to send password reset email: {}", e);
            // Don't fail the request if email fails
        } else {
            println!("Password reset email sent successfully to: {}", reset_data.email);
        }
    } else {
        println!("Email client not available, skipping password reset email");
    }

    Ok(HttpResponse::Ok().json(PasswordResetResponse {
        success: true,
        message: "If an account with this email exists, a password reset link has been sent.".to_string(),
    }))
}

pub async fn reset_password(
    pool: web::Data<PgPool>,
    reset_data: web::Json<PasswordResetConfirm>,
) -> Result<HttpResponse> {
    // Verify the reset token and get the associated email
    let email = match EmailService::verify_password_reset_token(&reset_data.token) {
        Some(email) => email,
        None => {
            return Ok(HttpResponse::BadRequest().json(PasswordResetResponse {
                success: false,
                message: "Invalid or expired reset token".to_string(),
            }));
        }
    };

    // Find user by email
    let user = sqlx::query_as!(
        User,
        "SELECT id, username, email, password, role, wallet_address, email_verified, totp_enabled, recovery_codes, is_banned, banned_until, last_login, created_at, updated_at
         FROM users WHERE email = $1",
        email
    )
    .fetch_optional(pool.get_ref())
    .await
    .map_err(|_| actix_web::error::ErrorInternalServerError("Database error"))?;

    let user = match user {
        Some(user) => user,
        None => {
            return Ok(HttpResponse::BadRequest().json(PasswordResetResponse {
                success: false,
                message: "User not found".to_string(),
            }));
        }
    };

    // Hash the new password
    let hashed_password = AuthUtils::hash_password(&reset_data.new_password)
        .map_err(|_| actix_web::error::ErrorInternalServerError("Password hashing failed"))?;

    // Update password in database
    sqlx::query!(
        "UPDATE users SET password = $1 WHERE id = $2",
        hashed_password,
        user.id
    )
    .execute(pool.get_ref())
    .await
    .map_err(|_| actix_web::error::ErrorInternalServerError("Failed to update password"))?;

    // Log password reset
    let _ = AuditLogger::log_password_reset(
        pool.get_ref(),
        user.id,
        None,
        None,
    ).await;

    Ok(HttpResponse::Ok().json(PasswordResetResponse {
        success: true,
        message: "Password has been reset successfully".to_string(),
    }))
}

pub async fn get_rate_limit_stats() -> Result<HttpResponse> {
    let stats = RateLimiter::get_stats();
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "rate_limits": stats
    })))
}

pub async fn debug_password_reset_tokens() -> Result<HttpResponse> {
    let tokens = EmailService::get_debug_password_reset_tokens();
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "tokens": tokens
    })))
}

pub async fn test_email_service() -> Result<HttpResponse> {
    match EmailService::new() {
        Ok(_) => Ok(HttpResponse::Ok().json(serde_json::json!({
            "status": "Email service initialized successfully"
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({
            "status": "Email service failed",
            "error": format!("{}", e)
        })))
    }
}

pub async fn verify_password_change_code(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    verify_data: web::Json<crate::models::auth::VerifyCodeRequest>,
) -> Result<HttpResponse> {
    // Get current user from JWT
    let current_user = crate::middleware::auth::get_current_user(&req)
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Not authenticated"))?;

    // Get user basic info including TOTP secret
    let user = sqlx::query!(
        "SELECT id, email, password, totp_enabled, totp_secret, username FROM users WHERE id = $1",
        current_user.sub
    )
    .fetch_optional(pool.get_ref())
    .await
    .map_err(|_| actix_web::error::ErrorInternalServerError("Database error"))?
    .ok_or_else(|| actix_web::error::ErrorNotFound("User not found"))?;

    // Verify code based on method
    println!("Verify code request - Method: {}, Code: {}", verify_data.verification_method, verify_data.verification_code);
    match verify_data.verification_method.as_str() {
        "totp" => {
            if !user.totp_enabled.unwrap_or(false) {
                 return Ok(HttpResponse::BadRequest().json(serde_json::json!({
                    "error": "2FA (TOTP) is not enabled for this account"
                })));
            }
            
            let totp_secret = user.totp_secret.ok_or_else(|| {
                actix_web::error::ErrorInternalServerError("TOTP secret missing")
            })?;

            let secret_bytes = base32::decode(base32::Alphabet::RFC4648 { padding: false }, &totp_secret)
                .ok_or_else(|| actix_web::error::ErrorInternalServerError("Invalid TOTP secret format"))?;

            let totp = TOTP::new(
                Algorithm::SHA1,
                6,
                1,
                30,
                secret_bytes,
                Some("USH".to_string()),
                user.username.clone(),
            ).map_err(|_| actix_web::error::ErrorInternalServerError("Failed to create TOTP instance"))?;

            let current_time = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs();

            if !totp.check(&verify_data.verification_code, current_time) {
                return Ok(HttpResponse::BadRequest().json(serde_json::json!({
                    "error": "Invalid 2FA code"
                })));
            }
        }
        "email" => {
             let email = user.email.clone().ok_or_else(|| {
                 actix_web::error::ErrorBadRequest("User does not have an email address")
             })?;
             
             // Normalize email to match how it's stored
             let normalized_email = email.to_lowercase().trim().to_string();
             println!("Verifying email code for: {}", normalized_email);

             let is_valid = EmailService::verify_code(&normalized_email, &verify_data.verification_code);
             println!("Verification result for {}: {}", normalized_email, is_valid);
             
             if !is_valid {
                 return Ok(HttpResponse::BadRequest().json(serde_json::json!({
                    "error": "Invalid or expired email verification code"
                })));
             }
        }
        _ => {
             return Ok(HttpResponse::BadRequest().json(serde_json::json!({
                "error": "Invalid verification method. Use 'totp' or 'email'."
            })));
        }
    }

    // Generate temp token
    let temp_token = EmailService::generate_password_reset_token();
    let email = user.email.ok_or_else(|| actix_web::error::ErrorBadRequest("User email required"))?;
    EmailService::store_password_reset_token(&email, &temp_token);

    Ok(HttpResponse::Ok().json(crate::models::auth::VerifyCodeResponse {
        success: true,
        temp_token,
        message: "Code verified successfully".to_string(),
    }))
}

pub async fn change_password(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    change_data: web::Json<crate::models::auth::ChangePasswordRequest>,
) -> Result<HttpResponse> {
    // Get current user from JWT
    let current_user = crate::middleware::auth::get_current_user(&req)
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Not authenticated"))?;

    // If temp_token is provided, verify it
    if let Some(token) = &change_data.temp_token {
        let email = EmailService::verify_password_reset_token(token);
        
        if email.is_none() {
             return Ok(HttpResponse::BadRequest().json(serde_json::json!({
                "error": "Invalid or expired verification token. Please verify your code again."
            })));
        }
        
        // Verify that the token belongs to the current user
        // We need to fetch the user's email to compare
        let user_email = sqlx::query!(
            "SELECT email FROM users WHERE id = $1",
            current_user.sub
        )
        .fetch_one(pool.get_ref())
        .await
        .map_err(|_| actix_web::error::ErrorInternalServerError("Database error"))?
        .email;

        if user_email != email {
             return Ok(HttpResponse::BadRequest().json(serde_json::json!({
                "error": "Token mismatch"
            })));
        }
    } else if let (Some(code), Some(method)) = (&change_data.verification_code, &change_data.verification_method) {
        // Legacy/Single-step flow
        // Get user basic info including TOTP secret
        let user = sqlx::query!(
            "SELECT id, email, password, totp_enabled, totp_secret, username FROM users WHERE id = $1",
            current_user.sub
        )
        .fetch_optional(pool.get_ref())
        .await
        .map_err(|_| actix_web::error::ErrorInternalServerError("Database error"))?
        .ok_or_else(|| actix_web::error::ErrorNotFound("User not found"))?;

        // Verify code based on method
        println!("Change password request - Method: {}, Code: {}", method, code);
        match method.as_str() {
            "totp" => {
                if !user.totp_enabled.unwrap_or(false) {
                     return Ok(HttpResponse::BadRequest().json(serde_json::json!({
                        "error": "2FA (TOTP) is not enabled for this account"
                    })));
                }
                
                let totp_secret = user.totp_secret.ok_or_else(|| {
                    actix_web::error::ErrorInternalServerError("TOTP secret missing")
                })?;

                let secret_bytes = base32::decode(base32::Alphabet::RFC4648 { padding: false }, &totp_secret)
                    .ok_or_else(|| actix_web::error::ErrorInternalServerError("Invalid TOTP secret format"))?;

                let totp = TOTP::new(
                    Algorithm::SHA1,
                    6,
                    1,
                    30,
                    secret_bytes,
                    Some("USH".to_string()),
                    user.username.clone(),
                ).map_err(|_| actix_web::error::ErrorInternalServerError("Failed to create TOTP instance"))?;

                let current_time = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs();

                if !totp.check(code, current_time) {
                    return Ok(HttpResponse::BadRequest().json(serde_json::json!({
                        "error": "Invalid 2FA code"
                    })));
                }
            }
            "email" => {
                 let email = user.email.ok_or_else(|| {
                     actix_web::error::ErrorBadRequest("User does not have an email address")
                 })?;
                 
                 // Normalize email to match how it's stored
                 let normalized_email = email.to_lowercase().trim().to_string();
                 println!("Verifying email code for: {}", normalized_email);

                 let is_valid = EmailService::verify_code(&normalized_email, code);
                 println!("Verification result for {}: {}", normalized_email, is_valid);
                 
                 if !is_valid {
                     return Ok(HttpResponse::BadRequest().json(serde_json::json!({
                        "error": "Invalid or expired email verification code"
                    })));
                 }
            }
            _ => {
                 return Ok(HttpResponse::BadRequest().json(serde_json::json!({
                    "error": "Invalid verification method. Use 'totp' or 'email'."
                })));
            }
        }
    } else {
        return Ok(HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Missing verification credentials"
        })));
    }

    // Validate new password
    if let Err(e) = crate::utils::validation::validate_password(&change_data.new_password) {
        return Ok(HttpResponse::BadRequest().json(serde_json::json!({
            "error": format!("Invalid password: {}", e)
        })));
    }

    // Hash new password
    let hashed_password = AuthUtils::hash_password(&change_data.new_password)
        .map_err(|_| actix_web::error::ErrorInternalServerError("Password hashing failed"))?;

    // Update password in database
    sqlx::query!(
        "UPDATE users SET password = $1, updated_at = NOW() WHERE id = $2",
        hashed_password,
        current_user.sub
    )
    .execute(pool.get_ref())
    .await
    .map_err(|_| actix_web::error::ErrorInternalServerError("Failed to update password"))?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "message": "Password changed successfully"
    })))
}