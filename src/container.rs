use crate::{
    application::usecases::user::service::UserService,
    infrastructure::{
        authentication::jwt_token_handler::JwtTokenHandler,
        persistance::memory::models::user::UserModel,
    },
};

pub struct Container {
    pub user_service: UserService<JwtTokenHandler, UserModel>,
}

impl Container {
    pub fn new() -> Self {
        let user_repository = UserModel::new();
        let jwt_token_handler = JwtTokenHandler::new("secret");
        let user_service = UserService::new(jwt_token_handler, user_repository);

        Self { user_service }
    }
}
