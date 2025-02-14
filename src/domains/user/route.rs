use actix_web::web;
use super::controller;
use crate::shared::middleware::auth::AuthMiddleware;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/user")
            .wrap(AuthMiddleware::new())
            .service(controller::get_profile)
    );
}