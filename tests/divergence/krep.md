scores: Sim=0.403 Reached=19/39 Early=4 Late=12 Partial=6 Missing=14 Used=9912/10000

## Tier rollup

| tier | batches | reached | partial | missing | avg_credit |
|-----:|--------:|--------:|--------:|--------:|-----------:|
| 1 | 8 | 8 | 0 | 0 | 1.00 |
| 2 | 10 | 5 | 2 | 3 | 0.66 |
| 3 | 5 | 1 | 1 | 3 | 0.33 |
| 4 | 3 | 0 | 0 | 3 | 0.00 |
| 5 | 8 | 5 | 1 | 2 | 0.66 |
| 6 | 2 | 0 | 2 | 0 | 0.63 |
| 7 | 3 | 0 | 0 | 3 | 0.00 |

## Arrival ledger (non-aligned or partial-credit NS batches)

| id | exp_t | reached_t | delta_t | credit | status | descriptor | nearby walker batch |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.3 | 226 | 5201 | +4975 | 1.00 | late | Usage line shapes | README.md section #8 (t=5201, 9 atoms) |
| 1.4 | 343 | 1014 | +671 | 1.00 | late | README scope disclaimer | README.md section #0 (t=1014, 4 atoms) |
| 1.5 | 519 | 9831 | +9312 | 1.00 | late | Command-line options reference (search/scope flags) | README.md section #10 (t=9831, 8 atoms) |
| 1.6 | 769 | 9831 | +9062 | 1.00 | late | Command-line options reference (recursive/threading/UI flags) | README.md section #10 (t=9831, 11 atoms) |
| 1.7 | 959 | 8276 | +7317 | 1.00 | late | README key-features bullets (algorithms + SIMD + I/O) | README.md section #2 (t=8276, 8 atoms) |
| 1.8 | 1131 | 8276 | +7145 | 1.00 | late | README key-features bullets (recursive/UX/limits) | README.md section #2 (t=8276, 7 atoms) |
| 2.1 | 1329 | — | — | 0.71 | partial | krep.h header banner + includes | c includes in krep.h (t=1779, 7 atoms) |
| 2.2 | 1514 | 3261 | +1747 | 0.93 | late | match_position_t and match_result_t | c decl at krep.h:55 (t=3261, 6 atoms) |
| 2.3 | 1605 | 3182 | +1577 | 1.00 | late | search_func_t function pointer typedef | c decl at krep.h:98 (t=2888, 4 atoms) |
| 2.4 | 2024 | — | — | 0.76 | partial | search_params_t struct (the central state) | c decl at krep.h:65 (t=4948, 24 atoms) |
| 2.5 | 2124 | 2839 | +715 | 1.00 | late | Helper API + select_search_algorithm + get_algorithm_name | c decl names surface in krep.h (t=2839, 4 atoms) |
| 2.6 | 2382 | — | — | 0.25 | missing | Internal search-algorithm declarations — names + ISA gates | c decl names surface in krep.h (t=2839, 6 atoms) |
| 2.7 | 2786 | 4174 | +1388 | 0.94 | late | Public API: search_file / search_string / search_directory_recursive | c decl doc at krep.h:180 (t=3893, 8 atoms) |
| 2.8 | 3120 | 4601 | +1481 | 1.00 | late | match_result_* lifecycle + print_matching_items | c decl doc at krep.h:200 (t=4601, 12 atoms) |
| 2.9 | 3287 | — | — | 0.00 | missing | Recursive-search skip lists (directories) |  |
| 2.10 | 3743 | — | — | 0.00 | missing | Recursive-search skip lists (file extensions) |  |
| 3.1 | 3761 | 6527 | +2766 | 1.00 | late | select_search_algorithm signature | c decl names surface in krep.c (t=6527, 1 atoms) |
| 3.2 | 4164 | — | — | 0.03 | missing | select_search_algorithm body — regex/AC/override branches | c decl at krep.c:1771 (t=6592, 2 atoms) |
| 3.3 | 5061 | — | — | 0.00 | missing | select_search_algorithm body — single-pattern dispatch tail |  |
| 3.4 | 5485 | — | — | 0.06 | missing | get_algorithm_name body — algorithm enumeration | c decl at krep.c:1964 (t=6604, 3 atoms) |
| 3.5 | 5604 | — | — | 0.56 | partial | Search algorithm signatures in krep.c — locations | c decl names surface in krep.c (t=6527, 33 atoms) |
| 4.1 | 5618 | — | — | 0.00 | missing | main() signature |  |
| 4.2 | 6016 | — | — | 0.00 | missing | main() — long_options table + getopt_long invocation |  |
| 4.3 | 6124 | — | — | 0.00 | missing | Param finalization — count/track-positions wiring |  |
| 5.1 | 6621 | 585 | -6036 | 0.85 | early | aho_corasick.h — public AC API | c decl names surface in aho_corasick.h (t=394, 19 atoms) |
| 5.2 | 6820 | 5072 | -1748 | 0.93 | aligned | ac_node_t + ac_trie struct definitions | c decl at aho_corasick.c:17 (t=5072, 8 atoms) |
| 5.3 | 7170 | 3499 | -3671 | 0.92 | early | thread_pool_t + task_t structures | c decl at krep.h:132 (t=3499, 12 atoms) |
| 5.4 | 7450 | 4418 | -3032 | 0.85 | early | thread_data_t — what each chunk task carries | c decl at krep.h:104 (t=4418, 16 atoms) |
| 5.5 | 7712 | 3954 | -3758 | 0.87 | early | Whole-word match inline helpers | c decl doc at krep.h:312 (t=3795, 9 atoms) |
| 5.6 | 7944 | — | — | 0.79 | partial | krep.c — key constants (chunk size / version / paths) | c decl names surface in krep.c (t=6527, 11 atoms) |
| 5.7 | 8345 | — | — | 0.00 | missing | krep.c — SIMD intrinsic include gating |  |
| 5.8 | 8912 | — | — | 0.05 | missing | search_chunk_thread — what each worker actually does | c decl at krep.c:1919 (t=6598, 3 atoms) |
| 6.1 | 9197 | — | — | 0.69 | partial | krep.c top-level function names — locations | c decl names surface in krep.c (t=6527, 53 atoms) |
| 6.2 | 9298 | — | — | 0.57 | partial | aho_corasick.c function names — locations | c decl names surface in aho_corasick.c (t=1312, 12 atoms) |
| 7.1 | 9340 | — | — | 0.00 | missing | test/ FS listing |  |
| 7.2 | 9620 | — | — | 0.00 | missing | Makefile — sources and main build target |  |
| 7.3 | 9938 | — | — | 0.00 | missing | Makefile — arch/SIMD flag detection |  |

## Walker waste rollup (by descriptor pattern)

| n | off_tokens_total | pattern |
|--:|-----------------:|:--------|
| 6 | 815 | README.md section #<n> |

## Walker waste (off-NS token spend ≥ 50)

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 340 | 0.33 | 1025 | 2839 | c decl names surface in krep.h |
| 333 | 1.00 | 333 | 9064 | c includes in krep.c |
| 259 | 1.00 | 259 | 8731 | README.md section #11 |
| 258 | 0.92 | 279 | 864 | headings outline in README.md |
| 239 | 1.00 | 239 | 7927 | README.md section #9 |
| 206 | 0.22 | 929 | 6527 | c decl names surface in krep.c |
| 156 | 1.00 | 156 | 9358 | c decl body at krep.c:401 |
| 138 | 1.00 | 138 | 9202 | c decl body at krep.c:4313 |
| 114 | 1.00 | 114 | 1639 | README.md section #1 |
| 114 | 1.00 | 114 | 5466 | c includes in aho_corasick.c |
| 104 | 1.00 | 104 | 4058 | c decl doc at krep.h:288 |
| 100 | 1.00 | 100 | 8423 | c decl body at krep.c:1125 |
| 97 | 1.00 | 97 | 3688 | c decl doc at krep.h:278 |
| 87 | 1.00 | 87 | 7688 | c decl body at krep.c:1198 |
| 81 | 1.00 | 81 | 9912 | README.md section #17 |
| 77 | 0.44 | 175 | 1312 | c decl names surface in aho_corasick.c |
| 77 | 1.00 | 77 | 5352 | c header banner in aho_corasick.c |
| 74 | 1.00 | 74 | 5275 | c header banner in krep.c |
| 67 | 1.00 | 67 | 7324 | c decl at krep.c:3154 |
| 63 | 1.00 | 63 | 5529 | c decl body at aho_corasick.c:274 |
| 63 | 0.93 | 68 | 3329 | c decl doc at krep.h:298 |
| 61 | 1.00 | 61 | 9419 | README.md section #15 |
| 61 | 1.00 | 61 | 1137 | README.md section #3 |
| 56 | 1.00 | 56 | 7451 | c decl doc at krep.c:1628 |
| 52 | 0.73 | 71 | 7522 | c decl at krep.c:1259 |
| 50 | 1.00 | 50 | 7257 | c decl at krep.c:1628 |
