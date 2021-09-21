use config::ConfigError;
use serde::Deserialize;
use std::env;

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
