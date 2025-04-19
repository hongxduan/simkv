//! Base Map
//!
//! author: Duan HongXing
//! date: 14 Apr, 2025
//!

use crate::{kvtp::kvtp::KvtpMessage, db::db::Db};

use super::{base_db::KeyInfo, OK};

pub struct MapSet;

impl MapSet {
    pub fn set(kvtp: KvtpMessage, ki: KeyInfo, db: &Db) -> Vec<u8> {

        OK.to_vec()
    }
}