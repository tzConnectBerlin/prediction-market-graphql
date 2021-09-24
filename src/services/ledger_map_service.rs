use crate::db::{get_schema, DBConnection};
use crate::models::LedgerMap;
use anyhow::Result;

/**
* TODO: Find a better way to do this
*/
pub async fn get_ledgers(
    conn: &DBConnection,
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
    let stmt = conn
        .prepare_cached(
            format!(
        "SELECT level, level_timestamp, idx_tokens_owner, tokens_nat_2, idx_tokens_token_id \
        FROM \"{}\".\"storage.ledger_map_live\" {} {} {}",
        schema,
        tokens,
        connector,
        owners
      )
            .as_str(),
        )
        .await?;

    let rows = conn.query(&stmt, &[]).await?;
    let ledger_maps = rows.iter().map(LedgerMap::from_row).collect();
    Ok(ledger_maps)
}
