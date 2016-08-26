extern crate aries;

use aries::cache::{self, Cache};

#[test]
fn test_cache_redis() {
    let mut ch = cache::redis::Cache::new("cache://", "localhost", 6379, 2, 12).unwrap();


    let key = "hi";
    let val = "Hello, Aries!";
    ch.set(key, val, 160).unwrap();
    let val1: String = ch.get(key).unwrap();
    println!("get {} = {}", key, val1);
    assert_eq!(val, val1);
    println!("cache keys: {:?}", ch.keys().unwrap());
    ch.clear().unwrap();

}
