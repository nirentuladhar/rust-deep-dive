# Rust Deep-Dive Plan

**7 November 2026 – 19 February 2027 · about 5–6 hours a week · macOS**

This is a 13-week, project-based plan to take your Rust from "rusty" to solid. Each project targets one hard area of the language. Each one also builds a piece of the final project, a **grid risk query service**.

---

## Contents

1. [How this plan works](#1-how-this-plan-works)
2. [Calendar](#2-calendar)
3. [One-time setup (do this before 7 November)](#3-one-time-setup-do-this-before-7-november)
4. [The weekly routine](#4-the-weekly-routine)
5. [Rules: done checklist, hints, grilling](#5-rules-done-checklist-hints-grilling)
6. [Week 1 — Refresher: grid stats CLI](#6-week-1--refresher-grid-stats-cli)
7. [P1 — Zero-copy grid parser (weeks 2–3)](#7-p1--zero-copy-grid-parser-weeks-23)
8. [P2 — Pluggable aggregators (weeks 4–5)](#8-p2--pluggable-aggregators-weeks-45)
9. [Checkpoint (Saturday 12 December)](#9-checkpoint-saturday-12-december)
10. [P3 — LRU cache, built twice (weeks 6–7)](#10-p3--lru-cache-built-twice-weeks-67)
11. [P4 — Concurrency (weeks 8–9)](#11-p4--concurrency-weeks-89)
12. [P5 — Async from the ground up (weeks 10–11)](#12-p5--async-from-the-ground-up-weeks-1011)
13. [Capstone — Grid risk query service (weeks 12–13)](#13-capstone--grid-risk-query-service-weeks-1213)
14. [Reading list](#14-reading-list)
15. [Sources](#15-sources)

---

## 1. How this plan works

| Item | Decision |
|---|---|
| Goal | Rust that's useful for your kind of work, plus a real understanding of what happens underneath |
| Pace | About 5–6 hours a week. Each 2-week project is about 10–12 hours |
| Shape | 1 refresher, 5 focused projects, 1 capstone |
| Help | Each project has a spec and tests. A hint ladder is there if you get stuck. Claude reviews your code and grills you afterwards |
| Readings | One short reading per project. Read it **before** you start |
| Scope | `unsafe`: yes. `macro_rules!`: small. Procedural macros: stretch goal only. Embedded: no |
| Data | Public or made-up data only. This is a personal project |
| Code | A private GitHub repo, set up one project at a time |
| Check-in | Saturday 12 December, before the holiday break |

**Why this order:** each project leans on the one before it. The parser (P1) becomes the capstone's file reader. The aggregators (P2) become its query maths. The cache (P3) becomes thread-safe in P4. P5 teaches the async machinery that axum hides from you in the capstone.

---

## 2. Calendar

| Week | Starts (Sat) | Project | Reading before you start |
|---|---|---|---|
| 1 | 7 Nov 2026 | Refresher: grid stats CLI | Rust Book ch. 9 and ch. 13, as needed |
| 2 | 14 Nov | P1: Zero-copy parser, part A | "Common Rust Lifetime Misconceptions" |
| 3 | 21 Nov | P1: Zero-copy parser, part B | *Rust for Rustaceans*: Foundations chapter |
| 4 | 28 Nov | P2: Aggregators, part A | *Rust for Rustaceans*: types/traits chapter |
| 5 | 5 Dec | P2: Aggregators, part B | *Rust for Rustaceans*: interface design chapter; Rust Book 20.5 (Macros) |
| — | **12 Dec** | **Checkpoint** (then start P3) | — |
| 6 | 12 Dec | P3: LRU cache, safe version | *Too Many Lists*: "A Bad Safe Deque" |
| 7 | 19 Dec | P3: LRU cache, unsafe version | *Too Many Lists*: "An Ok Unsafe Queue"; *Rust for Rustaceans*: unsafe chapter |
| — | 26 Dec | **Break** | — |
| — | 2 Jan 2027 | **Break** | — |
| 8 | 9 Jan | P4: Concurrency, parts A–B | *Rust Atomics and Locks* ch. 1 |
| 9 | 16 Jan | P4: Concurrency, part C | *Rust Atomics and Locks* ch. 2–4 |
| 10 | 23 Jan | P5: Async, parts A–B | Rust Book 17.5; *Rust for Rustaceans*: async chapter |
| 11 | 30 Jan | P5: Async, part C | Tokio tutorial |
| 12 | 6 Feb | Capstone, part 1 | axum and memmap2 docs |
| 13 | 13 Feb | Capstone, part 2 | criterion, oha and samply docs |
| — | **19 Feb 2027** | **Finish** | — |

---

## 3. One-time setup (do this before 7 November)

### 3.1 Toolchain

```sh
# 1. Apple's command line tools (rustup needs the linker from these)
xcode-select --install

# 2. Rust via rustup (skip if already installed): https://rustup.rs
rustup update stable
rustc --version          # 2024 edition needs 1.85 or newer

# 3. Formatter and linter (usually already included)
rustup component add rustfmt clippy

# 4. Nightly toolchain + Miri (needed in P3 for checking unsafe code)
rustup +nightly component add miri
```

**Editor:** use an editor with **rust-analyzer** (VS Code with the rust-analyzer extension, or RustRover). It shows inferred types and lifetimes inline, which helps a lot in P1.

### 3.2 Tools for later projects

You can install these now or when you need them.

| Tool | Used in | Install on macOS | What it does |
|---|---|---|---|
| criterion | P1 onwards | `cargo add --dev criterion` (per crate) | Benchmarking library |
| samply | Capstone | `cargo install --locked samply` or `brew install samply` | CPU profiler. Opens results in the Firefox Profiler |
| oha | Capstone | `brew install oha` or `cargo install oha` | HTTP load generator. Shows latency percentiles |

**macOS notes:**
- **samply:** `samply record ./target/release/your-binary` works for programs you launch yourself. Attaching to a process that's already running needs a one-time `samply setup`.
- **oha:** macOS limits open files per shell. Before a load test with many connections, run `ulimit -n 4096` in that terminal.
- **Apple Silicon:** if your Mac has an M-series chip, it uses ARM. ARM has a weaker memory model than Intel x86. That means memory-ordering bugs in P4 can actually show up on your machine, which makes it a better test bed than an Intel PC. (*Rust Atomics and Locks* ch. 7 covers Intel vs ARM.)

### 3.3 GitHub repo

The repo is `nirentuladhar/rust-deep-dive` (private).

To let Claude push starter code and read your solutions for reviews, connect GitHub in claude.ai under **Settings → Connectors**. Until then, Claude sends each project as a zip, and you attach your code for reviews.

### 3.4 Repo layout

The repo is one Cargo **workspace**. Each project is its own crate. Claude adds each crate when you reach that project.

```text
rust-deep-dive/
├── Cargo.toml              # workspace root
├── LOG.md                  # your weekly hours + notes (for the checkpoint)
├── week01-grid-stats/
├── p1-grid-parser/
├── p2-aggregators/
├── p3-lru/
├── p4-concurrency/
├── p5-async/
└── capstone/
    ├── grid-core/          # library: file format, views, aggregators
    ├── grid-gen/           # binary: makes the synthetic data file
    └── grid-server/        # binary: the axum API
```

Root `Cargo.toml`:

```toml
[workspace]
resolver = "3"
members = ["week01-grid-stats"]   # add each project as you reach it

[workspace.package]
edition = "2024"
```

Each crate's `Cargo.toml` then uses `edition.workspace = true`.

**Tip for the capstone:** don't make the capstone depend on the project crates directly. Copy the parts you want to keep into `grid-core` and clean them up. That cleanup is part of the learning.

### 3.5 Starting a new chat

Claude may not have this conversation's details in a new chat. If you start a new chat for a review or a hint, **attach this file** (and say which project you're on).

---

## 4. The weekly routine

**Saturday (or whenever you start):**

1. **Read** the paired reading first (30–60 minutes). Don't skip this. Several projects are designed around one idea from the reading.
2. **Get the starter code.** Ask Claude: *"Start P1"*. You'll get a `README.md` spec, a skeleton and failing tests, pushed to your repo (or as a zip).
3. **Read the spec and run the tests.** They should fail. Your job is to make them pass without changing them.

**During the week:**

4. **Build.** Run `cargo test` often.
5. **Stuck for more than 30 minutes?** Use the hint ladder (section 5.2).
6. **Log your hours** in `LOG.md` (date, hours, one line on what was hard).

**When finished:**

7. **Run the done checklist** (section 5.1).
8. **Push**, then tell Claude: *"Review P1"*. You'll get a code review, then the grilling.
9. If the grilling says so, do the fix-up exercise before starting the next project.

---

## 5. Rules: done checklist, hints, grilling

### 5.1 Done checklist (every project)

- [ ] `cargo test` passes, with no starter tests changed or deleted
- [ ] `cargo clippy --all-targets -- -D warnings` is clean
- [ ] `cargo fmt --check` is clean
- [ ] No `unwrap()` or `expect()` outside tests, unless a comment explains why it can't fail
- [ ] Every `unsafe` block has a `// SAFETY:` comment explaining why it's sound (P3 onwards)
- [ ] The crate's `README.md` has a **design note** (half a page at most):
  - What you chose, and what you rejected and why
  - What surprised you
  - What you'd do differently

The design note matters most. Explaining a choice is how you find out whether you really understand it.

### 5.2 Hint ladder

Each project has three levels of hints. Ask for them one at a time.

| Level | What you get | How to ask |
|---|---|---|
| Hint 1 | A nudge: which concept or doc page is relevant | *"P1 hint 1: [what you're stuck on] + the compiler error"* |
| Hint 2 | The approach, without code | *"P1 hint 2"* |
| Hint 3 | A code sketch of the tricky part only | *"P1 hint 3"* |

**Rules:**
- Try for 30 minutes before asking. Read the full compiler error first. Rust's errors often contain the fix.
- Always paste the actual error and the code it points to.
- Hints never give a full solution. If you need more than hint 3, that's a sign to re-read the reading. Note it in your design note.

### 5.3 Grilling

After each review, Claude asks **5–8 questions** about your code and the concepts behind it. For example: *"Why does this need `Arc` here and not `Rc`?"* or *"What would break if `GridView` didn't have a lifetime parameter?"*

- Answer from your own understanding first, without looking things up.
- **2 or more misses** → a short fix-up exercise (about 1 hour) on the weak area before the next project.
- Your answers feed the checkpoint and the shape of later projects.

---

## 6. Week 1 — Refresher: grid stats CLI

**When:** Saturday 7 November · **Time:** about 5–6 hours
**Reading:** Rust Book ch. 9 (Error Handling) and ch. 13 (Iterators and Closures), as needed
**Allowed crates:** `clap`, `serde`, `serde_json`. Parse the CSV and write the error type by hand. That's part of the exercise.

### What you build

A command-line tool, `grid-stats`. It reads a CSV of grid cells with a hazard value. For each cell, it works out stats over that cell's 3×3 neighbourhood, then prints JSON.

Input:
```text
row,col,hazard
0,0,0.12
0,1,0.40
1,0,0.33
```

Output, one JSON object per line:
```json
{"cell":"r0c0","count":3,"mean":0.2833,"max":0.40}
```

### Requirements

1. **`CellId { row: u32, col: u32 }`** implements `FromStr` (parses `"r12c7"`) and `Display` (prints `r12c7`).
2. **`Grid`** stores values in a `Vec<Option<f64>>`. Width and height come from the largest row and column seen. Missing cells are `None`.
3. **`GridError`**, a hand-written enum, with `Display`, `std::error::Error` and `From<std::io::Error>`. Variants:
   - `Io`
   - `BadHeader`
   - `BadLine { line, reason }`
   - `Duplicate { cell, line }`
   - `NotFinite { line }` (NaN or infinity)
4. **`Grid::neighbours(&self, cell) -> Neighbours<'_>`**. `Neighbours` is your own struct implementing `Iterator<Item = (CellId, f64)>`. It yields the cell and its up-to-8 neighbours, and skips cells that are missing or off the edge. Don't build a `Vec` and return it.
5. **`Grid::stats(&self, cell) -> Option<Stats>`** works out count, mean and max using the iterator.
6. **CLI:** `grid-stats <input.csv> [--format json|csv] [--only r12c7]`.
7. **Errors:** bad input exits with a non-zero code and a readable message that includes the line number. No panics.
8. **Tests:**
   - A corner cell sees at most 4 cells, an edge cell 6, an interior cell 9
   - Missing cells are skipped
   - One test per `GridError` variant
   - One integration test in `tests/` that runs on a fixture file

### Concepts

Structs, enums, pattern matching, custom error types, `?`, `FromStr`, `Display`, writing your own iterator, closures, unit and integration tests, module layout.

### Also a diagnostic

In your design note, list **every place you got stuck** and why: syntax, the borrow checker, trait bounds, and so on. Claude uses this to adjust P1–P5.

### Stretch

- Replace your hand-written error with `thiserror`. Compare the two in your design note.
- Add `--format csv` output using an iterator chain only, with no intermediate `Vec`.

---

## 7. P1 — Zero-copy grid parser (weeks 2–3)

**When:** 14 November and 21 November · **Time:** about 10–12 hours
**Reading:** "Common Rust Lifetime Misconceptions" (before part A). *Rust for Rustaceans*, Foundations chapter (before part B; it's the free sample chapter).
**Allowed crates:** `criterion` and `trybuild` (dev only).

**The idea:** parse data by **borrowing** from the input buffer instead of copying it into new `String`s and `Vec`s. The compiler uses lifetimes to prove the borrowed views never outlive the buffer.

### Part A (week 2): text records with `Cow`

A text format with one record per line:

```text
r12c7|SSP2-4.5 2050|0.42
r12c8|name with \| escaped pipe|0.37
```

1. `Record<'a> { cell: CellId, scenario: Cow<'a, str>, value: f32 }`. `scenario` borrows from the input, **unless** it contains `\|` escapes. Then (and only then) it allocates an owned, unescaped `String`.
2. `Records<'a>` is an iterator over `&'a str` that yields `Result<Record<'a>, ParseError>`.
3. An `OwnedRecord` version that always allocates, for comparison.
4. A **criterion benchmark** that parses about 1 million lines with both versions. Record the results in your design note.

### Part B (week 3): binary grid view

A binary format called `GRD1`. All numbers are little-endian:

| Offset | Size | Field |
|---|---|---|
| 0 | 4 | Magic bytes `b"GRD1"` |
| 4 | 2 | Version, `u16` (= 1) |
| 6 | 2 | Layers (scenarios), `u16` |
| 8 | 4 | Width, `u32` |
| 12 | 4 | Height, `u32` |
| 16 | 4 | No-data value, `f32` |
| 20 | 12 | Reserved (zeros) |
| 32 | … | Values: `f32`, layer by layer, then row by row |

1. `GridView::parse(bytes: &'a [u8]) -> Result<GridView<'a>, FormatError>`. It checks the header, but does **not** copy the values.
2. `get(layer, row, col) -> Option<f32>`. Returns `None` for out-of-range cells or the no-data value. Use `f32::from_le_bytes`.
3. `row(layer, r) -> impl Iterator<Item = f32> + 'a`.
4. `write_grid(...)` writes a `GRD1` file. You'll need it for tests and for the capstone's generator.
5. `FormatError` variants: `TooShort`, `BadMagic`, `UnsupportedVersion`, `SizeMismatch { expected, actual }`.
6. An **`OwnedGrid`** (`Vec<f32>`) version, plus a criterion benchmark summing one layer of a roughly 100 MB file with each version.
7. A **compile-fail test** using `trybuild` that proves a `GridView` can't outlive its buffer.

### Concepts

Lifetimes in structs and `impl` blocks, elision rules, borrowing iterators, `Cow`, slices, endianness, benchmarking, compile-fail tests.

### Stretch

- Make `GridView` generic over `B: AsRef<[u8]>`, so it can wrap a `Vec<u8>` or a memory map.
- Try viewing the values as `&[f32]` directly with the `bytemuck` crate. Explain in your design note why alignment and endianness make this tricky.

**Feeds the capstone:** `GridView` becomes the reader over the memory-mapped file.

---

## 8. P2 — Pluggable aggregators (weeks 4–5)

**When:** 28 November and 5 December · **Time:** about 10–12 hours
**Reading:** *Rust for Rustaceans*, the types/traits chapter (before part A), then the interface design chapter (before part B). Rust Book 20.5 (Macros).
**Allowed crates:** `criterion` (dev). For the stretch goal: `syn`, `quote`, `proc-macro2`.

**The idea:** design a small library the way a good crate would. Choose between generics (static dispatch) and trait objects (dynamic dispatch), and understand the cost of each.

### Part A (week 4): the trait and static dispatch

1. Define this trait:
   ```rust
   pub trait Aggregator {
       type Output;
       fn push(&mut self, value: f32);
       fn finish(self) -> Self::Output;
   }
   ```
2. Implement `Count`, `Mean`, `Max`, `Min` and `Percentile { p }`. `Percentile` needs storage. Think about the memory cost.
3. Write an **extension trait**, `AggregateExt`, with a blanket impl for every `Iterator<Item = f32>`. Then this works:
   ```rust
   let m = grid.row(0, 5).aggregate(Mean::default());
   ```
4. Use `macro_rules!` to cut test boilerplate. For example:
   ```rust
   agg_tests! { mean_basic: Mean => [1.0, 2.0, 3.0] => 2.0 }
   ```

### Part B (week 5): dynamic dispatch

1. The CLI lets the user choose aggregators at runtime, e.g. `--agg mean,max,p95`. So you need a `Vec<Box<dyn ...>>`.
2. `Aggregator` as written **can't** be used as `dyn Aggregator`. Find out why: the associated type and `finish(self)` both cause problems. Older docs call this "object safety"; newer docs call it "dyn compatibility".
3. Design a `DynAggregator` trait that *can* be used as a trait object. Add a blanket `impl<A> DynAggregator for A where A: Aggregator<Output = f64>`, or something similar.
4. **Benchmark** static vs dynamic dispatch on 10 million values. Explain the difference in your design note.

### Concepts

Traits, associated types, generics vs trait objects, dyn compatibility, blanket impls, extension traits, `macro_rules!`.

### Stretch (optional)

Write a derive macro so this generates a `push` that feeds every field:

```rust
#[derive(Aggregate)]
struct Summary { count: Count, mean: Mean, max: Max }
```

**Feeds the capstone:** the region query endpoint (`?agg=mean,max,p95`).

---

## 9. Checkpoint (Saturday 12 December)

Before starting P3, tell Claude *"Checkpoint"* and bring:

- Your `LOG.md`: hours per week and what was hard
- How the week 1, P1 and P2 grillings went
- Anything you want more or less of

Possible changes include adding a week to P3 if lifetimes are still shaky, shrinking P5 if async is clicking, or adjusting the capstone scope. Claude also confirms the Saturday reminders skip the two break weeks.

---

## 10. P3 — LRU cache, built twice (weeks 6–7)

**When:** 12 December and 19 December · **Time:** about 10–12 hours
**Reading:** *Too Many Lists*, "A Bad Safe Deque" (before week 6), then "An Ok Unsafe Queue" (before week 7; it also covers Miri). *Rust for Rustaceans*, the unsafe chapter.
**Allowed crates:** standard library only, plus `criterion` (dev).

**The idea:** an LRU (least recently used) cache needs a hash map plus a doubly linked list. Doubly linked lists clash with Rust's single-owner rules. Building one twice shows you exactly what the safe tools cost and what `unsafe` really means.

### The API (same for both versions)

```rust
LruCache::<K, V>::new(capacity)
cache.get(&key) -> Option<&V>        // marks the key as most recently used
cache.put(key, value) -> Option<(K, V)>  // returns the evicted entry, if any
cache.len()
cache.iter()                          // most to least recently used
```

### Week 6: safe version

- A `HashMap<K, Rc<RefCell<Node>>>` plus a linked list using `Rc` for forward links and `Weak` for back links.
- Expect runtime `BorrowMutError` panics while you work it out. That's the point. Note each one you hit and why it happened.

### Week 7: unsafe version

- Nodes allocated with `Box::into_raw`, linked with `NonNull<Node<K, V>>`, and freed in your own `Drop` impl.
- A `// SAFETY:` comment on every `unsafe` block.
- Run all tests under **Miri**: `cargo +nightly miri test -p p3-lru`. Miri catches undefined behaviour such as use-after-free and aliasing violations that normal tests miss.
- Tests use a value type that **counts drops**. That proves there are no leaks and no double frees.

### Both versions

- **One shared test suite** runs against both. Use a trait or a macro, reusing your P2 skills.
- Criterion benchmark: safe vs unsafe, for `get`-heavy and `put`-heavy workloads.

### Concepts

`Box`, `Rc`, `Weak`, `RefCell`, interior mutability, raw pointers, `NonNull`, `Drop`, safety invariants, Miri, `PhantomData`.

### Stretch

A third version using a `Vec` plus indices instead of pointers (an "arena"). It's safe, and often fast. Compare all three in your design note.

**Feeds the capstone:** the cache for region query results.

---

## 11. P4 — Concurrency (weeks 8–9)

**When:** 9 January and 16 January · **Time:** about 10–12 hours
**Reading:** *Rust Atomics and Locks* ch. 1 (before week 8), then ch. 2–4 (before week 9). It's free online.
**Allowed crates:** `rayon`, `criterion` (dev). For the stretch goal: `loom`.

### Part A: a thread-safe cache

1. Try to send your P3 safe cache to another thread. Read the error. Explain `Send` and `Sync` in your design note.
2. Wrap the cache in `Arc<Mutex<...>>`. Then build a `ShardedCache` with N shards, each its own `Mutex<LruCache>`, choosing the shard by hashing the key.
3. Try `RwLock` and notice it doesn't help. In an LRU cache, even `get` changes the recency order. Explain why in your design note.
4. Your unsafe cache needs `unsafe impl Send`. Justify it or reject it.
5. Benchmark with 1, 2, 4 and 8 threads using `std::thread::scope`.

### Part B: parallel tile processing

1. Split a large `GRD1` grid (from P1) into tiles. Work out per-tile stats using scoped threads and an `mpsc` channel to collect results.
2. Do the same with `rayon`'s `par_chunks`. Compare the code and the speed.

### Part C: a toy spin lock

1. Build a `SpinLock<T>` using `AtomicBool`, with `Acquire` on lock and `Release` on unlock. *Rust Atomics and Locks* ch. 4 walks through this.
2. Add a guard type so the lock is released on `Drop`.
3. In your design note, explain what could go wrong with `Relaxed` ordering, and why it matters more on your Mac if it's Apple Silicon.

### Concepts

`Arc`, `Mutex`, `RwLock`, `Send`/`Sync`, scoped threads, channels, atomics, memory ordering, data parallelism.

### Stretch

Model-check your spin lock with `loom`.

**Feeds the capstone:** the sharded cache shared across request handlers.

---

## 12. P5 — Async from the ground up (weeks 10–11)

**When:** 23 January and 30 January · **Time:** about 10–12 hours
**Reading:** Rust Book 17.5, "A Closer Look at the Traits for Async" (covers `Future`, `Pin` and `Unpin`). *Rust for Rustaceans*, the async chapter. Then the Tokio tutorial before part C.
**Allowed crates:** parts A–B use the standard library only. Part C uses `tokio` and `tokio-util`.

**The idea:** build the machinery yourself first, so that `async`/`await` and tokio stop being magic.

### Part A (week 10): a hand-written future and executor

1. A `Delay` future. On first poll, it starts a thread that sleeps and then calls `waker.wake()`. It returns `Poll::Pending` until the time is up.
2. A tiny single-threaded executor: a task queue (a channel), with tasks as `Arc<Task>` implementing `std::task::Wake`.
3. Run two delays at once and show that they overlap.

### Part B (week 10): combinators

1. Write `Join<A, B>` by hand. It polls two futures and finishes when both are done.
2. You'll hit `Pin` here. Do the pin projection with `pin-project-lite`, or by hand with `unsafe` (stretch). Explain in your design note why futures need pinning.

### Part C (week 11): a raw tokio server

A TCP server with a line-based protocol: send `GET r12c7`, get back that cell's value from an in-memory grid. Requirements:

- One task per connection
- A read timeout (`tokio::time::timeout`)
- A connection limit (`tokio::sync::Semaphore`)
- A **bounded** channel to a worker task, to show backpressure under load
- **Graceful shutdown** on Ctrl-C (`tokio::signal::ctrl_c` plus `tokio_util::sync::CancellationToken`): stop accepting new connections and let in-flight requests finish

**Why no axum yet:** building this by hand shows you exactly what axum and tower handle for you in the capstone.

### Concepts

`Future`, `Poll`, `Context`, `Waker`, `Pin`, executors, spawning, cancellation by drop, `select!`, timeouts, backpressure.

**Feeds the capstone:** the server layer, timeouts and shutdown.

---

## 13. Capstone — Grid risk query service (weeks 12–13)

**When:** 6 February and 13 February · **Time:** about 12 hours
**Allowed crates:** `axum` (0.8 or later; routes use the `/{param}` syntax), `tokio`, `tower-http`, `tracing`, `tracing-subscriber`, `serde`, `serde_json`, `memmap2`, `rand`, `criterion` (dev).

### The three crates

**`grid-core` (library):** the `GRD1` format and `GridView` (from P1), the aggregators (from P2), and the sharded cache (from P3/P4). Clean them up as you copy them in.

**`grid-gen` (binary):** writes a synthetic `GRD1` file.
- Suggested size: **16,384 × 16,384 cells × 3 scenario layers × 4 bytes ≈ 3.2 GB**. That's big enough that loading it all into memory is clumsy, so memory-mapping actually matters. Make sure you have about 5 GB of free disk space.
- Use a fixed random seed, so runs are repeatable. Make values vary smoothly across the grid rather than being pure noise, so neighbourhoods look realistic.

**`grid-server` (binary):** an axum API.

| Endpoint | Returns |
|---|---|
| `GET /health` | `ok` |
| `GET /cells/{row}/{col}?scenario=N` | That cell's value |
| `GET /cells/{row}/{col}/neighbourhood?scenario=N` | 3×3 stats (week 1's idea, now over the memory-mapped file) |
| `GET /region?row0=&col0=&row1=&col1=&scenario=N&agg=mean,max,p95` | Aggregates over a rectangle. Cache the results |

**Server requirements:**
- Open the file with `memmap2`. `Mmap::map` is an `unsafe` function, because the program has undefined behaviour if the file is changed while it's mapped. Write the `// SAFETY:` comment explaining what you assume.
- Share the data between handlers as `Arc` state.
- Region queries are CPU-heavy. Don't run them directly on the async runtime's threads. Use `tokio::task::spawn_blocking` (or rayon), and explain why in your design note.
- Request timeouts and request logging using `tower-http` layers and `tracing`.
- Graceful shutdown, as in P5.

### Week 12: make it work, then measure

1. Set up the workspace, `grid-core`, `grid-gen` and a basic server with the cell and neighbourhood endpoints.
2. **Baseline benchmarks:**
   - criterion for `grid-core` lookups
   - `oha` against the server, e.g. `oha -z 30s -c 64 http://localhost:3000/cells/100/200`
   - Record throughput (requests per second) and p50/p99 latency
3. **Set your own targets** from the baseline, for example "region queries p99 3× faster with the cache". Write them in the README before optimising.

### Week 13: make it fast, then explain

1. Add the region endpoint and the cache.
2. Profile with samply: `samply record ./target/release/grid-server`. Find the hottest function and improve it.
3. Re-run the load tests. Put a **before vs after** table in the README.
4. Write the final design note: architecture, the trade-offs you made, and what you'd change for production.

### Stretch goals (after 19 February, if you want)

- Compile `grid-core` to WebAssembly and call it from a small web page.
- Swap the synthetic data for a real public climate dataset.
- The P2 derive macro, if you skipped it.

---

## 14. Reading list

| Reading | Used in | Cost | Link |
|---|---|---|---|
| *The Rust Programming Language* (the Rust Book) | Week 1, P2, P5 | Free | https://doc.rust-lang.org/book/ |
| "Common Rust Lifetime Misconceptions" (pretzelhammer) | P1 | Free | https://github.com/pretzelhammer/rust-blog/blob/master/posts/common-rust-lifetime-misconceptions.md |
| *Rust for Rustaceans* (Jon Gjengset) | P1, P2, P3, P5 | Paid; Foundations chapter is a free sample | https://nostarch.com/rust-rustaceans |
| *Learning Rust With Entirely Too Many Linked Lists* | P3 | Free | https://rust-unofficial.github.io/too-many-lists/ |
| *Rust Atomics and Locks* (Mara Bos) | P4 | Free online (print copy is paid) | https://mara.nl/atomics/ |
| Tokio tutorial | P5 | Free | https://tokio.rs/tokio/tutorial |
| Miri | P3 | Free | https://github.com/rust-lang/miri |

**Direct chapter links:**
- Rust Book ch. 9 (Error Handling): https://doc.rust-lang.org/book/ch09-00-error-handling.html
- Rust Book ch. 13 (Iterators and Closures): https://doc.rust-lang.org/book/ch13-00-functional-features.html
- Rust Book 17.5 (Traits for Async): https://doc.rust-lang.org/book/ch17-05-traits-for-async.html
- Rust Book 20.5 (Macros): https://doc.rust-lang.org/book/ch20-05-macros.html
- *Too Many Lists*, "A Bad Safe Deque": https://rust-unofficial.github.io/too-many-lists/fourth.html
- *Too Many Lists*, "An Ok Unsafe Queue": https://rust-unofficial.github.io/too-many-lists/fifth.html
- *Rust Atomics and Locks* ch. 1: https://mara.nl/atomics/basics.html
- *Rust Atomics and Locks* ch. 4 (Spin Lock): https://mara.nl/atomics/building-spinlock.html

**Note on *Rust for Rustaceans* chapters:** this plan names chapters by topic (types/traits, interface design, unsafe, async) rather than by number. Two listings I found number and title them slightly differently, so go by topic.

**If you'd rather not buy *Rust for Rustaceans*:**
- P2: Rust Book ch. 10 (Generics, Traits) and 18 (trait objects)
- P3: the Rustonomicon (https://doc.rust-lang.org/nomicon/)
- P5: the Async Book (https://rust-lang.github.io/async-book/)

---

## 15. Sources

- Rust 1.85.0 release (2024 edition stabilised): https://blog.rust-lang.org/2025/02/20/Rust-1.85.0/
- axum 0.8.0 announcement (path syntax change to `/{param}`): https://tokio.rs/blog/2025-01-01-announcing-axum-0-8-0
- memmap2 `Mmap` docs (why `map` is `unsafe`): https://docs.rs/memmap2/latest/memmap2/struct.Mmap.html
- Miri README (install and `cargo miri test`): https://github.com/rust-lang/miri
- samply README (macOS, Linux, Windows): https://github.com/mstange/samply
- oha README (install, macOS via Homebrew): https://github.com/hatoo/oha
- *Rust Atomics and Locks* table of contents: https://mara.nl/atomics/
- *Rust for Rustaceans* (No Starch Press): https://nostarch.com/rust-rustaceans
