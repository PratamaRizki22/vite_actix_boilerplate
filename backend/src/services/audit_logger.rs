use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct AuditLogger {
    // in-memory log store for scaffolding; swap with persistent storage later
    pub inner: Arc<Mutex<Vec<String>>>,
}

impl AuditLogger {
    pub fn new() -> Self {
        Self { inner: Arc::new(Mutex::new(Vec::new())) }
    }

    pub fn log_event(&self, event: &str) {
        let mut v = self.inner.lock().unwrap();
        v.push(event.to_string());
    }

    pub fn get_events(&self) -> Vec<String> {
        let v = self.inner.lock().unwrap();
        v.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn audit_log_basic() {
        let logger = AuditLogger::new();
        logger.log_event("login:user:1");
        logger.log_event("logout:user:1");
        let events = logger.get_events();
        assert_eq!(events.len(), 2);
        assert!(events[0].contains("login:user:1"));
    }
}
