# rust-deep-dive

A 13-week, project-based plan to learn Rust properly (7 November 2026 – 19 February 2027).
The full plan is in [`docs/PLAN.md`](docs/PLAN.md).

## Layout

This repo is one Cargo **workspace**. Each project is its own crate, added when you reach it.

| Folder | Project | Weeks |
|---|---|---|
| `week01-grid-stats/` | Refresher: grid stats CLI | 1 |
| `p1-grid-parser/` | Zero-copy grid parser | 2–3 |
| `p2-aggregators/` | Pluggable aggregators | 4–5 |
| `p3-lru/` | LRU cache, built twice | 6–7 |
| `p4-concurrency/` | Concurrency | 8–9 |
| `p5-async/` | Async from the ground up | 10–11 |
| `capstone/` | Grid risk query service | 12–13 |

## Everyday commands

```sh
cargo test -p grid-stats                      # run one project's tests
cargo test                                    # run everything
cargo clippy --all-targets -- -D warnings     # lint (must be clean when done)
cargo fmt                                     # format
cargo run -p grid-stats -- week01-grid-stats/tests/fixtures/small.csv
```

## The routine (short version)

1. Read the project's paired reading first.
2. Read the project's `README.md` spec. Run the tests: they fail.
3. Make them pass **without changing the provided tests**. Add your own tests too.
4. Stuck for 30+ minutes? Ask Claude for a hint: *"Week 1 hint 1: …"* (paste the error).
5. Run the done checklist, write the design note, push, then ask Claude: *"Review week 1"*.
6. Log your hours in [`LOG.md`](LOG.md).
