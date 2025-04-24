//! Vote implement
//!
//! author: Duan HongXing
//! date: 22 Apr 2025
//!

use std::sync::{Arc, Mutex};

use super::raft::{Raft};

const VOTE_COOL_DOWN: i64 = 500;

pub struct Vote {}

impl Vote {
    ///
    /// Send vote request
    ///
    pub fn send(self) {}

    ///
    /// Receive vote request
    ///
    pub fn receive() {}

    ///
    /// Supress followers to vote by
    /// Periodicaly check the last time received Heartbeat from Leader
    /// If *not* received in the past VOTE_COOL_DOWN period
    /// Then start to send vote request
    /// Else, cool down and then repeat
    ///
    pub async fn supress(raft: &Raft) {
        let mut interval_timer = tokio::time::interval(
            chrono::Duration::milliseconds(VOTE_COOL_DOWN)
                .to_std()
                .unwrap(),
        );
        loop {
            interval_timer.tick().await;
            //let raft = raft_acc1.lock().unwrap();
            let copy = raft.clone();
            tokio::task::spawn_blocking(move || Self::do_supress(&copy));
        }
    }

    fn do_supress(raft: &Raft) {
        let shared = raft.shared.state.lock().unwrap();

        println!("vote::do_supress: {:?}", shared.last_hb);
    }
}
