///! Entry type implement
///!
///! author: Duan HongXing
///! date: 6 Apr, 2025
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum EntryType {
    STR,
    MAP,
    LST,
    SET,
}

#[derive(Debug, Clone)]
pub struct Entry {
    pub vtype: EntryType,
    pub byt: Option<Vec<u8>>,                  // String
    pub map: Option<HashMap<String, Vec<u8>>>, // Map
    pub lst: Option<Vec<u8>>,                  // List
}
