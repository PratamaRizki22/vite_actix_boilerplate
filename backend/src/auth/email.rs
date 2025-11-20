use crate::services::email_service::EmailService;
use crate::middleware::rate_limiter::RateLimiter;
use actix_web::{HttpRequest, HttpResponse, Result, web};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Deserialize)]
pub struct SendVerificationRequest {
    pub email: String,
}

#[derive(Serialize)]
pub struct SendVerificationResponse {
    pub message: String,
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

    // Store code
    EmailService::store_verification_code(&req.email, &code);

    // Send email
    match client.send_verification_email(&req.email, &code).await {
        Ok(_) => Ok(HttpResponse::Ok().json(SendVerificationResponse {
            message: "Verification email sent successfully".to_string(),
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
    println!("Verify email request - Email: {}, Code: {}", req.email, req.code);
    
    let is_valid = EmailService::verify_code(&req.email, &req.code);
    
    println!("Verification result - Valid: {}", is_valid);

    if is_valid {
        // Update email_verified status in database
        sqlx::query!(
            "UPDATE users SET email_verified = true WHERE email = $1",
            req.email
        )
        .execute(pool.get_ref())
        .await
        .map_err(|_| actix_web::error::ErrorInternalServerError("Database update failed"))?;

        println!("Email verified successfully for: {}", req.email);
        Ok(HttpResponse::Ok().json(VerifyEmailResponse {
            message: "Email verified successfully".to_string(),
            verified: true,
        }))
    } else {
        println!("Email verification failed for: {} with code: {}", req.email, req.code);
        Ok(HttpResponse::BadRequest().json(VerifyEmailResponse {
            message: "Invalid or expired verification code".to_string(),
            verified: false,
        }))
    }
}

// Temporary debug endpoint to see stored verification codes
#[derive(Serialize)]
pub struct DebugCodesResponse {
    pub codes: std::collections::HashMap<String, String>,
}

pub async fn debug_codes() -> Result<HttpResponse> {
    let codes = EmailService::get_debug_codes();
    Ok(HttpResponse::Ok().json(DebugCodesResponse { codes }))
}
