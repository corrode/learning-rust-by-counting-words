# Functional Style with try_fold

The same counting logic can be expressed as a single iterator chain using `try_fold`. This removes the mutable loop variable and propagates IO errors cleanly without `unwrap`.

`try_fold` is like `fold` but the accumulating closure returns a `Result`. If any step returns `Err`, the whole fold stops and returns that error. This makes it a natural fit for iterating over `BufRead::lines`, which yields `Result<String>` items.

Whether this style is more readable than the loop depends on who is reading it. The goal here is to see the approach and form your own opinion.

## Tasks

- Rewrite the `count` function using `lines()` and `try_fold`
- Return the three counts as a tuple `(lines, words, chars)` directly from the function
- Keep the existing tests passing
