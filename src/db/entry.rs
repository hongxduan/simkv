//! Entry type implement
//!
//! author: Duan HongXing
//! date: 6 Apr, 2025
//!
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum EntryType {
    STR,
    MAP,
    LST,
    SET,
}

#[derive(Debug, Clone)]
pub struct Entry {
    pub etype: EntryType,
    pub data: EntryData,
    //pub byt: Option<Vec<u8>>,                  // String
    //pub map: Option<HashMap<String, Vec<u8>>>, // Map
    //pub lst: Option<Vec<Vec<u8>>>,             // List
}

#[derive(Debug, Clone)]
pub enum EntryData {
    Byt(Vec<u8>),
    Map(HashMap<String, Vec<u8>>),
    Lst(Vec<Vec<u8>>),
}
