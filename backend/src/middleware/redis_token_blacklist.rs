use redis::aio::Connection;
use redis::AsyncCommands;
use std::sync::Arc;

#[derive(Clone)]
pub struct RedisTokenBlacklist {
    connection: Arc<tokio::sync::Mutex<Connection>>,
}

impl RedisTokenBlacklist {
    /// Create new Redis token blacklist
    pub async fn new(redis_url: &str) -> Result<Self, redis::RedisError> {
        let client = redis::Client::open(redis_url)?;
        let connection = client.get_async_connection().await?;
        
        Ok(RedisTokenBlacklist {
            connection: Arc::new(tokio::sync::Mutex::new(connection)),
        })
    }

    /// Add token to blacklist
    pub async fn blacklist_token(
        &self,
        token: &str,
        ttl_seconds: u64,
    ) -> Result<(), redis::RedisError> {
        let key = format!("blacklist:{}", token);
        let mut conn = self.connection.lock().await;
        
        // Set token in blacklist with TTL equal to token expiry
        let _: () = redis::cmd("SET")
            .arg(&key)
            .arg("1")
            .arg("EX")
            .arg(ttl_seconds)
            .query_async(&mut *conn)
            .await?;
        Ok(())
    }

    /// Check if token is blacklisted
    pub async fn is_blacklisted(&self, token: &str) -> Result<bool, redis::RedisError> {
        let key = format!("blacklist:{}", token);
        let mut conn = self.connection.lock().await;
        
        let exists: bool = conn.exists(&key).await?;
        Ok(exists)
    }

    /// Remove token from blacklist (for testing)
    pub async fn remove_from_blacklist(&self, token: &str) -> Result<(), redis::RedisError> {
        let key = format!("blacklist:{}", token);
        let mut conn = self.connection.lock().await;
        let _: () = redis::cmd("DEL")
            .arg(&key)
            .query_async(&mut *conn)
            .await?;
        Ok(())
    }

    /// Get TTL of blacklisted token
    pub async fn get_ttl(&self, token: &str) -> Result<i32, redis::RedisError> {
        let key = format!("blacklist:{}", token);
        let mut conn = self.connection.lock().await;
        conn.ttl(&key).await
    }

    /// Clear all blacklisted tokens (for testing)
    pub async fn clear_all(&self) -> Result<(), redis::RedisError> {
        let mut conn = self.connection.lock().await;
        
        // Find all blacklist keys and delete them
        let keys: Vec<String> = redis::cmd("KEYS")
            .arg("blacklist:*")
            .query_async(&mut *conn)
            .await?;
        
        if !keys.is_empty() {
            let _: () = redis::cmd("DEL")
                .arg(&keys)
                .query_async(&mut *conn)
                .await?;
        }
        Ok(())
    }
}
