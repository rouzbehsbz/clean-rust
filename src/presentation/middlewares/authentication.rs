use std::future::{Ready, ready};

use actix_web::{FromRequest, HttpRequest, dev::Payload, http, HttpMessage};

use crate::{application::common::{errors::Error, types::AppResult}, infrastructure::authentication::jwt_token_handler::JwtTokenHandler, container::Container, application::common::interfaces::authentication::jwt_token_handler::{IJwtTokenHandler, JwtPayload}};

pub struct AuthenticationMiddleware;

impl FromRequest for AuthenticationMiddleware {
    type Error = Error;
    type Future = Ready<AppResult<Self>>;

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        let access_token = req.headers().get(http::header::AUTHORIZATION);
        let jwt_token_handler = JwtTokenHandler::new();

        //TODO: better error handling
        match access_token {
            Some(token) => {
                let decoded_token = jwt_token_handler.decode_token(token.to_str().unwrap());

                match decoded_token {
                    Ok(value) => {
                        req.extensions_mut().insert::<JwtPayload>(value);
                    
                        ready(Ok(AuthenticationMiddleware))
                    },
                    Err(_) => {
                        ready(Err(Error::AuthorizationFailed(format!("You don't have access. please login again."))))
                    }
                }
            },
            None => {
                ready(Err(Error::AuthorizationFailed(format!("You don't have access. please login again."))))
            }
        }
    }
}