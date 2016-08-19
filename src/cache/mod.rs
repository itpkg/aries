
pub mod memcache;
pub mod redis;

use rustc_serialize::{Encodable, Decodable};
use super::error::Result;

pub trait Cache {
    fn get<T: Decodable>(&self, key: &'static str) -> Result<T>;
    fn set<T: Encodable>(&self, key: &'static str, val: T, ttl: usize) -> Result<String>;
    fn delete(&self, key: &'static str) -> Result<String>;
    fn clear(&self) -> Result<String>;
    fn keys(&self) -> Result<Vec<String>>;
}
