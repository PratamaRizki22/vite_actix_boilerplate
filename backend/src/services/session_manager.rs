use chrono::{DateTime, Utc, Duration, NaiveDateTime};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use sha2::{Sha256, Digest};

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

    /// Create new session
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
            data.device_name,
            data.ip_address,
            data.user_agent,
            expires_at
        )
        .fetch_one(pool)
        .await?;

        Ok(session)
    }

    /// Verify session is still valid
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
}
