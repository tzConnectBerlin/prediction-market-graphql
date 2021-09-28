use crate::db::get_schema;
use crate::models::LedgerMap;
use anyhow::Result;
use sqlx::{Pool, Postgres};

/**
* TODO: Find a better way to do this
*/
pub async fn get_ledgers(
    conn: &Pool<Postgres>,
    token_ids: Option<Vec<i32>>,
    owner_addrs: Option<Vec<String>>,
) -> Result<Vec<LedgerMap>> {
    let schema = get_schema();
    let tokens = match token_ids {
        Some(ref x) => {
            let toks = x
                .iter()
                .map(|i| i.to_string())
                .collect::<Vec<String>>()
                .join(", ");
            format!("WHERE idx_tokens_token_id in ({})", toks)
        }
        None => "".to_string(),
    };
    let owners = match owner_addrs {
        Some(ref x) => {
            let owner = x
                .iter()
                .map(|i| format!("'{}'", i))
                .collect::<Vec<String>>()
                .join(", ");
            let where_clause = if !tokens.is_empty() { "" } else { "WHERE" };
            format!("{} idx_tokens_owner in ({})", where_clause, owner)
        }
        None => "".to_string(),
    };
    let connector = match token_ids {
        Some(_) => match owner_addrs {
            Some(_) => " AND ",
            None => "",
        },
        None => "",
    };
    let ledger_maps = sqlx::query(
        format!(
            "SELECT level, level_timestamp, idx_tokens_owner, tokens_nat_2, idx_tokens_token_id \
        FROM \"{}\".\"storage.ledger_map_live\" {} {} {}",
            schema, tokens, connector, owners
        )
        .as_str(),
    )
    .map(LedgerMap::from_row_sqlx)
    .fetch_all(conn)
    .await?;
    Ok(ledger_maps)
}
