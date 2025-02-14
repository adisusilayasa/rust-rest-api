use actix_web::{post, web, HttpResponse};
use serde::Deserialize;
use sqlx::PgPool;
use crate::domains::auth::service::{register_user, login_user};
use crate::utils::error::AppError;
use serde_json::json;

#[derive(Deserialize)]
pub struct AuthRequest {
    email: String,
    password: String,
}

#[post("/register")]
pub async fn handle_register(
    pool: web::Data<PgPool>,
    req: web::Json<AuthRequest>,
) -> Result<HttpResponse, AppError> {
    let user = register_user(
        pool.get_ref(),
        req.email.clone(),
        req.password.clone()
    ).await?;
    
    Ok(HttpResponse::Created().json(user))
}

#[post("/login")]
pub async fn handle_login(
    pool: web::Data<PgPool>,
    req: web::Json<AuthRequest>,
) -> Result<HttpResponse, AppError> {
    let token = login_user(
        pool.get_ref(),
        req.email.clone(),
        req.password.clone()
    ).await?;
    
    Ok(HttpResponse::Ok().json(json!({ "token": token })))
}