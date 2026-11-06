//! End-to-end tests that run the `grid-stats` binary. Don't edit this file.

// unwrap()/expect() are fine in tests, including helper functions.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::process::{Command, Output};

const SMALL: &str = "tests/fixtures/small.csv";
const BAD: &str = "tests/fixtures/bad.csv";

fn run(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_grid-stats"))
        .args(args)
        .output()
        .expect("failed to start grid-stats")
}

fn stdout(out: &Output) -> String {
    String::from_utf8(out.stdout.clone()).unwrap()
}

fn stderr(out: &Output) -> String {
    String::from_utf8(out.stderr.clone()).unwrap()
}

/// (cell, count, mean, max) for every cell in the small fixture, row by row.
/// mean and max are rounded to 4 decimal places.
const EXPECTED: [(&str, u64, f64, f64); 8] = [
    ("r0c0", 4, 0.2875, 0.40),
    ("r0c1", 5, 0.4000, 0.85),
    ("r0c2", 3, 0.5167, 0.85),
    ("r1c0", 6, 0.2183, 0.40),
    ("r1c1", 8, 0.3450, 0.85),
    ("r2c0", 4, 0.1975, 0.33),
    ("r2c1", 5, 0.2780, 0.60),
    ("r2c2", 3, 0.3367, 0.60),
];

fn assert_close(actual: f64, expected: f64, what: &str) {
    assert!(
        (actual - expected).abs() < 1e-9,
        "{what}: expected {expected}, got {actual} (did you round to 4 decimal places?)"
    );
}

fn check_json_line(line: &str, (cell, count, mean, max): (&str, u64, f64, f64)) {
    let v: serde_json::Value =
        serde_json::from_str(line).unwrap_or_else(|e| panic!("not valid JSON: {line:?} ({e})"));
    assert_eq!(v["cell"], cell, "line: {line}");
    assert_eq!(v["count"].as_u64(), Some(count), "count in: {line}");
    assert_close(
        v["mean"].as_f64().unwrap(),
        mean,
        &format!("mean for {cell}"),
    );
    assert_close(v["max"].as_f64().unwrap(), max, &format!("max for {cell}"));
}

#[test]
fn json_lines_by_default() {
    let out = run(&[SMALL]);
    assert!(out.status.success(), "stderr: {}", stderr(&out));
    let text = stdout(&out);
    let lines: Vec<&str> = text.lines().collect();
    assert_eq!(
        lines.len(),
        EXPECTED.len(),
        "one line per cell with a value:\n{text}"
    );
    for (line, expected) in lines.iter().zip(EXPECTED) {
        check_json_line(line, expected);
    }
}

#[test]
fn json_when_asked_explicitly() {
    let out = run(&[SMALL, "--format", "json"]);
    assert!(out.status.success(), "stderr: {}", stderr(&out));
    assert_eq!(stdout(&out).lines().count(), EXPECTED.len());
}

#[test]
fn csv_format() {
    let out = run(&[SMALL, "--format", "csv"]);
    assert!(out.status.success(), "stderr: {}", stderr(&out));
    let text = stdout(&out);
    let mut lines = text.lines();
    assert_eq!(lines.next(), Some("cell,count,mean,max"));
    let rows: Vec<&str> = lines.collect();
    assert_eq!(rows.len(), EXPECTED.len(), "{text}");
    for (row, (cell, count, mean, max)) in rows.iter().zip(EXPECTED) {
        let fields: Vec<&str> = row.split(',').collect();
        assert_eq!(fields.len(), 4, "row: {row}");
        assert_eq!(fields[0], cell);
        assert_eq!(fields[1].parse::<u64>().unwrap(), count, "row: {row}");
        assert_close(
            fields[2].parse().unwrap(),
            mean,
            &format!("mean for {cell}"),
        );
        assert_close(fields[3].parse().unwrap(), max, &format!("max for {cell}"));
    }
}

#[test]
fn only_one_cell() {
    let out = run(&[SMALL, "--only", "r1c1"]);
    assert!(out.status.success(), "stderr: {}", stderr(&out));
    let text = stdout(&out);
    let lines: Vec<&str> = text.lines().collect();
    assert_eq!(lines.len(), 1, "{text}");
    check_json_line(lines[0], EXPECTED[4]);
}

#[test]
fn only_a_missing_cell_fails_and_names_it() {
    let out = run(&[SMALL, "--only", "r1c2"]);
    assert!(!out.status.success());
    assert!(stdout(&out).is_empty(), "nothing on stdout when failing");
    assert!(stderr(&out).contains("r1c2"), "stderr: {}", stderr(&out));
}

#[test]
fn only_with_a_malformed_cell_id_fails() {
    let out = run(&[SMALL, "--only", "nope"]);
    assert!(!out.status.success());
    assert!(stdout(&out).is_empty());
    let err = stderr(&out);
    assert!(!err.contains("panicked"), "should not panic: {err}");
}

#[test]
fn bad_input_fails_with_the_line_number() {
    let out = run(&[BAD]);
    assert!(!out.status.success());
    assert!(stdout(&out).is_empty(), "nothing on stdout when failing");
    let err = stderr(&out);
    assert!(
        err.starts_with("error:"),
        "stderr should start with 'error:': {err}"
    );
    assert!(err.contains("line 4"), "stderr: {err}");
}

#[test]
fn missing_file_fails_cleanly() {
    let out = run(&["tests/fixtures/does-not-exist.csv"]);
    assert!(!out.status.success());
    let err = stderr(&out);
    assert!(err.starts_with("error:"), "stderr: {err}");
    assert!(!err.contains("panicked"), "should not panic: {err}");
}
