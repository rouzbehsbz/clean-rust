use sqlx::{Pool, postgres, postgres::PgPoolOptions, Postgres};

pub mod models;

pub struct PostgresDatabase {
    connection_string: String,
    pool: Pool<Postgres>
}

//TODO: implement better error handling
impl PostgresDatabase {
    pub async fn new(connection_string: &str) -> Self {
        let pool = PgPoolOptions::new()
            .max_connections(4)
            .connect(connection_string)
            .await
            .unwrap();
        
        Self {
            connection_string: connection_string.to_string(),
            pool
        }
    }
}