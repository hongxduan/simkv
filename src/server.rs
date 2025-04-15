//! simkv-server
//!
//! author: Duan HongXing
//! date: 4 Apr, 2025
//! 

use tokio::net::TcpListener;
use tokio::signal;

use std::env;

mod db;

mod kvtp;
mod cmd;

mod runner;

/// SimKV server main fn
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    run().await
}

/// Run
async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:8303".to_string());

    let listener = TcpListener::bind("0.0.0.0:8303").await?;
    println!("Listening on: {}", addr);

    runner::run(listener, signal::ctrl_c()).await;

    Ok(())
}
