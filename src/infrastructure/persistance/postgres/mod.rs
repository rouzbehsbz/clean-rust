use sqlx::{Pool, postgres, postgres::PgPoolOptions, Postgres};

use crate::config::PostgresConfig;

pub mod repositories;

//TODO: is cloning ok for passing to multiple instances ?
#[derive(Clone)]
pub struct PostgresDatabase {
    pool: Pool<Postgres>
}

//TODO: implement better error handling
impl PostgresDatabase {
    pub async fn new() -> Self {
        let connection_string = PostgresConfig::connection_string();

        let pool = PgPoolOptions::new()
            .max_connections(4)
            .connect(&connection_string)
            .await
            .unwrap();
        
        Self {
            pool
        }
    }
}