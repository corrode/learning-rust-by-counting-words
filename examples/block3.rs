// New Tasks:
// * Accept multiple files as input
// * Parse CLI arguments with `std`
// * Proper error handling with `Result`

use std::env;
use std::fmt::{self, Display, Formatter};
use std::fs::File;
use std::io::{BufRead, BufReader};

/// Counts words, lines, and characters
#[derive(Default)]
struct Counter {
    words: usize,
    lines: usize,
    chars: usize,
}

impl Counter {
    fn count<R: BufRead>(&mut self, reader: R) -> Result<&mut Counter, std::io::Error> {
        for line in reader.lines() {
            let line = line?;
            self.lines += 1;
            self.words += line.split_whitespace().count();
            self.chars += line.chars().count();
        }

        Ok(self)
    }
}

impl Display for Counter {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        // Same output as `wc` command
        write!(f, "{:>7} {:>7} {:>7}", self.lines, self.words, self.chars)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get all paths from CLI arguments
    let paths = env::args().skip(1).collect::<Vec<_>>();

    for path in paths {
        let file = File::open(&path)?;
        let reader = BufReader::new(file);

        let mut counter = Counter::default();
        counter.count(reader)?;

        println!("{counter} {path}");
    }

    Ok(())
}
