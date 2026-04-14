# Performance profiling results (2026-04-12)

## Setup

Profiling binary: `cargo run --release --bin profile -- <path> [--budget N]`

Benchmark suite: `cargo bench-hot` (`cargo bench --bench hot_path -- --quick`)

Perf fixtures cloned via `cargo run --bin clone_fixtures -- --perf`. Shallow
tag clones stored in `test/perf-fixtures/`.

The hot-path benchmark covers regular snapshot fixtures plus the large
perf fixtures when present. Missing perf fixtures are skipped so local
iteration still works before cloning the full benchmark set.

Numbers below are warm-cache, release mode, Apple Silicon.

## Results

| Repo       |   Files | Budget 4k | Budget 10k |
|------------|--------:|----------:|-----------:|
| django     |   6,679 |     340ms |      596ms |
| deno       |   5,831 |     504ms |      692ms |
| cpython    |   4,761 |       1.8s |        2.0s |
| vscode     |   7,335 |     624ms |      912ms |
| typescript |  72,621 |     512ms |        2.6s |

All outputs come in within ±3% of budget. TypeScript no longer crashes
(the old tiktoken stack-overflow on a 40KB emoji JSON line is gone).

## vs. pre-rewrite

Captured 2026-04-06 on the same fixtures at the same 4k budget, after
rayon parallelism and composite-symbol fixes:

| Repo       | Pre-rewrite | Post-rewrite | Speedup |
|------------|------------:|-------------:|--------:|
| django     |        7.6s |        340ms |    22x  |
| deno       |        4.2s |        504ms |     8x  |
| cpython    |        5.1s |         1.8s |     3x  |
| vscode     |       27.1s |        624ms |    43x  |
| typescript |     crashed |        512ms |       — |

The rewrite replaced the old eager pipeline (walk → parse every file →
tokenize every line → schedule) with a lazy greedy scheduler. Files are
parsed on demand inside `Files::children()`, only when their parent
group has been selected — so most files in a large repo never get
parsed at small budgets.

## Scaling notes

- cpython is the outlier: 1.8s → 2.0s between 4k and 10k (almost flat).
- typescript is the most budget-sensitive: 512ms → 2.6s (~5x) between
  4k and 10k.
- django, deno, vscode all scale sub-linearly with budget.

No breakdown by stage yet — `profile` reports wall-clock only.
