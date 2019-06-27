use actix_service::{Service, Transform};

use actix_web::error::{ErrorBadRequest, ErrorForbidden, ErrorUnauthorized};
use actix_web::{
    dev::{ServiceRequest, ServiceResponse},
    web, Error, HttpResponse,
};
use futures::{
    future::{ok, Either, Future, FutureResult},
    Poll,
};
use log::{debug, error, info, trace, warn};
use serde_json::json;
use std::cell::RefCell;
use std::rc::Rc;


use crate::db;
use crate::token::{decode_token, verify_token};

pub struct CheckAuth;

impl<S, B> Transform<S> for CheckAuth
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    //S: 'static,
    //B: 'static,
    S::Future: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    //type Error = Error;
    type Error = S::Error;
    type InitError = ();
    type Transform = CheckAuthMiddleware<S>;
    type Future = FutureResult<Self::Transform, Self::InitError>;

    fn new_transform(&self, service: S) -> Self::Future {
        //ok(CheckAuthMiddleware {
        //    service: Rc::new(RefCell::new(service)),
        //})
        ok(CheckAuthMiddleware { service })
    }
}
pub struct CheckAuthMiddleware<S> {
    //service: Rc<RefCell<S>>,
    service: S,
}

impl<S, B> Service for CheckAuthMiddleware<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    //S: 'static,
    //B: 'static,
    S::Future: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    //type Error = S::Error;
    //type Future = Box<Future<Item = Self::Response, Error = Self::Error>>;
    type Error = Error;
    type Future = Either<S::Future, FutureResult<Self::Response, Self::Error>>;

    fn poll_ready(&mut self) -> Poll<(), Self::Error> {
        self.service.poll_ready()
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        //let db: web::Data<db::Pool> = req.app_data().unwrap();
        /*let allow = req.app_data().and_then(|db| {
            req.headers()
                .get("AUTHORIZATION")
                .and_then(|token| token::decode_token(&db, token.to_str().ok()?).ok())
        });
        match allow {
            Some(_) => Either::A(self.service.call(req)),
            None => Either::B(ok(req.error_response(ErrorForbidden("Please authenticate"),))),
        }
        */
        if let Some(pool) = req.app_data::<db::Pool>() {
            info!("Connecting to database...");
            if let Some(authen_header) = req.headers().get("Authorization") {
                info!("Parsing authorization header...");
                if let Ok(authen_str) = authen_header.to_str() {
                    if authen_str.starts_with("Bearer") {
                        info!("Parsing token...");
                        let token = authen_str[6..authen_str.len()].trim();
                        if let Ok(token_data) = decode_token(token.to_string()) {
                            info!("Decoding token...");
                            if verify_token(&token_data, &pool).is_ok() {
                                info!("Valid token");
                                return Either::A(self.service.call(req));
                            } else {
                                error!("Invalid token");
                                return Either::B(ok(req.into_response(
                                    HttpResponse::Unauthorized()
                                        .json(json!({
                                        "message":"Invalid token, please login again",
                                        "status":false,
                                        "data": "Not authorization"}))
                                        .into_body(),
                                )));
                            }
                        }
                    }
                }
            } else if req.path() == "/api/v1.0.0/sc-login" {
                return Either::A(self.service.call(req));
            } else {
                error!("{}", "Error process authentication");
                return Either::B(ok(req.into_response(
                    HttpResponse::BadRequest()
                        .json(json!({
                            "message":"Error process authentication",
                            "status":false,
                            "data": "Error authentication"}))
                        .into_body(),
                )));
            }
        }

        error!("{}", "Error process authentication");
        Either::B(ok(req.into_response(
            HttpResponse::Forbidden()
                .json(json!({
                    "message":"Error process authentication",
                    "status":false,
                    "data": "Error authentication"}))
                .into_body(),
        )))
    }
}
