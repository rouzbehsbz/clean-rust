use actix_web::{ResponseError, http::StatusCode, HttpResponse};
use thiserror::Error as ThisError;
use serde::Serialize;

use crate::infrastructure::common::date_service::DateService;


#[derive(Debug, ThisError)]
pub enum Error {
    #[error("Oops! Something went wrong.")]
    InternalServerError,
    #[error("{0}")]
    EntityNotFOund(String),
    #[error("{0}")]
    EntityExists(String)
}

impl ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        match *self {
            Self::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            Self::EntityNotFOund(_) => StatusCode::NOT_FOUND,
            Self::EntityExists(_) => StatusCode::CONFLICT
        }
    }

    fn error_response(&self) -> HttpResponse {
        let status_code = self.status_code();
        let timestamp = DateService::get_cuurent_timestamp();

        let error_response = ErrorResponse {
            code: status_code.as_u16(),
            message: self.to_string(),
            timestamp
        };

        HttpResponse::build(status_code).json(error_response)
    }
}

#[derive(Debug, Serialize)]
struct ErrorResponse {
    code: u16,
    message: String,
    timestamp: u64
}