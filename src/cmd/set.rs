//! Set command implement
//!
//! author: Duan HongXing
//! date: 4 Apr, 2025
//!

use crate::{
    akvp::kvtp::KvtpMessage,
    db::{
        db::Db,
        entry::{Entry, EntryData, EntryType},
    },
};

use super::base::{BaseCommand, INV_IDX, INV_TYP, KeyInfo, OK};

#[derive(Debug)]
pub struct Set {
    kvtp: KvtpMessage,
}

impl Set {
    fn set_str(self, ki: KeyInfo, db: &Db) -> Vec<u8> {
        let entry = Entry {
            etype: EntryType::STR,
            data: EntryData::Byt(self.kvtp.body.clone()), //byt: Some(self.kvtp.body.clone()),
                                                          //map: None,
                                                          //lst: None,
        };
        db.set(ki.key, entry);
        OK.to_vec()
    }

    /// Set List implement
    ///
    /// Return:
    ///     OK: if set success
    ///     INV_TYP: If the key already exists, and the type is is NOT list
    ///
    /// Set from left(Push Left)
    ///     - below command will produce users[tom, jerry]
    ///       `set users[0] tom jerry``
    ///     
    /// Set from right(Push Right)
    ///     - below command will produce users[jerry, tom]
    ///       `set users[-1] tom jerry
    ///
    /// Set in the middle(Insert)
    ///     - below command will produce users[x,tom,...]
    ///       `set users[1] tom``
    ///     - below command will produce users[x,tom, jerry,...]
    ///       `set users[1] tom jery`
    ///
    fn set_lst(self, ki: KeyInfo, db: &Db) -> Vec<u8> {
        let idx_result = ki.skey.parse::<isize>();
        let entry_opt = db.get(ki.key.clone());
        match entry_opt {
            // If entry exists already
            Some(entry) => match entry.data {
                EntryData::Lst(mut l) => {
                    match idx_result {
                        Ok(idx) => {
                            let mut udx: usize = 0;
                            if idx == -1 {
                            } else if idx >= 0 {
                                udx = idx as usize;
                            } else {
                                return INV_IDX.to_vec();
                            }
                            println!("udx:{}", udx);
                            // Entry type not match
                            if entry.etype != EntryType::LST {
                                return INV_TYP.to_vec();
                            }

                            println!("---");
                            // TODO: body may container multi value
                            //l.splice(udx..udx, [self.kvtp.body]);
                            println!("len:{}", l.len());
                            //l.insert(udx, self.kvtp.body);
                            l.push(self.kvtp.body);
                            //entry.data.insert(udx, self.kvtp.body);
                            //entry.data = EntryData::Lst(l);
                            //println!("len:{}", l.len());
                        }
                        Err(e) => {
                            println!("{}", e);
                            return INV_IDX.to_vec();
                        }
                    }
                }
                _ => {}
            },
            // New List
            None => {
                let entry = Entry {
                    etype: EntryType::LST,
                    data: EntryData::Lst(vec![self.kvtp.body]),
                };

                db.set(ki.key, entry);
            }
        }
        OK.to_vec()
    }

    fn set_map(self, ki: KeyInfo, db: &Db) -> Vec<u8> {
        OK.to_vec()
    }

    fn set_set(self, ki: KeyInfo, db: &Db) -> Vec<u8> {
        OK.to_vec()
    }
}

impl BaseCommand for Set {
    fn new(kvtp: KvtpMessage) -> Self {
        Set { kvtp }
    }

    fn execute(self, db: &Db) -> Vec<u8> {
        //println!("set::execute {}", self.kvtp.command);
        let key_info = self.parse_key(&self.kvtp.key);
        match key_info {
            Ok(ki) => match ki.entry_type {
                EntryType::STR => self.set_str(ki, db),
                EntryType::LST => self.set_lst(ki, db),
                EntryType::MAP => self.set_map(ki, db),
                EntryType::SET => self.set_set(ki, db),
            },
            Err(e) => e.to_string().as_bytes().to_vec(),
        }
    }
}
