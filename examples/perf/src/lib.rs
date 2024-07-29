use std::io::{BufReader, Read};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// Counts for words, lines, and characters
#[derive(Debug, Clone, Copy)]
pub struct Counts {
    pub words: usize,
    pub lines: usize,
    pub characters: usize,
}

// Implement addition for Counts
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

fn read_contents(input: &mut impl Read) -> Result<String> {
    let mut content = String::new();
    let mut reader = BufReader::new(input);
    reader.read_to_string(&mut content)?;
    Ok(content)
}

/// Count words, lines, and characters
/// Optimized for performance
fn count_all(mut input: impl Read) -> Result<Counts> {
    let mut counts = Counts {
        words: 0,
        lines: 0,
        characters: 0,
    };
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

/// Count words, lines, and characters
pub fn counter(input: &mut impl Read) -> Result<Counts> {
    let contents = read_contents(input)?;
    Ok(count_all(contents.as_bytes())?)
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_count_with_reader() {
        let input = "Hello, World!";
        let mut reader = input.as_bytes();
        let counts = super::counter(&mut reader).unwrap();
        assert_eq!(counts.words, 2);
        assert_eq!(counts.lines, 1);
        assert_eq!(counts.characters, 13);
    }
}
