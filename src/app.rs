
extern crate mustache;

use std::{path, env, process};
use std::fs::OpenOptions;
use std::cell::RefCell;

use super::config::{self, Loader};
use super::cache::{self, Cache};
use super::console;
use super::web::engine::Engine;
use super::error::Result;

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

    pub fn start(&mut self, args: console::Args) -> Result<bool> {
        debug!("{:?}", args);
        if args.flag_version {
            println!("{}", self.version());
            return Ok(true);
        }
        if args.cmd_init {
            return self.init(&args.flag_config);
        }
        if args.cmd_db {
            if args.cmd_console {
                return self.db_console(&args.flag_config);
            }
            if args.cmd_create {
                return self.db_create(&args.flag_config);
            }
            if args.cmd_migrate {
                return self.db_migrate(&args.flag_config);
            }
            if args.cmd_rollback {
                return self.db_rollback(&args.flag_config);
            }
            if args.cmd_drop {
                return self.db_drop(&args.flag_config);
            }
        }
        if args.cmd_cache {
            if args.cmd_console {
                return self.cache_console(&args.flag_config);
            }
            if args.cmd_clear {
                return self.cache_clear(&args.flag_config);
            }
        }
        if args.cmd_nginx {
            return self.nginx(&args.flag_domain, args.flag_port, args.flag_https);
        }
        if args.cmd_worker {
            return self.worker(&args.flag_config, args.flag_threads, args.flag_daemon);
        }
        if args.cmd_server {
            return self.server(&args.flag_config,
                               args.flag_port,
                               args.flag_threads,
                               args.flag_daemon);

        }
        println!("{}", console::usage());
        Ok(true)
    }
}

impl<'a, CL: Loader> Application<'a, CL> {
    fn version(&self) -> String {
        format!("v{}", env!("CARGO_PKG_VERSION"))
    }

    fn init<'x>(&mut self, file: &'x str) -> Result<bool> {
        if path::Path::new(file).exists() {
            error!("file {} already exists.", file);
            return Ok(false);
        }
        let db = config::Database {
            driver: "postgresql".to_string(),
            host: "localhost".to_string(),
            port: 5432,
            name: "aries_dev".to_string(),
            user: "postgres".to_string(),
            password: "".to_string(),
            pool: 64,
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
            pool: 32,
        };

        try!(self.loader.set("httpd", httpd));
        try!(self.loader.set("db", db));
        try!(self.loader.set("cache", cache));
        // TODO load from engines

        try!(self.loader.write(file));
        info!("done.");
        Ok(true)
    }

    fn db_console<'x>(&self, _: &'x str) -> Result<bool> {
        // TODO
        Ok(true)
    }
    fn db_create<'x>(&self, _: &'x str) -> Result<bool> {
        // TODO
        Ok(true)
    }
    fn db_migrate<'x>(&self, _: &'x str) -> Result<bool> {
        // TODO
        Ok(true)
    }
    fn db_rollback<'x>(&self, _: &'x str) -> Result<bool> {
        // TODO
        Ok(true)
    }
    fn db_drop<'x>(&self, _: &'x str) -> Result<bool> {
        // TODO
        Ok(true)
    }
    fn cache_clear<'x>(&mut self, file: &'x str) -> Result<bool> {
        try!(self.loader.read(file));
        let ch: config::Cache = try!(self.loader.get("cache"));
        info!("connect to cache server");
        if ch.driver == "redis" {
            let mut c =
                try!(cache::redis::Cache::new(&ch.prefix, &ch.host, ch.port, ch.db, ch.pool));
            try!(c.clear());
            Ok(true)
        } else {
            error!("unsupported cache driver {}", ch.driver);
            Ok(false)
        }
    }
    fn cache_console<'x>(&mut self, file: &'x str) -> Result<bool> {
        try!(self.loader.read(file));
        let ch: config::Cache = try!(self.loader.get("cache"));
        info!("connect to cache server");
        if ch.driver == "redis" {
            try!(process::Command::new("redis-cli")
                .arg("-h")
                .arg(ch.host)
                .arg("-p")
                .arg(ch.port.to_string())
                .arg("-n")
                .arg(ch.db.to_string())
                .status());
            Ok(true)
        } else {
            error!("unsupported cache driver {}", ch.driver);
            Ok(false)
        }

    }
    fn server<'x>(&self, _: &'x str, _: usize, _: usize, _: bool) -> Result<bool> {
        // TODO
        Ok(true)
    }
    fn worker<'x>(&self, _: &'x str, _: usize, _: bool) -> Result<bool> {
        // TODO
        Ok(true)
    }

    fn nginx<'x>(&self, domain: &'x str, port: usize, ssl: bool) -> Result<bool> {
        let name = "nginx.conf";
        if path::Path::new(name).exists() {
            error!("file {} already exists.", name);
            return Ok(false);
        }

        let tpl = try!(mustache::compile_str(r#"
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
"#));
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

        try!(tpl.render_data(&mut try!(OpenOptions::new().create(true).write(true).open(name)),
                             &data));
        info!("done.");
        Ok(true)
    }
}
