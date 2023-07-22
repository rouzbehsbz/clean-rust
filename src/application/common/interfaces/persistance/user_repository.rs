use async_trait::async_trait;
use crate::{common::types::AppResult, domain::entities::user::User};

#[async_trait]
pub trait IUserRepository: Send + Sync {
    async fn find_by_email(&self, email: &str) -> AppResult<Option<User>>;
    async fn create(&self, user: &User) -> AppResult<()>;
}
