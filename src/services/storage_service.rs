use crate::db::{get_schema, DBConnection};
use crate::models::Storage;
use anyhow::Result;

pub async fn get_storage(conn: &DBConnection) -> Result<Storage> {
    let schema = get_schema();
    let stmt = conn
        .prepare_cached(
            format!(
                "SELECT level, level_timestamp, lambda_repository_creator, \
        create_restrictions_creator_address, currency \
        FROM \"{}\".\"storage_live\";",
                schema
            )
            .as_str(),
        )
        .await?;
    let rows = conn.query(&stmt, &[]).await?;
    let first = &rows[0];
    Ok(Storage::from_row(first))
}

pub async fn get_storages(conn: &DBConnection) -> Result<Vec<Storage>> {
    let schema = get_schema();
    let stmt = conn
        .prepare_cached(
            format!(
                "SELECT level, level_timestamp, lambda_repository_creator, \
        create_restrictions_creator_address, currency \
        FROM \"{}\".\"storage_ordered\";",
                schema
            )
            .as_str(),
        )
        .await?;
    let rows = conn.query(&stmt, &[]).await?;
    let storages = rows.iter().map(Storage::from_row).collect();
    Ok(storages)
}
