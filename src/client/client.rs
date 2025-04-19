//! Cli module
//!
//! author: Duan HongXing
//! date: 7 Apr, 2025
//!

const SPACE_CHAR: char = ' ';
const BQUOTE_CHAR: char = '\\';
const DQUTE_CHAR: char = '"';

const ARG_EX: &str = "-ex";
const ARG_NX: &str = "-nx";
const ARG_DEL: &str = "-d";
const ARG_TTL: &str = "-ttl"; // set k1 v1 -ttl 10

///
///
///
#[derive(Debug)]
pub struct InputData {
    pub cmd: String,
    pub key: String,
    pub args: Vec<String>,
    pub ttl: i32,
    pub value: String,
}

/// Parse command line input
///
/// Parameters:
/// - input: user input
///
/// Return:
/// - String vector split by space
///
/// Sample:
/// - set k1 v1             -> ["set", "k1", "v1"]
/// - set k1 "hello world"  -> ["set", "k1", "hello world"]
pub fn parse_input(input: &str) -> Result<InputData, String> {
    let mut error_msg = String::new();
    let mut pieces: Vec<String> = Vec::new();

    let mut cmd: String = String::new();
    let mut key: String = String::new();
    //let mut flag: String = String::new();
    let mut args: Vec<String> = Vec::new();
    let mut ttl: i32 = 0;
    let mut value: String = String::new();

    let mut quoted: bool = false;
    let mut pc: char = '0';
    let mut si = 0; // the start index
    let mut ei: usize;

    for (i, c) in input.chars().enumerate() {
        // meet space
        if c == SPACE_CHAR || i == input.chars().count() - 1 {
            ei = i;
            // the last char
            if c == DQUTE_CHAR {
                if pc != BQUOTE_CHAR {
                    quoted = !quoted;
                }
            }

            if i == input.chars().count() - 1 && quoted {
                return Err("Quote not closed".to_string());
            }

            // if not quoted, then push piece to pieces
            if !quoted {
                let mut piece: &str = "";

                if i == input.chars().count() - 1 {
                    //pieces.push(input[si..ei + 1].trim().to_string());
                    piece = input[si..ei + 1].trim();
                } else {
                    if ei > si {
                        //pieces.push(input[si..ei].trim().to_string());
                        piece = input[si..ei].trim();
                    }
                }
                if piece.len() > 0 {
                    let first_char = piece.chars().next().unwrap();
                    // Check first is enough? need check last???
                    if first_char == DQUTE_CHAR {
                        piece = &piece[1..piece.len() - 1];
                    }
                }
                pieces.push(piece.to_string());

                si = i + 1;
            }
        } else if c == DQUTE_CHAR {
            if pc != BQUOTE_CHAR {
                quoted = !quoted;
            }
        }
        pc = c;
    }

    let mut i = 0;
    while i < pieces.len() {
        let piece = pieces[i].clone(); // clone may have performance issue
        if i == 0 {
            cmd = piece.to_string().to_uppercase();
        } else {
            match cmd.as_str() {
                // Match Command with key
                "DEL" | "GET" | "KEY" | "SET" => {
                    match piece.as_str() {
                        // Match arg
                        ARG_EX | ARG_NX | ARG_DEL => {
                            args.push(piece);
                        }
                        // Match ttl
                        ARG_TTL => {
                            if cmd == "SET" {
                                if i + 1 < pieces.len() {
                                    let ttl_val = pieces[i + 1].parse();
                                    match ttl_val {
                                        Ok(val) => ttl = val,
                                        Err(_) => {
                                            error_msg = String::from("Invalid ttl");
                                            break;
                                        }
                                    }
                                    i += 1;
                                } else {
                                    error_msg = String::from("Invalid ttl");
                                    break;
                                }
                            } else {
                                // For get key -ttl, the -ttl don't have value, just set 0
                                ttl = -3;
                            }
                        }
                        // Match key and value
                        _ => {
                            // if key not set yet
                            if key.len() == 0 {
                                key = piece;
                            } else {
                                value = piece;
                            }
                        }
                    }
                }
                // Match command without key
                _ => {}
            }
        }
        i += 1;
    }
    // TODO
    // Validate
    if error_msg.len() == 0 {
        let input_data = InputData {
            cmd,
            key,
            args,
            ttl,
            value,
        };
        println!("{:?}", input_data);
        return Ok(input_data);
    }
    Err(error_msg)
}
