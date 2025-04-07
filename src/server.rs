//! simkv-server
//!
//! author: Duan HongXing
//! date: 4 Apr, 2025

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

use std::{env, vec};

mod bucket;
use bucket::bucket::Bucket;

mod akvp;
mod cmd;
use cmd::command::Command;

/// SimKV server main fn
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    run().await
}

/// Init Buckets
/// Start accept loop
async fn run() -> Result<(), Box<dyn std::error::Error>> {
    // Init Buckets
    let buckets = Bucket::init();

    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:8303".to_string());

    let listener = TcpListener::bind("0.0.0.0:8303").await?;
    print!("Listening on: {}", addr);

    // Accept loop
    loop {
        let (socket, _) = listener.accept().await?;
        process(socket, &buckets).await;
    }
}

// Strt read loop
async fn process(mut socket: TcpStream, buckets: &Vec<Bucket>) {
    let mut lbuf: [u8; 4] = [0, 0, 0, 0];

    loop {
        // Read message lenght bytes
        let n = match socket.read(&mut lbuf).await {
            // socket closed
            Ok(0) => return,
            Ok(n) => n,
            Err(e) => {
                eprintln!("failed to read from socket; err = {:?}", e);
                return;
            }
        };
        let mlen = u32::from_be_bytes(lbuf) as usize;
        let mut buf = vec![0; mlen.try_into().unwrap()];

        // Read message content
        let mut n: usize = 0;
        while n < mlen {
            n += match socket.read(&mut buf).await {
                // socket closed
                Ok(0) => return,
                Ok(n) => n,
                Err(e) => {
                    eprintln!("failed to read from socket; err = {:?}", e);
                    return;
                }
            };
        }
        println!("{n}");

        // Execute command
        let command = Command::parse_command(buf);
        let result = command.execute();

        // Write the data back
        if let Err(e) = socket.write_all(&result).await {
            eprintln!("failed to write to socket; err = {:?}", e);
            return;
        }
    }
}
