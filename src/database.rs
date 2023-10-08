use sqlx::postgres::{PgPool, PgPoolOptions};
use sqlx::error::Error;
use std::env;

pub async fn establish_db_pool() -> Result<PgPool, Error> {
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL is not set in the environment variables");

    let pool = PgPoolOptions::new()
        .max_connections(10) 
        .connect(&database_url)
        .await?;

    Ok(pool)
}
