use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use actix_web::middleware::Logger;

mod domains;
mod shared;

use crate::shared::config::Config;
use crate::shared::middleware::logger::setup_logger;
use crate::shared::middleware::logger::LoggingMiddleware;
use crate::domains::user::repository::PostgresUserRepository;
use crate::domains::auth::service::AuthService;
use crate::domains::user::service::UserService;
use crate::domains::auth::route as auth_routes;
use crate::domains::user::route as user_routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    setup_logger();

    let config = Config::from_env()
        .expect("Failed to load configuration");
    
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await
        .expect("Failed to connect to database");

    let user_repository = PostgresUserRepository::new(pool.clone());
    let auth_service = web::Data::new(AuthService::new(user_repository.clone(), config.jwt_secret.clone()));
    let user_service = web::Data::new(UserService::new(user_repository));

    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(LoggingMiddleware::new())
            .app_data(auth_service.clone())
            .app_data(user_service.clone())
            .service(
                web::scope("/api")
                    .configure(auth_routes::configure)
                    .configure(user_routes::configure)
            )
    })
    .bind(&config.server_addr)?
    .run();

    log::info!("Server running at http://127.0.0.1:{}", config.server_addr);

    server.await
}