use actix_web::{HttpRequest, HttpResponse, Result, web};
use serde_json;
use sqlx::PgPool;

use crate::middleware::auth::get_current_user;
use crate::models::auth::Web3ChallengeRequest;
use crate::services::session_manager::SessionManager;

pub async fn connect_wallet(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    wallet_data: web::Json<Web3ChallengeRequest>,
) -> Result<HttpResponse> {
    let current_user = get_current_user(&req)
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Not authenticated"))?;

    // Check if wallet address is already connected to another user
    let existing_user = sqlx::query!(
        "SELECT id FROM users WHERE wallet_address = $1 AND id != $2",
        wallet_data.address,
        current_user.sub
    )
    .fetch_optional(pool.get_ref())
    .await
    .map_err(|_| actix_web::error::ErrorInternalServerError("Database error"))?;

    if existing_user.is_some() {
        return Ok(HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Wallet address already connected to another account"
        })));
    }

    // Update user's wallet address
    sqlx::query!(
        "UPDATE users SET wallet_address = $1, updated_at = NOW() WHERE id = $2",
        wallet_data.address,
        current_user.sub
    )
    .execute(pool.get_ref())
    .await
    .map_err(|_| actix_web::error::ErrorInternalServerError("Failed to connect wallet"))?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": "Wallet connected successfully",
        "wallet_address": wallet_data.address
    })))
}

pub async fn add_email(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    email_data: web::Json<serde_json::Value>,
) -> Result<HttpResponse> {
    let current_user = get_current_user(&req)
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Not authenticated"))?;

    let email = email_data
        .get("email")
        .and_then(|v| v.as_str())
        .ok_or_else(|| actix_web::error::ErrorBadRequest("Email is required"))?;

    // Check if email is already taken
    let existing_user = sqlx::query!(
        "SELECT id FROM users WHERE email = $1 AND id != $2",
        email,
        current_user.sub
    )
    .fetch_optional(pool.get_ref())
    .await
    .map_err(|_| actix_web::error::ErrorInternalServerError("Database error"))?;

    if existing_user.is_some() {
        return Ok(HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Email already taken"
        })));
    }

    // Update user's email
    sqlx::query!(
        "UPDATE users SET email = $1, updated_at = NOW() WHERE id = $2",
        email,
        current_user.sub
    )
    .execute(pool.get_ref())
    .await
    .map_err(|_| actix_web::error::ErrorInternalServerError("Failed to add email"))?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": "Email added successfully",
        "email": email
    })))
}

// Session Management Endpoints

pub async fn get_sessions(
    pool: web::Data<PgPool>,
    req: HttpRequest,
) -> Result<HttpResponse> {
    let current_user = get_current_user(&req)
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Not authenticated"))?;

    let sessions = SessionManager::get_user_sessions(pool.get_ref(), current_user.sub)
        .await
        .map_err(|_| actix_web::error::ErrorInternalServerError("Failed to fetch sessions"))?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "sessions": sessions,
        "count": sessions.len()
    })))
}

pub async fn logout_session(
    pool: web::Data<PgPool>,
    path: web::Path<i32>,
    req: HttpRequest,
) -> Result<HttpResponse> {
    let current_user = get_current_user(&req)
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Not authenticated"))?;

    let session_id = path.into_inner();

    // Verify session belongs to current user
    let session = sqlx::query!(
        "SELECT user_id FROM sessions WHERE id = $1",
        session_id
    )
    .fetch_optional(pool.get_ref())
    .await
    .map_err(|_| actix_web::error::ErrorInternalServerError("Database error"))?;

    let session = match session {
        Some(s) => s,
        None => return Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": "Session not found"
        }))),
    };

    if session.user_id != current_user.sub {
        return Ok(HttpResponse::Forbidden().json(serde_json::json!({
            "error": "Cannot logout other user's session"
        })));
    }

    // Invalidate session
    SessionManager::invalidate_session(pool.get_ref(), session_id)
        .await
        .map_err(|_| actix_web::error::ErrorInternalServerError("Failed to logout session"))?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": "Session logged out successfully"
    })))
}

pub async fn logout_all_sessions(
    pool: web::Data<PgPool>,
    req: HttpRequest,
) -> Result<HttpResponse> {
    let current_user = get_current_user(&req)
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Not authenticated"))?;

    // Get current session ID from token
    let current_session_id = if let Some(auth_header) = req.headers().get("Authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            if let Ok(token) = crate::utils::auth::AuthUtils::extract_token_from_header(auth_str) {
                if let Ok(Some(session)) = SessionManager::get_session_by_token(pool.get_ref(), token).await {
                    Some(session.id)
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        }
    } else {
        None
    };

    // Invalidate all sessions
    SessionManager::invalidate_all_sessions(pool.get_ref(), current_user.sub)
        .await
        .map_err(|_| actix_web::error::ErrorInternalServerError("Failed to logout all sessions"))?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": "All sessions logged out successfully"
    })))
}

pub async fn logout_other_sessions(
    pool: web::Data<PgPool>,
    req: HttpRequest,
) -> Result<HttpResponse> {
    let current_user = get_current_user(&req)
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Not authenticated"))?;

    // Get current session ID from token
    let current_session_id = if let Some(auth_header) = req.headers().get("Authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            if let Ok(token) = crate::utils::auth::AuthUtils::extract_token_from_header(auth_str) {
                if let Ok(Some(session)) = SessionManager::get_session_by_token(pool.get_ref(), token).await {
                    session.id
                } else {
                    return Ok(HttpResponse::BadRequest().json(serde_json::json!({
                        "error": "Current session not found"
                    })));
                }
            } else {
                return Ok(HttpResponse::BadRequest().json(serde_json::json!({
                    "error": "Invalid token format"
                })));
            }
        } else {
            return Ok(HttpResponse::BadRequest().json(serde_json::json!({
                "error": "Authorization header missing"
            })));
        }
    } else {
        return Ok(HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Authorization header missing"
        })));
    };

    // Invalidate all other sessions
    SessionManager::invalidate_other_sessions(pool.get_ref(), current_user.sub, current_session_id)
        .await
        .map_err(|_| actix_web::error::ErrorInternalServerError("Failed to logout other sessions"))?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": "All other sessions logged out successfully"
    })))
}
