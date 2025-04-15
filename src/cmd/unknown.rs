//!
//! author: Duan HongXing
//! date: 6 Apr, 2025
//!

use crate::kvtp::kvtp::KvtpMessage;

pub struct Unknown {
    akvp: KvtpMessage,
}

impl Unknown {
    pub fn new(akvp: KvtpMessage) -> Self {
        Unknown { akvp }
    }

    pub fn execute(&self) -> Vec<u8> {
        println!("{}", self.akvp.command);
        String::from("Invalid command").as_bytes().to_vec()
    }
}
