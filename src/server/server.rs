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
    tokio::spawn(async {
        let _ = crate::raft::raft::Raft::start().await;
    });

    let addr = format!("{}:{}", config.server.host, config.server.port);
    let listener = TcpListener::bind(addr.clone()).await?;
    println!("Listening on: {}", addr);

    runner::run(listener, signal::ctrl_c()).await;

    Ok(())
}
