//! Raft Implement
//!
//! author: Duan HongXing
//! date: 4 Apr, 2025
//!

use std::{
    collections::HashMap,
    default,
    sync::{Arc, Mutex},
};

use tokio::{net::TcpListener, time::Instant};

use crate::raft::server::Handler;

///
/// Node info
///
pub struct Node {
    id: String,
    ip: String,
    port: u16,
}

///
/// Raft data that need sync by leader to all followers
///
pub struct RaftData {
    leader_node_id: String,
    nodes: HashMap<String, Node>,
    buckets: HashMap<u16, String>, // bucket_id on which node_id
    ts: Instant,                   // when updated
}

#[derive(Debug, Clone)]
pub enum RaftRole {
    Leader,
    Follower,
}

#[derive(Debug, Clone)]
pub struct Raft {
    pub shared: Arc<Shared>,
}

#[derive(Debug)]
pub struct State {
    pub role: RaftRole,
    pub last_hb: Instant, // Last heartbeat instant received from Leader
}

#[derive(Debug)]
pub struct Shared {
    pub state: Mutex<State>,
}

impl Raft {
    pub fn new() -> Self {
        Raft {
            shared: Arc::new(Shared {
                state: Mutex::new(State {
                    role: RaftRole::Follower,
                    last_hb: Instant::now(),
                }),
            }),
        }
    }

    pub async fn start() -> Result<(), Box<dyn std::error::Error>> {
        println!("Raft::start");

        let tpc_listener = TcpListener::bind("0.0.0.0:18303").await?;

        let raft = Raft::new();
        let raft_copy = raft.clone();

        // Accept loop
        let listener = Listener {
            listener: tpc_listener,
            raft: raft.clone(),
        };

        tokio::spawn(async move {
            let _ = crate::raft::vote::Vote::supress(&raft_copy).await;
        });

        tokio::spawn(async move {
            let _ = listener.run().await;
        });

        Ok(())
    }

    async fn run(listener: TcpListener, raft: &Raft) -> Result<(), Box<dyn std::error::Error>> {
        // Accept loop
        loop {
            let (socket, _) = listener.accept().await?;
            println!("raft::server::accept");
            //let raft = Arc::clone(&raft_arc);
            //let raft = raft_arc1.lock().unwrap();
            let mut handler = Handler {
                socket,
                raft: raft.clone(),
            };
            let _ = handler.process().await;
        }
    }
}

pub struct Listener {
    pub listener: TcpListener,
    pub raft: Raft,
}

impl Listener {
    pub async fn run(self) -> Result<(), std::io::Error> {
        // Accept loop
        loop {
            match self.listener.accept().await {
                Ok((socket, _)) => {
                    println!("raft::server::accept");
                    let mut handler = Handler {
                        socket,
                        raft: self.raft.clone(),
                    };
                    let _ = handler.process().await;
                }
                Err(e) => {
                    println!("raft::Listener::run- {}", e);
                }
            }
        }
    }
}
