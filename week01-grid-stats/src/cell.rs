use std::fmt;
use std::str::FromStr;

/// A position in the grid. `row` and `col` both start at 0.
///
/// Its text form is `r<row>c<col>`, for example `r12c7`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CellId {
    pub row: u32,
    pub col: u32,
}

impl CellId {
    pub fn new(row: u32, col: u32) -> Self {
        Self { row, col }
    }
}

/// Returned when text isn't a valid cell ID.
///
/// TODO: decide what this needs to hold. Its message must include the bad input.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseCellIdError {}

impl fmt::Display for ParseCellIdError {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!("e.g. invalid cell ID \"r1x2\" (expected r<row>c<col>)")
    }
}

impl std::error::Error for ParseCellIdError {}

impl FromStr for CellId {
    type Err = ParseCellIdError;

    /// Parses `r<row>c<col>`. See the README for exactly what is accepted.
    fn from_str(_s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

impl fmt::Display for CellId {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}
