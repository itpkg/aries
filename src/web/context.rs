
extern crate hyper;

use self::hyper::server::Request;

pub struct Context {

}

impl Context {
    pub fn new(req: &Request) -> Context {
        info!("{} {} {}", req.version, req.method, req.uri);
        Context {}
    }

    // pub fn method(&self) -> Method {
    //     self.request.method
    // }
}

// pub struct Context<'a, 'b: 'a> {
//     request: Request<'a, 'b>,
// }
//
// impl<'a, 'b: 'a> Context<'a, 'b> {
//     pub fn new(req: Request<'a, 'b>) -> Context<'a, 'b> {
//         info!("{} {} {}", req.version, req.method, req.uri);
//         Context { request: req }
//     }
//
//     // pub fn method(&self) -> Method {
//     //     self.request.method
//     // }
// }
