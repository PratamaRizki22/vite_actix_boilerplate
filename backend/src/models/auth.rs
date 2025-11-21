use crate::models::user::UserResponse;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub token: Option<String>,
    pub refresh_token: Option<String>,
    pub user: UserResponse,
    pub requires_mfa: bool,
    pub mfa_methods: Option<Vec<String>>, // ["totp", "email"]
    pub temp_token: Option<String>, // Token untuk sementara untuk MFA verification
}

#[derive(Debug, Deserialize)]
pub struct MFAVerifyRequest {
    pub temp_token: String,
    pub method: String, // "totp" atau "email"
    pub code: String,
}

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub first_name: String,
    pub email: Option<String>,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct TOTPSetupResponse {
    pub secret: String,
    pub qr_code_url: String,
    pub recovery_codes: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct TOTPSetupRequiredResponse {
    pub setup_required: bool,
    pub secret: String,
    pub qr_code_url: String,
    pub message: String,
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

#[derive(Debug, Deserialize)]
pub struct Web3ChallengeRequest {
    pub address: String,
}

#[derive(Debug, Serialize)]
pub struct Web3ChallengeResponse {
    pub challenge: String,
    pub expires_at: u64,
}

#[derive(Debug, Deserialize)]
pub struct Web3VerifyRequest {
    pub address: String,
    pub signature: String,
    pub challenge: String,
}

#[derive(Debug, Serialize)]
pub struct Web3VerifyResponse {
    pub success: bool,
    pub token: Option<String>,
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

#[derive(Debug, Deserialize)]
pub struct PasswordResetRequest {
    pub email: String,
}

#[derive(Debug, Deserialize)]
pub struct PasswordResetConfirm {
    pub token: String,
    pub new_password: String,
}

#[derive(Debug, Serialize)]
pub struct PasswordResetResponse {
    pub success: bool,
    pub message: String,
}

#[derive(Debug, Deserialize)]
pub struct ChangePasswordRequest {
    pub current_password: String,
    pub new_password: String,
}
