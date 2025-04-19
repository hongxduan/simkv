//! Join Cluster implement
//!
//! author: Duan HongXing
//! date: 19 Apr, 2025

use crate::kvtp::{kvtp::KvtpMessage, response::KvtpResponse};

use super::{OK, base_op::OpCommand};

struct CJoin {
    kvtp: KvtpMessage,
}

impl OpCommand for CJoin {
    fn new(kvtp: KvtpMessage) -> Self {
        CJoin { kvtp }
    }

    fn execute(self) -> Vec<u8> {
        KvtpResponse::build_string(OK.to_vec())
    }
}
