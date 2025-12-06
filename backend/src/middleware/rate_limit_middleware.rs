use actix_web::dev::{Service, ServiceResponse, Transform, forward_ready};
use actix_web::{
    body::BoxBody,
    Error, HttpResponse,
    dev::ServiceRequest,
    http::{StatusCode, header::HeaderValue},
};
use futures_util::future::{Ready, ready, ok};
use std::future::{Future};
use std::pin::Pin;
use std::rc::Rc;
use std::task::{Context, Poll};

use crate::middleware::redis_rate_limiter::RedisRateLimiter;

/// Rate limiting middleware using Redis
pub struct RateLimitMiddleware {
    pub endpoint_name: String,
    pub max_attempts: u32,
    pub window_seconds: u32,
}

impl RateLimitMiddleware {
    pub fn new(endpoint_name: &str, max_attempts: u32, window_seconds: u32) -> Self {
        Self {
            endpoint_name: endpoint_name.to_string(),
            max_attempts,
            window_seconds,
        }
    }
}

impl<S> Transform<S, ServiceRequest> for RateLimitMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<BoxBody>, Error = Error> + 'static,
    S::Future: 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type InitError = ();
    type Transform = RateLimitMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(RateLimitMiddlewareService {
            service: Rc::new(service),
            endpoint_name: self.endpoint_name.clone(),
            max_attempts: self.max_attempts,
            window_seconds: self.window_seconds,
        }))
    }
}

pub struct RateLimitMiddlewareService<S> {
    service: Rc<S>,
    endpoint_name: String,
    max_attempts: u32,
    window_seconds: u32,
}

impl<S> Service<ServiceRequest> for RateLimitMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<BoxBody>, Error = Error> + 'static,
    S::Future: 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = Rc::clone(&self.service);
        let endpoint_name = self.endpoint_name.clone();
        let max_attempts = self.max_attempts;
        let window_seconds = self.window_seconds;

        Box::pin(async move {
            // Get Redis rate limiter from app data
            let rate_limiter = req
                .app_data::<actix_web::web::Data<RedisRateLimiter>>()
                .ok_or_else(|| {
                    actix_web::error::ErrorInternalServerError("Rate limiter not configured")
                })?;

            // Check rate limit
            let (is_allowed, remaining, reset_seconds) = rate_limiter
                .check_limit_service(&req, &endpoint_name, max_attempts, window_seconds)
                .await;

            if !is_allowed {
                // Rate limit exceeded
                let response = HttpResponse::build(StatusCode::TOO_MANY_REQUESTS)
                    .insert_header(("X-RateLimit-Limit", max_attempts.to_string()))
                    .insert_header(("X-RateLimit-Remaining", "0"))
                    .insert_header(("X-RateLimit-Reset", reset_seconds.to_string()))
                    .insert_header(("Retry-After", reset_seconds.to_string()))
                    .json(serde_json::json!({
                        "error": "Too many requests",
                        "message": format!("Rate limit exceeded. Try again in {} seconds", reset_seconds),
                        "retry_after": reset_seconds
                    }))
                    .map_into_boxed_body();
                return Ok(req.into_response(response));
            }

            // Add rate limit headers to response
            let mut res = service.call(req).await?;
            
            let headers = res.headers_mut();
            headers.insert(
                actix_web::http::header::HeaderName::from_static("x-ratelimit-limit"),
                HeaderValue::from_str(&max_attempts.to_string()).unwrap(),
            );
            headers.insert(
                actix_web::http::header::HeaderName::from_static("x-ratelimit-remaining"),
                HeaderValue::from_str(&remaining.to_string()).unwrap(),
            );
            headers.insert(
                actix_web::http::header::HeaderName::from_static("x-ratelimit-reset"),
                HeaderValue::from_str(&reset_seconds.to_string()).unwrap(),
            );

            Ok(res)
        })
    }
}
