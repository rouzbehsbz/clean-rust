use crate::common::types::AppResult;
use async_trait::async_trait;

use super::dto::{
    AuthenticatedUserOutput, UpdateUserPofileInput, UserLoginInput, UserRegisterInput, GetProfileInput, GetProfileOutput,
};

#[async_trait]
pub trait IUserService: Send + Sync {
    async fn register(&self, input: &UserRegisterInput) -> AppResult<AuthenticatedUserOutput>;
    async fn login(&self, input: &UserLoginInput) -> AppResult<AuthenticatedUserOutput>;
    async fn update_profile(&self, input: &UpdateUserPofileInput) -> AppResult<()>;
    async fn get_profile(&self, input: &GetProfileInput) -> AppResult<GetProfileOutput>;
}
