///! Cli module
///!
///! author: Duan HongXing
///! date: 7 Apr, 2025
///!

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

    for (i, c) in input.chars().enumerate() {
        // meet space
        if c == SPACE_CHAR || i == input.len() - 1 {
            // the last char
            if c == DQUTE_CHAR {
                if pc != BQUOTE_CHAR {
                    quoted = !quoted;
                }
            }

            // if not quoted, then push piece pieces
            if !quoted {
                if i == input.len() - 1 {
                    pieces.push(input[si..].trim().to_string());
                } else {
                    pieces.push(input[si..i].trim().to_string());
                }

                si = i;
            }
        } else if c == DQUTE_CHAR {
            if pc != BQUOTE_CHAR {
                quoted = !quoted;
                println!("{}", quoted);
            }
        }
        pc = c;
    }

    //println!("{:?}", pieces);

    let mut i = 0;
    while i < pieces.len() {
        let piece = pieces[i].clone(); // clone may have performance issue
        if i == 0 {
            cmd = piece.to_string();
        } else {
            match cmd.as_str() {
                // Match Command with key
                "del" | "get" | "key" | "set" | "ttl" => {
                    match piece.as_str() {
                        // Match arg
                        ARG_EX | ARG_NX | ARG_DEL => {
                            args.push(piece);
                        }
                        // Match ttl
                        ARG_TTL => {
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
        //println!("{:?}", input_data);
        return Ok(input_data);
    }
    Err(error_msg)
}
