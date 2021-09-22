use crate::db::{get_schema, Connection};
use crate::models::SupplyMap;
use anyhow::Result;

use bigdecimal::{BigDecimal, FromPrimitive};
use pg_bigdecimal::PgNumeric;

pub fn to_numeric(n: i32) -> PgNumeric {
    let dec = BigDecimal::from_i32(n).unwrap();
    PgNumeric { n: Some(dec) }
}

pub async fn get_supply_map_by_token(conn: &Connection, token_id: i32) -> Result<SupplyMap> {
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
    let rows = conn.query(&stmt, &[&to_numeric(token_id)]).await?;
    let first = &rows[0];
    Ok(SupplyMap::from_row(first))
}

pub async fn get_all_supply_maps(conn: &Connection) -> Result<Vec<SupplyMap>> {
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
