# Counter Struct

As the number of values grows, passing them around individually gets clunky. A struct groups related data and gives it a name.

This step also switches from reading the entire file into a string to reading it line by line through a `BufReader`. For large files this matters because it keeps memory use constant regardless of file size.

Implementing `std::fmt::Display` on the struct keeps the formatting logic in one place. Once you have `Display`, you can use the struct directly in `println!` with `{}`.

The method signature `fn count<R: BufRead>(&mut self, reader: R)` uses a generic type parameter with a trait bound. This lets the method work with any reader, whether it is a file, a string in memory, or stdin.

## Tasks

- Define a `Counter` struct with `words`, `lines`, and `chars` fields
- Add a `count` method that accepts any type implementing `BufRead`
- Implement `std::fmt::Display` for `Counter`
- Open the file with `BufReader` instead of reading it all at once
