extern crate postgres as pg;

use super::super::error::Result;

pub struct Connection {
    pub item: pg::Connection,
}

impl super::Connection for Connection {
    fn check(&self) -> bool {
        match self.item.query("SELECT VERSION()", &[]) {
            Ok(_) => true,
            Err(e) => {
                error!("bad in check postgresql connection: {}", e);
                false
            }
        }
    }
    fn close(&self) -> Result<&'static str> {
        // TODO
        Ok("close postgresql connection")
    }
}

pub struct Driver<'a> {
    url: &'a str,
    mode: pg::SslMode<'a>,
}

impl<'a> Driver<'a> {
    pub fn new(url: &'a str, mode: pg::SslMode<'a>) -> Result<Driver<'a>> {
        Ok(Driver {
            url: url,
            mode: mode,
        })
    }
}

impl<'a> super::Driver<Connection> for Driver<'a> {
    fn open(&self) -> Result<Connection> {
        // let mode = Box::new(self.mode);
        // FIXME
        let con = try!(pg::Connection::connect(self.url, pg::SslMode::None));
        Ok(Connection { item: con })
    }
}
