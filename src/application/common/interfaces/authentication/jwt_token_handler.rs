use crate::{domain::entities::user::User, common::types::AppResult};
use async_trait::async_trait;
use serde::{Serialize, Deserialize};

//TODO: add exp time to it
#[derive(Debug, Serialize, Deserialize)]
pub struct JwtPayload {
    pub user_id: u32,
    pub exp: u64
}

#[async_trait]
pub trait IJwtTokenHandler: Send + Sync {
    async fn generate_token(&self, user: &User) -> String;
    fn decode_token(&self, token: &str) -> AppResult<JwtPayload>;
}
