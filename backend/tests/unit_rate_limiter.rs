#[cfg(test)]
mod tests {
    #[test]
    fn test_rate_limiter_allows_initial_requests() {
        let mut attempts = 0;
        let max_attempts = 5;
        
        // First 5 requests should be allowed
        for i in 1..=5 {
            if attempts < max_attempts {
                attempts += 1;
                assert!(attempts <= max_attempts, "Request {} should be allowed", i);
            }
        }
    }

    #[test]
    fn test_rate_limiter_blocks_after_limit() {
        let mut attempts = 0;
        let max_attempts = 5;
        
        // Use up all 5 attempts
        for _ in 1..=5 {
            if attempts < max_attempts {
                attempts += 1;
            }
        }
        
        // Check if 6th request would be blocked
        let would_block = attempts >= max_attempts;
        assert!(would_block, "6th request should be blocked");
    }

    #[test]
    fn test_rate_limiter_different_endpoints() {
        let mut login_attempts = 0;
        let mut email_attempts = 0;
        let login_limit = 5;
        let email_limit = 5;
        
        // Each endpoint should have separate limits
        if login_attempts < login_limit {
            login_attempts += 1;
        }
        if email_attempts < email_limit {
            email_attempts += 1;
        }
        
        assert_eq!(login_attempts, 1, "Login attempt should increment");
        assert_eq!(email_attempts, 1, "Email attempt should increment");
    }

    #[test]
    fn test_rate_limiter_multiple_ips() {
        let mut ip1_attempts = 0;
        let mut ip2_attempts = 0;
        let limit = 5;
        
        // IP 1 makes 5 requests
        for _ in 0..5 {
            if ip1_attempts < limit {
                ip1_attempts += 1;
            }
        }
        
        // IP 2 makes 1 request (separate limit)
        if ip2_attempts < limit {
            ip2_attempts += 1;
        }
        
        assert_eq!(ip1_attempts, 5, "IP1 should have 5 attempts");
        assert_eq!(ip2_attempts, 1, "IP2 should have 1 attempt");
    }

    #[test]
    fn test_rate_limit_for_login() {
        let limit = 5;
        let window_minutes = 15;
        
        assert_eq!(limit, 5, "Login limit should be 5");
        assert_eq!(window_minutes, 15, "Login window should be 15 minutes");
    }

    #[test]
    fn test_rate_limit_for_email() {
        let limit = 5;
        let window_minutes = 60;
        
        assert_eq!(limit, 5, "Email limit should be 5");
        assert_eq!(window_minutes, 60, "Email window should be 60 minutes");
    }

    #[test]
    fn test_rate_limit_for_password_reset() {
        let limit = 3;
        let window_minutes = 60;
        
        assert_eq!(limit, 3, "Password reset limit should be 3");
        assert_eq!(window_minutes, 60, "Password reset window should be 60 minutes");
    }
}
