use crate::errors::Result;
use crate::utils::config_loader::BrokerConfig;
use tokio::net::TcpListener;
use log::info;

pub async fn run(config: BrokerConfig) -> Result<()> {
    let addr = format!("{}:{}", config.server.host, config.server.port);
    let listener = TcpListener::bind(&addr).await?;
    info!("listening on {}", addr);

    loop {
        let (socket, peer) = listener.accept().await?;
        info!("accepted {}", peer);
        tokio::spawn(async move {
            if let Err(e) = crate::server::connection::handle(socket).await {
                log::warn!("connection error: {}", e);
            }
        });
    }
}
