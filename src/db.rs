use anyhow::Result;
use config::ConfigError;
use serde::Deserialize;
use std::env;
use tokio::net::TcpStream;
use tokio_postgres::tls::{NoTls, NoTlsStream};
use tokio_postgres::{Client, Connection};
pub type DBConnection = deadpool::managed::Object<deadpool_postgres::Manager>;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub pg: deadpool_postgres::Config,
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        let mut cfg = config::Config::new();
        cfg.merge(config::Environment::new().separator("__"))?;
        cfg.try_into()
    }
}

pub fn get_schema() -> String {
    env::var("SCHEMA").expect("SCHEMA must be set")
}

pub async fn get_raw_connection() -> Result<(Client, Connection<TcpStream, NoTlsStream>)> {
    let host = env::var("PG__HOST").expect("HOST must be set");
    let port = env::var("PG__PORT").expect("PORT must be set");
    let user = env::var("PG__USER").expect("PORT must be set");
    let pwd = env::var("PG__PASSWORD").expect("PORT must be set");
    let dbname = env::var("PG__DBNAME").expect("PORT must be set");
    let s = format!("postgres://{}:{}@{}:{}/{}", user, pwd, host, port, dbname);
    let conn = format!("{}:{}", host, port);
    println!("Connecting to {}", conn);
    let config = s.parse::<tokio_postgres::Config>()?;
    let socket = TcpStream::connect(conn).await?;
    Ok(config.connect_raw(socket, NoTls).await?)
}
