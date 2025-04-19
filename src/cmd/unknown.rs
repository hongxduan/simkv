//!
//! author: Duan HongXing
//! date: 6 Apr, 2025
//!

use crate::{
    cmd::INV_CMD,
    kvtp::{kvtp::KvtpMessage, response::KvtpResponse},
};

pub struct Unknown {
    akvp: KvtpMessage,
}

impl Unknown {
    pub fn new(akvp: KvtpMessage) -> Self {
        Unknown { akvp }
    }

    pub fn execute(&self) -> Vec<u8> {
        println!("{}", self.akvp.command);
        KvtpResponse::build_err(INV_CMD.to_vec())
    }
}
