scores: Score(3000)=0.436 ns_rows≤3K=15/39 (reached=5 partial=3 missing=7)

## Per-budget scores

| B | A_B | I(B) | C(B) | Score(B) | walker_used |
|--:|----:|-----:|-----:|---------:|------------:|
| 1000 | 54 | 0.700 | 0.225 | 0.397 | 897 |
| 1442 | 78 | 0.720 | 0.338 | 0.493 | 1437 |
| 2080 | 131 | 0.669 | 0.228 | 0.391 | 2049 |
| 3000 | 187 | 0.696 | 0.273 | 0.436 | 2396 |
| 4327 | 270 | 0.715 | 0.321 | 0.479 | 4270 |
| 6240 | 407 | 0.886 | 0.421 | 0.611 | 6142 |
| 9000 | 616 | 0.840 | 0.476 | 0.633 | 8913 |

## Verdict

Verdict: wrong-slice bound
Likely primary lever: split walker batches to match NS semantic slices
Evidence: 4 ranking-recoverable (gap@3k=0.00), 22 wrong-slice/granularity (gap@3k=1.27), 2 no-discovered (gap@3k=0.00)
Secondary intervention: free T_max budget for 4 too-expensive candidates
Top rows: 2.4, 1.5, 2.7, 2.2, 1.6, ...

## Top opportunities

_`gap@B` is a non-additive priority score: `Σ over atoms with rank ≤ |A_B|: (1 − damped_credit(a)) / rank(a)`. `gap@3k` is the primary sort key — direct proxy for `Score(3000)` headroom. `gap@1k` and `gap@9k` show how the same intervention scales across the budget vector. Rows can overlap between opportunities; sums are upper bounds on Score(B) impact, not additive estimates._

| intervention | rows | gap@1k | gap@3k | gap@9k | evidence | top row ids |
|:-------------|-----:|-------:|-------:|-------:|:---------|:------------|
| split wrong-slice walker batches | 22 | 0.58 | 1.27 | 0.61 | nearby candidates have low exact atom overlap | 2.4, 1.5, 2.7, 2.2, 1.6, ... |
| finish partially-delivered NS batches | 4 | 0.58 | 0.35 | 0.00 | avg batch completion=0.51 | 1.5, 1.6, 5.2, 6.2 |

## Diagnosis rollup

| diagnosis | rows | missing | partial | likely lever |
|:----------|-----:|--------:|--------:|:-------------|
| ranking-recoverable | 4 | 4 | 0 | value/ranking |
| wrong-slice / granularity | 22 | 17 | 5 | walker granularity / wrong slice |
| no discovered candidate | 2 | 2 | 0 | walker coverage or predecessor-gated emit |
| fs/listing | 1 | 1 | 0 | filesystem/listing value |
| mixed/unknown | 4 | 4 | 0 | inspect row |

## Loss reason rollup (ranking-recoverable rows)

| loss reason | rows | gap@3k | likely lever |
|:------------|-----:|-------:|:-------------|
| too expensive at final margin | 4 | 0.00 | free T_max budget |

_Candidate coverage note: candidates are the walker batches discovered during this scheduled run; descendants behind unscheduled predecessors may not be present, so `no discovered candidate` is not proof that no walker emit path exists._
Candidate hint kinds: scheduled bbox=23, unscheduled bbox=1, scheduled same-file=6, fs-only=1, no discovered candidate=2

## Exact atom overlap rollup (bbox hints)

| kind | status | exact_overlap | rows |
|:-----|:-------|:--------------|-----:|
| scheduled bbox | missing | low | 14 |
| scheduled bbox | missing | high | 2 |
| scheduled bbox | missing | full | 2 |
| scheduled bbox | partial | low | 5 |
| unscheduled bbox | missing | high | 1 |

## Arrival ledger by diagnosis

### ranking-recoverable

| id | exp_t | credit | comp | status | descriptor | candidate hint |
|----|------:|-------:|-----:|:-------|:-----------|:--------------------|
| 3.2 | 4164 | 0.00 | 0.00 | missing | select_search_algorithm body — regex/AC/override branches | [scheduled bbox exact=2/30] c decl at krep.c:1771 (t=7382, 2 atoms); better unscheduled exact=24/30: c decl body at krep.c:1771 (24 atoms, too expensive at final margin) |
| 3.3 | 5061 | 0.00 | 0.00 | missing | select_search_algorithm body — single-pattern dispatch tail | [unscheduled bbox exact=64/69] c decl body at krep.c:1771 (64 atoms, too expensive at final margin) |
| 3.4 | 5485 | 0.00 | 0.00 | missing | get_algorithm_name body — algorithm enumeration | [scheduled bbox exact=3/33] c decl at krep.c:1964 (t=7394, 3 atoms); better unscheduled exact=30/33: c decl body at krep.c:1964 (30 atoms, too expensive at final margin) |
| 5.8 | 8912 | 0.00 | 0.00 | missing | search_chunk_thread — what each worker actually does | [scheduled bbox exact=3/41] c decl at krep.c:1919 (t=7388, 3 atoms); better unscheduled exact=33/41: c decl body at krep.c:1919 (33 atoms, too expensive at final margin) |

### wrong-slice / granularity

| id | exp_t | credit | comp | status | descriptor | candidate hint |
|----|------:|-------:|-----:|:-------|:-----------|:--------------------|
| 1.5 | 519 | 0.67 | 0.49 | partial | Command-line options reference (search/scope flags) | [scheduled bbox exact=2/9] headings outline in README.md (t=864, 2 atoms) |
| 1.6 | 769 | 0.73 | 0.63 | partial | Command-line options reference (recursive/threading/UI flags) | [scheduled bbox exact=1/11] README.md section #33 (t=6065, 1 atoms) |
| 2.1 | 1329 | 0.71 | 0.93 | partial | krep.h header banner + includes | [scheduled bbox exact=7/17] c includes in krep.h (t=2163, 7 atoms) |
| 2.2 | 1514 | 0.00 | 0.00 | missing | match_position_t and match_result_t | [scheduled bbox exact=6/14] c decl at krep.h:55 (t=3843, 6 atoms) |
| 2.3 | 1605 | 0.00 | 0.00 | missing | search_func_t function pointer typedef | [scheduled bbox exact=4/6] c decl at krep.h:98 (t=3470, 4 atoms) |
| 2.4 | 2024 | 0.00 | 0.00 | missing | search_params_t struct (the central state) | [scheduled bbox exact=24/33] c decl at krep.h:65 (t=5638, 24 atoms) |
| 2.6 | 2382 | 0.00 | 0.00 | missing | Internal search-algorithm declarations — names + ISA gates | [scheduled bbox exact=5/20] c decl names surface in krep.h (t=3421, 6 atoms) |
| 2.7 | 2786 | 0.00 | 0.00 | missing | Public API: search_file / search_string / search_directory_recursive | [scheduled bbox exact=8/31] c decl doc at krep.h:161 (t=4756, 8 atoms) |
| 2.8 | 3120 | 0.00 | 0.00 | missing | match_result_* lifecycle + print_matching_items | [scheduled bbox exact=12/22] c decl doc at krep.h:200 (t=5183, 12 atoms) |
| 2.9 | 3287 | 0.00 | 0.00 | missing | Recursive-search skip lists (directories) | [scheduled same-file] c decl names surface in krep.h (t=3421, 70 atoms) |
| 2.10 | 3743 | 0.00 | 0.00 | missing | Recursive-search skip lists (file extensions) | [scheduled same-file] c decl names surface in krep.h (t=3421, 70 atoms) |
| 3.5 | 5604 | 0.00 | 0.00 | missing | Search algorithm signatures in krep.c — locations | [scheduled bbox exact=5/9] c decl names surface in krep.c (t=7317, 33 atoms) |
| 4.1 | 5618 | 0.00 | 0.00 | missing | main() signature | [scheduled same-file] c decl names surface in krep.c (t=7317, 77 atoms) |
| 4.2 | 6016 | 0.00 | 0.00 | missing | main() — long_options table + getopt_long invocation | [scheduled same-file] c decl names surface in krep.c (t=7317, 77 atoms) |
| 4.3 | 6124 | 0.00 | 0.00 | missing | Param finalization — count/track-positions wiring | [scheduled same-file] c decl names surface in krep.c (t=7317, 77 atoms) |
| 5.2 | 6820 | 0.53 | 0.35 | partial | ac_node_t + ac_trie struct definitions | [scheduled bbox exact=8/15] c decl at aho_corasick.c:17 (t=5822, 8 atoms) |
| 5.3 | 7170 | 0.00 | 0.00 | missing | thread_pool_t + task_t structures | [scheduled bbox exact=12/26] c decl at krep.h:132 (t=4081, 12 atoms) |
| 5.5 | 7712 | 0.00 | 0.00 | missing | Whole-word match inline helpers | [scheduled bbox exact=9/23] c decl doc at krep.h:312 (t=4377, 9 atoms) |
| 5.6 | 7944 | 0.00 | 0.00 | missing | krep.c — key constants (chunk size / version / paths) | [scheduled bbox exact=11/14] c decl names surface in krep.c (t=7317, 11 atoms) |
| 5.7 | 8345 | 0.00 | 0.00 | missing | krep.c — SIMD intrinsic include gating | [scheduled same-file] c decl names surface in krep.c (t=7317, 77 atoms) |
| 6.1 | 9197 | 0.00 | 0.00 | missing | krep.c top-level function names — locations | [scheduled bbox exact=18/26] c decl names surface in krep.c (t=7317, 53 atoms) |
| 6.2 | 9298 | 0.57 | 0.58 | partial | aho_corasick.c function names — locations | [scheduled bbox exact=4/7] c decl names surface in aho_corasick.c (t=1657, 12 atoms) |

### no discovered candidate

| id | exp_t | credit | comp | status | descriptor | candidate hint |
|----|------:|-------:|-----:|:-------|:-----------|:--------------------|
| 7.2 | 9620 | 0.00 | 0.00 | missing | Makefile — sources and main build target | no discovered line candidate |
| 7.3 | 9938 | 0.00 | 0.00 | missing | Makefile — arch/SIMD flag detection | no discovered line candidate |

### fs/listing

| id | exp_t | credit | comp | status | descriptor | candidate hint |
|----|------:|-------:|-----:|:-------|:-----------|:--------------------|
| 7.1 | 9340 | 0.00 | 0.00 | missing | test/ FS listing | fs-only |

### mixed/unknown

| id | exp_t | credit | comp | status | descriptor | candidate hint |
|----|------:|-------:|-----:|:-------|:-----------|:--------------------|
| 1.3 | 226 | 0.00 | 0.00 | missing | Usage line shapes | [scheduled bbox exact=9/9] README.md section #21 (t=5951, 9 atoms) |
| 2.5 | 2124 | 0.00 | 0.00 | missing | Helper API + select_search_algorithm + get_algorithm_name | [scheduled bbox exact=4/5] c decl names surface in krep.h (t=3421, 4 atoms) |
| 3.1 | 3761 | 0.00 | 0.00 | missing | select_search_algorithm signature | [scheduled bbox exact=1/1] c decl at krep.c:1771 (t=7382, 1 atoms) |
| 5.4 | 7450 | 0.00 | 0.00 | missing | thread_data_t — what each chunk task carries | [scheduled bbox exact=16/20] c decl at krep.h:104 (t=5000, 16 atoms) |

## Walker waste rollup (by descriptor pattern)

| n | off_tokens_total | pattern |
|--:|-----------------:|:--------|
| 5 | 734 | README.md section #<n> |

## Walker waste (off-NS token spend ≥ 50)

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 340 | 0.33 | 1025 | 3421 | c decl names surface in krep.h |
| 333 | 1.00 | 333 | 9505 | c includes in krep.c |
| 259 | 1.00 | 259 | 9172 | README.md section #41 |
| 258 | 0.92 | 279 | 864 | headings outline in README.md |
| 239 | 1.00 | 239 | 8717 | README.md section #22 |
| 206 | 0.22 | 929 | 7317 | c decl names surface in krep.c |
| 156 | 1.00 | 156 | 9799 | c decl body at krep.c:401 |
| 138 | 1.00 | 138 | 9643 | c decl body at krep.c:4313 |
| 114 | 1.00 | 114 | 2003 | README.md section #1 |
| 114 | 1.00 | 114 | 6256 | c includes in aho_corasick.c |
| 1089 | — | — | — | +15 more rows |
