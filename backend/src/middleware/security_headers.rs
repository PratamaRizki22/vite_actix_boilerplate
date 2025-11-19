use actix_web::dev::{Service, Transform};
use actix_web::http::header;
use actix_web::{Error, HttpResponse};
use futures_util::future::{ok, LocalBoxFuture, Ready};
use std::rc::Rc;

/// Security Headers Middleware
/// Adds important security headers to all HTTP responses
pub struct SecurityHeadersMiddleware;

impl<S, B> Transform<S, actix_web::dev::ServiceRequest> for SecurityHeadersMiddleware
where
    S: Service<
            actix_web::dev::ServiceRequest,
            Response = actix_web::dev::ServiceResponse<B>,
            Error = Error,
        > + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = actix_web::dev::ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = SecurityHeadersMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(SecurityHeadersMiddlewareService {
            service: Rc::new(service),
        })
    }
}

pub struct SecurityHeadersMiddlewareService<S> {
    service: Rc<S>,
}

impl<S, B> Service<actix_web::dev::ServiceRequest> for SecurityHeadersMiddlewareService<S>
where
    S: Service<
            actix_web::dev::ServiceRequest,
            Response = actix_web::dev::ServiceResponse<B>,
            Error = Error,
        > + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = actix_web::dev::ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    actix_web::dev::forward_ready!(service);

    fn call(&self, req: actix_web::dev::ServiceRequest) -> Self::Future {
        let service = Rc::clone(&self.service);

        Box::pin(async move {
            let mut res = service.call(req).await?;

            // Add security headers
            let headers = res.headers_mut();

            // Content-Security-Policy: Prevent XSS attacks
            headers.insert(
                header::HeaderName::from_static("content-security-policy"),
                header::HeaderValue::from_static(
                    "default-src 'self'; script-src 'self'; style-src 'self'; img-src 'self' data: https:; font-src 'self' data:; connect-src 'self' https:; frame-ancestors 'none'; object-src 'none';"
                ),
            );

            // X-Content-Type-Options: Prevent MIME type sniffing
            headers.insert(
                header::HeaderName::from_static("x-content-type-options"),
                header::HeaderValue::from_static("nosniff"),
            );

            // X-Frame-Options: Prevent clickjacking
            headers.insert(
                header::HeaderName::from_static("x-frame-options"),
                header::HeaderValue::from_static("DENY"),
            );

            // X-XSS-Protection: Legacy XSS protection
            headers.insert(
                header::HeaderName::from_static("x-xss-protection"),
                header::HeaderValue::from_static("1; mode=block"),
            );

            // Strict-Transport-Security: Enforce HTTPS
            headers.insert(
                header::HeaderName::from_static("strict-transport-security"),
                header::HeaderValue::from_static("max-age=31536000; includeSubDomains"),
            );

            // Referrer-Policy: Control referrer information
            headers.insert(
                header::HeaderName::from_static("referrer-policy"),
                header::HeaderValue::from_static("strict-origin-when-cross-origin"),
            );

            // Permissions-Policy: Control browser features
            headers.insert(
                header::HeaderName::from_static("permissions-policy"),
                header::HeaderValue::from_static(
                    "geolocation=(), microphone=(), camera=(), payment=(), usb=(), magnetometer=(), gyroscope=(), accelerometer=()",
                ),
            );

            // Remove Server header to avoid information disclosure
            headers.remove(header::SERVER);

            Ok(res)
        })
    }
}
