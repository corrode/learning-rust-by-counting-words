# Extract a Function

The first solution works but mixes I/O and logic in one place. Pulling the counting step into its own function makes it easier to read, test, and reuse later.

This is also a chance to try a functional style. Instead of a loop with a counter variable, you can express the same thing as a chain of iterator operations. Rust's iterators are lazy and compose well, so `split_whitespace().count()` reads almost like a description of what is happening.

Both styles are valid. The point is to notice that Rust supports them and to think about which you find clearer.

## Tasks

- Extract a `count(contents: String) -> usize` function
- Keep `main` focused on I/O: reading the file and printing the result
- Try writing `count` with an iterator chain instead of a loop
