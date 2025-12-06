use redis::aio::Connection;
use redis::AsyncCommands;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Clone)]
pub struct RedisCache {
    connection: Arc<tokio::sync::Mutex<Connection>>,
}

impl RedisCache {
    /// Create new Redis cache
    pub async fn new(redis_url: &str) -> Result<Self, redis::RedisError> {
        let client = redis::Client::open(redis_url)?;
        let connection = client.get_async_connection().await?;
        
        Ok(RedisCache {
            connection: Arc::new(tokio::sync::Mutex::new(connection)),
        })
    }

    /// Store data in cache with TTL (in seconds)
    pub async fn set<T: Serialize>(
        &self,
        key: &str,
        value: &T,
        ttl_seconds: u64,
    ) -> Result<(), redis::RedisError> {
        let mut conn = self.connection.lock().await;
        
        let json_data = serde_json::to_string(value)
            .map_err(|e| redis::RedisError::from((redis::ErrorKind::TypeError, "Serialization error", e.to_string())))?;
        
        let _: () = redis::cmd("SET")
            .arg(key)
            .arg(&json_data)
            .arg("EX")
            .arg(ttl_seconds)
            .query_async(&mut *conn)
            .await?;
        
        Ok(())
    }

    /// Get data from cache
    pub async fn get<T: for<'de> Deserialize<'de>>(&self, key: &str) -> Result<Option<T>, redis::RedisError> {
        let mut conn = self.connection.lock().await;
        
        let json_data: Option<String> = conn.get(key).await?;
        
        match json_data {
            Some(data) => {
                let value: T = serde_json::from_str(&data)
                    .map_err(|e| redis::RedisError::from((redis::ErrorKind::TypeError, "Deserialization error", e.to_string())))?;
                Ok(Some(value))
            }
            None => Ok(None),
        }
    }

    /// Delete key from cache
    pub async fn delete(&self, key: &str) -> Result<bool, redis::RedisError> {
        let mut conn = self.connection.lock().await;
        let deleted: u32 = conn.del(key).await?;
        Ok(deleted > 0)
    }

    /// Delete multiple keys matching pattern
    pub async fn delete_pattern(&self, pattern: &str) -> Result<u32, redis::RedisError> {
        let mut conn = self.connection.lock().await;
        
        let keys: Vec<String> = redis::cmd("KEYS")
            .arg(pattern)
            .query_async(&mut *conn)
            .await?;
        
        if keys.is_empty() {
            return Ok(0);
        }
        
        let deleted: u32 = conn.del(&keys).await?;
        Ok(deleted)
    }

    /// Check if key exists
    pub async fn exists(&self, key: &str) -> Result<bool, redis::RedisError> {
        let mut conn = self.connection.lock().await;
        conn.exists(key).await
    }

    /// Update TTL for existing key
    pub async fn expire(&self, key: &str, ttl_seconds: u64) -> Result<bool, redis::RedisError> {
        let mut conn = self.connection.lock().await;
        conn.expire(key, ttl_seconds as i64).await
    }

    /// Get TTL of a key
    pub async fn get_ttl(&self, key: &str) -> Result<i64, redis::RedisError> {
        let mut conn = self.connection.lock().await;
        conn.ttl(key).await
    }

    /// Increment numeric value
    pub async fn increment(&self, key: &str, delta: i64) -> Result<i64, redis::RedisError> {
        let mut conn = self.connection.lock().await;
        conn.incr(key, delta).await
    }

    /// Decrement numeric value
    pub async fn decrement(&self, key: &str, delta: i64) -> Result<i64, redis::RedisError> {
        let mut conn = self.connection.lock().await;
        conn.decr(key, delta).await
    }

    // === Helper methods for common cache patterns ===

    /// Cache user data
    pub async fn cache_user<T: Serialize>(
        &self,
        user_id: i32,
        user_data: &T,
    ) -> Result<(), redis::RedisError> {
        let key = format!("user:{}", user_id);
        self.set(&key, user_data, 3600).await // 1 hour TTL
    }

    /// Get cached user data
    pub async fn get_user<T: for<'de> Deserialize<'de>>(
        &self,
        user_id: i32,
    ) -> Result<Option<T>, redis::RedisError> {
        let key = format!("user:{}", user_id);
        self.get(&key).await
    }

    /// Invalidate user cache
    pub async fn invalidate_user(&self, user_id: i32) -> Result<bool, redis::RedisError> {
        let key = format!("user:{}", user_id);
        self.delete(&key).await
    }

    /// Cache posts feed
    pub async fn cache_posts<T: Serialize>(
        &self,
        cache_key: &str,
        posts: &T,
        ttl_seconds: u64,
    ) -> Result<(), redis::RedisError> {
        let key = format!("posts:{}", cache_key);
        self.set(&key, posts, ttl_seconds).await
    }

    /// Get cached posts
    pub async fn get_posts<T: for<'de> Deserialize<'de>>(
        &self,
        cache_key: &str,
    ) -> Result<Option<T>, redis::RedisError> {
        let key = format!("posts:{}", cache_key);
        self.get(&key).await
    }

    /// Invalidate all posts cache
    pub async fn invalidate_all_posts(&self) -> Result<u32, redis::RedisError> {
        self.delete_pattern("posts:*").await
    }

    /// Invalidate specific post cache
    pub async fn invalidate_post(&self, post_id: i32) -> Result<u32, redis::RedisError> {
        // Invalidate all posts caches that might contain this post
        self.invalidate_all_posts().await
    }

    /// Cache search results
    pub async fn cache_search<T: Serialize>(
        &self,
        search_query: &str,
        results: &T,
    ) -> Result<(), redis::RedisError> {
        let key = format!("search:{}", search_query);
        self.set(&key, results, 600).await // 10 minutes TTL
    }

    /// Get cached search results
    pub async fn get_search<T: for<'de> Deserialize<'de>>(
        &self,
        search_query: &str,
    ) -> Result<Option<T>, redis::RedisError> {
        let key = format!("search:{}", search_query);
        self.get(&key).await
    }
}
