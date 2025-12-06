use actix_web::{HttpResponse, Result, web};
use sqlx::PgPool;

use crate::middleware::auth::get_current_user;
use crate::middleware::redis_cache::RedisCache;
use crate::models::user::{CreateUser, UpdateUser, User, UserResponse};
use crate::utils::auth::AuthUtils;
use crate::utils::validation::{validate_username, validate_email, validate_password};

pub async fn get_users(pool: web::Data<PgPool>) -> Result<HttpResponse> {
    let users = sqlx::query_as!(
        User,
        "SELECT id, username, email, password, role, wallet_address, email_verified, totp_enabled, recovery_codes, is_banned, banned_until, last_login, created_at, updated_at 
         FROM users
         ORDER BY created_at DESC"
    )
    .fetch_all(pool.get_ref())
    .await
    .map_err(|_| actix_web::error::ErrorInternalServerError("Database error"))?;

    let user_responses: Vec<UserResponse> = users.into_iter().map(UserResponse::from).collect();

    Ok(HttpResponse::Ok().json(user_responses))
}

pub async fn get_user(
    path: web::Path<i32>, 
    pool: web::Data<PgPool>,
    redis: web::Data<RedisCache>,
) -> Result<HttpResponse> {
    let user_id = path.into_inner();

    // Try to get from cache first
    if let Ok(Some(cached_user)) = redis.get_user::<UserResponse>(user_id).await {
        return Ok(HttpResponse::Ok().json(cached_user));
    }

    let user = sqlx::query_as!(
        User,
        "SELECT id, username, email, password, role, wallet_address, email_verified, totp_enabled, recovery_codes, is_banned, banned_until, last_login, created_at, updated_at FROM users WHERE id = $1",
        user_id
    )
    .fetch_optional(pool.get_ref())
    .await
    .map_err(|_| actix_web::error::ErrorInternalServerError("Database error"))?;

    match user {
        Some(user) => {
            let user_response = UserResponse::from(user);
            // Cache the user data
            let _ = redis.cache_user(user_id, &user_response).await;
            Ok(HttpResponse::Ok().json(user_response))
        },
        None => Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": "User not found"
        }))),
    }
}

pub async fn create_user(
    pool: web::Data<PgPool>,
    user_data: web::Json<CreateUser>,
) -> Result<HttpResponse> {
    // Validate input
    if let Err(e) = validate_username(&user_data.username) {
        return Ok(HttpResponse::BadRequest().json(serde_json::json!({
            "error": format!("Invalid username: {}", e)
        })));
    }

    if let Some(email) = &user_data.email {
        if let Err(e) = validate_email(email) {
            return Ok(HttpResponse::BadRequest().json(serde_json::json!({
                "error": format!("Invalid email: {}", e)
            })));
        }
    }

    if let Err(e) = validate_password(&user_data.password) {
        return Ok(HttpResponse::BadRequest().json(serde_json::json!({
            "error": format!("Invalid password: {}", e)
        })));
    }

    let role = user_data.role.as_deref().unwrap_or("user");

    // Hash password before storing
    let hashed_password = AuthUtils::hash_password(&user_data.password)
        .map_err(|_| actix_web::error::ErrorInternalServerError("Password hashing failed"))?;

    let user = sqlx::query_as!(
        User,
        "INSERT INTO users (username, email, password, role, email_verified)
         VALUES ($1, $2, $3, $4, false)
         RETURNING id, username, email, password, role, wallet_address, email_verified, totp_enabled, recovery_codes, is_banned, banned_until, last_login, created_at, updated_at",
        user_data.username,
        user_data.email,
        hashed_password,
        role
    )
    .fetch_one(pool.get_ref())
    .await
    .map_err(|err| {
        if let sqlx::Error::Database(db_err) = &err {
            if db_err.constraint().is_some() {
                return actix_web::error::ErrorBadRequest("Username or email already exists");
            }
        }
        actix_web::error::ErrorInternalServerError("Database error")
    })?;

    Ok(HttpResponse::Created().json(UserResponse::from(user)))
}

pub async fn update_user(
    req: actix_web::HttpRequest,
    path: web::Path<i32>,
    pool: web::Data<PgPool>,
    redis: web::Data<RedisCache>,
    user_data: web::Json<UpdateUser>,
) -> Result<HttpResponse> {
    let user_id = path.into_inner();
    let current_user = get_current_user(&req)
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Not authenticated"))?;

    // Check if user can update (owner or admin)
    if !AuthUtils::can_access_resource(current_user.sub, user_id, &current_user.role) {
        return Ok(HttpResponse::Forbidden().json(serde_json::json!({
            "error": "Insufficient permissions"
        })));
    }

    // Invalidate user cache before update
    let _ = redis.invalidate_user(user_id).await;

    // Update user with provided fields
    let mut has_updates = false;

    // Update username if provided
    if let Some(username) = &user_data.username {
        sqlx::query!(
            "UPDATE users SET username = $1, updated_at = NOW() WHERE id = $2",
            username,
            user_id
        )
        .execute(pool.get_ref())
        .await
        .map_err(|_| actix_web::error::ErrorInternalServerError("Database error"))?;
        has_updates = true;
    }

    // Update email if provided
    if let Some(email) = &user_data.email {
        sqlx::query!(
            "UPDATE users SET email = $1, updated_at = NOW() WHERE id = $2",
            email,
            user_id
        )
        .execute(pool.get_ref())
        .await
        .map_err(|_| actix_web::error::ErrorInternalServerError("Database error"))?;
        has_updates = true;
    }

    // Update password if provided
    if let Some(password) = &user_data.password {
        let hashed_password = AuthUtils::hash_password(password)
            .map_err(|_| actix_web::error::ErrorInternalServerError("Password hashing failed"))?;
        sqlx::query!(
            "UPDATE users SET password = $1, updated_at = NOW() WHERE id = $2",
            hashed_password,
            user_id
        )
        .execute(pool.get_ref())
        .await
        .map_err(|_| actix_web::error::ErrorInternalServerError("Database error"))?;
        has_updates = true;
    }

    // Update role if provided (admin only)
    if let Some(role) = &user_data.role {
        if current_user.role != "admin" {
            return Ok(HttpResponse::Forbidden().json(serde_json::json!({
                "error": "Only admins can update roles"
            })));
        }
        sqlx::query!(
            "UPDATE users SET role = $1, updated_at = NOW() WHERE id = $2",
            role,
            user_id
        )
        .execute(pool.get_ref())
        .await
        .map_err(|_| actix_web::error::ErrorInternalServerError("Database error"))?;
        has_updates = true;
    }

    if !has_updates {
        return Ok(HttpResponse::BadRequest().json(serde_json::json!({
            "error": "No fields to update"
        })));
    }

    // Get updated user
    let user = sqlx::query_as!(
        User,
        "SELECT id, username, email, password, role, wallet_address, email_verified, totp_enabled, recovery_codes, is_banned, banned_until, last_login, created_at, updated_at
         FROM users WHERE id = $1",
        user_id
    )
    .fetch_optional(pool.get_ref())
    .await
    .map_err(|_| actix_web::error::ErrorInternalServerError("Database error"))?;

    match user {
        Some(user) => Ok(HttpResponse::Ok().json(UserResponse::from(user))),
        None => Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": "User not found"
        }))),
    }
}

pub async fn delete_user(
    req: actix_web::HttpRequest,
    path: web::Path<i32>,
    pool: web::Data<PgPool>,
    redis: web::Data<RedisCache>,
) -> Result<HttpResponse> {
    let user_id = path.into_inner();
    let current_user = get_current_user(&req)
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Not authenticated"))?;

    // Check if user can delete (owner or admin)
    if !AuthUtils::can_access_resource(current_user.sub, user_id, &current_user.role) {
        return Ok(HttpResponse::Forbidden().json(serde_json::json!({
            "error": "Insufficient permissions"
        })));
    }

    // Invalidate user cache before delete
    let _ = redis.invalidate_user(user_id).await;

    let result = sqlx::query!("DELETE FROM users WHERE id = $1", user_id)
        .execute(pool.get_ref())
        .await
        .map_err(|e| {
            eprintln!("DELETE USER ERROR: Failed to delete user {}: {:?}", user_id, e);
            actix_web::error::ErrorInternalServerError(format!("Database error: {}", e))
        })?;

    if result.rows_affected() > 0 {
        eprintln!("DELETE USER SUCCESS: User {} deleted", user_id);
        Ok(HttpResponse::NoContent().finish())
    } else {
        eprintln!("DELETE USER: User {} not found", user_id);
        Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": "User not found"
        })))
    }
}

pub async fn search_users(
    pool: web::Data<PgPool>,
    query: web::Query<std::collections::HashMap<String, String>>,
) -> Result<HttpResponse> {
    let search_term = query.get("search").cloned().unwrap_or_default();
    let page = query
        .get("page")
        .and_then(|p| p.parse::<i64>().ok())
        .unwrap_or(1)
        .max(1);
    let limit: i64 = 20;
    let offset = (page - 1) * limit;
    
    if search_term.is_empty() {
        return Ok(HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Search term is required"
        })));
    }

    // Use full-text search with to_tsquery for better performance
    let search_query = format!("{}:*", search_term.trim());
    
    // Get total count
    let count_result = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM users WHERE to_tsvector('english', username) @@ to_tsquery('english', $1)"
    )
    .bind(&search_query)
    .fetch_one(pool.get_ref())
    .await
    .unwrap_or(0);

    // Get paginated results with ranking
    let users = sqlx::query_as!(
        User,
        "SELECT id, username, email, password, role, wallet_address, email_verified, totp_enabled, recovery_codes, is_banned, banned_until, last_login, created_at, updated_at 
         FROM users 
         WHERE to_tsvector('english', username) @@ to_tsquery('english', $1)
         ORDER BY ts_rank(to_tsvector('english', username), to_tsquery('english', $1)) DESC, username ASC
         LIMIT $2 OFFSET $3",
        search_query,
        limit,
        offset
    )
    .fetch_all(pool.get_ref())
    .await
    .map_err(|_| actix_web::error::ErrorInternalServerError("Database error"))?;

    let user_responses: Vec<UserResponse> = users.into_iter().map(UserResponse::from).collect();
    
    let total_pages = (count_result + limit - 1) / limit;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "data": user_responses,
        "pagination": {
            "page": page,
            "limit": limit,
            "total": count_result,
            "total_pages": total_pages
        }
    })))
}

pub async fn search_users_public(
    pool: web::Data<PgPool>,
    query: web::Query<std::collections::HashMap<String, String>>,
) -> Result<HttpResponse> {
    let search_term = query.get("search").cloned().unwrap_or_default();
    
    if search_term.is_empty() {
        return Ok(HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Search term is required"
        })));
    }

    // Use full-text search with to_tsquery for better performance
    let search_query = format!("{}:*", search_term.trim());
    
    // Get results with limit to 10 for quick search (like Instagram)
    let limit: i64 = 10;
    
    let users = sqlx::query!(
        "SELECT id, username FROM users 
         WHERE to_tsvector('english', username) @@ to_tsquery('english', $1)
         ORDER BY ts_rank(to_tsvector('english', username), to_tsquery('english', $1)) DESC, username ASC
         LIMIT $2",
        search_query,
        limit
    )
    .fetch_all(pool.get_ref())
    .await
    .map_err(|_| actix_web::error::ErrorInternalServerError("Database error"))?;

    let results: Vec<serde_json::Value> = users.into_iter().map(|user| {
        serde_json::json!({
            "id": user.id,
            "username": user.username
        })
    }).collect();

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "results": results,
        "count": results.len()
    })))
}

use serde::Deserialize;

#[derive(Deserialize)]
pub struct BanUserRequest {
    pub is_banned: bool,
    pub ban_days: Option<i64>, // Number of days to ban (None = permanent)
}

pub async fn ban_user(
    req: actix_web::HttpRequest,
    path: web::Path<i32>,
    pool: web::Data<PgPool>,
    redis: web::Data<RedisCache>,
    ban_data: web::Json<BanUserRequest>,
) -> Result<HttpResponse> {
    let user_id = path.into_inner();
    let current_user = get_current_user(&req)
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Not authenticated"))?;

    // Only admin can ban users
    if current_user.role != "admin" {
        return Ok(HttpResponse::Forbidden().json(serde_json::json!({
            "error": "Only admins can ban users"
        })));
    }

    // Get user to check if they're an admin
    let target_user = sqlx::query!("SELECT role FROM users WHERE id = $1", user_id)
        .fetch_optional(pool.get_ref())
        .await
        .map_err(|_| actix_web::error::ErrorInternalServerError("Database error"))?;

    match target_user {
        Some(user) => {
            // Prevent banning admins
            if user.role == "admin" {
                return Ok(HttpResponse::Forbidden().json(serde_json::json!({
                    "error": "Cannot ban admin users"
                })));
            }
        }
        None => {
            return Ok(HttpResponse::NotFound().json(serde_json::json!({
                "error": "User not found"
            })));
        }
    }

    // Invalidate user cache before update
    let _ = redis.invalidate_user(user_id).await;

    // Calculate banned_until timestamp
    use chrono::{Utc, Duration};
    let banned_until = if ban_data.is_banned {
        ban_data.ban_days.map(|days| {
            (Utc::now() + Duration::days(days)).naive_utc()
        })
    } else {
        None
    };

    // Update is_banned status and banned_until
    sqlx::query!(
        "UPDATE users SET is_banned = $1, banned_until = $2, updated_at = NOW() WHERE id = $3",
        ban_data.is_banned,
        banned_until,
        user_id
    )
    .execute(pool.get_ref())
    .await
    .map_err(|_| actix_web::error::ErrorInternalServerError("Database error"))?;

    // Get updated user
    let user = sqlx::query_as!(
        User,
        "SELECT id, username, email, password, role, wallet_address, email_verified, totp_enabled, recovery_codes, is_banned, banned_until, last_login, created_at, updated_at
         FROM users WHERE id = $1",
        user_id
    )
    .fetch_optional(pool.get_ref())
    .await
    .map_err(|_| actix_web::error::ErrorInternalServerError("Database error"))?;

    match user {
        Some(user) => {
            Ok(HttpResponse::Ok().json(UserResponse::from(user)))
        }
        None => Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": "User not found"
        }))),
    }
}
