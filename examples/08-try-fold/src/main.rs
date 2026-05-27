use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn count<R: BufRead>(reader: R) -> Result<(usize, usize, usize), std::io::Error> {
    // try_fold threads errors out of the closure without unwrap
    reader.lines().try_fold((0, 0, 0), |mut counts, line| {
        let line = line?;
        counts.0 += 1;
        counts.1 += line.split_whitespace().count();
        counts.2 += line.chars().count();
        Ok(counts)
    })
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let paths = env::args().skip(1).collect::<Vec<_>>();

    for path in paths {
        let file = File::open(&path)?;
        let reader = BufReader::new(file);
        let (lines, words, chars) = count(reader)?;
        println!("{lines:>7} {words:>7} {chars:>7} {path}");
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
        let (lines, words, chars) = count(reader).unwrap();

        assert_eq!(lines, 1);
        assert_eq!(words, 2);
        assert_eq!(chars, 13);
    }

    #[test]
    fn test_count_file() {
        let input = fs::read_to_string("../../fixtures/test.txt").unwrap();
        let reader = BufReader::new(input.as_bytes());
        let (lines, words, chars) = count(reader).unwrap();

        assert_eq!(lines, 9);
        assert_eq!(words, 128);
        assert_eq!(chars, 694);
    }
}
