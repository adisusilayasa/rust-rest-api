use actix_web::{post, web, HttpResponse};
use serde::Deserialize;
use serde_json::json;
use sqlx::PgPool;
use crate::domains::auth::service::{register_user, login_user};
use crate::utils::error::AppError;
use crate::utils::response::{Response, ResponseBuilder};
use log::{error, warn};
use validator::{Validate, ValidationErrors};
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Deserialize, Validate)]
pub struct AuthRequest {
    #[validate(email(message = "Invalid email format"))]
    #[validate(length(min = 1, message = "Email is required"))]
    pub email: String,

    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    #[validate(regex(path = "PASSWORD_REGEX", message = "Password must contain at least one number and one letter"))]
    pub password: String,
}

#[derive(Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(email(message = "Invalid email format"))]
    #[validate(length(min = 1, message = "Email is required"))]
    pub email: String,

    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    #[validate(regex(path = "PASSWORD_REGEX", message = "Password must contain at least one number and one letter"))]
    pub password: String,

    #[validate(length(min = 1, message = "Name is required"))]
    pub name: Option<String>,

    pub phone: Option<String>,
    pub address: Option<String>,
}

lazy_static! {
    static ref PASSWORD_REGEX: Regex = Regex::new(r"^.*[A-Za-z].*\d.*$|^.*\d.*[A-Za-z].*$").unwrap();
}

fn handle_validation_errors(errors: ValidationErrors) -> HttpResponse {
    warn!("Validation failed: {:?}", errors);
    let validation_errors = errors
        .field_errors()
        .into_iter()
        .map(|(field, errors)| {
            let messages: Vec<String> = errors
                .iter()
                .map(|error| error.message.as_ref().unwrap_or(&error.code).to_string())
                .collect();
            (field.to_string(), json!(messages))
        })
        .collect::<serde_json::Map<String, serde_json::Value>>();

    Response::bad_request_with_data("Validation failed", validation_errors)
}

#[post("/register")]
pub async fn handle_register(
    pool: web::Data<PgPool>,
    req: web::Json<RegisterRequest>,
) -> Result<HttpResponse, AppError> {
    if let Err(errors) = req.validate() {
        return Ok(handle_validation_errors(errors));
    }
    
    match register_user(
        pool.get_ref(), 
        req.email.clone(), 
        req.name.as_deref(), 
        req.address.as_deref(), 
        req.phone.as_deref(), 
        &req.password
    ).await {
        Ok(user) => Ok(Response::created(user)),
        Err(AppError::ValidationError(e)) => {
            warn!("Registration validation error: {}", e);
            Ok(Response::bad_request(&e))
        },
        Err(AppError::DatabaseError(e)) => {
            error!("Database error during registration: {}", e);
            Ok(Response::internal_error("Failed to create user account"))
        },
        Err(e) => {
            error!("Unexpected error during registration: {}", e);
            Ok(Response::internal_error("An unexpected error occurred"))
        }
    }
}

#[post("/login")]
pub async fn handle_login(
    pool: web::Data<PgPool>,
    req: web::Json<AuthRequest>,
) -> Result<HttpResponse, AppError> {
    if let Err(errors) = req.validate() {
        return Ok(handle_validation_errors(errors));
    }

    match login_user(pool.get_ref(), &req.email, &req.password).await {
        Ok(token) => Ok(Response::ok(json!({ "token": token }))),
        Err(AppError::AuthenticationError(e)) => {
            warn!("Authentication failed: {}", e);
            Ok(Response::unauthorized(&e))
        },
        Err(AppError::ValidationError(e)) => {
            warn!("Login validation error: {}", e);
            Ok(Response::bad_request(&e))
        },
        Err(AppError::DatabaseError(e)) => {
            error!("Database error during login: {}", e);
            Ok(Response::internal_error("Failed to process login request"))
        },
        Err(e) => {
            error!("Unexpected error during login: {}", e);
            Ok(Response::internal_error("An unexpected error occurred"))
        }
    }
}