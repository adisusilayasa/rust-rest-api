use sqlx::PgPool;
use uuid::Uuid;
use crate::domains::user::entity::User;

pub struct PostgresUserRepository {
    pool: PgPool,
}

pub fn new_repository(pool: PgPool) -> PostgresUserRepository {
    PostgresUserRepository { pool }
}

pub async fn create_user(pool: &PgPool, user: &User) -> Result<User, String> {
    sqlx::query_as!(
        User,
        r#"
        INSERT INTO users (id, email, name, phone, address, password_hash, created_at, updated_at, deleted_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        RETURNING id, email, name, phone, address, password_hash, created_at, updated_at, deleted_at
        "#,
        user.id,
        user.email,
        user.name,
        user.phone,
        user.address,
        user.password_hash,
        user.created_at,
        user.updated_at,
        user.deleted_at
    )
    .fetch_one(pool)
    .await
    .map_err(|e| format!("Database error: {}", e))
}

pub async fn find_user_by_email(pool: &PgPool, email: &str) -> Result<Option<User>, String> {
    sqlx::query_as!(
        User,
        r#"
        SELECT id, email, name, phone, address, password_hash, created_at, updated_at, deleted_at
        FROM users
        WHERE email = $1 AND deleted_at IS NULL
        "#,
        email
    )
    .fetch_optional(pool)
    .await
    .map_err(|e| format!("Database error: {}", e))
}

pub async fn find_user_by_id(pool: &PgPool, id: &Uuid) -> Result<Option<User>, String> {
    sqlx::query_as!(
        User,
        r#"
        SELECT id, email, name, phone, address, password_hash, created_at, updated_at, deleted_at
        FROM users
        WHERE id = $1 AND deleted_at IS NULL
        "#,
        id
    )
    .fetch_optional(pool)
    .await
    .map_err(|e| format!("Database error: {}", e))
}

pub async fn update_user(pool: &PgPool, user: &User) -> Result<User, String> {
    sqlx::query_as!(
        User,
        r#"
        UPDATE users
        SET 
            email = $1,
            name = $2,
            phone = $3,
            address = $4,
            password_hash = $5,
            updated_at = $6,
            deleted_at = $7
        WHERE id = $8 AND deleted_at IS NULL
        RETURNING id, email, name, phone, address, password_hash, created_at, updated_at, deleted_at
        "#,
        user.email,
        user.name,
        user.phone,
        user.address,
        user.password_hash,
        user.updated_at,
        user.deleted_at,
        user.id
    )
    .fetch_one(pool)
    .await
    .map_err(|e| format!("Database error: {}", e))
}