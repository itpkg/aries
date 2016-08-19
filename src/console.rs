extern crate rustc_serialize;
extern crate docopt;

use self::docopt::Docopt;

#[derive(Debug, RustcDecodable)]
struct Args {
    flag_help: bool,
    flag_version: bool,
    flag_config: String,
    flag_threads: usize,
    flag_daemon: bool,
    flag_https: bool,

    cmd_init: bool,
    cmd_server: bool,
    cmd_worker: bool,
    cmd_db: bool,
    cmd_cache: bool,
    cmd_nginx: bool,

    cmd_console: bool,
    cmd_create: bool,
    cmd_migrate: bool,
    cmd_seeds: bool,
    cmd_rollback: bool,
    cmd_drop: bool,

    cmd_clear: bool,
}

pub fn run() {
    let app = super::app::Application::new();
    let args: Args = Docopt::new(app.usage())
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());
    debug!("{:?}", args);
    if args.flag_version {
        println!("{}", app.version());
        return;
    }
    if args.cmd_init {
        app.init(&args.flag_config);
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
        if args.flag_https {
            // TODO
        }
        // TODO
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
    println!("{}", app.usage());
}
