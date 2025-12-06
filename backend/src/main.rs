mod auth;
mod handlers;
mod middleware;
mod models;
mod routes;
mod services;
mod utils;

use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use crate::middleware::security_headers::SecurityHeadersMiddleware;
use crate::middleware::redis_rate_limiter::RedisRateLimiter;
use crate::middleware::redis_token_blacklist::RedisTokenBlacklist;
use crate::middleware::redis_session::RedisSessionStore;
use crate::middleware::redis_cache::RedisCache;
use crate::services::token_blacklist::TokenBlacklistService;
use crate::services::scheduled_tasks::start_scheduled_tasks;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let jwt_secret = std::env::var("JWT_SECRET")
        .expect("JWT_SECRET environment variable must be set");

    let redis_url = std::env::var("REDIS_URL")
        .unwrap_or_else(|_| "redis://127.0.0.1:6379".to_string());

    let pool = PgPoolOptions::new()
        .max_connections(50)  // Increased from 5 to 50 for better concurrency
        .acquire_timeout(std::time::Duration::from_secs(10))
        .connect(&database_url)
        .await
        .expect("Failed to connect database");

    // Initialize Redis services
    let redis_rate_limiter = RedisRateLimiter::new(&redis_url)
        .await
        .expect("Failed to connect to Redis for rate limiting");

    let redis_token_blacklist = RedisTokenBlacklist::new(&redis_url)
        .await
        .expect("Failed to connect to Redis for token blacklist");

    let redis_session_store = RedisSessionStore::new(&redis_url)
        .await
        .expect("Failed to connect to Redis for session storage");

    let redis_cache = RedisCache::new(&redis_url)
        .await
        .expect("Failed to connect to Redis for caching");

    // Start background scheduled cleanup tasks
    start_scheduled_tasks(pool.clone());

    HttpServer::new(move || {
        let cors_origins = std::env::var("CORS_ALLOWED_ORIGINS")
            .unwrap_or_else(|_| "http://localhost:5173".to_string());
        
        let mut cors = Cors::default();
        for origin in cors_origins.split(',') {
            cors = cors.allowed_origin(origin.trim());
        }
        
        cors = cors
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec![
                actix_web::http::header::CONTENT_TYPE,
                actix_web::http::header::AUTHORIZATION,
            ])
            .supports_credentials()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .wrap(SecurityHeadersMiddleware)
            .app_data(actix_web::web::Data::new(pool.clone()))
            .app_data(actix_web::web::Data::new(jwt_secret.clone()))
            .app_data(actix_web::web::Data::new(TokenBlacklistService::new(pool.clone())))
            .app_data(actix_web::web::Data::new(redis_rate_limiter.clone()))
            .app_data(actix_web::web::Data::new(redis_token_blacklist.clone()))
            .app_data(actix_web::web::Data::new(redis_session_store.clone()))
            .app_data(actix_web::web::Data::new(redis_cache.clone()))
            .configure(crate::routes::api::config)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
