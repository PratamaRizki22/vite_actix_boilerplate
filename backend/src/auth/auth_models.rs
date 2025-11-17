use serde::{Deserialize, Serialize};
use crate::models::user::UserResponse;

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub user: UserResponse,
}

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct TOTPSetupResponse {
    pub secret: String,
    pub qr_code_url: String,
}

#[derive(Debug, Deserialize)]
pub struct TOTPVerifyRequest {
    pub code: String,
}

#[derive(Debug, Serialize)]
pub struct TOTPVerifyResponse {
    pub success: bool,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)] // ADD Clone
pub struct Claims {
    pub sub: i32,
    pub username: String,
    pub role: String,
    pub exp: usize,
    pub iat: usize,
}