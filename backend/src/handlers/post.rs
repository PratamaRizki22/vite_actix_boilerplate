use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use crate::models::post::{Post, CreatePost};

pub async fn get_all(pool: web::Data<PgPool>) -> HttpResponse {
    let posts = sqlx::query_as!(
        Post,
        "SELECT id, user_id, title, content FROM posts"
    )
    .fetch_all(pool.get_ref())
    .await;
    
    match posts {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(e) => HttpResponse::InternalServerError().json(format!("Error: {}", e)),
    }
}

pub async fn get_by_user(
    pool: web::Data<PgPool>,
    user_id: web::Path<i32>,
) -> HttpResponse {
    let posts = sqlx::query_as!(
        Post,
        "SELECT id, user_id, title, content FROM posts WHERE user_id = $1",
        *user_id
    )
    .fetch_all(pool.get_ref())
    .await;
    
    match posts {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(_) => HttpResponse::NotFound().json("Posts not found"),
    }
}

pub async fn create(
    pool: web::Data<PgPool>,
    post: web::Json<CreatePost>,
) -> HttpResponse {
    let result = sqlx::query_as!(
        Post,
        "INSERT INTO posts (user_id, title, content) VALUES ($1, $2, $3) 
         RETURNING id, user_id, title, content",
        post.user_id,
        post.title,
        post.content
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
    let result = sqlx::query!("DELETE FROM posts WHERE id = $1", *id)
        .execute(pool.get_ref())
        .await;
    
    match result {
        Ok(_) => HttpResponse::Ok().json("Post deleted"),
        Err(e) => HttpResponse::InternalServerError().json(format!("Error: {}", e)),
    }
}