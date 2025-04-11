//! simkv-server
//!
//! author: Duan HongXing
//! date: 4 Apr, 2025

use db::db::Db;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::signal;

use std::{env, vec};

mod db;

mod akvp;
mod cmd;
use cmd::command::Command;

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
