extern crate postgres as pg;

use std::error::Error as StdError;
use std::collections::btree_map::BTreeMap;
use self::pg::{SslMode, Connection};
use self::pg::error::ConnectError;
use super::super::Error;
use super::Model;

pub struct Dao<T: Model> {
    db: pg::Connection,
    t: T,
}

impl<T: Model> Dao<T> {
    pub fn new(t: T, url: &str, params: SslMode) -> Result<Dao<T>, ConnectError> {
        match Connection::connect(url, params) {
            Ok(v) => Ok(Dao { db: v, t: t }),
            Err(e) => Err(e),
        }

    }



    // pub fn execute(&self, query: &str, params: &[&ToSql]) -> Result<u64, String> {
    //     match self.db.execute(query, params) {
    //         Ok(v) => Ok(v),
    //         Err(e) => Err(e.description().to_string()),
    //     }
    // }

    // pub fn query(&self, query: &str, params: &[&ToSql]) -> Result<u64, String> {
    //     match self.db.execute(query, params) {
    //         Ok(v) => {},
    //         Err(e) => Err(e.description().to_string()),
    //     }
    // }
}

impl<T: super::Model> super::Dao<T> for Dao<T> {
    fn execute(&self,
               name: &'static str,
               _: BTreeMap<&'static str, super::Type>)
               -> Result<u64, Error> {
        match self.t.queries().get(name) {
            Some(query) => {
                // TODO args
                match self.db.execute(query, &[]) {
                    Ok(v) => Ok(v),
                    Err(e) => Err(Error::Db(e.description().to_string())),
                }
            }
            None => Err(Error::Db(format!("not find query by name {}", name))),
        }

    }
}
