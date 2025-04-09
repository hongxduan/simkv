///! Trait BaseCommand
///!
///! author: Duan HongXing
///! date: 4 Apr, 2025
///!
use crate::{
    akvp::kvtp::KvtpMessage,
    bucket::{
        bucket::{Bucket, SLOTS_PER_BUCKET},
        db::Db,
    },
};

pub trait BaseCommand {
    fn new(kvtp: KvtpMessage) -> Self;
    fn execute(self, db: &mut Db) -> Vec<u8>;

    fn calc_slot(&self, key: &str) -> usize {
        let crc16: crc::Crc<u16> = crc::Crc::<u16>::new(&crc::CRC_16_XMODEM);
        (crc16.checksum(key.as_bytes()) % (SLOTS_PER_BUCKET as u16)) as usize
    }
}
