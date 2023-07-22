use actix_web::web::{Json, Data};

use crate::{application::usecases::user::{dto::UserRegisterInput, interface::IUserService, service::UserService}, infrastructure::{authentication::jwt_token_handler::JwtTokenHandler, persistance::memory::models::user::UserModel}, common::types::AppResult};

use super::dto::{AuthenticatedUserResponse, UserRegisterRequest};

// TODO: make it more effecient
pub async fn register(
    user_service: Data<dyn IUserService>,
    payload: Json<UserRegisterRequest>,
) -> AppResult<Json<AuthenticatedUserResponse>> {
    let result = user_service
        .register(&UserRegisterInput {
            email: payload.email.clone(),
            first_name: payload.first_name.clone(),
            last_name: payload.last_name.clone(),
            password: payload.password.clone(),
        })
        .await?;

    Ok(Json(AuthenticatedUserResponse {
        id: result.user.id,
        first_name: result.user.first_name,
        last_name: result.user.last_name,
        email: result.user.email,
        access_token: result.access_token,
    }))
}
