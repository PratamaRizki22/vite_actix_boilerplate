use actix_web::{HttpResponse, Result, web};
use sqlx::PgPool;

use crate::services::token_blacklist::TokenBlacklist;
use crate::services::account_lockout::AccountLockout;
use crate::services::cleanup_service::CleanupService;
use crate::services::email_service::EmailService;

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

pub async fn check_lockout_status(pool: web::Data<PgPool>, user_id: web::Path<i32>) -> Result<HttpResponse> {
    let uid = user_id.into_inner();
    match AccountLockout::is_locked(pool.get_ref(), uid).await {
        Ok(is_locked) => {
            if is_locked {
                match AccountLockout::get_remaining_lockout_seconds(pool.get_ref(), uid).await {
                    Ok(seconds) => {
                        Ok(HttpResponse::Ok().json(serde_json::json!({
                            "user_id": uid,
                            "is_locked": true,
                            "remaining_seconds": seconds,
                            "message": "Account is locked due to failed login attempts"
                        })))
                    }
                    Err(_) => {
                        Ok(HttpResponse::Ok().json(serde_json::json!({
                            "user_id": uid,
                            "is_locked": true,
                            "message": "Account is locked"
                        })))
                    }
                }
            } else {
                Ok(HttpResponse::Ok().json(serde_json::json!({
                    "user_id": uid,
                    "is_locked": false,
                    "message": "Account is not locked"
                })))
            }
        }
        Err(e) => {
            eprintln!("Failed to check lockout status: {}", e);
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to check lockout status"
            })))
        }
    }
}

pub async fn cleanup_unverified_accounts(pool: web::Data<PgPool>) -> Result<HttpResponse> {
    match CleanupService::cleanup_old_unverified_accounts(pool.get_ref()).await {
        Ok(deleted_count) => {
            // Also cleanup expired verification codes
            let codes_cleaned = EmailService::cleanup_expired_codes();

            Ok(HttpResponse::Ok().json(serde_json::json!({
                "message": "Cleaned up unverified accounts and expired codes",
                "unverified_accounts_deleted": deleted_count,
                "expired_codes_cleaned": codes_cleaned
            })))
        }
        Err(e) => {
            eprintln!("Failed to cleanup unverified accounts: {}", e);
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to cleanup unverified accounts"
            })))
        }
    }
}

pub async fn get_unverified_accounts_stats(pool: web::Data<PgPool>) -> Result<HttpResponse> {
    match CleanupService::get_unverified_accounts_stats(pool.get_ref()).await {
        Ok(accounts) => {
            Ok(HttpResponse::Ok().json(serde_json::json!({
                "unverified_accounts": accounts,
                "total_count": accounts.len(),
                "message": "Unverified accounts statistics"
            })))
        }
        Err(e) => {
            eprintln!("Failed to get unverified accounts stats: {}", e);
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to get unverified accounts statistics"
            })))
        }
    }
}
