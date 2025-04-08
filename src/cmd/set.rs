///! Set implement
///! 
///! author: Duan HongXing
///! date: 4 Apr, 2025
///!

use crate::akvp::kvtp::KvtpMessage;

pub struct Set {
    akvp: KvtpMessage,
}

impl Set {
    pub fn new(akvp: KvtpMessage) -> Self {
        // TODO: parse key to determine data type
        println!(
            "{}{}{}{}{:?}",
            akvp.command, akvp.key, akvp.args, akvp.ttl, akvp.body
        );
        Set { akvp }
    }

    pub fn execute(self) -> Vec<u8> {
        println!("set::execute {}", self.akvp.command);
        Vec::new()
    }
}
