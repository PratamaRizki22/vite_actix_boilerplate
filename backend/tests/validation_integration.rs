use backend::utils::validation::{
    validate_username, validate_email, validate_password, validate_wallet_address, validate_length
};

#[test]
fn test_username_validation_valid() {
    assert!(validate_username("john_doe").is_ok());
    assert!(validate_username("user123").is_ok());
    assert!(validate_username("test_user").is_ok());
}

#[test]
fn test_username_validation_too_short() {
    assert!(validate_username("ab").is_err());
}

#[test]
fn test_username_validation_too_long() {
    assert!(validate_username(&"a".repeat(51)).is_err());
}

#[test]
fn test_username_validation_invalid_chars() {
    assert!(validate_username("user@domain").is_err());
    assert!(validate_username("user name").is_err());
    assert!(validate_username("user#").is_err());
}

#[test]
fn test_email_validation_valid() {
    assert!(validate_email("user@example.com").is_ok());
    assert!(validate_email("john.doe@company.co.uk").is_ok());
    assert!(validate_email("test+tag@domain.org").is_ok());
}

#[test]
fn test_email_validation_invalid() {
    assert!(validate_email("not-an-email").is_err());
    assert!(validate_email("user@").is_err());
    assert!(validate_email("@domain.com").is_err());
    assert!(validate_email("user@domain").is_err());
}

#[test]
fn test_password_validation_valid() {
    assert!(validate_password("SecurePass123").is_ok());
    assert!(validate_password("MyPassword2025!").is_ok());
}

#[test]
fn test_password_validation_too_short() {
    assert!(validate_password("Pass1").is_err());
}

#[test]
fn test_password_validation_too_long() {
    assert!(validate_password(&"a".repeat(129)).is_err());
}

#[test]
fn test_password_validation_missing_uppercase() {
    assert!(validate_password("lowercase123").is_err());
}

#[test]
fn test_password_validation_missing_lowercase() {
    assert!(validate_password("UPPERCASE123").is_err());
}

#[test]
fn test_password_validation_missing_digit() {
    assert!(validate_password("PasswordNoDigit").is_err());
}

#[test]
fn test_wallet_address_validation_valid() {
    assert!(validate_wallet_address("0x1234567890123456789012345678901234567890").is_ok());
    assert!(validate_wallet_address("0xABCDEF1234567890ABCDEF1234567890ABCDEF12").is_ok());
}

#[test]
fn test_wallet_address_validation_no_prefix() {
    assert!(validate_wallet_address("1234567890123456789012345678901234567890").is_err());
}

#[test]
fn test_wallet_address_validation_wrong_length() {
    assert!(validate_wallet_address("0x123456789012345678901234567890123456789").is_err());
    assert!(validate_wallet_address("0x12345678901234567890123456789012345678901").is_err());
}

#[test]
fn test_wallet_address_validation_invalid_chars() {
    assert!(validate_wallet_address("0x1234567890123456789012345678901234567G90").is_err());
}

#[test]
fn test_length_validation() {
    assert!(validate_length("test", 1, 10).is_ok());
    assert!(validate_length("test", 4, 4).is_ok());
    assert!(validate_length("", 1, 10).is_err());
    assert!(validate_length(&"a".repeat(11), 1, 10).is_err());
}
