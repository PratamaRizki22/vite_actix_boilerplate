use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use crate::models::user::{User, CreateUser};

pub async fn get_all(pool: web::Data<PgPool>) -> HttpResponse {
    let users = sqlx::query_as!(User, "Select id, username, email From users")
        .fetch_all(pool.get_ref())
        .await;

    match users {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(e) => HttpResponse::InternalServerError().json(format!("error: {}", e)),
    }
}

pub async fn get_by_id(
    pool: web::Data<PgPool>,
    id: web::Path<i32>,
) -> HttpResponse {
    let user = sqlx::query_as!(
        User,
        "SELECT id, username, email FROM users WHERE id = $1",
        *id
    )
    .fetch_one(pool.get_ref())
    .await;

    match user {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(_) => HttpResponse::NotFound().json("User not found"),
    }
}

pub async fn create(
    pool: web::Data<PgPool>,
    user: web::Json<CreateUser>,
) -> HttpResponse {
    let result = sqlx::query_as!(
        User,
        "INSERT INTO users (username, email) VALUES ($1, $2) RETURNING id, username, email",
        user.username,
        user.email
    )
    .fetch_one(pool.get_ref())
    .await;
    
    match result {
        Ok(data) => HttpResponse::Created().json(data),
        Err(e) => HttpResponse::InternalServerError().json(format!("Error: {}", e)),
    }
}

pub async fn delete(
    pool: web::Data<PgPool>,
    id: web::Path<i32>,
) -> HttpResponse {
    let result = sqlx::query!("DELETE FROM users WHERE id = $1", *id)
        .execute(pool.get_ref())
        .await;
    
    match result {
        Ok(_) => HttpResponse::Ok().json("User deleted"),
        Err(e) => HttpResponse::InternalServerError().json(format!("Error: {}", e)),
    }
}