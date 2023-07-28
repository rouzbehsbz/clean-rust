use crate::{application::common::types::AppResult, domain::entities::user::User};
use async_trait::async_trait;

#[async_trait]
pub trait IUserRepository: Send + Sync {
    async fn create(&self, user: &User) -> AppResult<User>;
    async fn update(&self, id: i32, updated_user: &User) -> AppResult<Option<User>>;
    async fn find_by_email(&self, email: &str) -> AppResult<Option<User>>;
    async fn find_by_id(&self, id: i32) -> AppResult<Option<User>>;
}
