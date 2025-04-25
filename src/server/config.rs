//! Server config data
//!
//! author: Duan HongXing
//! date: 25 Apr, 2025
//!

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub server: Server,
}

#[derive(Deserialize)]
pub struct Server {
    pub host: String,
    pub port: u16,
}
