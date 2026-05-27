use std::env;
use std::fmt::{self, Display, Formatter};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::AddAssign;

const STDIN: &str = "-";

/// Counts words, lines, and characters.
#[derive(Default, Copy, Clone)]
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

        // Account for newline characters stripped by lines()
        self.chars += self.lines;

        Ok(self)
    }
}

impl AddAssign for Counter {
    fn add_assign(&mut self, rhs: Self) {
        self.words += rhs.words;
        self.lines += rhs.lines;
        self.chars += rhs.chars;
    }
}

impl Display for Counter {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:>7} {:>7} {:>7}", self.lines, self.words, self.chars)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let paths = env::args().skip(1).collect::<Vec<_>>();
    let multiple = paths.len() > 1;
    let mut total = Counter::default();

    for path in &paths {
        let reader: Box<dyn BufRead> = if path == STDIN {
            Box::new(BufReader::new(std::io::stdin()))
        } else {
            Box::new(BufReader::new(File::open(path)?))
        };

        let mut counter = Counter::default();
        counter.count(reader)?;
        total += counter;

        println!("{counter} {path}");
    }

    if multiple {
        println!("{total} total");
    }

    Ok(())
}
