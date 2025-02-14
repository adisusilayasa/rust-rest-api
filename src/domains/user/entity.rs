use bcrypt::verify;
use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl User {
    pub fn new(email: String, password_hash: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            email,
            password_hash,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    pub fn verify_password(&self, password: &str) -> Result<bool, String> {
        verify(password, &self.password_hash)
            .map_err(|e| format!("Password verification error: {}", e))
    }
}