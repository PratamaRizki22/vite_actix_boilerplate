use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use chrono::{DateTime, Utc, Duration};
use sqlx::PgPool;

#[derive(Clone)]
pub struct AccountLockout;

impl AccountLockout {
    pub const MAX_ATTEMPTS: i32 = 5;
    pub const INITIAL_LOCKOUT_MINUTES: i64 = 15;
    pub const MAX_LOCKOUT_MINUTES: i64 = 240; // 4 hours

    /// Record a failed login attempt and check lockout status
    pub async fn record_failed_attempt(pool: &PgPool, user_id: i32) -> Result<(), sqlx::Error> {
        // Get current lockout record
        let record = sqlx::query!(
            "SELECT failed_attempts, locked_until FROM account_lockout WHERE user_id = $1",
            user_id
        )
        .fetch_optional(pool)
        .await?;

        match record {
            Some(row) => {
                // Existing record: update attempts or check if unlock time reached
                let now = Utc::now();
                let locked_until = row.locked_until.map(|t| DateTime::<Utc>::from_naive_utc_and_offset(t, Utc));
                
                // If still locked, don't increment attempts
                if let Some(until) = locked_until {
                    if now < until {
                        return Ok(());
                    }
                }

                // Unlock window expired or no lockout, increment attempts
                let failed_attempts = row.failed_attempts.unwrap_or(0);
                let new_attempts = failed_attempts + 1;
                let should_lock = new_attempts >= Self::MAX_ATTEMPTS;
                let lockout_until = if should_lock {
                    // Exponential backoff: 15, 30, 60, 120, 240 minutes
                    let lockout_minutes = std::cmp::min(
                        Self::INITIAL_LOCKOUT_MINUTES * (2_i64.pow((new_attempts - Self::MAX_ATTEMPTS) as u32)),
                        Self::MAX_LOCKOUT_MINUTES
                    );
                    Some(now + Duration::minutes(lockout_minutes))
                } else {
                    None
                };

                sqlx::query!(
                    "UPDATE account_lockout SET failed_attempts = $1, locked_until = $2, last_attempt = NOW(), updated_at = NOW() WHERE user_id = $3",
                    new_attempts,
                    lockout_until.map(|t| t.naive_utc()),
                    user_id
                )
                .execute(pool)
                .await?;
            }
            None => {
                // New record
                sqlx::query!(
                    "INSERT INTO account_lockout (user_id, failed_attempts, last_attempt) VALUES ($1, 1, NOW())",
                    user_id
                )
                .execute(pool)
                .await?;
            }
        }

        Ok(())
    }

    /// Check if account is currently locked
    pub async fn is_locked(pool: &PgPool, user_id: i32) -> Result<bool, sqlx::Error> {
        let record = sqlx::query!(
            "SELECT locked_until FROM account_lockout WHERE user_id = $1",
            user_id
        )
        .fetch_optional(pool)
        .await?;

        if let Some(row) = record {
            if let Some(locked_until_naive) = row.locked_until {
                let locked_until = DateTime::<Utc>::from_naive_utc_and_offset(locked_until_naive, Utc);
                return Ok(Utc::now() < locked_until);
            }
        }

        Ok(false)
    }

    /// Get remaining lockout time in seconds (0 if not locked)
    pub async fn get_remaining_lockout_seconds(pool: &PgPool, user_id: i32) -> Result<i64, sqlx::Error> {
        let record = sqlx::query!(
            "SELECT locked_until FROM account_lockout WHERE user_id = $1",
            user_id
        )
        .fetch_optional(pool)
        .await?;

        if let Some(row) = record {
            if let Some(locked_until_naive) = row.locked_until {
                let locked_until = DateTime::<Utc>::from_naive_utc_and_offset(locked_until_naive, Utc);
                let now = Utc::now();
                if now < locked_until {
                    return Ok((locked_until - now).num_seconds());
                }
            }
        }

        Ok(0)
    }

    /// Reset failed attempts after successful login
    pub async fn reset_attempts(pool: &PgPool, user_id: i32) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "UPDATE account_lockout SET failed_attempts = 0, locked_until = NULL, updated_at = NOW() WHERE user_id = $1",
            user_id
        )
        .execute(pool)
        .await?;

        Ok(())
    }
}
