# Lines and Bytes

The real `wc` command prints three numbers: lines, words, and bytes. This step adds the other two.

Run `wc fixtures/test.txt` on your system and note the output format. Each number is right-aligned in a fixed-width column, followed by the filename. Rust's format strings support width and alignment with `{value:>width}`.

For byte count, `str::len` returns the number of bytes in a UTF-8 string. For ASCII files this equals the character count. For lines, `str::lines` gives an iterator over lines without their newline characters.

## Tasks

- Count bytes with `str::len`
- Count lines with `str::lines().count()`
- Format the output as `{lines} {words} {bytes} {path}` with right-aligned columns matching `wc`

## Testing

Verify each count independently using the corresponding flags: `wc -l` for lines, `wc -w` for words, `wc -c` for bytes. Also test that your output format matches `wc` on a known file, including the column alignment. A good way to spot alignment differences is to run both side by side on a file with small counts and one with large counts.
