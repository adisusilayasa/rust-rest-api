use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::Error;
use actix_web::middleware::Logger;
use log::info;
use actix_web::dev::Transform;
use actix_web::dev::Service;
use futures_util::future::{LocalBoxFuture, Ready, ready};
use std::task::{Context, Poll};


pub fn setup_logger() {
    let env = env_logger::Env::new()
        .filter("RUST_LOG")
        .write_style("RUST_LOG_STYLE");

    env_logger::Builder::from_env(env)
        .format_timestamp_millis()
        .format_module_path(true)
        .format_target(false)
        .format_level(true)
        .filter_module("sqlx", log::LevelFilter::Warn)
        .init();
}


pub fn request_logging() -> Logger {
    Logger::new("%a \"%r\" %s %b \"%{Referer}i\" \"%{User-Agent}i\" %T")
}

#[derive(Default, Clone)]
pub struct LoggingMiddleware;

impl LoggingMiddleware {
    pub fn new() -> Self {
        LoggingMiddleware
    }
}

impl<S, B> Transform<S, ServiceRequest> for LoggingMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = LoggingMiddlewareService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(LoggingMiddlewareService { service }))
    }
}

pub struct LoggingMiddlewareService<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for LoggingMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let start = std::time::Instant::now();
        let method = req.method().clone();
        let path = req.path().to_owned();

        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;
            let duration = start.elapsed();
            
            info!(
                "{} {} - {} - {}ms",
                method,
                path,
                res.status().as_u16(),
                duration.as_millis()
            );

            Ok(res)
        })
    }
}
