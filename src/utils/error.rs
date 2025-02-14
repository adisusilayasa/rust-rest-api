use actix_web::{error::ResponseError, HttpResponse};
use derive_more::Display;
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error, Display)]
pub enum AppError {
    #[display(fmt = "Database error: {}", _0)]
    DatabaseError(sqlx::Error),
    #[display(fmt = "Authentication error: {}", _0)]
    AuthError(String),
    #[display(fmt = "Validation error: {}", _0)]
    ValidationError(String),
    #[display(fmt = "Not found: {}", _0)]
    NotFoundError(String),
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
    message: String,
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        let error_response = ErrorResponse {
            error: self.get_error_type(),
            message: self.to_string(),
        };

        match self {
            AppError::DatabaseError(_) => HttpResponse::InternalServerError().json(error_response),
            AppError::AuthError(_) => HttpResponse::Unauthorized().json(error_response),
            AppError::ValidationError(_) => HttpResponse::BadRequest().json(error_response),
            AppError::NotFoundError(_) => HttpResponse::NotFound().json(error_response),
        }
    }
}

impl AppError {
    fn get_error_type(&self) -> String {
        match self {
            AppError::DatabaseError(_) => "Database Error",
            AppError::AuthError(_) => "Authentication Error",
            AppError::ValidationError(_) => "Validation Error",
            AppError::NotFoundError(_) => "Not Found Error",
        }
        .to_string()
    }
}

impl From<sqlx::Error> for AppError {
    fn from(error: sqlx::Error) -> Self {
        AppError::DatabaseError(error)
    }
}