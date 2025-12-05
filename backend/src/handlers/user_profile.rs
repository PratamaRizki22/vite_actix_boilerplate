use actix_web::{HttpResponse, Result, web, HttpRequest};
use sqlx::PgPool;
use totp_rs::{TOTP, Algorithm};
use base32;

use crate::middleware::auth::get_current_user;
use crate::models::user::{UpdateUser, User, UserResponse};

// Handler for users to update their own profile
pub async fn update_own_profile(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    user_data: web::Json<UpdateUser>,
) -> Result<HttpResponse> {
    let current_user = get_current_user(&req)
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Not authenticated"))?;

    let user_id = current_user.sub; // Use current user's ID from token

    // Update user with provided fields
    let mut has_updates = false;

    // Update username if provided
    if let Some(username) = &user_data.username {
        // Security check: If changing username, require 2FA
        let user = sqlx::query!(
            "SELECT totp_enabled, totp_secret, username FROM users WHERE id = $1",
            user_id
        )
        .fetch_one(pool.get_ref())
        .await
        .map_err(|_| actix_web::error::ErrorInternalServerError("Database error"))?;

        if !user.totp_enabled.unwrap_or(false) {
             return Ok(HttpResponse::BadRequest().json(serde_json::json!({
                "error": "Two-factor authentication (2FA) must be enabled to update your username. Please enable 2FA first."
            })));
        }

        match &user_data.verification_code {
            Some(code) => {
                 let totp_secret = user.totp_secret.as_ref().ok_or_else(|| {
                    actix_web::error::ErrorInternalServerError("TOTP secret missing")
                })?;

                let secret_bytes = base32::decode(base32::Alphabet::RFC4648 { padding: false }, totp_secret)
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
            },
            None => {
                return Ok(HttpResponse::BadRequest().json(serde_json::json!({
                    "error": "2FA verification required",
                    "require_2fa": true
                })));
            }
        }

        sqlx::query!(
            "UPDATE users SET username = $1, updated_at = NOW() WHERE id = $2",
            username,
            user_id
        )
        .execute(pool.get_ref())
        .await
        .map_err(|_| actix_web::error::ErrorInternalServerError("Database error"))?;
        has_updates = true;
    }

    // Update email if provided
    if let Some(email) = &user_data.email {
        // Security check: If changing email, require 2FA
        let user = sqlx::query!(
            "SELECT totp_enabled, totp_secret, username FROM users WHERE id = $1",
            user_id
        )
        .fetch_one(pool.get_ref())
        .await
        .map_err(|_| actix_web::error::ErrorInternalServerError("Database error"))?;

        if !user.totp_enabled.unwrap_or(false) {
             return Ok(HttpResponse::BadRequest().json(serde_json::json!({
                "error": "Two-factor authentication (2FA) must be enabled to update your email. Please enable 2FA first."
            })));
        }

        match &user_data.verification_code {
            Some(code) => {
                 let totp_secret = user.totp_secret.as_ref().ok_or_else(|| {
                    actix_web::error::ErrorInternalServerError("TOTP secret missing")
                })?;

                let secret_bytes = base32::decode(base32::Alphabet::RFC4648 { padding: false }, totp_secret)
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
            },
            None => {
                return Ok(HttpResponse::BadRequest().json(serde_json::json!({
                    "error": "2FA verification required",
                    "require_2fa": true
                })));
            }
        }

        sqlx::query!(
            "UPDATE users SET email = $1, email_verified = false, updated_at = NOW() WHERE id = $2",
            email,
            user_id
        )
        .execute(pool.get_ref())
        .await
        .map_err(|_| actix_web::error::ErrorInternalServerError("Database error"))?;
        has_updates = true;
    }

    // Update wallet_address if provided
    if let Some(wallet_address) = &user_data.wallet_address {
        sqlx::query!(
            "UPDATE users SET wallet_address = $1, updated_at = NOW() WHERE id = $2",
            wallet_address.as_deref(),
            user_id
        )
        .execute(pool.get_ref())
        .await
        .map_err(|_| actix_web::error::ErrorInternalServerError("Database error"))?;
        has_updates = true;
    }

    // Users cannot update their own role or password through this endpoint
    // Password changes should go through /api/auth/password/change
    // Role changes require admin

    if !has_updates {
        return Ok(HttpResponse::BadRequest().json(serde_json::json!({
            "error": "No fields to update"
        })));
    }

    // Get updated user
    let user = sqlx::query_as!(
        User,
        "SELECT id, username, email, password, role, wallet_address, email_verified, totp_enabled, recovery_codes, created_at, updated_at
         FROM users WHERE id = $1",
        user_id
    )
    .fetch_optional(pool.get_ref())
    .await
    .map_err(|_| actix_web::error::ErrorInternalServerError("Database error"))?;

    match user {
        Some(user) => Ok(HttpResponse::Ok().json(UserResponse::from(user))),
        None => Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": "User not found"
        }))),
    }
}
