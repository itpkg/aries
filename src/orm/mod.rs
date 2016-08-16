extern crate time;

// use std::collections::HashMap;
// use std::collections::btree_map::BTreeMap;
// use self::time::Tm;

// use super::{Error, Result};

pub mod mysql;
pub mod postgresql;
pub mod sqlite;

pub trait Db {
    fn execute(&self, sql: &str);
}

pub struct Migration {
    pub driver: &'static str,
    pub version: &'static str,
    pub up: &'static str,
    pub down: &'static str,
}
