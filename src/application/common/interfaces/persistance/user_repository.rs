use crate::{common::types::AppResult, domain::entities::user::User};

pub trait IUserRepository {
    async fn find_by_email(&mut self, email: &str) -> AppResult<Option<User>>;
    async fn create(&mut self, user: &User) -> AppResult<()>;
}
