use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;
use crate::domains::user::entity::User;

#[derive(Clone)]  // Add this derive attribute
pub struct PostgresUserRepository {
    pool: PgPool,
}

#[async_trait]
pub trait UserRepository {
    async fn create(&self, user: &User) -> Result<User, String>;
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, String>;
    async fn find_by_id(&self, id: &Uuid) -> Result<Option<User>, String>;
}

impl PostgresUserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for PostgresUserRepository {
    async fn create(&self, user: &User) -> Result<User, String> {
        sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (id, email, password_hash, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, email, password_hash, created_at, updated_at
            "#,
            user.id,
            user.email,
            user.password_hash,
            user.created_at,
            user.updated_at
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| format!("Database error: {}", e))
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<User>, String> {
        sqlx::query_as!(
            User,
            r#"
            SELECT id, email, password_hash, created_at, updated_at
            FROM users
            WHERE email = $1
            "#,
            email
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| format!("Database error: {}", e))
    }

    async fn find_by_id(&self, id: &Uuid) -> Result<Option<User>, String> {
        sqlx::query_as!(
            User,
            r#"
            SELECT id, email, password_hash, created_at, updated_at
            FROM users
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| format!("Database error: {}", e))
    }
}