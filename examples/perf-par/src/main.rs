#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::as_conversions)]
#![warn(clippy::nursery)]
#![warn(clippy::cargo)]

use wc::{counter, Counts};

use std::{fs::File, io::Read};

const STDIN: &str = "-";

/// Custom error type
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    if std::env::args().len() < 2 {
        eprintln!("Usage: wc <path>");
        return Err("Could not read file".into());
    }

    let mut total = Counts {
        words: 0,
        lines: 0,
        characters: 0,
    };

    for file in std::env::args().skip(1) {
        let mut reader: Box<dyn Read> = if file == STDIN {
            Box::new(std::io::stdin())
        } else {
            Box::new(File::open(&file)?)
        };

        let counts = counter(&mut reader)?;

        total += counts;
        println!(
            "{:>7} {:>7} {:>7} {}",
            counts.lines, counts.words, counts.characters, file
        );
    }

    println!(
        "{:>7} {:>7} {:>7} total",
        total.lines, total.words, total.characters
    );

    Ok(())
}
