extern crate hyper;

use std::sync::atomic::{AtomicUsize, Ordering};
use super::super::error::Error;

pub struct Server {
    port: u32,
}

impl Server {
    pub fn start(&self) -> Option<Error> {
        info!("start on http://localhost:{}", self.port);

        let counter = AtomicUsize::new(0);
        match hyper::server::Server::http("0.0.0.0:0") {
            Ok(srv) => {
                match srv.handle(move |req: hyper::server::Request, res: hyper::server::Response| {
                        counter.fetch_add(1, Ordering::Relaxed);
                    }) {
                    Ok(_) => None,
                    Err(err) => Some(Error::Hyper(err)),
                }
            }
            Err(err) => Some(Error::Hyper(err)),
        }
    }
}
