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
        println!("{}", self.akvp.command);
        Vec::new()
    }
}
