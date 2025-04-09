//! simkv-server
//!
//! author: Duan HongXing
//! date: 4 Apr, 2025

use akvp::kvtp::KvtpMessage;
use db::db::Db;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

use std::{env, vec};

mod db;
use db::bucket::Bucket;

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
    //let buckets = Bucket::init();

    // Init Db
    let db = Db::new();

    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:8303".to_string());

    let listener = TcpListener::bind("0.0.0.0:8303").await?;
    println!("Listening on: {}", addr);

    // Accept loop
    loop {
        let (socket, _) = listener.accept().await?;
        process(socket, &db).await;
    }
}

///
/// Strt read loop
///
async fn process(mut socket: TcpStream, db: &Db) {
    let mut lbuf: [u8; 4] = [0, 0, 0, 0];

    loop {
        // Read message lenght bytes
        let _n = match socket.read(&mut lbuf).await {
            // socket closed
            Ok(0) => return,
            Ok(n) => n,
            Err(e) => {
                eprintln!("failed to read from socket; err = {:?}", e);
                return;
            }
        };

        // Why this happen???
        if lbuf == [0, 0, 0, 0] {
            continue;
        }

        //
        // Read one single command byte length
        //
        let mlen = u32::from_be_bytes(lbuf) as usize;
        let mut buf = vec![0; mlen.try_into().unwrap()];

        //
        // Read message content based on the length readed above
        //
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

        //println!("mlen: {}, {:?}", mlen, String::from_utf8(buf.clone()));

        //
        // Parse command
        //
        let command = Command::parse_command(&buf);

        //
        // Execute command
        //
        let result = command.execute(db);

        // Write the data back
        if let Err(e) = socket.write_all(&result).await {
            eprintln!("failed to write to socket; err = {:?}", e);
            return;
        }
        //let _ = socket.write_all(&result).await;
        //let _ = socket.flush().await;
    }
}
