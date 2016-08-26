extern crate redis;

use rustc_serialize::{Decodable, Encodable};
use rustc_serialize::json::{self, DecodeResult};

use self::redis::{RedisResult, Commands};
use super::super::error::Result;
use super::super::pool;



pub struct Cache<'a> {
    pool: pool::Pool<pool::redis::Connection, pool::redis::Driver>,
    prefix: &'a str,
}

impl<'a> Cache<'a> {
    pub fn new(prefix: &'a str, host: &'a str, port: u32, db: u8, len: usize) -> Result<Cache<'a>> {
        let url = &format!("redis://{}:{}/{}", host, port, db);
        let drv = try!(pool::redis::Driver::new(url));
        let pl = try!(pool::Pool::new(drv, len));

        Ok(Cache {
            pool: pl,
            prefix: prefix,
        })
    }

    fn key(&self, k: &'static str) -> String {
        format!("{}{}", self.prefix, k)
    }
}

impl<'a> super::Cache for Cache<'a> {
    fn get<T: Decodable>(&mut self, key: &'static str) -> Result<T> {
        let con = try!(self.pool.get());
        let rst: RedisResult<String> = con.item.get(self.key(key));
        self.pool.put(con);
        let buf = try!(rst);
        let val: DecodeResult<T> = json::decode(&buf);
        let val = try!(val);
        Ok(val)
    }
    fn set<T: Encodable>(&mut self, key: &'static str, val: T, ttl: usize) -> Result<String> {
        let buf = try!(json::encode(&val));
        let con = try!(self.pool.get());
        let rst: RedisResult<String> = con.item.set_ex(self.key(key), buf, ttl);
        self.pool.put(con);
        let rst = try!(rst);
        Ok(rst)
    }
    fn delete(&mut self, key: &'static str) -> Result<String> {
        let con = try!(self.pool.get());
        let rst: RedisResult<String> = con.item.del(self.key(key));
        self.pool.put(con);
        let rst = try!(rst);
        Ok(rst)
    }
    fn clear(&mut self) -> Result<String> {
        let con = try!(self.pool.get());
        let keys = try!(self.keys());
        if !keys.is_empty() {
            let rst: RedisResult<usize> = con.item.del(keys);
            try!(rst);
        }
        self.pool.put(con);
        Ok("ok".to_string())
    }
    fn keys(&mut self) -> Result<Vec<String>> {
        let con = try!(self.pool.get());
        let rst: RedisResult<Vec<String>> = con.item.keys(self.key("*"));
        self.pool.put(con);
        let rst = try!(rst);
        Ok(rst)
    }
}
