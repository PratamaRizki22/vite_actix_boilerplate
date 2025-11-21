use chrono::{Duration, Utc, NaiveDateTime};
use sqlx::PgPool;
use std::result::Result;

pub struct CleanupService;

impl CleanupService {
    /// Clean up unverified user accounts that are older than the specified days
    pub async fn cleanup_unverified_accounts(
        pool: &PgPool,
        older_than_days: i64,
    ) -> Result<u64, sqlx::Error> {
        let cutoff_date = Utc::now() - Duration::days(older_than_days);
        let cutoff_naive = cutoff_date.naive_utc();

        // Delete unverified accounts that are older than cutoff_date
        // Only delete traditional registration accounts (not Google OAuth or Web3)
        let result = sqlx::query!(
            "DELETE FROM users
             WHERE email_verified = false
             AND password != 'google_oauth'
             AND password != 'web3_auth'
             AND created_at < $1",
            cutoff_naive
        )
        .execute(pool)
        .await?;

        Ok(result.rows_affected())
    }

    /// Clean up unverified accounts that have been unverified for more than 7 days
    pub async fn cleanup_old_unverified_accounts(pool: &PgPool) -> Result<u64, sqlx::Error> {
        Self::cleanup_unverified_accounts(pool, 7).await
    }

    /// Get statistics about unverified accounts
    pub async fn get_unverified_accounts_stats(
        pool: &PgPool,
    ) -> Result<Vec<(String, NaiveDateTime, String)>, sqlx::Error> {
        let accounts = sqlx::query!(
            "SELECT username, created_at, email
             FROM users
             WHERE email_verified = false
             AND password != 'google_oauth'
             AND password != 'web3_auth'
             ORDER BY created_at DESC"
        )
        .fetch_all(pool)
        .await?;

        Ok(accounts
            .into_iter()
            .map(|row| (row.username, row.created_at, row.email.unwrap_or_else(|| "No email".to_string())))
            .collect())
    }
}