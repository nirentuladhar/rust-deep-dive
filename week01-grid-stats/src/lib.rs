//! Week 1 — grid stats. The full spec is in `README.md`.
//!
//! The public names below are fixed, because the tests in `tests/` use them.
//! Everything inside them is yours to write.

mod cell;
mod error;
mod grid;

pub use cell::{CellId, ParseCellIdError};
pub use error::GridError;
pub use grid::{Grid, MAX_DIM, Neighbours, Stats};
