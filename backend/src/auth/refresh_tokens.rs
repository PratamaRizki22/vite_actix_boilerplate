use chrono::{Utc, Duration};
use uuid::Uuid;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct RefreshTokenManager {
    store: Arc<Mutex<HashMap<String, String>>>, // user_id -> token
    rotation_window: Duration,
}

impl RefreshTokenManager {
    pub fn new(rotation_minutes: i64) -> Self {
        Self { store: Arc::new(Mutex::new(HashMap::new())), rotation_window: Duration::minutes(rotation_minutes) }
    }

    pub fn generate_for(&self, user_id: &str) -> String {
        let token = Uuid::new_v4().to_string();
        let mut s = self.store.lock().unwrap();
        s.insert(user_id.to_string(), token.clone());
        token
    }

    pub fn rotate(&self, user_id: &str) -> String {
        // simple rotation: generate new and replace
        self.generate_for(user_id)
    }

    pub fn validate(&self, user_id: &str, token: &str) -> bool {
        let s = self.store.lock().unwrap();
        if let Some(stored) = s.get(user_id) {
            return stored == token;
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn refresh_token_flow() {
        let mgr = RefreshTokenManager::new(60);
        let user = "1";
        let token = mgr.generate_for(user);
        assert!(mgr.validate(user, &token));
        let new_token = mgr.rotate(user);
        assert!(!new_token.is_empty());
        assert!(!mgr.validate(user, &token));
        assert!(mgr.validate(user, &new_token));
    }
}
