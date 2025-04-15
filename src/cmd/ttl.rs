//! Ttl command implement
//!
//! author: Duan HongXing
//! date: 4 Apr, 2025
//!
//! Command:
//!     `ttl key`
//!
//! Examples:
//!    ttl k1
//!
//! Return:
//!    -2: expired
//!    -1: never expire
//!    >0: seconds to expire
//!
//!
use crate::{kvtp::kvtp::KvtpMessage, db::db::Db};

use super::base::BaseCommand;

pub struct Ttl {
    kvtp: KvtpMessage,
}

impl BaseCommand for Ttl {
    fn new(kvtp: KvtpMessage) -> Self {
        Ttl { kvtp }
    }

    fn execute(self, db: &Db) -> Vec<u8> {
        println!("set::execute {}", self.kvtp.command);
        // TODO: parse key to determine data type
        println!(
            "{}{}{}{}{:?}",
            self.kvtp.command, self.kvtp.key, self.kvtp.args, self.kvtp.ttl, self.kvtp.body
        );
        "Ok".as_bytes().to_vec()
    }
}
