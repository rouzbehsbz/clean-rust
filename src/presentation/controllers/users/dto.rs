use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UserLoginRequest {
    #[serde(default)]
    #[validate(email(message = "Email address is not valid."))]
    pub email: String,

    #[serde(default)]
    #[validate(length(
        min = 8,
        message = "Password is not valid. It should be at least 8 characters."
    ))]
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UserRegisterRequest {
    #[serde(default)]
    #[validate(length(
        min = 3,
        max = 20,
        message = "First name is not valid. It should be between 3 and 20 characters."
    ))]
    pub first_name: String,

    #[serde(default)]
    #[validate(length(
        min = 3,
        max = 20,
        message = "Last name is not valid. It should be between 3 and 20 characters."
    ))]
    pub last_name: String,

    #[serde(default)]
    #[validate(email(message = "Email address is not valid."))]
    pub email: String,

    #[serde(default)]
    #[validate(length(
        min = 8,
        message = "Password is not valid. It should be at least 8 characters."
    ))]
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdateProfileRequest {
    #[validate(length(
        min = 3,
        max = 20,
        message = "First name is not valid. It should be between 3 and 20 characters."
    ))]
    pub first_name: Option<String>,

    #[validate(length(
        min = 3,
        max = 20,
        message = "Last name is not valid. It should be between 3 and 20 characters."
    ))]
    pub last_name: Option<String>,

    #[validate(email(message = "Email address is not valid."))]
    pub email: Option<String>,

    #[validate(length(
        min = 8,
        message = "Password is not valid. It should be at least 8 characters."
    ))]
    pub password: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateProfileResponse {
    pub message: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthenticatedUserResponse {
    pub id: u32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub access_token: String,
}
