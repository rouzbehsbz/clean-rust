use std::env;

pub struct HTTPServerConfig;
pub struct AuthenticationConfig;
pub struct PostgresConfig;

impl HTTPServerConfig {
    pub fn address() -> String {
        format!(
            "{}:{}",
            Self::host(),
            Self::port()
        )
    }

    fn host() -> String {
        let host = env::var("HTTP_HOST")
            .unwrap_or("0.0.0.0".to_string());

        host
    }

    fn port() -> u16 {
        let port = env::var("HTTP_PORT")
            .unwrap_or("3000".to_string())
            .parse::<u16>()
            .expect("HTTP port must be an integer.");

        port
    }
}

impl AuthenticationConfig {
    pub fn jwt_secret() -> String {
        let jwt_secret = env::var("JWT_SECRET")
            .unwrap_or("myveryimportantsecret".to_string());

        jwt_secret
    }
}

impl PostgresConfig {
    pub fn connection_string() -> String {
        let connection_string = env::var("DATABASE_URL")
            .unwrap_or("postgres://postgres:@localhost:5432/clean-rust".to_string());
                
        connection_string
    }
}