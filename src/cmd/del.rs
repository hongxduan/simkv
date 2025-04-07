//
// author: Duan HongXing
// date: 4 Apr, 2025
//

use crate::akvp::akvp::AkvpMessage;

pub struct Del {
    akvp: AkvpMessage,
}

impl Del {
    pub fn new(akvp: AkvpMessage) -> Self {
        Del { akvp }
    }

    pub fn execute(&self) -> Vec<u8> {
        println!("{}", self.akvp.command);
        Vec::new()
    }
}
