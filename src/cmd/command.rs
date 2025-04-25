//! Command implement
//!
//! author: Duan HongXing
//! date: 6 Apr, 2025
//!
use super::base_db::DbCommand;
use super::base_op::OpCommand;
use super::cinit::CInit;
use super::del::Del;
use super::get::Get;
use super::key::Key;
use super::set::Set;
use super::unknown::Unknown;
use crate::kvtp::kvtp::KvtpMessage;
use crate::db::db::Db;

pub enum Command {
    Get(Get),
    Del(Del),
    Key(Key),
    Set(Set),
    CInit(CInit),
    Unknown(Unknown),
}

impl Command {
    ///
    /// Execute command
    ///
    pub fn execute(self, db: &Db) -> Vec<u8> {
        // Execute specific command
        let result = match self {
            Command::Get(get) => get.execute(db),
            Command::Del(del) => del.execute(db),
            Command::Key(key) => key.execute(),
            Command::Set(set) => set.execute(db),
            Command::CInit(cinit) => cinit.execute(),
            Command::Unknown(unknown) => unknown.execute(),
        };

        // Return
        result
    }

    ///
    /// Execute command without(wo) key, like: cluster-init, cluster-info
    /// Unlike execute(db) has parameter Db, this method will not operate
    /// on key value data
    ///
    pub fn execute_op() -> Vec<u8> {
        Vec::new()
    }

    pub fn parse_command(message: &Vec<u8>) -> Command {
        // Parse KVTP
        let kvtp = KvtpMessage::parse(&message).unwrap();

        //println!("parse_command:{:?}", kvtp);

        // Get the string command
        let ref str_cmd = kvtp.command;

        // Get specific Command
        let command = match str_cmd.to_uppercase().as_str() {
            "GET" => Command::Get(Get::new(kvtp)),
            "DEL" => Command::Del(Del::new(kvtp)),
            "KEY" => Command::Key(Key::new(kvtp)),
            "SET" => Command::Set(Set::new(kvtp)),
            "CINIT" => Command::CInit(CInit::new(kvtp)),
            _ => Command::Unknown(Unknown::new(kvtp)),
        };

        // Return
        command
    }
}
