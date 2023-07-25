pub struct AuthenticationConfig;

impl AuthenticationConfig {
    pub fn jwt_secret() -> String {
        "myveryimportantsecret".to_string()
    }
}