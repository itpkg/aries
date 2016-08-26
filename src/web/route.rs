extern crate hyper;

use self::hyper::method::Method;
use self::hyper::server::Request;
use self::hyper::status::StatusCode;


use super::super::error::Result;
use super::context::Context;
use super::render::Render;

pub type Handler = fn(c: Context) -> Result<StatusCode>;

pub struct Route {
    method: Method,
    pattern: &'static str,
    pub handler: Handler,
}

impl Route {
    pub fn new(mth: Method, pat: &'static str, hnd: Handler) -> Route {
        Route {
            method: mth,
            pattern: pat,
            handler: hnd,
        }
    }
}

impl Route {
    pub fn capture(&self, req: &Request) -> bool {
        if self.method != req.method {
            return false;
        }
        true
    }
}
