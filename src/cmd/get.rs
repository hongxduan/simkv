//! Get command implement
//!
//! author: Duan HongXing
//! date: 4 Apr, 2025
//!
//! Get value by Key
use tokio::time::Instant;

use crate::{
    cmd::INV_KEY_FMT, db::{db::Db, entry::EntryType}, kvtp::{kvtp::KvtpMessage, response::KvtpResponse}
};

use super::{
    base_db::{DbCommand, KeyInfo, MINUS_1, MINUS_2},
    key,
    lst_get::LstGet,
    map_get::MapGet,
    set_get::SetGet,
    str_get::StrGet, KEY_NOT_EX,
};

pub struct Get {
    kvtp: KvtpMessage,
}

impl Get {
    fn get_ttl(self, ki: KeyInfo, db: &Db) -> Vec<u8> {
        let entry_opt = db.get(ki.key);
        match entry_opt {
            Some(entry) => {
                if let Some(when) = entry.expire_at {
                    // Store now, so that *no* gap between compare and calculate
                    let now = Instant::now();
                    if when.gt(&now) {
                        let ttl = when - now;
                        return KvtpResponse::build_integer(ttl.as_secs() as i32);
                    }
                    return KvtpResponse::build_integer(MINUS_2);
                } else {
                    // Minus 1: never expire
                    return KvtpResponse::build_integer(MINUS_1);
                }
            }
            None => {
                return KvtpResponse::build_err(KEY_NOT_EX.to_vec());
            }
        }
    }
}

///
///
///
impl DbCommand for Get {
    fn new(kvtp: KvtpMessage) -> Self {
        Get { kvtp }
    }

    fn execute(self, db: &Db) -> Vec<u8> {
        //println!("get::execute {}", self.kvtp.command);
        let key_info = self.parse_key(&self.kvtp.key);

        // Handle get ttl
        if self.kvtp.ttl == -3 {
            match key_info {
                Ok(ki) => {
                    return self.get_ttl(ki, db);
                }
                Err(e) => {
                    println!("GET::execute{:?}", e);
                    return KvtpResponse::build_err(INV_KEY_FMT.to_vec());
                }
            }
        }

        // Handle get value
        match key_info {
            Ok(ki) => match ki.entry_type {
                EntryType::STR => StrGet::get(self.kvtp, ki, db),
                EntryType::LST => LstGet::get(self.kvtp, ki, db),
                EntryType::MAP => MapGet::get(self.kvtp, ki, db),
                EntryType::SET => SetGet::get(self.kvtp, ki, db),
            },
            Err(e) => {
                println!("GET::execute{:?}", e);
                return KvtpResponse::build_err(INV_KEY_FMT.to_vec());
            }
        }
    }
}
