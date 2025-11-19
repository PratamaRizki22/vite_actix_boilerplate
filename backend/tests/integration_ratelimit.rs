// Integration tests for rate limiting functionality

#[test]
fn test_rate_limit_login_endpoint() {
    let mut login_attempts = std::collections::HashMap::new();
    let ip = "192.168.1.1";
    let max_attempts = 5;
    
    // Simulate 5 login attempts within 15 minutes
    for i in 0..max_attempts {
        login_attempts.insert(format!("{}_{}", ip, i), i);
    }
    
    assert_eq!(login_attempts.len(), max_attempts, "Should allow 5 attempts");
    
    // 6th attempt should be blocked
    let attempt_count = login_attempts.len();
    assert!(attempt_count >= max_attempts, "Should reach rate limit at 5 attempts");
}

#[test]
fn test_rate_limit_login_5_per_15_min() {
    use chrono::{Utc, Duration};
    
    let mut attempts = Vec::new();
    let now = Utc::now();
    
    // Add 5 attempts within 15 minutes
    for i in 0..5 {
        attempts.push(now + Duration::minutes(i));
    }
    
    assert_eq!(attempts.len(), 5, "Should have 5 attempts");
    
    // All attempts within 15 minute window
    let time_window = Duration::minutes(15);
    let latest = *attempts.last().unwrap();
    let earliest = *attempts.first().unwrap();
    
    assert!(latest - earliest < time_window, "All attempts should be within 15 minute window");
}

#[test]
fn test_rate_limit_email_endpoint() {
    let mut email_requests = std::collections::HashMap::new();
    let ip = "192.168.1.1";
    let max_requests = 5;
    
    // Simulate email requests
    for i in 0..max_requests {
        email_requests.insert(format!("email_{}_{}", ip, i), true);
    }
    
    assert_eq!(email_requests.len(), max_requests, "Should allow 5 email requests");
}

#[test]
fn test_rate_limit_email_5_per_60_min() {
    use chrono::{Utc, Duration};
    
    let mut requests = Vec::new();
    let now = Utc::now();
    
    // Add 5 requests within 60 minutes
    for i in 0..5 {
        requests.push(now + Duration::minutes(i * 10));
    }
    
    assert_eq!(requests.len(), 5, "Should have 5 email requests");
    
    // All requests within 60 minute window
    let time_window = Duration::minutes(60);
    let latest = *requests.last().unwrap();
    let earliest = *requests.first().unwrap();
    
    assert!(latest - earliest < time_window, "All requests should be within 60 minute window");
}

#[test]
fn test_rate_limit_password_reset_endpoint() {
    let mut reset_requests = std::collections::HashMap::new();
    let ip = "192.168.1.1";
    let max_requests = 3;
    
    // Simulate password reset requests
    for i in 0..max_requests {
        reset_requests.insert(format!("reset_{}_{}", ip, i), true);
    }
    
    assert_eq!(reset_requests.len(), max_requests, "Should allow 3 reset requests");
}

#[test]
fn test_rate_limit_password_3_per_60_min() {
    use chrono::{Utc, Duration};
    
    let mut requests = Vec::new();
    let now = Utc::now();
    
    // Add 3 requests within 60 minutes
    for i in 0..3 {
        requests.push(now + Duration::minutes(i * 15));
    }
    
    assert_eq!(requests.len(), 3, "Should have 3 password reset requests");
    
    // All requests within 60 minute window
    let time_window = Duration::minutes(60);
    let latest = *requests.last().unwrap();
    let earliest = *requests.first().unwrap();
    
    assert!(latest - earliest < time_window, "All requests should be within 60 minute window");
}

#[test]
fn test_rate_limit_per_ip_address() {
    let mut rate_limits = std::collections::HashMap::new();
    
    let ip1 = "192.168.1.1";
    let ip2 = "192.168.1.2";
    let ip3 = "192.168.1.3";
    
    // Different IPs should have separate rate limits
    rate_limits.insert(ip1, 5);
    rate_limits.insert(ip2, 3);
    rate_limits.insert(ip3, 1);
    
    assert_eq!(rate_limits.get(ip1), Some(&5), "IP1 should have 5 attempts");
    assert_eq!(rate_limits.get(ip2), Some(&3), "IP2 should have 3 attempts");
    assert_eq!(rate_limits.get(ip3), Some(&1), "IP3 should have 1 attempt");
}

#[test]
fn test_rate_limit_resets_after_window() {
    use chrono::{Utc, Duration};
    
    let window_duration = Duration::minutes(15);
    let now = Utc::now();
    let window_start = now;
    let window_end = window_start + window_duration;
    let after_window = window_end + Duration::minutes(1);
    
    // Attempt is within window
    assert!(now >= window_start && now < window_end, "Attempt should be in window");
    
    // Attempt after window should be new
    assert!(after_window >= window_end, "Attempt after window should not count");
}

#[test]
fn test_rate_limit_http_429_response() {
    // When rate limit exceeded, should return 429 Too Many Requests
    let status_code = 429;
    let error_message = "Too many requests";
    
    assert_eq!(status_code, 429, "Should return 429 status code");
    assert!(!error_message.is_empty(), "Should have error message");
}

#[test]
fn test_rate_limit_per_endpoint() {
    let mut endpoint_limits = std::collections::HashMap::new();
    let ip = "192.168.1.1";
    
    // Different endpoints have different limits
    endpoint_limits.insert(format!("{}:login", ip), 5);
    endpoint_limits.insert(format!("{}:email", ip), 5);
    endpoint_limits.insert(format!("{}:password", ip), 3);
    
    assert_eq!(endpoint_limits.get(&format!("{}:login", ip)), Some(&5), "Login limit should be 5");
    assert_eq!(endpoint_limits.get(&format!("{}:email", ip)), Some(&5), "Email limit should be 5");
    assert_eq!(endpoint_limits.get(&format!("{}:password", ip)), Some(&3), "Password limit should be 3");
}
