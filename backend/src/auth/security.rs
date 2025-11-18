use actix_web::{HttpRequest, HttpResponse, Result, web};
use sqlx::PgPool;

use crate::middleware::auth::get_current_user;
use crate::models::auth::{TOTPSetupResponse, TOTPVerifyRequest, TOTPVerifyResponse};

pub async fn setup_2fa(pool: web::Data<PgPool>, req: HttpRequest) -> Result<HttpResponse> {
    let current_user = get_current_user(&req)
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Not authenticated"))?;

    // Generate random secret
    use rand::Rng;
    let mut secret_bytes = [0u8; 20];
    rand::thread_rng().fill(&mut secret_bytes);
    let secret_bytes_vec = secret_bytes.to_vec();

    let secret_base32 = base32::encode(
        base32::Alphabet::RFC4648 { padding: false },
        &secret_bytes_vec,
    );

    // Save secret to database
    sqlx::query!(
        "UPDATE users SET totp_secret = $1 WHERE id = $2",
        secret_base32,
        current_user.sub
    )
    .execute(pool.get_ref())
    .await
    .map_err(|_| actix_web::error::ErrorInternalServerError("Failed to save 2FA secret"))?;

    // Generate QR code URL manually
    let qr_code_url = format!(
        "otpauth://totp/MyApp:{}?secret={}&issuer=MyApp&algorithm=SHA1&digits=6&period=30",
        current_user.username, secret_base32
    );

    let response = TOTPSetupResponse {
        secret: secret_base32,
        qr_code_url,
    };

    Ok(HttpResponse::Ok().json(response))
}

pub async fn verify_2fa(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    verify_data: web::Json<TOTPVerifyRequest>,
) -> Result<HttpResponse> {
    println!("Verify 2FA called with code: {}", verify_data.code);

    let current_user = get_current_user(&req)
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Not authenticated"))?;

    println!("User authenticated: {}", current_user.username);

    // Get secret from database
    let user_record = sqlx::query!(
        "SELECT totp_secret FROM users WHERE id = $1",
        current_user.sub
    )
    .fetch_one(pool.get_ref())
    .await
    .map_err(|_| actix_web::error::ErrorInternalServerError("Failed to get user data"))?;

    let totp_secret = user_record
        .totp_secret
        .ok_or_else(|| actix_web::error::ErrorBadRequest("2FA not set up for this user"))?;

    // Decode base32 secret
    let secret_bytes = base32::decode(base32::Alphabet::RFC4648 { padding: false }, &totp_secret)
        .ok_or_else(|| {
        actix_web::error::ErrorInternalServerError("Invalid TOTP secret format")
    })?;

    // Create TOTP instance
    let totp = totp_rs::TOTP::new_unchecked(
        totp_rs::Algorithm::SHA1,
        6,
        1,
        30,
        secret_bytes,
        Some("MyApp".to_string()),
        current_user.username.clone(),
    );

    // Check if the provided code is valid
    let is_valid = totp.check_current(&verify_data.code).unwrap_or(false);
    println!(
        "TOTP check result for code {}: {}",
        verify_data.code, is_valid
    );

    if is_valid {
        // Enable 2FA in database
        sqlx::query!(
            "UPDATE users SET totp_enabled = true WHERE id = $1",
            current_user.sub
        )
        .execute(pool.get_ref())
        .await
        .map_err(|_| actix_web::error::ErrorInternalServerError("Failed to enable 2FA"))?;

        let response = TOTPVerifyResponse {
            success: true,
            message: "2FA code verified successfully and 2FA is now enabled".to_string(),
        };
        Ok(HttpResponse::Ok().json(response))
    } else {
        let response = TOTPVerifyResponse {
            success: false,
            message: "Invalid 2FA code".to_string(),
        };
        Ok(HttpResponse::BadRequest().json(response))
    }
}
