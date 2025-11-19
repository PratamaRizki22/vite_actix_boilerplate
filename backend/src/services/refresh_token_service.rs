use sha2::{Sha256, Digest};
use sqlx::PgPool;
use chrono::{Utc, Duration};
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct RefreshTokenService;

impl RefreshTokenService {
    pub const REFRESH_TOKEN_EXPIRY_DAYS: i64 = 7;
    pub const MAX_ROTATION_ATTEMPTS: i32 = 10; // Prevent infinite loops

    /// Hash a refresh token using SHA256
    pub fn hash_token(token: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(token.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    /// Generate a new refresh token (UUID v4)
    pub fn generate_token() -> String {
        Uuid::new_v4().to_string()
    }

    /// Create a new refresh token for a user
    pub async fn create_refresh_token(
        pool: &PgPool,
        user_id: i32,
        token: &str,
    ) -> Result<String, sqlx::Error> {
        let token_hash = Self::hash_token(token);
        let token_family = Uuid::new_v4().to_string();
        let expires_at = Utc::now() + Duration::days(Self::REFRESH_TOKEN_EXPIRY_DAYS);

        sqlx::query!(
            "INSERT INTO refresh_tokens (user_id, token_hash, token_family, expires_at)
             VALUES ($1, $2, $3, $4)",
            user_id,
            token_hash,
            token_family,
            expires_at.naive_utc()
        )
        .execute(pool)
        .await?;

        Ok(token_family)
    }

    /// Verify and validate a refresh token
    pub async fn verify_refresh_token(
        pool: &PgPool,
        user_id: i32,
        token: &str,
    ) -> Result<bool, sqlx::Error> {
        let token_hash = Self::hash_token(token);
        let now = Utc::now().naive_utc();

        let record = sqlx::query!(
            "SELECT is_revoked, expires_at, reuse_detected FROM refresh_tokens
             WHERE user_id = $1 AND token_hash = $2",
            user_id,
            token_hash
        )
        .fetch_optional(pool)
        .await?;

        match record {
            Some(row) => {
                // Check if token is revoked
                if row.is_revoked.unwrap_or(false) {
                    return Ok(false);
                }

                // Check if reuse was detected (security threat)
                if row.reuse_detected.unwrap_or(false) {
                    return Ok(false);
                }

                // Check if token is expired
                let expires_at = row.expires_at;
                if expires_at <= now {
                    return Ok(false);
                }

                Ok(true)
            }
            None => Ok(false),
        }
    }

    /// Rotate a refresh token (generate new token, mark old as parent)
    pub async fn rotate_refresh_token(
        pool: &PgPool,
        user_id: i32,
        old_token: &str,
        new_token: &str,
    ) -> Result<String, sqlx::Error> {
        let old_token_hash = Self::hash_token(old_token);
        let new_token_hash = Self::hash_token(new_token);

        // Get the old token's family
        let old_record = sqlx::query!(
            "SELECT token_family FROM refresh_tokens
             WHERE user_id = $1 AND token_hash = $2",
            user_id,
            old_token_hash
        )
        .fetch_optional(pool)
        .await?;

        let token_family = match old_record {
            Some(record) => record.token_family,
            None => Uuid::new_v4().to_string(),
        };

        let expires_at = Utc::now() + Duration::days(Self::REFRESH_TOKEN_EXPIRY_DAYS);
        let now = Utc::now();

        // Insert new token with old token as parent
        sqlx::query!(
            "INSERT INTO refresh_tokens (user_id, token_hash, token_family, parent_token_hash, expires_at, rotated_at)
             VALUES ($1, $2, $3, $4, $5, $6)",
            user_id,
            new_token_hash,
            token_family,
            old_token_hash,
            expires_at.naive_utc(),
            now.naive_utc()
        )
        .execute(pool)
        .await?;

        Ok(token_family)
    }

    /// Detect and handle reuse attacks (same token used twice)
    pub async fn detect_reuse_attack(
        pool: &PgPool,
        user_id: i32,
        token: &str,
    ) -> Result<bool, sqlx::Error> {
        let token_hash = Self::hash_token(token);

        // Check if token was already used (has a child token)
        let has_child = sqlx::query!(
            "SELECT COUNT(*) as count FROM refresh_tokens
             WHERE user_id = $1 AND parent_token_hash = $2",
            user_id,
            token_hash
        )
        .fetch_one(pool)
        .await?;

        // If this token already generated a child, and it's being used again -> reuse attack
        if has_child.count.unwrap_or(0) > 0 {
            // Mark all tokens in this family as compromised
            let family = sqlx::query!(
                "SELECT token_family FROM refresh_tokens
                 WHERE user_id = $1 AND token_hash = $2",
                user_id,
                token_hash
            )
            .fetch_optional(pool)
            .await?;

            if let Some(record) = family {
                sqlx::query!(
                    "UPDATE refresh_tokens SET reuse_detected = TRUE
                     WHERE user_id = $1 AND token_family = $2",
                    user_id,
                    record.token_family
                )
                .execute(pool)
                .await?;
            }

            return Ok(true); // Reuse detected
        }

        Ok(false)
    }

    /// Revoke a refresh token
    pub async fn revoke_token(
        pool: &PgPool,
        user_id: i32,
        token: &str,
    ) -> Result<(), sqlx::Error> {
        let token_hash = Self::hash_token(token);

        sqlx::query!(
            "UPDATE refresh_tokens SET is_revoked = TRUE
             WHERE user_id = $1 AND token_hash = $2",
            user_id,
            token_hash
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    /// Revoke all tokens for a user (logout all devices)
    pub async fn revoke_all_tokens(pool: &PgPool, user_id: i32) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "UPDATE refresh_tokens SET is_revoked = TRUE
             WHERE user_id = $1",
            user_id
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    /// Revoke all tokens in a family (logout from all devices with same token generation)
    pub async fn revoke_token_family(
        pool: &PgPool,
        user_id: i32,
        token_family: &str,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "UPDATE refresh_tokens SET is_revoked = TRUE
             WHERE user_id = $1 AND token_family = $2",
            user_id,
            token_family
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    /// Get active token count for user (for multi-device tracking)
    pub async fn get_active_token_count(pool: &PgPool, user_id: i32) -> Result<i64, sqlx::Error> {
        let now = Utc::now().naive_utc();

        let result = sqlx::query!(
            "SELECT COUNT(*) as count FROM refresh_tokens
             WHERE user_id = $1 AND is_revoked = FALSE AND reuse_detected = FALSE
             AND expires_at > $2",
            user_id,
            now
        )
        .fetch_one(pool)
        .await?;

        Ok(result.count.unwrap_or(0))
    }

    /// Clean up expired tokens
    pub async fn cleanup_expired_tokens(pool: &PgPool) -> Result<u64, sqlx::Error> {
        let now = Utc::now().naive_utc();

        let result = sqlx::query!(
            "DELETE FROM refresh_tokens WHERE expires_at < $1",
            now
        )
        .execute(pool)
        .await?;

        Ok(result.rows_affected())
    }
}
