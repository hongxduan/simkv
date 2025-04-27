//! Server config data
//!
//! author: Duan HongXing
//! date: 25 Apr, 2025
//!

use std::sync::RwLock;

use lazy_static::lazy_static;
use serde::Deserialize;

lazy_static! {
    pub static ref GLOBAL_CONFIG: RwLock<GloablConfig> = RwLock::new(GloablConfig {
        config: Config {
            host: String::from(""),
            port: 0,
            datadir: String::from(""),
            logdir: String::from("")
        }
    });
}

pub struct GloablConfig {
    pub config: Config,
}

#[derive(Clone, Deserialize)]
pub struct Config {
    //pub server: Server,
    pub host: String,
    pub port: u16,
    pub datadir: String,
    pub logdir: String,
}

#[derive(Clone, Deserialize)]
pub struct Server {
    pub host: String,
    pub port: u16,
}
