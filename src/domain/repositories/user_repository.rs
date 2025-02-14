use async_trait::async_trait;
use uuid::Uuid;
use crate::domain::entities::user::User;

#[async_trait]
pub trait UserRepository {
    async fn create(&self, user: &User) -> Result<User, String>;
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, String>;
    async fn find_by_id(&self, id: &Uuid) -> Result<Option<User>, String>;
}