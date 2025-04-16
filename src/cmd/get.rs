//! Get command implement
//!
//! author: Duan HongXing
//! date: 4 Apr, 2025
//!
//! Get value by Key
use crate::{
    db::{db::Db, entry::EntryType},
    kvtp::kvtp::KvtpMessage,
};

use super::{
    base::{BaseCommand, KeyInfo},
    lst_get::LstGet,
    map_get::MapGet,
    set_get::SetGet,
    str_get::StrGet,
};

pub struct Get {
    kvtp: KvtpMessage,
}

///
///
///
///
///
///
///
impl BaseCommand for Get {
    fn new(kvtp: KvtpMessage) -> Self {
        Get { kvtp }
    }

    fn execute(self, db: &Db) -> Vec<u8> {
        //println!("get::execute {}", self.kvtp.command);
        let key_info = self.parse_key(&self.kvtp.key);
        match key_info {
            Ok(ki) => match ki.entry_type {
                EntryType::STR => StrGet::get(self.kvtp, ki, db),
                EntryType::LST => LstGet::get(self.kvtp, ki, db),
                EntryType::MAP => MapGet::get(self.kvtp, ki, db),
                EntryType::SET => SetGet::get(self.kvtp, ki, db),
            },
            Err(e) => {
                return e.to_string().as_bytes().to_vec();
            }
        }
    }
}
