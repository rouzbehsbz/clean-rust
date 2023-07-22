use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use serde::Serialize;
use thiserror::Error as ThisError;
use validator::{ValidationError, ValidationErrors};

use crate::infrastructure::common::date_service::DateService;

#[derive(Debug, ThisError)]
pub enum Error {
    #[error("Oops! Something went wrong.")]
    InternalServerError,
    #[error("{0}")]
    EntityNotFOund(String),
    #[error("{0}")]
    EntityExists(String),
    #[error("{0}")]
    InputValidation(String),
}

impl ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        match *self {
            Self::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            Self::EntityNotFOund(_) => StatusCode::NOT_FOUND,
            Self::EntityExists(_) => StatusCode::CONFLICT,
            Self::InputValidation(_) => StatusCode::CONFLICT,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let status_code = self.status_code();
        let timestamp = DateService::get_cuurent_timestamp();

        let error_response = ErrorResponse {
            code: status_code.as_u16(),
            message: self.to_string(),
            timestamp,
        };

        HttpResponse::build(status_code).json(error_response)
    }
}

impl From<ValidationErrors> for Error {
    fn from(value: ValidationErrors) -> Self {
        Self::InputValidation(value.to_string())
    }
}

#[derive(Debug, Serialize)]
struct ErrorResponse {
    code: u16,
    message: String,
    timestamp: u64,
}
