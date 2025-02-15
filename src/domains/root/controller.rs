use actix_web::{get, HttpResponse};
use serde_json::json;

#[get("/")]
pub async fn welcome() -> HttpResponse {
    HttpResponse::Ok().json(json!({
        "name": "Rust REST API",
        "version": env!("CARGO_PKG_VERSION"),
        "description": "A RESTful API built with Rust and Actix-web",
        "endpoints": {
            "health": "/health",
            "auth": {
                "login": "/api/auth/login",
                "register": "/api/auth/register"
            },
            "users": {
                "profile": "/api/users/me",
                "update": "/api/users/me"
            }
        },
        "documentation": "https://github.com/adisusilayasa/rust-rest"
    }))
}