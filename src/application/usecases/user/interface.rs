use async_trait::async_trait;
use crate::common::types::AppResult;

use super::dto::{AuthenticatedUserOutput, UserLoginInput, UserRegisterInput};

#[async_trait]
pub trait IUserService: Send + Sync {
    async fn register(&self, input: &UserRegisterInput) -> AppResult<AuthenticatedUserOutput>;
    async fn login(&self, input: &UserLoginInput) -> AppResult<AuthenticatedUserOutput>;
}
