use actix_web::web;
use super::controller;
use crate::utils::middleware::auth::AuthMiddleware;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .wrap(AuthMiddleware::new())
            .service(controller::handle_get_profile)
            .service(controller::handle_update_profile)
    );
}