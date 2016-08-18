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

    cmd_server: bool,
    cmd_worker: bool,
    cmd_db: bool,
    cmd_console: bool,
    cmd_create: bool,
    cmd_migrate: bool,
    cmd_seeds: bool,
    cmd_rollback: bool,
    cmd_drop: bool,
    cmd_cache: bool,
}

pub fn run() {
    let usage = format!("
{desc}

Usage:
  {name} server [--config=FILE --threads=NUM --daemon]
  {name} worker [--config=FILE --threads=NUM --daemon]
  {name} db (create|console|migrate|seeds|rollback|drop) [--config=FILE]
  {name} cache (console|clear) [--config=FILE]
  {name} [--help]
  {name} [--version]

Options:
  -h --help         Show help message.
  -v --version      Show version.
  -c --config=FILE  Config file's name [default: config.toml].
  -t --threads=NUM  Num of threads to run [default: 4].
  -d --daemon       Daemon mode.
    ",
                        name = env!("CARGO_PKG_NAME"),
                        desc = env!("CARGO_PKG_DESCRIPTION"));
    let args: Args = Docopt::new(usage.clone())
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());
    debug!("args: {:?}", args);
    if args.flag_version {
        println!(env!("CARGO_PKG_VERSION"));
        return;
    }
    if args.cmd_db {
        // TODO
    }
    if args.cmd_worker {
        // TODO
    }
    if args.cmd_server {
        // TODO
    }
    println!("{}", usage.clone());
}
