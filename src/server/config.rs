//! Server config data
//!
//! author: Duan HongXing
//! date: 25 Apr, 2025
//!

use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct Config {
    pub server: Server,
}

#[derive(Clone, Deserialize)]
pub struct Server {
    pub host: String,
    pub port: u16,
}
