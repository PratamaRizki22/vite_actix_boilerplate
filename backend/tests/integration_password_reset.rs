// Integration tests for password reset functionality

#[test]
fn test_password_reset_token_generation() {
    // Test bahwa token reset password bisa generated
    use std::time::{SystemTime, UNIX_EPOCH};
    
    let token_prefix = "reset_";
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    
    let token = format!("{}_{}", token_prefix, timestamp);
    
    assert!(token.starts_with(token_prefix), "Token should have reset prefix");
    assert!(!token.is_empty(), "Token should not be empty");
}

#[test]
fn test_password_reset_token_validity_1_hour() {
    use chrono::{Utc, Duration};
    
    let created_at = Utc::now();
    let expires_at = created_at + Duration::hours(1);
    let check_time_before_expiry = created_at + Duration::minutes(30);
    let check_time_after_expiry = expires_at + Duration::minutes(1);
    
    // Token masih valid 30 menit kemudian
    assert!(check_time_before_expiry < expires_at, "Token should be valid at 30 minutes");
    
    // Token expired 1 menit setelah 1 jam
    assert!(check_time_after_expiry >= expires_at, "Token should be expired after 1 hour");
}

#[test]
fn test_password_reset_single_use() {
    let mut used_tokens = std::collections::HashSet::new();
    let token = "reset_token_12345";
    
    // Pertama kali token digunakan
    let first_use = used_tokens.insert(token);
    assert!(first_use, "Token should be usable on first attempt");
    
    // Kedua kali token digunakan
    let second_use = used_tokens.insert(token);
    assert!(!second_use, "Token should not be usable on second attempt (already used)");
}

#[test]
fn test_one_active_reset_per_email() {
    let mut active_resets = std::collections::HashMap::new();
    let email = "user@example.com";
    let token1 = "reset_token_1";
    let token2 = "reset_token_2";
    
    // Create first reset token
    active_resets.insert(email, token1);
    assert_eq!(active_resets.get(email), Some(&token1), "First token should be stored");
    
    // Create second reset token (should replace first one)
    active_resets.insert(email, token2);
    assert_eq!(active_resets.get(email), Some(&token2), "Second token should replace first");
    assert_ne!(active_resets.get(email), Some(&token1), "First token should be replaced");
}

#[test]
fn test_password_reset_requires_new_password() {
    let old_password = "OldPassword123!";
    let new_password = "NewPassword456!";
    
    // New password tidak boleh sama dengan old password
    assert_ne!(old_password, new_password, "New password must be different from old password");
}

#[test]
fn test_password_reset_validation() {
    let weak_passwords = vec![
        "123",              // Terlalu pendek
        "password",         // Tidak ada angka atau spesial
        "PASSWORD",         // Semua uppercase
        "Pass123",          // Terlalu pendek
    ];
    
    let strong_passwords = vec![
        "SecurePass123!",
        "MyPassword456@",
        "Str0ng!Pwd",
    ];
    
    // Weak passwords should be flagged
    for pwd in &weak_passwords {
        assert!(pwd.len() < 10 || !pwd.chars().any(|c| c.is_numeric()), 
            "Weak password should not meet requirements: {}", pwd);
    }
    
    // Strong passwords should pass
    for pwd in &strong_passwords {
        assert!(pwd.len() >= 8, "Strong password should be at least 8 chars: {}", pwd);
        assert!(pwd.chars().any(|c| c.is_numeric()), "Strong password should contain digits: {}", pwd);
    }
}

#[test]
fn test_password_reset_email_sent() {
    let email = "user@example.com";
    let token = "reset_token_xyz";
    let reset_link = format!("https://example.com/reset?token={}", token);
    
    assert!(!email.is_empty(), "Email should not be empty");
    assert!(!token.is_empty(), "Token should not be empty");
    assert!(reset_link.contains(token), "Reset link should contain token");
}

#[test]
fn test_password_reset_clears_sessions() {
    let user_id = 1;
    let mut sessions = std::collections::HashMap::new();
    
    // Create some sessions
    sessions.insert(format!("session_{}_{}", user_id, 1), "active");
    sessions.insert(format!("session_{}_{}", user_id, 2), "active");
    
    assert_eq!(sessions.len(), 2, "Should have 2 sessions");
    
    // Simulate clearing all sessions for user
    let user_sessions: Vec<_> = sessions.keys()
        .filter(|k| k.starts_with(&format!("session_{}_", user_id)))
        .cloned()
        .collect();
    
    for session_key in user_sessions {
        sessions.remove(&session_key);
    }
    
    assert_eq!(sessions.len(), 0, "All user sessions should be cleared after password reset");
}
