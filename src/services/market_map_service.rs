use crate::db::get_schema;
use crate::models::Market;
use anyhow::Result;
use sqlx::{Pool, Postgres};

/**
* TODO: Find a better way to do this
*/
pub async fn get_markets(
    conn: &Pool<Postgres>,
    market_ids: Option<Vec<i32>>,
) -> Result<Vec<Market>> {
    let schema = get_schema();
    let market_clause = match market_ids {
        Some(ref x) => {
            let markts = x
                .iter()
                .map(|i| i.to_string())
                .collect::<Vec<String>>()
                .join(", ");
            format!("WHERE market_map.idx_markets_nat_4 in ({})", markts)
        }
        None => "".to_string(),
    };

    let markets = sqlx::query(format!(
                "SELECT market_map.state, market_map.level, market_map.level_timestamp, \
                market_map.currency, market_map.metadata_ipfs_hash, market_map.metadata_adjudicator,\
                market_map.idx_markets_nat_4, market_map.metadata_description,\
                auctions.\"auctionRunning_yes_preference\" , auctions.\"auctionRunning_uniswap_contribution\",\
                auctions.\"auctionRunning_auction_period_end\",\
                auctions.\"auctionRunning_quantity\",\
                markets.currency_pool_creator_reward_currency_pool, \
                markets.currency_pool_nat_5, markets.currency_pool_market_currency_pool,\
                markets.\"marketBootstrapped_nat_6\", markets.\"marketBootstrapped_bootstrap_yes_probability\",\
                markets.resolution_resolved_at_block, markets.resolution_winning_prediction,\
                markets.\"marketBootstrapped_bootstrapped_at_block\" \
                FROM {}.\"storage.market_map_live\" market_map \
                left join {}.\"storage.market_map.auctionRunning_ordered\" auctions \
                on market_map.id = auctions.\"storage.market_map_id\"\
                left join {}.\"storage.market_map.marketBootstrapped_ordered\" markets \
                on market_map.id = markets.\"storage.market_map_id\" {};",
                schema, schema, schema, market_clause,
            )
            .as_str()).map(Market::from_row).fetch_all(conn).await?;
    Ok(markets)
}
