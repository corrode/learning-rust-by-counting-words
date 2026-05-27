# Count Words

Your first program reads a file and counts the words in it.

A word is any sequence of characters separated by whitespace. The standard library method `split_whitespace` handles this: it splits on any combination of spaces, tabs, and newlines, and ignores leading and trailing whitespace.

You get the file path from the command line using `std::env::args`, and read the whole file into a string with `std::fs::read_to_string`.

Run it against the fixture file to see the result:

    cargo run -- ../../fixtures/test.txt

## Tasks

- Get the file path from the first command line argument
- Read the file contents into a string
- Count the words by splitting on whitespace
- Print the word count

## Testing

Run your program and compare the word count against the system `wc -w`:

    cargo run -- ../../fixtures/test.txt
    wc -w ../../fixtures/test.txt

The numbers should match.
