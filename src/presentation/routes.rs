use actix_web::web::{self, ServiceConfig};

use crate::{application::usecases::user::service::UserService, infrastructure::{authentication::jwt_token_handler::JwtTokenHandler, persistance::memory::models::user::UserModel}};

use super::controllers::users::handler::register;

pub fn routes(cfg: &mut ServiceConfig) {
    let jwt_token_handler = JwtTokenHandler::new("secret");
    let user_repository = UserModel::new();
    let user_service = UserService::new(
        jwt_token_handler,
        user_repository
    );

    cfg.service(web::scope("/api").service(
        web::scope("/v1").service(web::scope("/users").route("", web::post().to(register))),
    ));
}
