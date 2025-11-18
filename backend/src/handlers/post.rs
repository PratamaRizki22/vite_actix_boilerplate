use crate::middleware::auth::get_current_user;
use crate::models::post::{CreatePost, Post};
use actix_web::{HttpResponse, Result, web};
use sqlx::PgPool;

pub async fn get_posts(
    pool: web::Data<PgPool>,
    req: actix_web::HttpRequest,
) -> Result<HttpResponse> {
    let current_user = get_current_user(&req)
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Not authenticated"))?;

    let posts = sqlx::query_as!(
        Post,
        "SELECT id, user_id, title, content FROM posts WHERE user_id = $1",
        current_user.sub
    )
    .fetch_all(pool.get_ref())
    .await
    .map_err(|_| actix_web::error::ErrorInternalServerError("Database error"))?;

    Ok(HttpResponse::Ok().json(posts))
}

pub async fn create_post(
    pool: web::Data<PgPool>,
    post_data: web::Json<CreatePost>,
    req: actix_web::HttpRequest,
) -> Result<HttpResponse> {
    let current_user = get_current_user(&req)
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Not authenticated"))?;

    let post = sqlx::query_as!(
        Post,
        "INSERT INTO posts (user_id, title, content) VALUES ($1, $2, $3)
         RETURNING id, user_id, title, content",
        current_user.sub,
        post_data.title,
        post_data.content
    )
    .fetch_one(pool.get_ref())
    .await
    .map_err(|_| actix_web::error::ErrorInternalServerError("Database error"))?;

    Ok(HttpResponse::Created().json(post))
}

pub async fn get_post(
    pool: web::Data<PgPool>,
    path: web::Path<i32>,
    req: actix_web::HttpRequest,
) -> Result<HttpResponse> {
    let current_user = get_current_user(&req)
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Not authenticated"))?;

    let post_id = path.into_inner();

    let post = sqlx::query_as!(
        Post,
        "SELECT id, user_id, title, content FROM posts WHERE id = $1 AND user_id = $2",
        post_id,
        current_user.sub
    )
    .fetch_optional(pool.get_ref())
    .await
    .map_err(|_| actix_web::error::ErrorInternalServerError("Database error"))?;

    match post {
        Some(post) => Ok(HttpResponse::Ok().json(post)),
        None => Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": "Post not found"
        }))),
    }
}

pub async fn update_post(
    pool: web::Data<PgPool>,
    path: web::Path<i32>,
    post_data: web::Json<CreatePost>,
    req: actix_web::HttpRequest,
) -> Result<HttpResponse> {
    let current_user = get_current_user(&req)
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Not authenticated"))?;

    let post_id = path.into_inner();

    let result = sqlx::query!(
        "UPDATE posts SET title = $1, content = $2 WHERE id = $3 AND user_id = $4",
        post_data.title,
        post_data.content,
        post_id,
        current_user.sub
    )
    .execute(pool.get_ref())
    .await
    .map_err(|_| actix_web::error::ErrorInternalServerError("Database error"))?;

    if result.rows_affected() == 0 {
        return Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": "Post not found"
        })));
    }

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": "Post updated successfully"
    })))
}

pub async fn delete_post(
    pool: web::Data<PgPool>,
    path: web::Path<i32>,
    req: actix_web::HttpRequest,
) -> Result<HttpResponse> {
    let current_user = get_current_user(&req)
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Not authenticated"))?;

    let post_id = path.into_inner();

    let result = sqlx::query!(
        "DELETE FROM posts WHERE id = $1 AND user_id = $2",
        post_id,
        current_user.sub
    )
    .execute(pool.get_ref())
    .await
    .map_err(|_| actix_web::error::ErrorInternalServerError("Database error"))?;

    if result.rows_affected() == 0 {
        return Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": "Post not found"
        })));
    }

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": "Post deleted successfully"
    })))
}
