
extern crate hyper;

use self::hyper::server::{Handler, Request, Response};
use self::hyper::method::Method;
use self::hyper::status::StatusCode;

use super::context::Context;
use super::route::{Route, Handler as H};
use super::super::error::Result;

pub struct Router {
    routes: Vec<Route>,
}


impl Router {
    pub fn new() -> Router {
        let rts: Vec<Route> = Vec::new();
        Router { routes: rts }
    }
}

impl Handler for Router {
    fn handle(&self, req: Request, _: Response) {
        info!("handler");
        for r in &self.routes {
            if r.capture(&req) {
                let ctx = Context::new(&req);
                let hnd = r.handler;
                match hnd(ctx) {
                    Ok(code) => {
                        // TODO 200
                        return;
                    }
                    Err(err) => {
                        // TODO 500
                        error!("{}", err);
                        return;
                    }
                }

            }
        }
        error!("not match");
        // TODO 404
    }
}

impl Router {
    pub fn get(&mut self, pat: &'static str, hnd: H) {
        self.add(Method::Get, pat, hnd);
    }
    pub fn post(&mut self, pat: &'static str, hnd: H) {
        self.add(Method::Post, pat, hnd);
    }
    pub fn put(&mut self, pat: &'static str, hnd: H) {
        self.add(Method::Put, pat, hnd);
    }
    pub fn patch(&mut self, pat: &'static str, hnd: H) {
        self.add(Method::Patch, pat, hnd);
    }
    pub fn delete(&mut self, pat: &'static str, hnd: H) {
        self.add(Method::Delete, pat, hnd);
    }
    pub fn add(&mut self, mth: Method, pat: &'static str, hnd: H) {
        let r = Route::new(mth, pat, hnd);
        self.routes.push(r);
    }
}
