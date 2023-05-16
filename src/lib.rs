use std::error::Error;
use std::fs;
pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        // 'static str because returned errors will always be String literals
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let query = args[1].clone(); // Program name is stored in args[0] so start indexing at 1
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    for line in search(&config.query, &contents) {
        println!("{line}");
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // Returned vector will contain reference to the contents, so include the same lifetime
    let mut results: Vec<&str> = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

// pub fn search_case_insensitive();

// unit tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "Rust:\nsafe, fast, productive.\nPick three\nDuct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
