mod broker;
mod protocol;
mod security;
mod server;
mod utils;

mod errors;

use crate::utils::config_loader::BrokerConfig;
use log::{error, info};
use std::env;

#[tokio::main]
async fn main() {
    env_logger::init();

    let cfg_path = env::args().nth(1).unwrap_or_else(|| "config/default.toml".into());
    let config = match utils::config_loader::load_config(&cfg_path) {
        Ok(c) => c,
        Err(e) => {
            error!("failed to load config {}: {}", cfg_path, e);
            std::process::exit(2);
        }
    };

    info!("starting CrabMQ on {}:{} (tls: {})", config.server.host, config.server.port, config.server.use_tls);

    if let Err(e) = server::listener::run(config).await {
        error!("server crashed: {}", e);
        std::process::exit(1);
    }
}
