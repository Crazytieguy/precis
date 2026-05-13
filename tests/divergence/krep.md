scores: Score(3000)=0.437 ns_rows≤3K=15/39 (reached=5 partial=1 missing=9)

## Per-budget scores

| B | A_B | I(B) | C(B) | compl(B) | Score(B) | walker_used |
|--:|----:|-----:|-----:|---------:|---------:|------------:|
| 1000 | 54 | 0.700 | 0.225 | 0.518 | 0.397 | 897 |
| 1442 | 78 | 0.720 | 0.338 | 0.614 | 0.493 | 1437 |
| 2080 | 131 | 0.669 | 0.228 | 0.665 | 0.391 | 2049 |
| 3000 | 187 | 0.698 | 0.273 | 0.881 | 0.437 | 2438 |
| 4327 | 270 | 0.717 | 0.321 | 0.746 | 0.480 | 4312 |
| 6240 | 407 | 0.887 | 0.421 | 0.941 | 0.611 | 6184 |
| 9000 | 616 | 0.842 | 0.476 | 0.836 | 0.633 | 8955 |

## Top opportunities

### Additive (close partial / missing rows)

| intervention | rows | gap@1k | gap@3k | gap@9k | evidence | top row ids |
|:-------------|-----:|-------:|-------:|-------:|:---------|:------------|
| split wrong-slice walker batches | 22 | 2.23 | 1.86 | 0.64 | nearby candidates have low exact atom overlap | 2.4, 1.5, 2.7, 2.2, 1.6, ... |
| tune ranking for high-overlap unscheduled candidates | 4 | 0.51 | 0.51 | 0.51 | high-overlap candidates not in the schedule by T_max, exact total=151/173 | 3.3, 3.2, 3.4, 5.8 |
| add walker candidates for no-discovered rows | 2 | 0.06 | 0.06 | 0.06 | NS rows have no discovered line candidate | 7.2, 7.3 |

### Subtractive (suppress consistently off-NS batches)

| pattern | batches | freed@1k | freed@3k | freed@9k | evidence | top batch ids |
|:--------|--------:|---------:|---------:|---------:|:---------|:--------------|
| c decl names surface in krep.h | 1 | 0 | 340 | 340 | off_3k=621 | c decl names surface in krep.h |
| headings outline in README.md | 1 | 258 | 258 | 258 | off_3k=258 | headings outline in README.md |
| README.md section #<n> | 2 | 0 | 175 | 673 | off_3k=175 | README.md section #1, README.md section #16 |
| c decl names surface in aho_corasick.c | 1 | 0 | 77 | 77 | off_3k=175 | c decl names surface in aho_corasick.c |

Top missed paths (NS rows ≤ 3K): krep.h (7 rows, 126 atoms), README.md (3 rows, 29 atoms)

## Arrival ledger by diagnosis

### ranking-recoverable

| id | ns_t | credit | status | descriptor | candidate hint |
|----|-----:|-------:|:-------|:-----------|:---------------|
| 3.2 | 4164 | 0.00 | missing | select_search_algorithm body — regex/AC/override branches | [scheduled bbox exact=2/30] c decl at krep.c:1771 (t=7424, 2 atoms); better unscheduled exact=24/30: c decl body at krep.c:1771 (24 atoms, too expensive at final margin) |
| 3.3 | 5061 | 0.00 | missing | select_search_algorithm body — single-pattern dispatch tail | [unscheduled bbox exact=64/69] c decl body at krep.c:1771 (64 atoms, too expensive at final margin) |
| 3.4 | 5485 | 0.00 | missing | get_algorithm_name body — algorithm enumeration | [scheduled bbox exact=3/33] c decl at krep.c:1964 (t=7436, 3 atoms); better unscheduled exact=30/33: c decl body at krep.c:1964 (30 atoms, too expensive at final margin) |
| 5.8 | 8912 | 0.00 | missing | search_chunk_thread — what each worker actually does | [scheduled bbox exact=3/41] c decl at krep.c:1919 (t=7430, 3 atoms); better unscheduled exact=33/41: c decl body at krep.c:1919 (33 atoms, too expensive at final margin) |

### wrong-slice / granularity

| id | ns_t | credit | status | descriptor | candidate hint |
|----|-----:|-------:|:-------|:-----------|:---------------|
| 1.5 | 519 | 0.67 | missing | Command-line options reference (search/scope flags) | [scheduled bbox exact=2/9] headings outline in README.md (t=864, 2 atoms) |
| 1.6 | 769 | 0.73 | missing | Command-line options reference (recursive/threading/UI flags) | [scheduled bbox exact=1/11] README.md section #33 (t=6107, 1 atoms) |
| 2.1 | 1329 | 0.71 | partial | krep.h header banner + includes | [scheduled bbox exact=7/17] c includes in krep.h (t=2163, 7 atoms) |
| 2.2 | 1514 | 0.00 | missing | match_position_t and match_result_t | [scheduled bbox exact=6/14] c decl at krep.h:55 (t=3885, 6 atoms) |
| 2.3 | 1605 | 0.00 | missing | search_func_t function pointer typedef | [scheduled bbox exact=4/6] c decl at krep.h:98 (t=3512, 4 atoms) |
| 2.4 | 2024 | 0.00 | missing | search_params_t struct (the central state) | [scheduled bbox exact=24/33] c decl at krep.h:65 (t=5680, 24 atoms) |
| 2.6 | 2382 | 0.00 | missing | Internal search-algorithm declarations — names + ISA gates | [scheduled bbox exact=5/20] c decl names surface in krep.h (t=3463, 6 atoms) |
| 2.7 | 2786 | 0.00 | missing | Public API: search_file / search_string / search_directory_recursive | [scheduled bbox exact=8/31] c decl doc at krep.h:161 (t=4798, 8 atoms) |
| 2.8 | 3120 | 0.00 | missing | match_result_* lifecycle + print_matching_items | [scheduled bbox exact=12/22] c decl doc at krep.h:200 (t=5225, 12 atoms) |
| 2.9 | 3287 | 0.00 | missing | Recursive-search skip lists (directories) | [scheduled same-file] c decl names surface in krep.h (t=3463, 70 atoms) |
| 2.10 | 3743 | 0.00 | missing | Recursive-search skip lists (file extensions) | [scheduled same-file] c decl names surface in krep.h (t=3463, 70 atoms) |
| 3.5 | 5604 | 0.00 | missing | Search algorithm signatures in krep.c — locations | [scheduled bbox exact=5/9] c decl names surface in krep.c (t=7359, 33 atoms) |
| 4.1 | 5618 | 0.00 | missing | main() signature | [scheduled same-file] c decl names surface in krep.c (t=7359, 77 atoms) |
| 4.2 | 6016 | 0.00 | missing | main() — long_options table + getopt_long invocation | [scheduled same-file] c decl names surface in krep.c (t=7359, 77 atoms) |
| 4.3 | 6124 | 0.00 | missing | Param finalization — count/track-positions wiring | [scheduled same-file] c decl names surface in krep.c (t=7359, 77 atoms) |
| 5.2 | 6820 | 0.53 | missing | ac_node_t + ac_trie struct definitions | [scheduled bbox exact=8/15] c decl at aho_corasick.c:17 (t=5864, 8 atoms) |
| 5.3 | 7170 | 0.00 | missing | thread_pool_t + task_t structures | [scheduled bbox exact=12/26] c decl at krep.h:132 (t=4123, 12 atoms) |
| 5.5 | 7712 | 0.00 | missing | Whole-word match inline helpers | [scheduled bbox exact=9/23] c decl doc at krep.h:312 (t=4419, 9 atoms) |
| 5.6 | 7944 | 0.00 | missing | krep.c — key constants (chunk size / version / paths) | [scheduled bbox exact=11/14] c decl names surface in krep.c (t=7359, 11 atoms) |
| 5.7 | 8345 | 0.00 | missing | krep.c — SIMD intrinsic include gating | [scheduled same-file] c decl names surface in krep.c (t=7359, 77 atoms) |
| 6.1 | 9197 | 0.00 | missing | krep.c top-level function names — locations | [scheduled bbox exact=18/26] c decl names surface in krep.c (t=7359, 53 atoms) |
| 6.2 | 9298 | 0.57 | missing | aho_corasick.c function names — locations | [scheduled bbox exact=4/7] c decl names surface in aho_corasick.c (t=1657, 12 atoms) |

### no discovered candidate

| id | ns_t | credit | status | descriptor | candidate hint |
|----|-----:|-------:|:-------|:-----------|:---------------|
| 7.2 | 9620 | 0.00 | missing | Makefile — sources and main build target | no discovered line candidate |
| 7.3 | 9938 | 0.00 | missing | Makefile — arch/SIMD flag detection | no discovered line candidate |

### mixed/unknown

| id | ns_t | credit | status | descriptor | candidate hint |
|----|-----:|-------:|:-------|:-----------|:---------------|
| 1.3 | 226 | 0.00 | missing | Usage line shapes | [scheduled bbox exact=9/9] README.md section #21 (t=5993, 9 atoms) |
| 2.5 | 2124 | 0.00 | missing | Helper API + select_search_algorithm + get_algorithm_name | [scheduled bbox exact=4/5] c decl names surface in krep.h (t=3463, 4 atoms) |
| 3.1 | 3761 | 0.00 | missing | select_search_algorithm signature | [scheduled bbox exact=1/1] c decl at krep.c:1771 (t=7424, 1 atoms) |
| 5.4 | 7450 | 0.00 | missing | thread_data_t — what each chunk task carries | [scheduled bbox exact=16/20] c decl at krep.h:104 (t=5042, 16 atoms) |

Top wasted paths (off-NS at 3K): krep.h (621t, 1 batch), README.md (433t, 3 batches), aho_corasick.c (236t, 2 batches), aho_corasick.h (182t, 1 batch)

## Walker waste rollup (by descriptor pattern)

| n | off_3k_total | pattern |
|--:|-------------:|:--------|
| 2 | 175 | README.md section #<n> |

## Walker waste (walker_t ≤ 3000, off_3k ≥ 50)

| off_3k | off_3k_ratio | off_any | cost | walker_t | batch |
|-------:|-------------:|--------:|-----:|---------:|:------|
| 621 | 0.61 | 340 | 1025 | 2438 | c decl names surface in krep.h |
| 258 | 0.92 | 258 | 279 | 585 | headings outline in README.md |
| 182 | 1.00 | 0 | 182 | 212 | c decl names surface in aho_corasick.h |
| 175 | 1.00 | 77 | 175 | 1482 | c decl names surface in aho_corasick.c |
| 114 | 1.00 | 114 | 114 | 1889 | README.md section #1 |
| 61 | 1.00 | 61 | 61 | 1192 | README.md section #16 |
| 61 | 1.00 | 0 | 61 | 1809 | c decl at aho_corasick.c:26 |
