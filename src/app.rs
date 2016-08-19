
use std::path;
use super::config;
use super::console;
use super::config::Loader;

pub struct Application {

}

impl Application {
    pub fn new() -> Application {
        Application {}
    }
}

impl Application {
    pub fn start(&self, args: console::Args) {
        debug!("{:?}", args);
        if args.flag_version {
            println!("{}", self.version());
            return;
        }
        if args.cmd_init {
            self.init(&args.flag_config);
            return;
        }
        if args.cmd_db {
            if args.cmd_console {
                // TODO
                return;
            }
            if args.cmd_create {
                // TODO
                return;
            }
            if args.cmd_migrate {
                // TODO
                return;
            }
            if args.cmd_rollback {
                // TODO
                return;
            }
            if args.cmd_drop {
                // TODO
                return;
            }
        }
        if args.cmd_cache {
            if args.cmd_console {
                // TODO
                return;
            }
            if args.cmd_clear {
                // TODO
                return;
            }
        }
        if args.cmd_nginx {
            self.nginx(args.flag_https);
            return;
        }
        if args.cmd_worker {
            // TODO
            return;
        }
        if args.cmd_server {
            // TODO
            return;
        }
        println!("{}", console::usage());
    }
}

impl Application {
    fn version(&self) -> String {
        format!("v{}", env!("CARGO_PKG_VERSION"))
    }

    fn init<'a>(&self, file: &'a str) {
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

    fn db_migrate<'a>(&self, file: &'a str) {
        // TODO
    }

    fn nginx<'a>(&self, ssl: bool) {
        // TODO
    }
}
