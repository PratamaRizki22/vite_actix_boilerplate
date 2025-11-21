use totp_rs::{TOTP, Algorithm, Secret};

pub fn verify_totp_code(secret: &str, code: &str) -> Result<bool, Box<dyn std::error::Error>> {
    // Validate that code is 6 digits
    if code.len() != 6 || !code.chars().all(|c| c.is_digit(10)) {
        return Ok(false);
    }

    // Parse the secret from base32
    let secret_bytes = Secret::Encoded(secret.to_string())
        .to_bytes()
        .map_err(|e| format!("Failed to decode secret: {}", e))?;

    // Create TOTP instance with standard parameters
    let totp = TOTP::new(
        Algorithm::SHA1,
        6,      // 6 digits
        1,      // 1 step tolerance (allows codes from previous/next 30-second window)
        30,     // 30 seconds period
        secret_bytes,
        Some("USH".to_string()),  // issuer
        "user".to_string(),        // account name
    )?;

    // Verify the code
    let is_valid = totp.check_current(code)?;
    
    println!("\n=== TOTP Verification (utils/totp.rs) ===");
    println!("  Secret (base32): {}", secret);
    println!("  Code provided: {}", code);
    println!("  Is valid: {}", is_valid);
    
    // Always show what the expected code is for debugging
    match totp.generate_current() {
        Ok(current_code) => {
            println!("  Expected code: {}", current_code);
            println!("  Match: {}", if current_code == code { "✅ YES" } else { "❌ NO" });
        }
        Err(e) => println!("  Error generating code: {:?}", e),
    }
    println!("=== End TOTP Verification ===\n");
    
    Ok(is_valid)
}

pub fn generate_totp_secret() -> Result<String, Box<dyn std::error::Error>> {
    // Generate a random 32-byte secret and encode as base32
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let secret_bytes: Vec<u8> = (0..32).map(|_| rng.gen_range(0..=255)).collect();
    Ok(base32::encode(base32::Alphabet::RFC4648 { padding: false }, &secret_bytes))
}