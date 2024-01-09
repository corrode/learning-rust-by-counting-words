// Tasks:
// * Count words with the given path
// * Get it to run
// * Code quality doesn’t matter

use std::env;
use std::fs;

fn main() {
    let path = env::args().nth(1).expect("Usage: block1 <path>");
    let contents = fs::read_to_string(&path).expect("Could not read file");

    let bytes = contents.len();
    let words = contents.split_whitespace().count();
    let lines = contents.lines().count();

    println!("{lines:>8} {words:>7} {bytes:7} {path}");
}
