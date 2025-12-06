use redis::aio::Connection;
use redis::AsyncCommands;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedisSessionData {
    pub user_id: i32,
    pub device_name: Option<String>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub created_at: i64,
    pub last_activity: i64,
}

#[derive(Clone)]
pub struct RedisSessionStore {
    connection: Arc<tokio::sync::Mutex<Connection>>,
}

impl RedisSessionStore {
    /// Create new Redis session store
    pub async fn new(redis_url: &str) -> Result<Self, redis::RedisError> {
        let client = redis::Client::open(redis_url)?;
        let connection = client.get_async_connection().await?;
        
        Ok(RedisSessionStore {
            connection: Arc::new(tokio::sync::Mutex::new(connection)),
        })
    }

    /// Store session in Redis with TTL (in seconds)
    pub async fn store_session(
        &self,
        token_hash: &str,
        session_data: &RedisSessionData,
        ttl_seconds: u64,
    ) -> Result<(), redis::RedisError> {
        let key = format!("session:{}", token_hash);
        let mut conn = self.connection.lock().await;
        
        // Serialize session data to JSON
        let json_data = serde_json::to_string(session_data)
            .map_err(|e| redis::RedisError::from((redis::ErrorKind::TypeError, "Serialization error", e.to_string())))?;
        
        // Store with TTL
        let _: () = redis::cmd("SET")
            .arg(&key)
            .arg(&json_data)
            .arg("EX")
            .arg(ttl_seconds)
            .query_async(&mut *conn)
            .await?;
        
        Ok(())
    }

    /// Get session from Redis
    pub async fn get_session(&self, token_hash: &str) -> Result<Option<RedisSessionData>, redis::RedisError> {
        let key = format!("session:{}", token_hash);
        let mut conn = self.connection.lock().await;
        
        let json_data: Option<String> = conn.get(&key).await?;
        
        match json_data {
            Some(data) => {
                let session: RedisSessionData = serde_json::from_str(&data)
                    .map_err(|e| redis::RedisError::from((redis::ErrorKind::TypeError, "Deserialization error", e.to_string())))?;
                Ok(Some(session))
            }
            None => Ok(None),
        }
    }

    /// Update last activity timestamp
    pub async fn update_activity(&self, token_hash: &str) -> Result<bool, redis::RedisError> {
        let key = format!("session:{}", token_hash);
        let mut conn = self.connection.lock().await;
        
        // Get current session
        let json_data: Option<String> = conn.get(&key).await?;
        
        if let Some(data) = json_data {
            let mut session: RedisSessionData = serde_json::from_str(&data)
                .map_err(|e| redis::RedisError::from((redis::ErrorKind::TypeError, "Deserialization error", e.to_string())))?;
            
            // Update last activity
            session.last_activity = chrono::Utc::now().timestamp();
            
            let updated_json = serde_json::to_string(&session)
                .map_err(|e| redis::RedisError::from((redis::ErrorKind::TypeError, "Serialization error", e.to_string())))?;
            
            // Get current TTL to preserve it
            let ttl: i64 = conn.ttl(&key).await.unwrap_or(-1);
            
            if ttl > 0 {
                let _: () = redis::cmd("SET")
                    .arg(&key)
                    .arg(&updated_json)
                    .arg("EX")
                    .arg(ttl)
                    .query_async(&mut *conn)
                    .await?;
                Ok(true)
            } else {
                Ok(false)
            }
        } else {
            Ok(false)
        }
    }

    /// Delete session from Redis
    pub async fn delete_session(&self, token_hash: &str) -> Result<bool, redis::RedisError> {
        let key = format!("session:{}", token_hash);
        let mut conn = self.connection.lock().await;
        
        let deleted: u32 = conn.del(&key).await?;
        Ok(deleted > 0)
    }

    /// Delete all sessions for a user
    pub async fn delete_user_sessions(&self, user_id: i32) -> Result<u32, redis::RedisError> {
        let mut conn = self.connection.lock().await;
        
        // Find all session keys
        let pattern = "session:*";
        let keys: Vec<String> = redis::cmd("KEYS")
            .arg(pattern)
            .query_async(&mut *conn)
            .await?;
        
        let mut deleted_count = 0u32;
        
        for key in keys {
            if let Ok(Some(json_data)) = conn.get::<_, Option<String>>(&key).await {
                if let Ok(session) = serde_json::from_str::<RedisSessionData>(&json_data) {
                    if session.user_id == user_id {
                        let _: u32 = conn.del(&key).await.unwrap_or(0);
                        deleted_count += 1;
                    }
                }
            }
        }
        
        Ok(deleted_count)
    }

    /// Get all sessions for a user
    pub async fn get_user_sessions(&self, user_id: i32) -> Result<Vec<(String, RedisSessionData)>, redis::RedisError> {
        let mut conn = self.connection.lock().await;
        
        let pattern = "session:*";
        let keys: Vec<String> = redis::cmd("KEYS")
            .arg(pattern)
            .query_async(&mut *conn)
            .await?;
        
        let mut sessions = Vec::new();
        
        for key in keys {
            if let Ok(Some(json_data)) = conn.get::<_, Option<String>>(&key).await {
                if let Ok(session) = serde_json::from_str::<RedisSessionData>(&json_data) {
                    if session.user_id == user_id {
                        // Extract token hash from key (remove "session:" prefix)
                        let token_hash = key.trim_start_matches("session:").to_string();
                        sessions.push((token_hash, session));
                    }
                }
            }
        }
        
        Ok(sessions)
    }

    /// Extend session TTL
    pub async fn extend_session(&self, token_hash: &str, additional_seconds: u64) -> Result<bool, redis::RedisError> {
        let key = format!("session:{}", token_hash);
        let mut conn = self.connection.lock().await;
        
        let exists: bool = conn.exists(&key).await?;
        if exists {
            let current_ttl: i64 = conn.ttl(&key).await.unwrap_or(-1);
            if current_ttl > 0 {
                let new_ttl = current_ttl as u64 + additional_seconds;
                let _: bool = conn.expire(&key, new_ttl as i64).await?;
                Ok(true)
            } else {
                Ok(false)
            }
        } else {
            Ok(false)
        }
    }

    /// Check if session exists
    pub async fn exists(&self, token_hash: &str) -> Result<bool, redis::RedisError> {
        let key = format!("session:{}", token_hash);
        let mut conn = self.connection.lock().await;
        conn.exists(&key).await
    }

    /// Get session TTL in seconds
    pub async fn get_ttl(&self, token_hash: &str) -> Result<i64, redis::RedisError> {
        let key = format!("session:{}", token_hash);
        let mut conn = self.connection.lock().await;
        conn.ttl(&key).await
    }
}
