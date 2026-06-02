//! End-to-end regression tests for the `thesauromatic` binary.
//!
//! These lock the exact bytes the CLI writes to stdout for a fixed set of
//! words, so that performance work cannot silently change observable output.
//! The expected outputs live in `tests/golden/`; regenerate them with
//! `just bless` if the word data is ever intentionally updated.

use std::process::Command;

/// Run the compiled binary with a single argument and return its stdout bytes.
fn run(word: &str) -> Vec<u8> {
    let out = Command::new(env!("CARGO_BIN_EXE_thesauromatic"))
        .arg(word)
        .output()
        .expect("failed to spawn thesauromatic");
    assert!(
        out.status.success(),
        "binary exited with failure for {:?}",
        word
    );
    out.stdout
}

/// Assert the CLI output for `word` matches the checked-in golden file.
fn assert_golden(word: &str, golden_filename: &str) {
    let expected = std::fs::read(format!(
        "{}/tests/golden/{}",
        env!("CARGO_MANIFEST_DIR"),
        golden_filename
    ))
    .expect("missing golden file");
    assert_eq!(
        run(word),
        expected,
        "output for {:?} diverged from tests/golden/{}",
        word,
        golden_filename
    );
}

#[test]
fn deluge() {
    assert_golden("deluge", "deluge.txt");
}

#[test]
fn happy() {
    assert_golden("happy", "happy.txt");
}

#[test]
fn construction() {
    assert_golden("construction", "construction.txt");
}

#[test]
fn stamp() {
    assert_golden("stamp", "stamp.txt");
}

#[test]
fn multiword_key() {
    assert_golden("a cappella", "a_cappella.txt");
}

/// An unknown word produces a single empty line (a lone `\n`). This is a
/// deliberate quirk of the original output path and must not change.
#[test]
fn missing_word() {
    assert_golden("blahblahnotaword", "blahblahnotaword.txt");
}

/// With no arguments the CLI prints usage and exits successfully.
#[test]
fn no_args_prints_usage() {
    let out = Command::new(env!("CARGO_BIN_EXE_thesauromatic"))
        .output()
        .expect("failed to spawn thesauromatic");
    assert!(out.status.success());
    let stdout = String::from_utf8(out.stdout).unwrap();
    assert!(
        stdout.contains("Usage: thesauromatic <word>"),
        "usage text missing, got: {:?}",
        stdout
    );
}
