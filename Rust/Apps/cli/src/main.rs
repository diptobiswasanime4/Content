use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let filename = &args[2];

    println!("{:?}", args);
    println!("{} {}", query, filename);

    let contents = fs::read_to_string(filename)
    .expect("Failed to read file.");

    println!("{}", contents);
}

struct Config {
    query: String,
    file: String
}

fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let filename = args[2].clone();

    Config {query, filename}
}