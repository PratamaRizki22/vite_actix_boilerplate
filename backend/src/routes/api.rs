use actix_web::web;

use crate::auth::account::{add_email, connect_wallet, get_sessions, logout_all_sessions, logout_other_sessions, logout_session};
use crate::auth::debug::{blacklist_stats, cleanup_blacklist, check_lockout_status, cleanup_unverified_accounts, get_unverified_accounts_stats, hash_password_debug};
use crate::auth::email::{debug_codes, send_verification, verify_email, check_code_expiry, send_mfa_code, check_mfa_code_expiry};
use crate::auth::google::google_callback;
use crate::auth::password::{request_password_reset, reset_password, debug_password_reset_tokens, test_email_service, get_rate_limit_stats, change_password};
use crate::auth::security::{setup_2fa, verify_2fa, debug_2fa, disable_2fa};
use crate::auth::traditional::{login, logout, me, register, verify_mfa};
use crate::auth::web3::{web3_challenge, web3_verify};
use crate::handlers::{post, user, interaction};
use crate::middleware::auth::AuthMiddleware;
use crate::middleware::rate_limit_middleware::RateLimitMiddleware;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            // Auth routes (public)
            .service(
                web::scope("/auth")
                    .route("/login", web::post().to(login)
                        .wrap(RateLimitMiddleware::new("login", 5, 300)))
                    .route("/verify-mfa", web::post().to(verify_mfa))
                    .route("/register", web::post().to(register)
                        .wrap(RateLimitMiddleware::new("register", 3, 600)))
                    .route("/google/callback", web::post().to(google_callback))
                    .route(
                        "/setup-2fa",
                        web::post().to(setup_2fa).wrap(AuthMiddleware::new()),
                    )
                    .route(
                        "/verify-2fa",
                        web::post().to(verify_2fa).wrap(AuthMiddleware::new()),
                    )
                    .route(
                        "/debug-2fa",
                        web::get().to(debug_2fa).wrap(AuthMiddleware::new()),
                    )
                    .route(
                        "/disable-2fa",
                        web::post().to(disable_2fa).wrap(AuthMiddleware::new()),
                    )
                    .route("/web3/challenge", web::post().to(web3_challenge))
                    .route("/web3/verify", web::post().to(web3_verify))
                    .route(
                        "/connect-wallet",
                        web::post().to(connect_wallet).wrap(AuthMiddleware::new()),
                    )
                    .route(
                        "/add-email",
                        web::post().to(add_email).wrap(AuthMiddleware::new()),
                    )
                    .route(
                        "/email/send-verification",
                        web::post().to(send_verification)
                            .wrap(RateLimitMiddleware::new("email-verification", 5, 600)),
                    )
                    .route("/email/verify", web::post().to(verify_email)
                        .wrap(RateLimitMiddleware::new("email-verify", 10, 600)))
                    .route("/email/debug-codes", web::get().to(debug_codes))
                    .route("/email/check-expiry", web::post().to(check_code_expiry))
                    .route("/email/send-mfa-code", web::post().to(send_mfa_code))
                    .route("/email/check-mfa-expiry", web::post().to(check_mfa_code_expiry))
                    .route(
                        "/logout",
                        web::post().to(logout).wrap(AuthMiddleware::new()),
                    )
                    .route("/me", web::get().to(me).wrap(AuthMiddleware::new()))
                    .route("/password/request-reset", web::post().to(request_password_reset)
                        .wrap(RateLimitMiddleware::new("password-reset-request", 3, 900)))
                    .route("/password/reset", web::post().to(reset_password)
                        .wrap(RateLimitMiddleware::new("password-reset", 5, 600)))
                    .route("/password/verify-code", web::post().to(crate::auth::password::verify_password_change_code).wrap(AuthMiddleware::new()))
                    .route("/password/change", web::post().to(change_password).wrap(AuthMiddleware::new()))
                    .route("/password/debug-tokens", web::get().to(debug_password_reset_tokens))
                    .route("/password/test-email", web::get().to(test_email_service))
                    .route("/rate-limit-stats", web::get().to(get_rate_limit_stats))
                    .route("/debug/blacklist/stats", web::get().to(blacklist_stats))
                    .route("/debug/blacklist/cleanup", web::post().to(cleanup_blacklist))
                    .route("/debug/cleanup/unverified-accounts", web::post().to(cleanup_unverified_accounts))
                    .route("/debug/cleanup/unverified-stats", web::get().to(get_unverified_accounts_stats))
                    .route("/debug/hash-password", web::post().to(hash_password_debug))
                    // Session management endpoints
                    .route("/sessions", web::get().to(get_sessions).wrap(AuthMiddleware::new()))
                    .route("/sessions/{id}", web::delete().to(logout_session).wrap(AuthMiddleware::new()))
                    .route("/sessions/logout-all", web::post().to(logout_all_sessions).wrap(AuthMiddleware::new()))
                    .route("/sessions/logout-others", web::post().to(logout_other_sessions).wrap(AuthMiddleware::new()))
                    // Profile update endpoint (user can update their own profile)
                    .route("/profile", web::put().to(crate::handlers::user_profile::update_own_profile).wrap(AuthMiddleware::new())),
            )
            // User search (authenticated users only - like Instagram search)
            .service(
                web::scope("/users/search-public")
                    .wrap(AuthMiddleware::new())
                    .route("", web::get().to(user::search_users_public)),
            )
            // User management routes (admin only)
            .service(
                web::scope("/users")
                    .wrap(AuthMiddleware::require_role("admin"))
                    .route("", web::get().to(user::get_users))
                    .route("", web::post().to(user::create_user))
                    .route("/{id}", web::get().to(user::get_user))
                    .route("/{id}", web::put().to(user::update_user))
                    .route("/{id}/ban", web::put().to(user::ban_user))
                    .route("/{id}", web::delete().to(user::delete_user)),
            )
            // Post routes (authenticated users)
            .service(
                web::scope("/posts")
                    .wrap(AuthMiddleware::new())
                    .route("/feed", web::get().to(post::get_all_posts))
                    .route("/search", web::get().to(post::search_posts))
                    .route("", web::get().to(post::get_posts))
                    .route("", web::post().to(post::create_post))
                    .route("/{id}", web::get().to(post::get_post))
                    .route("/{id}", web::put().to(post::update_post))
                    .route("/{id}", web::delete().to(post::delete_post))
                    // Like routes
                    .route("/{id}/like", web::post().to(interaction::toggle_like))
                    .route("/{id}/like/check", web::get().to(interaction::check_like))
                    // Comment routes
                    .route("/{id}/comments", web::post().to(interaction::create_comment))
                    .route("/{id}/comments", web::get().to(interaction::get_comments))
                    .route("/comments/{id}", web::put().to(interaction::update_comment))
                    .route("/comments/{id}", web::delete().to(interaction::delete_comment)),
            )
            // Health check (public)
            .route("/health", web::get().to(health_check)),
    );
}

async fn health_check() -> actix_web::Result<actix_web::HttpResponse> {
    Ok(actix_web::HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now()
    })))
}
