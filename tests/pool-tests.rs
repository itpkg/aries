extern crate redis;
extern crate aries;
extern crate postgres as pg;

use self::aries::pool;
use self::redis::{Commands, RedisResult};

#[test]
fn test_pool_redis() {
    let d = pool::redis::Driver::new("redis://localhost:6379/2").unwrap();
    let mut p = pool::Pool::new(d, 12).unwrap();
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


#[test]
fn test_pool_postgresql() {
    let d = pool::postgresql::Driver::new("postgres://postgres@localhost:5432/aries_test",
                                          pg::SslMode::None)
        .unwrap();
    let mut p = pool::Pool::new(d, 12).unwrap();
    for _ in 0..20 {
        let c = p.get().unwrap();
        c.item
            .execute("CREATE TABLE IF NOT EXISTS logs (
                    id              SERIAL PRIMARY KEY,
                    message         VARCHAR NOT NULL,
                    created         TIMESTAMP
                  )",
                     &[])
            .unwrap();

        p.put(c);
    }
}
