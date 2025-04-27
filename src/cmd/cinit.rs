//! Init cluster implement
//!
//! author: Duan HongXing
//! date: 19 Apr, 2025

use crate::{
    kvtp::{kvtp::KvtpMessage, response::KvtpResponse},
    raft::raft::Raft,
};

use super::{OK, base_op::OpCommand};

///
/// Cluster Init
///
pub struct CInit {
    kvtp: KvtpMessage,
}

impl OpCommand for CInit {
    fn new(kvtp: KvtpMessage) -> Self {
        CInit { kvtp: kvtp }
    }

    fn execute(self) -> Vec<u8> {
        let result = Raft::init();
        match result {
            Ok(()) => {
                return KvtpResponse::build_string(OK.to_vec());
            }
            Err(e) => {
                return KvtpResponse::build_err(e.as_bytes().to_vec());
            }
        }
    }
}
