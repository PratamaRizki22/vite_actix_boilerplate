use redis::aio::Connection;
use redis::AsyncCommands;
use actix_web::HttpRequest;
use std::sync::Arc;

#[derive(Clone)]
pub struct RedisRateLimiter {
    connection: Arc<tokio::sync::Mutex<Connection>>,
}

impl RedisRateLimiter {
    /// Create new Redis rate limiter
    pub async fn new(redis_url: &str) -> Result<Self, redis::RedisError> {
        let client = redis::Client::open(redis_url)?;
        let connection = client.get_async_connection().await?;
        
        Ok(RedisRateLimiter {
            connection: Arc::new(tokio::sync::Mutex::new(connection)),
        })
    }

    /// Get client IP from request
    fn get_client_ip(req: &HttpRequest) -> String {
        req.connection_info()
            .peer_addr()
            .map(|s| s.to_string())
            .unwrap_or_else(|| "unknown".to_string())
    }

    /// Check rate limit using Redis
    /// Returns: (is_allowed, remaining_attempts, reset_seconds)
    pub async fn check_limit(
        &self,
        req: &HttpRequest,
        endpoint: &str,
        max_attempts: u32,
        window_seconds: u32,
    ) -> (bool, u32, u32) {
        let client_ip = Self::get_client_ip(req);
        let key = format!("rate_limit:{}:{}", endpoint, client_ip);

        let mut conn = self.connection.lock().await;

        // Try to increment the counter
        let current_count: u32 = redis::cmd("INCR")
            .arg(&key)
            .query_async(&mut *conn)
            .await
            .unwrap_or(0);

        // Set expiration if this is first request (count = 1)
        if current_count == 1 {
            let _: () = redis::cmd("EXPIRE")
                .arg(&key)
                .arg(window_seconds as i64)
                .query_async(&mut *conn)
                .await
                .unwrap_or(());
        }

        let remaining = if current_count > max_attempts {
            0
        } else {
            max_attempts - current_count
        };

        let ttl: i64 = redis::cmd("TTL")
            .arg(&key)
            .query_async(&mut *conn)
            .await
            .unwrap_or(-1);

        (current_count <= max_attempts, remaining, ttl.max(0) as u32)
    }

    /// Reset rate limit for a client (for testing)
    pub async fn reset_limit(&self, endpoint: &str, client_ip: &str) -> Result<(), redis::RedisError> {
        let key = format!("rate_limit:{}:{}", endpoint, client_ip);
        let mut conn = self.connection.lock().await;
        let _: () = redis::cmd("DEL")
            .arg(&key)
            .query_async(&mut *conn)
            .await?;
        Ok(())
    }

    /// Get current count for a client
    pub async fn get_count(&self, endpoint: &str, client_ip: &str) -> Result<u32, redis::RedisError> {
        let key = format!("rate_limit:{}:{}", endpoint, client_ip);
        let mut conn = self.connection.lock().await;
        let count: u32 = conn.get(&key).await.unwrap_or(0);
        Ok(count)
    }
}
