use env_logger::Env;
use std::time::Duration;

use actix_cors::Cors;
use actix_web::{
    http::header,
    middleware,
    web::{self, Data},
    App, Error, HttpRequest, HttpResponse, HttpServer,
};
use juniper_actix::{graphql_handler, playground_handler, subscriptions::subscriptions_handler};
use juniper_graphql_ws::ConnectionConfig;

use dotenv::dotenv;
mod db;
mod graphql;
mod models;
mod services;
mod utils;

async fn playground() -> Result<HttpResponse, Error> {
    playground_handler("/graphql", Some("/subscriptions")).await
}

async fn graphql(
    req: actix_web::HttpRequest,
    payload: actix_web::web::Payload,
    schema: web::Data<graphql::Schema>,
    context: web::Data<graphql::Context>,
) -> Result<HttpResponse, Error> {
    graphql_handler(&schema, &context, req, payload).await
}

async fn subscriptions(
    req: HttpRequest,
    stream: web::Payload,
    schema: web::Data<graphql::Schema>,
    context: web::Data<graphql::Context>,
) -> Result<HttpResponse, Error> {
    let schema = schema.into_inner();
    let config = ConnectionConfig::new(graphql::Context {
        pool: context.pool.clone(),
    });
    // set the keep alive interval to 15 secs so that it doesn't timeout in playground
    // playground has a hard-coded timeout set to 20 secs
    let config = config.with_keep_alive_interval(Duration::from_secs(15));

    subscriptions_handler(req, stream, schema, config).await
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let env = Env::default().filter_or("RUST_LOG", "info");
    env_logger::init_from_env(env);
    dotenv().ok();
    let config = db::Config::from_env().unwrap();
    let pool = config.pg.create_pool(tokio_postgres::NoTls).unwrap();
    let schema_context = graphql::Context { pool };
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(graphql::create_schema()))
            .app_data(Data::new(schema_context.clone()))
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allowed_methods(vec!["POST", "GET"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .supports_credentials()
                    .max_age(3600),
            )
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .service(web::resource("/subscriptions").route(web::get().to(subscriptions)))
            .service(
                web::resource("/graphql")
                    .route(web::post().to(graphql))
                    .route(web::get().to(graphql)),
            )
            .service(web::resource("/playground").route(web::get().to(playground)))
            .default_service(web::route().to(|| {
                HttpResponse::Found()
                    .append_header((header::LOCATION, "/playground"))
                    .finish()
            }))
    })
    .bind(format!("{}:{}", "127.0.0.1", 8080))?
    .run()
    .await
}
