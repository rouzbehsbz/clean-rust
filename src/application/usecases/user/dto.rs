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
