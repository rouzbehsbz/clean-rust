use actix_web::{web::{Data, Json, Path}, HttpRequest, HttpMessage, http};
use validator::Validate;

use crate::{
    application::{usecases::user::{
        dto::{UserLoginInput, UserRegisterInput, UpdateUserPofileInput, GetProfileInput},
        interface::IUserService,
    }, common::interfaces::authentication::jwt_token_handler::JwtPayload},
    common::types::AppResult, presentation::middlewares::authentication::AuthenticationMiddleware,
};

use super::dto::{AuthenticatedUserResponse, UserLoginRequest, UserRegisterRequest, UpdateProfileRequest, UpdateProfileResponse, GetUserProfileResponse, GetUserProfileParams};

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
    user_service: Data<dyn IUserService>,
    payload: Json<UpdateProfileRequest>,
    req: HttpRequest,
) -> AppResult<Json<UpdateProfileResponse>> {
    payload.validate()?;

    let user_id = req.extensions().get::<JwtPayload>().unwrap().user_id;

    user_service.update_profile(&UpdateUserPofileInput {
        id: user_id,
        email: payload.email.clone(),
        first_name: payload.first_name.clone(),
        last_name: payload.last_name.clone(),
        password: payload.password.clone()
    }).await?;

    Ok(Json(UpdateProfileResponse { message: format!("Your profile has been updated successfully.")}))
}

pub async fn get_profile(
    user_service: Data<dyn IUserService>,
    params: Path<GetUserProfileParams>
) -> AppResult<Json<GetUserProfileResponse>> {
    params.validate()?;

    let result = user_service.get_profile(&GetProfileInput {
        id: params.user_id
    }).await?;

    Ok(Json(GetUserProfileResponse { 
        first_name: result.first_name,
        last_name: result.last_name,
        email: result.email
    }))
}
