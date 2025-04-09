//
// author: Duan HongXing
// date: 6 Apr, 2025
//

use super::del::Del;
use super::get::Get;
use super::key::Key;
use super::set::Set;
use super::unknown::Unknown;
use crate::akvp::kvtp::KvtpMessage;
use crate::bucket::bucket::Bucket;
use crate::bucket::db::Db;

pub enum Command {
    Get(Get),
    Del(Del),
    Key(Key),
    Set(Set),
    Unknown(Unknown),
}

impl Command {
    ///
    /// Execute command
    /// 
    /// 
    pub fn execute(self, db: &Db) -> Vec<u8> {
        // Execute specific command
        let result = match self {
            Command::Get(get) => get.execute(db),
            Command::Del(del) => del.execute(),
            Command::Key(key) => key.execute(),
            Command::Set(set) => set.execute(db),
            Command::Unknown(unknown) => unknown.execute(),
        };

        // Return
        result
    }

    ///
    /// Execute command without(wo) key, like: cluster-init, cluster-info
    /// Unlike execute(buckets) has parameter Vec<Bucket>, this method will
    /// not operate on key value data
    ///
    pub fn execute_wo_key() -> Vec<u8> {
        Vec::new()
    }

    pub fn parse_command(message: &Vec<u8>) -> Command {
        // Parse KVTP
        let kvtp = KvtpMessage::parse(&message).unwrap();

        println!("{:?}", kvtp);

        // Get the string command
        let ref str_cmd = kvtp.command;

        // Get specific Command
        let command = match str_cmd.to_lowercase().as_str() {
            "get" => Command::Get(Get::new(kvtp)),
            "del" => Command::Del(Del::new(kvtp)),
            "key" => Command::Key(Key::new(kvtp)),
            "set" => Command::Set(Set::new(kvtp)),
            _ => Command::Unknown(Unknown::new(kvtp)),
        };

        // Return
        command
    }
}
