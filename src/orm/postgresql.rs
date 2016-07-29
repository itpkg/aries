extern crate postgres as pg;

use self::pg::{Connection, SslMode, error};
use super::DB;

pub struct Postgresql {
    con: Connection,
}

impl Postgresql {
    pub fn new(url: &str, params: SslMode) -> Result<Postgresql, error::ConnectError> {
        match Connection::connect(url, params) {
            Ok(v) => Ok(Postgresql { con: v }),
            Err(e) => Err(e),
        }

    }
}

impl DB for Postgresql {}
