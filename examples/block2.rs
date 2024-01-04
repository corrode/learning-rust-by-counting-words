// New Tasks:
// * Count characters
// * Count lines
// * Bonus: Focus on code structure, readability, and extensibility

use std::env;
use std::fmt::{self, Display, Formatter};
use std::fs::File;
use std::io::{BufRead, BufReader};

/// Counts words, lines, and characters
struct Counter {
    words: usize,
    lines: usize,
    chars: usize,
}

impl Counter {
    fn new() -> Self {
        Counter {
            words: 0,
            lines: 0,
            chars: 0,
        }
    }

    fn count<R: BufRead>(&mut self, reader: R) -> &mut Counter {
        for line in reader.lines() {
            let line = line.expect("Could not read line");
            self.lines += 1;
            self.words += line.split_whitespace().count();
            self.chars += line.chars().count();
        }

        self
    }
}

impl Display for Counter {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        // Same output as `wc` command
        write!(f, "{:>7} {:>7} {:>7}", self.lines, self.words, self.chars)
    }
}

fn main() {
    let path = env::args().nth(1).expect("Usage: block1 <path>");
    let file = File::open(&path).expect("Could not open file");
    let reader = BufReader::new(file);

    let mut counter = Counter::new();
    counter.count(reader);
    println!("{counter} {path}");
}
