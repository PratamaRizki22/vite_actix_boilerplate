use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: Option<String>,
    #[serde(skip_serializing)]
    pub password: String,
    pub role: String,
    pub wallet_address: Option<String>,
    pub email_verified: bool,
    pub totp_enabled: Option<bool>,
    #[serde(skip_serializing)]
    #[sqlx(default)]
    pub recovery_codes: Option<Vec<String>>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Deserialize)]
pub struct CreateUser {
    pub username: String,
    pub email: Option<String>,
    pub password: String,
    pub role: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUser {
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub role: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: i32,
    pub username: String,
    pub email: Option<String>,
    pub role: String,
    pub wallet_address: Option<String>,
    pub email_verified: bool,
    pub two_factor_enabled: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            username: user.username,
            email: user.email,
            role: user.role,
            wallet_address: user.wallet_address,
            email_verified: user.email_verified,
            two_factor_enabled: user.totp_enabled.unwrap_or(false),
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}
