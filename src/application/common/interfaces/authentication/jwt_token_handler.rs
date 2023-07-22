use async_trait::async_trait;
use crate::domain::entities::user::User;

#[async_trait]
pub trait IJwtTokenHandler: Send + Sync {
    async fn generate_token(&self, user: &User) -> String;
}
