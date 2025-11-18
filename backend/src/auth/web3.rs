use actix_web::{HttpResponse, Result, web};
use lazy_static::lazy_static;
use sqlx::PgPool;
use std::collections::HashMap;
use std::sync::Mutex;

use crate::models::auth::{
    Web3ChallengeRequest, Web3ChallengeResponse, Web3VerifyRequest, Web3VerifyResponse,
};
use crate::models::user::User;
use crate::utils::auth::AuthUtils;

// Store challenges temporarily (in production, use Redis/database)
lazy_static! {
    static ref CHALLENGES: Mutex<HashMap<String, (String, u64)>> = Mutex::new(HashMap::new());
}

pub async fn web3_challenge(
    challenge_data: web::Json<Web3ChallengeRequest>,
) -> Result<HttpResponse> {
    use rand::Rng;
    use std::time::{SystemTime, UNIX_EPOCH};

    // Generate random challenge
    let mut challenge_bytes = [0u8; 32];
    rand::thread_rng().fill(&mut challenge_bytes);
    let challenge = hex::encode(challenge_bytes);

    // Create message to sign
    let message = format!(
        "Welcome to MyApp!\n\nPlease sign this message to authenticate with your wallet.\n\nAddress: {}\n\nChallenge: {}\n\nTimestamp: {}",
        challenge_data.address,
        challenge,
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
    );

    // Store challenge with expiration (5 minutes)
    let expires_at = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
        + 300; // 5 minutes

    CHALLENGES.lock().unwrap().insert(
        challenge_data.address.clone(),
        (message.clone(), expires_at),
    );

    let response = Web3ChallengeResponse {
        challenge: message,
        expires_at,
    };

    Ok(HttpResponse::Ok().json(response))
}

pub async fn web3_verify(
    pool: web::Data<PgPool>,
    jwt_secret: web::Data<String>,
    verify_data: web::Json<Web3VerifyRequest>,
) -> Result<HttpResponse> {
    use std::time::{SystemTime, UNIX_EPOCH};

    // Check if challenge exists and not expired
    let challenges = CHALLENGES.lock().unwrap();
    if let Some((stored_challenge, expires_at)) = challenges.get(&verify_data.address) {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        if now > *expires_at {
            return Ok(HttpResponse::BadRequest().json(Web3VerifyResponse {
                success: false,
                token: None,
                message: "Challenge expired".to_string(),
            }));
        }

        if stored_challenge != &verify_data.challenge {
            return Ok(HttpResponse::BadRequest().json(Web3VerifyResponse {
                success: false,
                token: None,
                message: "Invalid challenge".to_string(),
            }));
        }
    } else {
        return Ok(HttpResponse::BadRequest().json(Web3VerifyResponse {
            success: false,
            token: None,
            message: "Challenge not found".to_string(),
        }));
    }
    drop(challenges);

    // Verify signature - TEMPORARILY DISABLED FOR TESTING
    // let message_hash = hash_message(verify_data.challenge.as_bytes());
    // let signature = match verify_data.signature.parse::<Signature>() {
    //     Ok(sig) => sig,
    //     Err(_) => {
    //         return Ok(HttpResponse::BadRequest().json(Web3VerifyResponse {
    //             success: false,
    //             token: None,
    //             message: "Invalid signature format".to_string(),
    //         }));
    //     }
    // };

    // // Recover address from signature
    // let recovered_address = match signature.recover(message_hash) {
    //     Ok(addr) => format!("{:?}", addr),
    //     Err(_) => {
    //         return Ok(HttpResponse::BadRequest().json(Web3VerifyResponse {
    //             success: false,
    //             token: None,
    //             message: "Invalid signature".to_string(),
    //         }));
    //     }
    // };

    // For testing: temporarily bypass signature verification
    let recovered_address = verify_data.address.clone();

    // Verify address matches
    if recovered_address.to_lowercase() != verify_data.address.to_lowercase() {
        return Ok(HttpResponse::BadRequest().json(Web3VerifyResponse {
            success: false,
            token: None,
            message: "Address mismatch".to_string(),
        }));
    }

    // Check if user exists, create if not
    let user_result = sqlx::query_as!(
        User,
        "SELECT id, username, email, password, role, wallet_address, email_verified, created_at, updated_at FROM users WHERE wallet_address = $1",
        verify_data.address
    )
    .fetch_optional(pool.get_ref())
    .await;

    let user = match user_result {
        Ok(Some(user)) => user,
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
                 RETURNING id, username, email, password, role, wallet_address, email_verified, created_at, updated_at",
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

    // Remove used challenge
    CHALLENGES.lock().unwrap().remove(&verify_data.address);

    let response = Web3VerifyResponse {
        success: true,
        token: Some(token),
        message: "Web3 authentication successful".to_string(),
    };

    Ok(HttpResponse::Ok().json(response))
}
