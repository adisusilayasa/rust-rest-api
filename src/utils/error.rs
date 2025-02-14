use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use sqlx;

#[derive(Debug, Display)]
pub enum AppError {
    #[display(fmt = "Internal error: {}", _0)]
    InternalError(String),
    #[display(fmt = "Validation error: {}", _0)]
    ValidationError(String),
    #[display(fmt = "Authentication error: {}", _0)]
    AuthenticationError(String),
    #[display(fmt = "Not found: {}", _0)]
    NotFoundError(String),
    #[display(fmt = "Database error: {}", _0)]
    DatabaseError(sqlx::Error),
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::InternalError(_) => HttpResponse::InternalServerError().json(self.to_string()),
            AppError::ValidationError(_) => HttpResponse::BadRequest().json(self.to_string()),
            AppError::AuthenticationError(_) => HttpResponse::Unauthorized().json(self.to_string()),
            AppError::NotFoundError(_) => HttpResponse::NotFound().json(self.to_string()),
            AppError::DatabaseError(_) => HttpResponse::InternalServerError().json("Database error"),
        }
    }
}