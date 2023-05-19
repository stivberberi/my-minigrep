use std::env;
use std::process;

use minigrep::Config;
fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        // no return value here, so don't use unwrap_or_else
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
