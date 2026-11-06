//! Tests for `CellId` parsing and display. Don't edit this file.

// unwrap()/expect() are fine in tests, including helper functions.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use grid_stats::CellId;

fn parse(s: &str) -> Result<CellId, grid_stats::ParseCellIdError> {
    s.parse::<CellId>()
}

#[test]
fn parses_a_simple_id() {
    assert_eq!(parse("r12c7"), Ok(CellId::new(12, 7)));
}

#[test]
fn parses_zero() {
    assert_eq!(parse("r0c0"), Ok(CellId::new(0, 0)));
}

#[test]
fn accepts_leading_zeros() {
    assert_eq!(parse("r007c01"), Ok(CellId::new(7, 1)));
}

#[test]
fn accepts_the_largest_u32() {
    assert_eq!(parse("r4294967295c0"), Ok(CellId::new(u32::MAX, 0)));
}

#[test]
fn rejects_malformed_ids() {
    let bad = [
        "",
        "r",
        "c",
        "rc",
        "r1",
        "c1",
        "r1c",
        "rc1",
        "12c7",
        "r12x7",
        "R1C2",
        "r1c2 ",
        " r1c2",
        "r-1c2",
        "r+1c2", // careful: u32::from_str accepts a leading '+'
        "r1c+2",
        "r1c2c3",
        "r1.5c2",
        "r4294967296c0", // one more than u32::MAX
    ];
    for s in bad {
        assert!(parse(s).is_err(), "{s:?} should be rejected");
    }
}

#[test]
fn error_message_includes_the_bad_input() {
    let err = parse("nope").unwrap_err();
    assert!(err.to_string().contains("nope"), "message was: {err}");
}

#[test]
fn displays_as_r_row_c_col() {
    assert_eq!(CellId::new(12, 7).to_string(), "r12c7");
    assert_eq!(CellId::new(0, 0).to_string(), "r0c0");
}

#[test]
fn display_and_parse_round_trip() {
    for cell in [
        CellId::new(0, 0),
        CellId::new(3, 9),
        CellId::new(9_999, 42),
        CellId::new(u32::MAX, u32::MAX),
    ] {
        assert_eq!(parse(&cell.to_string()), Ok(cell));
    }
}
