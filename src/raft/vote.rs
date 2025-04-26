//! Vote implement
//!
//! author: Duan HongXing
//! date: 22 Apr 2025
//!

use std::time::Duration;

use tokio::time::Instant;

use super::raft::Raft;

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
    pub async fn vote(raft: &Raft) {
        let mut interval_timer = tokio::time::interval(
            chrono::Duration::milliseconds(VOTE_COOL_DOWN)
                .to_std()
                .unwrap(),
        );
        loop {
            interval_timer.tick().await;

            let shared = raft.shared.state.lock().unwrap();

            match shared.role {
                super::raft::RaftRole::Leader => {
                    break;
                }
                super::raft::RaftRole::Follower => {}
            }

            let duration = Duration::from_millis(VOTE_COOL_DOWN as u64);
            // Didn't received heartbeat from Leader in VOTE_COOL_DOWN period
            // Then start to send vote request
            if shared.last_hb + duration < Instant::now() {
                tokio::task::spawn_blocking(move || Self::do_vote());
            }
        }
    }

    fn do_vote() {
        //println!("do_vote");
    }
}
