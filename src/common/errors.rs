use std::vec;

use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use serde::{Serialize, de::Error as DeserializeError};
use thiserror::Error as ThisError;
use validator::{ValidationErrors, ValidationErrorsKind::Field};

use crate::infrastructure::common::date_service::DateService;

#[derive(Debug, ThisError)]
pub enum Error {
    #[error("Oops! Something went wrong.")]
    InternalServerError,
    #[error("{0}")]
    EntityNotFOund(String),
    #[error("{0}")]
    EntityExists(String),
    #[error("Input validation failed.")]
    InputValidation(Vec<FieldError>),
}

impl ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            Self::EntityNotFOund(_) => StatusCode::NOT_FOUND,
            Self::EntityExists(_) => StatusCode::CONFLICT,
            Self::InputValidation(_) => StatusCode::CONFLICT,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let status_code = self.status_code();
        let message = self.to_string();
        let timestamp = DateService::get_cuurent_timestamp();

        match self {
            Self::InputValidation(fields) => {
                let error_response = ValidationErrorResponse {
                    code: status_code.as_u16(),
                    message,
                    fields: fields.clone(),
                    timestamp
                };

                HttpResponse::build(status_code).json(error_response)
            },
            _ => {
                let error_response = CommonErrorResponse {
                    code: status_code.as_u16(),
                    message,
                    timestamp,
                };

                HttpResponse::build(status_code).json(error_response)
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
                    reason: err.iter().next().unwrap().message.as_ref().unwrap().to_string()
                };
    
                fields.push(field_error);
            }
        }

        Self::InputValidation(fields)
    }
}

impl DeserializeError for Error {
    fn missing_field(field: &'static str) -> Self {        
        let mut fields: Vec<FieldError> = vec![];

        fields.push(FieldError { name: field.to_string(), reason: format!("Field is missing.") });

        Self::InputValidation(fields)
    }

    fn custom<T>(msg:T) -> Self where T:std::fmt::Display {
        Self::InternalServerError
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
    timestamp: u64
}

#[derive(Debug, Clone, Serialize)]
pub struct FieldError {
    name: String,
    reason: String
}
