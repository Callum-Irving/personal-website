use actix_web::{HttpResponse, ResponseError};
use deadpool_postgres::PoolError;
use std::fmt;

#[derive(Debug)]
pub enum MyError {
    NotFound,
    PoolError(PoolError),
    DBError,
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            MyError::NotFound => write!(f, "Error: not found"),
            MyError::DBError => write!(f, "Error: database error"),
            MyError::PoolError(ref e) => write!(f, "{}", e)
        }
    }
}

impl ResponseError for MyError {
    fn error_response(&self) -> actix_web::HttpResponse {
        match *self {
            MyError::NotFound => HttpResponse::NotFound().finish(),
            MyError::DBError => HttpResponse::InternalServerError().finish(),
            _ => HttpResponse::InternalServerError().finish(),
        }
    }
}
