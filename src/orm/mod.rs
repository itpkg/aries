extern crate time;

use std::collections::HashMap;
// use std::collections::btree_map::BTreeMap;
//use self::time::Tm;

// use super::{Error, Result};

// pub mod mysql;
pub mod postgresql;
// pub mod sqlite;

pub trait Db {}
// pub enum Type {
//     Varchar(&'static str),
//     Char(&'static str),
//     Bool(bool),
//     Int(i32),
//     Long(i64),
//     Text(&'static str),
//     Bytes(Vec<u8>),
//     Date(Tm),
//     Datetime(Tm),
//     Time(Tm),
// }

pub struct Migration {
    pub id: &'static str,
    pub up: &'static str,
    pub down: &'static str,
}

pub trait Model {
    fn queries(&self) -> HashMap<&'static str, &'static str>;
}
// pub trait Dao<T: Model> {
//     fn execute(&self,
//                name: &'static str,
//                params: BTreeMap<&'static str, Type>)
//                -> Result<u64>;
//
//     // fn find(&self, query: &str, item: T) -> Result<Vec<T>, Error>;
//     // fn delete(&self, query: &str, item: T) -> Result<i64, Error>;
// }
