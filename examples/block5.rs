// New Tasks:
// There are no task in this block, but you can try to:
// * Try different paradigms (imperative vs functional vs declarative)
// * Test out some design patterns

use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn count<R: BufRead>(reader: R) -> Result<(usize, usize, usize), std::io::Error> {
    // Functional approach
    // (not particularly idiomatic but perhaps interesting)
    let result = reader
        .lines()
        .map(|line| line.unwrap())
        .fold((0, 0, 0), |mut counter, line| {
            counter.0 += 1;
            counter.1 += line.split_whitespace().count();
            counter.2 += line.chars().count();
            counter
        });

    Ok(result)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get all paths from CLI arguments
    let paths = env::args().skip(1).collect::<Vec<_>>();

    for path in paths {
        let file = File::open(&path)?;
        let reader = BufReader::new(file);
        let result = count(reader)?;
        println!("{:>7} {:>7} {:>7} {}", result.0, result.1, result.2, path);
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

        let result = count(reader).unwrap();

        assert_eq!(result.0, 1);
        assert_eq!(result.1, 2);
        assert_eq!(result.2, 13);
    }

    #[test]
    fn test_count_file() {
        let input = fs::read_to_string("fixtures/test.txt").unwrap();
        let reader = BufReader::new(input.as_bytes());

        let result = count(reader).unwrap();
        assert_eq!(result.0, 9);
        assert_eq!(result.1, 128);
        assert_eq!(result.2, 694);
    }
}
