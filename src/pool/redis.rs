extern crate redis;

use self::redis::{IntoConnectionInfo, RedisResult};
use super::super::error::Result;


pub struct Connection {
    pub item: redis::Connection,
}

impl super::Connection for Connection {
    fn check(&self) -> bool {
        let rst: RedisResult<String> = redis::cmd("PING").query(&self.item);
        match rst {
            Ok(_) => true,
            Err(e) => {
                error!("bad in check redis connection: {}", e);
                false
            }
        }
    }
    fn close(&self) -> Result<&'static str> {
        // TODO
        Ok("close redis connection")
    }
}

pub struct Driver {
    client: redis::Client,
}

impl Driver {
    pub fn new<'x>(url: &'x str) -> Result<Driver> {
        let cli = try!(redis::Client::open(try!(url.into_connection_info())));
        Ok(Driver { client: cli })
    }
}

impl super::Driver<Connection> for Driver {
    fn open(&self) -> Result<Connection> {
        let con = try!(self.client.get_connection());
        Ok(Connection { item: con })
    }
}
