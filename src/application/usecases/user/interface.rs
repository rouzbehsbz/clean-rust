use crate::common::types::AppResult;

use super::dto::{AuthenticatedUserOutput, UserLoginInput, UserRegisterInput};

pub trait IUserService {
    async fn register(&mut self, input: &UserRegisterInput) -> AppResult<AuthenticatedUserOutput>;
    async fn login(&mut self, input: &UserLoginInput) -> AppResult<AuthenticatedUserOutput>;
}
