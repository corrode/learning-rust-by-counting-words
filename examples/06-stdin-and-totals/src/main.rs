use std::env;
use std::fmt::{self, Display, Formatter};
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::ops::AddAssign;

const STDIN: &str = "-";

#[derive(Default, Copy, Clone)]
struct Counts {
    words: usize,
    lines: usize,
    chars: usize,
}

impl AddAssign for Counts {
    fn add_assign(&mut self, rhs: Self) {
        self.words += rhs.words;
        self.lines += rhs.lines;
        self.chars += rhs.chars;
    }
}

impl Display for Counts {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:>7} {:>7} {:>7}", self.lines, self.words, self.chars)
    }
}

struct Counter {
    reader: Box<dyn BufRead>,
}

impl Counter {
    /// We use `impl BufRead + 'static` to accept any owned reader,
    /// and then we box it up internally.
    fn new(reader: impl BufRead + 'static) -> Self {
        Self {
            reader: Box::new(reader),
        }
    }

    fn count(self) -> Result<Counts, std::io::Error> {
        let mut counts = Counts::default();

        for line in self.reader.lines() {
            let line = line?;
            counts.lines += 1;
            counts.words += line.split_whitespace().count();
            counts.chars += line.chars().count();
        }

        counts.chars += counts.lines;
        Ok(counts)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let paths = env::args().skip(1).collect::<Vec<_>>();
    let multiple = paths.len() > 1;
    let mut total = Counts::default();

    for path in &paths {
        // Because `Counter` is not generic, both branches
        // evaluate to the exact same type: `Counter`.
        let counter = if path == STDIN {
            Counter::new(BufReader::new(io::stdin()))
        } else {
            Counter::new(BufReader::new(File::open(path)?))
        };

        let counts = counter.count()?;

        total += counts;
        println!("{counts} {path}");
    }

    if multiple {
        println!("{total} total");
    }

    Ok(())
}
