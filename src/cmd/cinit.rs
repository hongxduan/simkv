//! Init cluster implement
//!
//! author: Duan HongXing
//! date: 19 Apr, 2025

use crate::kvtp::{kvtp::KvtpMessage, response::KvtpResponse};

use super::{base_op::OpCommand, OK};

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

        return KvtpResponse::build_string(OK.to_vec());
    }
}
