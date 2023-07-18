use crate::domain::entities::user::User;

pub trait IJwtTokenHandler {
    async fn generate_token(&self, user: &User) -> String;
}
