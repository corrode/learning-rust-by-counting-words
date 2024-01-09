// New Tasks:
// - Poor man's performance profiling
// - Don't forget to run `cargo build --release` before profiling, e.g.
//   ```
//   cargo build --release --example block6
//   ./create_large_text.sh
//   /target/release/examples/block6 fixtures/large.txt
//   ```

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
            let line = line?; // Handle the Result, return error if any
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
    // Start time measurement
    let start = std::time::Instant::now();

    // Get all paths from CLI arguments
    let paths = env::args().skip(1).collect::<Vec<_>>();

    for path in paths {
        let file = File::open(&path)?;
        let reader = BufReader::new(file);

        let mut counter = Counter::default();
        counter.count(reader)?;

        println!("{counter} {path}");
    }

    // Print elapsed time.
    // You can compare it with the output of the `time wc` command.
    let elapsed = start.elapsed();
    println!(
        "Elapsed: {}.{:03}s",
        elapsed.as_secs(),
        elapsed.subsec_millis()
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_count() {
        let input = "Hello, world!\n";
        let reader = BufReader::new(input.as_bytes());

        let mut counter = Counter::default();
        counter.count(reader).unwrap();

        assert_eq!(counter.lines, 1);
        assert_eq!(counter.words, 2);
        assert_eq!(counter.chars, 13);
    }

    #[test]
    fn test_count_file() {
        let input = fs::read_to_string("fixtures/test.txt").unwrap();
        let reader = BufReader::new(input.as_bytes());

        let mut counter = Counter::default();
        counter.count(reader).unwrap();

        assert_eq!(counter.lines, 9);
        assert_eq!(counter.words, 128);
        assert_eq!(counter.chars, 694);
    }
}
