use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::error::Error;

pub struct DbPostgres {
    pub pool: Pool<Postgres>,
}

impl DbPostgres {
    pub async fn new(url: &str) -> Result<Self, Box<dyn Error>> {
        let pool = PgPoolOptions::new().max_connections(5).connect(url).await?;

        // Verify database connection
        sqlx::query("SELECT 1").execute(&pool).await?;

        Ok(Self { pool })
    }
}
