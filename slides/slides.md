---
theme: default
title: Learning Rust by Counting Words
author: Matthias Endler and Hanno Braun
info: Six-hour Rust workshop with ten exercises, progressive checks, hints, and solutions.
class: cover
layout: default
colorSchema: light
aspectRatio: 16/9
canvasWidth: 980
fonts:
  sans: Inter
  provider: none
drawings:
  persist: false
transition: none
mdc: false
exportFilename: learning-rust-by-counting-words
---

# Learning Rust by Counting Words

Matthias Endler and Hanno Braun

<svg class="cover-dots" viewBox="0 0 140 140" aria-hidden="true">
  <g v-for="row in 7" :key="row">
    <circle v-for="column in 7" :key="column" :cx="column * 20 - 10" :cy="row * 20 - 10" r="2.5" />
  </g>
</svg>

<!--
Welcome. We will build a small command-line program using only Rust's standard library. You need programming experience, not Rust experience. The styling follows the existing workshop banner; the bottom-right logo is its actual Corrode wordmark. Spend about five minutes on introductions and ask which languages people already use.
-->

---

# One small tool, ten exercises

- Read a file and count words.
- Separate counting from input and output.
- Test edge cases before optimizing.
- Explain what your counts actually mean.

<!--
Budget six hours including two fifteen-minute breaks. Opening and setup: 20 minutes. Exercises 1–3: 65 minutes. First break: 15 minutes. Exercises 4–6: 95 minutes. Second break: 15 minutes. Exercises 7–10: 115 minutes. Review, discussion, and show-and-tell: 35 minutes. Exercise times include explanation, coding, and solution discussion. Pairing is welcome. Hints are optional; let people attempt the task before revealing them. Ask for questions throughout rather than saving them for the end.
-->

---

# Get a program running

```sh
rustc --version
cargo --version
cargo run -p wc
```

Work in `wc/src/main.rs`, from the repository root.

<!--
Use Rust 1.85 or newer: the numbered examples use edition 2024. Rustup's current stable toolchain is suitable. The starter main is intentionally empty, so a successful run prints nothing. Explain that Cargo builds and runs the selected package. Arguments after -- go to the program. All learner commands in the deck run from the original repository root. Complete solutions are in examples/ at the repository root; keep working in the learner file rather than editing those snapshots. Do not run the entire starter test suite yet: many tests describe features we have not implemented.
-->

---

# How to work through each exercise

1. Read the task and its expected results.
2. Edit `wc/src/main.rs`, then run your program.
3. Try the suggested inputs and tests. Read any failures.
4. Edit and test again until the exercise checks pass.

Test inputs are in `fixtures/` at the repository root.

Stuck? Use the hints. Compare solutions after your own attempt.

<!--
Keep building the same learner program as features accumulate. Start with a small change, run it, and compare the output with the task's expected results. A compiler error or failed test tells you what to investigate next. Fix one problem, rerun the check, and repeat. Once the current checks pass, rerun earlier checks before moving on. Early exercises use manual input/output checks; add automated tests as we introduce them. For example, cargo test -p wc word_count_matches_fixture checks the first exercise. Do not run every starter test immediately: some describe later features. Those integration tests are coarse feature checks, so also inspect the actual counts. The numbered solutions are focused snapshots, not a feature-complete chain; later examples sometimes omit stdin and totals. Compare their approach without replacing your accumulated work.
-->

---

# 1. Count words in a file

Read the first argument. Print its word count.

<v-clicks>

- First: `test.txt` contains 128 words.
- Then: `empty.txt` contains zero words.
- Finally: missing paths must fail visibly.

</v-clicks>

<!--
Allow 20 minutes for exercise 1, including hints and solution. Run cargo run -p wc -- fixtures/test.txt, then fixtures/empty.txt. Try fixtures/missing.txt and no arguments. Introduce let, function calls, use statements, and println! only as needed. Accept expect for now; making failure visible is enough at this stage. Run cargo test -p wc word_count_matches_fixture and cargo test -p wc missing_file_exits_with_error once the manual checks work. Optional early bonus: make the usage message helpful. Do not introduce an argument-parsing crate.
-->

---

# 1. Hints: follow the data

<v-clicks>

- `env::args().nth(1)` selects the first argument.
- `fs::read_to_string` returns owned text or an error.
- `split_whitespace().count()` counts nonempty tokens.

</v-clicks>

[Arguments](https://doc.rust-lang.org/std/env/fn.args.html) · [Read a file](https://doc.rust-lang.org/std/fs/fn.read_to_string.html)

<!--
Reveal one hint at a time. Argument zero is the executable name. nth returns Option because the argument might not exist. Reading returns Result because the file may be missing, inaccessible, or invalid UTF-8. Explain expect as an explicit prototype shortcut rather than production error handling. Whitespace splitting handles repeated spaces and tabs, unlike splitting on a single literal space. Test fixtures/repeated-whitespace.txt: three words.
-->

---

# 1. Solution: read, split, count

Excerpt:

```rust
let path = env::args().nth(1).expect("Usage: count-words <path>");
let contents = fs::read_to_string(path).expect("Could not read file");
let words = contents.split_whitespace().count();
```

[Complete code](https://github.com/corrode/learning-rust-by-counting-words/blob/main/examples/01-count-words/src/main.rs)

<!--
The full program prints Word count: followed by the count. Walk through the types: Option<String>, Result<String, io::Error>, then usize. Methods can return iterators rather than materializing another collection. count consumes the iterator, not the original text. This is an excerpt, not a standalone program. Use the complete code locally. The same rule applies to all Rust excerpts in this deck.
-->

---

# 2. Extract the counting function

Move word counting into `count(&str) -> usize`.

<v-clicks>

- Keep the 128-word fixture check passing.
- Check empty text returns zero.
- Call `count` twice with the same text.

</v-clicks>

<!--
Allow 20 minutes. Keep file access and printing in main. Use cargo test -p wc word_count_matches_fixture after the refactor. Temporarily call count twice to demonstrate that borrowing does not consume contents. For empty text, either call the function directly while exploring or use fixtures/empty.txt. Optional early bonus: predict what changes if count takes String instead. We will add formal unit tests in exercise 7; assertions in an exploratory main are fine for now.
-->

---

# 2. Hints: borrow the text

<v-clicks>

- `String` owns its allocation.
- `&str` borrows a view of text.
- The final expression becomes the return value.

</v-clicks>

[References and borrowing](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html) · [String slices](https://doc.rust-lang.org/book/ch04-03-slices.html)

<!--
Draw attention to count(&contents). The caller keeps ownership of its String, while the function only needs read access. The borrow cannot outlive its source. No explicit lifetime annotation is needed here. Contrast an expression without a semicolon with a statement ending in a semicolon. If someone gets an expected usize, found () error, use it as a chance to read the compiler diagnostic together.
-->

---

# 2. Solution: a small borrowed interface

Excerpt:

```rust
fn count(contents: &str) -> usize {
    contents.split_whitespace().count()
}
```

[Complete code](https://github.com/corrode/learning-rust-by-counting-words/blob/main/examples/02-extract-function/src/main.rs)

<!--
The function works with a borrowed String or a string literal. It performs no file access, so it will be easy to test. Ask why returning a count is simpler than printing inside the function. Preserve the passing integration test. This is enough abstraction for now; there is no need to create a trait for a three-line function.
-->

---

# 3. Count lines and bytes

Print lines, words, bytes, and the filename.

<v-clicks>

- `test.txt`: 9 lines, 128 words, 702 bytes.
- `unicode.txt`: 1 line, 3 words, 22 bytes.
- `no-trailing-newline.txt`: 1 line, 2 words, 11 bytes.

</v-clicks>

<!--
Allow 25 minutes. Run each named fixture with cargo run -p wc -- fixtures/NAME.txt. These expectations use str::lines, not newline counting. Add cargo test -p wc line_count_matches_fixture and cargo test -p wc output_contains_filename. The current example computes all three counts in main; learners may keep their extracted function and extend its return value. Do not force an architectural rewrite. The Unicode fixture has 18 Unicode scalar values including its newline, but 22 UTF-8 bytes.
-->

---

# 3. Hints: choose the unit

<v-clicks>

- `str::len()` counts UTF-8 bytes.
- `chars().count()` counts Unicode scalar values.
- `lines()` includes a final unterminated fragment.

</v-clicks>

[String methods](https://doc.rust-lang.org/std/primitive.str.html)

<!--
Use café as the concrete example: four Unicode scalar values, five UTF-8 bytes. Scalar values are not necessarily visible characters; combining marks and emoji sequences make that distinction important. We are counting bytes in this exercise. Explain that lines strips LF or CRLF terminators from yielded strings and that a final newline does not create an extra empty line. Empty input produces zero lines.
-->

---

# 3. Solution: three different questions

Excerpt:

```rust
let bytes = contents.len();
let words = contents.split_whitespace().count();
let lines = contents.lines().count();
```

[Complete code](https://github.com/corrode/learning-rust-by-counting-words/blob/main/examples/03-lines-and-bytes/src/main.rs)

<!--
The full example formats aligned columns and includes the path. Formatting is less important than knowing what each column measures. Run fixtures/no-trailing-newline.txt through both our program and system wc: our line count is one, while wc counts zero LF bytes. That is a semantic difference, not an arithmetic error. Do not assert equivalence with system wc without specifying the contract.
-->

---

# What does “count” mean?

| Input | Text lines | LF bytes |
| --- | ---: | ---: |
| Empty | 0 | 0 |
| `hello` | 1 | 0 |
| `hello\n` | 1 | 1 |

<!--
Take the first fifteen-minute break after this checkpoint. Before the break, ask learners to explain the middle row. The next examples explore line-based I/O and count Unicode scalar values rather than raw bytes. That is an intentional change of unit, not a transparent optimization. The presenter notes describe the snapshots' counting rules and limitations. Bring people back by asking them to predict the Unicode fixture's result before running it.
-->

---

# 4. Give the counts a type

Create `Counter`, then count through a buffered reader.

<v-clicks>

- `test.txt`: 9 lines, 128 words, 694 scalars.
- `unicode.txt`: 1 line, 3 words, 17 scalars.
- `only-newlines.txt`: 3 lines, zero words or scalars.

</v-clicks>

<!--
Allow 30 minutes. Here scalars means Unicode scalar values after stripping line endings. Introduce struct fields, impl, new, and &mut self. The sample calls its field chars; keep that identifier when navigating code. A BufReader avoids a separate operating-system read for every small request, though lines still allocates a String per line and a single enormous line can require substantial memory. Run all three fixtures manually. Keep earlier word and filename checks. Do not call this a byte counter.
-->

---

# 4. Hints: separate storage and behavior

<v-clicks>

- Store `words`, `lines`, and `chars` together.
- `&mut self` permits updating the counter.
- `R: BufRead` accepts different buffered readers.
- `Display` controls the printed representation.

</v-clicks>

[Structs](https://doc.rust-lang.org/book/ch05-01-defining-structs.html) · [BufRead](https://doc.rust-lang.org/std/io/trait.BufRead.html)

<!--
Start with the struct and its loop; add Display only once counting works. Explain a trait bound as the operations this function needs from its argument. File input will use BufReader<File>; later tests can use in-memory bytes. The example returns &mut Counter to permit chaining, but returning () would also work for this task. The key lesson is a useful type, not reproducing every stylistic choice.
-->

---

# 4. Solution: update named fields

Excerpt from `Counter::count`:

```rust
let line = line.expect("Could not read line");
self.lines += 1;
self.words += line.split_whitespace().count();
self.chars += line.chars().count();
```

[Complete code](https://github.com/corrode/learning-rust-by-counting-words/blob/main/examples/04-counter-struct/src/main.rs)

<!--
reader.lines() yields Results, not bare strings. We still use expect here; exercise 5 removes it from this path. The yielded line contains no LF or CRLF terminator, so neither contributes to chars. Compare unicode.txt with exercise 3: 17 content scalars versus 22 raw bytes. Count test.txt to confirm 694 content scalars. These are different measurements of the same file.
-->

---

# 5. Accept multiple files

Print one result per path. Propagate I/O errors.

<v-clicks>

- One file still produces one row.
- Two files produce two labeled rows.
- A missing file returns a failure status.

</v-clicks>

<!--
Allow 30 minutes. Run cargo run -p wc -- fixtures/test.txt fixtures/alice.txt. Then put a missing path second and observe the first row followed by an error. This exercise uses fail-fast behavior; continuing after an error is a separate design decision. Run cargo test -p wc multiple_files_each_filename_appears and cargo test -p wc missing_file_exits_with_error. Do not enable the multiple_files_each_on_its_own_line test yet: despite its name, it expects a total row too. The reference also adds one scalar per yielded line; discuss that approximation explicitly on the solution slide.
-->

---

# 5. Hints: let errors travel upward

<v-clicks>

- `env::args().skip(1)` visits the supplied paths.
- `Result` represents success or failure.
- `?` returns early when an operation fails.
- Construct a fresh counter for each file.

</v-clicks>

[Recoverable errors](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html)

<!--
Show Result<T, E> and explain that ? extracts T on success or returns an error from the surrounding function. Start with an explicit match if that is easier to follow. main returns Result<(), Box<dyn Error>> in the example, allowing different error types behind one interface. Keep discussion of trait objects brief until exercise 6. Deriving Default replaces the all-zero constructor. No arguments currently produces no output and success in this snapshot; preserve a usage error in the learner implementation if already added.
-->

---

# 5. Solution: propagate a failed read

Excerpt from `Counter::count`:

```rust
let line = line?;
self.lines += 1;
self.words += line.split_whitespace().count();
self.chars += line.chars().count();
```

[Complete code](https://github.com/corrode/learning-rust-by-counting-words/blob/main/examples/05-multiple-files/src/main.rs)

<!--
The full example additionally uses self.chars += self.lines after the loop. This approximates restoring one newline per logical line: it invents a newline for unterminated input and restores only one scalar for CRLF. It also overcounts if the same Counter is reused across count calls, because self.lines is cumulative. A fresh counter per file avoids that last issue, not the terminator issues. Do not silently copy the approximation into a precise character counter. For this snapshot, no-trailing-newline.txt reports 12 chars, not the actual 11. Keep that counterexample for our testing discussion.
-->

---

# 6. Read stdin and print totals

Treat `-` as stdin. Sum multiple inputs.

<v-clicks>

- Piped `one two three` contains three words.
- Two copies of `test.txt` total 256 words.
- A single input has no total row.

</v-clicks>

<!--
Allow 35 minutes. Run printf 'one two three\n' | cargo run -p wc -- -. Run cargo run -p wc -- fixtures/test.txt fixtures/test.txt. Add cargo test -p wc stdin, cargo test -p wc total, and cargo test -p wc multiple_files. The snapshot requires an explicit dash; no arguments does not mean stdin. Reading the same stdin twice consumes the same stream, not two independent copies. Learners can retain their generic counting function and pass readers directly instead of adopting the full reference design.
-->

---

# 6. Hints: count first, combine second

<v-clicks>

- Keep counts separate from the reader.
- Use `AddAssign` for `total += counts`.
- `Box<dyn BufRead>` stores different reader types.
- Print totals only when input count exceeds one.

</v-clicks>

[Trait objects](https://doc.rust-lang.org/book/ch18-02-trait-objects.html) · [AddAssign](https://doc.rust-lang.org/std/ops/trait.AddAssign.html)

<!--
Explain why the two branches naturally produce different concrete types: BufReader<Stdin> and BufReader<File>. A boxed trait object is one way to put them behind the same Counter type. It is not mandatory for a word counter. The example's 'static bound means the stored reader cannot contain short-lived borrowed references; it does not mean the reader lives forever. Counts derives Copy and Clone, so adding a small value does not prevent printing it afterward.
-->

---

# 6. Solution: select a reader

Excerpt:

```rust
let counter = if path == STDIN {
    Counter::new(BufReader::new(io::stdin()))
} else {
    Counter::new(BufReader::new(File::open(path)?))
```

[Complete code](https://github.com/corrode/learning-rust-by-counting-words/blob/main/examples/06-stdin-and-totals/src/main.rs)

<!--
This excerpt omits the closing brace and subsequent statements; open the full source to run it. Each branch returns Counter, whose reader field is Box<dyn BufRead>. count consumes the Counter and returns a separate Counts value. Then total += counts and println print the result. This snapshot retains the line-ending approximation from exercise 5. Demonstrate stdin with an empty stream: printf '' | cargo run -p wc -- - should produce zero counts.
-->

---

# Break: fifteen minutes

When we return: can a passing test be wrong?

<!--
Take the second fifteen-minute break. Invite people to leave one confusing compiler message or counting question for the next segment. After the break, use Hello, world! followed by a newline to expose a mistaken expectation: thirteen content scalars, fourteen including the newline. Do not spend the break introducing additional tasks.
-->

---

# 7. Test the counting rules

Add unit tests without opening a file.

<v-clicks>

- Empty input produces zero counts.
- `Hello, world!\n` has 13 content scalars.
- LF and CRLF disappear from yielded lines.

</v-clicks>

<!--
Allow 30 minutes. Add #[cfg(test)] mod tests and #[test] functions. Use in-memory byte slices as readers. Assert all fields, not just that the printed string contains a digit. The exercise 7 reference returns to exercise 4's content-scalar rule, without adding presumed newlines. Its small-input assertion is 13: six scalars in Hello,, one space, and six in world!. The newline is excluded. If learners retained another counting contract, their expected values should match that explicit contract instead.
-->

---

# 7. Hints: make failures specific

<v-clicks>

- Build a reader from `input.as_bytes()`.
- Assert each count separately.
- Test empty, Unicode, and unterminated input.
- Run focused tests before the whole suite.

</v-clicks>

[Writing tests](https://doc.rust-lang.org/book/ch11-01-writing-tests.html)

<!--
Run cargo test -p wc with a chosen unit-test name while iterating, then all learner tests once the features are present. Existing integration checks can still fail if a feature or usage error is missing. cargo clippy -p wc --all-targets offers additional feedback but does not establish correctness. Ask what an injected read error should do, and add a test for it. Explain why unwrap inside a test can be acceptable: failure is the expected test harness signal, not a user-facing error policy.
-->

---

# 7. Solution: assert the chosen units

Excerpt from the reference test:

```rust
assert_eq!(counter.lines, 1);
assert_eq!(counter.words, 2);
assert_eq!(counter.chars, 13);
```

[Complete code and tests](https://github.com/corrode/learning-rust-by-counting-words/blob/main/examples/07-unit-tests/src/main.rs)

<!--
The input is Hello, world! followed by LF: six scalars in Hello,, one space, and six in world!, totaling thirteen content scalars. Including the newline would give fourteen. Ask learners to predict the result before running the test. This is why expected values deserve the same scrutiny as implementation code. The byte-chunk snapshot measures a different unit: its multiline input contains 24 raw bytes. Both assertions are correct for their respective inputs and contracts.
-->

---

# 8. Try an iterator-based version

Replace the counting loop with `try_fold`.

<v-clicks>

- Keep every counting expectation unchanged.
- Empty input returns the initial counts.
- A read error must still reach the caller.

</v-clicks>

<!--
Allow 25 minutes. This is a comparison of styles, not a claim that the shorter version is better or faster. Run the same tests before and after the change. The snapshot returns a tuple in the order lines, words, chars. If learners have a Counts struct, they can fold over that instead. Keep I/O errors visible. Add a regression test with an error-producing reader as well as successful inputs.
-->

---

# 8. Hints: carry one accumulator

<v-clicks>

- Start from `(0, 0, 0)`.
- The closure receives counts and a line result.
- Use `?` inside the closure.
- Return `Ok(counts)` for the next iteration.

</v-clicks>

[Iterator::try_fold](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.try_fold)

<!--
Explain closures before showing the reference. The accumulator is passed by value into each closure call; mut allows local updates. try_fold stops on the first error. Ordinary fold would need another way to handle Result. The tuple is compact but .0, .1, and .2 are less descriptive than named fields. Ask learners which version they would prefer to maintain, and accept either answer with a reason.
-->

---

# 8. Solution: stop at the first error

Excerpt from `count`:

```rust
reader.lines().try_fold((0, 0, 0), |mut counts, line| {
    let line = line?;
    counts.0 += 1;
    counts.1 += line.split_whitespace().count();
```

[Complete code](https://github.com/corrode/learning-rust-by-counting-words/blob/main/examples/08-try-fold/src/main.rs)

<!--
The complete closure also adds line.chars().count() to counts.2 and returns Ok(counts). Those lines are omitted only to keep the excerpt readable; they are not optional behavior. Compare the Result return types with the loop version. Keep the content-scalar convention from exercise 7. Do not claim this refactor improves performance without measurement.
-->

---

# 9. Measure before changing the algorithm

Time a release build on the same input repeatedly.

<v-clicks>

- Confirm counts before comparing timings.
- Run `alice.txt` once to warm up.
- Record five runs, input size, and build mode.

</v-clicks>

<!--
Allow 25 minutes. Use cargo run --release -p wc -- fixtures/alice.txt for the learner version. Add Instant around the work, not around compilation. The reference starts before argument handling and includes file opening, reading, counting, and printing in its interval. Small inputs may round down to 0.000s; that is a resolution issue, not proof of free work. For a bounded 24-MB input, run from the repository root: python3 -c 'from pathlib import Path; Path("fixtures/large.txt").write_bytes(b"one two\nthree four five\n" * 1000000)'. Keep generated data out of version control and remove it when finished if desired.
-->

---

# 9. Hints: define the measurement

<v-clicks>

- `Instant` measures elapsed time.
- Release and debug builds answer different questions.
- File caching changes repeated-run timings.
- Keep output and counting rules comparable.

</v-clicks>

[Instant](https://doc.rust-lang.org/std/time/struct.Instant.html)

<!--
Do not present made-up benchmark numbers. Record results from the participants' actual machines. Compare repeated measurements and discuss noise. The sample prints its elapsed time to stdout; that changes the CLI output and can break single-row integration tests. For the cumulative learner tool, prefer eprintln! for timing so normal stdout remains machine-readable. This is a deliberate improvement over the illustrative snapshot, not a reason to weaken the existing output tests. Criterion and flamegraphs can be later explorations, not dependencies for this workshop.
-->

---

# 9. Solution: bracket the work

Excerpt, with intervening work omitted:

```rust
let start = std::time::Instant::now();
// ... read, count, and print ...
let elapsed = start.elapsed();
```

[Complete code](https://github.com/corrode/learning-rust-by-counting-words/blob/main/examples/09-timing/src/main.rs)

<!--
The elapsed value is a Duration. The sample formats seconds and milliseconds. Its interval excludes compilation but includes output inside main; process startup before main is also excluded. Have pairs compare what they timed before comparing the numbers. This example uses the loop-based content-scalar counter, not the try_fold version, so change one thing at a time in the learner code if making a direct comparison.
-->

---

# 10. Count byte chunks

Keep word-boundary state across reads.

<v-clicks>

- `Hello, world!\n`: 1 newline, 2 words, 14 bytes.
- Unterminated `hello world`: 0 newlines, 2 words, 11 bytes.
- A word split across buffers still counts once.

</v-clicks>

<!--
Allow 35 minutes. Change the contract explicitly: count LF bytes and raw bytes, not text lines and content scalars. Match the reference's four separators: space, tab, CR, LF. Read only buf[..n]; a successful read can return less than the buffer length. Test with a custom reader returning two bytes at a time, not merely BufReader::with_capacity, since large reads can bypass BufReader's internal buffer. Add a regression test for this short-read case. Preserve stdin and totals in the learner tool if desired; the snapshot is a narrower performance example.
-->

---

# 10. Hints: remember one bit

<v-clicks>

- Keep `in_word` outside the read loop.
- Count a word when entering non-whitespace.
- Count only the bytes actually read.
- Test Unicode whitespace before claiming `wc` compatibility.

</v-clicks>

[Read::read](https://doc.rust-lang.org/std/io/trait.Read.html#tymethod.read)

<!--
The reference recognizes only four whitespace bytes. It omits ASCII vertical tab and form feed, and it does not recognize Unicode whitespace such as a nonbreaking space. System wc behavior also depends on implementation and locale. This scanner is therefore not a fully compatible wc replacement. Raw-byte reading can handle invalid UTF-8 that read_to_string and lines reject. The full snapshot also spawns one thread per file and unwraps worker errors; discuss these as limitations, not a production recommendation. No unbounded stream demo with /dev/zero during this exercise.
-->

---

# 10. Solution: count a transition

Excerpt from the non-whitespace branch:

```rust
if !in_word {
    self.words += 1;
    in_word = true;
}
```

[Complete code and tests](https://github.com/corrode/learning-rust-by-counting-words/blob/main/examples/10-byte-chunks/src/main.rs)

<!--
Whitespace clears in_word; LF additionally increments lines. Keeping the flag outside the outer loop joins a word split across two reads. The complete reference uses a one-MiB stack buffer per worker and one worker per file, so many paths can consume substantial resources. To isolate the scanner's performance, compare single-file runs first. Keep assertions for empty input, short reads, LF, CRLF, and invalid UTF-8. Unicode whitespace intentionally exposes a difference from split_whitespace. Faster code that changes the answer is not a drop-in optimization.
-->

---

# What does your program promise?

- Which units does each column count?
- What happens when a read fails?
- Which tests would catch a broken refactor?
- What did your timing actually measure?

<!--
Use about fifteen minutes for pairs to review their code and explain these answers to each other. Ask each pair to show one test that failed usefully. Do not require a polished program. A clear contract and a small set of reliable tests are a better outcome than copying every feature of the reference. The presenter notes record the snapshots' differences so learners can keep exploring without treating each example as a perfect implementation.
-->

---

# Show one thing you learned

A useful test, a compiler message, or a design you changed.

[The Rust Book](https://doc.rust-lang.org/book/) · [Standard library](https://doc.rust-lang.org/std/) · [Corrode](https://corrode.dev/)

<!--
Use the remaining twenty minutes for short demonstrations and questions. Invite two-minute contributions rather than full code tours. For a next step, choose one concrete improvement: precise counting semantics, a helpful usage error, or bounded concurrency. Thank everyone. Do not turn the closing into a list of ten new obligations; the useful follow-up is whichever question their own program exposed.
-->
