#pragma once

#define ARIES_NGINX_CONF                                                       \
  R"(
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
)"
