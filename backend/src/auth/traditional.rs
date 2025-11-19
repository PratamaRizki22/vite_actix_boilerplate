use actix_web::{HttpRequest, HttpResponse, Result, web};
use serde_json;
use sqlx::PgPool;

use crate::middleware::auth::get_current_user;
use crate::middleware::rate_limiter::RateLimiter;
use crate::models::auth::{LoginRequest, LoginResponse, RegisterRequest};
use crate::models::user::{User, UserResponse};
use crate::services::email_service::EmailService;
use crate::services::session_manager::{SessionManager, CreateSessionData};
use crate::services::token_blacklist::TokenBlacklist;
use crate::services::account_lockout::AccountLockout;
use crate::services::audit_logger::AuditLogger;
use crate::services::refresh_token_service::RefreshTokenService;
use crate::utils::auth::AuthUtils;

pub async fn login(
    pool: web::Data<PgPool>,
    jwt_secret: web::Data<String>,
    req: HttpRequest,
    login_data: web::Json<LoginRequest>,
) -> Result<HttpResponse> {
    // Rate limiting: 5 attempts per 15 minutes
    let (is_allowed, remaining, reset_seconds) = 
        RateLimiter::check_limit(&req, "login", 5, 15);

    if !is_allowed {
        return Ok(HttpResponse::TooManyRequests().json(serde_json::json!({
            "error": "Too many login attempts. Please try again later.",
            "retry_after": reset_seconds
        })));
    }

    // Find user by username, email, or wallet address
    let login_identifier = login_data
        .username
        .as_ref()
        .or(login_data.email.as_ref())
        .ok_or_else(|| actix_web::error::ErrorBadRequest("Username or email is required"))?;

    let user = sqlx::query_as!(
        User,
        "SELECT id, username, email, password, role, wallet_address, email_verified, created_at, updated_at
         FROM users WHERE username = $1 OR email = $1 OR wallet_address = $1",
        login_identifier
    )
    .fetch_optional(pool.get_ref())
    .await
    .map_err(|_| actix_web::error::ErrorInternalServerError("Database error"))?;

    let user = match user {
        Some(user) => user,
        None => {
            // Log failed login attempt (invalid user)
            let ip_address = req.connection_info()
                .peer_addr()
                .map(|s| s.to_string());
            let user_agent = req.headers()
                .get("User-Agent")
                .and_then(|h| h.to_str().ok())
                .map(|s| s.to_string());
            
            let _ = AuditLogger::log(
                pool.get_ref(),
                None,
                AuditLogger::EVENT_FAILED_LOGIN,
                "Failed login attempt - invalid username/email",
                ip_address.as_deref(),
                user_agent.as_deref(),
                AuditLogger::STATUS_FAILED,
                Some(serde_json::json!({"identifier": login_identifier})),
            ).await;

            return Ok(HttpResponse::Unauthorized().json(serde_json::json!({
                "error": "Invalid credentials"
            })));
        }
    };

    // Check if account is locked due to failed login attempts
    match AccountLockout::is_locked(pool.get_ref(), user.id).await {
        Ok(true) => {
            // Account is locked
            let ip_address = req.connection_info()
                .peer_addr()
                .map(|s| s.to_string());
            let user_agent = req.headers()
                .get("User-Agent")
                .and_then(|h| h.to_str().ok())
                .map(|s| s.to_string());
            
            // Log lockout attempt
            match AccountLockout::get_remaining_lockout_seconds(pool.get_ref(), user.id).await {
                Ok(seconds) => {
                    let _ = AuditLogger::log(
                        pool.get_ref(),
                        Some(user.id),
                        AuditLogger::EVENT_FAILED_LOGIN,
                        "Login attempt on locked account",
                        ip_address.as_deref(),
                        user_agent.as_deref(),
                        AuditLogger::STATUS_BLOCKED,
                        Some(serde_json::json!({"remaining_seconds": seconds})),
                    ).await;

                    return Ok(HttpResponse::Forbidden().json(serde_json::json!({
                        "error": "Account is temporarily locked due to too many failed login attempts",
                        "locked": true,
                        "retry_after": seconds
                    })));
                }
                Err(_) => {
                    let _ = AuditLogger::log(
                        pool.get_ref(),
                        Some(user.id),
                        AuditLogger::EVENT_FAILED_LOGIN,
                        "Login attempt on locked account",
                        ip_address.as_deref(),
                        user_agent.as_deref(),
                        AuditLogger::STATUS_BLOCKED,
                        None,
                    ).await;

                    return Ok(HttpResponse::Forbidden().json(serde_json::json!({
                        "error": "Account is temporarily locked",
                        "locked": true
                    })));
                }
            }
        }
        Err(e) => {
            eprintln!("Error checking account lockout: {}", e);
            // Don't block login on error, but log it
        }
        _ => {}
    }

    // Check if email is verified (except for Web3 users)
    if user.password != "web3_auth" && !user.email_verified {
        return Ok(HttpResponse::Unauthorized().json(serde_json::json!({
            "error": "Email not verified. Please check your email and verify your account.",
            "needs_verification": true
        })));
    }

    // For Web3 users (no password required), allow login without password check
    if user.password == "web3_auth" {
        // Create JWT token for Web3 user
        let token =
            AuthUtils::create_token(user.id, &user.username, &user.role, jwt_secret.get_ref())
                .map_err(|_| actix_web::error::ErrorInternalServerError("Token creation failed"))?;

        // Create session in database
        let device_name = req.headers()
            .get("User-Agent")
            .and_then(|h| h.to_str().ok())
            .map(|ua| {
                if ua.contains("Mobile") { "Mobile" }
                else if ua.contains("Tablet") { "Tablet" }
                else { "Desktop" }
            })
            .map(|s| s.to_string());

        let ip_address = req.connection_info()
            .peer_addr()
            .map(|s| s.to_string());

        let user_agent = req.headers()
            .get("User-Agent")
            .and_then(|h| h.to_str().ok())
            .map(|s| s.to_string());

        let session_data = CreateSessionData {
            user_id: user.id,
            token: token.clone(),
            device_name,
            ip_address: ip_address.clone(),
            user_agent: user_agent.clone(),
        };

        if let Err(e) = SessionManager::create_session(pool.get_ref(), session_data).await {
            eprintln!("Failed to create session: {}", e);
        }

        // Reset rate limit on successful login
        RateLimiter::reset(&req, "login");

        // Log successful Web3 login
        let ip_address = req.connection_info()
            .peer_addr()
            .map(|s| s.to_string());
        let user_agent = req.headers()
            .get("User-Agent")
            .and_then(|h| h.to_str().ok())
            .map(|s| s.to_string());
        
        let _ = AuditLogger::log_login(
            pool.get_ref(),
            user.id,
            ip_address.as_deref(),
            user_agent.as_deref(),
        ).await;

        // Generate refresh token
        let refresh_token = RefreshTokenService::generate_token();
        let _ = RefreshTokenService::create_refresh_token(
            pool.get_ref(),
            user.id,
            &refresh_token,
        ).await;

        let response = LoginResponse {
            token,
            refresh_token,
            user: UserResponse::from(user),
        };

        return Ok(HttpResponse::Ok().json(response));
    }

    // Verify password with bcrypt for traditional users
    let is_valid_password = AuthUtils::verify_password(&login_data.password, &user.password)
        .map_err(|_| actix_web::error::ErrorInternalServerError("Password verification failed"))?;

    if !is_valid_password {
        // Log failed login attempt
        let ip_address = req.connection_info()
            .peer_addr()
            .map(|s| s.to_string());
        let user_agent = req.headers()
            .get("User-Agent")
            .and_then(|h| h.to_str().ok())
            .map(|s| s.to_string());
        
        let _ = AuditLogger::log_failed_login(
            pool.get_ref(),
            user.id,
            "Invalid password",
            ip_address.as_deref(),
            user_agent.as_deref(),
        ).await;

        // Record failed login attempt
        if let Err(e) = AccountLockout::record_failed_attempt(pool.get_ref(), user.id).await {
            eprintln!("Error recording failed login attempt: {}", e);
        }
        
        return Ok(HttpResponse::Unauthorized().json(serde_json::json!({
            "error": "Invalid credentials"
        })));
    }

    // Create JWT token
    let token = AuthUtils::create_token(user.id, &user.username, &user.role, jwt_secret.get_ref())
        .map_err(|_| actix_web::error::ErrorInternalServerError("Token creation failed"))?;

    // Reset failed login attempts on successful login
    if let Err(e) = AccountLockout::reset_attempts(pool.get_ref(), user.id).await {
        eprintln!("Error resetting lockout attempts: {}", e);
    }

    // Create session in database
    let device_name = req.headers()
        .get("User-Agent")
        .and_then(|h| h.to_str().ok())
        .map(|ua| {
            if ua.contains("Mobile") { "Mobile" }
            else if ua.contains("Tablet") { "Tablet" }
            else { "Desktop" }
        })
        .map(|s| s.to_string());

    let ip_address = req.connection_info()
        .peer_addr()
        .map(|s| s.to_string());

    let user_agent = req.headers()
        .get("User-Agent")
        .and_then(|h| h.to_str().ok())
        .map(|s| s.to_string());

    let session_data = CreateSessionData {
        user_id: user.id,
        token: token.clone(),
        device_name,
        ip_address: ip_address.clone(),
        user_agent: user_agent.clone(),
    };

    if let Err(e) = SessionManager::create_session(pool.get_ref(), session_data).await {
        eprintln!("Failed to create session: {}", e);
        // Don't fail login if session creation fails
    }

    // Reset rate limit on successful login
    RateLimiter::reset(&req, "login");

    // Log successful login
    let _ = AuditLogger::log_login(
        pool.get_ref(),
        user.id,
        ip_address.as_deref(),
        user_agent.as_deref(),
    ).await;

    // Generate refresh token
    let refresh_token = RefreshTokenService::generate_token();
    let _ = RefreshTokenService::create_refresh_token(
        pool.get_ref(),
        user.id,
        &refresh_token,
    ).await;

    let response = LoginResponse {
        token,
        refresh_token,
        user: UserResponse::from(user),
    };

    Ok(HttpResponse::Ok().json(response))
}

pub async fn logout(req: HttpRequest, pool: web::Data<PgPool>) -> Result<HttpResponse> {
    let current_user = get_current_user(&req)
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Not authenticated"))?;

    let ip_address = req.connection_info()
        .peer_addr()
        .map(|s| s.to_string());

    let user_agent = req.headers()
        .get("User-Agent")
        .and_then(|h| h.to_str().ok())
        .map(|s| s.to_string());

    // Get token from Authorization header
    if let Some(auth_header) = req.headers().get("Authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            if let Ok(token) = AuthUtils::extract_token_from_header(auth_str) {
                // Invalidate session
                if let Err(e) = SessionManager::logout(pool.get_ref(), token).await {
                    eprintln!("Failed to logout session: {}", e);
                }

                // Blacklist token (get expiration from JWT)
                match AuthUtils::validate_token_without_expiry(token) {
                    Ok(claims) => {
                        let expires_at = chrono::DateTime::<chrono::Utc>::from_timestamp(claims.exp as i64, 0)
                            .unwrap_or_else(|| chrono::Utc::now());
                        
                        if let Err(e) = TokenBlacklist::blacklist_token(
                            pool.get_ref(),
                            current_user.sub,
                            token,
                            expires_at,
                            Some("User logout"),
                        ).await {
                            eprintln!("Failed to blacklist token: {}", e);
                        }
                    }
                    Err(e) => eprintln!("Failed to decode claims for blacklist: {:?}", e),
                }
            }
        }
    }

    // Log logout
    let _ = AuditLogger::log_logout(
        pool.get_ref(),
        current_user.sub,
        ip_address.as_deref(),
        user_agent.as_deref(),
    ).await;

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

    // First, insert user with temporary username to get the ID
    let temp_username = format!("temp_{}", chrono::Utc::now().timestamp_millis());
    let mut user = sqlx::query_as!(
        User,
        "INSERT INTO users (username, email, password, role, email_verified)
         VALUES ($1, $2, $3, 'user', false)
         RETURNING id, username, email, password, role, wallet_address, email_verified, created_at, updated_at",
        temp_username,
        register_data.email,
        hashed_password
    )
    .fetch_one(pool.get_ref())
    .await
    .map_err(|err| {
        if let sqlx::Error::Database(db_err) = &err {
            if db_err.constraint().is_some() {
                return actix_web::error::ErrorBadRequest("Email already exists");
            }
        }
        actix_web::error::ErrorInternalServerError("Database error")
    })?;

    // Generate the proper username: firstname_ush_id
    let proper_username = format!("{}_ush_{}", register_data.first_name.to_lowercase(), user.id);

    // Update the username in the database
    sqlx::query!(
        "UPDATE users SET username = $1 WHERE id = $2",
        proper_username,
        user.id
    )
    .execute(pool.get_ref())
    .await
    .map_err(|_| actix_web::error::ErrorInternalServerError("Failed to update username"))?;

    // Update the user object with the new username
    user.username = proper_username;

    // Send verification email
    let email_client = match EmailService::new() {
        Ok(client) => Some(client),
        Err(e) => {
            // Log error but don't fail registration
            eprintln!("Failed to initialize email client: {}", e);
            None
        }
    };

    if let Some(email) = &user.email {
        let code = EmailService::generate_verification_code();
        EmailService::store_verification_code(email, &code);

        if let Some(client) = email_client {
            if let Err(e) = client.send_verification_email(email, &code).await {
                eprintln!("Failed to send verification email: {}", e);
                // Don't fail registration if email fails
            }
        }
    }

    Ok(HttpResponse::Created().json(UserResponse::from(user)))
}
