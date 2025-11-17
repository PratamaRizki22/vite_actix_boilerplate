use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation, Algorithm};

use crate::auth::auth_models::Claims;

#[derive(Debug)]
pub enum AuthError {
    InvalidToken,
    TokenExpired,
    InvalidCredentials,
    HashingError,
    TokenCreationError,
}

impl std::fmt::Display for AuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AuthError::InvalidToken => write!(f, "Invalid token"),
            AuthError::TokenExpired => write!(f, "Token expired"),
            AuthError::InvalidCredentials => write!(f, "Invalid credentials"),
            AuthError::HashingError => write!(f, "Password hashing error"),
            AuthError::TokenCreationError => write!(f, "Token creation error"),
        }
    }
}

impl std::error::Error for AuthError {}

pub struct AuthUtils;

impl AuthUtils {
    /// Hash password with bcrypt
    pub fn hash_password(password: &str) -> Result<String, AuthError> {
        hash(password, DEFAULT_COST).map_err(|_| AuthError::HashingError)
    }

    /// Verify password against hash
    pub fn verify_password(password: &str, hash: &str) -> Result<bool, AuthError> {
        verify(password, hash).map_err(|_| AuthError::InvalidCredentials)
    }

    /// Create JWT token
    pub fn create_token(
        user_id: i32,
        username: &str,
        role: &str,
        secret: &str,
    ) -> Result<String, AuthError> {
        let now = Utc::now();
        let expires_at = now + Duration::hours(24); // Token valid 24 jam

        let claims = Claims {
            sub: user_id,
            username: username.to_string(),
            role: role.to_string(),
            exp: expires_at.timestamp() as usize,
            iat: now.timestamp() as usize,
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret.as_ref()),
        )
        .map_err(|_| AuthError::TokenCreationError)
    }

    /// Validate JWT token and extract claims
    pub fn validate_token(token: &str, secret: &str) -> Result<Claims, AuthError> {
        let validation = Validation::new(Algorithm::HS256);
        
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(secret.as_ref()),
            &validation,
        )
        .map_err(|err| match err.kind() {
            jsonwebtoken::errors::ErrorKind::ExpiredSignature => AuthError::TokenExpired,
            _ => AuthError::InvalidToken,
        })?;

        Ok(token_data.claims)
    }

    /// Extract token from Authorization header
    pub fn extract_token_from_header(auth_header: &str) -> Result<&str, AuthError> {
        if auth_header.starts_with("Bearer ") {
            Ok(&auth_header[7..]) // Remove "Bearer " prefix
        } else {
            Err(AuthError::InvalidToken)
        }
    }

    /// Check if user has required role
    pub fn has_role(user_role: &str, required_role: &str) -> bool {
        match (user_role, required_role) {
            ("admin", _) => true, // Admin has access to everything
            (role, required) => role == required,
        }
    }

    /// Check if user can access resource (owner or admin)
    pub fn can_access_resource(user_id: i32, resource_owner_id: i32, user_role: &str) -> bool {
        user_id == resource_owner_id || user_role == "admin"
    }
}