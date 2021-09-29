use crate::utils::numeric_to_string;
use chrono::{DateTime, Utc};
use juniper::GraphQLObject;
use sqlx::{postgres::PgRow, Row};

// Storage

#[derive(Debug, Clone, GraphQLObject)]
pub struct Storage {
    pub level: i32,
    pub lambda_repository_creator: Option<String>,
    pub create_restrictions_creator_address: Option<String>,
    pub currency: Option<String>,
    pub timestamp: DateTime<Utc>,
}

impl Storage {
    pub fn from_row(row: PgRow) -> Storage {
        Storage {
            level: row.get(0),
            timestamp: row.get(1),
            lambda_repository_creator: row.get(2),
            create_restrictions_creator_address: row.get(3),
            currency: row.get(4),
        }
    }
}

// Supply Map

#[derive(Debug, Clone, GraphQLObject)]
pub struct SupplyMap {
    pub level: i32,
    pub timestamp: DateTime<Utc>,
    pub total_supply: Option<String>,
    pub in_reserve: Option<String>,
    pub token_id: Option<String>,
}

impl SupplyMap {
    pub fn from_row(row: PgRow) -> SupplyMap {
        SupplyMap {
            level: row.get(0),
            timestamp: row.get(1),
            total_supply: numeric_to_string(row.get(2)),
            in_reserve: numeric_to_string(row.get(3)),
            token_id: numeric_to_string(row.get(4)),
        }
    }
}

// Ledger Map

#[derive(Debug, Clone, GraphQLObject)]
pub struct LedgerMap {
    pub level: i32,
    pub timestamp: DateTime<Utc>,
    pub owner: String,
    pub balance: Option<String>,
    pub token_id: Option<String>,
}

impl LedgerMap {
    pub fn from_row_sqlx(row: PgRow) -> LedgerMap {
        LedgerMap {
            level: row.get(1),
            timestamp: row.get(1),
            owner: row.get(2),
            balance: numeric_to_string(row.get(3)),
            token_id: numeric_to_string(row.get(4)),
        }
    }
}

// liquidity provider map

#[derive(Debug, Clone, GraphQLObject)]
pub struct LiquidityProviderMap {
    pub level: i32,
    pub timestamp: DateTime<Utc>,
    pub bet: Option<String>,
    pub probability: Option<String>,
    pub market_id: Option<String>,
    pub originator: String,
}

impl LiquidityProviderMap {
    pub fn from_row(row: PgRow) -> LiquidityProviderMap {
        LiquidityProviderMap {
            level: row.get(0),
            timestamp: row.get(1),
            bet: numeric_to_string(row.get(2)),
            probability: numeric_to_string(row.get(3)),
            market_id: numeric_to_string(row.get(4)),
            originator: row.get(5),
        }
    }
}

// Market auction running

#[derive(Debug, Clone, GraphQLObject)]
pub struct AuctionRunning {
    pub yes_preference: Option<String>,
    pub uniswap_contribution: Option<String>,
    pub period_end: Option<DateTime<Utc>>,
    pub quantity: Option<String>,
}

// Market bootstrapped

#[derive(Debug, Clone, GraphQLObject)]
pub struct MarketBootstrapped {
    pub creator_reward_currency_pool: Option<String>,
    pub liquidity_reward_currency_pool: Option<String>,
    pub market_currency_pool: Option<String>,
    pub yes_probability_at_bootstrap: Option<String>,
    pub liquidity_reward_supply_updated_at: Option<String>,
    pub winning_prediction: Option<String>,
    pub resolved_at: Option<String>,
    pub market_bootstrapped_at: Option<String>,
}

// Market

#[derive(Debug, Clone, GraphQLObject)]
pub struct Market {
    pub level: i32,
    pub timestamp: DateTime<Utc>,
    pub currency: Option<String>,
    pub ipfs_hash: Option<String>,
    pub adjudicator: String,
    pub market_id: Option<String>,
    pub state: String,
    pub description: String,
    pub auction_running: Option<AuctionRunning>,
    pub market_bootstrapped: Option<MarketBootstrapped>,
}

impl Market {
    pub fn from_row(row: PgRow) -> Market {
        let state: String = row.get(0);
        let auction_running: Option<AuctionRunning> = if state.contains("auctionRunning") {
            Some(AuctionRunning {
                yes_preference: numeric_to_string(row.get(8)),
                uniswap_contribution: numeric_to_string(row.get(9)),
                period_end: row.get(10),
                quantity: numeric_to_string(row.get(11)),
            })
        } else {
            None
        };
        let market_bootstrapped: Option<MarketBootstrapped> =
            if state.contains("marketBootstrapped") {
                Some(MarketBootstrapped {
                    creator_reward_currency_pool: numeric_to_string(row.get(12)),
                    liquidity_reward_currency_pool: numeric_to_string(row.get(13)),
                    market_currency_pool: numeric_to_string(row.get(14)),
                    liquidity_reward_supply_updated_at: numeric_to_string(row.get(15)),
                    yes_probability_at_bootstrap: numeric_to_string(row.get(16)),
                    resolved_at: numeric_to_string(row.get(17)),
                    winning_prediction: row.get(18),
                    market_bootstrapped_at: numeric_to_string(row.get(19)),
                })
            } else {
                None
            };
        Market {
            level: row.get(1),
            timestamp: row.get(2),
            currency: row.get(3),
            ipfs_hash: row.get(4),
            adjudicator: row.get(5),
            market_id: numeric_to_string(row.get(6)),
            description: row.get(7),
            state,
            auction_running,
            market_bootstrapped,
        }
    }
}
