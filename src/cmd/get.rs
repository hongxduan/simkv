//! Get command implement
//!
//! author: Duan HongXing
//! date: 4 Apr, 2025
//!
//! Get value by Key
use crate::{
    akvp::kvtp::KvtpMessage,
    cmd::base::INV_IDX,
    db::{
        db::Db,
        entry::{EntryData, EntryType},
    },
};

use super::base::{BaseCommand, INV_TYP, KEY_NOT_EX, KeyInfo};

pub struct Get {
    kvtp: KvtpMessage,
}

impl Get {
    fn get_str(self, ki: KeyInfo, db: &Db) -> Vec<u8> {
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

    ///
    /// Get first(POP Left)
    ///     `get users[0]`
    ///
    /// Get last(POP Right)
    ///     `get users[-1]`
    ///
    /// Get any
    ///     - Return the fifth entry
    ///     `get users[4]`
    /// Get range(Slice)
    ///     - Return entries from index 1 to 4
    ///     `get users[1:5]`
    ///     - Return entries from index 1 to last
    ///     `get users[1:-1]`
    ///
    ///
    fn get_lst(self, ki: KeyInfo, db: &Db) -> Vec<u8> {
        let entry = db.get(ki.key);
        let idx_result = ki.skey.parse::<isize>();
        match entry {
            Some(entry) => match entry.data {
                EntryData::Lst(l) => match idx_result {
                    Ok(idx) => {
                        let udx = idx as usize;
                        println!("udx: {}, len:{}", udx, l.len());
                        match l.get(udx) {
                            Some(val) => {
                                return val.to_vec();
                            }
                            None => {
                                println!("get_list");
                                return INV_IDX.to_vec();
                            }
                        }
                    }
                    Err(e) => {
                        println!("{}", e);
                        return INV_IDX.to_vec();
                    }
                },
                _ => {
                    return INV_TYP.to_vec();
                }
            },
            None => KEY_NOT_EX.to_vec(),
        }
    }

    fn get_map(self, ki: KeyInfo, db: &Db) -> Vec<u8> {
        /*let entry = db.get(ki.key);
        match entry {
            Some(val) => {
                let hm = val.map.unwrap();
                hm.get(ki.skey.as_str()).unwrap().clone()
            }
            None => "nil".as_bytes().to_vec(),
        }*/
        "nil".as_bytes().to_vec()
    }

    fn get_set(self, ki: KeyInfo, db: &Db) -> Vec<u8> {
        /*let entry = db.get(ki.key);
        match entry {
            Some(val) => {
                let hm = val.map.unwrap();
                hm.get(self.kvtp.key.as_str()).unwrap().clone()
            }
            None => "nil".as_bytes().to_vec(),
        }*/
        "nil".as_bytes().to_vec()
    }
}

impl BaseCommand for Get {
    fn new(kvtp: KvtpMessage) -> Self {
        Get { kvtp }
    }

    fn execute(self, db: &Db) -> Vec<u8> {
        //println!("get::execute {}", self.kvtp.command);
        let key_info = self.parse_key(&self.kvtp.key);
        match key_info {
            Ok(ki) => match ki.entry_type {
                EntryType::STR => self.get_str(ki, db),
                EntryType::LST => self.get_lst(ki, db),
                EntryType::MAP => self.get_map(ki, db),
                EntryType::SET => self.get_set(ki, db),
            },
            Err(e) => {
                return e.to_string().as_bytes().to_vec();
            }
        }
    }
}
