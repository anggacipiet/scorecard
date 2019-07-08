use actix_web::error::{BlockingError, ContentTypeError, JsonPayloadError};
use actix_web::{http, Error, HttpResponse, ResponseError};
use serde::{Deserialize, Serialize};
use serde::{de, ser};
use diesel;
use failure::Fail;
use jwt;
use log::error;
use mysql;
use reqwest;
use std::io;
use validator;
use std::convert::From;
use std::fmt::Display;

#[derive(Debug, Serialize, Deserialize)]
pub struct SCErrorResponse {
    errors: Vec<String>,
}


#[derive(Debug, Serialize)]
pub struct ApiError {
    message: String,
    errors: Option<validator::ValidationErrors>,
}

impl ApiError {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_owned(),
            errors: None,
        }
    }

    pub fn with_errors(message: &str, errors: validator::ValidationErrors) -> Self {
        Self {
            message: message.to_owned(),
            errors: Some(errors),
        }
    }
}

#[derive(Fail, Debug)]
pub enum AppError {
    /*#[fail(display = "Error in actix, {}", _0)]
    Actix(Error),
    #[fail(display = "Request was made with bad Content-Type header")]
    ContentType,
    #[fail(display = "Error in diesel, {}", _0)]
    Diesel(#[cause] diesel::result::Error),
    #[fail(display = "Error in r2d2, {}", _0)]
    R2d2(#[cause] r2d2::Error),
    #[fail(display = "Error IO, {}", _0)]
    IO(#[cause] io::Error),
    #[fail(display = "Error mysql, {}", _0)]
    Mysql(#[cause] mysql::Error),
    #[fail(display = "Error  mysql row, {}", _0)]
    MysqlRow(#[cause] mysql::FromRowError),
    #[fail(display = "Error  mysql value, {}", _0)]
    MysqlVal(#[cause] mysql::FromValueError),
    #[fail(display = "Error reqwest, {}", _0)]
    Reqwest(#[cause] reqwest::Error),
    #[fail(display = "Failed to log in user")]
    Login,
    #[fail(display = "Failed to logout in user")]
    Logout,
    #[fail(display = "Must be authorized to perform this action")]
    Auth,
    #[fail(display = "Missing required fields")]
    MissingFields,
    */

    #[fail(display = "internal error")]
    Internal,
    #[fail(display = "bad request")]
    BadRequest(ApiError),
    #[fail(display = "not found")]
    NotFound,
    #[fail(display = "timeout")]
    Timeout,
    #[fail(display = "unauthorized")]
    Unauthorized,
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            AppError::Internal => HttpResponse::new(http::StatusCode::INTERNAL_SERVER_ERROR),
            AppError::BadRequest(ref api_error) => {
                HttpResponse::build(http::StatusCode::BAD_REQUEST).json(api_error)
            }
            AppError::NotFound => HttpResponse::new(http::StatusCode::NOT_FOUND),
            AppError::Timeout => HttpResponse::new(http::StatusCode::GATEWAY_TIMEOUT),
            AppError::Unauthorized => HttpResponse::new(http::StatusCode::UNAUTHORIZED),
        }
    }
}

/*
impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            AppError::Actix(ref e) => {
                HttpResponse::InternalServerError().json(SCErrorResponse {
                    errors: vec![format!("{}", e)],
                })
            }
            AppError::Diesel(ref e) => {
                let body = SCErrorResponse {
                    errors: vec![format!("{}", e)],
                };

                match *e {
                    diesel::result::Error::NotFound => HttpResponse::NotFound().json(body),
                    _ => HttpResponse::InternalServerError().json(body),
                }
            }
            AppError::R2d2(ref e) => {
                HttpResponse::InternalServerError().json(SCErrorResponse {
                    errors: vec![format!("{}", e)],
                })
            }
            AppError::Mysql(ref e) => {
                HttpResponse::InternalServerError().json(SCErrorResponse {
                    errors: vec![format!("{}", e)],
                })
            }
            AppError::MysqlRow(ref e) => {
                HttpResponse::InternalServerError().json(SCErrorResponse {
                    errors: vec![format!("{}", e)],
                })
            }
            AppError::MysqlVal(ref e) => {
                HttpResponse::InternalServerError().json(SCErrorResponse {
                    errors: vec![format!("{}", e)],
                })
            }
            AppError::Reqwest(ref e) => {
                HttpResponse::InternalServerError().json(SCErrorResponse {
                    errors: vec![format!("{}", e)],
                })
            }
            AppError::IO(ref e) => {
                HttpResponse::InternalServerError().json(SCErrorResponse {
                    errors: vec![format!("{}", e)],
                })
            }
            AppError::Auth => HttpResponse::Unauthorized().json(SCErrorResponse {
                errors: vec![format!("{}", self)],
            }),
            AppError::MissingFields
            | AppError::ContentType
            | AppError::Login
            | AppError::Logout => {
                HttpResponse::BadRequest().json(SCErrorResponse {
                    errors: vec![format!("{:?}", self)],
                })
            }
        }
    }
}
*/
impl From<diesel::result::Error> for AppError {
    fn from(error: diesel::result::Error) -> Self {
        error!("ERROR diesel = {:?}", error);
        match error {
            diesel::result::Error::DatabaseError(
                diesel::result::DatabaseErrorKind::UniqueViolation,
                _,
            ) => AppError::BadRequest(ApiError::new("already.exist")),
            diesel::result::Error::NotFound => AppError::NotFound,
            _ => AppError::Internal,
        }
    }
}

impl From<JsonPayloadError> for AppError {
    fn from(error: JsonPayloadError) -> Self {
        error!("ERROR actix JsonPayloadError = {:?}", error);
        match error {
            JsonPayloadError::Deserialize(json_error) => {
                AppError::BadRequest(ApiError::new(&format!("{}", json_error)))
            }
            _ => AppError::BadRequest(ApiError::new("Json parsing error")),
        }
    }
}

impl From<BlockingError<AppError>> for AppError {
    fn from(error: BlockingError<Self>) -> Self {
        error!("ERROR actix BlockingError = {:?}", error);
        match error {
            BlockingError::Error(app_error) => app_error,
            BlockingError::Canceled => AppError::Internal,
        }
    }
}

impl From<r2d2::Error> for AppError {
    fn from(e: r2d2::Error) -> Self {
        error!("ERROR r2d2 = {:?}", e);
        AppError::BadRequest(ApiError::new(&format!("{}", e)))
    }
}

impl From<mysql::Error> for AppError {
    fn from(e: mysql::Error) -> Self {
        error!("ERROR mysql = {:?}", e);
        AppError::BadRequest(ApiError::new(&format!("{}", e)))
    }
}

impl From<mysql::FromRowError> for AppError {
    fn from(e: mysql::FromRowError) -> Self {
        error!("ERROR mysql row = {:?}", e);
        AppError::BadRequest(ApiError::new(&format!("{}", e)))
    }
}


impl From<mysql::FromValueError> for AppError {
    fn from(e: mysql::FromValueError) -> Self {
        error!("ERROR mysql value = {:?}", e);
       AppError::BadRequest(ApiError::new(&format!("{}", e)))
    }
}

impl From<serde_json::Error> for AppError {
    fn from(e: serde_json::Error) -> Self {
        error!("ERROR serde json value = {:?}", e);
       AppError::BadRequest(ApiError::new(&format!("{}", e)))
    }
}

impl From<jwt::errors::Error> for AppError {
    fn from(error: jsonwebtoken::errors::Error) -> Self {
        error!("ERROR jsonwebtoken = {:?}", error);
        AppError::Unauthorized
    }
}

impl From<reqwest::Error> for AppError {
    fn from(e: reqwest::Error) -> Self {
        error!("ERROR reqwest = {:?}", e);
        AppError::BadRequest(ApiError::new(&format!("{}", e)))
    }
}

impl From<io::Error> for AppError {
    fn from(e: io::Error) -> Self {
        error!("ERROR IO = {:?}", e);
       AppError::BadRequest(ApiError::new(&format!("{}", e)))
    }
}

impl From<validator::ValidationErrors> for AppError {
    fn from(error: validator::ValidationErrors) -> Self {
        error!("ERROR validator = {:?}", error);
        AppError::BadRequest(error.into())
    }
}

impl From<validator::ValidationErrors> for ApiError {
    fn from(errors: validator::ValidationErrors) -> Self {
        error!("ERROR validator = {:?}", errors);
        Self::with_errors("validation.error", errors)
    }
}
