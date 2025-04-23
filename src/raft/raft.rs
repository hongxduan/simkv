//! Raft Implement
//!
//! author: Duan HongXing
//! date: 4 Apr, 2025
//!

use std::collections::HashMap;

use tokio::time::Instant;

use super::server;

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
    pub role: RaftRole,
    pub last_hb: Instant, // Last heartbeat instant received from Leader
}

impl Raft {
    pub fn new() -> Self {
        Raft {
            role: RaftRole::Follower,
            last_hb: Instant::now(),
        }
    }

    pub async fn start() {
        println!("Raft::start");
        tokio::spawn(async {
            let _t = server::run().await;
        });
    }
}
