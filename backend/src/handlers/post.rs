use crate::middleware::auth::get_current_user;
use crate::models::post::{CreatePost, Post};
use actix_web::{HttpResponse, Result, web};
use sqlx::PgPool;

pub async fn get_all_posts(
    pool: web::Data<PgPool>,
    query: web::Query<std::collections::HashMap<String, String>>,
) -> Result<HttpResponse> {
    let page = query
        .get("page")
        .and_then(|p| p.parse::<i64>().ok())
        .unwrap_or(1)
        .max(1);
    let limit: i64 = 50; // Changed from unlimited to 50 per page
    let offset = (page - 1) * limit;
    
    // Get total count
    let count_result = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM posts"
    )
    .fetch_one(pool.get_ref())
    .await
    .unwrap_or(0);

    let posts = sqlx::query_as!(
        Post,
        "SELECT id, user_id, title, content, created_at, updated_at FROM posts ORDER BY created_at DESC LIMIT $1 OFFSET $2",
        limit,
        offset
    )
    .fetch_all(pool.get_ref())
    .await
    .map_err(|_| actix_web::error::ErrorInternalServerError("Database error"))?;

    let total_pages = (count_result + limit - 1) / limit;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "data": posts,
        "pagination": {
            "page": page,
            "limit": limit,
            "total": count_result,
            "total_pages": total_pages
        }
    })))
}

pub async fn search_posts(
    pool: web::Data<PgPool>,
    query: web::Query<std::collections::HashMap<String, String>>,
) -> Result<HttpResponse> {
    let search_term = query.get("search").cloned().unwrap_or_default();
    
    if search_term.is_empty() {
        return Ok(HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Search term is required"
        })));
    }

    let search_query = format!("{}:*", search_term.trim());
    
    let posts = sqlx::query_as!(
        Post,
        "SELECT id, user_id, title, content, created_at, updated_at FROM posts 
         WHERE to_tsvector('english', title || ' ' || content) @@ to_tsquery('english', $1)
         ORDER BY ts_rank(to_tsvector('english', title || ' ' || content), to_tsquery('english', $1)) DESC, created_at DESC",
        search_query
    )
    .fetch_all(pool.get_ref())
    .await
    .map_err(|_| actix_web::error::ErrorInternalServerError("Database error"))?;

    Ok(HttpResponse::Ok().json(posts))
}

pub async fn get_posts(
    pool: web::Data<PgPool>,
    req: actix_web::HttpRequest,
) -> Result<HttpResponse> {
    let current_user = get_current_user(&req)
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Not authenticated"))?;

    let posts = sqlx::query_as!(
        Post,
        "SELECT id, user_id, title, content, created_at, updated_at FROM posts WHERE user_id = $1 ORDER BY created_at DESC",
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
         RETURNING id, user_id, title, content, created_at, updated_at",
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
        "SELECT id, user_id, title, content, created_at, updated_at FROM posts WHERE id = $1 AND user_id = $2",
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
