use actix_web::{HttpResponse, Result, web};
use sqlx::PgPool;

use crate::services::token_blacklist::TokenBlacklist;

pub async fn blacklist_stats(pool: web::Data<PgPool>) -> Result<HttpResponse> {
    match TokenBlacklist::get_stats(pool.get_ref()).await {
        Ok((active, expired)) => {
            Ok(HttpResponse::Ok().json(serde_json::json!({
                "active_blacklisted_tokens": active,
                "expired_blacklisted_tokens": expired,
                "total": active + expired,
                "message": "Token blacklist statistics"
            })))
        }
        Err(e) => {
            eprintln!("Failed to get blacklist stats: {}", e);
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to get blacklist statistics"
            })))
        }
    }
}

pub async fn cleanup_blacklist(pool: web::Data<PgPool>) -> Result<HttpResponse> {
    match TokenBlacklist::cleanup_expired_tokens(pool.get_ref()).await {
        Ok(count) => {
            Ok(HttpResponse::Ok().json(serde_json::json!({
                "message": "Cleaned up expired tokens from blacklist",
                "deleted_count": count
            })))
        }
        Err(e) => {
            eprintln!("Failed to cleanup blacklist: {}", e);
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to cleanup blacklist"
            })))
        }
    }
}
