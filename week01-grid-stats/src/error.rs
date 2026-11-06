use crate::CellId;
use std::fmt;

/// Everything that can go wrong while loading a grid.
///
/// The variants and their fields are fixed, because the tests match on them.
/// You may add more variants if you want to.
///
/// All `line` numbers start at 1 and count the header line.
#[derive(Debug)]
pub enum GridError {
    /// Reading the input failed.
    Io(std::io::Error),
    /// The first line wasn't `row,col,hazard`. `found` is what was there instead.
    BadHeader { found: String },
    /// A data line couldn't be parsed. `reason` says why, in plain words.
    BadLine { line: usize, reason: String },
    /// The same cell appeared twice. `line` is where the second one is.
    Duplicate { cell: CellId, line: usize },
    /// The hazard value was NaN or infinite.
    NotFinite { line: usize },
}

impl fmt::Display for GridError {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!("see the README for what each message must include")
    }
}

// TODO: an empty impl compiles, but `source()` then always returns None.
// Override `source()` so an `Io` error exposes the underlying io::Error.
impl std::error::Error for GridError {}

impl From<std::io::Error> for GridError {
    fn from(_err: std::io::Error) -> Self {
        todo!()
    }
}
