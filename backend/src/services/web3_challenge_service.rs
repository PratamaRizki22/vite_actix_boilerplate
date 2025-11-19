use sqlx::PgPool;
use chrono::{DateTime, Utc, Duration};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Web3ChallengeService;

impl Web3ChallengeService {
    /// Create a new Web3 challenge for an address
    pub async fn create_challenge(
        pool: &PgPool,
        address: &str,
        challenge: &str,
        ttl_seconds: i64,
    ) -> Result<DateTime<Utc>, sqlx::Error> {
        let expires_at = Utc::now() + Duration::seconds(ttl_seconds);
        
        sqlx::query!(
            "INSERT INTO web3_challenges (address, challenge, expires_at) 
             VALUES ($1, $2, $3)
             ON CONFLICT(challenge) DO UPDATE SET expires_at = EXCLUDED.expires_at",
            address,
            challenge,
            expires_at.naive_utc()
        )
        .execute(pool)
        .await?;

        Ok(expires_at)
    }

    /// Verify a Web3 challenge exists and is not expired
    pub async fn verify_challenge(
        pool: &PgPool,
        address: &str,
        challenge: &str,
    ) -> Result<bool, sqlx::Error> {
        let result = sqlx::query!(
            "SELECT id, expires_at, used_at FROM web3_challenges 
             WHERE address = $1 AND challenge = $2 AND used_at IS NULL",
            address,
            challenge
        )
        .fetch_optional(pool)
        .await?;

        match result {
            Some(record) => {
                let expires_at = DateTime::<Utc>::from_naive_utc_and_offset(record.expires_at, Utc);
                Ok(expires_at > Utc::now())
            }
            None => Ok(false),
        }
    }

    /// Mark a challenge as used
    pub async fn mark_used(
        pool: &PgPool,
        challenge: &str,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "UPDATE web3_challenges SET used_at = NOW() WHERE challenge = $1",
            challenge
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    /// Get active challenge for address (if any)
    pub async fn get_active_challenge(
        pool: &PgPool,
        address: &str,
    ) -> Result<Option<String>, sqlx::Error> {
        let result = sqlx::query!(
            "SELECT challenge FROM web3_challenges 
             WHERE address = $1 AND used_at IS NULL AND expires_at > NOW()
             ORDER BY created_at DESC LIMIT 1",
            address
        )
        .fetch_optional(pool)
        .await?;

        Ok(result.map(|r| r.challenge))
    }

    /// Clean up expired challenges (call periodically)
    pub async fn cleanup_expired(pool: &PgPool) -> Result<u64, sqlx::Error> {
        let result = sqlx::query!(
            "DELETE FROM web3_challenges WHERE expires_at < NOW()"
        )
        .execute(pool)
        .await?;

        Ok(result.rows_affected())
    }

    /// Generate a random challenge string
    pub fn generate_challenge() -> String {
        Uuid::new_v4().to_string()
    }
}
