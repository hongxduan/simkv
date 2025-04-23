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
    pub fn send(mut raft: Raft) {
        //let (tx, rx) = mpsc::channel(1);
        tokio::spawn(async move {
            loop {
                // If not leader then break, else keep looping
                match raft.role {
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
    pub fn receive(raft: &mut Raft) {
        // Update last_hb
        raft.last_hb = Instant::now();
    }
}
