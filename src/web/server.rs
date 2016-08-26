// extern crate hyper;
//
// use self::hyper::server::{Request, Response};
// use super::super::error::Error;
// use super::context::Context;
// use super::router::Router;
// use super::handler::Handler;
//
// pub struct Server {
//     port: u32,
// }
//
// impl Server {
//     pub fn start<H: Handler>(&self, rt: Router<H>) -> Option<Error> {
//         info!("start on http://localhost:{}", self.port);
//
//         match hyper::server::Server::http("0.0.0.0:0") {
//             Ok(srv) => {
//                 match srv.handle(|mut req: Request, mut res: Response| {
//                     let ctx = Context::new(req, res);
//                     // TODO
//                 }) {
//                     Ok(_) => None,
//                     Err(err) => Some(Error::Hyper(err)),
//                 }
//             }
//             Err(err) => Some(Error::Hyper(err)),
//         }
//     }
//
//     pub fn handle(&self, _: Request, _: Response) {}
// }
