extern crate hyper;

use self::hyper::method::Method;

use super::context::{METHOD, Context};
use super::handler::Handler;
use super::super::error::Result;

pub struct Router<H: Handler> {
    handlers: Vec<H>,
}

impl<H: Handler> Router<H> {
    // fn get(&self, pat: &'static str, hnd: Vec<H>) {
    //     self.add(Method::Get, pat, hnd);
    // }
    // fn post<H: Handler>(&self, pat: &'static str, hnd: Vec<H>) {
    //     self.add(Method::Post, pat, hnd);
    // }
    // fn put<H: Handler>(&self, pat: &'static str, hnd: Vec<H>) {
    //     self.add(Method::Put, pat, hnd);
    // }
    // fn patch<H: Handler>(&self, pat: &'static str, hnd: Vec<H>) {
    //     self.add(Method::Patch, pat, hnd);
    // }
    // fn delete<H: Handler>(&self, pat: &'static str, hnd: Vec<H>) {
    //     self.add(Method::Delete, pat, hnd);
    // }

    fn add(&self, mth: Method, pat: &'static str, hnd: Vec<H>) {}
    // fn capture(ctx: Context);
}

// -----------------------------------------------------------------------------

// pub struct RegexRouter {
//
// }
//
// impl RegexRouter {
//     pub fn new() -> RegexRouter {
//         RegexRouter {}
//     }
// }
//
// impl Router for RegexRouter {
//     fn add<H: Handler>(&self, mth: Method, pat: &'static str, hnd: Vec<H>) {}
//     fn capture(ctx: Context) {}
// }
