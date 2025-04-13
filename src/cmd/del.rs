//! Del command implement
//!
//! author: Duan HongXing
//! date: 4 Apr, 2025
//!
use crate::{akvp::kvtp::KvtpMessage, cmd::base::INV_KEY_FMT, db::db::Db};

use super::base::{BaseCommand, KEY_NOT_EX, OK};

pub struct Del {
    kvtp: KvtpMessage,
}

impl Del {}

impl BaseCommand for Del {
    fn new(kvtp: KvtpMessage) -> Self {
        Del { kvtp }
    }

    /// Execute delete
    ///
    /// Return:
    /// - Ok: if the key deleted
    /// - KEY_NOT_FOUND: if the entry to be deleted is not exsist
    /// - INVALID_KEY: if invalid key format
    ///
    fn execute(self, db: &Db) -> Vec<u8> {
        let key_info = self.parse_key(&self.kvtp.key);
        match key_info {
            Ok(ki) => {
                let del_result = db.del(ki.key);
                match del_result {
                    Some(_) => OK.to_vec(),
                    None => KEY_NOT_EX.to_vec(),
                }
            }
            Err(e) => {
                println!("{:?}", e);
                INV_KEY_FMT.to_vec()
            }
        }
    }
}
