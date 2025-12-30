use serde::Deserialize;
use std::{fs, path::Path};
use crate::errors::{Result, Error};

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub use_tls: bool,
    pub cert_file: String,
    pub key_file: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct LoggingConfig {
    pub level: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct BrokerSection {
    pub keep_alive: u64,
    pub offline_queue_max: usize,
    pub allow_anonymous: bool,
}

#[derive(Debug, Deserialize, Clone)]
pub struct BrokerConfig {
    pub server: ServerConfig,
    pub logging: LoggingConfig,
    pub broker: BrokerSection,
}

pub fn load_config<P: AsRef<Path>>(path: P) -> Result<BrokerConfig> {
    let content = fs::read_to_string(&path)?;
    toml::from_str(&content).map_err(|e| Error::Config(format!("{}", e)))
}
