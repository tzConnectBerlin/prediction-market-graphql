use crate::models::{Storage, SupplyMap};
use crate::services::storage_service::{get_storage, get_storages};
use crate::services::supply_map_service::{get_all_supply_maps, get_supply_map_by_token};
use juniper::{graphql_object, EmptyMutation, EmptySubscription, FieldResult, RootNode};
use pg_bigdecimal::PgNumeric;

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
    async fn supply_map(context: &Context, token_id: i32) -> FieldResult<SupplyMap> {
        let conn = context.pool.get().await?;
        let result = get_supply_map_by_token(&conn, token_id).await?;
        Ok(result)
    }

    #[graphql(description = "get supply map for a given token id")]
    async fn supply_maps(context: &Context) -> FieldResult<Vec<SupplyMap>> {
        let conn = context.pool.get().await?;
        let result = get_all_supply_maps(&conn).await?;
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
