# Performance profiling results (2026-04-12)

## Setup

Profiling binary: `cargo run --release --bin profile -- <path> [--budget N]`

Perf fixtures cloned via `cargo run --bin clone_fixtures -- --perf`. Shallow
tag clones stored in `test/perf-fixtures/`.

Numbers below are warm-cache, release mode, Apple Silicon.

## Results

| Repo       |   Files | Budget 4k | Budget 10k |
|------------|--------:|----------:|-----------:|
| django     |   6,679 |     340ms |      596ms |
| deno       |   5,831 |     504ms |      692ms |
| cpython    |   4,761 |       1.8s |        2.0s |
| vscode     |   7,335 |     624ms |      912ms |
| typescript |  72,621 |     512ms |        2.6s |

All outputs come in within ±3% of budget. TypeScript no longer crashes — the
old tiktoken stack-overflow was triggered by eagerly tokenizing every line
of a 40KB emoji JSON; the new architecture never reaches that file.

## Architectural context

The rewrite replaced the old eager pipeline (walk → parse every file →
build groups → tokenize every line → schedule) with a lazy greedy
scheduler. Files are parsed on demand inside `Files::children()`, only
when their parent group has been selected by the scheduler. For a 4k-token
budget, the vast majority of files in a large repo never get parsed — the
scheduler commits the biggest-ratio groups and stops when nothing else
fits.

The old per-stage bottlenecks (tree-sitter parsing, tiktoken tokenization,
group construction) still exist, but each one only runs over the much
smaller set of files that actually contribute to output.

vs. numbers from the pre-rewrite era (captured 2026-04-06, after rayon
parallelism + composite-symbol fixes, on the same fixtures at the same
4k budget):

| Repo       | Pre-rewrite | Post-rewrite | Speedup |
|------------|------------:|-------------:|--------:|
| django     |        7.6s |        340ms |    22x  |
| deno       |        4.2s |        504ms |     8x  |
| cpython    |        5.1s |         1.8s |     3x  |
| vscode     |       27.1s |        624ms |    43x  |
| typescript |     crashed |        512ms |       — |

## Remaining hot spots

cpython is the outlier (1.8s at 4k, 2.0s at 10k). Its flatness between
budgets suggests the scheduler spends most of its time exploring many
small, similarly-valued groups before converging — not parsing. A
profiler run would confirm whether it's cost probing or frontier
management.

TypeScript jumps the most between budgets (512ms → 2.6s, ~5x). Larger
budgets pull more files through `Files::children()` and therefore more
through tree-sitter parsing. At 72k files, parse cost per committed file
dominates.

django, deno, vscode all scale sub-linearly with budget and are well
below any interactive-usage concern.

## Deferred optimization ideas

Nothing is urgent. Documented here so the list doesn't get lost.

- **Cost-probe caching.** `schedule::probe_cost` probes each candidate's
  marginal cost against the current committed set. For small candidates
  over a large committed set, this is O(committed_files). A per-candidate
  cache keyed on `len(committed)` would help, but only on
  large-frontier workloads.
- **Parallelism.** The old code used rayon across files. The lazy
  scheduler intentionally serializes parsing to keep the cost model
  honest (each probe observes the real rendered cost). Re-introducing
  parallelism would require batched speculative parsing, which trades
  complexity for maybe 2-3x on parse-bound workloads.
- **Incremental tokenization.** Each committed group's tokens are cached
  per-file in `CachedGroupRender`. Probes re-run tokenization on any
  file the candidate touches. For small candidates this is cheap.
  Skipping re-tokenization of unchanged entries is possible but
  significant plumbing.
