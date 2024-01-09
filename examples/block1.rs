// Tasks:
// * Count words with the given path
// * Get it to run
// * Code quality doesn’t matter

use std::env;
use std::fs;

fn main() {
    let path = env::args().nth(1).expect("Usage: block1 <path>");
    let contents = fs::read_to_string(path).expect("Could not read file");
    let words = contents.split_whitespace().count();

    println!("Word count: {}", words);
}
