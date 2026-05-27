use std::io::Read;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// Counts for words, lines, and characters
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

/// Count words, lines, and characters from any `Read` source.
///
/// Reads the input in fixed-size chunks to avoid allocating the entire file
/// into memory as a `String`.
pub fn counter(input: &mut impl Read) -> Result<Counts> {
    let mut counts = Counts::default();
    let mut in_word = false;
    let mut buf = [0u8; 4096];

    loop {
        let bytes_read = input.read(&mut buf)?;
        if bytes_read == 0 {
            break;
        }

        for &byte in &buf[..bytes_read] {
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
    }

    Ok(counts)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_with_reader() {
        let input = "Hello, World!";
        let mut reader = input.as_bytes();
        let counts = counter(&mut reader).unwrap();
        assert_eq!(counts.words, 2);
        assert_eq!(counts.lines, 0); // no newline at end
        assert_eq!(counts.characters, 13);
    }

    #[test]
    fn test_count_with_newline() {
        let input = "Hello, World!\n";
        let mut reader = input.as_bytes();
        let counts = counter(&mut reader).unwrap();
        assert_eq!(counts.words, 2);
        assert_eq!(counts.lines, 1);
        assert_eq!(counts.characters, 14);
    }
}
