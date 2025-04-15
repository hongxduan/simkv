//! In-Memory Database implement
//!
//! author: Duan HongXing
//! date: 8 Apr, 2025
//!
//!
//! Page:
//!    Total 16 pages
//!    Each page contain 64 bucket
//!
//! Bucket:
//!    Total 1024 buckets, this is also the nodes limit
//!
//! Slot:
//!    Each bucket contain BUCKETS_PER_PAGE slots
//!
use std::{
    collections::BTreeSet,
    sync::{Arc, Mutex},
};
use tokio::{
    sync::Notify,
    time::{self, Instant},
};

use super::{
    bucket::{BUCKETS_PER_PAGE, Bucket, SLOTS_PER_BUCKET},
    entry::Entry,
};

const PAGE_NUM: usize = 16;
const BUCKET_NUM: usize = PAGE_NUM * BUCKETS_PER_PAGE; // 1024

#[derive(Debug)]
struct Page {
    buckets: Vec<Bucket>,
}

#[derive(Debug)]
struct State {
    pages: Vec<Page>,
    expirations: BTreeSet<(Instant, String)>,
    shutdown: bool,
}

#[derive(Debug)]
struct Shared {
    state: Mutex<State>,
    background_task: Notify,
}

#[derive(Debug)]
pub(crate) struct DbDropGuard {
    db: Db,
}

#[derive(Debug, Clone)]
pub struct Db {
    //pages: Arc<Vec<Page>>
    shared: Arc<Shared>,
}

impl Db {
    pub fn new() -> Self {
        /*let pages = std::iter::repeat_with(|| Page {
            state: Mutex::new(State {
                buckets: Bucket::init(),
                //expirations: BTreeSet::new(),
            }),
            background_task: Notify::new(),
        })
        .take(16)
        .collect();*/

        let pages = std::iter::repeat_with(|| Page {
            buckets: Bucket::init(),
        })
        .take(16)
        .collect::<Vec<_>>();

        let shared = Arc::new(Shared {
            state: Mutex::new(State {
                pages,
                shutdown: false,
                expirations: BTreeSet::new(),
            }),
            background_task: Notify::new(),
        });

        tokio::spawn(purge_expired_tasks(shared.clone()));

        Db {
            //pages: Arc::new(pages),
            shared,
        }
    }

    pub fn set(&self, key: String, entry: Entry) {
        //let bucket_id = Bucket::calc_bucket_id(&key);
        //let slot = self.calc_slot(&key);
        //let mut state = self.page0.state.lock().unwrap();
        //state.buckets[bucket_id].slots[slot].insert(key, entry);

        let (page, bucket, slot) = self.locate_pbs(&key);
        let mut state = self.shared.state.lock().unwrap();
        state.pages[page].buckets[bucket].slots[slot].insert(key.as_bytes().to_vec(), entry);
        //state.buckets[bucket].slots[slot].insert(key, entry);

        drop(state);
    }

    pub fn get(&self, key: String) -> Option<Entry> {
        //let bucket_id = Bucket::calc_bucket_id(&key);
        //let slot = self.calc_slot(&key);

        let (page, bucket, slot) = self.locate_pbs(&key);
        let state = self.shared.state.lock().unwrap();
        let entry = state.pages[page].buckets[bucket].slots[slot].get(key.as_bytes());

        entry.cloned()
    }

    pub fn del(&self, key: String) -> Option<Entry> {
        let (page, bucket, slot) = self.locate_pbs(&key);
        let mut state = self.shared.state.lock().unwrap();
        let entry = state.pages[page].buckets[bucket].slots[slot].remove(&key.as_bytes().to_vec());
        entry
    }

    ///
    /// Calculate page, bucket, slot
    ///
    /// +--Page--+--Bucket index--+--Bucket id--+
    /// | page0: |    0-63        |   0-63      |
    /// | page1: |    0-63        |   64-127    |
    /// | page2: |    0-63        |   128-191   |
    /// | ...    |    ...         |   ...       |
    /// +--------+----------------+-------------+
    ///
    fn locate_pbs(&self, key: &str) -> (usize, usize, usize) {
        let crc16: crc::Crc<u16> = crc::Crc::<u16>::new(&crc::CRC_16_XMODEM);
        let bucket_id = (crc16.checksum(key.as_bytes()) % (BUCKET_NUM as u16)) as usize;
        let page = bucket_id / BUCKETS_PER_PAGE; // 155 / 64 = 2.42 = 2 => page2
        let bucket = bucket_id % BUCKETS_PER_PAGE; // 155 % 64 = 27 => page2.buckets[27]
        let slot = bucket_id % SLOTS_PER_BUCKET; // 155 % 64 = 27 => page2.buckets[27].slot[27]

        (page, bucket, slot)
    }

    fn shutdown_purge_task(&self) {
        // The background task must be signaled to shut down. This is done by
        // setting `State::shutdown` to `true` and signalling the task.
        let mut state = self.shared.state.lock().unwrap();
        state.shutdown = true;

        // Drop the lock before signalling the background task. This helps
        // reduce lock contention by ensuring the background task doesn't
        // wake up only to be unable to acquire the mutex.
        drop(state);
        self.shared.background_task.notify_one();
    }
}

impl DbDropGuard {
    pub(crate) fn new() -> DbDropGuard {
        DbDropGuard { db: Db::new() }
    }

    pub(crate) fn db(&self) -> Db {
        self.db.clone()
    }
}

impl Drop for DbDropGuard {
    fn drop(&mut self) {
        self.db.shutdown_purge_task();
    }
}

impl Shared {
    fn purge_expired_keys(&self) -> Option<Instant> {
        let mut state = self.state.lock().unwrap();

        if state.shutdown {
            // The database is shutting down. All handles to the shared state
            // have dropped. The background task should exit.
            return None;
        }

        // This is needed to make the borrow checker happy. In short, `lock()`
        // returns a `MutexGuard` and not a `&mut State`. The borrow checker is
        // not able to see "through" the mutex guard and determine that it is
        // safe to access both `state.expirations` and `state.entries` mutably,
        // so we get a "real" mutable reference to `State` outside of the loop.
        let state = &mut *state;

        // Find all keys scheduled to expire **before** now.
        let now = Instant::now();

        while let Some(&(when, ref key)) = state.expirations.iter().next() {
            if when > now {
                // Done purging, `when` is the instant at which the next key
                // expires. The worker task will wait until this instant.
                return Some(when);
            }

            // The key expired, remove it
            state.pages[1].buckets[1].slots[1].remove(key.as_bytes()); // TODO: remove hardcode index
            state.expirations.remove(&(when, key.clone()));
        }

        None
    }

    fn is_shutdown(&self) -> bool {
        self.state.lock().unwrap().shutdown
    }
}

async fn purge_expired_tasks(shared: Arc<Shared>) {
    while !shared.is_shutdown() {
        println!("purge...");
        if let Some(when) = shared.purge_expired_keys() {
            tokio::select! {
                _=time::sleep_until(when)=>{}
                _=shared.background_task.notified()=>{}
            }
        } else {
            shared.background_task.notified().await;
        }
    }
}
