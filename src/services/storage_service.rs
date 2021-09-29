use crate::db::get_schema;
use crate::models::Storage;
use anyhow::Result;
use sqlx::{Pool, Postgres};

pub async fn get_storage(conn: &Pool<Postgres>) -> Result<Storage> {
    let schema = get_schema();
    let first = sqlx::query(
        format!(
            "SELECT level, level_timestamp, lambda_repository_creator, \
        create_restrictions_creator_address, currency \
        FROM \"{}\".\"storage_live\";",
            schema
        )
        .as_str(),
    )
    .fetch_one(conn)
    .await?;
    Ok(Storage::from_row(first))
}

pub async fn get_storages(conn: &Pool<Postgres>) -> Result<Vec<Storage>> {
    let schema = get_schema();
    let storages = sqlx::query(
        format!(
            "SELECT level, level_timestamp, lambda_repository_creator, \
        create_restrictions_creator_address, currency \
        FROM \"{}\".\"storage_ordered\";",
            schema
        )
        .as_str(),
    )
    .map(Storage::from_row)
    .fetch_all(conn)
    .await?;
    Ok(storages)
}
