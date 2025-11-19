use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use chrono::{DateTime, Utc, Duration};

#[derive(Clone)]
pub struct AccountLockout {
    // simple in-memory store for scaffold; replace with DB in production
    pub inner: Arc<Mutex<HashMap<String, (u32, DateTime<Utc>)>>>,
    pub max_attempts: u32,
    pub lock_duration: Duration,
}

impl AccountLockout {
    pub fn new(max_attempts: u32, lock_minutes: i64) -> Self {
        Self {
            inner: Arc::new(Mutex::new(HashMap::new())),
            max_attempts,
            lock_duration: Duration::minutes(lock_minutes),
        }
    }

    pub fn record_failed_attempt(&self, email: &str) {
        let mut map = self.inner.lock().unwrap();
        let now = Utc::now();
        let entry = map.entry(email.to_string()).or_insert((0, now));
        entry.0 += 1;
        entry.1 = now;
    }

    pub fn reset_attempts(&self, email: &str) {
        let mut map = self.inner.lock().unwrap();
        map.remove(email);
    }

    pub fn is_locked(&self, email: &str) -> bool {
        let map = self.inner.lock().unwrap();
        if let Some((count, last_time)) = map.get(email) {
            if *count >= self.max_attempts {
                return Utc::now() < (*last_time + self.lock_duration);
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lockout_flow() {
        let al = AccountLockout::new(3, 1); // 1 minute lock for test
        let email = "user@example.com";

        assert!(!al.is_locked(email));
        al.record_failed_attempt(email);
        al.record_failed_attempt(email);
        assert!(!al.is_locked(email));
        al.record_failed_attempt(email);
        assert!(al.is_locked(email));
        al.reset_attempts(email);
        assert!(!al.is_locked(email));
    }
}
