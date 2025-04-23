//! Raft server for internal communication
//!
//! Run this server if the node init or join cluster
//!
//! author: Duan HongXing
//! date: 20 Apr, 2025 Changi airport

use std::sync::{Arc, Mutex};

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

use crate::raft::raft::Raft;

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("0.0.0.0:18303").await?;

    let raft_arc = Arc::new(Mutex::new(Raft::new()));

    // Accept loop
    loop {
        let (socket, _) = listener.accept().await?;
        println!("raft::server::accept");
        //let raft = Arc::clone(&raft_arc);
        let raft = raft_arc.clone();
        let handler = Handler { socket, raft };
        tokio::spawn(async move {
            let _ = handler.process();
        });
    }
}

struct Handler {
    socket: TcpStream,
    raft: Arc<Mutex<Raft>>,
}

impl Handler {
    pub async fn process(mut self) -> Result<(), std::io::Error> {
        loop {
            // buffer for request length
            let mut lbuf: [u8; 4] = [0, 0, 0, 0];

            let _n = match self.socket.read(&mut lbuf).await {
                // socket closed
                Ok(0) => return Ok(()),
                Ok(n) => n,
                Err(e) => {
                    eprintln!("failed to read from socket; err = {:?}", e);
                    return Err(e.into());
                }
            };
            if lbuf == [0, 0, 0, 0] {
                continue;
            }

            let mlen = u32::from_be_bytes(lbuf) as usize;
            let mut buf = vec![0; mlen.try_into().unwrap()];

            // data
            let mut n: usize = 0;
            while n < mlen {
                n += match self.socket.read(&mut buf).await {
                    // socket closed
                    Ok(0) => return Ok(()),
                    Ok(n) => n,
                    Err(e) => {
                        eprintln!("failed to read from socket; err = {:?}", e);
                        return Err(e);
                    }
                };
            }
            println!("{:?}", n);

            let raft = self.raft.lock().unwrap();
            //Raft::receive(&buf);
            raft.receive(&buf);

            if let Err(e) = self.socket.write_all(&buf[0..n]).await {
                eprintln!("failed to write to socket; err = {:?}", e);
                return Err(e);
            }
        }
    }
}
