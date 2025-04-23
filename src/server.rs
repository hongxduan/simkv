//! simkv-server
//!
//! author: Duan HongXing
//! date: 4 Apr, 2025
//!

use raft::raft::Raft;
use tokio::net::TcpListener;
use tokio::signal;

use std::env;
use std::sync::{Arc, Mutex};

mod db;

mod cmd;
mod kvtp;

mod runner;

mod raft;

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

    /*tokio::spawn(async {
        let _ = crate::raft::server::run().await;
    });*/
    let _ = crate::raft::raft::Raft::start().await;

    tokio::spawn(async {
        let _= crate::raft::vote::Vote::supress().await;
    });

    let listener = TcpListener::bind("0.0.0.0:8303").await?;
    println!("Listening on: {}", addr);

    runner::run(listener, signal::ctrl_c()).await;

    Ok(())
}
