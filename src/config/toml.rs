extern crate toml;

use std::fs;
use std::fs::OpenOptions;
use std::io;
use std::io::{Read, Write};
use rustc_serialize::{Decodable, Encodable};

use super::super::error::{Result, Error};

pub struct Loader {

}
impl super::Loader for Loader {
    fn read<T: Decodable>(file: &'static str, name: &'static str, _: T) -> Result<T> {
        info!("read config from file {:?}", file);

        let mut cfg = try!(fs::File::open(file));
        let mut buf = String::new();
        try!(cfg.read_to_string(&mut buf));
        let mut parser = toml::Parser::new(&buf);
        match parser.parse() {
            Some(root) => {
                match root.get(name) {
                    Some(val) => {
                        let val = val.clone();
                        let mut dec = toml::Decoder::new(val);
                        let cfg = try!(T::decode(&mut dec));
                        Ok(cfg)
                    }
                    None => Err(Error::Io(io::Error::new(io::ErrorKind::InvalidData, ""))),
                }
            }
            None => {
                error!("bad toml format: {:?}", parser.errors);
                Err(Error::TomlParser(parser.errors.pop().unwrap()))
            }
        }
    }

    fn write<T: Encodable>(file: &'static str, name: &'static str, t: T) -> Option<Error> {
        info!("write config to file {:?}", file);
        let mut enc = toml::Encoder::new();
        match t.encode(&mut enc) {
            Ok(_) => {
                let mut root = toml::Table::new();
                root.insert(name.to_string(), toml::Value::Table(enc.toml));
                match OpenOptions::new().create(true).write(true).append(true).open(file) {
                    Ok(fd) => {
                        let mut fd = fd;
                        match writeln!(fd, "{}", toml::Value::Table(root).to_string()) {
                            Ok(_) => None,
                            Err(e) => Some(Error::Io(e)),
                        }
                    }
                    Err(er) => Some(Error::Io(er)),
                }
            }
            Err(e) => Some(Error::Toml(e)),
        }
    }
}
