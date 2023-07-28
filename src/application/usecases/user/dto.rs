use serde::{Deserialize, Serialize};

use crate::domain::entities::user::User;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserRegisterInput {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserLoginInput {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthenticatedUserOutput {
    pub user: User,
    pub access_token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateUserPofileInput {
    pub id: i32,
    pub email: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub password: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetProfileInput {
    pub id: i32
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetProfileOutput {
    pub first_name: String,
    pub last_name: String,
    pub email: String
}
