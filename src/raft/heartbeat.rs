//! Heartbeat implement
//!
//! author: Duan HongXing
//! date: 22 Apr, 2025
//!

use tokio::time::Instant;

use super::raft::{GLOBAL_RAFT_STATE, Raft, RaftRole};

const HB_LOWER: u16 = 200; // random lower
const HB_UPPER: u16 = 400; // random upper

pub struct Heartbeat {}

impl Heartbeat {
    ///
    /// Send heartbeat request
    ///
    pub async fn send() {
        let mut interval_timer =
            tokio::time::interval(chrono::Duration::milliseconds(300).to_std().unwrap());
        loop {
            interval_timer.tick().await;

            let read_lock = GLOBAL_RAFT_STATE.read().unwrap();
            let _ = match read_lock.role {
                RaftRole::Follower => break,
                RaftRole::Leader => {}
            };
            drop(read_lock);

            tokio::task::spawn_blocking(move || Self::do_send());
        }
    }

    fn do_send() {
        //println!("sending heartbeat");
        // TODO: send 
    }

    ///
    /// Receive heartbeat request
    ///
    pub fn receive() {
        // Update last_hb
        let mut write_lock = GLOBAL_RAFT_STATE.write().unwrap();
        write_lock.last_hb = Instant::now();
        drop(write_lock);
    }
}
