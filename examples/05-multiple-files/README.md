# Multiple Files

Real programs handle errors and accept more than one argument. This step adds both.

`main` can return `Result<(), Box<dyn Error>>`. That lets you use `?` to propagate any error that occurs, instead of calling `unwrap` and crashing. The error is printed automatically if `main` returns `Err`.

Collecting all arguments after the program name is straightforward with `env::args().skip(1)`. Iterate over them, open each file, and count it.

## Tasks

- Collect all paths from the command line arguments after the program name
- Open and process each file, using `?` to propagate errors
- Print the counts for each file on a separate line

## Testing

Also test for the error case: pass a path that does not exist and confirm the program exits with a non-zero status code. You can run your binary as a subprocess using `std::process::Command` and inspect `output.status.success()`. This is the start of integration testing, where you test the program as a whole rather than individual functions.
