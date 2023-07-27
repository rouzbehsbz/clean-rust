use super::{
    dto::{AuthenticatedUserOutput, UpdateUserPofileInput, UserLoginInput, UserRegisterInput, GetProfileInput, GetProfileOutput},
    interface::IUserService,
};
use crate::{
    application::common::interfaces::{
        authentication::jwt_token_handler::IJwtTokenHandler,
        persistance::user_repository::IUserRepository,
    },
    application::common::{errors::Error, types::AppResult},
    domain::entities::user::User,
};
use async_trait::async_trait;

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

#[async_trait]
impl<T, K> IUserService for UserService<T, K>
where
    T: IJwtTokenHandler,
    K: IUserRepository,
{
    async fn login(&self, input: &UserLoginInput) -> AppResult<AuthenticatedUserOutput> {
        let found_user: Option<User> = self.user_repository.find_by_email(&input.email).await?;

        match found_user {
            Some(user) => match user.is_password_valid(&input.password).await {
                true => {
                    let access_token = self.jwt_token_handler.generate_token(&user).await;
                    Ok(AuthenticatedUserOutput { user, access_token })
                }
                false => Err(Error::EntityValidationFailed(format!(
                    "Email or password is incorrect."
                ))),
            },
            None => Err(Error::EntityValidationFailed(format!(
                "Email or password is incorrect."
            ))),
        }
    }

    async fn register(&self, input: &UserRegisterInput) -> AppResult<AuthenticatedUserOutput> {
        let found_user = self.user_repository.find_by_email(&input.email).await?;

        match found_user {
            Some(_) => Err(Error::EntityExists(format!(
                "This email has already been taken."
            ))),
            None => {
                let new_user = User::new(
                    &input.first_name,
                    &input.last_name,
                    &input.email,
                    &input.password,
                )
                .await;

                let created_user = self.user_repository.create(&new_user).await?;
                let access_token = self.jwt_token_handler.generate_token(&created_user).await;

                Ok(AuthenticatedUserOutput {
                    user: created_user,
                    access_token,
                })
            }
        }
    }

    async fn update_profile(&self, input: &UpdateUserPofileInput) -> AppResult<()> {
        let found_user = self.user_repository.find_by_id(input.id).await?;

        match found_user {
            Some(mut user) => {
                if let Some(email) = &input.email {
                    user.email = email.to_owned();
                }
                if let Some(first_name) = &input.first_name {
                    user.first_name = first_name.to_owned();
                }
                if let Some(last_name) = &input.last_name {
                    user.last_name = last_name.to_owned();
                }
                if let Some(password) = &input.password {
                    user.password = password.to_owned();
                    user.hash_password().await;
                }

                self.user_repository.update(input.id, &user).await?;

                Ok(())
            }
            None => Err(Error::EntityNotFound(format!("User does not exists."))),
        }
    }

    async fn get_profile(&self, input: &GetProfileInput) -> AppResult<GetProfileOutput> {
        let found_user = self.user_repository.find_by_id(input.id).await?;

        match found_user {
            Some(user) => {
                Ok(GetProfileOutput { 
                    first_name: user.first_name,
                    last_name: user.last_name,
                    email: user.email 
                })    
            },
            None => Err(Error::EntityNotFound(format!("User does not exists.")))
        }
    }
}
