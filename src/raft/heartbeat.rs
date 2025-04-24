//! Heartbeat implement
//!
//! author: Duan HongXing
//! date: 22 Apr, 2025
//!

use tokio::{sync::mpsc, time::Instant};

use super::raft::{Raft, RaftRole};

const HB_LOWER: u16 = 200; // random lower
const HB_UPPER: u16 = 400; // random upper

pub struct Heartbeat {}

impl Heartbeat {
    ///
    /// Send heartbeat request
    ///
    pub fn send(raft: Raft) {
        //let (tx, rx) = mpsc::channel(1);
        tokio::spawn(async move {
            loop {
                // If not leader then break, else keep looping
                let shared = raft.shared.state.lock().unwrap();
                match shared.role {
                    RaftRole::Leader => {}
                    RaftRole::Follower => {
                        break;
                    }
                }

                // TODO: sending heartbeat to each follower
            }
        });
    }

    ///
    /// Receive heartbeat request
    ///
    pub fn receive(raft: Raft) {
        // Update last_hb
        let mut shared = raft.shared.state.lock().unwrap();
        shared.last_hb = Instant::now();
    }
}
