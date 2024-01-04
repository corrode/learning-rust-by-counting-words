// Tasks:
// * Bonus: refactor the code a bit
// * Bonus: try a different programming paradigm (e.g. functional)

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let path = env::args().nth(1).expect("Usage: block1 <path>");
    let file = File::open(path).expect("Could not open file");
    let reader = BufReader::new(file);

    // Refactored the code a bit by using a function to count the words
    println!("Word count: {}", count(reader));
}

// Functional approach
fn count<R: BufRead>(reader: R) -> usize {
    reader
        .lines()
        .map(|line| line.expect("Could not read line"))
        .map(|line| line.split_whitespace().count())
        .sum()
}
