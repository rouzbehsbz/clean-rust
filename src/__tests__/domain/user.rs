#[cfg(test)]
mod tests {
    use crate::domain::entities::user::User;

    #[tokio::test]
    async fn is_hashed_password_correct() {
        let password = "test@123";

        let user = User::new(
            "rouzbeh",
            "sbz",
            "rouzbehsbz@gmail.com",
            password
        ).await;

        let result = user.is_password_valid(password).await;

        assert_eq!(result, true);
    }
}