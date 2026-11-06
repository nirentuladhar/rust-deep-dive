//! Tests for loading a grid from CSV, and for `GridError`. Don't edit this file.

// unwrap()/expect() are fine in tests, including helper functions.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use grid_stats::{CellId, Grid, GridError, MAX_DIM};
use std::error::Error;

fn load(csv: &str) -> Result<Grid, GridError> {
    Grid::from_reader(csv.as_bytes())
}

fn c(row: u32, col: u32) -> CellId {
    CellId::new(row, col)
}

// ---------- Successful loads ----------

#[test]
fn loads_the_small_fixture() {
    let grid = Grid::from_path("tests/fixtures/small.csv").unwrap();
    assert_eq!(grid.width(), 3);
    assert_eq!(grid.height(), 3);
    assert_eq!(grid.get(c(0, 0)), Some(0.12));
    assert_eq!(grid.get(c(0, 2)), Some(0.85));
    assert_eq!(grid.get(c(2, 1)), Some(0.11));
    assert_eq!(grid.get(c(1, 2)), None, "r1c2 isn't in the file");
}

#[test]
fn size_comes_from_the_largest_row_and_col() {
    let grid = load("row,col,hazard\n4,1,0.5\n").unwrap();
    assert_eq!(grid.height(), 5);
    assert_eq!(grid.width(), 2);
    assert_eq!(grid.get(c(4, 1)), Some(0.5));
    assert_eq!(grid.get(c(0, 0)), None);
}

#[test]
fn cells_outside_the_grid_are_none() {
    let grid = load("row,col,hazard\n0,0,0.5\n").unwrap();
    assert_eq!(grid.get(c(0, 1)), None);
    assert_eq!(grid.get(c(1, 0)), None);
    assert_eq!(grid.get(c(u32::MAX, u32::MAX)), None);
}

#[test]
fn header_only_gives_an_empty_grid() {
    let grid = load("row,col,hazard\n").unwrap();
    assert_eq!(grid.width(), 0);
    assert_eq!(grid.height(), 0);
    assert_eq!(grid.cells().count(), 0);
}

#[test]
fn cells_are_listed_row_by_row() {
    // The fixture's lines are deliberately out of order.
    let grid = Grid::from_path("tests/fixtures/small.csv").unwrap();
    let cells: Vec<CellId> = grid.cells().collect();
    assert_eq!(
        cells,
        vec![
            c(0, 0),
            c(0, 1),
            c(0, 2),
            c(1, 0),
            c(1, 1),
            // r1c2 is missing
            c(2, 0),
            c(2, 1),
            c(2, 2),
        ]
    );
}

#[test]
fn trims_spaces_skips_blank_lines_and_handles_crlf() {
    let csv = " row,col,hazard \r\n0, 0 ,0.5\r\n\r\n   \n1,1, 0.25\r\n";
    let grid = load(csv).unwrap();
    assert_eq!(grid.get(c(0, 0)), Some(0.5));
    assert_eq!(grid.get(c(1, 1)), Some(0.25));
}

#[test]
fn accepts_the_largest_allowed_index() {
    let csv = format!("row,col,hazard\n{},0,0.5\n", MAX_DIM - 1);
    let grid = load(&csv).unwrap();
    assert_eq!(grid.height(), MAX_DIM);
}

// ---------- Header errors ----------

#[test]
fn empty_input_is_a_bad_header() {
    match load("") {
        Err(GridError::BadHeader { found }) => assert_eq!(found, ""),
        other => panic!("expected BadHeader, got {other:?}"),
    }
}

#[test]
fn wrong_header_is_reported() {
    for header in ["x,y,z", "row,col", "row,col,hazard,extra", "col,row,hazard"] {
        match load(&format!("{header}\n0,0,0.5\n")) {
            Err(GridError::BadHeader { found }) => assert_eq!(found, header),
            other => panic!("{header:?}: expected BadHeader, got {other:?}"),
        }
    }
}

// ---------- Line errors ----------

fn expect_bad_line(csv: &str, expected_line: usize) {
    match load(csv) {
        Err(GridError::BadLine { line, reason }) => {
            assert_eq!(line, expected_line, "wrong line for {csv:?}");
            assert!(!reason.is_empty(), "reason should explain the problem");
        }
        other => panic!("{csv:?}: expected BadLine, got {other:?}"),
    }
}

#[test]
fn wrong_number_of_fields() {
    expect_bad_line("row,col,hazard\n0,0\n", 2);
    expect_bad_line("row,col,hazard\n0,0,0.5,9\n", 2);
}

#[test]
fn fields_that_are_not_numbers() {
    expect_bad_line("row,col,hazard\nx,0,0.5\n", 2);
    expect_bad_line("row,col,hazard\n0,x,0.5\n", 2);
    expect_bad_line("row,col,hazard\n0,0,abc\n", 2);
    expect_bad_line("row,col,hazard\n0,0,\n", 2);
}

#[test]
fn negative_rows_are_rejected() {
    expect_bad_line("row,col,hazard\n-1,0,0.5\n", 2);
}

#[test]
fn indexes_at_or_above_max_dim_are_rejected() {
    expect_bad_line(&format!("row,col,hazard\n{MAX_DIM},0,0.5\n"), 2);
    expect_bad_line(&format!("row,col,hazard\n0,{MAX_DIM},0.5\n"), 2);
    expect_bad_line("row,col,hazard\n4000000000,0,0.5\n", 2);
}

#[test]
fn line_numbers_count_the_header_and_blank_lines() {
    expect_bad_line("row,col,hazard\n\n0,0,0.5\nbad\n", 4);
}

#[test]
fn nan_and_infinity_are_not_finite() {
    // Note: Rust's f64 parser happily accepts "NaN" and "inf".
    for value in ["NaN", "nan", "inf", "-inf", "infinity"] {
        match load(&format!("row,col,hazard\n0,0,0.1\n0,1,{value}\n")) {
            Err(GridError::NotFinite { line }) => assert_eq!(line, 3, "{value}"),
            other => panic!("{value:?}: expected NotFinite, got {other:?}"),
        }
    }
}

#[test]
fn duplicates_report_the_second_line() {
    match load("row,col,hazard\n0,0,0.1\n1,1,0.2\n0,0,0.3\n") {
        Err(GridError::Duplicate { cell, line }) => {
            assert_eq!(cell, c(0, 0));
            assert_eq!(line, 4);
        }
        other => panic!("expected Duplicate, got {other:?}"),
    }
}

// ---------- I/O errors ----------

#[test]
fn missing_file_is_an_io_error_with_a_source() {
    let err = Grid::from_path("tests/fixtures/does-not-exist.csv").unwrap_err();
    assert!(matches!(err, GridError::Io(_)), "got {err:?}");
    assert!(
        err.source().is_some(),
        "Io errors should expose the io::Error via source()"
    );
}

#[test]
fn io_errors_convert_with_into() {
    let err: GridError = std::io::Error::other("boom").into();
    assert!(matches!(err, GridError::Io(_)));
}

// ---------- Messages ----------

#[test]
fn messages_include_the_useful_details() {
    let msg = GridError::BadHeader {
        found: "x,y,z".into(),
    }
    .to_string();
    assert!(msg.contains("x,y,z"), "BadHeader: {msg}");

    let msg = GridError::BadLine {
        line: 4,
        reason: "expected 3 fields".into(),
    }
    .to_string();
    assert!(msg.contains("line 4"), "BadLine: {msg}");
    assert!(msg.contains("expected 3 fields"), "BadLine: {msg}");

    let msg = GridError::Duplicate {
        cell: c(0, 0),
        line: 4,
    }
    .to_string();
    assert!(
        msg.contains("r0c0") && msg.contains("line 4"),
        "Duplicate: {msg}"
    );

    let msg = GridError::NotFinite { line: 2 }.to_string();
    assert!(msg.contains("line 2"), "NotFinite: {msg}");

    let msg = GridError::Io(std::io::Error::other("disk on fire")).to_string();
    assert!(msg.contains("disk on fire"), "Io: {msg}");
}
