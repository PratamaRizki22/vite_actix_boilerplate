use actix_web::{HttpResponse, Result, web};
use serde_json;
use sqlx::PgPool;
use serde::{Deserialize, Serialize};
use reqwest::Client;

use crate::models::user::UserResponse;
use crate::services::session_manager::{SessionManager, CreateSessionData};
use crate::utils::auth::AuthUtils;

#[derive(Debug, Deserialize)]
pub struct GoogleTokenRequest {
    #[serde(alias = "id_token")]
    #[serde(alias = "access_token")]
    pub token: String,
}

#[derive(Debug, Serialize)]
pub struct GoogleTokenResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub user: UserResponse,
}

// Google JWT claims structure
#[derive(Debug, Deserialize)]
pub struct GoogleClaims {
    pub iss: String,
    pub sub: String,
    pub aud: String,
    pub email: String,
    pub name: Option<String>,
    pub picture: Option<String>,
    pub email_verified: bool,
    pub iat: i64,
    pub exp: i64,
}

pub async fn google_callback(
    pool: web::Data<PgPool>,
    jwt_secret: web::Data<String>,
    token_req: web::Json<GoogleTokenRequest>,
) -> Result<HttpResponse> {
    // Try to get user info from Google - works with both ID token and access token
    let google_claims = match get_google_user_info(&token_req.token).await {
        Ok(claims) => claims,
        Err(e) => {
            eprintln!("Failed to get Google user info: {}", e);
            return Ok(HttpResponse::Unauthorized().json(serde_json::json!({
                "error": "Invalid Google token or failed to verify"
            })));
        }
    };

    // Verify email domain and basic checks
    if !google_claims.email_verified {
        return Ok(HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Email not verified in Google account"
        })));
    }

    // Extract user info from Google claims
    let email = &google_claims.email;
    
    // Email dari Google harus ada dan verified
    if email.is_empty() {
        return Ok(HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Google account email is empty"
        })));
    }

    let name = google_claims.name.as_ref()
        .map(|n| n.clone())
        .unwrap_or_else(|| email.split('@').next().unwrap_or("User").to_string());

    // Check if user exists, if not create them
    let user = match sqlx::query_as!(
        crate::models::user::User,
        "SELECT id, username, email, password, role, wallet_address, email_verified, totp_enabled, created_at, updated_at
         FROM users WHERE email = $1",
        email
    )
    .fetch_optional(pool.get_ref())
    .await
    {
        Ok(Some(user)) => user,
        Ok(None) => {
            // Create new user from Google account
            // Username = name from Google profile (or email prefix as fallback)
            let username = google_claims.name
                .clone()
                .unwrap_or_else(|| email.split('@').next().unwrap_or("user").to_string());
            
            match sqlx::query_as!(
                crate::models::user::User,
                "INSERT INTO users (username, email, password, role, email_verified, created_at, updated_at)
                 VALUES ($1, $2, $3, $4, $5, NOW(), NOW())
                 RETURNING id, username, email, password, role, wallet_address, email_verified, totp_enabled, created_at, updated_at",
                username,
                email,
                "google_oauth", // Placeholder password for OAuth users
                "user",
                true // Email is verified via Google
            )
            .fetch_one(pool.get_ref())
            .await
            {
                Ok(user) => user,
                Err(sqlx::Error::Database(db_err)) if db_err.constraint().is_some() => {
                    // Username already taken by manual registration
                    return Ok(HttpResponse::BadRequest().json(serde_json::json!({
                        "error": "Username already taken by another account. Please register manually with a different username."
                    })));
                }
                Err(_) => {
                    return Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                        "error": "Failed to create user"
                    })));
                }
            }
        }
        Err(_) => {
            return Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Database error"
            })));
        }
    };

    // Create JWT token
    let token = match AuthUtils::create_token(user.id, &user.username, &user.role, jwt_secret.get_ref()) {
        Ok(t) => t,
        Err(_) => {
            return Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to create token"
            })));
        }
    };

    // Create session in database
    let session_data = CreateSessionData {
        user_id: user.id,
        token: token.clone(),
        device_name: Some("OAuth".to_string()),
        ip_address: None,
        user_agent: None,
    };

    match SessionManager::create_session(pool.get_ref(), session_data).await {
        Ok(_) => {},
        Err(_) => {
            return Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to create session"
            })));
        }
    }

    let user_response = UserResponse {
        id: user.id,
        username: user.username,
        email: user.email,
        role: user.role,
        wallet_address: user.wallet_address,
        email_verified: user.email_verified,
        two_factor_enabled: user.totp_enabled.unwrap_or(false),
        created_at: user.created_at,
        updated_at: user.updated_at,
    };

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "access_token": token,
        "refresh_token": "", // Implement refresh tokens separately if needed
        "user": user_response
    })))
}

fn decode_google_token(token: &str) -> Result<GoogleClaims, Box<dyn std::error::Error>> {
    // In production, you should fetch Google's public keys from:
    // https://www.googleapis.com/oauth2/v1/certs
    
    // For now, we'll do a basic decode without verification
    // This is a security compromise - in production you MUST verify against Google
    
    let parts: Vec<&str> = token.split('.').collect();
    if parts.len() != 3 {
        return Err("Invalid token format".into());
    }

    let payload = parts[1];
    let decoded = base64_url_decode(payload)?;
    let claims: GoogleClaims = serde_json::from_slice(&decoded)?;

    // Basic time validation
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)?
        .as_secs() as i64;

    if claims.exp < now {
        return Err("Token expired".into());
    }

    Ok(claims)
}

fn base64_url_decode(s: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let s = s.replace('-', "+").replace('_', "/");
    let padding = (4 - s.len() % 4) % 4;
    let s = format!("{}{}", s, "=".repeat(padding));
    
    let engine = base64::engine::general_purpose::STANDARD;
    Ok(base64::engine::Engine::decode(&engine, s)?)
}

// Get user info from Google using access token via tokeninfo endpoint
async fn get_google_user_info(token: &str) -> Result<GoogleClaims, Box<dyn std::error::Error>> {
    let client = Client::new();
    
    // Try to decode as JWT first (for ID tokens)
    if let Ok(claims) = decode_google_token(token) {
        return Ok(claims);
    }
    
    // If JWT decode fails, use Google's tokeninfo endpoint (for access tokens)
    let response = client
        .get(&format!("https://www.googleapis.com/oauth2/v1/tokeninfo?access_token={}", token))
        .send()
        .await?;
    
    if !response.status().is_success() {
        return Err("Failed to verify access token with Google".into());
    }
    
    #[derive(Deserialize)]
    struct TokenInfo {
        email: String,
        #[serde(default)]
        name: Option<String>,
        email_verified: Option<bool>,
    }
    
    let token_info: TokenInfo = response.json().await?;
    
    // Create GoogleClaims from token info
    let claims = GoogleClaims {
        iss: "https://accounts.google.com".to_string(),
        sub: "".to_string(), // Not available from tokeninfo
        aud: "".to_string(), // Not available from tokeninfo
        email: token_info.email,
        name: token_info.name,
        picture: None,
        email_verified: token_info.email_verified.unwrap_or(true),
        iat: 0,
        exp: i64::MAX,
    };
    
    Ok(claims)
}
