use actix_web::{HttpResponse, http::StatusCode};
use serde::Serialize;
use crate::infrastructure::common::date_service::DateService;

#[derive(Debug, Serialize)]
pub struct ApiResponse<T>
where T: Serialize
{
    code: u16,
    error: bool,
    message: Option<String>,
    result: Option<T>,
    timestamp: u64
}

impl <T> ApiResponse<T> 
where T: Serialize
{
    pub fn success(message: Option<String>, result: Option<T>) -> Self {
        Self {
            code: 200,
            error: false,
            message,
            result,
            timestamp: DateService::get_cuurent_timestamp()
        }
    }

    pub fn error(code: u16, message: Option<String>, result: Option<T>) -> Self {
        Self {
            code,
            error: true,
            message,
            result,
            timestamp: DateService::get_cuurent_timestamp()
        }
    }
}

//TODO: seprate error codes with http status code
impl <T> Into<HttpResponse> for ApiResponse<T> 
where T: Serialize
{
    fn into(self) -> HttpResponse {
        HttpResponse::build(StatusCode::from_u16(self.code).unwrap()).json(self)
    }
}