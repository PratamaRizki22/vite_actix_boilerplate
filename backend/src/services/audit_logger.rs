use sqlx::PgPool;
use serde_json::{json, Value};
use chrono::{DateTime, Utc};

#[derive(Clone, Debug)]
pub struct AuditLogger;

impl AuditLogger {
    /// Event types
    pub const EVENT_LOGIN: &'static str = "LOGIN";
    pub const EVENT_LOGOUT: &'static str = "LOGOUT";
    pub const EVENT_REGISTER: &'static str = "REGISTER";
    pub const EVENT_PASSWORD_RESET: &'static str = "PASSWORD_RESET";
    pub const EVENT_PASSWORD_RESET_REQUEST: &'static str = "PASSWORD_RESET_REQUEST";
    pub const EVENT_EMAIL_VERIFICATION: &'static str = "EMAIL_VERIFICATION";
    pub const EVENT_ACCOUNT_LOCKOUT: &'static str = "ACCOUNT_LOCKOUT";
    pub const EVENT_TOKEN_BLACKLIST: &'static str = "TOKEN_BLACKLIST";
    pub const EVENT_FAILED_LOGIN: &'static str = "FAILED_LOGIN";

    /// Status types
    pub const STATUS_SUCCESS: &'static str = "success";
    pub const STATUS_FAILED: &'static str = "failed";
    pub const STATUS_BLOCKED: &'static str = "blocked";

    /// Log an audit event
    pub async fn log(
        pool: &PgPool,
        user_id: Option<i32>,
        event_type: &str,
        event_action: &str,
        ip_address: Option<&str>,
        user_agent: Option<&str>,
        status: &str,
        details: Option<Value>,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "INSERT INTO audit_logs (user_id, event_type, event_action, ip_address, user_agent, status, details)
             VALUES ($1, $2, $3, $4, $5, $6, $7)",
            user_id,
            event_type,
            event_action,
            ip_address,
            user_agent,
            status,
            details
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    /// Log successful login
    pub async fn log_login(
        pool: &PgPool,
        user_id: i32,
        ip_address: Option<&str>,
        user_agent: Option<&str>,
    ) -> Result<(), sqlx::Error> {
        Self::log(
            pool,
            Some(user_id),
            Self::EVENT_LOGIN,
            "User login successful",
            ip_address,
            user_agent,
            Self::STATUS_SUCCESS,
            Some(json!({"event": "login"})),
        )
        .await
    }

    /// Log failed login attempt
    pub async fn log_failed_login(
        pool: &PgPool,
        user_id: i32,
        reason: &str,
        ip_address: Option<&str>,
        user_agent: Option<&str>,
    ) -> Result<(), sqlx::Error> {
        Self::log(
            pool,
            Some(user_id),
            Self::EVENT_FAILED_LOGIN,
            &format!("Failed login attempt: {}", reason),
            ip_address,
            user_agent,
            Self::STATUS_FAILED,
            Some(json!({"reason": reason})),
        )
        .await
    }

    /// Log logout
    pub async fn log_logout(
        pool: &PgPool,
        user_id: i32,
        ip_address: Option<&str>,
        user_agent: Option<&str>,
    ) -> Result<(), sqlx::Error> {
        Self::log(
            pool,
            Some(user_id),
            Self::EVENT_LOGOUT,
            "User logout",
            ip_address,
            user_agent,
            Self::STATUS_SUCCESS,
            Some(json!({"event": "logout"})),
        )
        .await
    }

    /// Log account lockout
    pub async fn log_account_lockout(
        pool: &PgPool,
        user_id: i32,
        lockout_minutes: i64,
        ip_address: Option<&str>,
        user_agent: Option<&str>,
    ) -> Result<(), sqlx::Error> {
        Self::log(
            pool,
            Some(user_id),
            Self::EVENT_ACCOUNT_LOCKOUT,
            "Account locked due to failed login attempts",
            ip_address,
            user_agent,
            Self::STATUS_BLOCKED,
            Some(json!({"lockout_minutes": lockout_minutes})),
        )
        .await
    }

    /// Log password reset
    pub async fn log_password_reset(
        pool: &PgPool,
        user_id: i32,
        ip_address: Option<&str>,
        user_agent: Option<&str>,
    ) -> Result<(), sqlx::Error> {
        Self::log(
            pool,
            Some(user_id),
            Self::EVENT_PASSWORD_RESET,
            "Password reset",
            ip_address,
            user_agent,
            Self::STATUS_SUCCESS,
            Some(json!({"event": "password_reset"})),
        )
        .await
    }

    /// Log token blacklist event
    pub async fn log_token_blacklist(
        pool: &PgPool,
        user_id: i32,
        reason: &str,
        ip_address: Option<&str>,
        user_agent: Option<&str>,
    ) -> Result<(), sqlx::Error> {
        Self::log(
            pool,
            Some(user_id),
            Self::EVENT_TOKEN_BLACKLIST,
            "Token blacklisted",
            ip_address,
            user_agent,
            Self::STATUS_SUCCESS,
            Some(json!({"reason": reason})),
        )
        .await
    }

    /// Get user's audit trail
    pub async fn get_user_audit_trail(
        pool: &PgPool,
        user_id: i32,
        limit: i64,
    ) -> Result<Vec<(String, String, String, Option<String>, DateTime<Utc>)>, sqlx::Error> {
        let records = sqlx::query!(
            "SELECT event_type, event_action, status, details, created_at 
             FROM audit_logs 
             WHERE user_id = $1 
             ORDER BY created_at DESC 
             LIMIT $2",
            user_id,
            limit
        )
        .fetch_all(pool)
        .await?;

        Ok(records
            .into_iter()
            .map(|r| {
                let created_at = r.created_at.unwrap_or_else(|| chrono::NaiveDateTime::from_timestamp_opt(0, 0).unwrap());
                (
                    r.event_type,
                    r.event_action,
                    r.status,
                    r.details.map(|d| d.to_string()),
                    DateTime::<Utc>::from_naive_utc_and_offset(created_at, Utc),
                )
            })
            .collect())
    }

    /// Get audit logs for specific event type
    pub async fn get_logs_by_event(
        pool: &PgPool,
        event_type: &str,
        limit: i64,
    ) -> Result<i64, sqlx::Error> {
        let result = sqlx::query!("SELECT COUNT(*) as count FROM audit_logs WHERE event_type = $1", event_type)
            .fetch_one(pool)
            .await?;

        Ok(result.count.unwrap_or(0))
    }
}
