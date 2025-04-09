///! In-Memory Database implement
///!
///! author: Duan HongXing
///! date: 8 Apr, 2025
///!
///!
///! Page:
///!    Total 16 pages
///!    Each page contain 64 bucket
///!
///! Bucket:
///!    Total 1024 buckets, this is also the nodes limit
///!
///! Slot:
///!    Each bucket contain 256 slots
///!
use std::{
    collections::BTreeSet,
    sync::{Arc, Mutex},
};
use tokio::{sync::Notify, time::Instant};

use super::{
    bucket::{BUCKETS_PER_PAGE, Bucket, SLOTS_PER_BUCKET},
    entry::Entry,
};

const PAGE_NUM: usize = 16;
const BUCKET_NUM: usize = PAGE_NUM * BUCKETS_PER_PAGE; // 1024

#[derive(Debug, Clone)]
pub struct State {
    pub buckets: Vec<Bucket>,
    expirations: BTreeSet<(Instant, String)>,
}

#[derive(Debug)]
struct Page {
    state: Mutex<State>,
    background_task: Notify,
}

#[derive(Debug, Clone)]
pub struct Db {
    page0: Arc<Page>,
    page1: Arc<Page>,
    page2: Arc<Page>,
    page3: Arc<Page>,
    page4: Arc<Page>,
    page5: Arc<Page>,
    page6: Arc<Page>,
    page7: Arc<Page>,
    page8: Arc<Page>,
    page9: Arc<Page>,
    page10: Arc<Page>,
    page11: Arc<Page>,
    page12: Arc<Page>,
    page13: Arc<Page>,
    page14: Arc<Page>,
    page15: Arc<Page>,
}

impl Db {
    pub fn new() -> Self {
        let page0 = Arc::new(Page {
            state: Mutex::new(State {
                buckets: Bucket::init(),
                expirations: BTreeSet::new(),
            }),
            background_task: Notify::new(),
        });

        let page1 = Arc::new(Page {
            state: Mutex::new(State {
                buckets: Bucket::init(),
                expirations: BTreeSet::new(),
            }),
            background_task: Notify::new(),
        });

        let page2 = Arc::new(Page {
            state: Mutex::new(State {
                buckets: Bucket::init(),
                expirations: BTreeSet::new(),
            }),
            background_task: Notify::new(),
        });

        let page3 = Arc::new(Page {
            state: Mutex::new(State {
                buckets: Bucket::init(),
                expirations: BTreeSet::new(),
            }),
            background_task: Notify::new(),
        });

        let page4 = Arc::new(Page {
            state: Mutex::new(State {
                buckets: Bucket::init(),
                expirations: BTreeSet::new(),
            }),
            background_task: Notify::new(),
        });

        let page5 = Arc::new(Page {
            state: Mutex::new(State {
                buckets: Bucket::init(),
                expirations: BTreeSet::new(),
            }),
            background_task: Notify::new(),
        });

        let page6 = Arc::new(Page {
            state: Mutex::new(State {
                buckets: Bucket::init(),
                expirations: BTreeSet::new(),
            }),
            background_task: Notify::new(),
        });

        let page7 = Arc::new(Page {
            state: Mutex::new(State {
                buckets: Bucket::init(),
                expirations: BTreeSet::new(),
            }),
            background_task: Notify::new(),
        });

        let page8 = Arc::new(Page {
            state: Mutex::new(State {
                buckets: Bucket::init(),
                expirations: BTreeSet::new(),
            }),
            background_task: Notify::new(),
        });

        let page9 = Arc::new(Page {
            state: Mutex::new(State {
                buckets: Bucket::init(),
                expirations: BTreeSet::new(),
            }),
            background_task: Notify::new(),
        });

        let page10 = Arc::new(Page {
            state: Mutex::new(State {
                buckets: Bucket::init(),
                expirations: BTreeSet::new(),
            }),
            background_task: Notify::new(),
        });

        let page11 = Arc::new(Page {
            state: Mutex::new(State {
                buckets: Bucket::init(),
                expirations: BTreeSet::new(),
            }),
            background_task: Notify::new(),
        });
        let page12 = Arc::new(Page {
            state: Mutex::new(State {
                buckets: Bucket::init(),
                expirations: BTreeSet::new(),
            }),
            background_task: Notify::new(),
        });
        let page13 = Arc::new(Page {
            state: Mutex::new(State {
                buckets: Bucket::init(),
                expirations: BTreeSet::new(),
            }),
            background_task: Notify::new(),
        });
        let page14 = Arc::new(Page {
            state: Mutex::new(State {
                buckets: Bucket::init(),
                expirations: BTreeSet::new(),
            }),
            background_task: Notify::new(),
        });
        let page15 = Arc::new(Page {
            state: Mutex::new(State {
                buckets: Bucket::init(),
                expirations: BTreeSet::new(),
            }),
            background_task: Notify::new(),
        });

        Db {
            page0,
            page1,
            page2,
            page3,
            page4,
            page5,
            page6,
            page7,
            page8,
            page9,
            page10,
            page11,
            page12,
            page13,
            page14,
            page15,
        }
    }

    pub fn set(&self, key: String, entry: Entry) {
        //let bucket_id = Bucket::calc_bucket_id(&key);
        //let slot = self.calc_slot(&key);
        //let mut state = self.page0.state.lock().unwrap();
        //state.buckets[bucket_id].slots[slot].insert(key, entry);

        let (page, bucket, slot) = self.locate_pbs(&key);
        let mut state = match page {
            0 => self.page0.state.lock().unwrap(),
            1 => self.page1.state.lock().unwrap(),
            2 => self.page2.state.lock().unwrap(),
            3 => self.page3.state.lock().unwrap(),
            4 => self.page4.state.lock().unwrap(),
            5 => self.page5.state.lock().unwrap(),
            6 => self.page6.state.lock().unwrap(),
            7 => self.page7.state.lock().unwrap(),
            8 => self.page8.state.lock().unwrap(),
            9 => self.page9.state.lock().unwrap(),
            10 => self.page10.state.lock().unwrap(),
            11 => self.page11.state.lock().unwrap(),
            12 => self.page12.state.lock().unwrap(),
            13 => self.page13.state.lock().unwrap(),
            14 => self.page14.state.lock().unwrap(),
            15 => self.page15.state.lock().unwrap(),
            _ => self.page15.state.lock().unwrap(),
        };
        state.buckets[bucket].slots[slot].insert(key, entry);

        drop(state);
    }

    pub fn get(&self, key: String) -> Option<Entry> {
        //let bucket_id = Bucket::calc_bucket_id(&key);
        //let slot = self.calc_slot(&key);

        let (page, bucket, slot) = self.locate_pbs(&key);
        let state = match page {
            0 => self.page0.state.lock().unwrap(),
            1 => self.page1.state.lock().unwrap(),
            2 => self.page2.state.lock().unwrap(),
            3 => self.page3.state.lock().unwrap(),
            4 => self.page4.state.lock().unwrap(),
            5 => self.page5.state.lock().unwrap(),
            6 => self.page6.state.lock().unwrap(),
            7 => self.page7.state.lock().unwrap(),
            8 => self.page8.state.lock().unwrap(),
            9 => self.page9.state.lock().unwrap(),
            10 => self.page10.state.lock().unwrap(),
            11 => self.page11.state.lock().unwrap(),
            12 => self.page12.state.lock().unwrap(),
            13 => self.page13.state.lock().unwrap(),
            14 => self.page14.state.lock().unwrap(),
            15 => self.page15.state.lock().unwrap(),
            _ => self.page15.state.lock().unwrap(),
        };

        //let state = self.page0.state.lock().unwrap();
        let entry = state.buckets[bucket].slots[slot].get(&key);

        entry.cloned()
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
        let slot = bucket_id % SLOTS_PER_BUCKET; // 155 % 256 = 155 => page2.buckets[27].slot[155]

        (page, bucket, slot)
    }
}
