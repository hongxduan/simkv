///! Bucket implement
///!
///! author: Duan HongXing
///! date: 4 Apr, 2025
use std::collections::HashMap;

pub const BUCKET_SIZE: usize = 512;

pub struct Bucket {
    id: u16,
    slots: Vec<HashMap<String, Vec<u8>>>,
    //left: &<'a>Bucket,
    //right: &'a Bucket,
}

impl Bucket {
    pub fn init() -> Vec<Bucket> {
        let mut buckets: Vec<Bucket> = Vec::new();
        for i in 0..BUCKET_SIZE {
            buckets.push(Bucket::new(i.try_into().unwrap()));
        }
        buckets
    }

    pub fn new(id: u16) -> Bucket {
        let slots = std::iter::repeat_with(|| HashMap::new())
            .take(20)
            .collect::<Vec<_>>();
        let bucket = Bucket { id, slots };
        bucket
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
