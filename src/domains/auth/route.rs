use actix_web::web;
use super::controller;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .service(controller::handle_register)
            .service(controller::handle_login)
    );
}