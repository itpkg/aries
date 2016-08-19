extern crate toml;

use std::fs;
use std::fs::OpenOptions;
use std::io;
use std::io::{Read, Write};
use rustc_serialize::{Decodable, Encodable};

use super::super::error::{Result, Error};

pub struct Loader {
    root: toml::Table,
}
impl Loader {
    pub fn new() -> Loader {
        Loader { root: toml::Table::new() }
    }
}
impl super::Loader for Loader {
    fn read<'x>(&mut self, file: &'x str) -> Result<bool> {
        info!("read config from file {:?}", file);

        let mut cfg = try!(fs::File::open(file));
        let mut buf = String::new();
        try!(cfg.read_to_string(&mut buf));
        let mut parser = toml::Parser::new(&buf);
        match parser.parse() {
            Some(root) => {
                for (key, val) in root {
                    self.root.insert(key, val);
                }
                Ok(true)
            }
            None => {
                error!("bad toml format: {:?}", parser.errors);
                Err(Error::TomlParser(parser.errors.pop().unwrap()))
            }
        }
    }

    fn write<'x>(&self, file: &'x str) -> Result<bool> {
        info!("write config into file {}", file);
        let mut fd = try!(OpenOptions::new().create(true).write(true).open(file));
        try!(writeln!(fd, "{}", toml::Value::Table(self.root.clone()).to_string()));
        Ok(true)
    }

    fn get<T: Decodable>(&self, name: &'static str) -> Result<T> {
        match self.root.get(name) {
            Some(val) => {
                let val = val.clone();
                let mut dec = toml::Decoder::new(val);
                let cfg = try!(T::decode(&mut dec));
                Ok(cfg)
            }
            None => Err(Error::Io(io::Error::new(io::ErrorKind::InvalidData, ""))),
        }
    }

    fn set<'x, T: Encodable>(&mut self, name: &'x str, t: T) -> Result<bool> {
        let mut enc = toml::Encoder::new();
        try!(t.encode(&mut enc));
        self.root.insert(name.to_string(), toml::Value::Table(enc.toml));
        Ok(true)
    }
}
