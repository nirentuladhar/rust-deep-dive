use crate::{CellId, GridError};
use std::io::BufRead;
use std::path::Path;

/// Rows and columns must be less than this.
///
/// Without a limit, one typo like `4000000000,0,0.5` would try to allocate gigabytes.
pub const MAX_DIM: u32 = 10_000;

/// Stats for one cell's 3×3 neighbourhood.
#[derive(Debug, Clone, PartialEq)]
pub struct Stats {
    /// How many cells in the neighbourhood have a value (1 to 9).
    pub count: usize,
    pub mean: f64,
    pub max: f64,
}

/// A grid of optional hazard values.
#[derive(Debug)]
pub struct Grid {
    width: u32,
    height: u32,
    /// Row by row: the value for (row, col) is at index `row * width + col`.
    /// `None` means that cell wasn't in the input.
    values: Vec<Option<f64>>,
}

impl Grid {
    /// Reads `row,col,hazard` CSV. See the README for the exact rules.
    pub fn from_reader<R: BufRead>(_reader: R) -> Result<Self, GridError> {
        todo!()
    }

    /// Opens a file and reads it with [`Grid::from_reader`].
    pub fn from_path(_path: impl AsRef<Path>) -> Result<Self, GridError> {
        todo!()
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    /// The value at `cell`, or `None` if it's missing or outside the grid.
    pub fn get(&self, _cell: CellId) -> Option<f64> {
        todo!()
    }

    /// Every cell that has a value, row by row (top to bottom, left to right).
    #[allow(unreachable_code)] // TODO: delete this line once you've written the body
    pub fn cells(&self) -> impl Iterator<Item = CellId> + '_ {
        todo!();
        std::iter::empty()
    }

    /// The 3×3 block centred on `cell`. See the README for the exact rules.
    ///
    /// This must return your own iterator type, not a Vec.
    pub fn neighbours(&self, _cell: CellId) -> Neighbours<'_> {
        todo!()
    }

    /// Stats over the 3×3 block centred on `cell`, using [`Grid::neighbours`].
    ///
    /// Returns `None` if `cell` itself has no value.
    pub fn stats(&self, _cell: CellId) -> Option<Stats> {
        todo!()
    }
}

/// Iterator over the cells with values in a 3×3 block. Made by [`Grid::neighbours`].
pub struct Neighbours<'a> {
    grid: &'a Grid,
    centre: CellId,
    // TODO: add whatever state you need to remember where you are.
}

impl Iterator for Neighbours<'_> {
    type Item = (CellId, f64);

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
