# Fixtures

Small files for testing and exploring edge cases. Each one is designed to produce
surprising or instructive output from wc.

Run any of them with:

    wc fixtures/<name>.txt

## test.txt

The main general-purpose fixture. Nine lines, 128 words, 694 characters (not counting
newlines), 702 bytes total.

## alice.txt

A longer text for performance comparisons and multi-file tests.

---

## Edge case fixtures

### empty.txt

A completely empty file, zero bytes.

    wc: 0 0 0

All counts are zero. Good baseline to verify your program does not crash or
produce garbage output on empty input.

### no-trailing-newline.txt

Contains `hello world` with no newline at the end.

    wc: 0 2 11

The line count is zero. `wc` counts newline characters, not lines of content,
so a file that ends without `\n` has one fewer line than you might expect.
Two words, eleven bytes.

Our implementation using `BufRead::lines()` reports 1 line here, because `lines()`
yields the final fragment even without a terminating newline. This is a real
difference between "count newlines" and "count lines".

### only-whitespace.txt

Spaces, a tab, and a newline. No visible content.

    wc: 1 0 8

One line because the file ends with `\n`. Zero words because `split_whitespace`
requires at least one non-whitespace character to produce a token. Eight bytes
for the three spaces, one tab, three more spaces, and the newline.

### only-newlines.txt

Three newline characters, nothing else.

    wc: 3 0 3

Three lines, zero words, three bytes. A file can have many lines and no words.

### unicode.txt

Contains `café naïve résumé` followed by a newline. Three words, all with
accented characters encoded as multi-byte UTF-8 sequences.

    wc:   1   3  22    (bytes)
    wc -m: 18          (Unicode characters)

The byte count (22) is higher than the character count (18) because each accented
letter such as `é` or `ï` takes two bytes in UTF-8.

Our implementation counts Unicode codepoints via `.chars().count()` and reports
18, which matches `wc -m` but not the default `wc` output. The system `wc`
counts bytes by default, not characters.

### punctuation.txt

Contains `Hello, world! (100 items) foo@bar.com` followed by a newline.

    wc: 1 5 38

Five words: `Hello,`, `world!`, `(100`, `items)`, `foo@bar.com`. Punctuation
attached to a word is part of that word. Commas, exclamation marks, and `@` do
not act as word separators. Only whitespace splits words.

### windows-crlf.txt

Two lines with Windows-style `\r\n` line endings.

    wc: 2 4 22

Four words, twenty-two bytes. The `\r` byte is counted in the byte total but
`BufRead::lines()` strips it, so our implementation counts the characters
without `\r`. This means our character count (20) differs from the raw byte
count (22) for CRLF files.

### repeated-whitespace.txt

Contains `one   two		three` with multiple spaces between the first pair and
two tabs between the second pair, followed by a newline.

    wc: 1 3 17

Three words despite the extra whitespace. `split_whitespace` collapses any
run of whitespace characters, regardless of type or length, into a single
word boundary.
