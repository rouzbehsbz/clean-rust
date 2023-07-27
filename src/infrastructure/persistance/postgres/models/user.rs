use crate::{infrastructure::persistance::postgres::PostgresDatabase, application::common::{interfaces::persistance::user_repository::IUserRepository, types::AppResult}, domain::entities::user::User};
use async_trait::async_trait;
use sqlx::query_as;

pub struct UserModel {
    source: PostgresDatabase
}

impl UserModel {
    pub fn new(source: PostgresDatabase) -> Self {
        Self {
            source
        }
    }
}

#[async_trait]
impl IUserRepository for UserModel {
    async fn create(&self, user: &User) -> AppResult<User> {

    }

    async fn update(&self, id: u32, updated_user: &User) -> AppResult<Option<User>> {

    }

    async fn find_by_email(&self, email: &str) -> AppResult<Option<User>> {

    }

    async fn find_by_id(&self, id: u32) -> AppResult<Option<User>> {

    }
}