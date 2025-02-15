use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::Error;
use log::{info, warn, error, debug};
use actix_web::dev::Transform;
use actix_web::{dev::Service, http::header, HttpMessage};
use futures_util::future::{LocalBoxFuture, Ready, ready};
use serde::{Deserialize, Serialize};
use std::task::{Context, Poll};
use crate::utils::auth;
use crate::utils::response::{Response, ResponseBuilder};


#[derive(Default, Clone)]
pub struct LoggingMiddleware;

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
        let remote_addr = req.connection_info().realip_remote_addr()
            .unwrap_or("unknown").to_string();

        debug!("Incoming request: {} {} from {}", method, path, remote_addr);

        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;
            let duration = start.elapsed();
            let status = res.status();
            
            if status.is_success() {
                info!(
                    "Request completed: {} {} - {} - {}ms from {}",
                    method, path, status.as_u16(), duration.as_millis(), remote_addr
                );
            } else {
                warn!(
                    "Request failed: {} {} - {} - {}ms from {}",
                    method, path, status.as_u16(), duration.as_millis(), remote_addr
                );
            }

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
        let path = req.path().to_owned();
        let method = req.method().clone();
        let remote_addr = req.connection_info().realip_remote_addr()
            .unwrap_or("unknown").to_string();

        debug!("Checking authentication for {} {} from {}", method, path, remote_addr);

        let auth_header = match req.headers().get(header::AUTHORIZATION) {
            Some(header) => header,
            None => {
                warn!("No authorization header in request from {}", remote_addr);
                return Box::pin(ready(Err(
                    actix_web::error::InternalError::from_response(
                        "Unauthorized",
                        Response::unauthorized("No authorization header provided")
                    ).into()
                )));
            }
        };

        let auth_str = match auth_header.to_str() {
            Ok(str) => str,
            Err(_) => {
                error!("Invalid authorization header format from {}", remote_addr);
                return Box::pin(ready(Err(
                    actix_web::error::InternalError::from_response(
                        "Unauthorized",
                        Response::unauthorized("Invalid authorization header format")
                    ).into()
                )));
            }
        };

        if !auth_str.starts_with("Bearer ") {
            warn!("Invalid token format from {}: missing Bearer prefix", remote_addr);
            return Box::pin(ready(Err(
                actix_web::error::InternalError::from_response(
                    "Unauthorized",
                    Response::unauthorized("Invalid authorization header format")
                ).into()
            )));
        }

        let token = &auth_str["Bearer ".len()..];
        
        match auth::verify_token(token) {
            Ok(claims) => {
                debug!("Successfully authenticated user {} for {} {}", 
                    claims.sub, method, path);
                req.extensions_mut().insert(claims);
                let fut = self.service.call(req);
                Box::pin(async move {
                    let res = fut.await?;
                    Ok(res)
                })
            }
            Err(e) => {
                error!("Token verification failed from {}: {}", remote_addr, e);
                Box::pin(ready(Err(
                    actix_web::error::InternalError::from_response(
                        "Unauthorized",
                        Response::unauthorized(&format!("Invalid or expired token: {}", e))
                    ).into()
                )))
            }
        }
    }
}