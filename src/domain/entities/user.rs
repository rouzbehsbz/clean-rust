use bcrypt::{hash, verify};
use serde::{Deserialize, Serialize};
use tokio::task;
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
}

impl User {
    pub async fn new(first_name: &str, last_name: &str, email: &str, password: &str) -> Self {
        let mut user = Self {
            id: 0,
            first_name: first_name.to_string(),
            last_name: last_name.to_string(),
            email: email.to_string(),
            password: password.to_string(),
        };

        user.hash_password().await;

        user
    }

    pub fn set_id(&mut self, id: i32) {
        self.id = id;
    }

    //TODO: better error handling
    pub async fn hash_password(&mut self) {
        let password = self.password.clone();

        let hashed_password = task::spawn_blocking(move || hash(password, 12).unwrap())
            .await
            .unwrap();

        self.password = hashed_password;
    }

    pub async fn is_password_valid(&self, password: &str) -> bool {
        let password = password.to_string();
        let hashed_password = self.password.clone();

        let result =
            task::spawn_blocking(move || verify(password, &hashed_password).unwrap_or(false))
                .await
                .unwrap_or(false);

        result
    }
}
