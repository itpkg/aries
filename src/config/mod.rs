pub mod toml;

use rustc_serialize::{Decodable, Encodable};

use super::error::Result;

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
    pub port: u32,
    pub db: u8,
    pub pool: usize,
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
    pub pool: usize,
    pub extra: String,
}


pub trait Loader {
    fn read<'x>(&mut self, file: &'x str) -> Result<bool>;
    fn write<'x>(&self, file: &'x str) -> Result<bool>;
    fn get<T: Decodable>(&self, name: &'static str) -> Result<T>;
    fn set<'x, T: Encodable>(&mut self, name: &'x str, t: T) -> Result<bool>;
}
