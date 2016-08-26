
extern crate hyper;

use self::hyper::server::Server;
use super::super::error::Result;
use super::router::Router;

pub fn start(rt: Router, port: u32) -> Result<&'static str> {
    info!("start on http://localhost:{}", port);
    let http = try!(Server::http("0.0.0.0:0"));
    try!(http.handle(rt));
    Ok("done!")
}
