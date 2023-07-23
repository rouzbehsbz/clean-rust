use crate::{
    application::common::interfaces::authentication::jwt_token_handler::IJwtTokenHandler,
    domain::entities::user::User,
};
use async_trait::async_trait;
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};

//TODO: add exp time to it
#[derive(Debug, Serialize, Deserialize)]
struct JwtPayload {
    user_id: u32,
}

pub struct JwtTokenHandler {
    secret: String,
}

impl JwtTokenHandler {
    pub fn new(secret: &str) -> Self {
        Self {
            secret: secret.to_string(),
        }
    }
}

#[async_trait]
impl IJwtTokenHandler for JwtTokenHandler {
    async fn generate_token(&self, user: &User) -> String {
        let payload = JwtPayload { user_id: user.id };

        //TODO: handle error here
        let token = encode(
            &Header::default(),
            &payload,
            &EncodingKey::from_secret(self.secret.as_ref()),
        )
        .unwrap();

        token
    }
}
