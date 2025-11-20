use actix_web::web;

use crate::auth::account::{add_email, connect_wallet, get_sessions, logout_all_sessions, logout_other_sessions, logout_session};
use crate::auth::debug::{blacklist_stats, cleanup_blacklist};
use crate::auth::email::{debug_codes, send_verification, verify_email};
use crate::auth::google::google_callback;
use crate::auth::password::{request_password_reset, reset_password, debug_password_reset_tokens, test_email_service, get_rate_limit_stats};
use crate::auth::security::{setup_2fa, verify_2fa};
use crate::auth::traditional::{login, logout, me, register};
use crate::auth::web3::{web3_challenge, web3_verify};
use crate::handlers::{post, user};
use crate::middleware::auth::AuthMiddleware;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            // Auth routes (public)
            .service(
                web::scope("/auth")
                    .route("/login", web::post().to(login))
                    .route("/register", web::post().to(register))
                    .route("/google/callback", web::post().to(google_callback))
                    .route(
                        "/setup-2fa",
                        web::post().to(setup_2fa).wrap(AuthMiddleware::new()),
                    )
                    .route(
                        "/verify-2fa",
                        web::post().to(verify_2fa).wrap(AuthMiddleware::new()),
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
                        web::post().to(send_verification),
                    )
                    .route("/email/verify", web::post().to(verify_email))
                    .route("/email/debug-codes", web::get().to(debug_codes))
                    .route(
                        "/logout",
                        web::post().to(logout).wrap(AuthMiddleware::new()),
                    )
                    .route("/me", web::get().to(me).wrap(AuthMiddleware::new()))
                    .route("/password/request-reset", web::post().to(request_password_reset))
                    .route("/password/reset", web::post().to(reset_password))
                    .route("/password/debug-tokens", web::get().to(debug_password_reset_tokens))
                    .route("/password/test-email", web::get().to(test_email_service))
                    .route("/rate-limit-stats", web::get().to(get_rate_limit_stats))
                    .route("/debug/blacklist/stats", web::get().to(blacklist_stats))
                    .route("/debug/blacklist/cleanup", web::post().to(cleanup_blacklist))
                    // Session management endpoints
                    .route("/sessions", web::get().to(get_sessions).wrap(AuthMiddleware::new()))
                    .route("/sessions/{id}", web::delete().to(logout_session).wrap(AuthMiddleware::new()))
                    .route("/sessions/logout-all", web::post().to(logout_all_sessions).wrap(AuthMiddleware::new()))
                    .route("/sessions/logout-others", web::post().to(logout_other_sessions).wrap(AuthMiddleware::new())),
            )
            // User routes (admin only)
            .service(
                web::scope("/users")
                    .wrap(AuthMiddleware::require_role("admin"))
                    .route("", web::get().to(user::get_users))
                    .route("/search", web::get().to(user::search_users))
                    .route("", web::post().to(user::create_user))
                    .route("/{id}", web::get().to(user::get_user))
                    .route("/{id}", web::put().to(user::update_user))
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
                    .route("/{id}", web::delete().to(post::delete_post)),
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
