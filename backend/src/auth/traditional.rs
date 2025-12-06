use actix_web::{HttpRequest, HttpResponse, Result, web};
use serde_json;
use sqlx::PgPool;

use crate::middleware::auth::get_current_user;
use crate::middleware::rate_limiter::RateLimiter;
use crate::middleware::redis_token_blacklist::RedisTokenBlacklist;
use crate::models::auth::{LoginRequest, LoginResponse, RegisterRequest};
use crate::models::user::{User, UserResponse};
use crate::services::email_service::EmailService;
use crate::services::session_manager::{SessionManager, CreateSessionData};
use crate::services::token_blacklist::TokenBlacklist;
use crate::services::account_lockout::AccountLockout;
use crate::services::audit_logger::AuditLogger;
use crate::services::refresh_token_service::RefreshTokenService;
use crate::services::mfa_service::MFAService;
use crate::utils::auth::AuthUtils;

pub async fn login(
    pool: web::Data<PgPool>,
    jwt_secret: web::Data<String>,
    req: HttpRequest,
    login_data: web::Json<LoginRequest>,
) -> Result<HttpResponse> {
    // Rate limiting: 5 attempts per 3 minutes
    let (is_allowed, remaining, reset_seconds) = 
        RateLimiter::check_limit(&req, "login", 5, 3);

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
        "SELECT id, username, email, password, role, wallet_address, email_verified, totp_enabled, recovery_codes, is_banned, banned_until, last_login, created_at, updated_at
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
            token: Some(token),
            refresh_token: Some(refresh_token),
            user: UserResponse::from(user),
            requires_mfa: false,
            mfa_methods: None,
            temp_token: None,
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

    // Reset failed login attempts on successful password verification
    if let Err(e) = AccountLockout::reset_attempts(pool.get_ref(), user.id).await {
        eprintln!("Error resetting lockout attempts: {}", e);
    }

    // Reset rate limit on successful login
    RateLimiter::reset(&req, "login");

    // Check if user has 2FA enabled
    let has_2fa_enabled = user.totp_enabled.unwrap_or(false);

    // If 2FA is NOT enabled, allow direct login without MFA
    if !has_2fa_enabled {
        println!("User {} does not have 2FA enabled - allowing direct login", user.username);
        return complete_login(pool, jwt_secret, req, user).await;
    }

    // User has 2FA enabled - require MFA verification
    let mut mfa_methods: Vec<String> = Vec::new();
    // TOTP is primary method if enabled
    if has_2fa_enabled {
        mfa_methods.push("totp".to_string());
    }
    // Email is always available as fallback
    if user.email.is_some() {
        mfa_methods.push("email".to_string());
    }

    // Generate temporary MFA token (5 minutes validity)
    let temp_mfa_token = MFAService::generate_temp_mfa_token(
        user.id,
        &user.username,
        user.email.as_deref(),
        jwt_secret.get_ref()
    )
    .map_err(|_| actix_web::error::ErrorInternalServerError("MFA token generation failed"))?;

    // Return MFA challenge
    return Ok(HttpResponse::Ok().json(serde_json::json!({
        "requires_mfa": true,
        "mfa_methods": mfa_methods,
        "temp_token": temp_mfa_token,
        "user": UserResponse::from(user),
        "message": "Login successful. Please verify with 2FA to complete authentication."
    })));
}

pub async fn verify_mfa(
    pool: web::Data<PgPool>,
    jwt_secret: web::Data<String>,
    req: HttpRequest,
    verify_data: web::Json<crate::models::auth::MFAVerifyRequest>,
) -> Result<HttpResponse> {
    // SECURITY: Rate limiting for MFA verification - 10 attempts per 5 minutes
    let (is_allowed, _remaining, reset_seconds) = 
        RateLimiter::check_limit(&req, "verify_mfa", 10, 5);

    if !is_allowed {
        return Ok(HttpResponse::TooManyRequests().json(serde_json::json!({
            "error": "Too many MFA verification attempts. Please try again later.",
            "retry_after": reset_seconds
        })));
    }

    // Verify temp MFA token
    let mfa_claims = MFAService::verify_mfa_token(&verify_data.temp_token, jwt_secret.get_ref())
        .map_err(|_| actix_web::error::ErrorUnauthorized("Invalid or expired MFA token"))?;

    // Get user from database
    let user = sqlx::query_as!(
        User,
        "SELECT id, username, email, password, role, wallet_address, email_verified, totp_enabled, recovery_codes, is_banned, banned_until, last_login, created_at, updated_at
         FROM users WHERE id = $1",
        mfa_claims.user_id
    )
    .fetch_optional(pool.get_ref())
    .await
    .map_err(|_| actix_web::error::ErrorInternalServerError("Database error"))?
    .ok_or_else(|| actix_web::error::ErrorNotFound("User not found"))?;

    // Verify the code based on method
    match verify_data.method.as_str() {
        "totp" => {
            // Check if this is a setup request (empty code)
            if verify_data.code.trim().is_empty() {
                // SECURITY: Only allow QR code generation if TOTP is NOT enabled yet
                // This prevents showing QR code during login (security risk!)
                if user.totp_enabled.unwrap_or(false) {
                    return Ok(HttpResponse::BadRequest().json(serde_json::json!({
                        "error": "2FA is already enabled. Please enter your authentication code."
                    })));
                }

                // Check if user already has totp_secret (from profile or registration setup)
                let existing_secret = sqlx::query!(
                    "SELECT totp_secret FROM users WHERE id = $1",
                    user.id
                )
                .fetch_one(pool.get_ref())
                .await
                .map_err(|_| actix_web::error::ErrorInternalServerError("Failed to get user data"))?
                .totp_secret;

                // If secret exists and TOTP is NOT enabled yet, this is registration/setup - return QR
                if let Some(secret) = existing_secret {
                    let qr_code_url = format!(
                        "otpauth://totp/USH:{}?secret={}&issuer=USH&algorithm=SHA1&digits=6&period=30",
                        user.username, secret
                    );
                    
                    println!("DEBUG: Returning existing TOTP setup for registration/profile (totp_enabled=false)");
                    return Ok(HttpResponse::Ok().json(serde_json::json!({
                        "setup_required": true,
                        "secret": secret,
                        "qr_code_url": qr_code_url,
                        "message": "Please scan this QR code with your authenticator app, then enter the 6-digit code to complete setup."
                    })));
                }

                // No secret exists - generate one for first-time setup
                println!("DEBUG: Generating new TOTP secret for first-time setup");
                let new_secret = crate::utils::totp::generate_totp_secret()
                    .map_err(|_| actix_web::error::ErrorInternalServerError("Failed to generate TOTP secret"))?;
                
                // Store secret in database
                sqlx::query!(
                    "UPDATE users SET totp_secret = $1 WHERE id = $2",
                    new_secret,
                    user.id
                )
                .execute(pool.get_ref())
                .await
                .map_err(|_| actix_web::error::ErrorInternalServerError("Failed to save TOTP secret"))?;
                
                let qr_code_url = format!(
                    "otpauth://totp/USH:{}?secret={}&issuer=USH&algorithm=SHA1&digits=6&period=30",
                    user.username, new_secret
                );
                
                return Ok(HttpResponse::Ok().json(serde_json::json!({
                    "setup_required": true,
                    "secret": new_secret,
                    "qr_code_url": qr_code_url,
                    "message": "Please scan this QR code with your authenticator app, then enter the 6-digit code to complete setup."
                })));
            }

            let user_data = sqlx::query!(
                "SELECT totp_secret, recovery_codes FROM users WHERE id = $1",
                user.id
            )
            .fetch_one(pool.get_ref())
            .await
            .map_err(|_| actix_web::error::ErrorInternalServerError("Failed to get TOTP secret"))?;

            let totp_secret = user_data.totp_secret
                .ok_or_else(|| actix_web::error::ErrorBadRequest("TOTP secret not found. Please set up 2FA first."))?;

            // Try to verify as TOTP code first
            let mut is_valid = crate::utils::totp::verify_totp_code(&totp_secret, &verify_data.code)
                .unwrap_or(false);

            // If TOTP fails, check if it's a recovery code
            if !is_valid {
                if let Some(recovery_codes) = user_data.recovery_codes {
                    if recovery_codes.contains(&verify_data.code) {
                        println!("✓ Valid recovery code used by user: {}", user.username);
                        is_valid = true;
                        
                        // Log recovery code usage for security audit
                        let ip_address = req.connection_info()
                            .peer_addr()
                            .map(|s| s.to_string());
                        let user_agent = req.headers()
                            .get("User-Agent")
                            .and_then(|h| h.to_str().ok())
                            .map(|s| s.to_string());
                        
                        let _ = AuditLogger::log(
                            pool.get_ref(),
                            Some(user.id),
                            "RECOVERY_CODE_USED",
                            &format!("Recovery code used for 2FA bypass - {} codes remaining", recovery_codes.len() - 1),
                            ip_address.as_deref(),
                            user_agent.as_deref(),
                            AuditLogger::STATUS_SUCCESS,
                            Some(serde_json::json!({
                                "recovery_codes_remaining": recovery_codes.len() - 1,
                                "method": "recovery_code"
                            })),
                        ).await;
                        
                        // Remove used recovery code from database
                        let remaining_codes: Vec<String> = recovery_codes
                            .into_iter()
                            .filter(|c| c != &verify_data.code)
                            .collect();
                        
                        sqlx::query!(
                            "UPDATE users SET recovery_codes = $1 WHERE id = $2",
                            &remaining_codes,
                            user.id
                        )
                        .execute(pool.get_ref())
                        .await
                        .map_err(|_| actix_web::error::ErrorInternalServerError("Failed to update recovery codes"))?;
                    }
                }
            }

            if !is_valid {
                return Ok(HttpResponse::Unauthorized().json(serde_json::json!({
                    "error": "Invalid TOTP code or recovery code"
                })));
            }

            // If TOTP was not enabled but verification succeeded, enable it now
            if !user.totp_enabled.unwrap_or(false) {
                sqlx::query!(
                    "UPDATE users SET totp_enabled = true WHERE id = $1",
                    user.id
                )
                .execute(pool.get_ref())
                .await
                .map_err(|_| actix_web::error::ErrorInternalServerError("Failed to enable 2FA"))?;
            }
        }
        "email" => {
            // SECURITY: Check if email code has expired
            if let Some(expiry) = EmailService::get_mfa_code_expiry(user.id) {
                if expiry <= chrono::Utc::now() {
                    EmailService::clear_mfa_verification_code(user.id);
                    return Ok(HttpResponse::Unauthorized().json(serde_json::json!({
                        "error": "Email verification code has expired. Please request a new code."
                    })));
                }
            }
            
            // Verify email code
            let stored_code = EmailService::get_mfa_verification_code(user.id);
            if stored_code.as_deref() != Some(&verify_data.code) {
                return Ok(HttpResponse::Unauthorized().json(serde_json::json!({
                    "error": "Invalid email verification code"
                })));
            }
            // Clear the code after successful verification
            EmailService::clear_mfa_verification_code(user.id);
        }
        _ => {
            return Ok(HttpResponse::BadRequest().json(serde_json::json!({
                "error": "Invalid verification method"
            })));
        }
    }

    // MFA verification successful - complete login
    complete_login(pool, jwt_secret, req, user).await
}

async fn complete_login(
    pool: web::Data<PgPool>,
    jwt_secret: web::Data<String>,
    req: HttpRequest,
    user: User,
) -> Result<HttpResponse> {
    // Create JWT token
    let token = AuthUtils::create_token(user.id, &user.username, &user.role, jwt_secret.get_ref())
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

    // Update last_login timestamp
    let _ = sqlx::query!(
        "UPDATE users SET last_login = NOW() WHERE id = $1",
        user.id
    )
    .execute(pool.get_ref())
    .await;

    // Generate refresh token
    let refresh_token = RefreshTokenService::generate_token();
    let _ = RefreshTokenService::create_refresh_token(
        pool.get_ref(),
        user.id,
        &refresh_token,
    ).await;

    // Log successful login
    let _ = AuditLogger::log_login(
        pool.get_ref(),
        user.id,
        ip_address.as_deref(),
        user_agent.as_deref(),
    ).await;

    let response = LoginResponse {
        token: Some(token),
        refresh_token: Some(refresh_token),
        user: UserResponse::from(user),
        requires_mfa: false,
        mfa_methods: None,
        temp_token: None,
    };

    Ok(HttpResponse::Ok().json(response))
}


pub async fn logout(
    req: HttpRequest, 
    pool: web::Data<PgPool>,
    redis_blacklist: web::Data<RedisTokenBlacklist>,
) -> Result<HttpResponse> {
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

                // Blacklist token in Redis first (faster)
                match AuthUtils::validate_token_without_expiry(token) {
                    Ok(claims) => {
                        let expires_at = chrono::DateTime::<chrono::Utc>::from_timestamp(claims.exp as i64, 0)
                            .unwrap_or_else(|| chrono::Utc::now());
                        
                        let now = chrono::Utc::now();
                        let ttl_seconds = (expires_at - now).num_seconds().max(0) as u64;
                        
                        // Blacklist in Redis (fast)
                        if let Err(e) = redis_blacklist.blacklist_token(token, ttl_seconds).await {
                            eprintln!("Failed to blacklist token in Redis: {}", e);
                        }
                        
                        // Also blacklist in database (persistence)
                        if let Err(e) = TokenBlacklist::blacklist_token(
                            pool.get_ref(),
                            current_user.sub,
                            token,
                            expires_at,
                            Some("User logout"),
                        ).await {
                            eprintln!("Failed to blacklist token in database: {}", e);
                        }
                    }
                    Err(e) => eprintln!("Failed to decode claims for blacklist: {:?}", e),
                }
            }
        }
    }

    // Log logout
    let _ = AuditLogger::log(
        pool.get_ref(),
        Some(current_user.sub),
        AuditLogger::EVENT_LOGOUT,
        "User logged out",
        ip_address.as_deref(),
        user_agent.as_deref(),
        AuditLogger::STATUS_SUCCESS,
        None,
    ).await;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": "Logged out successfully"
    })))
}

pub async fn me(
    pool: web::Data<PgPool>,
    req: HttpRequest,
) -> Result<HttpResponse> {
    let current_user = get_current_user(&req)
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Not authenticated"))?;

    // Fetch full user data from database to get totp_enabled status
    let user = sqlx::query_as!(
        User,
        "SELECT id, username, email, password, role, wallet_address, email_verified, totp_enabled, recovery_codes, is_banned, banned_until, last_login, created_at, updated_at
         FROM users WHERE id = $1",
        current_user.sub
    )
    .fetch_optional(pool.get_ref())
    .await
    .map_err(|_| actix_web::error::ErrorInternalServerError("Database error"))?;

    let user = user
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("User not found"))?;

    let user_response = UserResponse::from(user);

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "user": user_response
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
         RETURNING id, username, email, password, role, wallet_address, email_verified, totp_enabled, recovery_codes, is_banned, banned_until, last_login, created_at, updated_at",
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

    Ok(HttpResponse::Created().json(serde_json::json!({
        "message": "Registration successful. Verification email sent.",
        "user": UserResponse::from(user)
    })))
}


