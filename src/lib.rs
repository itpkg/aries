pub mod cache;
pub mod crypto;
pub mod i18n;
pub mod log;
pub mod orm;
pub mod web;
pub mod config;
pub mod inject;
pub mod jwt;

//-----------------------------------------------------------------------------
extern crate postgres as pgsql;

use std::{io, num, result};

type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Parse(num::ParseIntError),
    PgConn(pgsql::error::ConnectError),
    Pg(pgsql::error::Error),
    Db(String),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}

impl From<num::ParseIntError> for Error {
    fn from(err: num::ParseIntError) -> Error {
        Error::Parse(err)
    }
}

impl From<pgsql::error::ConnectError> for Error {
    fn from(err: pgsql::error::ConnectError) -> Error {
        Error::Postgresql(err)
    }
}
