extern crate postgres as pgsql;
extern crate toml;

use std::{io, num, result, fmt, error};

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    TomlDecode(toml::DecodeError),
    TomlParser(toml::ParserError),
    Parse(num::ParseIntError),
    PgConn(pgsql::error::ConnectError),
    Pg(pgsql::error::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Io(ref err) => err.fmt(f),
            Error::TomlDecode(ref err) => err.fmt(f),
            Error::TomlParser(ref err) => err.fmt(f),
            Error::Parse(ref err) => err.fmt(f),
            Error::PgConn(ref err) => err.fmt(f),
            Error::Pg(ref err) => err.fmt(f),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Io(ref err) => err.description(),
            Error::TomlDecode(ref err) => err.description(),
            Error::TomlParser(ref err) => err.description(),
            Error::Parse(ref err) => err.description(),
            Error::PgConn(ref err) => err.description(),
            Error::Pg(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::Io(ref err) => Some(err),
            Error::TomlDecode(ref err) => Some(err),
            Error::TomlParser(ref err) => Some(err),
            Error::Parse(ref err) => Some(err),
            Error::PgConn(ref err) => Some(err),
            Error::Pg(ref err) => Some(err),
        }
    }
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
        Error::PgConn(err)
    }
}


impl From<toml::DecodeError> for Error {
    fn from(err: toml::DecodeError) -> Error {
        Error::TomlDecode(err)
    }
}


impl From<toml::ParserError> for Error {
    fn from(err: toml::ParserError) -> Error {
        Error::TomlParser(err)
    }
}
