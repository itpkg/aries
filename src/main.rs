#[macro_use]
extern crate log;
extern crate env_logger;
extern crate rustc_serialize;

extern crate aries;


fn main() {
    env_logger::init().unwrap();
    info!("starting up");
    let app = aries::app::Application::new();
    let args = aries::console::Args::new();
    app.start(args);
}
