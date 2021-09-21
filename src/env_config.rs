use config::ConfigError;
use serde::Deserialize;

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
