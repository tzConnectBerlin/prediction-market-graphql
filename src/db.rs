use anyhow::Result;
use std::env;

use sqlx::{pool::PoolOptions, postgres::PgPoolOptions, Pool, Postgres};

pub fn get_schema() -> String {
    env::var("SCHEMA").expect("SCHEMA must be set")
}

pub fn get_pool_options() -> PoolOptions<Postgres> {
    let max_listener_pool = env::var("MAX_POOL_CONNECTIONS")
        .expect("Max lister pool not provided")
        .as_str()
        .parse::<u32>()
        .unwrap();

    PgPoolOptions::new().max_connections(max_listener_pool)
}

pub async fn get_db_pool() -> Result<Pool<Postgres>> {
    let options = get_pool_options();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL not provided");
    let pool = options.connect(&db_url).await?;
    Ok(pool)
}
