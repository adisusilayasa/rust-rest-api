use crate::domain::entities::user::User;
use crate::domain::repositories::user_repository::UserRepository;

pub struct AuthService<T: UserRepository> {
    pub user_repository: T,
}

impl<T: UserRepository> AuthService<T> {
    pub fn new(user_repository: T) -> Self {
        Self { user_repository }
    }
}

impl<T: UserRepository> AuthService<T> {
    pub async fn authenticate_user(&self, email: &str, password: &str) -> Result<User, String> {
        let user = self.user_repository
            .find_by_email(email)
            .await?
            .ok_or("Invalid credentials")?;

        if !user.verify_password(password)? {
            return Err("Invalid credentials".to_string());
        }

        Ok(user)
    }
}