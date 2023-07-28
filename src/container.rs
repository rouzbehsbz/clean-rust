use crate::{
    application::usecases::user::service::UserService,
    infrastructure::{
        authentication::jwt_token_handler::JwtTokenHandler,
        persistance::postgres::{repositories::user::UserRepository, PostgresDatabase},
    },
};

pub struct Container {
    pub user_service: UserService<JwtTokenHandler, UserRepository>,
}

impl Container {
    pub async fn new() -> Self {
        let postgres_database = PostgresDatabase::new().await;
        let user_repository = UserRepository::new(postgres_database.clone());
        let jwt_token_handler = JwtTokenHandler::new();
        let user_service = UserService::new(jwt_token_handler, user_repository);

        Self { user_service }
    }
}
