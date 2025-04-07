///! Key key implement
///!
///! author: Duan HongXing
///! date: 4 Apr, 2025
///
use crate::akvp::akvp::AkvpMessage;

pub struct Key {
    akvp: AkvpMessage,
}

impl Key {
    pub fn new(akvp: AkvpMessage) -> Self {
        Key { akvp }
    }

    pub fn execute(&self) -> Vec<u8> {
        println!("{}", self.akvp.command);

        Vec::new()
    }
}
