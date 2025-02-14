use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    Error, error::ErrorUnauthorized, http::header, HttpMessage,
};
use futures_util::future::{LocalBoxFuture, Ready, ready};
use jsonwebtoken::{decode, DecodingKey, Validation};  // Remove TokenData since it's not used
use serde::{Deserialize, Serialize};
use std::env;

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
        let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

        let token_data = match decode::<Claims>(
            token,
            &DecodingKey::from_secret(secret.as_bytes()),
            &Validation::default(),
        ) {
            Ok(data) => data,
            Err(_) => {
                return Box::pin(ready(Err(ErrorUnauthorized("Invalid token"))));
            }
        };

        req.extensions_mut().insert(token_data.claims);

        let fut = self.service.call(req);
        Box::pin(async move {
            let res = fut.await?;
            Ok(res)
        })
    }
}