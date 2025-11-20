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
        "otpauth://totp/USH:{}?secret={}&issuer=USH&algorithm=SHA1&digits=6&period=30",
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
    println!("\n=== 2FA Verification Started ===");
    println!("Received code: {}", verify_data.code);
    
    // Get current server time
    let server_time = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    println!("Server time (UTC): {} ({})", server_time, chrono::DateTime::<chrono::Utc>::from(std::time::UNIX_EPOCH) + chrono::Duration::seconds(server_time as i64));

    // Validate code format
    if verify_data.code.is_empty() || verify_data.code.len() < 6 {
        println!("❌ Invalid code format: length={}", verify_data.code.len());
        return Ok(HttpResponse::BadRequest().json(TOTPVerifyResponse {
            success: false,
            message: "Code must be at least 6 digits".to_string(),
        }));
    }

    let current_user = get_current_user(&req)
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Not authenticated"))?;

    println!("✓ User authenticated: {} (ID: {})", current_user.username, current_user.sub);

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
        .ok_or_else(|| {
            println!("❌ No TOTP secret found in database for user");
            actix_web::error::ErrorBadRequest("2FA not set up for this user")
        })?;

    println!("✓ TOTP secret exists (base32): {}", &totp_secret);

    // Decode base32 secret
    let secret_bytes = base32::decode(base32::Alphabet::RFC4648 { padding: false }, &totp_secret)
        .ok_or_else(|| {
            println!("❌ Failed to decode base32 secret");
            actix_web::error::ErrorInternalServerError("Invalid TOTP secret format")
        })?;

    println!("✓ Secret decoded, length: {} bytes", secret_bytes.len());

    // Create TOTP instance with proper parameters (RFC 6238 standard)
    let totp = totp_rs::TOTP::new_unchecked(
        totp_rs::Algorithm::SHA1,
        6,              // 6-digit code
        1,              // 1 digit (internal use)
        30,             // 30 second period
        secret_bytes.clone(),
        Some("USH".to_string()),
        current_user.username.clone(),
    );

    // Check if the provided code is valid with time window tolerance
    // totp_rs has built-in ±1 step tolerance by default (±30 seconds)
    let is_valid = totp.check_current(&verify_data.code).unwrap_or(false);
    
    println!("🔍 TOTP Verification Result:");
    println!("  - Code entered: {}", verify_data.code);
    println!("  - Valid: {}", is_valid);
    
    // Also check what codes would be valid now (for debugging)
    if !is_valid {
        // Generate codes for debugging (current + previous + next)
        println!("  - Debugging info:");
        match totp.generate_current() {
            Ok(current_code) => println!("    Current valid code: {}", current_code),
            Err(e) => println!("    Error generating current code: {:?}", e),
        }
    }
    println!("=== 2FA Verification End ===\n");

    if is_valid {
        // Enable 2FA in database
        sqlx::query!(
            "UPDATE users SET totp_enabled = true WHERE id = $1",
            current_user.sub
        )
        .execute(pool.get_ref())
        .await
        .map_err(|_| actix_web::error::ErrorInternalServerError("Failed to enable 2FA"))?;

        println!("✓ 2FA enabled in database for user {}", current_user.username);

        let response = TOTPVerifyResponse {
            success: true,
            message: "2FA code verified successfully and 2FA is now enabled".to_string(),
        };
        Ok(HttpResponse::Ok().json(response))
    } else {
        let response = TOTPVerifyResponse {
            success: false,
            message: "Invalid 2FA code. Please check your code and try again. Make sure your device time is synchronized.".to_string(),
        };
        Ok(HttpResponse::BadRequest().json(response))
    }
}

pub async fn debug_2fa(
    pool: web::Data<PgPool>,
    req: HttpRequest,
) -> Result<HttpResponse> {
    let current_user = get_current_user(&req)
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Not authenticated"))?;

    // Get current server time
    let server_time = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    // Get secret from database
    let user_record = sqlx::query!(
        "SELECT totp_secret, totp_enabled FROM users WHERE id = $1",
        current_user.sub
    )
    .fetch_one(pool.get_ref())
    .await
    .map_err(|_| actix_web::error::ErrorInternalServerError("Failed to get user data"))?;

    let totp_secret = user_record.totp_secret.clone();
    let totp_enabled = user_record.totp_enabled.unwrap_or(false);

    let mut debug_info = serde_json::json!({
        "user_id": current_user.sub,
        "username": current_user.username,
        "server_time_unix": server_time,
        "server_time_readable": chrono::DateTime::<chrono::Utc>::from(std::time::UNIX_EPOCH) + chrono::Duration::seconds(server_time as i64),
        "totp_setup": totp_secret.is_some(),
        "totp_enabled": totp_enabled,
    });

    if let Some(secret) = totp_secret {
        if let Some(secret_bytes) = base32::decode(base32::Alphabet::RFC4648 { padding: false }, &secret) {
            let totp = totp_rs::TOTP::new_unchecked(
                totp_rs::Algorithm::SHA1,
                6,
                1,
                30,
                secret_bytes,
                Some("USH".to_string()),
                current_user.username.clone(),
            );

            if let Ok(current_code) = totp.generate_current() {
                debug_info["current_valid_code"] = serde_json::json!(current_code);
                debug_info["code_valid_for_seconds"] = serde_json::json!("~30 seconds");
            }
        }
    }

    Ok(HttpResponse::Ok().json(debug_info))
}

pub async fn disable_2fa(
    pool: web::Data<PgPool>,
    req: HttpRequest,
) -> Result<HttpResponse> {
    let current_user = get_current_user(&req)
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Not authenticated"))?;

    println!("=== Disable 2FA Started ===");
    println!("User: {} (ID: {})", current_user.username, current_user.sub);

    // Disable 2FA in database
    sqlx::query!(
        "UPDATE users SET totp_enabled = false, totp_secret = NULL WHERE id = $1",
        current_user.sub
    )
    .execute(pool.get_ref())
    .await
    .map_err(|_| actix_web::error::ErrorInternalServerError("Failed to disable 2FA"))?;

    println!("✓ 2FA disabled for user {}", current_user.username);
    println!("=== Disable 2FA End ===\n");

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "message": "2FA has been successfully disabled for your account"
    })))
}
