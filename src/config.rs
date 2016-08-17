extern crate toml;

use std::fs;
use std::io::Read;
use rustc_serialize::Decodable;
use super::error::{Result, Error};

#[derive(Debug)]
#[derive(RustcEncodable, RustcDecodable)]
pub struct Httpd {
    // secret: &'static str,
    port: i32, // host: &'static str,
}
#[derive(Debug)]
#[derive(RustcEncodable, RustcDecodable)]
pub struct Redis {
    // host: &'static str,
    port: i32,
    db: i32,
}
#[derive(Debug)]
#[derive(RustcEncodable, RustcDecodable)]
pub struct Database {
    // host: &'static str,
    port: i32,
}
#[derive(Debug)]
#[derive(RustcEncodable, RustcDecodable)]
pub struct Configuration {
    Httpd: Httpd,
    Database: Database,
    Redis: Redis,
}

impl Configuration {
    pub fn from_toml(file: &'static str) -> Result<Configuration> {
        info!("read config from file {:?}", file);

        let mut cfg = try!(fs::File::open(file));
        let mut buf = String::new();
        try!(cfg.read_to_string(&mut buf));
        let mut parser = toml::Parser::new(&buf);
        match parser.parse() {
            Some(val) => {
                let mut decoder = toml::Decoder::new(toml::Value::Table(val));
                let cfg = try!(Configuration::decode(&mut decoder));
                Ok(cfg)
            }
            None => {
                error!("bad toml format: {:?}", parser.errors);
                Err(Error::TomlParser(parser.errors.pop().unwrap()))
            }
        }



    }
    fn to_toml(&self, file: &'static str) {
        info!("write config from file {:?}", file);
    }
}
