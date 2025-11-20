use crate::services::turbo_smtp::TurboSmtpService;
use std::collections::HashMap;
use std::sync::Mutex;
use lazy_static::lazy_static;
use chrono::{DateTime, Utc, Duration};

lazy_static! {
    static ref VERIFICATION_CODES: Mutex<HashMap<String, (String, DateTime<Utc>)>> = Mutex::new(HashMap::new());
    static ref PASSWORD_RESET_TOKENS: Mutex<HashMap<String, (String, DateTime<Utc>)>> = Mutex::new(HashMap::new()); // token -> (email, expires_at)
}

pub struct EmailService {
    smtp_service: TurboSmtpService,
}

impl EmailService {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        println!("Initializing Turbo SMTP email service");
        let smtp_service = TurboSmtpService::new()?;

        Ok(Self { smtp_service })
    }

    pub async fn send_verification_email(
        &self,
        to_email: &str,
        verification_code: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.smtp_service.send_verification_email(to_email, verification_code).await
    }

    // Delegate other methods to GmailSmtpService for now
    // These methods are specific to the in-memory storage approach
    pub fn generate_verification_code() -> String {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        format!("{:06}", rng.gen_range(100000..999999))
    }

    pub fn store_verification_code(email: &str, code: &str) {
        let mut codes = VERIFICATION_CODES.lock().unwrap();
        // Remove any old code for this email first
        codes.remove(email);
        let expires_at = Utc::now() + Duration::minutes(3);
        codes.insert(email.to_string(), (code.to_string(), expires_at));
        println!("Stored verification code {} for email {} (expires in 3 minutes)", code, email);
    }

    pub fn verify_code(email: &str, code: &str) -> bool {
        let mut codes = VERIFICATION_CODES.lock().unwrap();
        if let Some((stored_code, expires_at)) = codes.get(email) {
            if Utc::now() > *expires_at {
                codes.remove(email);
                return false;
            }
            if stored_code == code {
                codes.remove(email);
                return true;
            }
        }
        false
    }

    pub fn get_debug_codes() -> std::collections::HashMap<String, String> {
        let codes = VERIFICATION_CODES.lock().unwrap();
        let mut result = std::collections::HashMap::new();

        for (email, (code, expires_at)) in codes.iter() {
            if Utc::now() < *expires_at {
                result.insert(email.clone(), code.clone());
            }
        }

        result
    }

    // Password Reset Methods
    pub fn generate_password_reset_token() -> String {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        // Generate a secure 32-character token
        let token: String = (0..32)
            .map(|_| {
                let chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
                chars.chars().nth(rng.gen_range(0..chars.len())).unwrap()
            })
            .collect();
        token
    }

    pub fn store_password_reset_token(email: &str, token: &str) {
        let mut tokens = PASSWORD_RESET_TOKENS.lock().unwrap();
        
        // Delete any existing tokens for this email
        tokens.retain(|_, (stored_email, _)| stored_email != email);
        
        let expires_at = Utc::now() + Duration::hours(1); // 1 hour expiry
        tokens.insert(token.to_string(), (email.to_string(), expires_at));
        println!("Stored password reset token for email {}", email);
    }

    pub fn verify_password_reset_token(token: &str) -> Option<String> {
        let mut tokens = PASSWORD_RESET_TOKENS.lock().unwrap();
        if let Some((email, expires_at)) = tokens.get(token) {
            if Utc::now() > *expires_at {
                tokens.remove(token);
                return None;
            }
            let email = email.clone();
            tokens.remove(token); // Token is single-use
            return Some(email);
        }
        None
    }

    pub fn get_debug_password_reset_tokens() -> std::collections::HashMap<String, String> {
        let tokens = PASSWORD_RESET_TOKENS.lock().unwrap();
        let mut result = std::collections::HashMap::new();

        for (token, (email, expires_at)) in tokens.iter() {
            if Utc::now() < *expires_at {
                result.insert(token.clone(), email.clone());
            }
        }

        result
    }

    pub async fn send_password_reset_email(
        &self,
        to_email: &str,
        reset_token: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.smtp_service.send_password_reset_email(to_email, reset_token).await
    }
}