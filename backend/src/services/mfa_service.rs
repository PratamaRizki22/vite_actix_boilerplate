use chrono::{Duration, Utc};
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MFAClaims {
    pub user_id: i32,
    pub username: String,
    pub email: Option<String>,
    pub exp: usize,
    pub iat: usize,
}

pub struct MFAService;

impl MFAService {
    /// Generate temporary MFA token (3 minutes validity for better security)
    pub fn generate_temp_mfa_token(user_id: i32, username: &str, email: Option<&str>, secret: &str) -> Result<String, jsonwebtoken::errors::Error> {
        let now = Utc::now();
        let exp = (now + Duration::minutes(3)).timestamp() as usize;
        let iat = now.timestamp() as usize;

        let claims = MFAClaims {
            user_id,
            username: username.to_string(),
            email: email.map(|e| e.to_string()),
            exp,
            iat,
        };

        let encoding_key = EncodingKey::from_secret(secret.as_bytes());
        encode(&Header::default(), &claims, &encoding_key)
    }

    /// Verify temporary MFA token
    pub fn verify_mfa_token(token: &str, secret: &str) -> Result<MFAClaims, jsonwebtoken::errors::Error> {
        let decoding_key = DecodingKey::from_secret(secret.as_bytes());
        let token_data = decode::<MFAClaims>(token, &decoding_key, &Validation::default())?;
        Ok(token_data.claims)
    }
}
