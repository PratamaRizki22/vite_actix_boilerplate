use actix_web::{web, HttpResponse, HttpRequest};
use sqlx::{PgPool, Row};
use crate::models::interaction::{CreateCommentRequest, CommentWithUser, UpdateCommentRequest};
use crate::middleware::auth::get_current_user;

// Like handlers
pub async fn toggle_like(
    db: web::Data<PgPool>,
    req: HttpRequest,
    post_id_param: web::Path<i32>,
) -> HttpResponse {
    let current_user = match get_current_user(&req) {
        Some(user) => user,
        None => return HttpResponse::Unauthorized().json("Not authenticated"),
    };

    let user_id = current_user.sub;
    let post_id = post_id_param.into_inner();

    // Start transaction
    let mut tx = match db.begin().await {
        Ok(tx) => tx,
        Err(_) => return HttpResponse::InternalServerError().json("Database error"),
    };

    // Check if like exists
    let existing_like = sqlx::query_scalar::<_, i32>(
        "SELECT id FROM likes WHERE post_id = $1 AND user_id = $2"
    )
    .bind(post_id)
    .bind(user_id)
    .fetch_optional(&mut *tx)
    .await;

    match existing_like {
        Ok(Some(_)) => {
            // Remove like
            if let Err(_) = sqlx::query("DELETE FROM likes WHERE post_id = $1 AND user_id = $2")
                .bind(post_id)
                .bind(user_id)
                .execute(&mut *tx)
                .await
            {
                return HttpResponse::InternalServerError().json("Failed to unlike");
            }

            // Decrement likes_count
            if let Err(_) = sqlx::query("UPDATE posts SET likes_count = GREATEST(likes_count - 1, 0) WHERE id = $1")
                .bind(post_id)
                .execute(&mut *tx)
                .await
            {
                return HttpResponse::InternalServerError().json("Failed to update post");
            }

            // Commit transaction
            if let Err(_) = tx.commit().await {
                return HttpResponse::InternalServerError().json("Failed to commit transaction");
            }

            // Fetch updated count
            let likes_count = sqlx::query_scalar::<_, i32>("SELECT likes_count FROM posts WHERE id = $1")
                .bind(post_id)
                .fetch_optional(db.get_ref())
                .await
                .unwrap_or(Some(0))
                .unwrap_or(0);

            HttpResponse::Ok().json(serde_json::json!({ 
                "liked": false,
                "likes_count": likes_count
            }))
        }
        Ok(None) => {
            // Add like
            if let Err(_) = sqlx::query("INSERT INTO likes (post_id, user_id) VALUES ($1, $2)")
                .bind(post_id)
                .bind(user_id)
                .execute(&mut *tx)
                .await
            {
                return HttpResponse::InternalServerError().json("Failed to like");
            }

            // Increment likes_count
            if let Err(_) = sqlx::query("UPDATE posts SET likes_count = likes_count + 1 WHERE id = $1")
                .bind(post_id)
                .execute(&mut *tx)
                .await
            {
                return HttpResponse::InternalServerError().json("Failed to update post");
            }

            // Commit transaction
            if let Err(_) = tx.commit().await {
                return HttpResponse::InternalServerError().json("Failed to commit transaction");
            }

            // Fetch updated count
            let likes_count = sqlx::query_scalar::<_, i32>("SELECT likes_count FROM posts WHERE id = $1")
                .bind(post_id)
                .fetch_optional(db.get_ref())
                .await
                .unwrap_or(Some(0))
                .unwrap_or(0);

            HttpResponse::Ok().json(serde_json::json!({ 
                "liked": true,
                "likes_count": likes_count
            }))
        }
        Err(_) => HttpResponse::InternalServerError().json("Database error"),
    }
}

pub async fn check_like(
    db: web::Data<PgPool>,
    req: HttpRequest,
    post_id_param: web::Path<i32>,
) -> HttpResponse {
    let current_user = match get_current_user(&req) {
        Some(user) => user,
        None => return HttpResponse::Unauthorized().json("Not authenticated"),
    };

    let user_id = current_user.sub;
    let post_id = post_id_param.into_inner();

    match sqlx::query_scalar::<_, i32>(
        "SELECT id FROM likes WHERE post_id = $1 AND user_id = $2"
    )
    .bind(post_id)
    .bind(user_id)
    .fetch_optional(db.get_ref())
    .await
    {
        Ok(Some(_)) => HttpResponse::Ok().json(serde_json::json!({ "liked": true })),
        Ok(None) => HttpResponse::Ok().json(serde_json::json!({ "liked": false })),
        Err(_) => HttpResponse::InternalServerError().json("Database error"),
    }
}

// Comment handlers
pub async fn create_comment(
    db: web::Data<PgPool>,
    req: HttpRequest,
    post_id_param: web::Path<i32>,
    body: web::Json<CreateCommentRequest>,
) -> HttpResponse {
    let current_user = match get_current_user(&req) {
        Some(user) => user,
        None => return HttpResponse::Unauthorized().json("Not authenticated"),
    };

    let user_id = current_user.sub;
    let post_id = post_id_param.into_inner();

    if body.content.trim().is_empty() {
        return HttpResponse::BadRequest().json("Comment cannot be empty");
    }

    // Get username first
    let username = match sqlx::query_scalar::<_, String>(
        "SELECT username FROM users WHERE id = $1"
    )
    .bind(user_id)
    .fetch_optional(db.get_ref())
    .await
    {
        Ok(Some(name)) => name,
        _ => return HttpResponse::InternalServerError().json("Failed to fetch user info"),
    };

    match sqlx::query(
        "INSERT INTO comments (post_id, user_id, parent_comment_id, content) VALUES ($1, $2, $3, $4)"
    )
    .bind(post_id)
    .bind(user_id)
    .bind(body.parent_comment_id)
    .bind(body.content.trim())
    .execute(db.get_ref())
    .await
    {
        Ok(_) => {
            // Increment comments_count
            let _ = sqlx::query("UPDATE posts SET comments_count = comments_count + 1 WHERE id = $1")
                .bind(post_id)
                .execute(db.get_ref())
                .await;
            
            HttpResponse::Ok().json(serde_json::json!({
                "post_id": post_id,
                "user_id": user_id,
                "username": username,
                "content": body.content.trim(),
                "parent_comment_id": body.parent_comment_id,
                "created_at": chrono::Utc::now()
            }))
        }
        Err(e) => {
            eprintln!("Failed to create comment: {:?}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to create comment",
                "details": format!("{:?}", e)
            }))
        }
    }
}

pub async fn get_comments(
    db: web::Data<PgPool>,
    post_id_param: web::Path<i32>,
) -> HttpResponse {
    let post_id = post_id_param.into_inner();

    // Query without struct deserialization to handle NULL values gracefully
    match sqlx::query(
        "SELECT c.id, c.post_id, c.user_id, COALESCE(u.username, 'Unknown') as username, 
                c.parent_comment_id, c.content, c.created_at, c.updated_at
         FROM comments c
         LEFT JOIN users u ON c.user_id = u.id
         WHERE c.post_id = $1
         ORDER BY c.created_at ASC"
    )
    .bind(post_id)
    .fetch_all(db.get_ref())
    .await
    {
        Ok(rows) => {
            let mut comments = Vec::new();
            eprintln!("Fetched {} rows from comments", rows.len());
            for row in rows {
                match (
                    row.try_get::<i32, _>(0),
                    row.try_get::<i32, _>(1),
                    row.try_get::<i32, _>(2),
                    row.try_get::<String, _>(3),
                    row.try_get::<Option<i32>, _>(4),
                    row.try_get::<String, _>(5),
                    row.try_get::<chrono::NaiveDateTime, _>(6),
                    row.try_get::<chrono::NaiveDateTime, _>(7),
                ) {
                    (Ok(id), Ok(post_id), Ok(user_id), Ok(username), Ok(parent_comment_id), Ok(content), Ok(created_at), Ok(updated_at)) => {
                        eprintln!("Successfully parsed comment: id={}", id);
                        comments.push(serde_json::json!({
                            "id": id,
                            "post_id": post_id,
                            "user_id": user_id,
                            "username": username,
                            "parent_comment_id": parent_comment_id,
                            "content": content,
                            "created_at": created_at,
                            "updated_at": updated_at,
                        }));
                    }
                    (id_res, post_res, user_res, user_res2, parent_res, content_res, created_res, updated_res) => {
                        eprintln!("Failed to deserialize comment row:");
                        eprintln!("  id: {:?}", id_res);
                        eprintln!("  post_id: {:?}", post_res);
                        eprintln!("  user_id: {:?}", user_res);
                        eprintln!("  username: {:?}", user_res2);
                        eprintln!("  parent_comment_id: {:?}", parent_res);
                        eprintln!("  content: {:?}", content_res);
                        eprintln!("  created_at: {:?}", created_res);
                        eprintln!("  updated_at: {:?}", updated_res);
                    }
                }
            }
            eprintln!("Returning {} comments", comments.len());
            HttpResponse::Ok().json(comments)
        }
        Err(e) => {
            eprintln!("Failed to fetch comments from DB: {:?}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to fetch comments",
                "details": format!("{:?}", e)
            }))
        }
    }
}

pub async fn update_comment(
    db: web::Data<PgPool>,
    req: HttpRequest,
    comment_id_param: web::Path<i32>,
    body: web::Json<UpdateCommentRequest>,
) -> HttpResponse {
    let current_user = match get_current_user(&req) {
        Some(user) => user,
        None => return HttpResponse::Unauthorized().json("Not authenticated"),
    };

    let user_id = current_user.sub;
    let comment_id = comment_id_param.into_inner();

    if body.content.trim().is_empty() {
        return HttpResponse::BadRequest().json("Comment cannot be empty");
    }

    // Verify ownership
    match sqlx::query_scalar::<_, i32>(
        "SELECT user_id FROM comments WHERE id = $1"
    )
    .bind(comment_id)
    .fetch_optional(db.get_ref())
    .await
    {
        Ok(Some(owner_id)) if owner_id == user_id => {
            match sqlx::query(
                "UPDATE comments SET content = $1, updated_at = NOW() WHERE id = $2"
            )
            .bind(body.content.trim())
            .bind(comment_id)
            .execute(db.get_ref())
            .await
            {
                Ok(_) => HttpResponse::Ok().json(serde_json::json!({ "updated": true })),
                Err(_) => HttpResponse::InternalServerError().json("Failed to update comment"),
            }
        }
        Ok(Some(_)) => HttpResponse::Forbidden().json("Not authorized to update this comment"),
        Ok(None) => HttpResponse::NotFound().json("Comment not found"),
        Err(_) => HttpResponse::InternalServerError().json("Database error"),
    }
}

pub async fn delete_comment(
    db: web::Data<PgPool>,
    req: HttpRequest,
    comment_id_param: web::Path<i32>,
) -> HttpResponse {
    let current_user = match get_current_user(&req) {
        Some(user) => user,
        None => return HttpResponse::Unauthorized().json("Not authenticated"),
    };

    let user_id = current_user.sub;
    let comment_id = comment_id_param.into_inner();

    // Get owner_id and post_id
    match sqlx::query("SELECT user_id, post_id FROM comments WHERE id = $1")
        .bind(comment_id)
        .fetch_optional(db.get_ref())
        .await
    {
        Ok(Some(row)) => {
            let owner_id: i32 = row.get("user_id");
            let post_id: i32 = row.get("post_id");
            
            let is_admin = current_user.role == "admin";
            let is_comment_owner = owner_id == user_id;
            
            eprintln!("DELETE COMMENT: Request for comment_id={} from user_id={} role={}", comment_id, user_id, current_user.role);
            eprintln!("DELETE COMMENT: Owner check: owner_id={} is_comment_owner={}", owner_id, is_comment_owner);

            // Check if user is post owner (only if not admin or comment owner to save DB call)
            let is_post_owner = if !is_admin && !is_comment_owner {
                 let post_owner_check = sqlx::query_scalar::<_, i32>("SELECT user_id FROM posts WHERE id = $1")
                    .bind(post_id)
                    .fetch_optional(db.get_ref())
                    .await
                    .unwrap_or(None)
                    .map(|id| id == user_id)
                    .unwrap_or(false);
                 eprintln!("DELETE COMMENT: Post owner check: post_id={} is_post_owner={}", post_id, post_owner_check);
                 post_owner_check
            } else {
                false
            };
            
            if is_admin || is_comment_owner || is_post_owner {
                match sqlx::query("DELETE FROM comments WHERE id = $1")
                    .bind(comment_id)
                    .execute(db.get_ref())
                    .await
                {
                    Ok(_) => {
                        // Decrement comments_count
                        let _ = sqlx::query("UPDATE posts SET comments_count = comments_count - 1 WHERE id = $1")
                            .bind(post_id)
                            .execute(db.get_ref())
                            .await;
                        eprintln!("DELETE COMMENT: Success");
                        HttpResponse::Ok().json(serde_json::json!({ "deleted": true }))
                    }
                    Err(e) => {
                        eprintln!("DELETE COMMENT: Database error: {:?}", e);
                        HttpResponse::InternalServerError().json("Failed to delete comment")
                    },
                }
            } else {
                eprintln!("DELETE COMMENT: Forbidden");
                HttpResponse::Forbidden().json("Not authorized to delete this comment")
            }
        }
        Ok(None) => HttpResponse::NotFound().json("Comment not found"),
        Err(e) => {
            eprintln!("Database error: {:?}", e);
            HttpResponse::InternalServerError().json("Database error")
        }
    }
}
