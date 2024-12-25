use std::env;
use std::future::{ready, Ready};

use actix_web::{dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform}, Error, HttpMessage, HttpResponse};
use actix_web::body::EitherBody;
use futures_util::future::LocalBoxFuture;
use futures_util::FutureExt;
use jsonwebtoken::{decode, DecodingKey, Validation};
use jsonwebtoken::errors::ErrorKind;

use crate::token::Claims;

pub struct Auth;

impl<S, B> Transform<S, ServiceRequest> for Auth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Transform = AuthMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddleware { service }))
    }
}

pub struct AuthMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        if let Some(auth_header) = req.headers().get("Authorization") {
            if let Ok(auth_str) = auth_header.to_str() {
                if auth_str.starts_with("Bearer ") {
                    let token = &auth_str[7..];

                    let private_key = env::var("SECRET_KEY")
                        .expect("SECRET_KEY is not set in .env");

                    match decode::<Claims>(
                        token,
                        &DecodingKey::from_secret(private_key.as_bytes()),
                        &Validation::default(),
                    ) {
                        Ok(token) => {
                            req.extensions_mut().insert::<i64>(token.claims.user_id.clone());
                            let res = self.service
                                .call(req)
                                .boxed_local();
                            return Box::pin(async move {
                                res.await.map(ServiceResponse::map_into_left_body)
                            })
                        },
                        Err(err) => {
                            if let ErrorKind::ExpiredSignature = err.kind() {
                                let (request, _pl) = req.into_parts();
                                let response = HttpResponse::Unauthorized()
                                    .body("Token expired")
                                    .map_into_right_body();
                                return Box::pin(async { Ok(ServiceResponse::new(request, response)) })
                            }
                        }
                    }
                }
            }
        }
        let (request, _pl) = req.into_parts();
        let response = HttpResponse::Unauthorized()
            .body("Invalid token")
            .map_into_right_body();
        Box::pin(async { Ok(ServiceResponse::new(request, response)) })
    }
}