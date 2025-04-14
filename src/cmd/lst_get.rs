//! List GET Implement
//!
//! author: Duan HongXing
//! date: 14 Apr, 2025
//!

use regex::Regex;

use super::base::KeyError;

pub enum LstGetSubKey {
    Number(i32),       // [5]       purely number
    Range((i32, i32)), // [1..5]    Range
    Ampersand(String), // [&tom]    Get index , Index(Address) of tom
    Hash(()),          // [#]       Get length
}

const PATTERN_NUMBER: &str = r"\d+";
const PATTERN_RANGE: &str = r"(?<start>\d+)\.\.(?<end>\d+)";
const PATTERN_AMPERSAND: &str = r"^&(?<value>[\[]]]+)";
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
            println!("LstSubKey::parse: number: {}", skey);
            Ok(LstGetSubKey::Number(skey.parse::<i32>().unwrap()))
        } else if re_range.is_match(skey) {
            match re_range.captures(skey) {
                Some(caps) => {
                    println!(
                        "LstSubKey::parse: range: {},{}",
                        &caps["start"], &caps["end"]
                    );
                    Ok(LstGetSubKey::Range((
                        caps["start"].parse::<i32>().unwrap(),
                        caps["end"].parse::<i32>().unwrap(),
                    )))
                }
                _ => Err(KeyError),
            }
        } else if re_ampersand.is_match(skey) {
            match re_ampersand.captures(skey) {
                Some(caps) => Ok(LstGetSubKey::Ampersand(caps["value"].to_string())),
                _ => Err(KeyError),
            }
        } else if skey == "#" {
            Ok(LstGetSubKey::Hash(()))
        } else {
            Err(KeyError)
        }
    }
}
