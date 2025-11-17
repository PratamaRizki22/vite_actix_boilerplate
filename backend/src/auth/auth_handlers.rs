use actix_web::{web, HttpRequest, HttpResponse, Result};
use sqlx::PgPool;
use serde_json;

use crate::auth::auth_models::{LoginRequest, LoginResponse, RegisterRequest, TOTPSetupResponse, TOTPVerifyRequest, TOTPVerifyResponse};
use crate::models::user::{User, UserResponse};
use crate::auth::auth_utils::AuthUtils;
use crate::middleware::auth::get_current_user;

pub async fn login(
    pool: web::Data<PgPool>,
    jwt_secret: web::Data<String>,
    login_data: web::Json<LoginRequest>,
) -> Result<HttpResponse> {
    // Find user by username
    let user = sqlx::query_as!(
        User,
        "SELECT id, username, email, password, role, created_at, updated_at
         FROM users WHERE username = $1",
        login_data.username
    )
    .fetch_optional(pool.get_ref())
    .await
    .map_err(|_| actix_web::error::ErrorInternalServerError("Database error"))?;

    let user = match user {
        Some(user) => user,
        None => return Ok(HttpResponse::Unauthorized().json(serde_json::json!({
            "error": "Invalid credentials"
        }))),
    };

    // Verify password with bcrypt
    let is_valid_password = AuthUtils::verify_password(&login_data.password, &user.password)
        .map_err(|_| actix_web::error::ErrorInternalServerError("Password verification failed"))?;

    if !is_valid_password {
        return Ok(HttpResponse::Unauthorized().json(serde_json::json!({
            "error": "Invalid credentials"
        })));
    }

    // Create JWT token
    let token = AuthUtils::create_token(
        user.id,
        &user.username,
        &user.role,
        jwt_secret.get_ref(),
    )
    .map_err(|_| actix_web::error::ErrorInternalServerError("Token creation failed"))?;

    let response = LoginResponse {
        token,
        user: UserResponse::from(user),
    };

    Ok(HttpResponse::Ok().json(response))
}

pub async fn logout(req: HttpRequest) -> Result<HttpResponse> {
    let _current_user = get_current_user(&req)
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Not authenticated"))?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": "Successfully logged out"
    })))
}

pub async fn me(req: HttpRequest) -> Result<HttpResponse> {
    let current_user = get_current_user(&req)
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Not authenticated"))?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "user": {
            "id": current_user.sub,
            "username": current_user.username,
            "role": current_user.role
        }
    })))
}

pub async fn register(
    pool: web::Data<PgPool>,
    register_data: web::Json<RegisterRequest>,
) -> Result<HttpResponse> {
    // Hash password before storing
    let hashed_password = AuthUtils::hash_password(&register_data.password)
        .map_err(|_| actix_web::error::ErrorInternalServerError("Password hashing failed"))?;

    // Create user with default role "user"
    let user = sqlx::query_as!(
        User,
        "INSERT INTO users (username, email, password, role)
         VALUES ($1, $2, $3, 'user')
         RETURNING id, username, email, password, role, created_at, updated_at",
        register_data.username,
        register_data.email,
        hashed_password
    )
    .fetch_one(pool.get_ref())
    .await
    .map_err(|err| {
        if let sqlx::Error::Database(db_err) = &err {
            if db_err.constraint().is_some() {
                return actix_web::error::ErrorBadRequest("Username or email already exists");
            }
        }
        actix_web::error::ErrorInternalServerError("Database error")
    })?;

    Ok(HttpResponse::Created().json(UserResponse::from(user)))
}

pub async fn setup_2fa(
    pool: web::Data<PgPool>,
    req: HttpRequest,
) -> Result<HttpResponse> {
    let current_user = get_current_user(&req)
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Not authenticated"))?;

    // Generate random secret
    use rand::Rng;
    let mut secret_bytes = [0u8; 20];
    rand::thread_rng().fill(&mut secret_bytes);
    let secret_bytes_vec = secret_bytes.to_vec();

    let secret_base32 = base32::encode(base32::Alphabet::RFC4648 { padding: false }, &secret_bytes_vec);

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
        current_user.username,
        secret_base32
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

    let totp_secret = user_record.totp_secret
        .ok_or_else(|| actix_web::error::ErrorBadRequest("2FA not set up for this user"))?;

    // Decode base32 secret
    let secret_bytes = base32::decode(base32::Alphabet::RFC4648 { padding: false }, &totp_secret)
        .ok_or_else(|| actix_web::error::ErrorInternalServerError("Invalid TOTP secret format"))?;

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
    println!("TOTP check result for code {}: {}", verify_data.code, is_valid);

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