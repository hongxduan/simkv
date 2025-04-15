//! String GET implement
//!
//! author: Duan HongXing
//! date: 14 Apr, 2025
//!

use crate::{
    kvtp::kvtp::KvtpMessage,
    db::{db::Db, entry::EntryData},
};

use super::base::{INV_TYP, KEY_NOT_EX, KeyInfo};

pub struct StrGet;

impl StrGet {
    pub fn get(_: KvtpMessage, ki: KeyInfo, db: &Db) -> Vec<u8> {
        let entry = db.get(ki.key);
        match entry {
            Some(entry) => match entry.data {
                EntryData::Byt(val) => {
                    return val;
                }
                _ => {
                    return INV_TYP.to_vec();
                }
            },
            None => KEY_NOT_EX.to_vec(),
        }
    }
}
