///! Set implement
///!
///! author: Duan HongXing
///! date: 4 Apr, 2025
///!
use crate::{akvp::kvtp::KvtpMessage, bucket::bucket::Bucket};

pub struct Ttl {
    kvtp: KvtpMessage,
}

impl Ttl {
    pub fn new(kvtp: KvtpMessage) -> Self {
        Ttl { kvtp }
    }

    pub fn execute(self, bucket: &Bucket) -> Vec<u8> {
        println!("set::execute {}", self.kvtp.command);
        // TODO: parse key to determine data type
        println!(
            "{}{}{}{}{:?}",
            self.kvtp.command, self.kvtp.key, self.kvtp.args, self.kvtp.ttl, self.kvtp.body
        );
        "Ok".as_bytes().to_vec()
    }
}
