use rayon::prelude::*;
use std::io::{BufReader, Read};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug, Default, Clone, Copy)]
pub struct Counts {
    pub words: usize,
    pub lines: usize,
    pub characters: usize,
}

impl std::ops::Add for Counts {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            words: self.words + other.words,
            lines: self.lines + other.lines,
            characters: self.characters + other.characters,
        }
    }
}

impl std::ops::AddAssign for Counts {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

fn count_chunk(chunk: &[u8]) -> Counts {
    let mut counts = Counts::default();
    let mut in_word = false;

    for &byte in chunk {
        counts.characters += 1;
        match byte {
            b'\n' => {
                counts.lines += 1;
                in_word = false;
            }
            _ if byte.is_ascii_whitespace() => in_word = false,
            _ => {
                if !in_word {
                    counts.words += 1;
                    in_word = true;
                }
            }
        }
    }

    counts
}

pub fn counter(input: impl Read) -> Result<Counts> {
    let mut reader = BufReader::with_capacity(1024 * 1024, input); // 1MB buffer
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer)?;

    // Split the buffer into chunks of about 1MB each
    let chunk_size = 1024 * 1024;
    let counts: Counts = buffer
        .par_chunks(chunk_size)
        .map(count_chunk)
        .reduce(|| Counts::default(), |a, b| a + b);

    Ok(counts)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counter() {
        let input = "Hello, World!\nThis is a test.";
        let counts = counter(input.as_bytes()).unwrap();
        assert_eq!(counts.words, 5);
        assert_eq!(counts.lines, 2);
        assert_eq!(counts.characters, 28);
    }
}
