scores: Sim=0.438 Reached=9/39 Early=1 Late=6 Partial=0 Missing=30 Used=3265/10000

## Tier rollup

| tier | batches | reached | partial | missing | avg_credit |
|-----:|--------:|--------:|--------:|--------:|-----------:|
| 1 | 8 | 8 | 0 | 0 | 1.00 |
| 2 | 10 | 0 | 0 | 10 | 0.00 |
| 3 | 5 | 0 | 0 | 5 | 0.00 |
| 4 | 3 | 0 | 0 | 3 | 0.00 |
| 5 | 8 | 0 | 0 | 8 | 0.00 |
| 6 | 2 | 0 | 0 | 2 | 0.00 |
| 7 | 3 | 1 | 0 | 2 | 0.33 |

## Arrival ledger (non-aligned or partial-credit NS batches)

| id | exp_t | reached_t | delta_t | credit | status | descriptor | nearby walker batch |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.3 | 226 | 948 | +722 | 1.00 | late | Usage line shapes | README.md section #8 (t=948, 9 atoms) |
| 1.4 | 343 | 547 | +204 | 1.00 | late | README scope disclaimer | README.md section #0 (t=547, 4 atoms) |
| 1.5 | 519 | 2433 | +1914 | 1.00 | late | Command-line options reference (search/scope flags) | README.md section #10 (t=2433, 8 atoms) |
| 1.6 | 769 | 2433 | +1664 | 1.00 | late | Command-line options reference (recursive/threading/UI flags) | README.md section #10 (t=2433, 11 atoms) |
| 1.7 | 959 | 1605 | +646 | 1.00 | late | README key-features bullets (algorithms + SIMD + I/O) | README.md section #2 (t=1605, 8 atoms) |
| 1.8 | 1131 | 1605 | +474 | 1.00 | late | README key-features bullets (recursive/UX/limits) | README.md section #2 (t=1605, 7 atoms) |
| 2.1 | 1329 | — | — | 0.00 | missing | krep.h header banner + includes |  |
| 2.2 | 1514 | — | — | 0.00 | missing | match_position_t and match_result_t |  |
| 2.3 | 1605 | — | — | 0.00 | missing | search_func_t function pointer typedef |  |
| 2.4 | 2024 | — | — | 0.00 | missing | search_params_t struct (the central state) |  |
| 2.5 | 2124 | — | — | 0.00 | missing | Helper API + select_search_algorithm + get_algorithm_name |  |
| 2.6 | 2382 | — | — | 0.00 | missing | Internal search-algorithm declarations — names + ISA gates |  |
| 2.7 | 2786 | — | — | 0.00 | missing | Public API: search_file / search_string / search_directory_recursive |  |
| 2.8 | 3120 | — | — | 0.00 | missing | match_result_* lifecycle + print_matching_items |  |
| 2.9 | 3287 | — | — | 0.00 | missing | Recursive-search skip lists (directories) |  |
| 2.10 | 3743 | — | — | 0.00 | missing | Recursive-search skip lists (file extensions) |  |
| 3.1 | 3761 | — | — | 0.00 | missing | select_search_algorithm signature |  |
| 3.2 | 4164 | — | — | 0.00 | missing | select_search_algorithm body — regex/AC/override branches |  |
| 3.3 | 5061 | — | — | 0.00 | missing | select_search_algorithm body — single-pattern dispatch tail |  |
| 3.4 | 5485 | — | — | 0.00 | missing | get_algorithm_name body — algorithm enumeration |  |
| 3.5 | 5604 | — | — | 0.00 | missing | Search algorithm signatures in krep.c — locations |  |
| 4.1 | 5618 | — | — | 0.00 | missing | main() signature |  |
| 4.2 | 6016 | — | — | 0.00 | missing | main() — long_options table + getopt_long invocation |  |
| 4.3 | 6124 | — | — | 0.00 | missing | Param finalization — count/track-positions wiring |  |
| 5.1 | 6621 | — | — | 0.00 | missing | aho_corasick.h — public AC API |  |
| 5.2 | 6820 | — | — | 0.00 | missing | ac_node_t + ac_trie struct definitions |  |
| 5.3 | 7170 | — | — | 0.00 | missing | thread_pool_t + task_t structures |  |
| 5.4 | 7450 | — | — | 0.00 | missing | thread_data_t — what each chunk task carries |  |
| 5.5 | 7712 | — | — | 0.00 | missing | Whole-word match inline helpers |  |
| 5.6 | 7944 | — | — | 0.00 | missing | krep.c — key constants (chunk size / version / paths) |  |
| 5.7 | 8345 | — | — | 0.00 | missing | krep.c — SIMD intrinsic include gating |  |
| 5.8 | 8912 | — | — | 0.00 | missing | search_chunk_thread — what each worker actually does |  |
| 6.1 | 9197 | — | — | 0.00 | missing | krep.c top-level function names — locations |  |
| 6.2 | 9298 | — | — | 0.00 | missing | aho_corasick.c function names — locations |  |
| 7.1 | 9340 | 2747 | -6593 | 1.00 | early | test/ FS listing |  |
| 7.2 | 9620 | — | — | 0.00 | missing | Makefile — sources and main build target |  |
| 7.3 | 9938 | — | — | 0.00 | missing | Makefile — arch/SIMD flag detection |  |

## Walker waste rollup (by descriptor pattern)

| n | off_tokens_total | pattern |
|--:|-----------------:|:--------|
| 9 | 1187 | README.md section #<n> |

## Walker waste (off-NS token spend ≥ 50)

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 337 | 1.00 | 337 | 3265 | plaintext config LICENSE |
| 259 | 1.00 | 259 | 1960 | README.md section #11 |
| 258 | 0.92 | 279 | 397 | headings outline in README.md |
| 239 | 1.00 | 239 | 1256 | README.md section #9 |
| 181 | 1.00 | 181 | 2928 | README.md section #13 |
| 114 | 1.00 | 114 | 784 | README.md section #1 |
| 107 | 1.00 | 107 | 2621 | README.md section #5 |
| 84 | 1.00 | 84 | 2705 | README.md section #14 |
| 81 | 1.00 | 81 | 2514 | README.md section #17 |
| 61 | 1.00 | 61 | 2021 | README.md section #15 |
| 61 | 1.00 | 61 | 670 | README.md section #3 |
