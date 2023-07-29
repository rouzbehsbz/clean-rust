#[cfg(test)]
mod tests {
    use crate::{application::{usecases::user::{service::UserService, interface::IUserService, dto::{UserRegisterInput, UserLoginInput, UpdateUserPofileInput, GetProfileInput}}, common::types::AppResult}, infrastructure::{authentication::jwt_token_handler::JwtTokenHandler, persistance::memory::repositories::user::UserRepository}};

    #[tokio::test]
    async fn is_new_user_registered() -> AppResult<()> {
        let jwt_token_handler = JwtTokenHandler::new();
        let user_repository = UserRepository::new();
        
        let user_service = UserService::new(
            jwt_token_handler,
            user_repository
        );

        let result = user_service.register(&UserRegisterInput {
            email: "rouzbehsbz@gmail.com".to_string(),
            first_name: "rouzbeh".to_string(),
            last_name: "sbz".to_string(),
            password: "test@123".to_string()
        }).await?;

        assert_eq!(result.user.id, 1);

        Ok(())
    }

    #[tokio::test]
    #[should_panic]
    async fn should_panic_with_duplicate_user_register() {
        let jwt_token_handler = JwtTokenHandler::new();
        let user_repository = UserRepository::new();
        
        let user_service = UserService::new(
            jwt_token_handler,
            user_repository
        );

        user_service.register(&UserRegisterInput {
            email: "rouzbehsbz@gmail.com".to_string(),
            first_name: "rouzbeh".to_string(),
            last_name: "sbz".to_string(),
            password: "test@123".to_string()
        }).await
        .unwrap();

        user_service.register(&UserRegisterInput {
            email: "rouzbehsbz@gmail.com".to_string(),
            first_name: "satoshi".to_string(),
            last_name: "nakamoto".to_string(),
            password: "satoshi#123".to_string()
        }).await
        .unwrap();
    }

    #[tokio::test]
    async fn can_user_login() -> AppResult<()> {
        let jwt_token_handler = JwtTokenHandler::new();
        let user_repository = UserRepository::new();
        
        let user_service = UserService::new(
            jwt_token_handler,
            user_repository
        );

        let email = "rouzbehsbz@gmail.com";
        let password = "test@123";

        let registered_user = user_service.register(&UserRegisterInput {
            email: email.to_string(),
            first_name: "rouzbeh".to_string(),
            last_name: "sbz".to_string(),
            password: password.to_string()
        }).await?;

        let logged_in_user = user_service.login(&UserLoginInput {
            email: email.to_string(),
            password: password.to_string()
        })
        .await?;

        assert_eq!(logged_in_user.user.id, registered_user.user.id);

        Ok(())
    }

    #[tokio::test]
    #[should_panic]
    async fn should_panic_with_non_registered_user_trying_to_login() {
        let jwt_token_handler = JwtTokenHandler::new();
        let user_repository = UserRepository::new();
        
        let user_service = UserService::new(
            jwt_token_handler,
            user_repository
        );

        user_service.login(&UserLoginInput {
            email: "rouzbehsbz@gmail.com".to_string(),
            password: "test@123".to_string()
        })
        .await
        .unwrap();
    }

    #[tokio::test]
    async fn can_user_update_its_profile() -> AppResult<()> {
        let jwt_token_handler = JwtTokenHandler::new();
        let user_repository = UserRepository::new();
        
        let user_service = UserService::new(
            jwt_token_handler,
            user_repository
        );

        let registered_user = user_service.register(&UserRegisterInput {
            email: "rouzbehsbz@gmail.com".to_string(),
            first_name: "rouzbeh".to_string(),
            last_name: "sbz".to_string(),
            password: "test@123".to_string()
        }).await?;

        let result = user_service.update_profile(&UpdateUserPofileInput {
            id: registered_user.user.id,
            email: Some("satoshinakamoto@bitcoin.com".to_string()),
            first_name: Some("satoshi".to_string()),
            last_name: None,
            password: None
        })
        .await?;

        assert_eq!(result, ());

        Ok(())
    }

    #[tokio::test]
    #[should_panic]
    async fn should_panic_with_non_existed_user_trying_to_update_its_profile() {
        let jwt_token_handler = JwtTokenHandler::new();
        let user_repository = UserRepository::new();
        
        let user_service = UserService::new(
            jwt_token_handler,
            user_repository
        );

        user_service.update_profile(&UpdateUserPofileInput {
            id: 1,
            email: Some("satoshinakamoto@bitcoin.com".to_string()),
            first_name: Some("satoshi".to_string()),
            last_name: None,
            password: None
        })
        .await
        .unwrap();
    }

    #[tokio::test]
    async fn can_get_specific_user_profile() -> AppResult<()> {
        let jwt_token_handler = JwtTokenHandler::new();
        let user_repository = UserRepository::new();
        
        let user_service = UserService::new(
            jwt_token_handler,
            user_repository
        );

        let registered_user = user_service.register(&UserRegisterInput {
            email: "rouzbehsbz@gmail.com".to_string(),
            first_name: "rouzbeh".to_string(),
            last_name: "sbz".to_string(),
            password: "test@123".to_string()
        }).await?;

        let result = user_service.get_profile(&GetProfileInput {
            id: 1
        })
        .await?;

        assert_eq!(registered_user.user.email, result.email);
        assert_eq!(registered_user.user.first_name, result.first_name);
        assert_eq!(registered_user.user.last_name, result.last_name);

        Ok(())
    }

    #[tokio::test]
    #[should_panic]
    async fn should_panic_when_user_is_not_exists_for_getting_its_profile() {
        let jwt_token_handler = JwtTokenHandler::new();
        let user_repository = UserRepository::new();
        
        let user_service = UserService::new(
            jwt_token_handler,
            user_repository
        );

        user_service.get_profile(&GetProfileInput {
            id: 1
        })
        .await
        .unwrap();
    }
}   