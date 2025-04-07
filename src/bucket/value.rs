///! Value type implement
///!
///! author: Duan HongXing
///! date: 6 Apr, 2025
use std::collections::HashMap;
pub enum ValueType {
    STR,
    MAP,
    LST,
    SET,
}

pub struct Value {
    vtype: ValueType,
    str: Option<String>,
    map: Option<HashMap<String, Vec<u8>>>,
    lst: Option<Vec<u8>>,
}
