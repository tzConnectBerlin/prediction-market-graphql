use crate::db::{get_schema, DBConnection};
use crate::models::SupplyMap;
use crate::utils::i32_to_numeric;
use anyhow::Result;

pub async fn get_supply_map_by_token(conn: &DBConnection, token_id: i32) -> Result<SupplyMap> {
    let schema = get_schema();
    let stmt = conn
        .prepare_cached(
            format!(
        "SELECT level, level_timestamp, tokens_total_supply, tokens_in_reserve, idx_tokens_nat_3 \
        FROM \"{}\".\"storage.supply_map_live\" WHERE idx_tokens_nat_3 = $1;",
        schema,
      )
            .as_str(),
        )
        .await?;
    let rows = conn.query(&stmt, &[&i32_to_numeric(&token_id)]).await?;
    let first = &rows[0];
    Ok(SupplyMap::from_row(first))
}

pub async fn get_all_supply_maps(conn: &DBConnection) -> Result<Vec<SupplyMap>> {
    let schema = get_schema();
    let stmt = conn
        .prepare_cached(
            format!(
        "SELECT level, level_timestamp, tokens_total_supply, tokens_in_reserve, idx_tokens_nat_3 \
        FROM \"{}\".\"storage.supply_map_ordered\";",
        schema
      )
            .as_str(),
        )
        .await?;
    let rows = conn.query(&stmt, &[]).await?;
    let supply_maps = rows.iter().map(SupplyMap::from_row).collect();
    Ok(supply_maps)
}
