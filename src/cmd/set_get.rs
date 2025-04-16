//! Set GET implement
//!
//! author: Duan HongXing
//! date: 14 Apr, 2025
//!

use crate::{db::db::Db, kvtp::kvtp::KvtpMessage};

use super::base::{KeyInfo, OK};

pub struct SetGet;

impl SetGet {
    pub fn get(kvtp: KvtpMessage, ki: KeyInfo, db: &Db) -> Vec<u8> {
        OK.to_vec()
    }
}