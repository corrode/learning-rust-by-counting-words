//! Integration tests for the wc binary.
//!
//! Each test spawns the compiled binary as a subprocess and inspects its
//! stdout, stderr, and exit code. The tests compile even with an empty main,
//! but they will fail until you implement the features they describe.
//!
//! Run with: cargo test

use std::io::Write;
use std::process::{Command, Output, Stdio};

fn run(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_wc"))
        .args(args)
        .output()
        .expect("failed to execute wc")
}

fn run_with_stdin(input: &str) -> Output {
    let mut child = Command::new(env!("CARGO_BIN_EXE_wc"))
        .arg("-")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("failed to spawn wc");
    child
        .stdin
        .take()
        .unwrap()
        .write_all(input.as_bytes())
        .unwrap();
    child.wait_with_output().expect("failed to wait on wc")
}

// --- argument handling ---

#[test]
fn no_arguments_exits_with_error() {
    assert!(!run(&[]).status.success());
}

#[test]
fn missing_file_exits_with_error() {
    assert!(!run(&["no_such_file.txt"]).status.success());
}

// --- counts ---

#[test]
fn word_count_matches_fixture() {
    let out = run(&["../fixtures/test.txt"]);
    assert!(out.status.success());
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("128"), "expected 128 words, got: {stdout}");
}

#[test]
fn line_count_matches_fixture() {
    let out = run(&["../fixtures/test.txt"]);
    assert!(out.status.success());
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("9"), "expected 9 lines, got: {stdout}");
}

// --- output format ---

#[test]
fn output_contains_filename() {
    let out = run(&["../fixtures/test.txt"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("test.txt"),
        "output should include the filename, got: {stdout}"
    );
}

#[test]
fn single_file_output_is_one_line() {
    let out = run(&["../fixtures/test.txt"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert_eq!(
        stdout.lines().count(),
        1,
        "single file should produce exactly one output line, got: {stdout}"
    );
}

// --- multiple files ---

#[test]
fn multiple_files_each_filename_appears() {
    let out = run(&["../fixtures/test.txt", "../fixtures/alice.txt"]);
    assert!(out.status.success());
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("test.txt"), "got: {stdout}");
    assert!(stdout.contains("alice.txt"), "got: {stdout}");
}

#[test]
fn multiple_files_each_on_its_own_line() {
    let out = run(&["../fixtures/test.txt", "../fixtures/alice.txt"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    // two files plus a total line = at least 3 lines of output
    assert!(
        stdout.lines().count() >= 3,
        "expected at least 3 output lines, got: {stdout}"
    );
}

#[test]
fn multiple_files_shows_total_line() {
    let out = run(&["../fixtures/test.txt", "../fixtures/test.txt"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("total"),
        "expected a total line when given multiple files, got: {stdout}"
    );
}

#[test]
fn single_file_no_total_line() {
    let out = run(&["../fixtures/test.txt"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        !stdout.contains("total"),
        "should not show a total line for a single file, got: {stdout}"
    );
}

#[test]
fn total_word_count_is_sum_of_individual_files() {
    // Running test.txt twice: 128 + 128 = 256 words total.
    let out = run(&["../fixtures/test.txt", "../fixtures/test.txt"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("256"),
        "expected total of 256 words (128 * 2), got: {stdout}"
    );
}

// --- stdin ---

#[test]
fn dash_reads_from_stdin() {
    let out = run_with_stdin("one two three\n");
    assert!(out.status.success());
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("3"),
        "expected 3 words from stdin, got: {stdout}"
    );
}

#[test]
fn empty_stdin_produces_zero_counts() {
    let out = run_with_stdin("");
    assert!(out.status.success());
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("0"),
        "expected zero counts for empty input, got: {stdout}"
    );
}

#[test]
fn stdin_line_count() {
    let out = run_with_stdin("line one\nline two\nline three\n");
    assert!(out.status.success());
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("3"),
        "expected 3 lines from stdin, got: {stdout}"
    );
}
