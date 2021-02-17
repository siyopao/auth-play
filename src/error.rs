use actix_web::{HttpResponse, ResponseError};
use derive_more::{Display, From};

#[derive(Debug, Display, From)]
pub enum MyError {
    NotFound,
    JWTError(jsonwebtoken::errors::Error),
    PGError(deadpool_postgres::tokio_postgres::error::Error),
    PGMError(tokio_pg_mapper::Error),
    PoolError(deadpool_postgres::PoolError),
    RowError,
    Utf8Error(std::str::Utf8Error),
}

impl ResponseError for MyError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            MyError::NotFound => HttpResponse::NotFound().finish(),
            MyError::PoolError(ref error) => {
                HttpResponse::InternalServerError().body(error.to_string())
            }
            _ => HttpResponse::InternalServerError().finish(),
        }
    }
}
