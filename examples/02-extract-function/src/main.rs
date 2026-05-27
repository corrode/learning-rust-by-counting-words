use std::env;
use std::fs;

fn main() {
    let path = env::args().nth(1).expect("Usage: extract-function <path>");
    let contents = fs::read_to_string(path).expect("Could not read file");

    println!("Word count: {}", count(&contents));
}

fn count(contents: &str) -> usize {
    contents.split_whitespace().count()
}
