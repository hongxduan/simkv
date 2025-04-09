///! Key command implement
///!
///! author: Duan HongXing
///! date: 4 Apr, 2025
///
use crate::akvp::kvtp::KvtpMessage;

pub struct Key {
    akvp: KvtpMessage,
}

impl Key {
    pub fn new(akvp: KvtpMessage) -> Self {
        Key { akvp }
    }

    pub fn execute(&self) -> Vec<u8> {
        println!("{}", self.akvp.command);

        Vec::new()
    }
}
