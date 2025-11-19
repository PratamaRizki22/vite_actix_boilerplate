use actix_web::dev::{ServiceRequest, ServiceResponse, Transform, Service};
use actix_web::{Error, HttpMessage};
use futures_util::future::{ready, Ready, LocalBoxFuture};
use std::rc::Rc;
use crate::services::token_blacklist::TokenBlacklist;

// Lightweight middleware scaffold: checks token against provided TokenBlacklist instance
pub struct JwtBlacklist {
    pub blacklist: Rc<TokenBlacklist>,
}

impl JwtBlacklist {
    pub fn new(blacklist: Rc<TokenBlacklist>) -> Self {
        Self { blacklist }
    }
}

impl<S, B> Transform<S, ServiceRequest> for JwtBlacklist
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = JwtBlacklistMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(JwtBlacklistMiddleware { service: Rc::new(service), blacklist: self.blacklist.clone() }))
    }
}

pub struct JwtBlacklistMiddleware<S> {
    service: Rc<S>,
    blacklist: Rc<TokenBlacklist>,
}

impl<S, B> Service<ServiceRequest> for JwtBlacklistMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&self, cx: &mut std::task::Context<'_>) -> std::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let srv = self.service.clone();
        let blacklist = self.blacklist.clone();

        Box::pin(async move {
            // TODO: integrate with TokenBlacklist::is_blacklisted(pool, token) using app data
            // For now this middleware is a pass-through scaffold so tests/compilation succeed.
            srv.call(req).await
        })
    }
}
