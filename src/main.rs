#[macro_use]
extern crate log;
extern crate env_logger;
extern crate rustc_serialize;
extern crate aries;

use self::aries::app::Application;
use self::aries::console::Args;
use self::aries::config;

// use self::aries::web::engine::Engine;
// use self::aries::engines::teamwork;


fn main() {
    env_logger::init().unwrap();
    info!("starting up");
    let mut app = Application::new(config::toml::Loader::new());

    // let en: &Engine = &teamwork::Engine::new();
    // app.register(en);
    let args = Args::new().unwrap();
    app.start(args).unwrap();
}
