use crate::{
    application::common::interfaces::persistance::user_repository::IUserRepository,
    common::types::AppResult, domain::entities::user::User,
    infrastructure::persistance::memory::Memory,
};
use async_trait::async_trait;
use tokio::sync::Mutex;

pub struct UserModel {
    id_counter: Mutex<u32>,
    source: Memory<String, User>,
}

impl UserModel {
    pub fn new() -> Self {
        Self {
            id_counter: Mutex::new(0),
            source: Memory::new(),
        }
    }
}

#[async_trait]
impl IUserRepository for UserModel {
    // TODO: implement more effecient way
    async fn create(&self, user: &User) -> AppResult<User> {
        let mut counter = self.id_counter.lock().await;
        let mut owned_user = user.clone();

        *counter += 1;

        owned_user.set_id(*counter);

        self.source
            .add(owned_user.email.to_string(), owned_user.to_owned())
            .await;

        Ok(owned_user)
    }

    // TODO: implement more effecient way
    async fn find_by_email(&self, email: &str) -> AppResult<Option<User>> {
        let found_user = self.source.get(&email.to_string()).await;

        match found_user {
            Some(user) => Ok(Some(user)),
            None => Ok(None),
        }
    }
}
