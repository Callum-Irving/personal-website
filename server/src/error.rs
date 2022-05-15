use actix_web::{HttpResponse, ResponseError};
use deadpool_postgres::PoolError;
use std::fmt;

#[derive(Debug)]
pub enum MyError {
    Postgres(tokio_postgres::Error),
    Pool(PoolError),
    Database,
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            MyError::Postgres(_) => write!(f, "Error: Postgres error"),
            MyError::Database => write!(f, "Error: database error"),
            MyError::Pool(ref e) => write!(f, "{}", e),
        }
    }
}

impl ResponseError for MyError {
    fn error_response(&self) -> actix_web::HttpResponse {
        HttpResponse::InternalServerError().finish()
    }
}
