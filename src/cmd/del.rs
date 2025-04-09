///! Del command implement
///!
///! author: Duan HongXing
///! date: 4 Apr, 2025
///!
use crate::akvp::kvtp::KvtpMessage;

pub struct Del {
    akvp: KvtpMessage,
}

impl Del {
    pub fn new(akvp: KvtpMessage) -> Self {
        Del { akvp }
    }

    pub fn execute(&self) -> Vec<u8> {
        println!("{}", self.akvp.command);
        Vec::new()
    }
}
