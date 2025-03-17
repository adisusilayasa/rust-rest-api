use std::sync::Arc;
use tokio::sync::Mutex;
use std::collections::HashMap;
use std::time::{Duration, Instant};
use log::warn;
use crate::utils::error::AppError;

#[derive(Clone)]
pub struct RateLimiter {
    attempts: Arc<Mutex<HashMap<String, Vec<Instant>>>>,
    max_attempts: usize,
    window_secs: u64,
}

impl RateLimiter {
    pub fn new(max_attempts: usize, window_secs: u64) -> Self {
        Self {
            attempts: Arc::new(Mutex::new(HashMap::new())),
            max_attempts,
            window_secs,
        }
    }

    pub async fn check_rate_limit(&self, key: &str) -> Result<(), AppError> {
        let now = Instant::now();
        let window = Duration::from_secs(self.window_secs);
        
        let mut attempts = self.attempts.lock().await;
        let attempt_times = attempts.entry(key.to_string()).or_insert_with(Vec::new);
        
        // Remove old attempts
        attempt_times.retain(|&time| now.duration_since(time) < window);
        
        if attempt_times.len() >= self.max_attempts {
            warn!("Rate limit exceeded for {}", key);
            return Err(AppError::rate_limited(format!(
                "Too many login attempts. Please try again after {} seconds",
                self.window_secs
            )));
        }
        
        attempt_times.push(now);
        Ok(())
    }

    pub async fn reset(&self, key: &str) {
        let mut attempts = self.attempts.lock().await;
        attempts.remove(key);
    }
}

// Create a static rate limiter for login
lazy_static::lazy_static! {
    pub static ref LOGIN_LIMITER: RateLimiter = RateLimiter::new(5, 300); // 5 attempts per 5 minutes
}