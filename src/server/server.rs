//! Server
//!
//! author: Duan HongXing
//! date: 25 Apr, 2025
//!

use tokio::{net::TcpListener, signal};

use crate::server::runner;

use super::config::Config;

/// Run
pub async fn run(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    let config_copy = config.clone();

    // Init raft
    crate::raft::raft::Raft::init(config.clone());

    // Run Raft server
    tokio::spawn(async move {
        let _ = crate::raft::raft::Raft::start(config_copy).await;
    });

    // Run main server
    let addr = format!("{}:{}", config.host, config.port);
    let listener = TcpListener::bind(addr.clone()).await?;
    println!("Listening on: {}", addr);
    runner::run(listener, signal::ctrl_c()).await;

    Ok(())
}
