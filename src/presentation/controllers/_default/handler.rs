use actix_web::HttpResponse;

use crate::application::common::api_response::ApiResponse;

pub async fn route_not_found() -> HttpResponse {
    ApiResponse::<()>::error(
        404,
        Some(format!("Endpoint not found.")),
        None
    ).into()
}