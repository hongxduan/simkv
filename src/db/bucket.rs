///! Bucket implement
///!
///! author: Duan HongXing
///! date: 4 Apr, 2025
use std::collections::HashMap;

use super::entry::Entry;

pub const BUCKETS_PER_PAGE: usize = 64;
pub const SLOTS_PER_BUCKET: usize = 256;

#[derive(Debug, Clone)]
pub struct Bucket {
    id: u16,
    pub slots: Vec<HashMap<String, Entry>>,
    //slots: [HashMap<String, Vec<u8>>; SLOTS_PER_BUCKET],
}

impl Bucket {
    pub fn init() -> Vec<Bucket> {
        let mut buckets: Vec<Bucket> = Vec::new();
        for i in 0..BUCKETS_PER_PAGE {
            buckets.push(Bucket::new(i.try_into().unwrap()));
        }
        buckets
    }

    pub fn new(id: u16) -> Bucket {
        let slots = std::iter::repeat_with(|| HashMap::new())
            .take(SLOTS_PER_BUCKET)
            .collect::<Vec<_>>();
        let bucket = Bucket { id, slots };
        bucket
    }

    pub fn calc_bucket_id(key: &str) -> usize {
        let crc16: crc::Crc<u16> = crc::Crc::<u16>::new(&crc::CRC_16_XMODEM);
        (crc16.checksum(key.as_bytes()) % (BUCKETS_PER_PAGE as u16)) as usize
    }

    /*
    fn locate_bucket(self: &Self, key: u16) -> Box<Bucket> {
        if key < self.key {
            return self.left.locate_bucket(key);
        } else if key > self.key {
            return self.right.locate_bucket(key);
        } else {
            return self;
        }
    }*/
}
