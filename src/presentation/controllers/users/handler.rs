use actix_web::{web::{Data, Json, Path}, HttpRequest, HttpMessage, http, HttpResponse};
use validator::Validate;

use crate::{
    application::{usecases::user::{
        dto::{UserLoginInput, UserRegisterInput, UpdateUserPofileInput, GetProfileInput},
        interface::IUserService,
    }, common::interfaces::authentication::jwt_token_handler::JwtPayload},
    application::common::{types::AppResult, api_response::ApiResponse}, presentation::middlewares::authentication::AuthenticationMiddleware,
};

use super::dto::{AuthenticatedUserResponse, UserLoginRequest, UserRegisterRequest, UpdateProfileRequest, UpdateProfileResponse, GetUserProfileResponse, GetUserProfileParams};

// TODO: make payload validation automatic for handlers
// TODO: make it more effecient
// TODO: make global Http Response
pub async fn register(
    user_service: Data<dyn IUserService>,
    payload: Json<UserRegisterRequest>,
) -> AppResult<HttpResponse> {
    payload.validate()?;

    let result = user_service
        .register(&UserRegisterInput {
            email: payload.email.clone(),
            first_name: payload.first_name.clone(),
            last_name: payload.last_name.clone(),
            password: payload.password.clone(),
        })
        .await?;

    let response_result = AuthenticatedUserResponse {
        id: result.user.id,
        first_name: result.user.first_name,
        last_name: result.user.last_name,
        email: result.user.email,
        access_token: result.access_token,
    };

    Ok(ApiResponse::success(
        Some(format!("You have successfully registered.")),
        Some(response_result)
    ).into())
}

pub async fn login(
    user_service: Data<dyn IUserService>,
    payload: Json<UserLoginRequest>,
) -> AppResult<HttpResponse> {
    payload.validate()?;

    let result = user_service
        .login(&UserLoginInput {
            email: payload.email.clone(),
            password: payload.password.clone(),
        })
        .await?;

    let response_result = AuthenticatedUserResponse {
        id: result.user.id,
        first_name: result.user.first_name,
        last_name: result.user.last_name,
        email: result.user.email,
        access_token: result.access_token,
    };

    Ok(ApiResponse::success(
        Some(format!("You have successfully logged in.")),
        Some(response_result)
    ).into())
}

pub async fn update_profile(
    _: AuthenticationMiddleware,
    user_service: Data<dyn IUserService>,
    payload: Json<UpdateProfileRequest>,
    req: HttpRequest,
) -> AppResult<HttpResponse> {
    payload.validate()?;

    let user_id = req.extensions().get::<JwtPayload>().unwrap().user_id;

    user_service.update_profile(&UpdateUserPofileInput {
        id: user_id,
        email: payload.email.clone(),
        first_name: payload.first_name.clone(),
        last_name: payload.last_name.clone(),
        password: payload.password.clone()
    }).await?;

    Ok(ApiResponse::<()>::success(
        Some(format!("Your profile has been successfully updated.")),
        None
    ).into())
}

pub async fn get_profile(
    user_service: Data<dyn IUserService>,
    params: Path<GetUserProfileParams>
) -> AppResult<HttpResponse> {
    params.validate()?;

    let result = user_service.get_profile(&GetProfileInput {
        id: params.user_id
    }).await?;

    let response_result = GetUserProfileResponse { 
        first_name: result.first_name,
        last_name: result.last_name,
        email: result.email
    };

    Ok(ApiResponse::success(
        None,
        Some(response_result)
    ).into())
}
