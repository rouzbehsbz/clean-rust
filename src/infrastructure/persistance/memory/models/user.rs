use crate::{
    application::common::interfaces::persistance::user_repository::IUserRepository,
    common::types::AppResult, domain::entities::user::User,
    infrastructure::persistance::memory::Memory,
};

pub struct UserModel {
    source: Memory<String, User>,
}

impl UserModel {
    pub fn new() -> Self {
        Self {
            source: Memory::new(),
        }
    }
}

impl IUserRepository for UserModel {
    // TODO: implement more effecient way
    async fn create(&mut self, user: &User) -> AppResult<()> {
        let owned_user = User::new(
            user.id,
            &user.first_name,
            &user.last_name,
            &user.email,
            &user.password,
        )
        .await;

        self.source.add(owned_user.email.to_string(), owned_user);

        Ok(())
    }

    // TODO: implement more effecient way
    async fn find_by_email(&mut self, email: &str) -> AppResult<Option<User>> {
        let found_user = self.source.get(&email.to_string());

        match found_user {
            Some(user) => Ok(Some(
                User::new(
                    user.id,
                    &user.first_name,
                    &user.last_name,
                    &user.email,
                    &user.password,
                )
                .await,
            )),
            None => Ok(None),
        }
    }
}
