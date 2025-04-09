//
// author: Duan HongXing
// date: 4 Apr, 2025
//
// Get value by Key

use crate::{
    akvp::kvtp::{self, KvtpMessage},
    bucket::{bucket::Bucket, db::Db},
};

pub struct Get {
    kvtp: KvtpMessage,
}

impl Get {
    pub fn new(kvtp: KvtpMessage) -> Self {
        Get { kvtp }
    }

    pub fn execute(&self, db: &Db) -> Vec<u8> {
        //println!("get::execute {}", self.kvtp.command);
        let entry = db.get(self.kvtp.key.clone());
        match entry {
            Some(val) => val.byt.unwrap(),
            None => "nil".as_bytes().to_vec(),
        }
    }
}
