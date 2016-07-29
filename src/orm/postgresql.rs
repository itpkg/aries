extern crate postgres as pg;

use std::error::Error;
use self::pg::{Connection, SslMode};
use self::pg::types::ToSql;
use self::pg::error::ConnectError;


pub struct Postgresql {
    db: Connection,
}

impl Postgresql {
    pub fn new(url: &str, params: SslMode) -> Result<Postgresql, ConnectError> {
        match Connection::connect(url, params) {
            Ok(v) => Ok(Postgresql { db: v }),
            Err(e) => Err(e),
        }

    }

    pub fn execute(&self, query: &str, params: &[&ToSql]) -> Result<u64, String> {
        match self.db.execute(query, params) {
            Ok(v) => Ok(v),
            Err(e) => Err(e.description().to_string()),
        }
    }
}

impl super::DB for Postgresql {}
