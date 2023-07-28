use crate::{infrastructure::persistance::postgres::PostgresDatabase, application::common::{interfaces::persistance::user_repository::IUserRepository, types::AppResult}, domain::entities::user::User};
use async_trait::async_trait;
use sqlx;

pub struct UserRepository {
    source: PostgresDatabase
}

impl UserRepository {
    pub fn new(source: PostgresDatabase) -> Self {
        Self {
            source
        }
    }
}

//TODO: implement database errors handling
#[async_trait]
impl IUserRepository for UserRepository {
    async fn create(&self, user: &User) -> AppResult<User> {
        let created_user = sqlx::query_as!(
            User,
            r#"
                INSERT INTO users (first_name, last_name, email, password)
                VALUES ($1, $2, $3, $4)
                RETURNING id, first_name, last_name, email, password
            "#,
            &user.first_name,
            &user.last_name,
            &user.email,
            &user.password,
        )
        .fetch_one(&self.source.pool)
        .await
        .unwrap();

        Ok(created_user)
    }

    async fn update(&self, id: i32, updated_user: &User) -> AppResult<Option<User>> {
        let found_user = self.find_by_id(id).await?;

        match found_user {
            Some(_) => {
                let updated_user = sqlx::query_as!(
                    User,
                    r#"
                        UPDATE users SET
                        first_name = $1,
                        last_name = $2,
                        email = $3,
                        password = $4
                        WHERE id = $5
                        RETURNING id, first_name, last_name, email, password
                    "#,
                    &updated_user.first_name,
                    &updated_user.last_name,
                    &updated_user.email,
                    &updated_user.password,
                    id
                )
                .fetch_one(&self.source.pool)
                .await
                .unwrap();

                Ok(Some(updated_user))
            }
            None => Ok(None)
        }
    }

    async fn find_by_email(&self, email: &str) -> AppResult<Option<User>> {
        let found_user = sqlx::query_as!(
            User,
            r#"
                SELECT * FROM users WHERE email = $1
            "#,
            email
        )
        .fetch_optional(&self.source.pool)
        .await
        .unwrap();

        Ok(found_user)
    }

    async fn find_by_id(&self, id: i32) -> AppResult<Option<User>> {
        let found_user = sqlx::query_as!(
            User,
            r#"
                SELECT * FROM users WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.source.pool)
        .await
        .unwrap();

        Ok(found_user)
    }
}