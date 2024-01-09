// Tasks:
// * Bonus: refactor the code a bit
// * Bonus: try a different programming paradigm (e.g. functional)

use std::env;
use std::fs;

fn main() {
    let path = env::args().nth(1).expect("Usage: block1 <path>");
    let contents = fs::read_to_string(path).expect("Could not read file");

    // Use a function to count the words
    println!("Word count: {}", count(contents));
}

fn count(contents: String) -> usize {
    contents.split_whitespace().count()
}
