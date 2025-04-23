//! Vote implement
//!
//! author: Duan HongXing
//! date: 22 Apr 2025
//!

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
    pub async fn supress() {
        let mut interval_timer = tokio::time::interval(
            chrono::Duration::milliseconds(VOTE_COOL_DOWN)
                .to_std()
                .unwrap(),
        );
        loop {
            interval_timer.tick().await;
            tokio::task::spawn_blocking(|| Self::do_supress());
        }
    }

    fn do_supress() {
        println!("do_supress");
    }
}
