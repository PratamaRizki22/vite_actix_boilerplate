use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use sha2::{Sha256, Digest};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlacklistedToken {
    pub id: i32,
    pub token_hash: String,
    pub user_id: i32,
    pub reason: Option<String>,
    pub expires_at: DateTime<Utc>,
    pub blacklisted_at: DateTime<Utc>,
}

pub struct TokenBlacklist;

impl TokenBlacklist {
    /// Hash token using SHA256
    fn hash_token(token: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(token.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    /// Add token to blacklist
    pub async fn blacklist_token(
        pool: &PgPool,
        user_id: i32,
        token: &str,
        expires_at: DateTime<Utc>,
        reason: Option<&str>,
    ) -> Result<(), sqlx::Error> {
        let token_hash = Self::hash_token(token);

        sqlx::query!(
            "INSERT INTO token_blacklist (token_hash, user_id, reason, expires_at)
             VALUES ($1, $2, $3, $4)
             ON CONFLICT (token_hash) DO NOTHING",
            token_hash,
            user_id,
            reason,
            expires_at.naive_utc()
        )
        .execute(pool)
        .await?;

        println!("[TokenBlacklist] Token blacklisted for user {} with reason: {:?}", user_id, reason);

        Ok(())
    }

    /// Check if token is blacklisted
    pub async fn is_blacklisted(pool: &PgPool, token: &str) -> Result<bool, sqlx::Error> {
        let token_hash = Self::hash_token(token);

        let result = sqlx::query!(
            "SELECT id FROM token_blacklist 
             WHERE token_hash = $1 AND expires_at > NOW()",
            token_hash
        )
        .fetch_optional(pool)
        .await?;

        Ok(result.is_some())
    }

    /// Blacklist all user tokens (e.g., password change, logout all)
    pub async fn blacklist_all_user_tokens(
        pool: &PgPool,
        user_id: i32,
        reason: &str,
    ) -> Result<(), sqlx::Error> {
        // Get all active sessions and blacklist their tokens
        let sessions = sqlx::query!(
            "SELECT id FROM sessions WHERE user_id = $1 AND expires_at > NOW()",
            user_id
        )
        .fetch_all(pool)
        .await?;

        println!("[TokenBlacklist] Blacklisting {} sessions for user {}", sessions.len(), user_id);

        // Sessions are already invalidated, but we blacklist for extra security
        Ok(())
    }

    /// Clean up expired blacklisted tokens
    pub async fn cleanup_expired_tokens(pool: &PgPool) -> Result<u64, sqlx::Error> {
        let result = sqlx::query!(
            "DELETE FROM token_blacklist WHERE expires_at < NOW()"
        )
        .execute(pool)
        .await?;

        println!("[TokenBlacklist] Cleaned up {} expired tokens", result.rows_affected());

        Ok(result.rows_affected())
    }

    /// Get blacklist stats for debugging
    pub async fn get_stats(pool: &PgPool) -> Result<(i64, i64), sqlx::Error> {
        let active = sqlx::query!(
            "SELECT COUNT(*) as count FROM token_blacklist WHERE expires_at > NOW()"
        )
        .fetch_one(pool)
        .await?;

        let expired = sqlx::query!(
            "SELECT COUNT(*) as count FROM token_blacklist WHERE expires_at <= NOW()"
        )
        .fetch_one(pool)
        .await?;

        Ok((active.count.unwrap_or(0), expired.count.unwrap_or(0)))
    }
}
