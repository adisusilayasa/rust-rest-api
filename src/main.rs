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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    setup_logger();

    let database_url = config::get_database_url();
    let jwt_secret = config::get_jwt_secret();
    let port = config::get_port();
    
    let pool = db::establish_connection()
        .await
        .expect("Failed to connect to database");

    let pool = web::Data::new(pool);
    let server_addr = format!("127.0.0.1:{}", port);

    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(LoggingMiddleware::new())
            .app_data(pool.clone())
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