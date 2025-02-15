use actix_web::{get, web, HttpResponse};
use sqlx::PgPool;

#[get("")]
pub async fn health_check(pool: web::Data<PgPool>) -> HttpResponse {
    match sqlx::query("SELECT 1").execute(pool.get_ref()).await {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({
            "status": "healthy",
            "components": {
                "api": "up",
                "database": "up"
            },
            "timestamp": chrono::Utc::now()
        })),
        Err(e) => HttpResponse::ServiceUnavailable().json(serde_json::json!({
            "status": "unhealthy",
            "components": {
                "api": "up",
                "database": "down"
            },
            "error": e.to_string(),
            "timestamp": chrono::Utc::now()
        }))
    }
}