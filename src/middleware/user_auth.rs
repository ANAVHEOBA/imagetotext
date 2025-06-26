use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage, HttpResponse,
    http::header,
};
use futures::future::{ready, Ready, LocalBoxFuture};
use std::rc::Rc;

use crate::services::jwt::{JwtService, JwtError, Claims};
use crate::modules::{UserCRUD, ErrorResponse};

fn create_error_response(status: actix_web::http::StatusCode, error: &str, message: &str) -> Error {
    let response = HttpResponse::build(status).json(ErrorResponse {
        error: error.to_string(),
        message: message.to_string(),
    });
    actix_web::error::InternalError::from_response("", response).into()
}

pub struct Authentication;

impl Authentication {
    pub fn new() -> Self {
        Authentication
    }
}

impl<S, B> Transform<S, ServiceRequest> for Authentication
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = AuthenticationMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthenticationMiddleware { service: Rc::new(service) }))
    }
}

pub struct AuthenticationMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for AuthenticationMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        println!("AUTH_MIDDLEWARE: Running authentication check for path: {}", req.path());
        // Skip authentication for some public routes
        if req.path().contains("/public/") {
            println!("AUTH_MIDDLEWARE: Skipping auth for public route.");
            return Box::pin(self.service.call(req));
        }

        let auth_header = match req.headers().get(header::AUTHORIZATION) {
            Some(header) => header.to_str().unwrap_or(""),
            None => {
                return Box::pin(async {
                    Err(create_error_response(
                        actix_web::http::StatusCode::UNAUTHORIZED,
                        "Unauthorized",
                        "Missing authorization header",
                    ))
                });
            }
        };

        let token = match auth_header.strip_prefix("Bearer ") {
            Some(token) => token.trim(),
            None => {
                return Box::pin(async {
                    Err(create_error_response(
                        actix_web::http::StatusCode::UNAUTHORIZED,
                        "Unauthorized",
                        "Invalid token format",
                    ))
                });
            }
        };

        match JwtService::validate_token(token) {
            Ok(claims) => {
                println!("AUTH_MIDDLEWARE: Token validation successful for user: {}", claims.email);
                req.extensions_mut().insert(claims);
                Box::pin(self.service.call(req))
            }
            Err(e) => {
                let message = match e {
                    JwtError::TokenExpired => "Token has expired",
                    JwtError::InvalidToken => "Invalid token",
                    _ => "Authentication failed",
                };
                Box::pin(async move {
                    Err(create_error_response(
                        actix_web::http::StatusCode::UNAUTHORIZED,
                        "Unauthorized",
                        message,
                    ))
                })
            }
        }
    }
}

pub struct RequireVerifiedEmail;

impl RequireVerifiedEmail {
    pub fn new() -> Self {
        RequireVerifiedEmail
    }
}

impl<S, B> Transform<S, ServiceRequest> for RequireVerifiedEmail
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = VerifiedEmailMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(VerifiedEmailMiddleware { service: Rc::new(service) }))
    }
}

pub struct VerifiedEmailMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for VerifiedEmailMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let claims = match req.extensions().get::<Claims>().cloned() {
            Some(claims) => claims,
            None => {
                return Box::pin(async {
                    Err(create_error_response(
                        actix_web::http::StatusCode::UNAUTHORIZED,
                        "Unauthorized",
                        "Authentication required",
                    ))
                });
            }
        };
        
        let service = self.service.clone();
        Box::pin(async move {
            match UserCRUD::find_by_uuid(&claims.sub).await {
                Ok(Some(user)) if user.is_verified => {
                    service.call(req).await
                }
                Ok(Some(_)) => Err(create_error_response(
                    actix_web::http::StatusCode::FORBIDDEN,
                    "Email not verified",
                    "Please verify your email address to access this resource",
                )),
                _ => Err(create_error_response(
                    actix_web::http::StatusCode::UNAUTHORIZED,
                    "User not found",
                    "Invalid user credentials",
                )),
            }
        })
    }
}
