mod common;

use sqlx::PgPool;

#[actix_web::test]
async fn test_audit_log_direct_insertion() {
    let pool = common::setup_test_db().await;

    // Create test user
    let (user_id, _, _) = common::create_test_user(&pool, "testuser", "test@example.com", true).await;

    // Log an event directly
    let result = sqlx::query!(
        "INSERT INTO audit_logs (user_id, event_type, event_action, ip_address, user_agent, status, details)
         VALUES ($1, $2, $3, $4, $5, $6, $7)",
        user_id,
        "LOGIN",
        "Test login event",
        "127.0.0.1",
        "Mozilla/5.0",
        "success",
        None::<serde_json::Value>
    )
    .execute(&pool)
    .await;

    assert!(result.is_ok());

    // Verify it was inserted
    let logs = sqlx::query!(
        "SELECT event_type, event_action, status, ip_address, user_agent FROM audit_logs WHERE user_id = $1",
        user_id
    )
    .fetch_all(&pool)
    .await
    .expect("Failed to fetch audit logs");

    assert_eq!(logs.len(), 1);
    assert_eq!(logs[0].event_type, "LOGIN");
    assert_eq!(logs[0].event_action, "Test login event");
    assert_eq!(logs[0].status, "success");
    assert_eq!(logs[0].ip_address, Some("127.0.0.1".to_string()));
    assert_eq!(logs[0].user_agent, Some("Mozilla/5.0".to_string()));
}

#[actix_web::test]
async fn test_audit_log_login_event() {
    let pool = common::setup_test_db().await;

    let (user_id, _, _) = common::create_test_user(&pool, "testuser", "test@example.com", true).await;

    let _ = sqlx::query!(
        "INSERT INTO audit_logs (user_id, event_type, event_action, ip_address, user_agent, status) VALUES ($1, $2, $3, $4, $5, $6)",
        user_id, "LOGIN", "User login", "192.168.1.1", "Chrome/120", "success"
    ).execute(&pool).await;

    let logs = sqlx::query!(
        "SELECT event_type, status FROM audit_logs WHERE user_id = $1 AND event_type = $2",
        user_id, "LOGIN"
    )
    .fetch_all(&pool)
    .await
    .expect("Failed to fetch logs");

    assert_eq!(logs.len(), 1);
    assert_eq!(logs[0].status, "success");
}

#[actix_web::test]
async fn test_audit_log_failed_login() {
    let pool = common::setup_test_db().await;

    let (user_id, _, _) = common::create_test_user(&pool, "testuser", "test@example.com", true).await;

    let _ = sqlx::query!(
        "INSERT INTO audit_logs (user_id, event_type, event_action, ip_address, user_agent, status, details)
         VALUES ($1, $2, $3, $4, $5, $6, $7)",
        user_id, "FAILED_LOGIN", "Failed login attempt: Invalid password", "10.0.0.1", "Firefox/122", "failed",
        Some(serde_json::json!({"reason": "invalid_password"}))
    ).execute(&pool).await;

    let logs = sqlx::query!(
        "SELECT event_type, status, event_action FROM audit_logs WHERE user_id = $1 AND event_type = $2",
        user_id, "FAILED_LOGIN"
    )
    .fetch_all(&pool)
    .await
    .expect("Failed to fetch logs");

    assert_eq!(logs.len(), 1);
    assert_eq!(logs[0].status, "failed");
    assert!(logs[0].event_action.contains("Invalid password"));
}

#[actix_web::test]
async fn test_audit_log_logout() {
    let pool = common::setup_test_db().await;

    let (user_id, _, _) = common::create_test_user(&pool, "testuser", "test@example.com", true).await;

    let _ = sqlx::query!(
        "INSERT INTO audit_logs (user_id, event_type, event_action, ip_address, user_agent, status)
         VALUES ($1, $2, $3, $4, $5, $6)",
        user_id, "LOGOUT", "User logout", "172.16.0.1", "Safari/537", "success"
    ).execute(&pool).await;

    let logs = sqlx::query!(
        "SELECT event_type, status FROM audit_logs WHERE user_id = $1 AND event_type = $2",
        user_id, "LOGOUT"
    )
    .fetch_all(&pool)
    .await
    .expect("Failed to fetch logs");

    assert_eq!(logs.len(), 1);
    assert_eq!(logs[0].status, "success");
}

#[actix_web::test]
async fn test_audit_log_account_lockout() {
    let pool = common::setup_test_db().await;

    let (user_id, _, _) = common::create_test_user(&pool, "testuser", "test@example.com", true).await;

    let _ = sqlx::query!(
        "INSERT INTO audit_logs (user_id, event_type, event_action, ip_address, user_agent, status, details)
         VALUES ($1, $2, $3, $4, $5, $6, $7)",
        user_id, "ACCOUNT_LOCKOUT", "Account locked due to failed login attempts", "8.8.8.8", "Edge/120", "blocked",
        Some(serde_json::json!({"lockout_minutes": 30}))
    ).execute(&pool).await;

    let logs = sqlx::query!(
        "SELECT event_type, status FROM audit_logs WHERE user_id = $1 AND event_type = $2",
        user_id, "ACCOUNT_LOCKOUT"
    )
    .fetch_all(&pool)
    .await
    .expect("Failed to fetch logs");

    assert_eq!(logs.len(), 1);
    assert_eq!(logs[0].status, "blocked");
}

#[actix_web::test]
async fn test_audit_log_password_reset() {
    let pool = common::setup_test_db().await;

    let (user_id, _, _) = common::create_test_user(&pool, "testuser", "test@example.com", true).await;

    let _ = sqlx::query!(
        "INSERT INTO audit_logs (user_id, event_type, event_action, status)
         VALUES ($1, $2, $3, $4)",
        user_id, "PASSWORD_RESET", "Password reset", "success"
    ).execute(&pool).await;

    let logs = sqlx::query!(
        "SELECT event_type, status FROM audit_logs WHERE user_id = $1 AND event_type = $2",
        user_id, "PASSWORD_RESET"
    )
    .fetch_all(&pool)
    .await
    .expect("Failed to fetch logs");

    assert_eq!(logs.len(), 1);
    assert_eq!(logs[0].status, "success");
}

#[actix_web::test]
async fn test_audit_log_multiple_events() {
    let pool = common::setup_test_db().await;

    let (user_id, _, _) = common::create_test_user(&pool, "testuser", "test@example.com", true).await;

    let _ = sqlx::query!(
        "INSERT INTO audit_logs (user_id, event_type, event_action, status) VALUES ($1, $2, $3, $4)",
        user_id, "LOGIN", "Test login", "success"
    ).execute(&pool).await;

    let _ = sqlx::query!(
        "INSERT INTO audit_logs (user_id, event_type, event_action, status) VALUES ($1, $2, $3, $4)",
        user_id, "LOGOUT", "Test logout", "success"
    ).execute(&pool).await;

    let _ = sqlx::query!(
        "INSERT INTO audit_logs (user_id, event_type, event_action, status) VALUES ($1, $2, $3, $4)",
        user_id, "LOGIN", "Second login", "success"
    ).execute(&pool).await;

    let trail = sqlx::query!(
        "SELECT event_type FROM audit_logs WHERE user_id = $1 ORDER BY created_at DESC",
        user_id
    )
    .fetch_all(&pool)
    .await
    .expect("Failed to get audit trail");

    assert_eq!(trail.len(), 3);
    assert_eq!(trail[0].event_type, "LOGIN");
    assert_eq!(trail[1].event_type, "LOGOUT");
    assert_eq!(trail[2].event_type, "LOGIN");
}

#[actix_web::test]
async fn test_audit_log_get_by_event_type() {
    let pool = common::setup_test_db().await;

    let (user1_id, _, _) = common::create_test_user(&pool, "user1", "user1@example.com", true).await;
    let (user2_id, _, _) = common::create_test_user(&pool, "user2", "user2@example.com", true).await;

    let _ = sqlx::query!(
        "INSERT INTO audit_logs (user_id, event_type, event_action, status) VALUES ($1, $2, $3, $4)",
        user1_id, "LOGIN", "User1 login", "success"
    ).execute(&pool).await;

    let _ = sqlx::query!(
        "INSERT INTO audit_logs (user_id, event_type, event_action, status) VALUES ($1, $2, $3, $4)",
        user1_id, "LOGOUT", "User1 logout", "success"
    ).execute(&pool).await;

    let _ = sqlx::query!(
        "INSERT INTO audit_logs (user_id, event_type, event_action, status) VALUES ($1, $2, $3, $4)",
        user2_id, "LOGIN", "User2 login", "success"
    ).execute(&pool).await;

    let login_count = sqlx::query!(
        "SELECT COUNT(*) as count FROM audit_logs WHERE event_type = $1",
        "LOGIN"
    )
    .fetch_one(&pool)
    .await
    .expect("Failed to count");

    assert_eq!(login_count.count.unwrap_or(0), 2);

    let logout_count = sqlx::query!(
        "SELECT COUNT(*) as count FROM audit_logs WHERE event_type = $1",
        "LOGOUT"
    )
    .fetch_one(&pool)
    .await
    .expect("Failed to count");

    assert_eq!(logout_count.count.unwrap_or(0), 1);
}

#[actix_web::test]
async fn test_audit_log_with_jsonb_details() {
    let pool = common::setup_test_db().await;

    let (user_id, _, _) = common::create_test_user(&pool, "testuser", "test@example.com", true).await;

    let details = serde_json::json!({"reason": "incorrect_password", "attempts": 3});
    let _ = sqlx::query!(
        "INSERT INTO audit_logs (user_id, event_type, event_action, status, details)
         VALUES ($1, $2, $3, $4, $5)",
        user_id, "FAILED_LOGIN", "Multiple failed attempts", "failed", details
    )
    .execute(&pool)
    .await;

    let logs = sqlx::query!(
        "SELECT details FROM audit_logs WHERE user_id = $1",
        user_id
    )
    .fetch_all(&pool)
    .await
    .expect("Failed to fetch logs");

    assert_eq!(logs.len(), 1);
    assert!(logs[0].details.is_some());
}

#[actix_web::test]
async fn test_audit_log_with_null_user_id() {
    let pool = common::setup_test_db().await;

    let _ = sqlx::query!(
        "INSERT INTO audit_logs (user_id, event_type, event_action, ip_address, user_agent, status)
         VALUES ($1, $2, $3, $4, $5, $6)",
        None::<i32>, "FAILED_LOGIN", "Failed login: unknown user", "10.20.30.40", "Unknown Agent", "failed"
    )
    .execute(&pool)
    .await;

    let logs = sqlx::query!(
        "SELECT user_id, event_type FROM audit_logs WHERE user_id IS NULL"
    )
    .fetch_all(&pool)
    .await
    .expect("Failed to fetch logs");

    assert!(!logs.is_empty());
    let found = logs.iter().any(|log| log.event_type == "FAILED_LOGIN");
    assert!(found);
}

#[actix_web::test]
async fn test_audit_log_timestamp_is_set() {
    let pool = common::setup_test_db().await;

    let (user_id, _, _) = common::create_test_user(&pool, "testuser", "test@example.com", true).await;

    let before = chrono::Utc::now();
    
    let _ = sqlx::query!(
        "INSERT INTO audit_logs (user_id, event_type, event_action, status)
         VALUES ($1, $2, $3, $4)",
        user_id, "LOGIN", "Test login", "success"
    )
    .execute(&pool)
    .await;

    let after = chrono::Utc::now();

    let logs = sqlx::query!(
        "SELECT created_at FROM audit_logs WHERE user_id = $1",
        user_id
    )
    .fetch_all(&pool)
    .await
    .expect("Failed to fetch audit logs");

    assert_eq!(logs.len(), 1);
    let created_at = logs[0].created_at.unwrap_or_else(|| chrono::NaiveDateTime::from_timestamp_opt(0, 0).unwrap());
    let log_timestamp = chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(created_at, chrono::Utc);
    
    assert!(log_timestamp >= before);
    assert!(log_timestamp <= after);
}
