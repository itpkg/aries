
extern crate mustache;

use rustc_serialize::{Decodable, Encodable};
use std::{path, env, any};
use std::fs::OpenOptions;
use std::collections::HashMap;
use std::cell::RefCell;

use super::config::{self, Loader};
use super::console;
use super::web::engine::Engine;

pub struct Application<'a, CL: Loader> {
    engines: RefCell<Vec<&'a Engine>>,
    loader: CL,
}

impl<'a, CL: Loader> Application<'a, CL> {
    pub fn new(loader: CL) -> Application<'a, CL> {
        Application {
            engines: RefCell::new(Vec::new()),
            loader: loader,
        }
    }
}

impl<'a, CL: Loader> Application<'a, CL> {
    pub fn register(&self, en: &'a Engine) {
        self.engines.borrow_mut().push(en);
    }

    pub fn start(&mut self, args: console::Args) {
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
                self.db_console(&args.flag_config);
                return;
            }
            if args.cmd_create {
                self.db_create(&args.flag_config);
                return;
            }
            if args.cmd_migrate {
                self.db_migrate(&args.flag_config);
                return;
            }
            if args.cmd_rollback {
                self.db_rollback(&args.flag_config);
                return;
            }
            if args.cmd_drop {
                self.db_drop(&args.flag_config);
                return;
            }
        }
        if args.cmd_cache {
            if args.cmd_console {
                self.cache_console(&args.flag_config);
                return;
            }
            if args.cmd_clear {
                self.cache_clear(&args.flag_config);
                return;
            }
        }
        if args.cmd_nginx {
            self.nginx(&args.flag_domain, args.flag_port, args.flag_https);
            return;
        }
        if args.cmd_worker {
            self.worker(&args.flag_config, args.flag_threads, args.flag_daemon);
            return;
        }
        if args.cmd_server {
            self.server(&args.flag_config,
                        args.flag_port,
                        args.flag_threads,
                        args.flag_daemon);
            return;
        }
        println!("{}", console::usage());
    }
}

impl<'a, CL: Loader> Application<'a, CL> {
    fn version(&self) -> String {
        format!("v{}", env!("CARGO_PKG_VERSION"))
    }

    fn init<'x>(&mut self, file: &'x str) {
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

        self.loader.put("httpd", httpd).unwrap();
        self.loader.put("db", db).unwrap();
        self.loader.put("cache", cache).unwrap();
        self.loader.write(file).unwrap();

        // match self.loader.write(file, "httpd", httpd) {
        //     Some(e) => error!("{:?}", e),
        //     None => {}
        // };
        // match self.loader.write(file, "database", db) {
        //     Some(e) => error!("{:?}", e),
        //     None => {}
        // };
        // match self.loader.write(file, "cache", cache) {
        //     Some(e) => error!("{:?}", e),
        //     None => {}
        // };
        info!("done.");
    }

    fn db_console<'x>(&self, _: &'x str) {
        // TODO
    }
    fn db_create<'x>(&self, _: &'x str) {
        // TODO
    }
    fn db_migrate<'x>(&self, _: &'x str) {
        // TODO
    }
    fn db_rollback<'x>(&self, _: &'x str) {
        // TODO
    }
    fn db_drop<'x>(&self, _: &'x str) {
        // TODO
    }
    fn cache_clear<'x>(&self, _: &'x str) {
        // TODO
    }
    fn cache_console<'x>(&self, _: &'x str) {
        // TODO
    }
    fn server<'x>(&self, _: &'x str, _: usize, _: usize, _: bool) {
        // TODO
    }
    fn worker<'x>(&self, _: &'x str, _: usize, _: bool) {
        // TODO
    }

    fn nginx<'x>(&self, domain: &'x str, port: usize, ssl: bool) {
        let name = "nginx.conf";
        if path::Path::new(name).exists() {
            error!("file {} already exists.", name);
            return;
        }

        let tpl = mustache::compile_str(r#"
  upstream {{host}}_prod {
    server localhost:{{port}} fail_timeout=0;
  }

  server {
    listen {{#ssl}}443{{/ssl}}{{^ssl}}80{{/ssl}};

  {{#ssl}}
    ssl  on;
    ssl_certificate  ssl/{{host}}-cert.pem;
    ssl_certificate_key  ssl/{{host}}-key.pem;
    ssl_session_timeout  5m;
    ssl_protocols  SSLv2 SSLv3 TLSv1;
    ssl_ciphers  RC4:HIGH:!aNULL:!MD5;
    ssl_prefer_server_ciphers  on;
  {{/ssl}}

    client_max_body_size 4G;
    keepalive_timeout 10;

    proxy_buffers 16 64k;
    proxy_buffer_size 128k;

    server_name {{host}};

    root {{&root}}/public;
    index index.html;

    access_log /var/log/nginx/{{host}}.access.log;
    error_log /var/log/nginx/{{host}}.error.log;

    location / {
      try_files $uri $uri/ /index.html?/$request_uri;
    }

  #  location ^~ /assets/ {
  #    gzip_static on;
  #    expires max;
  #    access_log off;
  #    add_header Cache-Control "public";
  #  }

    location ~* \.(?:css|js)$ {
      gzip_static on;
      expires max;
      access_log off;
      add_header Cache-Control "public";
    }
    location ~* \.(?:jpg|jpeg|gif|png|ico|cur|gz|svg|svgz|mp4|ogg|ogv|webm|htc)$ {
      expires 1M;
      access_log off;
      add_header Cache-Control "public";
    }

    location ~* \.(?:rss|atom)$ {
      expires 12h;
      access_log off;
      add_header Cache-Control "public";
    }

    location ~ ^/api/{{version}}(/?)(.*) {
      {{#ssl}}proxy_set_header X-Forwarded-Proto https;{{/ssl}}
      proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
      proxy_set_header Host $http_host;
      proxy_set_header X-Real-IP $remote_addr;
      proxy_redirect off;
      proxy_pass http://{{host}}_prod/$2$is_args$args;
      # limit_req zone=one;
    }

  }
"#)
            .unwrap();
        let data = mustache::MapBuilder::new()
            .insert_str("root",
                        env::current_dir()
                            .unwrap()
                            .to_str()
                            .unwrap_or("/var/www/htdocs"))
            .insert_str("host", domain)
            .insert_bool("ssl", ssl)
            .insert("port", &port)
            .expect("Failed to encode port")
            .build();

        tpl.render_data(&mut OpenOptions::new().create(true).write(true).open(name).unwrap(),
                         &data)
            .unwrap();
        info!("done.");
    }
}
