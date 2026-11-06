//! Tests for `Grid::neighbours` and `Grid::stats`. Don't edit this file.
//!
//! The small fixture looks like this (r1c2 is missing):
//!
//! ```text
//!         c0     c1     c2
//!   r0   0.12   0.40   0.85
//!   r1   0.33   0.30    --
//!   r2   0.05   0.11   0.60
//! ```

// unwrap()/expect() are fine in tests, including helper functions.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use grid_stats::{CellId, Grid, Neighbours, Stats};

fn small() -> Grid {
    Grid::from_path("tests/fixtures/small.csv").unwrap()
}

fn full_3x3() -> Grid {
    let mut csv = String::from("row,col,hazard\n");
    for row in 0..3 {
        for col in 0..3 {
            csv.push_str(&format!("{row},{col},0.5\n"));
        }
    }
    Grid::from_reader(csv.as_bytes()).unwrap()
}

fn c(row: u32, col: u32) -> CellId {
    CellId::new(row, col)
}

/// The cell IDs the iterator yields, in order.
fn ids(grid: &Grid, centre: CellId) -> Vec<CellId> {
    grid.neighbours(centre).map(|(cell, _)| cell).collect()
}

fn assert_close(actual: f64, expected: f64) {
    assert!(
        (actual - expected).abs() < 1e-9,
        "expected {expected}, got {actual}"
    );
}

// ---------- neighbours ----------

#[test]
fn returns_your_own_iterator_type() {
    // This only compiles if `neighbours` returns `Neighbours`, not a Vec.
    let grid = small();
    let _it: Neighbours<'_> = grid.neighbours(c(1, 1));
}

#[test]
fn counts_on_a_full_grid() {
    let grid = full_3x3();
    assert_eq!(ids(&grid, c(0, 0)).len(), 4, "corner");
    assert_eq!(ids(&grid, c(0, 1)).len(), 6, "edge");
    assert_eq!(ids(&grid, c(1, 1)).len(), 9, "interior");
    assert_eq!(ids(&grid, c(2, 2)).len(), 4, "opposite corner");
}

#[test]
fn yields_row_by_row_and_skips_missing_cells() {
    let grid = small();
    assert_eq!(
        ids(&grid, c(1, 1)),
        vec![
            c(0, 0),
            c(0, 1),
            c(0, 2),
            c(1, 0),
            c(1, 1),
            c(2, 0),
            c(2, 1),
            c(2, 2)
        ]
    );
}

#[test]
fn includes_the_centre_cell() {
    let grid = small();
    assert_eq!(
        ids(&grid, c(0, 0)),
        vec![c(0, 0), c(0, 1), c(1, 0), c(1, 1)]
    );
}

#[test]
fn bottom_right_corner_skips_the_missing_cell() {
    let grid = small();
    assert_eq!(ids(&grid, c(2, 2)), vec![c(1, 1), c(2, 1), c(2, 2)]);
}

#[test]
fn yields_values_too() {
    let grid = small();
    let pairs: Vec<(CellId, f64)> = grid.neighbours(c(0, 0)).collect();
    assert_eq!(
        pairs,
        vec![
            (c(0, 0), 0.12),
            (c(0, 1), 0.40),
            (c(1, 0), 0.33),
            (c(1, 1), 0.30)
        ]
    );
}

#[test]
fn centre_outside_the_grid_still_finds_cells_inside_it() {
    let grid = small();
    assert_eq!(ids(&grid, c(3, 3)), vec![c(2, 2)]);
    assert_eq!(ids(&grid, c(100, 100)), vec![]);
}

#[test]
fn does_not_overflow_at_the_edges_of_u32() {
    let grid = small();
    assert_eq!(ids(&grid, c(u32::MAX, u32::MAX)), vec![]);
    assert_eq!(ids(&grid, c(u32::MAX, 0)), vec![]);
}

#[test]
fn keeps_returning_none_once_finished() {
    let grid = small();
    let mut it = grid.neighbours(c(0, 0));
    for _ in 0..4 {
        assert!(it.next().is_some());
    }
    assert_eq!(it.next(), None);
    assert_eq!(it.next(), None);
}

// ---------- stats ----------

#[test]
fn stats_for_a_corner() {
    let s = small().stats(c(0, 0)).unwrap();
    assert_eq!(s.count, 4);
    assert_close(s.mean, (0.12 + 0.40 + 0.33 + 0.30) / 4.0);
    assert_close(s.max, 0.40);
}

#[test]
fn stats_for_the_centre() {
    let s = small().stats(c(1, 1)).unwrap();
    assert_eq!(s.count, 8);
    assert_close(s.mean, 0.345);
    assert_close(s.max, 0.85);
}

#[test]
fn stats_for_a_single_cell_grid() {
    let grid = Grid::from_reader("row,col,hazard\n0,0,0.7\n".as_bytes()).unwrap();
    assert_eq!(
        grid.stats(c(0, 0)),
        Some(Stats {
            count: 1,
            mean: 0.7,
            max: 0.7
        })
    );
}

#[test]
fn stats_need_a_value_at_the_centre() {
    let grid = small();
    assert_eq!(grid.stats(c(1, 2)), None, "r1c2 is missing");
    assert_eq!(grid.stats(c(3, 3)), None, "outside the grid");
}

#[test]
fn stats_handle_negative_values() {
    // max must start from the first value, not from 0.0.
    let grid = Grid::from_reader("row,col,hazard\n0,0,-0.5\n0,1,-0.25\n".as_bytes()).unwrap();
    let s = grid.stats(c(0, 0)).unwrap();
    assert_close(s.max, -0.25);
    assert_close(s.mean, -0.375);
}
