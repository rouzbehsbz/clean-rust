use super::{
    dto::{AuthenticatedUserOutput, UserLoginInput, UserRegisterInput},
    interface::IUserService,
};
use crate::{
    application::common::interfaces::{
        authentication::jwt_token_handler::IJwtTokenHandler,
        persistance::user_repository::IUserRepository,
    },
    common::{errors::Error, types::AppResult},
    domain::entities::user::User,
};

pub struct UserService<T, K>
where
    T: IJwtTokenHandler,
    K: IUserRepository,
{
    jwt_token_handler: T,
    user_repository: K,
}

impl<T, K> UserService<T, K>
where
    T: IJwtTokenHandler,
    K: IUserRepository,
{
    pub fn new(jwt_token_handler: T, user_repository: K) -> Self {
        Self {
            jwt_token_handler,
            user_repository,
        }
    }
}

impl<T, K> IUserService for UserService<T, K>
where
    T: IJwtTokenHandler,
    K: IUserRepository,
{
    // TODO: better error handling
    async fn login(&mut self, input: &UserLoginInput) -> AppResult<AuthenticatedUserOutput> {
        let found_user: Option<User> = self.user_repository.find_by_email(&input.email).await?;

        match found_user {
            Some(user) => match user.is_password_valid(&input.password).await {
                true => {
                    let access_token = self.jwt_token_handler.generate_token(&user).await;
                    Ok(AuthenticatedUserOutput { user, access_token })
                }
                false => Err(Error::InternalServerError),
            },
            None => Err(Error::InternalServerError),
        }
    }

    async fn register(&mut self, input: &UserRegisterInput) -> AppResult<AuthenticatedUserOutput> {
        let found_user = self.user_repository.find_by_email(&input.email).await?;

        match found_user {
            Some(_) => Err(Error::InternalServerError),
            None => {
                let new_user = User::new(
                    0,
                    &input.first_name,
                    &input.last_name,
                    &input.email,
                    &input.password,
                )
                .await;

                self.user_repository.create(&new_user).await?;

                let access_token = self.jwt_token_handler.generate_token(&new_user).await;

                Ok(AuthenticatedUserOutput {
                    user: new_user,
                    access_token,
                })
            }
        }
    }
}
