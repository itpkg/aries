#[macro_use]
extern crate log;
extern crate env_logger;
extern crate rustc_serialize;
extern crate aries;

use self::aries::app::Application;
use self::aries::console::Args;
use self::aries::engines::teamwork;
use self::aries::web::engine::Engine;

fn main() {
    env_logger::init().unwrap();
    info!("starting up");
    let app = &Application::new();

    // let en: &Engine = &teamwork::Engine::new();
    // app.register(en);

    let args = Args::new();
    app.start(args);
}
