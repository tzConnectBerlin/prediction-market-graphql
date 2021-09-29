use crate::models::{LedgerMap, LiquidityProviderMap, Market, Storage, SupplyMap};
use crate::services::ledger_map_service::get_ledgers;
use crate::services::liquidity_provider_service::get_liquidity_providers;
use crate::services::market_map_service::get_markets;
use crate::services::storage_service::{get_storage, get_storages};
use crate::services::supply_map_service::get_supply_maps;
use futures::TryStreamExt;
use juniper::{
    graphql_object, graphql_subscription, graphql_value, EmptyMutation, FieldError, FieldResult,
    RootNode,
};
use log::{error, info};
use sqlx::postgres::PgListener;
use sqlx::{Pool, Postgres};
use std::pin::Pin;
#[derive(Clone)]
pub struct Context {
    pub pool: Pool<Postgres>,
}

impl juniper::Context for Context {}

pub struct Query;

#[graphql_object(Context = Context,
    description = "Query Root",)]
impl Query {
    #[graphql(description = "get current storage")]
    async fn storage(context: &Context) -> FieldResult<Storage> {
        let result = get_storage(&context.pool).await?;
        Ok(result)
    }

    #[graphql(description = "get all storages updates")]
    async fn storages(context: &Context) -> FieldResult<Vec<Storage>> {
        let storages = get_storages(&context.pool).await?;
        Ok(storages)
    }

    #[graphql(description = "get supply map for a given token id")]
    async fn token_supply(
        context: &Context,
        token_id: Option<Vec<i32>>,
    ) -> FieldResult<Vec<SupplyMap>> {
        let result = get_supply_maps(&context.pool, token_id).await?;
        Ok(result)
    }

    #[graphql(description = "get ledger maps and also filter by token and owner")]
    async fn ledgers(
        context: &Context,
        token_ids: Option<Vec<i32>>,
        owners: Option<Vec<String>>,
    ) -> FieldResult<Vec<LedgerMap>> {
        let result = get_ledgers(&context.pool, token_ids, owners).await?;
        Ok(result)
    }

    #[graphql(description = "get liquidity providers and also filter by market id and originator")]
    async fn liquidity_providers(
        context: &Context,
        market_ids: Option<Vec<i32>>,
        originators: Option<Vec<String>>,
    ) -> FieldResult<Vec<LiquidityProviderMap>> {
        let result = get_liquidity_providers(&context.pool, market_ids, originators).await?;
        Ok(result)
    }

    #[graphql(description = "get markets")]
    async fn markets(context: &Context, market_ids: Option<Vec<i32>>) -> FieldResult<Vec<Market>> {
        let result = get_markets(&context.pool, market_ids).await?;
        Ok(result)
    }
}

pub struct Subscription;

pub type LedgerStream =
    Pin<Box<dyn futures::Stream<Item = Result<Vec<LedgerMap>, FieldError>> + Send>>;

pub type MarketStream =
    Pin<Box<dyn futures::Stream<Item = Result<Vec<Market>, FieldError>> + Send>>;

pub type LiquidityProviderMapStream =
    Pin<Box<dyn futures::Stream<Item = Result<Vec<LiquidityProviderMap>, FieldError>> + Send>>;

pub type SupplyMapStream =
    Pin<Box<dyn futures::Stream<Item = Result<Vec<SupplyMap>, FieldError>> + Send>>;

#[graphql_subscription(Context = Context)]
impl Subscription {
    #[graphql(description = "Sends all the ledgers when they change")]
    async fn ledgers(context: &Context) -> LedgerStream {
        let conn = context.pool.clone();
        let mut listener = PgListener::connect_with(&conn).await.unwrap();
        listener.listen("ledger_notify").await.unwrap();
        let mut stream = listener.into_stream();

        let new_stream = async_stream::stream! {
            loop {
                match stream.try_next().await {
                    Ok(n) => {
                        if let Some(msg) = n {
                            info!("{:?}", msg);

                            let ledgers = get_ledgers(&conn, None, None).await.unwrap();
                            yield Ok(ledgers)
                        }
                    }
                    Err(err) => {
                        error!("{}", err);
                        yield Err(FieldError::new(
                                "Ledger Subscription Error",
                                graphql_value!(format!("{}", err)),
                            ))
                    }
              }
            }
        };

        Box::pin(new_stream)
    }

    #[graphql(description = "Sends all the markets when they change")]
    async fn markets(context: &Context) -> MarketStream {
        let conn = context.pool.clone();
        let mut listener = PgListener::connect_with(&conn).await.unwrap();
        listener.listen("market_notify").await.unwrap();
        let mut stream = listener.into_stream();

        let new_stream = async_stream::stream! {
            loop {
                match stream.try_next().await {
                    Ok(n) => {
                        if let Some(msg) = n {
                            info!("{:?}", msg);
                            let markets = get_markets(&conn, None).await.unwrap();
                            yield Ok(markets)
                        }
                    }
                    Err(err) => {
                        error!("{}", err);
                        yield Err(FieldError::new(
                                "Market Subscription Error",
                                graphql_value!(format!("{}", err)),
                            ))
                    }
              }
            }
        };

        Box::pin(new_stream)
    }

    #[graphql(description = "Sends all the liquidity providers when they change")]
    async fn liquidity_providers(context: &Context) -> LiquidityProviderMapStream {
        let conn = context.pool.clone();
        let mut listener = PgListener::connect_with(&conn).await.unwrap();
        listener.listen("liquidity_provider_notify").await.unwrap();
        let mut stream = listener.into_stream();

        let new_stream = async_stream::stream! {
            loop {
                match stream.try_next().await {
                    Ok(n) => {
                        if let Some(msg) = n {
                            info!("{:?}", msg);
                            let providers = get_liquidity_providers(&conn, None, None).await.unwrap();
                            yield Ok(providers)
                        }
                    }
                    Err(err) => {
                        error!("{}", err);
                        yield Err(FieldError::new(
                                "Liquidity Provider Subscription Error",
                                graphql_value!(format!("{}", err)),
                            ))
                    }
              }
            }
        };

        Box::pin(new_stream)
    }

    #[graphql(description = "Sends all the token supplies when they change")]
    async fn token_supplies(context: &Context) -> SupplyMapStream {
        let conn = context.pool.clone();
        let mut listener = PgListener::connect_with(&conn).await.unwrap();
        listener.listen("token_supplies_notify").await.unwrap();
        let mut stream = listener.into_stream();

        let new_stream = async_stream::stream! {
            loop {
                match stream.try_next().await {
                    Ok(n) => {
                        if let Some(msg) = n {
                            info!("{:?}", msg);
                            let tokens = get_supply_maps(&conn, None).await.unwrap();
                            yield Ok(tokens)
                        }
                    }
                    Err(err) => {
                        error!("{}", err);
                        yield Err(FieldError::new(
                                "Token Supply Subscription Error",
                                graphql_value!(format!("{}", err)),
                            ))
                    }
              }
            }
        };

        Box::pin(new_stream)
    }
}

pub type Schema = RootNode<'static, Query, EmptyMutation<Context>, Subscription>;

pub fn create_schema() -> Schema {
    Schema::new(Query {}, EmptyMutation::<Context>::new(), Subscription {})
}
