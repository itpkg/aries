
extern crate mustache;

use std::{path, env};
use std::fs::OpenOptions;

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
            self.nginx(&args.flag_domain, args.flag_port, args.flag_https);
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
        info!("done.");
    }

    fn db_migrate<'a>(&self, _: &'a str) {
        // TODO
    }

    fn nginx<'a>(&self, domain: &'a str, port: usize, ssl: bool) {
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
