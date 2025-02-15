use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use actix_web::middleware::Logger;

mod domains;
mod utils;
mod db;
mod config;

use crate::utils::middleware::logger::setup_logger;
use crate::utils::middleware::logger::LoggingMiddleware;
use crate::domains::auth::route as auth_routes;
use crate::domains::user::route as user_routes;
use crate::domains::health::route as health_routes;
use crate::domains::root::controller::welcome;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    setup_logger();

    log::info!("Starting application...");
    
    let port = config::get_port();
    log::info!("Using port: {}", port);
    
    let pool = db::establish_connection()
        .await
        .expect("Failed to connect to database");
    log::info!("Database connection established");

    let pool = web::Data::new(pool);
    let server_addr = format!("0.0.0.0:{}", port);  // Changed from 127.0.0.1 to 0.0.0.0

    let server = HttpServer::new(move || {
        use actix_cors::Cors;
        
        App::new()
            .wrap(Cors::default()
                .allow_any_origin()
                .allow_any_method()
                .allow_any_header()
                .max_age(3600))
            .wrap(Logger::default())
            .wrap(LoggingMiddleware::new())
            .app_data(pool.clone())
            .service(welcome)  // Add this line
            .configure(health_routes::configure)
            .service(
                web::scope("/api")
                    .configure(auth_routes::configure)
                    .configure(user_routes::configure)
            )
    })
    .bind(&server_addr)?
    .run();

    log::info!("Server running at http://{}", server_addr);

    server.await
}