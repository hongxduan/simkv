//! Del command implement
//!
//! author: Duan HongXing
//! date: 4 Apr, 2025
//!
use crate::{akvp::kvtp::KvtpMessage, cmd::base::INVALID_KEY_FMT, db::db::Db};

use super::base::{BaseCommand, KEY_NOT_FOUND, OK};

pub struct Del {
    kvtp: KvtpMessage,
}

impl Del {}

impl BaseCommand for Del {
    fn new(kvtp: KvtpMessage) -> Self {
        Del { kvtp }
    }

    fn execute(self, db: &Db) -> Vec<u8> {
        let key_info = self.parse_key(&self.kvtp.key);
        match key_info {
            Ok(ki) => {
                let del_result = db.del(ki.key);
                match del_result {
                    Some(_) => OK.to_vec(),
                    None => KEY_NOT_FOUND.to_vec(),
                }
            }
            Err(e) => {
                println!("{:?}", e);
                INVALID_KEY_FMT.to_vec()
            }
        }
    }
}
