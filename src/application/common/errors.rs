use std::vec;

use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use serde::{Serialize};
use thiserror::Error as ThisError;
use validator::{ValidationErrors, ValidationErrorsKind::Field};
use sqlx::Error as SqlxError;

use super::api_response::ApiResponse;

#[derive(Debug, ThisError)]
pub enum Error {
    #[error("Oops! Something went wrong.")]
    InternalServerError,
    #[error("{0}")]
    EntityNotFound(String),
    #[error("{0}")]
    EntityExists(String),
    #[error("{0}")]
    EntityValidationFailed(String),
    #[error("Input validation failed.")]
    InputValidation(Vec<FieldError>),
    #[error("{0}")]
    AuthorizationFailed(String),
    #[error("Database operation failed")]
    DatabaseFailed
}

impl ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            Self::EntityNotFound(_) => StatusCode::NOT_FOUND,
            Self::EntityExists(_) => StatusCode::CONFLICT,
            Self::InputValidation(_) => StatusCode::CONFLICT,
            Self::EntityValidationFailed(_) => StatusCode::CONFLICT,
            Self::AuthorizationFailed(_) => StatusCode::UNAUTHORIZED,
            Self::DatabaseFailed => StatusCode::INTERNAL_SERVER_ERROR
        }
    }

    fn error_response(&self) -> HttpResponse {
        let status_code = self.status_code();
        let message = self.to_string();

        match self {
            Self::InputValidation(fields) => {
                let result = fields.clone();

                ApiResponse::error(
                    status_code.as_u16(),
                    Some(message),
                    Some(result)
                ).into()
            }
            _ => {
                ApiResponse::<()>::error(
                    status_code.as_u16(),
                    Some(message),
                    None
                ).into()
            }
        }
    }
}

impl From<ValidationErrors> for Error {
    fn from(value: ValidationErrors) -> Self {
        let mut fields: Vec<FieldError> = vec![];

        for (field_name, error) in value.errors().iter() {
            if let Field(err) = error {
                let field_error = FieldError {
                    name: field_name.to_string(),
                    reason: err
                        .iter()
                        .next()
                        .unwrap()
                        .message
                        .as_ref()
                        .unwrap()
                        .to_string(),
                };

                fields.push(field_error);
            }
        }

        Self::InputValidation(fields)
    }
}

impl From<SqlxError> for Error {
    fn from(_: SqlxError) -> Self {
        Self::DatabaseFailed
    }
}

#[derive(Debug, Serialize)]
struct CommonErrorResponse {
    code: u16,
    message: String,
    timestamp: u64,
}

#[derive(Debug, Serialize)]
struct ValidationErrorResponse {
    code: u16,
    message: String,
    fields: Vec<FieldError>,
    timestamp: u64,
}

#[derive(Debug, Clone, Serialize)]
pub struct FieldError {
    name: String,
    reason: String,
}
