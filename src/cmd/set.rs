//! Set command implement
//!
//! author: Duan HongXing
//! date: 4 Apr, 2025
//!

use crate::{
    akvp::kvtp::KvtpMessage,
    db::{
        db::Db,
        entry::{Entry, EntryType},
    },
};

use super::base::{BaseCommand, KeyInfo};

#[derive(Debug)]
pub struct Set {
    kvtp: KvtpMessage,
}

impl Set {
    fn set_str(self, ki: KeyInfo, db: &Db) -> Vec<u8> {
        let entry = Entry {
            etype: crate::db::entry::EntryType::STR,
            byt: Some(self.kvtp.body.clone()),
            map: None,
            lst: None,
        };
        db.set(ki.key, entry);
        "Ok".as_bytes().to_vec()
    }
    fn set_lst(self, ki: KeyInfo, db: &Db) -> Vec<u8> {
        "Ok".as_bytes().to_vec()
    }
    fn set_map(self, ki: KeyInfo, db: &Db) -> Vec<u8> {
        "Ok".as_bytes().to_vec()
    }
    fn set_set(self, ki: KeyInfo, db: &Db) -> Vec<u8> {
        "Ok".as_bytes().to_vec()
    }
}

impl BaseCommand for Set {
    fn new(kvtp: KvtpMessage) -> Self {
        Set { kvtp }
    }

    fn execute(self, db: &Db) -> Vec<u8> {
        //println!("set::execute {}", self.kvtp.command);
        let key_info = self.parse_key(&self.kvtp.key);
        match key_info {
            Ok(ki) => match ki.entry_type {
                EntryType::STR => self.set_str(ki, db),
                EntryType::LST => self.set_lst(ki, db),
                EntryType::MAP => self.set_map(ki, db),
                EntryType::SET => self.set_set(ki, db),
            },
            Err(e) => e.to_string().as_bytes().to_vec(),
        }
    }
}
