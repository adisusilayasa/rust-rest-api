use actix_web::{post, web, HttpResponse};
use serde::Deserialize;
use crate::domains::auth::service::AuthService;
use crate::domains::user::repository::PostgresUserRepository;
use crate::shared::error::AppError;
use serde_json::json;

#[derive(Deserialize)]
pub struct AuthRequest {
    email: String,
    password: String,
}

#[post("/register")]
pub async fn register(
    auth_service: web::Data<AuthService<PostgresUserRepository>>,
    req: web::Json<AuthRequest>,
) -> Result<HttpResponse, AppError> {
    let user = auth_service.register(req.email.clone(), req.password.clone()).await?;
    Ok(HttpResponse::Created().json(user))
}

#[post("/login")]
pub async fn login(
    auth_service: web::Data<AuthService<PostgresUserRepository>>,
    req: web::Json<AuthRequest>,
) -> Result<HttpResponse, AppError> {
    let token = auth_service.login(req.email.clone(), req.password.clone()).await?;
    Ok(HttpResponse::Ok().json(json!({ "token": token })))
}