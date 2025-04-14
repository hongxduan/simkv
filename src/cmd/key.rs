//! Key command implement
//!
//! author: Duan HongXing
//! date: 4 Apr, 2025
//!
use crate::akvp::kvtp::KvtpMessage;

pub struct Key {
    kvtp: KvtpMessage,
}

impl Key {
    pub fn new(kvtp: KvtpMessage) -> Self {
        Key { kvtp }
    }

    pub fn execute(&self) -> Vec<u8> {
        println!("{}", self.kvtp.command);

        Vec::new()
    }
}
