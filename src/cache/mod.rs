
pub mod memcache;
pub mod redis;

use rustc_serialize::{Encodable, Decodable};
use super::error::Result;

pub trait Cache {
    fn get<T: Decodable>(&mut self, key: &'static str) -> Result<T>;
    fn set<T: Encodable>(&mut self, key: &'static str, val: T, ttl: usize) -> Result<String>;
    fn delete(&mut self, key: &'static str) -> Result<String>;
    fn clear(&mut self) -> Result<String>;
    fn keys(&mut self) -> Result<Vec<String>>;
}
