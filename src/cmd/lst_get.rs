//! List GET Implement
//!
//! author: Duan HongXing
//! date: 14 Apr, 2025
//!

use std::collections::LinkedList;

use regex::Regex;

use crate::{
    cmd::base::INV_TYP,
    db::{db::Db, entry::EntryData},
    kvtp::{kvtp::KvtpMessage, response::KvtpResponse},
};

use super::base::{INV_IDX, INV_SUB_KEY_FMT, KEY_NOT_EX, KeyError, KeyInfo, PATTERN_NUMBER};

pub struct LstGet;

impl LstGet {
    pub fn get(_: KvtpMessage, ki: KeyInfo, db: &Db) -> Vec<u8> {
        let entry_opt = db.get(ki.key);
        // TODO: the skey can be not number
        //let idx_result = ki.skey.parse::<isize>();
        let lst_skey_result = LstGetSubKey::parse(&ki.skey);
        match lst_skey_result {
            Ok(lsk) => {
                match entry_opt {
                    Some(entry) => match entry.data {
                        EntryData::Lst(mut l) => match lsk {
                            LstGetSubKey::Number(idx) => {
                                return get_by_index(&mut l, idx);
                            }
                            // Find value index in the list
                            LstGetSubKey::Ampersand(value) => {
                                let pos_result = l.iter().position(|e| *e == value);
                                match pos_result {
                                    Some(idx) => return KvtpResponse::build_integer(idx as i32),
                                    // Return -1 if element not found
                                    None => return KvtpResponse::build_integer(-1),
                                }
                            }
                            // Get list length
                            LstGetSubKey::Hash(()) => {
                                let len = l.len();
                                return KvtpResponse::build_integer(len as i32);
                            }
                            // Get all list elements in the range
                            LstGetSubKey::Range((start, end)) => {
                                return get_range(&mut l, start, end);
                                //return KvtpResponse::build_err(INV_IDX.to_vec());
                            }
                        },
                        t => {
                            println!("{:?}", t);
                            return KvtpResponse::build_err(INV_TYP.to_vec());
                        }
                    },
                    None => KvtpResponse::build_err(KEY_NOT_EX.to_vec()),
                }
            }
            Err(e) => {
                println!("{}", e);
                return KvtpResponse::build_err(INV_SUB_KEY_FMT.to_vec());
            }
        }
    }
}

fn get_by_index(l: &mut LinkedList<Vec<u8>>, idx: i32) -> Vec<u8> {
    //println!("get_lst- {}, {}", idx, l.len());
    if idx == -1 {
        let entry_opt = l.pop_back();
        match entry_opt {
            Some(v) => {
                return KvtpResponse::build_string(v);
            }
            None => {
                return KvtpResponse::build_err(INV_IDX.to_vec());
            }
        }
    } else if idx == 0 {
        let entry_opt = l.pop_front();
        match entry_opt {
            Some(v) => {
                return KvtpResponse::build_string(v);
            }
            None => {
                return KvtpResponse::build_err(INV_IDX.to_vec());
            }
        }
    } else if idx > 0 {
        let udx = idx as usize;
        if udx > l.len() {
            return KvtpResponse::build_err(INV_IDX.to_vec());
        } else {
            let mut tail = l.split_off(udx);
            let result_opt = tail.pop_front();
            l.append(&mut tail);
            match result_opt {
                Some(v) => {
                    return KvtpResponse::build_string(v);
                }
                None => {
                    return KvtpResponse::build_err(INV_IDX.to_vec());
                }
            }
        }
    } else {
        return KvtpResponse::build_err(INV_IDX.to_vec());
    }
}

fn get_range(l: &mut LinkedList<Vec<u8>>, start: i32, end: i32) -> Vec<u8> {
    let mut values: Vec<Vec<u8>> = Vec::new();

    let mut start_u: usize = 0;
    let mut end_u: usize = 0;

    // TODO: convert start, end
    start_u = start as usize;
    end_u = end as usize;

    println!("{},{}", start_u, end_u);

    let mut middle = l.split_off(start_u);
    // Because len of middle f.len() - start_u, so the split must orginal end_u minus start_u
    let mut tail = middle.split_off(end_u - start_u);

    for (_, value) in middle.iter().enumerate() {
        values.push(value.to_vec());
    }
    println!("{:?}", values);
    return KvtpResponse::build_list_string(values);
}

/*---------------------------------------------------------------------------*/

pub enum LstGetSubKey {
    Number(i32),        // [5]       purely number
    Range((i32, i32)),  // [1..5]    Range
    Ampersand(Vec<u8>), // [&tom]    Get index , Index(Address) of tom
    Hash(()),           // [#]       Get length
}

const PATTERN_RANGE: &str = r"(?<start>-?[0-9]+)\.\.(?<end>-?[0-9]+)";
const PATTERN_AMPERSAND: &str = r"^&(?<value>.+)";
//const PATTERN_HASH: &str = r"#";

///
///
///
impl LstGetSubKey {
    ///
    /// Parse List sub key, to determine what operation on the list
    ///
    pub fn parse(skey: &str) -> Result<LstGetSubKey, KeyError> {
        let re_number = Regex::new(PATTERN_NUMBER).unwrap();
        let re_range = Regex::new(PATTERN_RANGE).unwrap();
        let re_ampersand = Regex::new(PATTERN_AMPERSAND).unwrap();
        //let re_hash = Regex::new(PATTERN_HASH).unwrap();

        if re_number.is_match(skey) {
            Ok(LstGetSubKey::Number(skey.parse::<i32>().unwrap()))
        } else if re_range.is_match(skey) {
            match re_range.captures(skey) {
                Some(caps) => Ok(LstGetSubKey::Range((
                    caps["start"].parse::<i32>().unwrap(),
                    caps["end"].parse::<i32>().unwrap(),
                ))),
                _ => Err(KeyError),
            }
        } else if re_ampersand.is_match(skey) {
            match re_ampersand.captures(skey) {
                Some(caps) => Ok(LstGetSubKey::Ampersand(caps["value"].as_bytes().to_vec())),
                _ => Err(KeyError),
            }
        } else if skey == "#" {
            Ok(LstGetSubKey::Hash(()))
        } else {
            Err(KeyError)
        }
    }
}
