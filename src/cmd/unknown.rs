//
// author: Duan HongXing
// date: 6 Apr, 2025
//

use crate::akvp::{self, akvp::AkvpMessage};

pub struct Unknown {
    akvp: AkvpMessage,
}

impl Unknown {
    pub fn new(akvp: AkvpMessage) -> Self {
        Unknown { akvp }
    }

    pub fn execute(&self) -> Vec<u8> {
        println!("{}", self.akvp.command);
        Vec::new()
    }
}
