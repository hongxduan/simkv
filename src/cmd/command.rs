//
// author: Duan HongXing
// date: 6 Apr, 2025
//

use super::del::Del;
use super::get::Get;
use super::key::Key;
use super::set::Set;
use super::unknown::Unknown;
use crate::akvp::akvp::AkvpMessage;

pub enum Command {
    Get(Get),
    Del(Del),
    Key(Key),
    Set(Set),
    Unknown(Unknown),
}

impl Command {
    pub fn execute(self) -> Vec<u8> {
        // Execute specific command
        let result = match self {
            Command::Get(get) => get.execute(),
            Command::Del(del) => del.execute(),
            Command::Key(key) => key.execute(),
            Command::Set(set) => set.execute(),
            Command::Unknown(unknown) => unknown.execute(),
        };

        // Return
        result
    }

    pub fn parse_command(message: Vec<u8>) -> Command {
        // Parse AKVP
        let akvp = AkvpMessage::parse(&message);

        // Get the string command
        let ref str_cmd = akvp.command;

        // Get specific Command
        let command = match str_cmd.to_lowercase().as_str() {
            "get" => Command::Get(Get::new(akvp)),
            "del" => Command::Del(Del::new(akvp)),
            "key" => Command::Key(Key::new(akvp)),
            "set" => Command::Set(Set::new(akvp)),
            _ => Command::Unknown(Unknown::new(akvp)),
        };

        // Return
        command
    }
}
