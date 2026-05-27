# Functional Style with try_fold

The same counting logic can be expressed as a single iterator chain using `try_fold`. This removes the mutable loop variable and propagates IO errors cleanly without `unwrap`.

`try_fold` is like `fold` but the accumulating closure returns a `Result`. If any step returns `Err`, the whole fold stops and returns that error. This makes it a natural fit for iterating over `BufRead::lines`, which yields `Result<String>` items.

Whether this style is more readable than the loop depends on who is reading it. The goal here is to see the approach and form your own opinion.

## Tasks

- Rewrite the `count` function using `lines()` and `try_fold`
- Return the three counts as a tuple `(lines, words, chars)` directly from the function
- Keep the existing tests passing

## Testing

Since `count` returns a `Result`, you can test error propagation directly. Implement `Read` on a struct that always returns `Err(...)` from its `read` method, wrap it in a `BufReader`, and pass it to `count`. Assert that the result is `Err`, not `Ok`. This confirms that errors are surfaced rather than swallowed, which is the main advantage of `try_fold` over `fold` with `unwrap`.
