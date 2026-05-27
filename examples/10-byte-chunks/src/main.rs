use std::env;
use std::fmt::{self, Display, Formatter};
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

// 1 MB read buffer
const READ_BUFFER_SIZE: usize = 1024 * 1024;

/// Counts words, lines, and bytes.
///
/// Words are counted by byte-level whitespace transitions, not Unicode splitting.
/// Characters are raw bytes. This matches what the system `wc` reports by default.
#[derive(Default)]
struct Counter {
    words: usize,
    lines: usize,
    bytes: usize,
}

impl Counter {
    fn count<R: BufRead>(&mut self, mut reader: R) -> Result<&mut Counter, std::io::Error> {
        let mut buf = [0; READ_BUFFER_SIZE];
        let mut in_word = false;

        loop {
            let n = reader.read(&mut buf)?;
            if n == 0 {
                break;
            }

            for &byte in &buf[..n] {
                self.bytes += 1;
                match byte {
                    b'\n' => {
                        self.lines += 1;
                        in_word = false;
                    }
                    b' ' | b'\t' | b'\r' => in_word = false,
                    _ => {
                        if !in_word {
                            self.words += 1;
                            in_word = true;
                        }
                    }
                }
            }
        }

        Ok(self)
    }
}

impl Display for Counter {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:>7} {:>7} {:>7}", self.lines, self.words, self.bytes)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let paths = env::args().skip(1).collect::<Vec<_>>();

    // Lock stdout once to avoid per-line lock overhead
    let stdout = std::io::stdout();
    let mut handle = stdout.lock();

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
        writeln!(handle, "{counter} {path}")?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count() {
        let input = "Hello, world!\n";
        let reader = BufReader::new(input.as_bytes());

        let mut counter = Counter::default();
        counter.count(reader).unwrap();

        assert_eq!(counter.lines, 1);
        assert_eq!(counter.words, 2);
        assert_eq!(counter.bytes, 14); // 13 chars + 1 newline byte
    }

    #[test]
    fn test_count_multiline() {
        let input = "one two\nthree four five\n";
        let reader = BufReader::new(input.as_bytes());

        let mut counter = Counter::default();
        counter.count(reader).unwrap();

        assert_eq!(counter.lines, 2);
        assert_eq!(counter.words, 5);
        assert_eq!(counter.bytes, 24);
    }
}
