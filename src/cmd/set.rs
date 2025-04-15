//! Set command implement
//!
//! author: Duan HongXing
//! date: 4 Apr, 2025
//!

use crate::{
    kvtp::kvtp::KvtpMessage,
    db::{db::Db, entry::EntryType},
};

use super::{
    base::BaseCommand, lst_set::LstSet, map_set::MapSet, set_set::SetSet, str_set::StrSet,
};

#[derive(Debug)]
pub struct Set {
    kvtp: KvtpMessage,
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
                EntryType::STR => StrSet::set(self.kvtp, ki, db),
                EntryType::LST => LstSet::set(self.kvtp, ki, db),
                EntryType::MAP => MapSet::set(self.kvtp, ki, db),
                EntryType::SET => SetSet::set(self.kvtp, ki, db),
            },
            Err(e) => e.to_string().as_bytes().to_vec(),
        }
    }
}
