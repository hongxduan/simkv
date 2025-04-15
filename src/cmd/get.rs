//! Get command implement
//!
//! author: Duan HongXing
//! date: 4 Apr, 2025
//!
//! Get value by Key
use crate::{
    kvtp::kvtp::KvtpMessage,
    db::{db::Db, entry::EntryType},
};

use super::{
    base::{BaseCommand, KeyInfo},
    lst_get::LstGet,
    str_get::StrGet,
};

pub struct Get {
    kvtp: KvtpMessage,
}

impl Get {
    ///
    ///
    ///
    ///
    ///
    ///
    fn get_map(self, ki: KeyInfo, db: &Db) -> Vec<u8> {
        /*let entry = db.get(ki.key);
        match entry {
            Some(val) => {
                let hm = val.map.unwrap();
                hm.get(ki.skey.as_str()).unwrap().clone()
            }
            None => "nil".as_bytes().to_vec(),
        }*/
        "nil".as_bytes().to_vec()
    }

    ///
    ///
    ///
    ///
    ///
    ///
    ///
    fn get_set(self, ki: KeyInfo, db: &Db) -> Vec<u8> {
        /*let entry = db.get(ki.key);
        match entry {
            Some(val) => {
                let hm = val.map.unwrap();
                hm.get(self.kvtp.key.as_str()).unwrap().clone()
            }
            None => "nil".as_bytes().to_vec(),
        }*/
        "nil".as_bytes().to_vec()
    }
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
                EntryType::MAP => self.get_map(ki, db),
                EntryType::SET => self.get_set(ki, db),
            },
            Err(e) => {
                return e.to_string().as_bytes().to_vec();
            }
        }
    }
}
