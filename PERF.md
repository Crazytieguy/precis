# Performance profiling results (2026-04-06)

## Setup

Profiling binary: `cargo run --release --bin profile -- <path> [--budget N]`

Perf fixtures cloned via `cargo run --bin clone_fixtures -- --perf`. Shallow tag clones stored in `test/perf-fixtures/`.

## Results

| Repo | Files | Symbols | Total | parse | groups | schedule | other |
|------|------:|--------:|------:|------:|-------:|---------:|------:|
| django | 6,679 | 50,094 | 92s | 38s (41%) | 42s (45%) | 9s (10%) | <2% |
| deno | 5,831 | 51,059 | 141s | 107s (76%) | 23s (16%) | 1s (1%) | <7% |
| cpython | 4,761 | 123,472 | 218s | 101s (46%) | 112s (52%) | 1s (0.5%) | <2% |
| vscode | 7,335 | 151,690 | 449s | 243s (54%) | 197s (44%) | 4s (1%) | <1% |
| typescript | 72,621 files | — | crash | — | — | — | — |

Walk, read, layout, render, and final token count are all negligible (<2% combined).

## Bottleneck 1: parse (tree-sitter)

`format::extract_all_symbols` calls `parse::extract_symbols` per file. Each call initializes a tree-sitter parser, parses the source into an AST, and runs a query. This is pure CPU work, embarrassingly parallel across files.

## Bottleneck 2: groups (BPE tokenization)

`schedule::build_groups` calls `compute_symbol_costs` per symbol. That function calls `format::count_tokens` (tiktoken-rs BPE encoding) for:
- The name line (1 call)
- Each signature line (1 call per line)
- Each doc line (1 call per line, via `count_line_range`)
- Each body line (1 call per line, via `count_line_range`)
- Each truncation marker (1 call per line, via `truncation_marker_cost`)

That's 2 `count_tokens` calls per source line across all symbols. For cpython (123k symbols), this is hundreds of thousands of BPE encodings. Also embarrassingly parallel, or could be optimized by estimating token counts instead of exact encoding.

## Bug: tiktoken-rs stack overflow on TypeScript repo

tiktoken-rs panics with `StackOverflow` during the groups stage on the TypeScript fixture (72k files). Likely triggered by an extremely long generated line. The profiler crashes before producing any output.

## Schedule stage

The priority queue algorithm in `schedule::schedule` is relatively cheap even at 150k+ symbols (1-9s). Django is the outlier at 9s with 3,924 groups; the others are ~1s. Not a priority for optimization.
