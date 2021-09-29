use dotenv::dotenv;
use futures::FutureExt as _;
use juniper_graphql_ws::ConnectionConfig;
use juniper_warp::{playground_filter, subscriptions::serve_graphql_ws};
use std::env;
use std::sync::Arc;
use warp::{http::Response, Filter};

mod db;
mod graphql;
mod models;
mod services;
mod utils;

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();

    let log = warp::log("prediction-market-graphql");

    let server_port = env::var("SERVER_PORT")
        .expect("SERVER_PORT not set")
        .as_str()
        .parse::<u16>()
        .unwrap();

    let homepage = warp::path::end().map(|| {
        Response::builder()
            .header("content-type", "text/html")
            .body("<html><h1>juniper_subscriptions demo</h1><div>visit <a href=\"/playground\">graphql playground</a></html>".to_string())
    });
    let pool = db::get_db_pool().await.unwrap();
    sqlx::migrate!().run(&pool).await.unwrap();
    let schema_context = graphql::Context { pool: pool.clone() };

    let qm_schema = graphql::create_schema();
    let qm_state = warp::any().map(move || schema_context.clone());
    let qm_graphql_filter = juniper_warp::make_graphql_filter(qm_schema, qm_state.boxed());

    let root_node = Arc::new(graphql::create_schema());

    log::info!("Listening on 127.0.0.1:{}", server_port);

    let routes = (warp::path("subscriptions")
        .and(warp::ws())
        .map(move |ws: warp::ws::Ws| {
            let root_node = root_node.clone();
            let pool = pool.clone();
            ws.on_upgrade(move |websocket| async move {
                serve_graphql_ws(
                    websocket,
                    root_node,
                    ConnectionConfig::new(graphql::Context { pool }),
                )
                .map(|r| {
                    if let Err(e) = r {
                        println!("Websocket error: {}", e);
                    }
                })
                .await
            })
        }))
    .map(|reply| {
        // TODO#584: remove this workaround
        warp::reply::with_header(reply, "Sec-WebSocket-Protocol", "graphql-ws")
    })
    .or(warp::post()
        .and(warp::path("graphql"))
        .and(qm_graphql_filter))
    .or(warp::get()
        .and(warp::path("playground"))
        .and(playground_filter("/graphql", Some("/subscriptions"))))
    .or(homepage)
    .with(log);

    warp::serve(routes).run(([127, 0, 0, 1], server_port)).await;
}
