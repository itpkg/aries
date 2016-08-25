
use super::super::error::Result;

pub struct Connection {

}

impl super::Connection for Connection {
    fn check(&self) -> bool {
        // TODO
        true
    }
    fn close(&self) -> Result<&'static str> {
        // TODO
        Ok("close postgresql connection")
    }
}
