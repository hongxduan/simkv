//! String SET implement
//!
//! author: Duan HongXing
//! date: 14 Apr, 2025
//!

use crate::cmd::base::KeyInfo;
use crate::db::db::Db;
use crate::db::entry::{Entry, EntryData, EntryType};
use crate::kvtp::kvtp::KvtpMessage;
use crate::kvtp::response::KvtpResponse;

use super::base::OK;

pub struct StrSet;

impl StrSet {
    pub fn set(kvtp: KvtpMessage, ki: KeyInfo, db: &Db) -> Vec<u8> {
        //let ttl = kvtp.ttl_to_duration();
        let expire_at = kvtp.ttl_to_instant();
        let entry = Entry {
            etype: EntryType::STR,
            data: EntryData::Byt(kvtp.body),
            expire_at,
        };
        db.set(ki.key, entry);
        KvtpResponse::build_string(OK.to_vec())
    }
}
