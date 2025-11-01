pub mod databases;
pub mod logger;

use color_eyre::Result;
use serde::Deserialize;

/// Represents configuration structure
#[derive(Deserialize, Clone, Debug, Default)]
pub struct Config {
    /// Environment: `developement` or `production`
    pub environment: String,

    /// Server URL
    pub bind_address: String,
    /// Server port
    pub bind_port: String,

    /// Database URL (Ex.: postgres://foo:bar@127.0.0.1:5436/crud_api)
    pub database_url: String,
    /// Database auto migration enabled
    pub database_auto_migration: bool,
    /// Database maximum connections (in seconds)
    pub database_max_connections: u32,
    /// Database minimum connections (in seconds)
    pub database_min_connections: u32,
    /// Database maximum lifetime (in seconds)
    pub database_connection_lifetime: u64,
    /// Database connection timeout (in seconds)
    pub database_connect_timeout: u64,
    /// Database connection timeout (in seconds)
    pub database_idle_timeout: u64,

    /// Prometheus metics enabled
    pub prometheus_metrics_enabled: bool,
}

impl Config {
    /// from_env loads configuration from environment variables
    pub fn from_env() -> Result<Config> {
        dotenvy::dotenv().ok();

        Ok(config::Config::builder()
            .add_source(config::Environment::default())
            .build()?
            .try_deserialize()?)
    }
}
