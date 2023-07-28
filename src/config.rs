pub struct AuthenticationConfig;
pub struct PostgresConfig;

impl AuthenticationConfig {
    pub fn jwt_secret() -> String {
        "myveryimportantsecret".to_string()
    }
}

impl PostgresConfig {
    pub fn connection_string() -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            Self::username(),
            Self::password(),
            Self::host(),
            Self::port(),
            Self::db_name()
        )
    }

    fn host() -> String {
        "localhost".to_string()
    }

    fn port() -> u16 {
        5432
    }

    fn db_name() -> String {
        "clean-rust".to_string()
    }

    fn username() -> String {
        "postgres".to_string()
    }

    fn password() -> String {
        "test@123".to_string()
    }
}