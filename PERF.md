# Performance profiling results (2026-04-06)

## Setup

Profiling binary: `cargo run --release --bin profile -- <path> [--budget N]`

Perf fixtures cloned via `cargo run --bin clone_fixtures -- --perf`. Shallow tag clones stored in `test/perf-fixtures/`.

## Results (before optimization)

| Repo | Files | Symbols | Total | parse | groups | schedule | other |
|------|------:|--------:|------:|------:|-------:|---------:|------:|
| django | 6,679 | 50,094 | 92s | 38s (41%) | 42s (45%) | 9s (10%) | <2% |
| deno | 5,831 | 51,059 | 141s | 107s (76%) | 23s (16%) | 1s (1%) | <7% |
| cpython | 4,761 | 123,472 | 218s | 101s (46%) | 112s (52%) | 1s (0.5%) | <2% |
| vscode | 7,335 | 151,690 | 449s | 243s (54%) | 197s (44%) | 4s (1%) | <1% |
| typescript | 72,621 files | — | crash | — | — | — | — |

Walk, read, layout, render, and final token count are all negligible (<2% combined).

## Results (after budget-aware truncation)

| Repo | Total | parse | groups | schedule | other |
|------|------:|------:|-------:|---------:|------:|
| django | 21s | 9s (43%) | 5.5s (26%) | 5.6s (26%) | <5% |
| deno | 42s | 37s (88%) | 3.5s (8%) | 0.4s (1%) | <3% |
| cpython | 37s | 30s (82%) | 5.3s (14%) | 0.4s (1%) | <3% |
| vscode | 122s | 85s (70%) | 33s (27%) | 2s (2%) | <1% |

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

## Optimization attempt: cost estimation (2026-04-06)

We tried replacing exact BPE token counting in `build_groups` with calibrated linear models (`tokens ≈ a * bytes + b` per language). This eliminated the tokenization bottleneck but introduced ~7-13% output divergence from token-based scheduling, because bytes-to-tokens ratios vary by content type and the estimation errors compound at budget boundaries.

## Budget-aware truncation (2026-04-06)

Compute exact token costs in `build_groups` but stop per-group once cumulative cost exceeds the budget. Doc/body lines are computed layer-by-layer; layers whose prerequisites already exceed the budget are skipped. Output is exact (all snapshot tests unchanged). Groups stage speedup: 6-21x across repos.

Parse is now the dominant bottleneck (43-88%). Both parse and groups are embarrassingly parallel.

## Query caching (2026-04-06)

`extract_symbols` was compiling the same tree-sitter `Query` from `.scm` text for every file. With ~7,000 files across ~12 distinct language/query pairs, that's thousands of redundant compilations. Fix: `build_language_configs` compiles each `Query` once per unique extension, and `extract_all_symbols_cached` reuses the pre-compiled configs.

| Repo | Total | parse:init | parse | groups | schedule | other |
|------|------:|----------:|------:|-------:|---------:|------:|
| django | 17s | 63ms | 3.3s (19%) | 7.5s (43%) | 5.0s (29%) | <9% |
| deno | 13s | 214ms | 6.5s (50%) | 4.5s (35%) | 0.4s (3%) | <12% |
| cpython | 20s | 40ms | 12.5s (63%) | 5.7s (29%) | 0.7s (4%) | <4% |
| vscode | 46s | 86ms | 12.7s (28%) | 30.4s (66%) | 1.7s (4%) | <2% |

Parse speedup: 2.4-6.6x. Query compilation (parse:init) is now negligible (<0.5%).

Parse and groups remain the dominant bottlenecks. Both are embarrassingly parallel.