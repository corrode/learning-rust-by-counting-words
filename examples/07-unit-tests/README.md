# Unit Tests and Linting

Tests give you confidence that the code does what you think it does, and that changes do not break things that already work. Rust's test framework is built in and needs no extra dependencies.

Mark a function with `#[test]` and Cargo will run it when you call `cargo test`. Put tests in a `#[cfg(test)]` module so they are only compiled when testing. You can pass an in-memory string to your counter instead of opening a file, which keeps tests fast and self-contained.

`cargo clippy` is the Rust linter. It catches common mistakes, suggests more idiomatic patterns, and often explains why. Running it regularly is a good habit.

## Tasks

- Add a `#[cfg(test)]` module with at least two test functions
- Test word, line, and character counts with a short in-memory string
- Test against the fixture file to catch regressions
- Run `cargo clippy` and address any warnings

## Testing

By now your test module should cover at least: an empty string, a single word, multiple lines, and a file read from disk. Use `assert_eq!(actual, expected, "description")` rather than plain `assert!` so you see both the expected and actual values when a test fails. Group related tests together and give each one a name that describes what it checks, not how it does it.

A test for the file fixture pins down the expected values and will catch any regression you introduce while refactoring.
