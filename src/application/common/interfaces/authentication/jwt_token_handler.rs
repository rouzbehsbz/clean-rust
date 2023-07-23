use crate::domain::entities::user::User;
use async_trait::async_trait;

#[async_trait]
pub trait IJwtTokenHandler: Send + Sync {
    async fn generate_token(&self, user: &User) -> String;
}
