use crate::db::get_schema;
use crate::models::SupplyMap;
use anyhow::Result;
use sqlx::{Pool, Postgres};

pub async fn get_supply_maps(
    conn: &Pool<Postgres>,
    token_ids: Option<Vec<i32>>,
) -> Result<Vec<SupplyMap>> {
    let schema = get_schema();
    let sql = match token_ids {
        Some(tokens) => format!(
          "SELECT level, level_timestamp, tokens_total_supply, tokens_in_reserve, idx_tokens_nat_3 \
          FROM \"{}\".\"storage.supply_map_live\" WHERE idx_tokens_nat_3 in ({});",
          schema,
          tokens.iter()
                  .map(|i| i.to_string())
                  .collect::<Vec<String>>()
                  .join(", ")
        ),
        _ => format!(
        "SELECT level, level_timestamp, tokens_total_supply, tokens_in_reserve, idx_tokens_nat_3 \
        FROM \"{}\".\"storage.supply_map_ordered\";",
        schema
      ),
    };
    let rows = sqlx::query(sql.as_str())
        .map(SupplyMap::from_row)
        .fetch_all(conn)
        .await?;
    Ok(rows)
}
