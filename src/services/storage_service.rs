use crate::env_config::get_schema;
use crate::models::Storage;
use anyhow::Result;

type Connection = deadpool::managed::Object<deadpool_postgres::Manager>;

pub async fn get_storage(conn: &Connection, identifier: i32) -> Result<Storage> {
  let schema = get_schema();
  let stmt = conn
    .prepare_cached(
      format!("SELECT s.id as storageid, s.lambda_repository_creator, \
  s.create_restrictions_creator_address , s.currency, tc.id as tcid, tc.\"level\", tc.contract ,\
  tc.operation_hash , tc.operation_group_number , tc.operation_number , tc.content_number , \
  tc.\"source\" , tc.entrypoint , tc.destination FROM {}.\"storage\" as s inner join public.tx_contexts tc \
  on s.tx_context_id = tc.id where s.id = $1;", schema).as_str(),
    )
    .await?;
  let rows = conn.query(&stmt, &[&identifier]).await?;
  let first = &rows[0];
  Ok(Storage::from_row(first))
}

pub async fn get_storages(conn: &Connection) -> Result<Vec<Storage>> {
  let schema = get_schema();
  let stmt = conn
    .prepare_cached(
      format!("SELECT s.id as storageid, s.lambda_repository_creator, \
  s.create_restrictions_creator_address , s.currency, tc.id as tcid, tc.\"level\", tc.contract ,\
  tc.operation_hash , tc.operation_group_number , tc.operation_number , tc.content_number , \
  tc.\"source\" , tc.entrypoint , tc.destination FROM {}.\"storage\" as s inner join public.tx_contexts tc \
  on s.tx_context_id = tc.id;", schema).as_str(),
    )
    .await?;
  let rows = conn.query(&stmt, &[]).await?;
  let storages = rows.iter().map(Storage::from_row).collect();
  Ok(storages)
}
