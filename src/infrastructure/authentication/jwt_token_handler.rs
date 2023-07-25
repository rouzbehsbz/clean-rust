use crate::{
    application::common::interfaces::authentication::jwt_token_handler::{IJwtTokenHandler, JwtPayload},
    domain::entities::user::User, common::{types::AppResult, errors::Error, config::AuthenticationConfig},
};
use async_trait::async_trait;
use jsonwebtoken::{encode, decode, DecodingKey, EncodingKey, Header, Validation};

//TODO: handle this task async
//TODO: handle error here
pub struct JwtTokenHandler {
    secret: String,
}

impl JwtTokenHandler {
    pub fn new() -> Self {
        Self {
            secret: AuthenticationConfig::jwt_secret()
        }
    }
}

#[async_trait]
impl IJwtTokenHandler for JwtTokenHandler {
    async fn generate_token(&self, user: &User) -> String {
        let payload = JwtPayload { user_id: user.id };

        let token = encode(
            &Header::default(),
            &payload,
            &EncodingKey::from_secret(self.secret.as_ref()),
        )
        .unwrap();

        token
    }

    fn decode_token(&self, token: &str) -> AppResult<JwtPayload> {
        let decoded_token = decode::<JwtPayload>(
            token,
            &DecodingKey::from_secret(self.secret.as_ref()),
            &Validation::default()
        );

        match decoded_token {
            Ok(value) => {
                Ok(value.claims)
            },
            Err(_) => Err(Error::AuthorizationFailed(format!("Your access token is not valid. please login again.")))
        }
    }
}
