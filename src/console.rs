extern crate rustc_serialize;
extern crate docopt;

use self::docopt::Docopt;
use super::error::Result;

#[derive(Debug, RustcDecodable)]
pub struct Args {
    pub flag_help: bool,
    pub flag_version: bool,
    pub flag_config: String,
    pub flag_threads: usize,
    pub flag_port: usize,
    pub flag_domain: String,
    pub flag_daemon: bool,
    pub flag_https: bool,

    pub cmd_init: bool,
    pub cmd_server: bool,
    pub cmd_worker: bool,
    pub cmd_db: bool,
    pub cmd_cache: bool,
    pub cmd_nginx: bool,

    pub cmd_console: bool,
    pub cmd_create: bool,
    pub cmd_migrate: bool,
    pub cmd_seeds: bool,
    pub cmd_rollback: bool,
    pub cmd_drop: bool,

    pub cmd_clear: bool,
}

impl Args {
    pub fn new() -> Result<Args> {
        let dec = try!(Docopt::new(usage()));
        let args: Args = try!(dec.decode());
        Ok(args)
    }
}

// -----------------------------------------------------------------------------
pub fn usage() -> String {
    format!("
{desc}

Usage:
  {name} init [--config=FILE]
  {name} server [--port=PORT --config=FILE --threads=NUM --daemon]
  {name} worker [--config=FILE --threads=NUM --daemon]
  {name} db (create|console|migrate|seeds|rollback|drop) [--config=FILE]
  {name} cache (console|clear) [--config=FILE]
  {name} nginx [--https --port=PORT --domain=NAME]
  {name} [--help]
  {name} [--version]

Options:
  -h --help         Show help message.
  -v --version      Show version.
  -c --config=FILE  Config file's name [default: config.toml].
  -p --port=PORT     Port to listen [default: 8080].
  -t --threads=NUM  Num of threads to run [default: 4].
  --https           With https support.
  --domain=NAME     Domain name [default: www.change-me.com].
  -d --daemon       Daemon mode.
    ",
            name = env!("CARGO_PKG_NAME"),
            desc = env!("CARGO_PKG_DESCRIPTION"))
}
