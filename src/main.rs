//! simkv-server
//!
//! author: Duan HongXing
//! date: 4 Apr, 2025
//!

use clap::Parser;
use config_file::FromConfigFile;
use server::config::{Config, GLOBAL_CONFIG};

mod cmd;
mod db;
mod kvtp;
mod raft;
mod server;
mod utils;

#[derive(Parser, Debug)]
#[command(version, about, disable_help_flag=true, long_about = None)]
struct Args {
    #[arg(short,long, default_value_t=String::from("server.toml"))]
    config: String,
}

/// SimKV server main fn
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let config = Config::from_config_file(args.config).unwrap();

    let mut config_lock = GLOBAL_CONFIG.write().unwrap();
    config_lock.config = config;
    drop(config_lock);

    crate::server::server::run().await
}
