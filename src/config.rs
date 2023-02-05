//! The URL database

use crate::error::Error;
use serde::Deserialize;
use std::{borrow::Cow, env, ops::Deref};

/// The server config
#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    /// The IP address and port to listen on
    pub address: String,
    /// The connection hart limit; i.e. the amount of threads to spawn at max to process incoming connections
    #[serde(default = "ServerConfig::connection_limit_default")]
    pub connection_limit: usize,
}
impl ServerConfig {
    /// The default value for the connection hard limit
    const fn connection_limit_default() -> usize {
        2048
    }
}

/// The URL database
#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    /// The URL redirects
    pub server: ServerConfig,
}
impl Config {
    /// Loads the config from the file
    pub fn load() -> Result<Self, Error> {
        // Get the path from the environment or fallback to a default path
        let path = match env::var("CONFIG_FILE") {
            Ok(path) => Cow::Owned(path),
            Err(_) => Cow::Borrowed("config.toml"),
        };

        // Decode the database
        let data = std::fs::read(path.deref())?;
        let config: Self = toml::from_slice(&data)?;
        Ok(config)
    }
}
