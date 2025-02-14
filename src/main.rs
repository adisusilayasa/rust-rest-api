use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use crate::utils::logger::setup_logger;
use crate::utils::config_validator::validate_required_env_vars;

mod application;
mod config;
mod db;
mod domain;
mod infrastructure;
mod presentation;
mod utils;

use presentation::api::auth_controller::{login, register};
use presentation::api::user_controller::get_profile;  // Add this import
use config::Config;
use db::establish_connection;
use infrastructure::persistence::postgres_user_repository::PostgresUserRepository;
use application::services::auth_application_service::AuthApplicationService;
use application::services::user_application_service::UserApplicationService;
use actix_web::middleware::Logger;
use infrastructure::auth::middleware::AuthMiddleware;  // Fix this import
use utils::middleware::LoggingMiddleware;

// Remove the duplicate import:
// use utils::{
//     logger::setup_logger,
//     config_validator::validate_required_env_vars,
//     middleware::{request_logging, LoggingMiddleware},
// };

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    setup_logger();

    if let Err(e) = validate_required_env_vars() {
        log::error!("Configuration error: {}", e);
        std::process::exit(1);
    }

    let config = Config::from_env();
    
    let pool = match establish_connection().await {
        Ok(pool) => pool,
        Err(e) => {
            log::error!("Failed to connect to database: {}", e);
            std::process::exit(1);
        }
    };

    let user_repository = PostgresUserRepository::new(pool.clone());
    let auth_service = web::Data::new(AuthApplicationService::new(user_repository.clone()));
    let user_service = web::Data::new(UserApplicationService::new(user_repository));

    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(LoggingMiddleware::new())
            .app_data(auth_service.clone())
            .app_data(user_service.clone())
            .service(
                web::scope("/api")
                    .service(register)
                    .service(login)
                    .service(
                        web::scope("/users")
                            .wrap(AuthMiddleware::new())
                            .service(get_profile)
                    )
            )
    })
    .bind(format!("127.0.0.1:{}", config.port))?
    .run();

    log::info!("Server running at http://127.0.0.1:{}", config.port);

    server.await
}