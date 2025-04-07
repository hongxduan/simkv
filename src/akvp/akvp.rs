use std::i32;

///! Advance Key-Value Protocol implement
///!
///! author: Duan HongXing
///! date: 5 Apr, 2025
///!

const PROTOCOL: &str = "AKVP/1";
const CMD_PREFIX: &str = "CMD: ";
const KEY_PREFIX: &str = "KEY: ";
const ARGS_PREFIX: &str = "ARGS: ";
const TTL_PREFIX: &str = "TTL: ";
const SPACE_CHAR: u8 = b' ';

pub struct AkvpMessage {
    protocol: String,
    pub command: String,
    key: String,
    args: String,
    ttl: i32,

    body: Vec<u8>,
}

impl AkvpMessage {
    pub fn parse(message: &Vec<u8>) -> Result<Self, String> {
        println!("akvp::parse {:?}", message);

        let mut err_msg = String::new();

        // psudo code
        let mut protocol = String::new();
        let mut command = String::new();
        let mut key = String::new();
        let mut args = String::new();
        let mut s_ttl = String::new();
        let mut ttl: i32 = 0;
        let mut body: Vec<u8> = Vec::new();

        let lines = message.split(|&b| b == b'\n');

        let mut header_len = 0;

        for (i, line) in lines.enumerate() {
            let sline = String::from_utf8(line.to_vec());

            match sline {
                Ok(l) => {
                    header_len += l.len();
                    header_len += 1; // line feed

                    // Check if the first empty line, then body started
                    if l == String::from("") {
                        break;
                    }

                    if i == 0 {
                        protocol = String::from_utf8(line.to_vec()).unwrap();
                    } else {
                        // Split line
                        let mut parts = l.split(":");
                        // Get the first part separated by colon
                        let fst_part = parts.next();
                        // Get the second part separated by colon
                        let sec_part = parts.next();
                        match fst_part {
                            Some(first) => match first {
                                CMD_PREFIX => match sec_part {
                                    Some(second) => command = second.trim().to_string(),
                                    None => {
                                        err_msg = String::from("Invalid command");
                                        break;
                                    }
                                },
                                KEY_PREFIX => match sec_part {
                                    Some(second) => key = second.trim().to_string(),
                                    None => {
                                        err_msg = String::from("Invalid key");
                                        break;
                                    }
                                },
                                ARGS_PREFIX => match sec_part {
                                    Some(second) => args = second.trim().to_string(),
                                    None => {
                                        err_msg = String::from("Invalid args");
                                        break;
                                    }
                                },
                                TTL_PREFIX => {
                                    match sec_part {
                                        Some(second) => s_ttl = second.trim().to_string(),
                                        None => {
                                            err_msg = String::from("Invalid ttl");
                                            break;
                                        }
                                    }
                                    let tmp = s_ttl.parse::<i32>();
                                    match tmp {
                                        Ok(i) => ttl = i,
                                        Err(_) => {
                                            err_msg = String::from("Invalid ttl");
                                            break;
                                        }
                                    }
                                }
                                _ => {}
                            },
                            None => {}
                        }
                    }
                }
                Err(e) => {}
            }
        }

        //let tmp:Vec<_> = message.iter().skip(header_len).take(10).collect();
        println!("header_len:{}/{}", header_len, &message.len());
        println!("{:?}", String::from_utf8(message.to_vec()));

        //TO FIX: 
        let tmp = message.to_vec();
        body = tmp[header_len..tmp.len()].to_vec();

        if err_msg.len() != 0 {
            return Err(err_msg);
        }
        let akvp_msg = AkvpMessage {
            protocol,
            command,
            key,
            args,
            ttl,
            body,
        };

        Ok(akvp_msg)
    }
}
