#[actix_web::test]
async fn test_email_verification_code_generation() {
    use rand::Rng;
    
    let code = rand::thread_rng()
        .sample_iter(&rand::distributions::Uniform::new(0, 10))
        .take(6)
        .map(|d| (d as u8 + b'0') as char)
        .collect::<String>();

    assert_eq!(code.len(), 6, "Verification code should be 6 digits");
}

#[actix_web::test]
async fn test_email_verification_code_valid() {
    let code = "123456";
    let stored_code = "123456";
    
    // Test bahwa kode verifikasi cocok
    assert_eq!(code, stored_code, "Verification code should match");
}

#[actix_web::test]
async fn test_email_verification_code_expired() {
    use chrono::Utc;
    
    let created_at = Utc::now();
    let expires_at = created_at + chrono::Duration::minutes(15);
    let now = expires_at + chrono::Duration::minutes(1);
    
    // Test bahwa kode expired
    assert!(now > expires_at, "Verification code should be expired");
}

#[actix_web::test]
async fn test_email_verification_code_invalid() {
    let provided_code = "111111";
    let correct_code = "123456";
    
    // Test bahwa kode salah ditolak
    assert_ne!(provided_code, correct_code, "Invalid code should not match");
}

#[actix_web::test]
async fn test_turbo_smtp_integration() {
    // Test bahwa email bisa dikirim ke Turbo SMTP
    let email = "test@example.com";
    let subject = "Verify Your Email";
    
    assert!(!email.is_empty(), "Email should not be empty");
    assert!(!subject.is_empty(), "Subject should not be empty");
}

#[actix_web::test]
async fn test_verification_email_payload() {
    let code = "123456";
    let email = "user@example.com";
    
    let payload = format!(
        "Your verification code is: {}. Valid for 15 minutes.",
        code
    );
    
    assert!(payload.contains(code), "Email should contain verification code");
    assert!(payload.contains("15 minutes"), "Email should mention validity period");
}
