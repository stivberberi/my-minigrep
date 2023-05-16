use std::env;
use std::process;

use minigrep::Config;
fn main() {
    let args: Vec<String> = env::args().collect(); // Have to annotate as Vec<String> because a collection could be of many different types

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        // no return value here, so don't use unwrap_or_else
        println!("Application error: {e}");
        process::exit(1);
    }
}
