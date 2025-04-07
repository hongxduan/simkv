///! Cli module
///!
///! author: Duan HongXing
///! date: 7 Apr, 2025
///!
use crate::client::client::InputData;

const LINE_FEED: u8 = b'\n'; // 0xA

const PROTOCOL_BYTES: &[u8] = "AKVP/1".as_bytes();
const KEY_PREFIX_BYTES: &[u8] = "KEY: ".as_bytes();
const CMD_PREFIX_BYTES: &[u8] = "CMD: ".as_bytes();
const ARGS_PREFIX_BYTES: &[u8] = "ARGS: ".as_bytes();
const TTL_PREFIX_BYTES: &[u8] = "TTL: ".as_bytes();
//const SPACE_CHAR: u8 = b' ';

///
///
///
pub fn build_akvp_message(input_data: InputData) -> Vec<u8> {
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

///
///
///
pub fn parse_akvp_response(buf: Vec<u8>) -> String {
    let mut result: String = String::new();

    result
}
