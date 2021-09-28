use crate::db::get_schema;
use crate::models::SupplyMap;
use anyhow::Result;
use sqlx::{Pool, Postgres};

pub async fn get_supply_map_by_token(
    conn: &Pool<Postgres>,
    token_id: i32,
) -> Result<Option<SupplyMap>> {
    let schema = get_schema();
    let first = sqlx::query(
        format!(
        "SELECT level, level_timestamp, tokens_total_supply, tokens_in_reserve, idx_tokens_nat_3 \
        FROM \"{}\".\"storage.supply_map_live\" WHERE idx_tokens_nat_3 = $1;",
        schema,
      )
        .as_str(),
    )
    .bind(token_id)
    .fetch_optional(conn)
    .await?;
    match first {
        Some(x) => Ok(Some(SupplyMap::from_row(x))),
        _ => Ok(None),
    }
}

pub async fn get_all_supply_maps(conn: &Pool<Postgres>) -> Result<Vec<SupplyMap>> {
    let schema = get_schema();
    let supply_maps = sqlx::query(
        format!(
        "SELECT level, level_timestamp, tokens_total_supply, tokens_in_reserve, idx_tokens_nat_3 \
        FROM \"{}\".\"storage.supply_map_ordered\";",
        schema
      )
        .as_str(),
    )
    .map(SupplyMap::from_row)
    .fetch_all(conn)
    .await?;

    Ok(supply_maps)
}
