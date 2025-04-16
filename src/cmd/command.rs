//! Command implement
//!
//! author: Duan HongXing
//! date: 6 Apr, 2025
//!
use super::base::BaseCommand;
use super::del::Del;
use super::get::Get;
use super::key::Key;
use super::set::Set;
use super::ttl::Ttl;
use super::unknown::Unknown;
use crate::kvtp::kvtp::KvtpMessage;
use crate::db::db::Db;

pub enum Command {
    Get(Get),
    Del(Del),
    Key(Key),
    Set(Set),
    Ttl(Ttl),
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
            Command::Ttl(ttl) => ttl.execute(db),
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
    pub fn execute_wo_key() -> Vec<u8> {
        Vec::new()
    }

    pub fn parse_command(message: &Vec<u8>) -> Command {
        // Parse KVTP
        let kvtp = KvtpMessage::parse(&message).unwrap();

        //println!("parse_command:{:?}", kvtp);

        // Get the string command
        let ref str_cmd = kvtp.command;

        // Get specific Command
        let command = match str_cmd.to_lowercase().as_str() {
            "get" => Command::Get(Get::new(kvtp)),
            "del" => Command::Del(Del::new(kvtp)),
            "key" => Command::Key(Key::new(kvtp)),
            "set" => Command::Set(Set::new(kvtp)),
            "ttl" => Command::Ttl(Ttl::new(kvtp)),
            _ => Command::Unknown(Unknown::new(kvtp)),
        };

        // Return
        command
    }
}
