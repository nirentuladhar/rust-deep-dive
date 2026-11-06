//! The `grid-stats` command-line tool. The exact behaviour is in `README.md`.
//!
//! Usage: grid-stats <INPUT> [--format json|csv] [--only <CELL>]

use std::process::ExitCode;

fn main() -> ExitCode {
    // TODO:
    // 1. Parse the arguments with clap's derive API.
    // 2. Load the grid with `grid_stats::Grid::from_path`.
    // 3. Print stats for every cell that has a value (or only `--only`) in the chosen format.
    // 4. On any error, print `error: <message>` to stderr and return ExitCode::FAILURE.
    //    No panics, and nothing on stdout.
    todo!()
}
