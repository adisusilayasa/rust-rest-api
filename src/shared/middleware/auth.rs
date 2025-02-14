use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::Error;
use actix_web::middleware::Logger;
use log::info;
use actix_web::dev::Transform;
use actix_web::{dev::Service, error::ErrorUnauthorized, http::header, HttpMessage};
use futures_util::future::{LocalBoxFuture, Ready, ready};
use serde::{Deserialize, Serialize};
use std::task::{Context, Poll};
use crate::shared::auth;

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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

pub struct AuthMiddleware;

impl AuthMiddleware {
    pub fn new() -> Self {
        AuthMiddleware
    }
}

impl<S, B> Transform<S, ServiceRequest> for AuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddlewareService { service }))
    }
}

pub struct AuthMiddlewareService<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&self, cx: &mut std::task::Context<'_>) -> std::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let auth_header = match req.headers().get(header::AUTHORIZATION) {
            Some(header) => header,
            None => {
                return Box::pin(ready(Err(ErrorUnauthorized("No authorization header"))));
            }
        };

        let auth_str = match auth_header.to_str() {
            Ok(str) => str,
            Err(_) => {
                return Box::pin(ready(Err(ErrorUnauthorized("Invalid authorization header"))));
            }
        };

        if !auth_str.starts_with("Bearer ") {
            return Box::pin(ready(Err(ErrorUnauthorized("Invalid authorization header format"))));
        }

        let token = &auth_str["Bearer ".len()..];
        
        match auth::verify_token(token) {
            Ok(claims) => {
                req.extensions_mut().insert(claims);
                let fut = self.service.call(req);
                Box::pin(async move {
                    let res = fut.await?;
                    Ok(res)
                })
            }
            Err(_) => {
                Box::pin(ready(Err(ErrorUnauthorized("Invalid token"))))
            }
        }
    }
}