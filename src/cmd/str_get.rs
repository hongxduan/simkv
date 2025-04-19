//! String GET implement
//!
//! author: Duan HongXing
//! date: 14 Apr, 2025
//!

use crate::{
    db::{db::Db, entry::EntryData},
    kvtp::{kvtp::KvtpMessage, response::KvtpResponse},
};

use super::{INV_TYP, KEY_NOT_EX, base_db::KeyInfo};

pub struct StrGet;

impl StrGet {
    pub fn get(_: KvtpMessage, ki: KeyInfo, db: &Db) -> Vec<u8> {
        let entry = db.get(ki.key);
        match entry {
            Some(entry) => match entry.data {
                EntryData::Byt(val) => {
                    return KvtpResponse::build_string(val);
                }
                _ => {
                    return KvtpResponse::build_string(INV_TYP.to_vec());
                }
            },
            None => KvtpResponse::build_err(KEY_NOT_EX.to_vec()),
        }
    }
}
