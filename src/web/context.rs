extern crate hyper;

use self::hyper::server::{Request, Response};

pub enum METHOD {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE,
}


pub struct Context {
}

impl Context {
    pub fn new(req: Request, res: Response) -> Context {
        Context {}
    }
}
