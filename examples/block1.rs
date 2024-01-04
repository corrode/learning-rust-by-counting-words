// Tasks:
// * Count words with the given path
// * Get it to run
// * Code quality doesn’t matter

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let path = env::args().nth(1).expect("Usage: block1 <path>");
    let file = File::open(path).expect("Could not open file");
    let reader = BufReader::new(file);

    let mut count = 0;
    for line in reader.lines() {
        let line = line.expect("Could not read line");
        count += line.split_whitespace().count();
    }

    println!("Word count: {}", count);
}
