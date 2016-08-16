extern crate toml;

use std::fs;
use std::io;
use std::io::Read;
use super::super::error::{Result, Error};

pub struct Config {
    value: toml::Table,
}

impl Config {
    fn new(file: &str) -> Result<Config> {
        info!("load config from file {:?}", file);


        let mut cfg = try!(fs::File::open(file));
        let mut buf = String::new();
        try!(cfg.read_to_string(&mut buf));
        let mut parser = toml::Parser::new(&buf);
        match parser.parse() {
            Some(val) => Ok(Config { value: val }),
            None => {
                let msg: &str = &format!("{:?}", parser.errors);
                Err(Error::Io(io::Error::new(io::ErrorKind::InvalidData, msg)))
            }
        }
    }
}
impl super::Config for Config {
    fn string(&self, key: &str, def_val: &str) -> &str {
        match self.value.get(key) {
            Some(val) => {
                // match val.as_str() {
                //     Some(v) => "",
                //     None => def_val,
                // }
                return def_val;
            }
            None => def_val,
        }
    }
    fn int(&self, key: &str, def_val: i64) -> i64 {
        match self.value.get(key) {
            Some(val) => {
                match val.as_integer() {
                    Some(v) => v,
                    None => def_val,
                }
            }
            None => def_val,
        }
    }
}
