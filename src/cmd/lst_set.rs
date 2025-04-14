//! List SET Implement
//!
//! author: Duan HongXing
//! date: 14 Apr, 2025
//!

use std::collections::LinkedList;

use crate::{
    akvp::kvtp::KvtpMessage,
    db::{
        db::Db,
        entry::{Entry, EntryData, EntryType},
    },
};

use super::base::{INV_IDX, INV_TYP, KeyInfo, OK};

pub struct LstSet;

impl LstSet {
    pub fn set(kvtp: KvtpMessage, ki: KeyInfo, db: &Db) -> Vec<u8> {
        let idx_result = ki.skey.parse::<isize>();
        let entry_opt = db.get(ki.key.clone());
        match entry_opt {
            // If entry exists already
            Some(mut entry) => match entry.data {
                EntryData::Lst(mut l) => {
                    match idx_result {
                        Ok(idx) => {
                            if idx == -1 {
                                l.push_back(kvtp.body);
                            } else if idx == 0 {
                                l.push_front(kvtp.body);
                            } else if idx >= 0 {
                                let udx = idx as usize;
                                println!("udx:{}", udx);
                                if udx >= l.len() {
                                    l.push_back(kvtp.body);
                                } else {
                                    // Insert in the middle
                                    let mut tail = l.split_off(udx);
                                    l.push_back(kvtp.body);
                                    l.append(&mut tail);
                                }
                            } else {
                                return INV_IDX.to_vec();
                            }

                            // Need set back, or else the set not works
                            entry.data = EntryData::Lst(l);
                            db.set(ki.key.clone(), entry);
                        }
                        Err(e) => {
                            println!("{}", e);
                            return INV_IDX.to_vec();
                        }
                    }
                }
                _ => {
                    return INV_TYP.to_vec();
                }
            },
            // New List
            None => {
                let mut l: LinkedList<Vec<u8>> = LinkedList::new();
                l.push_front(kvtp.body);
                let entry = Entry {
                    etype: EntryType::LST,
                    data: EntryData::Lst(l),
                };

                db.set(ki.key, entry);
            }
        }
        OK.to_vec()
    }
}
