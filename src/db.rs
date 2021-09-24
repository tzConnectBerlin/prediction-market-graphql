use anyhow::Result;
use config::ConfigError;
use serde::Deserialize;
use std::env;
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

pub fn get_db_url() -> Result<String> {
    let host = env::var("PG__HOST").expect("HOST must be set");
    let port = env::var("PG__PORT").expect("PORT must be set");
    let user = env::var("PG__USER").expect("PORT must be set");
    let pwd = env::var("PG__PASSWORD").expect("PORT must be set");
    let dbname = env::var("PG__DBNAME").expect("PORT must be set");
    Ok(format!(
        "postgres://{}:{}@{}:{}/{}",
        user, pwd, host, port, dbname
    ))
}
