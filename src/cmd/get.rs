//
// author: Duan HongXing
// date: 4 Apr, 2025
//
// Get value by Key

use crate::akvp::{self, akvp::AkvpMessage};

pub struct Get {
    akvp: AkvpMessage,
}

impl Get {
    pub fn new(akvp: AkvpMessage) -> Self {
        Get { akvp }
    }

    pub fn execute(&self) -> Vec<u8> {
        println!("get::execute {}", self.akvp.command);
        let mut result: Vec<u8> = Vec::new();
        result.push(b'H');
        result.push(b'o');
        result
    }
}
