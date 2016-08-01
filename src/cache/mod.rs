
pub mod memcache;
pub mod redis;


pub trait Provider {
    fn get(&self, key: &'static str) -> Vec<u8>;
    fn set(&self, key: &'static str, val: Vec<u8>, ttl: i64);
    fn delete(&self, key: &'static str);
    fn clear(&self );
    fn keys(&self) -> Vec<&'static str>;
}
