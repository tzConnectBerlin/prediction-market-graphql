use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use juniper::GraphQLObject;
use pg_bigdecimal::PgNumeric;

pub fn numeric_to_string(n: PgNumeric) -> String {
    let optn: Option<BigDecimal> = n.n;
    match optn {
        Some(n) => n.normalized().to_string(),
        None => "Null".to_string(),
    }
}

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
    pub fn from_row(row: &tokio_postgres::Row) -> Storage {
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
    pub total_supply: String,
    pub in_reserve: String,
    pub token_id: String,
}

impl SupplyMap {
    pub fn from_row(row: &tokio_postgres::Row) -> SupplyMap {
        SupplyMap {
            level: row.get(0),
            timestamp: row.get(1),
            total_supply: numeric_to_string(row.get(2)),
            in_reserve: numeric_to_string(row.get(3)),
            token_id: numeric_to_string(row.get(4)),
        }
    }
}
