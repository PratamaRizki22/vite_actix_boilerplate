mod common;

use backend::services::refresh_token_service::RefreshTokenService;
use uuid::Uuid;
use chrono;

#[actix_web::test]
async fn test_generate_refresh_token() {
    let pool = common::setup_test_db().await;
    
    let unique_user = format!("testuser_{}", uuid::Uuid::new_v4());
    let (user_id, _, _) = common::create_test_user(&pool, &unique_user, &format!("test_{}@example.com", uuid::Uuid::new_v4()), true).await;
    
    // Generate refresh token
    let token = RefreshTokenService::generate_token();
    
    assert!(!token.is_empty());
    assert_eq!(token.len(), 36); // UUID format with hyphens
}

#[actix_web::test]
async fn test_create_refresh_token() {
    let pool = common::setup_test_db().await;
    
    let (user_id, _, _) = common::create_test_user(&pool, &format!("testuser_{}", uuid::Uuid::new_v4()), &format!("test_{}@example.com", uuid::Uuid::new_v4()), true).await;
    
    let token = RefreshTokenService::generate_token();
    
    // Create refresh token
    let result = RefreshTokenService::create_refresh_token(
        &pool,
        user_id,
        &token,
    ).await;
    
    assert!(result.is_ok());
    
    // Verify it was inserted
    let records = sqlx::query!(
        "SELECT token_hash, token_family FROM refresh_tokens WHERE user_id = $1",
        user_id
    )
    .fetch_all(&pool)
    .await
    .expect("Failed to fetch");
    
    assert_eq!(records.len(), 1);
    assert!(!records[0].token_hash.is_empty());
}

#[actix_web::test]
async fn test_verify_valid_refresh_token() {
    let pool = common::setup_test_db().await;
    
    let (user_id, _, _) = common::create_test_user(&pool, &format!("testuser_{}", uuid::Uuid::new_v4()), &format!("test_{}@example.com", uuid::Uuid::new_v4()), true).await;
    
    let token = RefreshTokenService::generate_token();
    
    // Create token
    let _ = RefreshTokenService::create_refresh_token(
        &pool,
        user_id,
        &token,
    ).await;
    
    // Verify token
    let is_valid = RefreshTokenService::verify_refresh_token(
        &pool,
        user_id,
        &token,
    )
    .await
    .expect("Failed to verify");
    
    assert!(is_valid);
}

#[actix_web::test]
async fn test_verify_invalid_token() {
    let pool = common::setup_test_db().await;
    
    let (user_id, _, _) = common::create_test_user(&pool, &format!("testuser_{}", uuid::Uuid::new_v4()), &format!("test_{}@example.com", uuid::Uuid::new_v4()), true).await;
    
    let invalid_token = RefreshTokenService::generate_token();
    
    // Try to verify non-existent token
    let is_valid = RefreshTokenService::verify_refresh_token(
        &pool,
        user_id,
        &invalid_token,
    )
    .await
    .expect("Failed to verify");
    
    assert!(!is_valid);
}

#[actix_web::test]
async fn test_revoke_token() {
    let pool = common::setup_test_db().await;
    
    let (user_id, _, _) = common::create_test_user(&pool, &format!("testuser_{}", uuid::Uuid::new_v4()), &format!("test_{}@example.com", uuid::Uuid::new_v4()), true).await;
    
    let token = RefreshTokenService::generate_token();
    
    // Create token
    let _ = RefreshTokenService::create_refresh_token(
        &pool,
        user_id,
        &token,
    ).await;
    
    // Verify it works
    let is_valid_before = RefreshTokenService::verify_refresh_token(
        &pool,
        user_id,
        &token,
    )
    .await
    .expect("Failed to verify");
    assert!(is_valid_before);
    
    // Revoke token
    let _ = RefreshTokenService::revoke_token(
        &pool,
        user_id,
        &token,
    ).await;
    
    // Verify it no longer works
    let is_valid_after = RefreshTokenService::verify_refresh_token(
        &pool,
        user_id,
        &token,
    )
    .await
    .expect("Failed to verify");
    assert!(!is_valid_after);
}

#[actix_web::test]
async fn test_rotate_refresh_token() {
    let pool = common::setup_test_db().await;
    
    let (user_id, _, _) = common::create_test_user(&pool, &format!("testuser_{}", uuid::Uuid::new_v4()), &format!("test_{}@example.com", uuid::Uuid::new_v4()), true).await;
    
    let old_token = RefreshTokenService::generate_token();
    let new_token = RefreshTokenService::generate_token();
    
    // Create initial token
    let _ = RefreshTokenService::create_refresh_token(
        &pool,
        user_id,
        &old_token,
    ).await;
    
    // Rotate token
    let result = RefreshTokenService::rotate_refresh_token(
        &pool,
        user_id,
        &old_token,
        &new_token,
    ).await;
    
    assert!(result.is_ok());
    
    // Verify new token works
    let is_new_valid = RefreshTokenService::verify_refresh_token(
        &pool,
        user_id,
        &new_token,
    )
    .await
    .expect("Failed to verify");
    
    assert!(is_new_valid);
    
    // Verify parent relationship
    let records = sqlx::query!(
        "SELECT parent_token_hash FROM refresh_tokens WHERE user_id = $1 ORDER BY created_at DESC LIMIT 1",
        user_id
    )
    .fetch_all(&pool)
    .await
    .expect("Failed to fetch");
    
    assert_eq!(records.len(), 1);
    assert!(records[0].parent_token_hash.is_some());
}

#[actix_web::test]
async fn test_detect_token_reuse() {
    let pool = common::setup_test_db().await;
    
    let (user_id, _, _) = common::create_test_user(&pool, &format!("testuser_{}", uuid::Uuid::new_v4()), &format!("test_{}@example.com", uuid::Uuid::new_v4()), true).await;
    
    let token1 = RefreshTokenService::generate_token();
    let token2 = RefreshTokenService::generate_token();
    
    // Create and rotate
    let _ = RefreshTokenService::create_refresh_token(
        &pool,
        user_id,
        &token1,
    ).await;
    
    let _ = RefreshTokenService::rotate_refresh_token(
        &pool,
        user_id,
        &token1,
        &token2,
    ).await;
    
    // Try to reuse old token - should detect attack
    let reuse_detected = RefreshTokenService::detect_reuse_attack(
        &pool,
        user_id,
        &token1,
    )
    .await
    .expect("Failed to detect");
    
    assert!(reuse_detected);
    
    // Verify all tokens in family are marked as compromised
    let compromised = sqlx::query!(
        "SELECT reuse_detected FROM refresh_tokens WHERE user_id = $1 AND reuse_detected = TRUE",
        user_id
    )
    .fetch_all(&pool)
    .await
    .expect("Failed to fetch");
    
    assert!(!compromised.is_empty());
}

#[actix_web::test]
async fn test_revoke_all_tokens() {
    let pool = common::setup_test_db().await;
    
    let (user_id, _, _) = common::create_test_user(&pool, &format!("testuser_{}", uuid::Uuid::new_v4()), &format!("test_{}@example.com", uuid::Uuid::new_v4()), true).await;
    
    // Create multiple tokens
    let token1 = RefreshTokenService::generate_token();
    let token2 = RefreshTokenService::generate_token();
    
    let _ = RefreshTokenService::create_refresh_token(
        &pool,
        user_id,
        &token1,
    ).await;
    
    let _ = RefreshTokenService::create_refresh_token(
        &pool,
        user_id,
        &token2,
    ).await;
    
    // Revoke all
    let _ = RefreshTokenService::revoke_all_tokens(
        &pool,
        user_id,
    ).await;
    
    // Verify both are revoked
    let is_token1_valid = RefreshTokenService::verify_refresh_token(
        &pool,
        user_id,
        &token1,
    )
    .await
    .expect("Failed to verify");
    
    let is_token2_valid = RefreshTokenService::verify_refresh_token(
        &pool,
        user_id,
        &token2,
    )
    .await
    .expect("Failed to verify");
    
    assert!(!is_token1_valid);
    assert!(!is_token2_valid);
}

#[actix_web::test]
async fn test_get_active_token_count() {
    let pool = common::setup_test_db().await;
    
    let (user_id, _, _) = common::create_test_user(&pool, &format!("testuser_{}", uuid::Uuid::new_v4()), &format!("test_{}@example.com", uuid::Uuid::new_v4()), true).await;
    
    // Initially no tokens
    let count = RefreshTokenService::get_active_token_count(
        &pool,
        user_id,
    )
    .await
    .expect("Failed to count");
    
    assert_eq!(count, 0);
    
    // Create tokens
    let token1 = RefreshTokenService::generate_token();
    let token2 = RefreshTokenService::generate_token();
    
    let _ = RefreshTokenService::create_refresh_token(
        &pool,
        user_id,
        &token1,
    ).await;
    
    let _ = RefreshTokenService::create_refresh_token(
        &pool,
        user_id,
        &token2,
    ).await;
    
    // Now should have 2
    let count = RefreshTokenService::get_active_token_count(
        &pool,
        user_id,
    )
    .await
    .expect("Failed to count");
    
    assert_eq!(count, 2);
}

#[actix_web::test]
async fn test_cleanup_expired_tokens() {
    let pool = common::setup_test_db().await;
    
    let (user_id, _, _) = common::create_test_user(&pool, &format!("testuser_{}", uuid::Uuid::new_v4()), &format!("test_{}@example.com", uuid::Uuid::new_v4()), true).await;
    
    let token = RefreshTokenService::generate_token();
    
    // Create token with past expiry (using direct SQL)
    let past_expires = chrono::Utc::now() - chrono::Duration::days(1);
    
    let _ = sqlx::query!(
        "INSERT INTO refresh_tokens (user_id, token_hash, token_family, expires_at)
         VALUES ($1, $2, $3, $4)",
        user_id,
        RefreshTokenService::hash_token(&token),
        uuid::Uuid::new_v4().to_string(),
        past_expires.naive_utc()
    )
    .execute(&pool)
    .await;
    
    // Cleanup
    let deleted = RefreshTokenService::cleanup_expired_tokens(
        &pool,
    )
    .await
    .expect("Failed to cleanup");
    
    assert!(deleted > 0);
}

#[actix_web::test]
async fn test_token_family_isolation() {
    let pool = common::setup_test_db().await;
    
    let (user_id, _, _) = common::create_test_user(&pool, &format!("testuser_{}", uuid::Uuid::new_v4()), &format!("test_{}@example.com", uuid::Uuid::new_v4()), true).await;
    
    let token1 = RefreshTokenService::generate_token();
    let token2 = RefreshTokenService::generate_token();
    let token3 = RefreshTokenService::generate_token();
    
    // Create two separate families
    let _ = RefreshTokenService::create_refresh_token(
        &pool,
        user_id,
        &token1,
    ).await;
    
    let _ = RefreshTokenService::create_refresh_token(
        &pool,
        user_id,
        &token2,
    ).await;
    
    // Rotate token1 -> token3
    let family1 = RefreshTokenService::rotate_refresh_token(
        &pool,
        user_id,
        &token1,
        &token3,
    )
    .await
    .expect("Failed to rotate");
    
    // Get family info
    let token2_family = sqlx::query!(
        "SELECT token_family FROM refresh_tokens WHERE user_id = $1 AND token_hash = $2",
        user_id,
        RefreshTokenService::hash_token(&token2)
    )
    .fetch_one(&pool)
    .await
    .expect("Failed to fetch");
    
    // Families should be different
    assert_ne!(family1, token2_family.token_family);
    
    // Revoking one family should not affect the other
    let _ = RefreshTokenService::revoke_token_family(
        &pool,
        user_id,
        &family1,
    ).await;
    
    // token2 should still be valid (different family)
    let is_token2_valid = RefreshTokenService::verify_refresh_token(
        &pool,
        user_id,
        &token2,
    )
    .await
    .expect("Failed to verify");
    
    assert!(is_token2_valid);
}

#[actix_web::test]
async fn test_token_hash_consistency() {
    let token = "test-token-12345";
    
    let hash1 = RefreshTokenService::hash_token(token);
    let hash2 = RefreshTokenService::hash_token(token);
    
    // Same token should always produce same hash
    assert_eq!(hash1, hash2);
    
    // Hash should be deterministic
    assert!(!hash1.is_empty());
    assert_eq!(hash1.len(), 64); // SHA256 produces 64 hex characters
}
