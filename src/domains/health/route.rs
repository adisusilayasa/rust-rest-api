use actix_web::web;
use super::controller;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/health")
            .service(controller::health_check)
    );
}