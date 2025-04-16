//! KVTP Response Implement
//!
//! author: Duan HongXing
//! date: 15 Apr, 2025
//!

const LINE_FEED: u8 = b'\n'; // 0xA

const PROTOCOL_BYTES: &[u8] = "KVTP/1".as_bytes();
const OK_BYTES: &[u8] = "OK".as_bytes();
const ERR_BYTES: &[u8] = "ERR".as_bytes();

const DTYPE_I: &[u8] = "I".as_bytes();
const DTYPE_L: &[u8] = "L".as_bytes();
const DTYPE_D: &[u8] = "D".as_bytes();
const DTYPE_S: &[u8] = "S".as_bytes();
const DTYPE_LI: &[u8] = "LI".as_bytes();
const DTYPE_LL: &[u8] = "LL".as_bytes();
const DTYPE_LD: &[u8] = "LD".as_bytes();
const DTYPE_LS: &[u8] = "LS".as_bytes();
const DTYPE_M: &[u8] = "M".as_bytes();

pub struct KvtpResponse {
    pub protocol: String,
    pub status: String,
    pub dtype: String,

    pub body: Vec<u8>,
}

pub enum DType {
    I,  // Integer 32 bit
    L,  // Long 64 bit
    D,  // Double
    S,  // String
    LI, // List of Integer
    LL, // List of Long
    LD, // List of Double
    LS, // List of String
    M,  // Map
}

impl KvtpResponse {
    pub fn build_integer(i: i32) -> Vec<u8> {
        let mut response: Vec<u8> = Vec::new();

        // Protocol and Status
        Self::append_ok(&mut response);

        // Data Type
        let dt = &mut DTYPE_I.to_vec();
        response.append(dt);
        response.push(LINE_FEED);

        // Empty line
        response.push(LINE_FEED);

        // Body
        let mut iv = i.to_be_bytes().to_vec();
        response.append(&mut iv);

        return response;
    }

    pub fn build_long(l: i64) -> Vec<u8> {
        let mut response: Vec<u8> = Vec::new();

        Self::append_ok(&mut response);

        let mut lv = l.to_be_bytes().to_vec();
        response.append(&mut lv);
        return response;
    }

    pub fn build_double() -> Vec<u8> {
        let response: Vec<u8> = Vec::new();

        return response;
    }

    pub fn build_string(mut val: Vec<u8>) -> Vec<u8> {
        let mut response: Vec<u8> = Vec::new();

        // Protocol and Status
        Self::append_ok(&mut response);

        // Data Type
        let dt = &mut DTYPE_S.to_vec();
        response.append(dt);
        response.push(LINE_FEED);

        // Empty line
        response.push(LINE_FEED);

        response.append(&mut val);

        response
    }

    pub fn build_list_integer() -> Vec<u8> {
        let response: Vec<u8> = Vec::new();

        return response;
    }

    pub fn build_list_long() -> Vec<u8> {
        let response: Vec<u8> = Vec::new();

        return response;
    }

    pub fn build_list_double() -> Vec<u8> {
        let response: Vec<u8> = Vec::new();

        return response;
    }

    pub fn build_list_string(values: Vec<Vec<u8>>) -> Vec<u8> {
        let mut response: Vec<u8> = Vec::new();
        //1. Protocol and Status
        Self::append_ok(&mut response);

        //2. Data Type
        let dt = &mut DTYPE_LS.to_vec();
        response.append(dt);
        response.push(LINE_FEED);

        //3. Empty line
        response.push(LINE_FEED);

        //4. Body
        for mut item in values {
            // Make sure the length bytes fixed 4
            let len = item.len() as u32;
            let mut len_v = len.to_be_bytes().to_vec();
            response.append(&mut len_v);
            response.append(&mut item);
        }

        return response;
    }

    pub fn build_list_map() -> Vec<u8> {
        let response: Vec<u8> = Vec::new();

        return response;
    }

    ///
    /// Error is special String
    pub fn build_err(mut msg: Vec<u8>) -> Vec<u8> {
        let mut response: Vec<u8> = Vec::new();

        // Protocol and Status
        for b in PROTOCOL_BYTES {
            response.push(*b);
        }

        response.push(b' ');

        for b in ERR_BYTES {
            response.push(*b);
        }

        response.push(LINE_FEED);

        // Data Type
        let dt = &mut DTYPE_S.to_vec();
        response.append(dt);
        response.push(LINE_FEED);

        // Empty line
        response.push(LINE_FEED);

        // Error message
        response.append(&mut msg);

        return response;
    }

    fn append_ok(response: &mut Vec<u8>) {
        for b in PROTOCOL_BYTES {
            response.push(*b);
        }

        response.push(b' ');

        for b in OK_BYTES {
            response.push(*b);
        }

        response.push(LINE_FEED);
    }
}
