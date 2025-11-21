use actix_web::HttpRequest;
use chrono::{DateTime, Utc, Duration};
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;

#[derive(Clone, Debug)]
struct RateLimitEntry {
    count: u32,
    reset_time: DateTime<Utc>,
}

lazy_static! {
    static ref RATE_LIMIT_STORE: Mutex<HashMap<String, RateLimitEntry>> = Mutex::new(HashMap::new());
}

/// RateLimiter handles rate limiting for different endpoints
pub struct RateLimiter;

impl RateLimiter {
    /// Get client IP from request
    fn get_client_ip(req: &HttpRequest) -> String {
        req.connection_info()
            .peer_addr()
            .map(|s| s.to_string())
            .unwrap_or_else(|| "unknown".to_string())
    }

    /// Check if client has exceeded rate limit
    /// Returns: (is_allowed, remaining_attempts, reset_seconds)
    pub fn check_limit(
        req: &HttpRequest,
        endpoint: &str,
        max_attempts: u32,
        window_minutes: u32,
    ) -> (bool, u32, u32) {
        let client_ip = Self::get_client_ip(req);
        let key = format!("{}:{}", endpoint, client_ip);

        let mut store = RATE_LIMIT_STORE.lock().unwrap();
        let now = Utc::now();

        // Get or create entry
        let entry = store.entry(key.clone()).or_insert_with(|| RateLimitEntry {
            count: 0,
            reset_time: now + Duration::minutes(window_minutes as i64),
        });

        // Check if window has expired
        if now > entry.reset_time {
            // Reset the window
            entry.count = 0;
            entry.reset_time = now + Duration::minutes(window_minutes as i64);
        }

        // Increment count
        entry.count += 1;

        // Calculate remaining attempts and reset seconds
        let remaining = if entry.count > max_attempts {
            0
        } else {
            max_attempts - entry.count
        };

        let reset_seconds = (entry.reset_time - now).num_seconds() as u32;
        let is_allowed = entry.count <= max_attempts;

        println!(
            "[RateLimit] {} - Count: {}/{}, Remaining: {}, Reset in: {}s",
            key, entry.count, max_attempts, remaining, reset_seconds
        );

        (is_allowed, remaining, reset_seconds)
    }

    /// Reset rate limit for a specific key (useful for successful login, etc)
    pub fn reset(req: &HttpRequest, endpoint: &str) {
        let client_ip = Self::get_client_ip(req);
        let key = format!("{}:{}", endpoint, client_ip);

        let mut store = RATE_LIMIT_STORE.lock().unwrap();
        store.remove(&key);

        println!("[RateLimit] Reset limit for: {}", key);
    }

    /// Check if client has exceeded rate limit with custom key
    /// Returns: (is_allowed, remaining_attempts, reset_seconds)
    pub fn check_limit_with_key(
        key: &str,
        max_attempts: u32,
        window_minutes: u32,
    ) -> (bool, u32, u32) {
        let mut store = RATE_LIMIT_STORE.lock().unwrap();
        let now = Utc::now();

        // Get or create entry
        let entry = store.entry(key.to_string()).or_insert_with(|| RateLimitEntry {
            count: 0,
            reset_time: now + Duration::minutes(window_minutes as i64),
        });

        // Check if window has expired
        if now > entry.reset_time {
            // Reset the window
            entry.count = 0;
            entry.reset_time = now + Duration::minutes(window_minutes as i64);
        }

        // Increment count
        entry.count += 1;

        // Calculate remaining attempts and reset seconds
        let remaining = if entry.count > max_attempts {
            0
        } else {
            max_attempts - entry.count
        };

        let reset_seconds = (entry.reset_time - now).num_seconds() as u32;
        let is_allowed = entry.count <= max_attempts;

        println!(
            "[RateLimit] {} - Count: {}/{}, Remaining: {}, Reset in: {}s",
            key, entry.count, max_attempts, remaining, reset_seconds
        );

        (is_allowed, remaining, reset_seconds)
    }

    /// Get current rate limit stats for debugging
    pub fn get_stats() -> HashMap<String, (u32, String)> {
        let store = RATE_LIMIT_STORE.lock().unwrap();
        let mut stats = HashMap::new();

        for (key, entry) in store.iter() {
            let reset_in = (entry.reset_time - Utc::now()).num_seconds();
            stats.insert(
                key.clone(),
                (entry.count, format!("resets in {}s", reset_in)),
            );
        }

        stats
    }

    /// Clear all rate limit entries (for testing)
    pub fn clear_all() {
        let mut store = RATE_LIMIT_STORE.lock().unwrap();
        store.clear();
        println!("[RateLimit] Cleared all rate limit entries");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rate_limiting() {
        RateLimiter::clear_all();

        // Simulate 5 requests
        for i in 1..=5 {
            println!("Request {}", i);
        }
    }
}
