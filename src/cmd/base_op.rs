//! Base command for *non* db operation
//!
//! author: Duan HongXing
//! date: 19 Apr, 2025

use crate::kvtp::kvtp::KvtpMessage;

pub trait OpCommand {
    fn new(kvtp: KvtpMessage) -> Self;
    fn execute(self) -> Vec<u8>;
}
