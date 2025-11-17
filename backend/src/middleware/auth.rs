use actix_web::{
    dev::{ServiceRequest, ServiceResponse},
    Error, HttpMessage, Result,
};
use actix_web::dev::{forward_ready, Service, Transform};
use futures_util::future::{ready, Ready};
use std::future::Future;
use std::pin::Pin;
use std::rc::Rc;

use crate::auth::auth_utils::{AuthUtils, AuthError};
use crate::auth::auth_models::Claims;

pub struct AuthMiddleware {
    pub required_role: Option<String>,
}

impl AuthMiddleware {
    pub fn new() -> Self {
        Self { required_role: None }
    }

    pub fn require_role(role: &str) -> Self {
        Self {
            required_role: Some(role.to_string()),
        }
    }
}

impl<S> Transform<S, ServiceRequest> for AuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse, Error = Error> + 'static,
    S::Future: 'static,
{
    type Response = ServiceResponse;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddlewareService {
            service: Rc::new(service),
            required_role: self.required_role.clone(),
        }))
    }
}

pub struct AuthMiddlewareService<S> {
    service: Rc<S>,
    required_role: Option<String>,
}

impl<S> Service<ServiceRequest> for AuthMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse, Error = Error> + 'static,
    S::Future: 'static,
{
    type Response = ServiceResponse;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = Rc::clone(&self.service);
        let required_role = self.required_role.clone();

        Box::pin(async move {
            // Extract JWT secret from app data
            let jwt_secret = req
                .app_data::<actix_web::web::Data<String>>()
                .ok_or_else(|| {
                    actix_web::error::ErrorInternalServerError("JWT secret not found")
                })?;

            // Get Authorization header
            let auth_header = req
                .headers()
                .get("Authorization")
                .and_then(|h| h.to_str().ok())
                .ok_or_else(|| {
                    actix_web::error::ErrorUnauthorized("Authorization header missing")
                })?;

            // Extract and validate token
            let token = AuthUtils::extract_token_from_header(auth_header)
                .map_err(|_| actix_web::error::ErrorUnauthorized("Invalid token format"))?;

            let claims = AuthUtils::validate_token(token, jwt_secret)
                .map_err(|err| match err {
                    AuthError::TokenExpired => {
                        actix_web::error::ErrorUnauthorized("Token expired")
                    }
                    _ => actix_web::error::ErrorUnauthorized("Invalid token"),
                })?;

            // Check role if required
            if let Some(required) = &required_role {
                if !AuthUtils::has_role(&claims.role, required) {
                    return Err(actix_web::error::ErrorForbidden("Insufficient permissions"));
                }
            }

            // Add claims to request extensions for handlers to use
            req.extensions_mut().insert(claims);

            // Continue to the next service
            service.call(req).await
        })
    }
}

// Helper function to extract claims from request
pub fn get_current_user(req: &actix_web::HttpRequest) -> Option<Claims> {
    req.extensions().get::<Claims>().cloned()
}