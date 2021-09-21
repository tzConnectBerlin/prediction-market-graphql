use crate::models::Storage;
use crate::services::storage_service::{get_storage, get_storages};
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
  #[graphql(description = "get a storage")]
  async fn storage(context: &Context, id: i32) -> FieldResult<Storage> {
    let conn = context.pool.get().await?;
    let result = get_storage(&conn, id).await?;
    Ok(result)
  }

  #[graphql(description = "get all storages")]
  async fn storages(context: &Context) -> FieldResult<Vec<Storage>> {
    let conn = context.pool.get().await?;
    let storages = get_storages(&conn).await?;
    Ok(storages)
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
