extern crate postgres as pg;

// use std::error::Error as StdError;
// use std::collections::btree_map::BTreeMap;
// use self::pg::{SslMode, Connection};
// use self::pg::error::ConnectError;
// use super::super::{Error, Result};
// use super::Model;
//
// pub struct Dao<T: Model> {
//     db: pg::Connection,
//     t: T,
// }
//
// impl<T: Model> Dao<T> {
//     pub fn new(t: T, url: &str, params: SslMode) -> Result<Dao<T>> {
//         let db = try!(Connection::connect(url, params));
//         Ok(Dao { db: db, t: t })
//     }
//
//
//
//     // pub fn execute(&self, query: &str, params: &[&ToSql]) -> Result<u64, String> {
//     //     match self.db.execute(query, params) {
//     //         Ok(v) => Ok(v),
//     //         Err(e) => Err(e.description().to_string()),
//     //     }
//     // }
//
//     // pub fn query(&self, query: &str, params: &[&ToSql]) -> Result<u64, String> {
//     //     match self.db.execute(query, params) {
//     //         Ok(v) => {},
//     //         Err(e) => Err(e.description().to_string()),
//     //     }
//     // }
// }
//
// impl<T: super::Model> super::Dao<T> for Dao<T> {
//     fn execute(&self,
//                name: &'static str,
//                _: BTreeMap<&'static str, super::Type>)
//                -> Result<u64> {
//         let query = try!(self.t.queries().get(name));
//         //TODO args
//         let val = try!(self.db.execute(query, &[]));
//         Ok(val)
//     }
// }
