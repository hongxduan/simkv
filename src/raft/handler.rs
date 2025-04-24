//! Raft request Handler
//!
//! author: Duan HongXing
//! date: 20 Apr, 2025 Changi airport

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
    time::Instant,
};

use crate::raft::raft::Raft;

use super::{heartbeat::Heartbeat, vote::Vote};

const REQUTST_VOTE: u8 = 1;
const REQUEST_HEARTBEAT: u8 = 2;

pub struct Handler {
    pub socket: TcpStream,
    pub raft: Raft,
}

impl Handler {
    pub async fn process(&mut self) -> Result<(), std::io::Error> {
        println!("Handler::process");
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
            //let mut shared = self.raft.shared.state.lock().unwrap();
            //shared.last_hb = Instant::now();
            println!("{:?}", n);

            //let mut raft = self.raft.lock().unwrap();
            //Raft::receive(&buf);
            self.receive(&buf, &self.raft);

            if let Err(e) = self.socket.write_all(&buf[0..n]).await {
                eprintln!("failed to write to socket; err = {:?}", e);
                return Err(e);
            }
        }
    }

    ///
    /// Recevie request from other nodes
    ///
    /// 1. Convert first byte to u8
    ///
    pub fn receive(&self, buf: &Vec<u8>, raft: &Raft) {
        let mut state = raft.shared.state.lock().unwrap();
        state.last_hb = Instant::now();

        let icmd = u8::from_be_bytes([buf[0]]);
        match icmd {
            REQUTST_VOTE => {
                Vote::receive();
            }
            REQUEST_HEARTBEAT => {
                Heartbeat::receive(self.raft.clone());
            }
            _ => {
                println!("Invalid Raft request: {}", icmd);
            }
        }
    }
}
