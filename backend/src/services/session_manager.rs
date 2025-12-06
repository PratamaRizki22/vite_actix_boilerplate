use chrono::{Utc, Duration, NaiveDateTime};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use sha2::{Sha256, Digest};
use crate::middleware::redis_session::{RedisSessionStore, RedisSessionData};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionInfo {
    pub id: i32,
    pub user_id: i32,
    pub device_name: Option<String>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub last_activity: NaiveDateTime,
    pub expires_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Clone)]
pub struct CreateSessionData {
    pub user_id: i32,
    pub token: String,
    pub device_name: Option<String>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
}

pub struct SessionManager;

impl SessionManager {
    /// Hash token using SHA256
    pub fn hash_token(token: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(token.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    /// Create new session with Redis caching
    pub async fn create_session(
        pool: &PgPool,
        data: CreateSessionData,
    ) -> Result<SessionInfo, sqlx::Error> {
        let token_hash = Self::hash_token(&data.token);
        let expires_at = (Utc::now() + Duration::hours(24)).naive_utc(); // 24 hour session

        let session = sqlx::query_as!(
            SessionInfo,
            "INSERT INTO sessions 
             (user_id, token_hash, device_name, ip_address, user_agent, last_activity, expires_at)
             VALUES ($1, $2, $3, $4, $5, NOW(), $6)
             RETURNING id, user_id, device_name, ip_address, user_agent, 
                       last_activity, expires_at, created_at",
            data.user_id,
            token_hash,
            data.device_name.clone(),
            data.ip_address.clone(),
            data.user_agent.clone(),
            expires_at
        )
        .fetch_one(pool)
        .await?;

        Ok(session)
    }

    /// Create new session with Redis (preferred method)
    pub async fn create_session_with_redis(
        pool: &PgPool,
        redis: &RedisSessionStore,
        data: CreateSessionData,
    ) -> Result<SessionInfo, sqlx::Error> {
        let token_hash = Self::hash_token(&data.token);
        let expires_at = (Utc::now() + Duration::hours(24)).naive_utc();
        let now = Utc::now();

        // Create in database first
        let session = sqlx::query_as!(
            SessionInfo,
            "INSERT INTO sessions 
             (user_id, token_hash, device_name, ip_address, user_agent, last_activity, expires_at)
             VALUES ($1, $2, $3, $4, $5, NOW(), $6)
             RETURNING id, user_id, device_name, ip_address, user_agent, 
                       last_activity, expires_at, created_at",
            data.user_id,
            token_hash,
            data.device_name.clone(),
            data.ip_address.clone(),
            data.user_agent.clone(),
            expires_at
        )
        .fetch_one(pool)
        .await?;

        // Store in Redis
        let redis_session = RedisSessionData {
            user_id: data.user_id,
            device_name: data.device_name,
            ip_address: data.ip_address,
            user_agent: data.user_agent,
            created_at: now.timestamp(),
            last_activity: now.timestamp(),
        };

        let ttl_seconds = 24 * 3600; // 24 hours
        let _ = redis.store_session(&token_hash, &redis_session, ttl_seconds).await;

        Ok(session)
    }

    /// Verify session is still valid (database only)
    pub async fn verify_session(pool: &PgPool, token: &str) -> Result<Option<SessionInfo>, sqlx::Error> {
        let token_hash = Self::hash_token(token);

        let session = sqlx::query_as!(
            SessionInfo,
            "SELECT id, user_id, device_name, ip_address, user_agent,
                    last_activity, expires_at, created_at
             FROM sessions 
             WHERE token_hash = $1 AND expires_at > NOW()",
            token_hash
        )
        .fetch_optional(pool)
        .await?;

        Ok(session)
    }

    /// Verify session with Redis (preferred method)
    pub async fn verify_session_with_redis(
        pool: &PgPool,
        redis: &RedisSessionStore,
        token: &str,
    ) -> Result<Option<SessionInfo>, sqlx::Error> {
        let token_hash = Self::hash_token(token);

        // Try Redis first
        if let Ok(Some(redis_session)) = redis.get_session(&token_hash).await {
            // Update activity in Redis
            let _ = redis.update_activity(&token_hash).await;

            // Convert Redis session to SessionInfo
            let session_info = SessionInfo {
                id: 0, // Redis sessions don't have DB ID
                user_id: redis_session.user_id,
                device_name: redis_session.device_name,
                ip_address: redis_session.ip_address,
                user_agent: redis_session.user_agent,
                last_activity: chrono::NaiveDateTime::from_timestamp_opt(redis_session.last_activity, 0).unwrap_or_default(),
                expires_at: chrono::NaiveDateTime::from_timestamp_opt(redis_session.created_at + (24 * 3600), 0).unwrap_or_default(),
                created_at: chrono::NaiveDateTime::from_timestamp_opt(redis_session.created_at, 0).unwrap_or_default(),
            };

            return Ok(Some(session_info));
        }

        // Fallback to database
        Self::verify_session(pool, token).await
    }

    /// Update session last activity (keep alive)
    pub async fn update_activity(
        pool: &PgPool,
        session_id: i32,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "UPDATE sessions SET last_activity = NOW(), updated_at = NOW() WHERE id = $1",
            session_id
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    /// Get all user sessions
    pub async fn get_user_sessions(
        pool: &PgPool,
        user_id: i32,
    ) -> Result<Vec<SessionInfo>, sqlx::Error> {
        let sessions = sqlx::query_as!(
            SessionInfo,
            "SELECT id, user_id, device_name, ip_address, user_agent,
                    last_activity, expires_at, created_at
             FROM sessions 
             WHERE user_id = $1 AND expires_at > NOW()
             ORDER BY last_activity DESC",
            user_id
        )
        .fetch_all(pool)
        .await?;

        Ok(sessions)
    }

    /// Invalidate session (logout)
    pub async fn invalidate_session(
        pool: &PgPool,
        session_id: i32,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "UPDATE sessions SET expires_at = NOW() WHERE id = $1",
            session_id
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    /// Invalidate all user sessions (e.g., password change)
    pub async fn invalidate_all_sessions(
        pool: &PgPool,
        user_id: i32,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "UPDATE sessions SET expires_at = NOW() WHERE user_id = $1",
            user_id
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    /// Invalidate all sessions except current (logout from other devices)
    pub async fn invalidate_other_sessions(
        pool: &PgPool,
        user_id: i32,
        session_id: i32,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "UPDATE sessions SET expires_at = NOW() WHERE user_id = $1 AND id != $2",
            user_id,
            session_id
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    /// Clean up expired sessions
    pub async fn cleanup_expired_sessions(pool: &PgPool) -> Result<u64, sqlx::Error> {
        let result = sqlx::query!(
            "DELETE FROM sessions WHERE expires_at < NOW()"
        )
        .execute(pool)
        .await?;

        Ok(result.rows_affected())
    }

    /// Get session info from token
    pub async fn get_session_by_token(
        pool: &PgPool,
        token: &str,
    ) -> Result<Option<SessionInfo>, sqlx::Error> {
        let token_hash = Self::hash_token(token);

        let session = sqlx::query_as!(
            SessionInfo,
            "SELECT id, user_id, device_name, ip_address, user_agent,
                    last_activity, expires_at, created_at
             FROM sessions WHERE token_hash = $1",
            token_hash
        )
        .fetch_optional(pool)
        .await?;

        Ok(session)
    }

    /// Logout session (invalidate token)
    pub async fn logout(pool: &PgPool, token: &str) -> Result<bool, sqlx::Error> {
        let token_hash = Self::hash_token(token);

        let result = sqlx::query!(
            "UPDATE sessions SET expires_at = NOW() WHERE token_hash = $1",
            token_hash
        )
        .execute(pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    /// Logout session with Redis
    pub async fn logout_with_redis(
        pool: &PgPool,
        redis: &RedisSessionStore,
        token: &str,
    ) -> Result<bool, sqlx::Error> {
        let token_hash = Self::hash_token(token);

        // Delete from Redis first
        let _ = redis.delete_session(&token_hash).await;

        // Then from database
        Self::logout(pool, token).await
    }

    /// Invalidate all user sessions with Redis
    pub async fn invalidate_all_sessions_with_redis(
        pool: &PgPool,
        redis: &RedisSessionStore,
        user_id: i32,
    ) -> Result<(), sqlx::Error> {
        // Delete from Redis
        let _ = redis.delete_user_sessions(user_id).await;

        // Then from database
        Self::invalidate_all_sessions(pool, user_id).await
    }
}
