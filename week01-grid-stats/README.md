# Week 1 — Refresher: grid stats CLI

**When:** Saturday 7 November 2026 · **Time:** about 5–6 hours
**Reading (as needed):** Rust Book [ch. 9, Error Handling](https://doc.rust-lang.org/book/ch09-00-error-handling.html) and [ch. 13, Iterators and Closures](https://doc.rust-lang.org/book/ch13-00-functional-features.html)
**Allowed crates:** `clap`, `serde`, `serde_json` (already added). Parse the CSV and write the error type by hand. That's part of the exercise.

This week gets you back into Rust. It also shows where your weak spots are, so later projects can push on them.

---

## What you're building

A command-line tool called `grid-stats`. It reads a CSV of grid cells with a hazard value. For each cell, it works out stats over that cell's 3×3 neighbourhood, then prints the results.

```sh
cargo run -p grid-stats -- week01-grid-stats/tests/fixtures/small.csv
```

```json
{"cell":"r0c0","count":4,"mean":0.2875,"max":0.4}
{"cell":"r0c1","count":5,"mean":0.4,"max":0.85}
...
```

## Getting started

```sh
cargo test -p grid-stats --no-fail-fast
```

Everything fails with `not yet implemented`. That's expected. (`--no-fail-fast` runs every test file. Without it, cargo stops at the first file with a failure.) The skeleton in `src/` has the public names the tests need, with `todo!()` where your code goes.

**Suggested order:**

1. `src/cell.rs`: `CellId` parsing and display → `cargo test -p grid-stats --test cell_id`
2. `src/error.rs`: `GridError` → part of `--test loading`
3. `src/grid.rs`: loading, `get`, `cells` → `--test loading`
4. `src/grid.rs`: `Neighbours` and `stats` → `--test neighbours`
5. `src/main.rs`: the CLI → `--test cli`

**Rules:**

- Don't edit the files in `tests/`. Make them pass as they are.
- You can change anything private: add fields, helper functions, modules.
- Add **at least 3 tests of your own**, as unit tests inside `src/` (in a `#[cfg(test)] mod tests`). Test cases you think the provided tests missed.

---

## The spec

### 1. `CellId` (src/cell.rs)

Text form: `r<row>c<col>`, for example `r12c7`.

**`FromStr`** accepts exactly: lowercase `r`, one or more ASCII digits, lowercase `c`, one or more ASCII digits. Nothing else.

| Input | Result |
|---|---|
| `r12c7` | row 12, col 7 |
| `r007c01` | row 7, col 1 (leading zeros are fine) |
| `r4294967295c0` | row `u32::MAX` |
| `R1C2`, `12c7`, `r1c`, `r1c2c3`, `r1.5c2` | error |
| ` r1c2`, `r1c2 ` | error (no spaces) |
| `r-1c2`, `r+1c2` | error |
| `r4294967296c0` | error (too big for `u32`) |

Watch out: `"+1".parse::<u32>()` succeeds. Your parser must still reject `r+1c2`.

**`ParseCellIdError`**: its message must include the bad input, for example: `invalid cell ID "nope" (expected r<row>c<col>)`.

**`Display`**: prints `r<row>c<col>`, so `CellId::new(12, 7).to_string() == "r12c7"`.

### 2. `GridError` (src/error.rs)

The variants are given, because the tests match on them. You write:

- **`Display`**. Each message must include:
  - `BadHeader`: what was found
  - `BadLine`: `line N` and the reason
  - `Duplicate`: the cell (e.g. `r0c0`) and `line N`
  - `NotFinite`: `line N`
  - `Io`: the underlying error's message
- **`std::error::Error`**, with `source()` returning the inner `io::Error` for `Io`
- **`From<std::io::Error>`**, so `?` works on I/O calls

### 3. Loading a grid (src/grid.rs)

`Grid::from_reader(reader)` reads CSV. `Grid::from_path(path)` opens a file and calls `from_reader`.

**Lines:**

- Line numbers start at **1** and count the header and blank lines.
- Trim spaces, tabs and `\r` from the start and end of every line (so Windows line endings work).
- Skip lines that are empty after trimming.

**Header:**

- The first line, after trimming, must be exactly `row,col,hazard`.
- Otherwise: `BadHeader { found }`, where `found` is the trimmed first line. Empty input gives `found: ""`.

**Data lines:**

- Exactly 3 comma-separated fields. Trim spaces around each field.
- `row` and `col` parse as `u32`, and each must be **less than `MAX_DIM`** (10,000).
- `hazard` parses as `f64`.
- Any of these failing → `BadLine { line, reason }`, with a short plain reason like `expected 3 fields, found 2`.
- `hazard` is NaN or infinite → `NotFinite { line }`. Rust's `f64` parser accepts `"NaN"` and `"inf"`, so you must check this yourself.
- The same cell twice → `Duplicate { cell, line }`, where `line` is the second occurrence.

**The grid:**

- `height` = largest row + 1, `width` = largest col + 1. Header-only input gives a 0×0 grid.
- Store values row by row in `Vec<Option<f64>>`. Cells not in the input are `None`.
- `get(cell)` returns `None` for missing cells and for cells outside the grid.
- `cells()` yields every cell that **has a value**, row by row, whatever order the input lines were in.

### 4. `neighbours` (src/grid.rs)

`grid.neighbours(cell)` returns a `Neighbours` iterator over the 3×3 block centred on `cell`:

- It visits positions row by row, from (row−1, col−1) to (row+1, col+1). **The centre is included.**
- It yields `(CellId, f64)` only for positions that are **inside the grid** and **have a value**.
- The centre doesn't need a value, or even to be inside the grid. Centre `r3c3` on a 3×3 grid yields just `r2c2`.
- It must not overflow or panic at the edges, including `row = 0` and `row = u32::MAX`. Look at `checked_sub` and `checked_add`.
- Once it returns `None`, it keeps returning `None`.
- Write it as your own struct that implements `Iterator`. Don't build a `Vec` and return its iterator.

### 5. `stats` (src/grid.rs)

`grid.stats(cell)` returns `Some(Stats { count, mean, max })` over everything `neighbours(cell)` yields. Use the iterator; don't loop over the grid a second way.

- Returns `None` if `cell` itself has no value.
- `max` must work for negative values. Don't start it at `0.0`.

### 6. The CLI (src/main.rs)

```text
grid-stats <INPUT> [--format json|csv] [--only <CELL>]
```

Use clap's derive API. `--format` defaults to `json`.

**Output:** one result per cell that has a value, in `cells()` order. `mean` and `max` are **rounded to 4 decimal places**.

JSON (one object per line, fields in this order):
```json
{"cell":"r0c0","count":4,"mean":0.2875,"max":0.4}
```

CSV (with a header):
```text
cell,count,mean,max
r0c0,4,0.2875,0.4
```

**`--only r1c1`** prints just that cell. If the cell has no value, that's an error, and the message must name the cell.

**Errors:** print `error: <message>` to **stderr**, print nothing to stdout, and exit with a non-zero code (return `ExitCode::FAILURE`). No panics. Clap's own errors for bad arguments are fine as they are.

---

## Done checklist

- [ ] `cargo test -p grid-stats` passes
- [ ] You added at least 3 tests of your own in `src/`
- [ ] `cargo clippy -p grid-stats --all-targets -- -D warnings` is clean (this includes no `unwrap()`/`expect()` outside tests)
- [ ] `cargo fmt --check` is clean
- [ ] The skeleton's `#[allow(unreachable_code)]` line is gone
- [ ] The **design note** below is filled in
- [ ] Hours logged in `LOG.md`

Then push, and ask Claude: *"Review week 1"*.

## Stretch (only if you have time)

- Replace your hand-written error with the `thiserror` crate. Compare the two in your design note.
- Make CSV output use one iterator chain, with no intermediate `Vec`.

---

## Design note

*Half a page at most. Fill this in when you're done.*

**What I chose, what I rejected, and why:**

**What surprised me:**

**What I'd do differently:**

**Where I got stuck (the diagnostic: list every one, however small):**

| What I was doing | What went wrong | What fixed it | Kind (syntax / borrow checker / types / traits / other) |
|---|---|---|---|
| | | | |
