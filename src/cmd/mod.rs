///! cmd Mod
///!
///! author: Duan HongXing
///! date: 4 Apr, 2025
///!
mod base_db;
mod base_op;

pub mod command;

pub mod del;

pub mod get;

pub mod key;

pub mod set;

pub mod ttl;

pub mod cinit;
pub mod cjoin;

pub mod unknown;

/* get and set */
pub mod lst_get;
pub mod lst_set;
pub mod map_get;
pub mod map_set;
pub mod set_get;
pub mod set_set;
pub mod str_get;
pub mod str_set;


pub const OK: &[u8] = "Ok".as_bytes();
pub const INV_CMD: &[u8] = "INV_CMD".as_bytes();
pub const INV_KEY_FMT: &[u8] = "INV_KEY_FMT".as_bytes();
pub const INV_IDX: &[u8] = "INV_IDX".as_bytes(); // for list
pub const INV_TYP: &[u8] = "INV_TYP".as_bytes();
pub const INV_SUB_KEY_FMT: &[u8] = "INV_SUB_KEY_FMT".as_bytes();
pub const KEY_NOT_EX: &[u8] = "KEY_NOT_EX".as_bytes();