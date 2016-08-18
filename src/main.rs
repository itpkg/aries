#[macro_use]
extern crate log;
extern crate env_logger;
extern crate rustc_serialize;

// use std::env;
// use super::app;

extern crate aries;


fn main() {
    env_logger::init().unwrap();
    info!("starting up");
    aries::console::run();
}
