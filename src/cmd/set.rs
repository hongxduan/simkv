//
// author: Duan HongXing
// date: 4 Apr, 2025
//

use crate::akvp::akvp::AkvpMessage;

pub struct Set {
    akvp: AkvpMessage,
}

impl Set {
    pub fn new(akvp: AkvpMessage) -> Self {
        Set { akvp }
    }

    pub fn execute(self) -> Vec<u8> {
        println!("set::execute {}", self.akvp.command);
        Vec::new()
    }
}
