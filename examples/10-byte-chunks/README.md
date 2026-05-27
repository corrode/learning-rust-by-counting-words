# Byte-level Reading

The line-by-line approach allocates a new `String` for every line. Reading raw bytes in fixed-size chunks avoids those allocations and can be significantly faster on large files.

Word boundaries are detected by watching for transitions between whitespace and non-whitespace bytes rather than splitting strings. Line count is just the number of `\n` bytes. Character count is the total number of bytes read. This is approximate for multibyte Unicode characters but matches how the system `wc` counts bytes by default.

Two other improvements: locking stdout once before writing instead of once per line removes repeated lock acquisitions, and spawning one thread per file lets multiple files be counted in parallel.

Note that this step counts bytes, not Unicode characters, so the numbers will differ from the earlier steps on non-ASCII input.

## Tasks

- Replace `BufRead::lines` with raw chunk reads using `Read::read` into a fixed buffer
- Count `\n` bytes for lines, whitespace transitions for words, and total bytes for characters
- Lock stdout with `io::stdout().lock()` before the loop
- Spawn one thread per file using `std::thread::spawn` and collect the results
