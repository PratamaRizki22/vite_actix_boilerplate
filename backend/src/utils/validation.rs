/// Input validation utilities for security

pub fn validate_username(username: &str) -> Result<&str, String> {
    if username.is_empty() {
        return Err("Username cannot be empty".to_string());
    }
    
    if username.len() < 3 {
        return Err("Username must be at least 3 characters".to_string());
    }
    
    if username.len() > 50 {
        return Err("Username must not exceed 50 characters".to_string());
    }
    
    // Only alphanumeric and underscore
    if !username.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return Err("Username can only contain alphanumeric characters and underscores".to_string());
    }
    
    Ok(username)
}

pub fn validate_email(email: &str) -> Result<&str, String> {
    if email.is_empty() {
        return Err("Email cannot be empty".to_string());
    }
    
    if email.len() > 254 {
        return Err("Email must not exceed 254 characters".to_string());
    }
    
    // Basic email validation
    if !email.contains('@') || !email.contains('.') {
        return Err("Invalid email format".to_string());
    }
    
    let parts: Vec<&str> = email.split('@').collect();
    if parts.len() != 2 {
        return Err("Invalid email format".to_string());
    }
    
    if parts[0].is_empty() || parts[1].is_empty() {
        return Err("Invalid email format".to_string());
    }
    
    Ok(email)
}

pub fn validate_password(password: &str) -> Result<&str, String> {
    if password.is_empty() {
        return Err("Password cannot be empty".to_string());
    }
    
    if password.len() < 8 {
        return Err("Password must be at least 8 characters".to_string());
    }
    
    if password.len() > 128 {
        return Err("Password must not exceed 128 characters".to_string());
    }
    
    // Check for at least one uppercase letter
    if !password.chars().any(|c| c.is_uppercase()) {
        return Err("Password must contain at least one uppercase letter".to_string());
    }
    
    // Check for at least one lowercase letter
    if !password.chars().any(|c| c.is_lowercase()) {
        return Err("Password must contain at least one lowercase letter".to_string());
    }
    
    // Check for at least one number
    if !password.chars().any(|c| c.is_numeric()) {
        return Err("Password must contain at least one number".to_string());
    }
    
    Ok(password)
}

pub fn validate_wallet_address(address: &str) -> Result<&str, String> {
    if address.is_empty() {
        return Err("Wallet address cannot be empty".to_string());
    }
    
    // Ethereum address format: 0x followed by 40 hex characters
    if !address.starts_with("0x") {
        return Err("Wallet address must start with 0x".to_string());
    }
    
    if address.len() != 42 {
        return Err("Wallet address must be 42 characters (0x + 40 hex)".to_string());
    }
    
    // Check if all characters after 0x are valid hex
    if !address[2..].chars().all(|c| c.is_ascii_hexdigit()) {
        return Err("Wallet address contains invalid hexadecimal characters".to_string());
    }
    
    Ok(address)
}

pub fn validate_length(input: &str, min: usize, max: usize) -> Result<&str, String> {
    let len = input.len();
    
    if len < min {
        return Err(format!("Input must be at least {} characters", min));
    }
    
    if len > max {
        return Err(format!("Input must not exceed {} characters", max));
    }
    
    Ok(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_username() {
        assert!(validate_username("validuser").is_ok());
        assert!(validate_username("user_123").is_ok());
        assert!(validate_username("ab").is_err()); // too short
        assert!(validate_username("user@name").is_err()); // invalid char
    }

    #[test]
    fn test_validate_email() {
        assert!(validate_email("test@example.com").is_ok());
        assert!(validate_email("invalid@").is_err());
        assert!(validate_email("invalid.com").is_err());
    }

    #[test]
    fn test_validate_password() {
        assert!(validate_password("SecurePass123").is_ok());
        assert!(validate_password("weak").is_err()); // too short
        assert!(validate_password("nouppercase123").is_err());
        assert!(validate_password("NoNumbers").is_err());
    }

    #[test]
    fn test_validate_wallet_address() {
        assert!(validate_wallet_address("0x1234567890123456789012345678901234567890").is_ok());
        assert!(validate_wallet_address("1234567890123456789012345678901234567890").is_err()); // no 0x
        assert!(validate_wallet_address("0xZZZZ567890123456789012345678901234567890").is_err()); // invalid hex
    }

    #[test]
    fn test_validate_length() {
        assert!(validate_length("hello", 1, 10).is_ok());
        assert!(validate_length("hi", 3, 10).is_err()); // too short
        assert!(validate_length("this is too long", 1, 10).is_err()); // too long
    }
}
