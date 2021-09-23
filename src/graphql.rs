use crate::models::{LedgerMap, LiquidityProviderMap, Market, Storage, SupplyMap};
use crate::services::ledger_map_service::get_ledgers;
use crate::services::liquidity_provider_service::get_liquidity_providers;
use crate::services::market_map_service::get_markets;
use crate::services::storage_service::{get_storage, get_storages};
use crate::services::supply_map_service::{get_all_supply_maps, get_supply_map_by_token};
use juniper::{graphql_object, EmptyMutation, EmptySubscription, FieldResult, RootNode};

#[derive(Clone)]
pub struct Context {
    pub pool: deadpool::managed::Pool<deadpool_postgres::Manager>,
}

impl juniper::Context for Context {}

pub struct Query;

#[graphql_object(Context = Context,
    description = "Query Root",)]
impl Query {
    #[graphql(description = "get current storage")]
    async fn storage(context: &Context) -> FieldResult<Storage> {
        let conn = context.pool.get().await?;
        let result = get_storage(&conn).await?;
        Ok(result)
    }

    #[graphql(description = "get all storages updates")]
    async fn storages(context: &Context) -> FieldResult<Vec<Storage>> {
        let conn = context.pool.get().await?;
        let storages = get_storages(&conn).await?;
        Ok(storages)
    }

    #[graphql(description = "get supply map for a given token id")]
    async fn token_supply(context: &Context, token_id: i32) -> FieldResult<SupplyMap> {
        let conn = context.pool.get().await?;
        let result = get_supply_map_by_token(&conn, token_id).await?;
        Ok(result)
    }

    #[graphql(description = "get supply map for a given token id")]
    async fn token_supplies(context: &Context) -> FieldResult<Vec<SupplyMap>> {
        let conn = context.pool.get().await?;
        let result = get_all_supply_maps(&conn).await?;
        Ok(result)
    }

    #[graphql(description = "get ledger maps and also filter by token and owner")]
    async fn ledger(
        context: &Context,
        token_ids: Option<Vec<i32>>,
        owners: Option<Vec<String>>,
    ) -> FieldResult<Vec<LedgerMap>> {
        let conn = context.pool.get().await?;
        let result = get_ledgers(&conn, token_ids, owners).await?;
        Ok(result)
    }

    #[graphql(description = "get liquidity providers and also filter by market id and originator")]
    async fn liquidity_providers(
        context: &Context,
        market_ids: Option<Vec<i32>>,
        originators: Option<Vec<String>>,
    ) -> FieldResult<Vec<LiquidityProviderMap>> {
        let conn = context.pool.get().await?;
        let result = get_liquidity_providers(&conn, market_ids, originators).await?;
        Ok(result)
    }

    #[graphql(description = "get markets")]
    async fn markets(context: &Context, market_ids: Option<Vec<i32>>) -> FieldResult<Vec<Market>> {
        let conn = context.pool.get().await?;
        let result = get_markets(&conn, market_ids).await?;
        Ok(result)
    }
}

pub type Schema = RootNode<'static, Query, EmptyMutation<Context>, EmptySubscription<Context>>;

pub fn create_schema() -> Schema {
    Schema::new(
        Query {},
        EmptyMutation::<Context>::new(),
        EmptySubscription::<Context>::new(),
    )
}
