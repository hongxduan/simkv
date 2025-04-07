///! simkv-cli
///!
///! author: Duan HongXing
///! date: 4 Apr, 2025
///!
use clap::{ArgAction, Parser};
use std::io::Write;
use std::io::{self, Read};
use std::net::TcpStream;

mod client;
use client::akvp::build_akvp_message;
use client::client::parse_input;

const CLI_PREFIX: &str = "simkv> ";

#[derive(Parser, Debug)]
#[command(version, about, disable_help_flag=true, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = String::from("localhost"))]
    host: String,

    #[arg(short, long, default_value_t = 8303)]
    port: u16,

    #[arg(short='?', long="help",action=ArgAction::Help, help=String::from("Print help"))]
    help: Option<String>,
}

///
///
///
fn main() {
    let args = Args::parse();
    //println!("arg_name: {}, arg_value:{}", args.host, args.port);
    let mut stream = TcpStream::connect(format!("{}:{}", args.host, args.port))
        .expect("Couldn't connect to the server...");

    let stdin = io::stdin();
    let input = &mut String::new();
    loop {
        print_leading(&args.host, args.port);
        std::io::stdout().flush().unwrap();
        input.clear();
        stdin.read_line(input).unwrap();

        let input_data = parse_input(input);
        match input_data {
            Ok(data) => {
                let akvp = build_akvp_message(data);
                let result = stream.write_all(akvp.as_slice());
                match result {
                    Ok(_) => {
                        let mut buf = [0; 1024];
                        //let mut buf: Vec<u8> = Vec::new();
                        // TODO: loop
                        let len_result = stream.read(&mut buf);
                        match len_result {
                            Ok(len) => {
                                println!("{},{:?}", len, String::from_utf8(buf[0..len].to_vec()));
                            }
                            Err(e) => {
                                println!("{:?}", e);
                                continue;
                            }
                        }
                    }
                    Err(e) => {
                        println!("{:?}", e);
                        continue;
                    }
                }

                //println!("{:?}", String::from_utf8(akvp));
            }
            Err(err) => {
                println!("{}", err);
                continue;
            }
        }
    }
}

///
///
///
fn print_leading(host: &str, port: u16) {
    print!("[{}:{}]{}", host, port, CLI_PREFIX);
}
