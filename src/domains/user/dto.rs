use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use crate::domains::user::entity::User;

#[derive(Serialize, Deserialize)]
pub struct UserProfileResponse {
    pub id: Uuid,
    pub email: String,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
}

pub fn create_user_profile_response(user: User) -> UserProfileResponse {
    UserProfileResponse {
        id: user.id,
        email: user.email,
        phone: user.phone,
        address: user.address,
        created_at: user.created_at,
        updated_at: user.updated_at,
        deleted_at: user.deleted_at,
    }
}
