use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use std::env;

pub struct TurboSmtpService {
    smtp_username: String,
    smtp_password: String,
    smtp_server: String,
    smtp_port: u16,
}

impl TurboSmtpService {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let smtp_username = env::var("TURBO_SMTP_USERNAME")
            .map_err(|_| "TURBO_SMTP_USERNAME environment variable not set")?;
        let smtp_password = env::var("TURBO_SMTP_PASSWORD")
            .map_err(|_| "TURBO_SMTP_PASSWORD environment variable not set")?;
        let smtp_server = env::var("TURBO_SMTP_SERVER")
            .unwrap_or_else(|_| "pro.turbo-smtp.com".to_string());
        let smtp_port = env::var("TURBO_SMTP_PORT")
            .unwrap_or_else(|_| "587".to_string())
            .parse()
            .unwrap_or(587);

        println!("TurboSMTP Config:");
        println!("  Username: {}", smtp_username);
        println!("  Server: {}", smtp_server);
        println!("  Port: {}", smtp_port);
        println!("  Password: [HIDDEN]");

        Ok(Self {
            smtp_username,
            smtp_password,
            smtp_server,
            smtp_port,
        })
    }

    pub async fn send_verification_email(
        &self,
        to_email: &str,
        verification_code: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        println!("Sending verification email to: {} via Turbo SMTP", to_email);

        let html_body = format!(
            r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>Email Verification</title>
</head>
<body style="font-family: Arial, sans-serif; line-height: 1.6; color: #333;">
    <div style="max-width: 600px; margin: 0 auto; padding: 20px;">
        <h2 style="color: #4F46E5;">Welcome to MyApp!</h2>
        <p>Thank you for registering. To complete your account setup, please verify your email address.</p>

        <div style="background-color: #F3F4F6; padding: 20px; border-radius: 8px; margin: 20px 0;">
            <h3 style="margin-top: 0; color: #1F2937;">Your Verification Code:</h3>
            <div style="font-size: 32px; font-weight: bold; color: #4F46E5; text-align: center; letter-spacing: 4px;">
                {}
            </div>
        </div>

        <p><strong>This code will expire in 10 minutes.</strong></p>
        <p>If you didn't request this verification, please ignore this email.</p>

        <hr style="border: none; border-top: 1px solid #E5E7EB; margin: 30px 0;">
        <p style="color: #6B7280; font-size: 14px;">
            This is an automated message from MyApp. Please do not reply to this email.
        </p>
    </div>
</body>
</html>"#,
            verification_code
        );

        let from_email = "MyApp <rizkipurnomo914@gmail.com>";
        let to_email_full = to_email;

        println!("Attempting to send email:");
        println!("From: {}", from_email);
        println!("To: {}", to_email_full);
        println!("Server: {}:{}", self.smtp_server, self.smtp_port);
        println!("Username: {}", self.smtp_username);

        let email = Message::builder()
            .from(from_email.parse()?)
            .to(to_email_full.parse()?)
            .subject("Email Verification Code - MyApp")
            .singlepart(
                lettre::message::SinglePart::html(html_body)
            )?;

        let creds = Credentials::new(self.smtp_username.clone(), self.smtp_password.clone());

        // Turbo SMTP official configuration based on port
        let mailer = match self.smtp_port {
            465 | 25025 => {
                // SSL ports: 465, 25025
                println!("Using SSL/TLS on port {}", self.smtp_port);
                SmtpTransport::relay(&self.smtp_server)?
                    .port(self.smtp_port)
                    .credentials(creds)
                    .build()
            },
            25 | 587 | 2525 => {
                // Non-SSL ports: 25, 587, 2525 (plain SMTP)
                println!("Using plain SMTP on port {}", self.smtp_port);
                SmtpTransport::builder_dangerous(&self.smtp_server)
                    .port(self.smtp_port)
                    .credentials(creds)
                    .build()
            },
            _ => {
                // Default: plain SMTP
                println!("Using default plain SMTP on port {}", self.smtp_port);
                SmtpTransport::builder_dangerous(&self.smtp_server)
                    .port(self.smtp_port)
                    .credentials(creds)
                    .build()
            }
        };

        match mailer.send(&email) {
            Ok(response) => {
                println!("✅ Email sent successfully via Turbo SMTP!");
                println!("✅ Response: {:?}", response);
                println!("✅ From: {} To: {}", from_email, to_email);
                Ok(())
            }
            Err(e) => {
                println!("❌ Turbo SMTP error: {:?}", e);
                println!("❌ Error details: {}", e);
                println!("❌ From: {} To: {}", from_email, to_email);
                Err(format!("SMTP Error: {}", e).into())
            }
        }
    }

    pub async fn send_password_reset_email(
        &self,
        to_email: &str,
        reset_token: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        println!("Sending password reset email to: {} via Turbo SMTP", to_email);

        let html_body = format!(
            r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Password Reset - MyApp</title>
    <style>
        body {{ font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', 'Roboto', 'Oxygen', 'Ubuntu', 'Cantarell', sans-serif; }}
        .container {{ max-width: 600px; margin: 0 auto; padding: 20px; }}
        .button {{ background-color: #DC2626; color: white; padding: 12px 24px; text-decoration: none; border-radius: 5px; display: inline-block; font-weight: bold; }}
        .button:hover {{ background-color: #B91C1C; }}
        .footer {{ color: #666; font-size: 12px; margin-top: 30px; border-top: 1px solid #eee; padding-top: 20px; }}
    </style>
</head>
<body style="background-color: #F9FAFB; font-family: Arial, sans-serif; line-height: 1.6; color: #333;">
    <div style="background-color: white;">
        <div class="container">
            <h2 style="color: #DC2626; margin-bottom: 20px;">🔐 Password Reset Request</h2>
            
            <p>You have requested to reset your password for your MyApp account.</p>
            <p>If you didn't make this request, you can safely ignore this email.</p>
            
            <div style="text-align: center; margin: 30px 0;">
                <a href="http://localhost:5173/reset-password?token={}" 
                   style="background-color: #DC2626; color: white; padding: 12px 24px; text-decoration: none; border-radius: 5px; display: inline-block; font-weight: bold;">
                    Reset Your Password
                </a>
            </div>
            
            <p><strong>⏱️ Security Notice:</strong> This link will expire in <strong>1 hour</strong> for your security.</p>
            
            <p>If the button above doesn't work, copy and paste this link into your browser:</p>
            <p style="word-break: break-all; background-color: #F3F4F6; padding: 10px; border-radius: 5px; color: #1F2937;">
                http://localhost:5173/reset-password?token={}
            </p>
            
            <hr style="border: none; border-top: 1px solid #E5E7EB; margin: 30px 0;">
            
            <div class="footer">
                <p><strong>Didn't request this?</strong></p>
                <p>If you didn't request a password reset, your account is still secure. Someone else may have entered your email by mistake. Your password hasn't been changed.</p>
                <p style="margin-bottom: 0; color: #999;">This is an automated message from MyApp. Please do not reply to this email.</p>
            </div>
        </div>
    </div>
</body>
</html>"#,
            reset_token, reset_token
        );

        let from_email = "MyApp <rizkipurnomo914@gmail.com>";
        let to_email_full = to_email;

        let email = Message::builder()
            .from(from_email.parse()?)
            .to(to_email_full.parse()?)
            .subject("🔐 Password Reset - MyApp")
            .singlepart(
                lettre::message::SinglePart::html(html_body)
            )?;

        println!("Attempting to send password reset email:");
        println!("From: {}", from_email);
        println!("To: {}", to_email_full);
        println!("Server: {}:{}", self.smtp_server, self.smtp_port);
        println!("Username: {}", self.smtp_username);

        let creds = Credentials::new(self.smtp_username.clone(), self.smtp_password.clone());

        let mailer = if self.smtp_port == 465 {
            println!("Using SSL/TLS on port {}", self.smtp_port);
            SmtpTransport::relay(&self.smtp_server)?
                .credentials(creds)
                .build()
        } else if self.smtp_port == 587 {
            println!("Using STARTTLS on port {}", self.smtp_port);
            SmtpTransport::starttls_relay(&self.smtp_server)?
                .credentials(creds)
                .build()
        } else {
            println!("Using plain SMTP on port {}", self.smtp_port);
            SmtpTransport::builder_dangerous(&self.smtp_server)
                .credentials(creds)
                .build()
        };

        match mailer.send(&email) {
            Ok(response) => {
                println!("✅ Password reset email sent successfully via Turbo SMTP!");
                println!("✅ Response: {:?}", response);
                println!("✅ From: {} To: {}", from_email, to_email);
                Ok(())
            }
            Err(e) => {
                println!("❌ Turbo SMTP password reset email error: {:?}", e);
                println!("❌ Error details: {}", e);
                println!("❌ From: {} To: {}", from_email, to_email);
                Err(format!("SMTP Error: {}", e).into())
            }
        }
    }
}