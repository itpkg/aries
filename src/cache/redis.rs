extern crate redis;

use self::redis::Commands;
use super::super::error::{Result, Error};
use self::redis::{IntoConnectionInfo, RedisResult, Connection, ToRedisArgs};
use rustc_serialize::{Decodable, Encodable};
use rustc_serialize::json::{self, ToJson, EncoderError, DecodeResult};


pub struct Cache {
    client: redis::Client,
    prefix: &'static str,
}

impl Cache {
    pub fn new(prefix: &'static str, host: &'static str, port: usize, db: u8) -> Result<Cache> {
        let url = &format!("redis://{}:{}/{}", host, port, db);
        let cli = try!(redis::Client::open(try!(url.into_connection_info())));
        Ok(Cache {
            client: cli,
            prefix: prefix,
        })
    }

    fn key(&self, k: &'static str) -> String {
        format!("{}{}", self.prefix, k)
    }
}

impl super::Cache for Cache {
    fn get<T: Decodable>(&self, key: &'static str) -> Result<T> {
        let con = try!(self.client.get_connection());
        let rst: RedisResult<String> = con.get(self.key(key));
        let buf = try!(rst);
        let val: DecodeResult<T> = json::decode(&buf);
        let val = try!(val);
        Ok(val)
    }
    fn set<T: Encodable>(&self, key: &'static str, val: T, ttl: usize) -> Result<String> {
        let buf = try!(json::encode(&val));
        let con = try!(self.client.get_connection());
        let rst: RedisResult<String> = con.set_ex(self.key(key), buf, ttl);
        let rst = try!(rst);
        Ok(rst)
    }
    fn delete(&self, key: &'static str) -> Result<String> {
        let con = try!(self.client.get_connection());
        let rst: RedisResult<String> = con.del(self.key(key));
        let rst = try!(rst);
        Ok(rst)
    }
    fn clear(&self) -> Result<String> {
        let con = try!(self.client.get_connection());
        for k in self.keys() {
            let rst: RedisResult<usize> = con.del(k);
            try!(rst);
        }
        Ok("ok".to_string())
    }
    fn keys(&self) -> Result<Vec<String>> {
        let con = try!(self.client.get_connection());
        let rst: RedisResult<Vec<String>> = con.keys(self.key("*"));
        let rst = try!(rst);
        Ok(rst)
    }
}
