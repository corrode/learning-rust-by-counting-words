# Timing

Before optimizing anything you need to know how slow it is. This step wraps the implementation in a simple timer using `std::time::Instant`.

Always build in release mode before comparing timings. Debug builds include overflow checks, extra assertions, and no inlining, which makes them significantly slower than what users actually run. Use `cargo build --release` and then run the binary directly.

Compare your result against the system `wc` with `time wc fixtures/large.txt`. Run `./create_large_text.sh` first to generate the large fixture file.

## Tasks

- Record the start time with `Instant::now` before processing files
- Print the elapsed time after all output using `elapsed.as_secs()` and `elapsed.subsec_millis()`
- Build with `cargo build --release` and run on the large fixture
- Compare the timing against the system `wc`

## Testing

You can test that the timing output is present by running the binary as a subprocess and checking that stdout contains `"Elapsed:"`. The actual number will vary, but the line should always be there. This pattern of spawning a binary and asserting on its stdout is useful for any CLI tool and scales well as the program grows.
