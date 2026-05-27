# Stdin and Totals

Two small additions that make the tool more useful in practice.

When the path is `-`, read from standard input instead of a file. This is the Unix convention and lets the tool work in pipelines like `cat file.txt | wc -`. The type `Box<dyn BufRead>` lets you hold either a file reader or a stdin reader in the same variable without knowing which one at compile time.

When more than one file is given, print a total line at the end. `AddAssign` (`+=`) is the trait that makes `total += counts` work. Implementing it on your counter type makes accumulation straightforward.

## Tasks

- Treat the path `-` as a signal to read from `std::io::stdin`
- Use `Box<dyn BufRead>` to hold either kind of reader in the same binding
- Implement `AddAssign` on the counter type
- Print a total line when more than one file was processed
