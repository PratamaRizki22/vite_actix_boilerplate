use crate::services::email_service::EmailService;
use crate::middleware::rate_limiter::RateLimiter;
use actix_web::{HttpRequest, HttpResponse, Result, web};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use chrono::{Utc, Duration};

#[derive(Deserialize)]
pub struct SendVerificationRequest {
    pub email: String,
}

#[derive(Serialize)]
pub struct SendVerificationResponse {
    pub message: String,
    pub resend_cooldown_seconds: i32,
}

#[derive(Deserialize)]
pub struct VerifyEmailRequest {
    pub email: String,
    pub code: String,
}

#[derive(Serialize)]
pub struct VerifyEmailResponse {
    pub message: String,
    pub verified: bool,
}

pub async fn send_verification(
    req_http: HttpRequest,
    req: web::Json<SendVerificationRequest>,
) -> Result<HttpResponse> {
    let email = req.email.to_lowercase().trim().to_string();
    println!("Send verification request - Original email: {}, Normalized email: {}", req.email, email);
    
    // Rate limiting: 5 attempts per hour
    let (is_allowed, _, reset_seconds) = 
        RateLimiter::check_limit(&req_http, "send_verification", 5, 60);

    if !is_allowed {
        return Ok(HttpResponse::TooManyRequests().json(serde_json::json!({
            "error": "Too many verification email requests. Please try again later.",
            "retry_after": reset_seconds
        })));
    }

    let client = match EmailService::new() {
        Ok(client) => client,
        Err(e) => {
            return Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": format!("Email service not configured: {}", e)
            })));
        }
    };

    // Generate verification code
    let code = EmailService::generate_verification_code();

    // Store code with normalized email
    EmailService::store_verification_code(&email, &code);

    // Send email
    match client.send_verification_email(&email, &code).await {
        Ok(_) => Ok(HttpResponse::Ok().json(SendVerificationResponse {
            message: "Verification email sent successfully".to_string(),
            resend_cooldown_seconds: 60,
        })),
        Err(e) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({
            "error": format!("Failed to send email: {}", e)
        }))),
    }
}

pub async fn verify_email(
    pool: web::Data<PgPool>,
    req: web::Json<VerifyEmailRequest>,
) -> Result<HttpResponse> {
    let email = req.email.to_lowercase().trim().to_string();
    println!("Verify email request - Email: {}, Code: {}", email, req.code);
    
    let is_valid = EmailService::verify_code(&email, &req.code);
    
    println!("Verification result - Valid: {}", is_valid);
    println!("Available codes: {:?}", EmailService::get_debug_codes());

    if is_valid {
        // Update email_verified status in database
        sqlx::query!(
            "UPDATE users SET email_verified = true WHERE LOWER(TRIM(email)) = $1",
            &email
        )
        .execute(pool.get_ref())
        .await
        .map_err(|_| actix_web::error::ErrorInternalServerError("Database update failed"))?;

        println!("Email verified successfully for: {}", email);
        Ok(HttpResponse::Ok().json(VerifyEmailResponse {
            message: "Email verified successfully".to_string(),
            verified: true,
        }))
    } else {
        println!("Email verification failed for: {} with code: {}", email, req.code);
        Ok(HttpResponse::BadRequest().json(VerifyEmailResponse {
            message: "Invalid or expired verification code".to_string(),
            verified: false,
        }))
    }
}

#[derive(Serialize)]
pub struct DebugCodesResponse {
    pub codes: std::collections::HashMap<String, String>,
}

pub async fn debug_codes() -> Result<HttpResponse> {
    let codes = EmailService::get_debug_codes();
    Ok(HttpResponse::Ok().json(DebugCodesResponse { codes }))
}

#[derive(Deserialize)]
pub struct CheckExpiryRequest {
    pub email: String,
}

#[derive(Serialize)]
pub struct CheckExpiryResponse {
    pub has_code: bool,
    pub expires_in_seconds: Option<i64>,
    pub message: String,
}

pub async fn check_code_expiry(
    req: web::Json<CheckExpiryRequest>,
) -> Result<HttpResponse> {
    let email = req.email.to_lowercase().trim().to_string();
    let codes = EmailService::get_all_codes_with_expiry();
    
    if let Some((_, expires_at)) = codes.get(&email) {
        let duration = *expires_at - Utc::now();
        let remaining_seconds = duration.num_seconds().max(0);
        Ok(HttpResponse::Ok().json(CheckExpiryResponse {
            has_code: true,
            expires_in_seconds: Some(remaining_seconds),
            message: "Code found".to_string(),
        }))
    } else {
        Ok(HttpResponse::Ok().json(CheckExpiryResponse {
            has_code: false,
            expires_in_seconds: None,
            message: "No active code found for this email".to_string(),
        }))
    }
}

#[derive(Deserialize)]
pub struct SendMfaCodeRequest {
    pub temp_token: String,
}

#[derive(Serialize)]
pub struct SendMfaCodeResponse {
    pub message: String,
    pub resend_cooldown_seconds: i32,
}

pub async fn send_mfa_code(
    req_http: HttpRequest,
    req: web::Json<SendMfaCodeRequest>,
    jwt_secret: web::Data<String>,
) -> Result<HttpResponse> {
    // Verify temp MFA token
    let mfa_claims = crate::services::mfa_service::MFAService::verify_mfa_token(&req.temp_token, jwt_secret.get_ref())
        .map_err(|_| actix_web::error::ErrorUnauthorized("Invalid or expired MFA token"))?;

    // Get email from claims
    let email = mfa_claims.email
        .ok_or_else(|| actix_web::error::ErrorBadRequest("No email associated with this account"))?;

    println!("Send MFA code request - User ID: {}, Email: {}", mfa_claims.user_id, email);
    
    // Rate limiting: 5 attempts per hour per user
    let rate_limit_key = format!("send_mfa_code_{}", mfa_claims.user_id);
    let (is_allowed, _, reset_seconds) = 
        RateLimiter::check_limit_with_key(&rate_limit_key, 5, 60);

    if !is_allowed {
        return Ok(HttpResponse::TooManyRequests().json(serde_json::json!({
            "error": "Too many MFA code requests. Please try again later.",
            "retry_after": reset_seconds
        })));
    }

    let client = match EmailService::new() {
        Ok(client) => client,
        Err(e) => {
            return Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": format!("Email service not configured: {}", e)
            })));
        }
    };

    // Generate verification code
    let code = EmailService::generate_verification_code();

    // Store code for MFA (indexed by user_id, not email)
    EmailService::store_mfa_verification_code(mfa_claims.user_id, &code);

    // Send email
    match client.send_verification_email(&email, &code).await {
        Ok(_) => {
            println!("MFA code sent successfully to user {}", mfa_claims.user_id);
            Ok(HttpResponse::Ok().json(SendMfaCodeResponse {
                message: "MFA code sent successfully".to_string(),
                resend_cooldown_seconds: 60,
            }))
        },
        Err(e) => {
            eprintln!("Failed to send MFA code: {}", e);
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": format!("Failed to send email: {}", e)
            })))
        },
    }
}

#[derive(Deserialize)]
pub struct CheckMfaExpiryRequest {
    pub temp_token: String,
}

pub async fn check_mfa_code_expiry(
    req: web::Json<CheckMfaExpiryRequest>,
    jwt_secret: web::Data<String>,
) -> Result<HttpResponse> {
    // Verify temp MFA token
    let mfa_claims = crate::services::mfa_service::MFAService::verify_mfa_token(&req.temp_token, jwt_secret.get_ref())
        .map_err(|_| actix_web::error::ErrorUnauthorized("Invalid or expired MFA token"))?;

    // Check if MFA code exists for this user
    let code = EmailService::get_mfa_verification_code(mfa_claims.user_id);
    
    if code.is_some() {
        // Code exists, calculate expiry (MFA codes expire in 5 minutes)
        // We need to get the actual expiry time from the store
        let codes = crate::services::email_service::EmailService::get_all_mfa_codes_with_expiry();
        if let Some((_, expires_at)) = codes.get(&mfa_claims.user_id) {
            let duration = *expires_at - Utc::now();
            let remaining_seconds = duration.num_seconds().max(0);
            Ok(HttpResponse::Ok().json(CheckExpiryResponse {
                has_code: true,
                expires_in_seconds: Some(remaining_seconds),
                message: "MFA code found".to_string(),
            }))
        } else {
            Ok(HttpResponse::Ok().json(CheckExpiryResponse {
                has_code: false,
                expires_in_seconds: None,
                message: "No active MFA code found".to_string(),
            }))
        }
    } else {
        Ok(HttpResponse::Ok().json(CheckExpiryResponse {
            has_code: false,
            expires_in_seconds: None,
            message: "No active MFA code found".to_string(),
        }))
    }
}
