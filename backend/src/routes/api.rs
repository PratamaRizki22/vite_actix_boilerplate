use actix_web::web;

use crate::handlers::{user, post};
use crate::auth::auth_handlers as auth;
use crate::middleware::auth::AuthMiddleware;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            // Auth routes (public)
            .service(
                web::scope("/auth")
                    .route("/login", web::post().to(auth::login))
                    .route("/logout", web::post().to(auth::logout).wrap(AuthMiddleware::new()))
                    .route("/me", web::get().to(auth::me).wrap(AuthMiddleware::new()))
            )
            // User routes (admin only)
            .service(
                web::scope("/users")
                    .wrap(AuthMiddleware::require_role("admin"))
                    .route("", web::get().to(user::get_users))
                    .route("", web::post().to(user::create_user))
                    .route("/{id}", web::get().to(user::get_user))
                    .route("/{id}", web::put().to(user::update_user))
                    .route("/{id}", web::delete().to(user::delete_user))
            )
            // Post routes (authenticated users)
            .service(
                web::scope("/posts")
                    .wrap(AuthMiddleware::new())
                    .route("", web::get().to(post::get_posts))
                    .route("", web::post().to(post::create_post))
                    .route("/{id}", web::get().to(post::get_post))
                    .route("/{id}", web::put().to(post::update_post))
                    .route("/{id}", web::delete().to(post::delete_post))
            )
            // Health check (public)
            .route("/health", web::get().to(health_check))
    );
}

async fn health_check() -> actix_web::Result<actix_web::HttpResponse> {
    Ok(actix_web::HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now()
    })))
}