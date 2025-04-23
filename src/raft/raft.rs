//! Raft Implement
//!
//! author: Duan HongXing
//! date: 4 Apr, 2025
//!

use std::{collections::HashMap, time::Instant};

use super::heartbeat::Heartbeat;

const REQUTST_VOTE: u8 = 1;
const REQUEST_HEARTBEAT: u8 = 2;
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
}

impl Raft {
    pub fn new() -> Self {
        Raft {
            role: RaftRole::Follower,
        }
    }
    ///
    /// Recevie request from other nodes
    ///
    /// 1. Convert first byte to u8
    ///
    pub fn receive(&self, buf: &Vec<u8>) {
        let icmd = u8::from_be_bytes([buf[0]]);
        match icmd {
            REQUTST_VOTE => {}
            REQUEST_HEARTBEAT => {
                //Heartbeat::receive(Self);
            }
            _ => {
                println!("Invalid Raft request: {}", icmd);
            }
        }
    }
}
