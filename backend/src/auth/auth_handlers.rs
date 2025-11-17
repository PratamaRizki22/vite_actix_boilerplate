use actix_web::{web, HttpRequest, HttpResponse, Result};
use sqlx::PgPool;
use serde_json;

use crate::auth::auth_models::{LoginRequest, LoginResponse};
use crate::models::user::{User, UserResponse};
use crate::auth::auth_utils::AuthUtils;
use crate::middleware::auth::get_current_user;

pub async fn login(
    pool: web::Data<PgPool>,
    jwt_secret: web::Data<String>,
    login_data: web::Json<LoginRequest>,
) -> Result<HttpResponse> {
    // Find user by username
    let user = sqlx::query_as!(
        User,
        "SELECT id, username, email, password, role, created_at, updated_at
         FROM users WHERE username = $1",
        login_data.username
    )
    .fetch_optional(pool.get_ref())
    .await
    .map_err(|_| actix_web::error::ErrorInternalServerError("Database error"))?;

    let user = match user {
        Some(user) => user,
        None => return Ok(HttpResponse::Unauthorized().json(serde_json::json!({
            "error": "Invalid credentials"
        }))),
    };

    // Verify password with bcrypt
    let is_valid_password = AuthUtils::verify_password(&login_data.password, &user.password)
        .map_err(|_| actix_web::error::ErrorInternalServerError("Password verification failed"))?;

    if !is_valid_password {
        return Ok(HttpResponse::Unauthorized().json(serde_json::json!({
            "error": "Invalid credentials"
        })));
    }

    // Create JWT token
    let token = AuthUtils::create_token(
        user.id,
        &user.username,
        &user.role,
        jwt_secret.get_ref(),
    )
    .map_err(|_| actix_web::error::ErrorInternalServerError("Token creation failed"))?;

    let response = LoginResponse {
        token,
        user: UserResponse::from(user),
    };

    Ok(HttpResponse::Ok().json(response))
}

pub async fn logout(req: HttpRequest) -> Result<HttpResponse> {
    let _current_user = get_current_user(&req)
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Not authenticated"))?;

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