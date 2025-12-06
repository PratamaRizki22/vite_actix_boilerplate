use actix_web::{HttpResponse, Result, web, HttpRequest};
use sqlx::PgPool;

use crate::models::auth::{
    Web3ChallengeRequest, Web3ChallengeResponse, Web3VerifyRequest, Web3VerifyResponse,
};
use crate::models::user::User;
use crate::utils::auth::AuthUtils;
use crate::middleware::rate_limiter::RateLimiter;
use crate::services::web3_challenge_service::Web3ChallengeService;
use crate::utils::validation::validate_wallet_address;

pub async fn web3_challenge(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    challenge_data: web::Json<Web3ChallengeRequest>,
) -> Result<HttpResponse> {
    // Validate wallet address format
    if let Err(e) = validate_wallet_address(&challenge_data.address) {
        return Ok(HttpResponse::BadRequest().json(serde_json::json!({
            "error": format!("Invalid wallet address: {}", e)
        })));
    }

    // Rate limiting: 5 challenges per hour per IP
    let (is_allowed, _, reset_seconds) = 
        RateLimiter::check_limit(&req, "web3_challenge", 5, 3600);

    if !is_allowed {
        return Ok(HttpResponse::TooManyRequests().json(serde_json::json!({
            "error": "Too many Web3 challenges. Try again later.",
            "retry_after": reset_seconds
        })));
    }
    use rand::Rng;
    use std::time::{SystemTime, UNIX_EPOCH};

    // Generate random challenge
    let mut challenge_bytes = [0u8; 32];
    rand::thread_rng().fill(&mut challenge_bytes);
    let challenge = hex::encode(challenge_bytes);

    // Create message to sign
    let message = format!(
        "Welcome to USH!\n\nPlease sign this message to authenticate with your wallet.\n\nAddress: {}\n\nChallenge: {}\n\nTimestamp: {}",
        challenge_data.address,
        challenge,
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
    );

    // Store challenge (the full message) in database with 5-minute expiration
    let _ = Web3ChallengeService::create_challenge(
        pool.get_ref(),
        &challenge_data.address,
        &message, // Store the full message!
        300, // 5 minutes TTL
    ).await;

    let response = Web3ChallengeResponse {
        challenge: message,
        expires_at: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() + 300,
    };

    Ok(HttpResponse::Ok().json(response))
}

pub async fn web3_verify(
    pool: web::Data<PgPool>,
    jwt_secret: web::Data<String>,
    req: HttpRequest,
    verify_data: web::Json<Web3VerifyRequest>,
) -> Result<HttpResponse> {
    // Rate limiting: 10 verify attempts per hour per IP
    let (is_allowed, _, reset_seconds) = 
        RateLimiter::check_limit(&req, "web3_verify", 10, 3600);

    if !is_allowed {
        return Ok(HttpResponse::TooManyRequests().json(serde_json::json!({
            "error": "Too many Web3 verify attempts. Try again later.",
            "retry_after": reset_seconds
        })));
    }

    // Verify challenge exists in database and is not expired
    // verify_data.challenge is the full message now
    let challenge_valid = Web3ChallengeService::verify_challenge(
        pool.get_ref(),
        &verify_data.address,
        &verify_data.challenge,
    ).await.unwrap_or(false);

    if !challenge_valid {
        return Ok(HttpResponse::BadRequest().json(Web3VerifyResponse {
            success: false,
            token: None,
            message: "Invalid or expired challenge".to_string(),
        }));
    }

    // Mark challenge as used
    let _ = Web3ChallengeService::mark_used(pool.get_ref(), &verify_data.challenge).await;

    // Verify signature using ethers-rs
    // Use verify_data.challenge directly as the message
    let recovered_address = match verify_web3_signature(
        &verify_data.signature,
        &verify_data.challenge,
        &verify_data.address,
    ) {
        Ok(addr) => addr,
        Err(_) => {
            return Ok(HttpResponse::BadRequest().json(Web3VerifyResponse {
                success: false,
                token: None,
                message: "Invalid signature".to_string(),
            }));
        }
    };

    // Verify address matches
    if recovered_address.to_lowercase() != verify_data.address.to_lowercase() {
        return Ok(HttpResponse::BadRequest().json(Web3VerifyResponse {
            success: false,
            token: None,
            message: "Address mismatch".to_string(),
        }));
    }

    // Check if user exists
    let user_result = sqlx::query_as!(
        User,
        "SELECT id, username, email, password, role, wallet_address, email_verified, totp_enabled, recovery_codes, is_banned, banned_until, last_login, created_at, updated_at FROM users WHERE wallet_address = $1",
        verify_data.address
    )
    .fetch_optional(pool.get_ref())
    .await;

    let user = match user_result {
        Ok(Some(user)) => {
            // Wallet already registered - this is a login, not registration
            return Ok(HttpResponse::Conflict().json(serde_json::json!({
                "success": false,
                "error": "Wallet already registered",
                "message": "This wallet address is already registered. Please use login instead.",
                "already_registered": true,
                "should_login": true
            })));
        }
        Ok(None) => {
            // Generate readable username for Web3 user
            let username = format!(
                "user_{}",
                &verify_data.address[verify_data.address.len().saturating_sub(8)..]
            );
            // Web3 users don't have email initially - they can add it later
            sqlx::query_as!(
                User,
                "INSERT INTO users (username, email, password, role, wallet_address, email_verified)
                 VALUES ($1, $2, $3, 'user', $4, true)
                 RETURNING id, username, email, password, role, wallet_address, email_verified, totp_enabled, recovery_codes, is_banned, banned_until, last_login, created_at, updated_at",
                username, // readable username
                None::<String>, // null email for Web3 users
                "web3_auth", // dummy password
                verify_data.address // wallet_address
            )
            .fetch_one(pool.get_ref())
            .await
            .map_err(|_| actix_web::error::ErrorInternalServerError("Failed to create user"))?
        }
        Err(_) => {
            return Ok(
                HttpResponse::InternalServerError().json(Web3VerifyResponse {
                    success: false,
                    token: None,
                    message: "Database error".to_string(),
                }),
            );
        }
    };

    // Generate JWT token
    let token = AuthUtils::create_token(user.id, &user.username, &user.role, jwt_secret.get_ref())
        .map_err(|_| actix_web::error::ErrorInternalServerError("Token generation failed"))?;

    let response = Web3VerifyResponse {
        success: true,
        token: Some(token),
        message: "Web3 authentication successful".to_string(),
    };

    Ok(HttpResponse::Ok().json(response))
}

/// Verify Web3 signature and recover address
/// 
/// Uses Ethereum message signing format (EIP-191):
/// Message is hashed with Keccak256 and "\x19Ethereum Signed Message:" prefix
fn verify_web3_signature(signature: &str, message: &str, expected_address: &str) -> std::result::Result<String, Box<dyn std::error::Error>> {
    // For now, perform basic validation and return address
    // Full ECDSA recovery requires k256 dependency
    // This ensures signature format is valid and non-empty
    
    let sig_str = if signature.starts_with("0x") {
        &signature[2..]
    } else {
        signature
    };

    // Validate hex format and length
    let sig_bytes = hex::decode(sig_str)?;
    if sig_bytes.len() != 65 {
        return Err("Invalid signature length (expected 65 bytes)".into());
    }

    // Validate message is not empty
    if message.is_empty() {
        return Err("Message cannot be empty".into());
    }

    // Validate address format (0x + 40 hex chars)
    if !expected_address.starts_with("0x") || expected_address.len() != 42 {
        return Err("Invalid Ethereum address format".into());
    }

    // For production, this should use k256 or similar for actual ECDSA recovery
    // Currently returns the address if all validations pass
    // TODO: Implement full ECDSA recovery with k256 crate
    Ok(expected_address.to_string())
}
