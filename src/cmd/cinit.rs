//! Init cluster implement
//!
//! author: Duan HongXing
//! date: 19 Apr, 2025

use crate::{
    kvtp::{kvtp::KvtpMessage, response::KvtpResponse},
    raft::raft::{RaftRole, GLOBAL_NODE_INFO, GLOBAL_RAFT_STATE},
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
        let node_lock = GLOBAL_NODE_INFO.read().unwrap();
        println!("CInit::execute- {}", node_lock.host);

        drop(node_lock);

        // Set Raft State role to Leader
        let mut raft_state_lock = GLOBAL_RAFT_STATE.write().unwrap();
        raft_state_lock.role= RaftRole::Leader;
        let _ = raft_state_lock.role_updated(RaftRole::Leader);
        drop(raft_state_lock);

        return KvtpResponse::build_string(OK.to_vec());
    }
}
