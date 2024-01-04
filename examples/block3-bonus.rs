// New Tasks:
// * Bonus: Print the total count over all files at the end
// * Bonus: support `stdin` (standard input), e.g. `cat file.txt | wc`

use std::env;
use std::fmt::{self, Display, Formatter};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::AddAssign;

const STDIN_FILE: &str = "-";

/// Counts words, lines, and characters
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
        // Same output as `wc` command
        write!(f, "{:>7} {:>7} {:>7}", self.lines, self.words, self.chars)
    }
}

struct Input {
    path: String,
    reader: Box<dyn BufRead>,
}

impl Input {
    fn new(path: String, reader: Box<dyn BufRead>) -> Self {
        Self { path, reader }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let paths = env::args().skip(1).collect::<Vec<_>>();

    let inputs = paths
        .into_iter()
        .map(|path| {
            // If the path is "-", use stdin; otherwise, open the file
            let reader = match path.as_str() {
                STDIN_FILE => Box::new(BufReader::new(std::io::stdin())) as Box<dyn BufRead>,
                _ => {
                    Box::new(BufReader::new(File::open(path.clone()).unwrap())) as Box<dyn BufRead>
                }
            };
            Input::new(path, reader)
        })
        .collect::<Vec<_>>();

    let mut total = Counter::default();

    for input in inputs {
        let mut counter = Counter::default();
        counter.count(input.reader)?;
        total += counter;

        println!("{counter} {}", input.path);
    }
    println!("{counter} total", counter = total);

    Ok(())
}
