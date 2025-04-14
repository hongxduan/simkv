//! List SET Implement
//!
//! author: Duan HongXing
//! date: 14 Apr, 2025
//!
use std::collections::LinkedList;

use regex::Regex;

use crate::{
    akvp::kvtp::KvtpMessage,
    db::{
        db::Db,
        entry::{Entry, EntryData, EntryType},
    },
};

use super::base::{INV_IDX, INV_SUB_KEY_FMT, INV_TYP, KeyError, KeyInfo, OK};

pub struct LstSet;

impl LstSet {
    pub fn set(kvtp: KvtpMessage, ki: KeyInfo, db: &Db) -> Vec<u8> {
        let entry_opt = db.get(ki.key.clone());
        let lst_skey_result = LstSetSubKey::parse(&ki.skey);
        match lst_skey_result {
            Ok(lsk) => {
                match entry_opt {
                    // If entry exists already
                    Some(mut entry) => match entry.data {
                        EntryData::Lst(mut l) => {
                            match lsk {
                                // If purely Numbers in the bracket
                                // Push front or Push back base on the Number
                                // Or insert in the middle
                                LstSetSubKey::Number(idx) => {
                                    if idx == -1 {
                                        l.push_back(kvtp.body);
                                    } else if idx == 0 {
                                        l.push_front(kvtp.body);
                                    } else if idx >= 0 {
                                        let udx = idx as usize;
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
                                    db.set(ki.key, entry);
                                }
                                //
                                // Replace
                                LstSetSubKey::Dollar(idx) => {
                                    let mut udx: usize = 0;
                                    if idx < 0 && (idx.abs() as usize) <= l.len() {
                                        udx = ((l.len() as isize) + (idx as isize)) as usize;
                                    } else if idx >= 0 {
                                        udx = idx as usize;
                                    } else {
                                        // Error
                                        return INV_IDX.to_vec();
                                    }

                                    let mut tail = l.split_off(udx);
                                    let old = tail.pop_front(); // remove old
                                    l.push_back(kvtp.body); // append new
                                    l.append(&mut tail);

                                    entry.data = EntryData::Lst(l);
                                    db.set(ki.key, entry);
                                    return old.unwrap();
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
            }
            Err(e) => {
                return INV_SUB_KEY_FMT.to_vec();
            }
        }

        OK.to_vec()
    }
}

#[derive(Debug)]
pub enum LstSetSubKey {
    Number(i32), // [5]       purely number
    //Range((i32, i32)), // [1..5]    Range
    Dollar(i32), // [$5]      Replace at
                 //Ampersand(String), // [&tom]    Get index , Index(Address) of tom
                 //Hash(()),          // [#]       Get length
}

const PATTERN_NUMBER: &str = r"^[0-9]+$";
const PATTERN_DOLLAR: &str = r"^\$(?<index>[0-9]+$)";

///
/// Parse Sub Key in the bracket
/// 1. Purely numbers
/// 2. $numbers
///
impl LstSetSubKey {
    ///
    /// Parse List sub key, to determine what operation on the list
    ///
    pub fn parse(skey: &str) -> Result<LstSetSubKey, KeyError> {
        let re_number = Regex::new(PATTERN_NUMBER).unwrap();
        let re_dollar = Regex::new(PATTERN_DOLLAR).unwrap();

        if re_number.is_match(skey) {
            Ok(LstSetSubKey::Number(skey.parse::<i32>().unwrap()))
        } else if re_dollar.is_match(skey) {
            match re_dollar.captures(skey) {
                Some(caps) => Ok(LstSetSubKey::Dollar(caps["index"].parse::<i32>().unwrap())),
                _ => Err(KeyError),
            }
        } else {
            Err(KeyError)
        }
    }
}
