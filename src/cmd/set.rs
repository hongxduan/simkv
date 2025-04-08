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
        Set { akvp }
    }

    pub fn execute(self) -> Vec<u8> {
        println!("set::execute {}", self.akvp.command);
        // TODO: parse key to determine data type
        println!(
            "{}{}{}{}{:?}",
            self.akvp.command, self.akvp.key, self.akvp.args, self.akvp.ttl, self.akvp.body
        );
        "Ok".as_bytes().to_vec()
    }
}
