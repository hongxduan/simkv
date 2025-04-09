///! Set command implement
///!
///! author: Duan HongXing
///! date: 4 Apr, 2025
///!
use crate::{
    akvp::kvtp::KvtpMessage,
    db::{db::Db, entry::Entry},
};

use super::base::BaseCommand;

#[derive(Debug)]
pub struct Set {
    kvtp: KvtpMessage,
}

impl BaseCommand for Set {
    fn new(kvtp: KvtpMessage) -> Self {
        Set { kvtp }
    }

    fn execute(self, db: &Db) -> Vec<u8> {
        println!("set::execute {}", self.kvtp.command);
        // TODO: parse key to determine data type
        let entry = Entry {
            vtype: crate::db::entry::EntryType::STR,
            byt: Some(self.kvtp.body.clone()),
            map: None,
            lst: None,
        };
        db.set(self.kvtp.key.clone(), entry);
        "Ok".as_bytes().to_vec()
    }
}
