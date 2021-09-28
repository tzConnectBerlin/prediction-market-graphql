use crate::db::get_schema;
use crate::models::LiquidityProviderMap;
use anyhow::Result;
use sqlx::{Pool, Postgres};

/**
* TODO: Find a better way to do this
*/
pub async fn get_liquidity_providers(
    conn: &Pool<Postgres>,
    market_ids: Option<Vec<i32>>,
    providers: Option<Vec<String>>,
) -> Result<Vec<LiquidityProviderMap>> {
    let schema = get_schema();
    let market_clause = match market_ids {
        Some(ref x) => {
            let markts = x
                .iter()
                .map(|i| i.to_string())
                .collect::<Vec<String>>()
                .join(", ");
            format!("WHERE b.idx_markets_market_id in ({})", markts)
        }
        None => "".to_string(),
    };
    let lp_provider_clause = match providers {
        Some(ref x) => {
            let lp_providers = x
                .iter()
                .map(|i| format!("'{}'", i))
                .collect::<Vec<String>>()
                .join(", ");
            let where_clause = if !market_clause.is_empty() {
                ""
            } else {
                "WHERE"
            };
            format!(
                "{} b.idx_markets_originator in ({})",
                where_clause, lp_providers
            )
        }
        None => "".to_string(),
    };
    let connector = match market_ids {
        Some(_) => match providers {
            Some(_) => " AND ",
            None => "",
        },
        None => "",
    };

    let liquidity_providers = sqlx::query(
        format!(
            "SELECT a.level, a.level_timestamp, a.bet_quantity, a.bet_predicted_probability,\
                  b.idx_markets_market_id, b.idx_markets_originator FROM \
                  {}.\"storage.liquidity_provider_map.bet_ordered\" a \
                  inner join {}.\"storage.liquidity_provider_map_ordered\" b on \
                  a.\"storage.liquidity_provider_map_id\" = b.id {} {} {};",
            schema, schema, market_clause, connector, lp_provider_clause
        )
        .as_str(),
    )
    .map(LiquidityProviderMap::from_row)
    .fetch_all(conn)
    .await?;
    Ok(liquidity_providers)
}
