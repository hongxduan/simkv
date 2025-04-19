//! Advance Key-Value Protocol implement
//!
//! author: Duan HongXing
//! date: 5 Apr, 2025
//!

use std::time::Duration;

use tokio::time::Instant;
const CMD_LINE_HEAD: &str = "CMD";
const KEY_LINE_HEAD: &str = "KEY";
const ARGS_LINE_HEAD: &str = "ARGS";
const TTL_LINE_HEAD: &str = "TTL";
const LINE_FEED: u8 = b'\n';

#[derive(Debug)]
pub struct KvtpMessage {
    pub protocol: String,
    pub command: String,
    pub key: String,
    pub args: String,
    pub ttl: i32,

    pub body: Vec<u8>,
}

///
///
///
impl KvtpMessage {
    ///
    /// Parse bytes message to Kvtp
    ///
    pub fn parse(message: &Vec<u8>) -> Result<Self, String> {
        //println!("akvp::parse {:?}", message);

        let mut err_msg = String::new();

        // psudo code
        let mut protocol = String::new();
        let mut command = String::new();
        let mut key = String::new();
        let mut args = String::new();
        let mut ttl: i32 = 0;

        //Split message by line feed
        let lines = message.split(|&b| b == LINE_FEED);

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

                    if i == 0 {
                        protocol = line;
                    } else {
                        // Split line by Colon
                        let mut parts = line.split(":");
                        // Get the first part separated by colon
                        let fst_part = parts.next().unwrap();
                        // Get the second part separated by colon
                        let sec_part = parts.next();
                        //println!("{}:{}", fst_part, sec_part.unwrap());
                        match fst_part.to_uppercase().as_str() {
                            CMD_LINE_HEAD => match sec_part {
                                Some(second) => {
                                    command = second.trim().to_string();
                                }
                                None => {
                                    err_msg = String::from("Invalid command");
                                    break;
                                }
                            },
                            KEY_LINE_HEAD => match sec_part {
                                Some(second) => key = second.to_string(),
                                None => {
                                    err_msg = String::from("Invalid key");
                                    break;
                                }
                            },
                            ARGS_LINE_HEAD => match sec_part {
                                Some(second) => args = second.trim().to_string(),
                                None => {
                                    err_msg = String::from("Invalid args");
                                    break;
                                }
                            },
                            TTL_LINE_HEAD => {
                                let s_ttl;
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
                            _ => {
                                println!("{}:{}", fst_part, sec_part.unwrap());
                            }
                        }
                    }
                }
                Err(_) => {}
            }
        }

        // Body
        let body = message[header_len..].to_vec();

        if err_msg.len() != 0 {
            return Err(err_msg);
        }
        let akvp_msg = KvtpMessage {
            protocol,
            command,
            key,
            args,
            ttl,
            body,
        };

        println!("{:?}", akvp_msg);

        Ok(akvp_msg)
    }

    pub fn ttl_to_instant(&self) -> Option<Instant> {
        if self.ttl > 0 {
            return Some(Instant::now() + Duration::from_secs(self.ttl as u64));
        }
        None
    }
}
