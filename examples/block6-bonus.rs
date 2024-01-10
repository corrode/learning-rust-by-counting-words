// Performance improvements being made:
// - Use `BufReader` to read the file line by line
// - Lock the `stdout` handle to avoid interleaved output
// - Read each file in a separate thread
//
// This is alternative way to measure elapsed time
// by using the `time` command from the shell:
// ```
// time cargo run --release --example block6-bonus fixtures/test.txt
// ```
//
// Suggestions for even faster output:
// - Use SIMD instructions to count the characters
//   https://github.com/expr-fi/fastlwc

use std::env;
use std::fmt::{self, Display, Formatter};
use std::fs::File;
use std::io::{BufRead, BufReader};

use std::io::Write; // Import the Write trait to lock stdout

// Fixed size buffer for modern hardware dimensions
const READ_BUFFER_SIZE: usize = 1024 * 1024;

/// Counts words, lines, and characters
#[derive(Default)]
struct Counter {
    words: usize,
    lines: usize,
    chars: usize,
}

impl Counter {
    fn count<R: BufRead>(&mut self, mut reader: R) -> Result<&mut Counter, std::io::Error> {
        let mut buf = [0; READ_BUFFER_SIZE];

        // Read the file in chunks
        loop {
            let n = reader.read(&mut buf)?;
            if n == 0 {
                break;
            }
            // Count the lines
            self.lines += buf[..n].iter().filter(|&&c| c == b'\n').count();

            // Count the words
            // This is a bit faster than `split_whitespace`
            // but it's just an estimation of `split_whitespace`
            self.words += buf[..n]
                .iter()
                .filter(|&&c| c == b' ' || c == b'\n' || c == b'\t' || c == b'\r')
                .count();

            // Count the characters
            self.chars += n;
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

    // Lock stdout to avoid interleaved output
    let stdout = std::io::stdout();
    let mut handle = stdout.lock();

    // Single threaded version
    // for path in paths {
    //     let file = File::open(&path)?;
    //     let reader = BufReader::new(file);

    //     let mut counter = Counter::default();
    //     counter.count(reader)?;

    //     // Print the result
    //     writeln!(handle, "{counter} {path}")?;
    // }

    // Multi threaded version
    let mut pool = Vec::new();
    for path in paths {
        let file = File::open(&path)?;
        let reader = BufReader::new(file);

        let mut counter = Counter::default();
        pool.push(std::thread::spawn(move || {
            counter.count(reader).unwrap();
            (counter, path)
        }));
    }

    for thread in pool {
        let (counter, path) = thread.join().unwrap();
        // Print the result
        writeln!(handle, "{counter} {path}")?;
    }

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
