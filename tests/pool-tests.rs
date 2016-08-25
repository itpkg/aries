extern crate redis;
extern crate aries;

use self::aries::pool;
use self::redis::{Commands, RedisResult};

#[test]
fn test_pool_redis() {
    let d = pool::redis::Driver::new("localhost", 6379, 2).unwrap();
    let mut p = pool::Pool::new(12, d);
    for i in 0..500 {
        let c = p.get().unwrap();
        let key = format!("test://hi/{}", i);
        let val = format!("Hello, aries {}!", i);
        // println!("set {} = {}", key, val);
        let rst: RedisResult<String> = c.item.set(key, val);
        rst.unwrap();
        p.put(c);
    }
}
