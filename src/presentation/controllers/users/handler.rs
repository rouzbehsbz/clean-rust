use actix_web::web::Json;

use crate::application::usecases::user::{dto::UserRegisterInput, interface::IUserService};

use super::dto::{AuthenticatedUserResponse, UserRegisterRequest};

// TODO: make it more effecient
pub async fn register<T: IUserService>(
    mut user_service: T,
    payload: Json<UserRegisterRequest>,
) -> Json<AuthenticatedUserResponse> {
    let result = user_service
        .register(&UserRegisterInput {
            email: payload.email.clone(),
            first_name: payload.first_name.clone(),
            last_name: payload.last_name.clone(),
            password: payload.password.clone(),
        })
        .await
        .unwrap();

    Json(AuthenticatedUserResponse {
        id: result.user.id,
        first_name: result.user.first_name,
        last_name: result.user.last_name,
        email: result.user.email,
        access_token: result.access_token,
    })
}
