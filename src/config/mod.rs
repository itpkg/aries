pub mod toml;

use rustc_serialize::{Decodable, Encodable};

use super::error::{Result, Error};

#[derive(Debug, Clone)]
#[derive(RustcEncodable, RustcDecodable)]
pub struct Httpd {
    pub host: String,
    pub port: i32,
}

#[derive(Debug, Clone)]
#[derive(RustcEncodable, RustcDecodable)]
pub struct Redis {
    host: String,
    port: i32,
    db: i32,
}

#[derive(Debug, Clone)]
#[derive(RustcEncodable, RustcDecodable)]
pub struct Database {
    host: String,
    pub port: i32,
}

#[derive(Debug, Clone)]
#[derive(RustcEncodable, RustcDecodable)]
pub struct Configuration {
    httpd: Httpd,
    database: Database,
    redis: Redis,
}

pub trait Loader {
    fn read<T: Decodable>(file: &'static str, name: &'static str, t: T) -> Result<T>;
    fn write<T: Encodable>(file: &'static str, name: &'static str, t: T) -> Option<Error>;
}
