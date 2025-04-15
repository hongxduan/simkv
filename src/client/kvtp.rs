//! Cli module
//!
//! author: Duan HongXing
//! date: 7 Apr, 2025
//!
use std::io::{Write, stdout};

use crate::client::client::InputData;

const LINE_FEED: u8 = b'\n'; // 0xA

const PROTOCOL_BYTES: &[u8] = "KVTP/1".as_bytes();
const KEY_PREFIX_BYTES: &[u8] = "KEY: ".as_bytes();
const CMD_PREFIX_BYTES: &[u8] = "CMD: ".as_bytes();
const ARGS_PREFIX_BYTES: &[u8] = "ARGS: ".as_bytes();
const TTL_PREFIX_BYTES: &[u8] = "TTL: ".as_bytes();

///
///
///
pub fn build_kvtp_message(input_data: InputData) -> Vec<u8> {
    let mut message: Vec<u8> = Vec::new();
    //
    // Protocol line
    for b in PROTOCOL_BYTES {
        message.push(*b);
    }
    message.push(LINE_FEED);

    //
    // Command line
    for b in CMD_PREFIX_BYTES {
        message.push(*b);
    }
    for b in input_data.cmd.as_bytes() {
        message.push(*b);
    }
    message.push(LINE_FEED);

    //
    // Key line
    if input_data.key.len() > 0 {
        for b in KEY_PREFIX_BYTES {
            message.push(*b);
        }
        for b in input_data.key.as_bytes() {
            message.push(*b);
        }
        message.push(LINE_FEED);
    }

    //
    // Args line
    if input_data.args.len() > 0 {
        // Args line
        for b in ARGS_PREFIX_BYTES {
            message.push(*b);
        }
        // Combine args to a single string
        let mut args_str = String::new();
        for arg in input_data.args {
            args_str.push_str(arg.as_str());
        }
        // Append args
        for b in args_str.as_bytes() {
            message.push(*b);
        }
        message.push(LINE_FEED);
    }

    //
    // TTL line
    if input_data.ttl > 0 {
        // TTL line
        for b in TTL_PREFIX_BYTES {
            message.push(*b);
        }
        for b in input_data.ttl.to_string().as_bytes() {
            message.push(*b);
        }
        message.push(LINE_FEED);
    }

    //
    // Body Separator line
    //message.push(SPACE_CHAR);
    message.push(LINE_FEED);

    //
    // Body
    for b in input_data.value.as_bytes() {
        message.push(*b);
    }

    let mut akvp: Vec<u8> = Vec::new();
    let len = message.len();
    for b in len.to_be_bytes() {
        akvp.push(b);
    }
    akvp.extend(message);

    akvp
}

const DTYPE_I: &str = "I";
const DTYPE_L: &str = "L";
const DTYPE_D: &str = "D";
const DTYPE_S: &str = "S";
const DTYPE_LI: &str = "LI";
const DTYPE_LL: &str = "LL";
const DTYPE_LD: &str = "LD";
const DTYPE_LS: &str = "LS";
const DTYPE_M: &str = "M";

///
/// Parse response from server
///
pub fn parse_kvtp_response(response: Vec<u8>) {
    let result: String = String::new();

    let mut err_msg = String::new();

    // psudo code
    let mut prot_status = String::new();
    let mut dtype = String::new();

    //Split message by line feed
    let lines = response.split(|&b| b == LINE_FEED);

    let mut header_len = 0;

    for (i, l) in lines.enumerate() {
        let sline = String::from_utf8(l.to_vec());

        match sline {
            Ok(line) => {
                header_len += line.len();
                header_len += 1; // line feed

                // Check if the first empty line, then body started
                if line == String::from("") {
                    break;
                }

                // 1st line: Protocol and Status
                if i == 0 {
                    prot_status = line;
                }
                // 2nd line: Data Type
                else if i == 1 {
                    dtype = line;
                } else {
                    // no more header line for now
                }
            }
            Err(_) => {}
        }
    }

    // Body
    let body = response[header_len..].to_vec();
    //println!("{:#?}", body);

    // Get Status
    let mut status = "";
    let pieces = prot_status.split(" ");
    //println!("{:?}", pieces);
    for (i, p) in pieces.enumerate() {
        if i == 0 {
            // protocol
        } else if i == 1 {
            status = p;
            //println!("{}:", status);
        } else {
            // this should never happen
            println!("unknown error");
            return;
        }
    }

    // Match Data Types
    match dtype.as_str() {
        DTYPE_I => {
            print_status(status, true);
            if body.len() == 4 {
                let arr: [u8; 4] = [body[0], body[1], body[2], body[3]];
                let i = i32::from_be_bytes(arr);
                println!("{}", i);
            } else {
                println!("unknown error");
            }
        }
        DTYPE_L => {}
        DTYPE_D => {}
        DTYPE_S => {
            print_status(status, true);
            let s = String::from_utf8(body);
            println!("{}", s.unwrap());
        }
        DTYPE_LI => {}
        DTYPE_LL => {}
        DTYPE_LD => {}
        DTYPE_LS => {}
        DTYPE_M => {}
        _ => {}
    }

    stdout().flush().unwrap();
}

fn print_status(status: &str, inline: bool) {
    if status == "OK" {
        if inline {
            print!("{}", '\u{2705}');
        } else {
            println!("{}", '\u{2705}');
        }
    } else {
        if inline {
            print!("{}", '\u{274C}');
        } else {
            println!("{}", '\u{274C}');
        }
    }
}
