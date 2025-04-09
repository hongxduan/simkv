///! Set implement
///!
///! author: Duan HongXing
///! date: 4 Apr, 2025
///!
use crate::{
    akvp::kvtp::KvtpMessage,
    bucket::{db::Db, entry::Entry},
};


#[derive(Debug)]
pub struct Set {
    kvtp: KvtpMessage,
}

impl Set {
    pub fn new(kvtp: KvtpMessage) -> Self {
        Set { kvtp }
    }

    pub fn execute(self, db: &Db) -> Vec<u8> {
        println!("set::execute {}", self.kvtp.command);
        // TODO: parse key to determine data type
        let entry = Entry {
            vtype: crate::bucket::entry::EntryType::STR,
            byt: Some(self.kvtp.body.clone()),
            map: None,
            lst: None,
        };
        db.set(self.kvtp.key.clone(), entry);
        "Ok".as_bytes().to_vec()
    }
}
