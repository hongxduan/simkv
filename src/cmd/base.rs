///! Trait BaseCommand
///!
///! author: Duan HongXing
///! date: 4 Apr, 2025
///!
use crate::{akvp::kvtp::KvtpMessage, db::db::Db};

pub trait BaseCommand {
    fn new(kvtp: KvtpMessage) -> Self;
    fn execute(self, db: &Db) -> Vec<u8>;

    ///
    /// To parse the orignal key
    /// 
    /// Return:
    ///     The key
    ///     The key type
    ///     The sub key
    /// 
    fn parse_key(&self, key: &str) {}
}
