use actix_web::{web::{Data, Json}, HttpRequest, HttpMessage, http};
use validator::Validate;

use crate::{
    application::{usecases::user::{
        dto::{UserLoginInput, UserRegisterInput},
        interface::IUserService,
    }, common::interfaces::authentication::jwt_token_handler::JwtPayload},
    common::types::AppResult, presentation::middlewares::authentication::AuthenticationMiddleware,
};

use super::dto::{AuthenticatedUserResponse, UserLoginRequest, UserRegisterRequest, UpdateProfileRequest, UpdateProfileResponse};

// TODO: make payload validation automatic for handlers
// TODO: make it more effecient
// TODO: make global Http Response
pub async fn register(
    user_service: Data<dyn IUserService>,
    payload: Json<UserRegisterRequest>,
) -> AppResult<Json<AuthenticatedUserResponse>> {
    payload.validate()?;

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

pub async fn login(
    user_service: Data<dyn IUserService>,
    payload: Json<UserLoginRequest>,
) -> AppResult<Json<AuthenticatedUserResponse>> {
    payload.validate()?;

    let result = user_service
        .login(&UserLoginInput {
            email: payload.email.clone(),
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

pub async fn update_profile(
    _: AuthenticationMiddleware,
    // user_service: Data<dyn IUserService>,
    // payload: Json<UpdateProfileRequest>,
    req: HttpRequest,
) -> AppResult<Json<UpdateProfileResponse>>{
    let message = req.extensions().get::<JwtPayload>().unwrap().user_id;

    Ok(Json(UpdateProfileResponse { message: format!("Your user id is {}", message) }))
}
