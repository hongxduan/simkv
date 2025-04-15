//! Trait BaseCommand
//!
//! author: Duan HongXing
//! date: 4 Apr, 2025
//!
use core::fmt;

use crate::{
    kvtp::kvtp::KvtpMessage,
    db::{db::Db, entry::EntryType},
};
use regex::Regex;

pub const OK: &[u8] = "Ok".as_bytes();
pub const KEY_NOT_EX: &[u8] = "KEY_NOT_EX".as_bytes();
pub const INV_KEY_FMT: &[u8] = "INV_KEY_FMT".as_bytes();
pub const INV_IDX: &[u8] = "INV_IDX".as_bytes(); // for list
pub const INV_TYP: &[u8] = "INV_TYP".as_bytes();
pub const INV_SUB_KEY_FMT: &[u8] = "INV_SUB_KEY_FMT".as_bytes();

// users[1]
// users[1..5]
// users[#]
// users[$1]
// users[&tom]
const LST_KEY_PATTERN: &str =
    r"(?<key>.+)\[(?<skey>(-?[0-9]+|-?[0-9]+\.\.-?[0-9]+|#|\$-?[0-9]+|\&[\[\]]+)+)\]$";
//
const MAP_KEY_PATTERN: &str = r"(?<key>.+)\{(?<skey>[^\{\}]+)\}$";
//
const SET_KEY_PATTERN: &str = r"(?<key>.+)<(?<skey>[^<>]+)>$";

// Pattern to match Numbers only
pub const PATTERN_NUMBER: &str = r"^-?[0-9]+$";

pub struct KeyInfo {
    pub entry_type: EntryType,
    pub key: String,  // The key
    pub skey: String, // The sub key, like `name` in user001{name} of map, like `1` in users[1] of list
}

pub struct KeyError;

impl fmt::Display for KeyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid key format!") // user-facing output
    }
}

impl fmt::Debug for KeyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ file: {}, line: {} }}", file!(), line!()) // programmer-facing output
    }
}

pub trait BaseCommand {
    fn new(kvtp: KvtpMessage) -> Self;
    fn execute(self, db: &Db) -> Vec<u8>;

    ///
    /// To parse the orignal key
    ///
    /// Return:
    ///     KeyInfo
    ///
    fn parse_key(&self, key: &str) -> Result<KeyInfo, KeyError> {
        let re_lst = Regex::new(LST_KEY_PATTERN).unwrap();
        let re_map = Regex::new(MAP_KEY_PATTERN).unwrap();
        let re_set = Regex::new(SET_KEY_PATTERN).unwrap();

        if re_lst.is_match(key) {
            match re_lst.captures(key) {
                Some(caps) => {
                    println!(
                        "BaseCommand::parse_key: lst: {},{}",
                        &caps["key"], &caps["skey"]
                    );
                    Ok(KeyInfo {
                        entry_type: EntryType::LST,
                        key: caps["key"].to_string(),
                        skey: caps["skey"].to_string(),
                    })
                }
                _ => Err(KeyError),
            }
        } else if re_map.is_match(key) {
            match re_map.captures(key) {
                Some(caps) => {
                    println!(
                        "BaseCommand::parse_key: map: {},{}",
                        &caps["key"], &caps["skey"]
                    );
                    Ok(KeyInfo {
                        entry_type: EntryType::MAP,
                        key: caps["key"].to_string(),
                        skey: caps["skey"].to_string(),
                    })
                }
                _ => Err(KeyError),
            }
        } else if re_set.is_match(key) {
            match re_set.captures(key) {
                Some(caps) => {
                    println!(
                        "BaseCommand::parse_key: set: {},{}",
                        &caps["key"], &caps["skey"]
                    );
                    Ok(KeyInfo {
                        entry_type: EntryType::SET,
                        key: caps["key"].to_string(),
                        skey: caps["skey"].to_string(),
                    })
                }
                _ => Err(KeyError),
            }
        } else {
            println!("BaseCommand::parse_key: str: {}", key);
            Ok(KeyInfo {
                entry_type: EntryType::STR,
                key: key.to_string(),
                skey: "".to_string(),
            })
        }
    }
}
