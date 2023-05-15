use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect(); // Have to annotate as Vec<String> because a collection could be of many different types

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    let contents =
        fs::read_to_string(config.file_path).expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        // 'static str because returned errors will always be String literals
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let query = args[1].clone(); // Program name is stored in args[0] so start indexing at 1
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}
