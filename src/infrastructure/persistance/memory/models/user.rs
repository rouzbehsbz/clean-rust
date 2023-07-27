use crate::{
    application::common::interfaces::persistance::user_repository::IUserRepository,
    application::common::types::AppResult, domain::entities::user::User,
    infrastructure::persistance::memory::Memory,
};
use async_trait::async_trait;
use tokio::sync::Mutex;

pub struct UserModel {
    id_counter: Mutex<u32>,
    source: Memory<u32, User>,
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
    async fn create(&self, user: &User) -> AppResult<User> {
        let mut counter = self.id_counter.lock().await;
        let mut owned_user = user.clone();

        *counter += 1;

        owned_user.set_id(*counter);

        self.source.add(owned_user.id, owned_user.to_owned()).await;

        Ok(owned_user)
    }

    async fn find_by_email(&self, email: &str) -> AppResult<Option<User>> {
        let all_users = self.source.get_all().await;

        for user in all_users {
            if user.email == email {
                return Ok(Some(user));
            }
        }

        Ok(None)
    }

    async fn find_by_id(&self, id: u32) -> AppResult<Option<User>> {
        let found_user = self.source.get(&id).await;

        match found_user {
            Some(user) => Ok(Some(user)),
            None => Ok(None),
        }
    }

    async fn update(&self, id: u32, updated_user: &User) -> AppResult<Option<User>> {
        let found_user = self.source.get(&id).await;

        match found_user {
            Some(_) => {
                self.source.remove(id).await;
                self.source.add(id, updated_user.clone()).await;

                Ok(Some(updated_user.clone()))
            }
            None => Ok(None),
        }
    }
}
