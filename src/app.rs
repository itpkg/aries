
use std::path;
use super::config;
use super::config::Loader;

pub struct Application {

}

impl Application {
    pub fn new() -> Application {
        Application {}
    }
}

impl Application {
    pub fn usage(&self) -> String {
        format!("
    {desc}

    Usage:
      {name} init [--config=FILE]
      {name} server [--config=FILE --threads=NUM --daemon]
      {name} worker [--config=FILE --threads=NUM --daemon]
      {name} db (create|console|migrate|seeds|rollback|drop) [--config=FILE]
      {name} cache (console|clear) [--config=FILE]
      {name} nginx [--https]
      {name} [--help]
      {name} [--version]

    Options:
      -h --help         Show help message.
      -v --version      Show version.
      -c --config=FILE  Config file's name [default: config.toml].
      -t --threads=NUM  Num of threads to run [default: 4].
      --https           With https support.
      -d --daemon       Daemon mode.
        ",
                name = env!("CARGO_PKG_NAME"),
                desc = env!("CARGO_PKG_DESCRIPTION"))
    }

    pub fn version(&self) -> String {
        format!("v{}", env!("CARGO_PKG_VERSION"))
    }

    pub fn init<'a>(&self, file: &'a str) {
        if path::Path::new(file).exists() {
            error!("file {} already exists.", file);
            return;
        }
        let db = config::Database {
            driver: "postgresql".to_string(),
            host: "localhost".to_string(),
            port: 5432,
            name: "aries_dev".to_string(),
            user: "postgres".to_string(),
            password: "".to_string(),
            extra: "sslmod=none".to_string(),
        };
        let httpd = config::Httpd {
            secrets: "change-me".to_string(),
            host: "http://localhost:8080".to_string(),
            port: 8080,
        };
        let cache = config::Cache {
            driver: "redis".to_string(),
            prefix: "cache://".to_string(),
            host: "localhost".to_string(),
            port: 6379,
            db: 0,
        };
        match config::toml::Loader::write(file, "httpd", httpd) {
            Some(e) => error!("{:?}", e),
            None => {}
        };
        match config::toml::Loader::write(file, "database", db) {
            Some(e) => error!("{:?}", e),
            None => {}
        };
        match config::toml::Loader::write(file, "cache", cache) {
            Some(e) => error!("{:?}", e),
            None => {}
        };
    }

    pub fn nginx(ssl: bool) {}
}
