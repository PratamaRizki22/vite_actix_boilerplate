mod models;
mod handlers;
mod routes;
mod utils;
mod middleware;
mod auth;


use actix_web::{App, HttpServer};
use actix_web::middleware::Logger;
use actix_cors::Cors;
use sqlx::postgres::PgPoolOptions;
use dotenvy::dotenv;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let jwt_secret = std::env::var("JWT_SECRET")
        .unwrap_or_else(|_| "default-secret-key-change-in-production".to_string());

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect database");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:5173")  
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
            .app_data(actix_web::web::Data::new(pool.clone()))
            .app_data(actix_web::web::Data::new(jwt_secret.clone()))
            .configure(crate::routes::api::config)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}