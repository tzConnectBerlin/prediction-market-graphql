#[macro_use]
extern crate rocket;
extern crate deadpool_postgres;
extern crate dotenv;
extern crate juniper_rocket;
use dotenv::dotenv;
use rocket::{response::content, State};
mod env_config;
mod graphql;
mod models;
mod services;

#[rocket::get("/")]
fn graphiql() -> content::Html<String> {
    juniper_rocket::graphiql_source("/graphql", None)
}

#[rocket::get("/graphql?<request>")]
async fn get_graphql_handler(
    context: &State<graphql::Context>,
    request: juniper_rocket::GraphQLRequest,
    schema: &State<graphql::Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute(&*schema, &*context).await
}

#[rocket::post("/graphql", data = "<request>")]
async fn post_graphql_handler(
    context: &State<graphql::Context>,
    request: juniper_rocket::GraphQLRequest,
    schema: &State<graphql::Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute(&*schema, &*context).await
}

#[launch]
fn rocket() -> _ {
    dotenv().ok();
    let config = env_config::Config::from_env().unwrap();
    let pool = config.pg.create_pool(tokio_postgres::NoTls).unwrap();
    let schema_context = graphql::Context { pool: pool.clone() };
    let schema = graphql::create_schema();
    rocket::build()
        .manage(schema_context.clone())
        .manage(schema)
        .mount(
            "/",
            rocket::routes![graphiql, get_graphql_handler, post_graphql_handler],
        )
}
