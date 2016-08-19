extern crate postgres as pgsql;
extern crate toml;
extern crate hyper;
extern crate redis;
extern crate rustc_serialize;
extern crate docopt;
extern crate mustache;

use std::{io, num, result, fmt, error};
use rustc_serialize::json;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Toml(toml::Error),
    TomlDecode(toml::DecodeError),
    TomlParser(toml::ParserError),
    Parse(num::ParseIntError),
    PgConn(pgsql::error::ConnectError),
    Pg(pgsql::error::Error),
    Hyper(hyper::Error),
    Redis(redis::RedisError),
    JsonDecoder(json::DecoderError),
    JsonEncoder(json::EncoderError),
    Docopt(docopt::Error),
    Mustache(mustache::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Io(ref err) => err.fmt(f),
            Error::Toml(ref err) => err.fmt(f),
            Error::TomlDecode(ref err) => err.fmt(f),
            Error::TomlParser(ref err) => err.fmt(f),
            Error::Parse(ref err) => err.fmt(f),
            Error::PgConn(ref err) => err.fmt(f),
            Error::Pg(ref err) => err.fmt(f),
            Error::Hyper(ref err) => err.fmt(f),
            Error::Redis(ref err) => err.fmt(f),
            Error::JsonDecoder(ref err) => err.fmt(f),
            Error::JsonEncoder(ref err) => err.fmt(f),
            Error::Docopt(ref err) => err.fmt(f),
            Error::Mustache(ref err) => err.fmt(f),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Io(ref err) => err.description(),
            Error::Toml(ref err) => err.description(),
            Error::TomlDecode(ref err) => err.description(),
            Error::TomlParser(ref err) => err.description(),
            Error::Parse(ref err) => err.description(),
            Error::PgConn(ref err) => err.description(),
            Error::Pg(ref err) => err.description(),
            Error::Hyper(ref err) => err.description(),
            Error::Redis(ref err) => err.description(),
            Error::JsonDecoder(ref err) => err.description(),
            Error::JsonEncoder(ref err) => err.description(),
            Error::Docopt(ref err) => err.description(),
            Error::Mustache(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::Io(ref err) => Some(err),
            Error::Toml(ref err) => Some(err),
            Error::TomlDecode(ref err) => Some(err),
            Error::TomlParser(ref err) => Some(err),
            Error::Parse(ref err) => Some(err),
            Error::PgConn(ref err) => Some(err),
            Error::Pg(ref err) => Some(err),
            Error::Hyper(ref err) => Some(err),
            Error::Redis(ref err) => Some(err),
            Error::JsonDecoder(ref err) => Some(err),
            Error::JsonEncoder(ref err) => Some(err),
            Error::Docopt(ref err) => Some(err),
            Error::Mustache(ref err) => Some(err),
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


impl From<toml::Error> for Error {
    fn from(err: toml::Error) -> Error {
        Error::Toml(err)
    }
}

impl From<hyper::Error> for Error {
    fn from(err: hyper::Error) -> Error {
        Error::Hyper(err)
    }
}


impl From<redis::RedisError> for Error {
    fn from(err: redis::RedisError) -> Error {
        Error::Redis(err)
    }
}

impl From<json::DecoderError> for Error {
    fn from(err: json::DecoderError) -> Error {
        Error::JsonDecoder(err)
    }
}

impl From<json::EncoderError> for Error {
    fn from(err: json::EncoderError) -> Error {
        Error::JsonEncoder(err)
    }
}


impl From<docopt::Error> for Error {
    fn from(err: docopt::Error) -> Error {
        Error::Docopt(err)
    }
}


impl From<mustache::Error> for Error {
    fn from(err: mustache::Error) -> Error {
        Error::Mustache(err)
    }
}
