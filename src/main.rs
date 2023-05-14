use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect(); // Have to annotate as Vec<String> because a collection could be of many different types

    let query = &args[1]; // Program name is stored in args[0] so start indexing at 1
    let filename = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
}
