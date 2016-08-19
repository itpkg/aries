pub mod toml;

use rustc_serialize::{Decodable, Encodable};

use super::error::{Result, Error};

#[derive(Debug, Clone)]
#[derive(RustcEncodable, RustcDecodable)]
pub struct Httpd {
    pub secrets: String,
    pub host: String,
    pub port: i32,
}

#[derive(Debug, Clone)]
#[derive(RustcEncodable, RustcDecodable)]
pub struct Cache {
    pub driver: String,
    pub prefix: String,
    pub host: String,
    pub port: usize,
    pub db: u8,
}

#[derive(Debug, Clone)]
#[derive(RustcEncodable, RustcDecodable)]
pub struct Database {
    pub driver: String,
    pub host: String,
    pub port: i32,
    pub name: String,
    pub user: String,
    pub password: String,
    pub extra: String,
}


pub trait Loader {
    fn read<T: Decodable>(file: &'static str, name: &'static str, t: T) -> Result<T>;
    fn write<'a, 'b, T: Encodable>(file: &'a str, name: &'b str, t: T) -> Option<Error>;
}
