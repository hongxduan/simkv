//! Server
//!
//! author: Duan HongXing
//! date: 25 Apr, 2025
//!

use tokio::{net::TcpListener, signal};

use crate::server::{config::GLOBAL_CONFIG, runner};

/// Run
pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let config_lock = GLOBAL_CONFIG.read().unwrap();

    // Init raft
    crate::raft::raft::Raft::pre_start();

    // Run Raft server
    let port = config_lock.config.port + 10000;
    let addr = format!("{}:{}", config_lock.config.host, port);
    tokio::spawn(async move {
        let _ = crate::raft::raft::Raft::start(addr).await;
    });

    // Run main server
    let addr = format!("{}:{}", config_lock.config.host, config_lock.config.port);
    let listener = TcpListener::bind(addr.clone()).await?;
    println!("Listening on: {}", addr);
    runner::run(listener, signal::ctrl_c()).await;

    drop(config_lock);

    Ok(())
}
