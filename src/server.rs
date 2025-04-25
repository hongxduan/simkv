//! simkv-server
//!
//! author: Duan HongXing
//! date: 4 Apr, 2025
//!

use clap::Parser;
use config_file::FromConfigFile;
use serde::Deserialize;
use tokio::net::TcpListener;
use tokio::signal;

use std::env;

mod db;

mod cmd;
mod kvtp;

mod runner;

mod raft;

#[derive(Parser, Debug)]
#[command(version, about, disable_help_flag=true, long_about = None)]
struct Args {
    #[arg(short,long, default_value_t=String::from("server.toml"))]
    config: String,
}

#[derive(Deserialize)]
struct Config {
    server: Server,
}

#[derive(Deserialize)]
struct Server {
    host: String,
    port: u16,
}

/// SimKV server main fn
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let config = Config::from_config_file(args.config).unwrap();

    run(config).await
}

/// Run
async fn run(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    tokio::spawn(async {
        let _ = crate::raft::raft::Raft::start().await;
    });

    let addr = format!("{}:{}", config.server.host, config.server.port);
    let listener = TcpListener::bind(addr.clone()).await?;
    println!("Listening on: {}", addr);

    runner::run(listener, signal::ctrl_c()).await;

    Ok(())
}
