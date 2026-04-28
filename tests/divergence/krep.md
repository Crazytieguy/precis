scores: Sim=0.403 Reached=19/39 Early=4 Late=12 Partial=6 Missing=14 Used=9912/10000

## Verdict

Verdict: wrong-slice bound
Likely primary lever: split walker batches to match NS semantic slices
Evidence: 4 ranking-recoverable (w×gap=0.27), 13 wrong-slice/granularity (w×gap=1.02), 2 no-discovered (w×gap=0.02)
Secondary intervention: free final budget for 4 too-expensive candidates
Loss reasons: 0 predecessor-gated, 4 too-expensive, 0 discovered-unscheduled
Top rows: 2.6, 2.9, 2.10, 2.1, 2.4, ...
Note: likely lever is heuristic; verify `Sim` moves, not just bucket counts.

## Top opportunities

_`w(t)×gap` is a non-additive priority score: Σ exp(-exp_t/τ) × (1 - credit) per row, τ=2000. Same time weighting and credit gap as Sim, but rows can overlap between opportunities — sums across rows are an upper bound on Sim impact, not an additive estimate._

| intervention | rows | w(t)×gap | bands ≤3k/≤6k/total | evidence | top row ids |
|:-------------|-----:|---------:|:----------------------|:---------|:------------|
| split wrong-slice walker batches | 13 | 1.02 | 3/7/13 | nearby candidates have low exact atom overlap | 2.6, 2.9, 2.10, 2.1, 2.4, ... |
| free final budget / demote late waste | 4 | 0.27 | 0/3/4 | high-overlap candidates exceed final remaining budget, exact total=151/173 | 3.2, 3.3, 3.4, 5.8 |
| add walker candidates for no-discovered rows | 2 | 0.02 | 0/0/2 | NS rows have no discovered line candidate | 7.2, 7.3 |

Tiers: 1=8/8 reached, 0 partial, 0 missing, avg=1.00; 2=5/10 reached, 2 partial, 3 missing, avg=0.66; 3=1/5 reached, 1 partial, 3 missing, avg=0.33; 4=0/3 reached, 0 partial, 3 missing, avg=0.00; 5=5/8 reached, 1 partial, 2 missing, avg=0.66; 6=0/2 reached, 2 partial, 0 missing, avg=0.63; 7=0/3 reached, 0 partial, 3 missing, avg=0.00

## Diagnosis rollup

| diagnosis | rows | missing | partial | timing | likely lever |
|:----------|-----:|--------:|--------:|-------:|:-------------|
| ranking-recoverable | 4 | 4 | 0 | 0 | value/ranking |
| wrong-slice / granularity | 13 | 7 | 6 | 0 | walker granularity / wrong slice |
| no discovered candidate | 2 | 2 | 0 | 0 | walker coverage or predecessor-gated emit |
| fs/listing | 1 | 1 | 0 | 0 | filesystem/listing value |
| timing-only | 17 | 0 | 0 | 17 | usually no code change |

## Loss reason rollup (ranking-recoverable rows)

| loss reason | rows | w(t)×gap | likely lever |
|:------------|-----:|---------:|:-------------|
| too expensive at final margin | 4 | 0.27 | free final budget |

_Candidate coverage note: candidates are the walker batches discovered during this scheduled run; descendants behind unscheduled predecessors may not be present, so `no discovered candidate` is not proof that no walker emit path exists._
Candidate hint kinds: scheduled bbox=27, unscheduled bbox=1, scheduled same-file=6, fs-only=1, no discovered candidate=2

## Exact atom overlap rollup (bbox hints)

| kind | status | exact_overlap | rows |
|:-----|:-------|:--------------|-----:|
| scheduled bbox | aligned | low | 1 |
| scheduled bbox | early | low | 3 |
| scheduled bbox | early | high | 1 |
| scheduled bbox | late | low | 4 |
| scheduled bbox | late | high | 3 |
| scheduled bbox | late | full | 5 |
| scheduled bbox | missing | low | 4 |
| scheduled bbox | partial | low | 6 |
| unscheduled bbox | missing | high | 1 |

## Arrival ledger by diagnosis

### ranking-recoverable

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 3.2 | 4164 | — | — | 0.03 | missing | select_search_algorithm body — regex/AC/override branches | [scheduled bbox exact=2/30] c decl at krep.c:1771 (t=6592, 2 atoms); better unscheduled exact=24/30: c decl body at krep.c:1771 (24 atoms, too expensive at final margin) |
| 3.3 | 5061 | — | — | 0.00 | missing | select_search_algorithm body — single-pattern dispatch tail | [unscheduled bbox exact=64/69] c decl body at krep.c:1771 (64 atoms, too expensive at final margin) |
| 3.4 | 5485 | — | — | 0.06 | missing | get_algorithm_name body — algorithm enumeration | [scheduled bbox exact=3/33] c decl at krep.c:1964 (t=6604, 3 atoms); better unscheduled exact=30/33: c decl body at krep.c:1964 (30 atoms, too expensive at final margin) |
| 5.8 | 8912 | — | — | 0.05 | missing | search_chunk_thread — what each worker actually does | [scheduled bbox exact=3/41] c decl at krep.c:1919 (t=6598, 3 atoms); better unscheduled exact=33/41: c decl body at krep.c:1919 (33 atoms, too expensive at final margin) |

### wrong-slice / granularity

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 2.1 | 1329 | — | — | 0.71 | partial | krep.h header banner + includes | [scheduled bbox exact=7/17] c includes in krep.h (t=1779, 7 atoms) |
| 2.4 | 2024 | — | — | 0.76 | partial | search_params_t struct (the central state) | [scheduled bbox exact=24/33] c decl at krep.h:65 (t=4948, 24 atoms) |
| 2.6 | 2382 | — | — | 0.25 | missing | Internal search-algorithm declarations — names + ISA gates | [scheduled bbox exact=5/20] c decl names surface in krep.h (t=2839, 6 atoms) |
| 2.9 | 3287 | — | — | 0.00 | missing | Recursive-search skip lists (directories) | [scheduled same-file] c decl names surface in krep.h (t=2839, 70 atoms) |
| 2.10 | 3743 | — | — | 0.00 | missing | Recursive-search skip lists (file extensions) | [scheduled same-file] c decl names surface in krep.h (t=2839, 70 atoms) |
| 3.5 | 5604 | — | — | 0.56 | partial | Search algorithm signatures in krep.c — locations | [scheduled bbox exact=5/9] c decl names surface in krep.c (t=6527, 33 atoms) |
| 4.1 | 5618 | — | — | 0.00 | missing | main() signature | [scheduled same-file] c decl names surface in krep.c (t=6527, 77 atoms) |
| 4.2 | 6016 | — | — | 0.00 | missing | main() — long_options table + getopt_long invocation | [scheduled same-file] c decl names surface in krep.c (t=6527, 77 atoms) |
| 4.3 | 6124 | — | — | 0.00 | missing | Param finalization — count/track-positions wiring | [scheduled same-file] c decl names surface in krep.c (t=6527, 77 atoms) |
| 5.6 | 7944 | — | — | 0.79 | partial | krep.c — key constants (chunk size / version / paths) | [scheduled bbox exact=11/14] c decl names surface in krep.c (t=6527, 11 atoms) |
| 5.7 | 8345 | — | — | 0.00 | missing | krep.c — SIMD intrinsic include gating | [scheduled same-file] c decl names surface in krep.c (t=6527, 77 atoms) |
| 6.1 | 9197 | — | — | 0.69 | partial | krep.c top-level function names — locations | [scheduled bbox exact=18/26] c decl names surface in krep.c (t=6527, 53 atoms) |
| 6.2 | 9298 | — | — | 0.57 | partial | aho_corasick.c function names — locations | [scheduled bbox exact=4/7] c decl names surface in aho_corasick.c (t=1312, 12 atoms) |

### no discovered candidate

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 7.2 | 9620 | — | — | 0.00 | missing | Makefile — sources and main build target | no discovered line candidate |
| 7.3 | 9938 | — | — | 0.00 | missing | Makefile — arch/SIMD flag detection | no discovered line candidate |

### fs/listing

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 7.1 | 9340 | — | — | 0.00 | missing | test/ FS listing | fs-only |

### timing-only

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.3 | 226 | 5201 | +4975 | 1.00 | late | Usage line shapes | [scheduled bbox exact=9/9] README.md section #8 (t=5201, 9 atoms) |
| 1.4 | 343 | 1014 | +671 | 1.00 | late | README scope disclaimer | [scheduled bbox exact=4/4] README.md section #0 (t=1014, 4 atoms) |
| 1.5 | 519 | 9831 | +9312 | 1.00 | late | Command-line options reference (search/scope flags) | [scheduled bbox exact=8/9] README.md section #10 (t=9831, 8 atoms) |
| 1.6 | 769 | 9831 | +9062 | 1.00 | late | Command-line options reference (recursive/threading/UI flags) | [scheduled bbox exact=11/11] README.md section #10 (t=9831, 11 atoms) |
| 1.7 | 959 | 8276 | +7317 | 1.00 | late | README key-features bullets (algorithms + SIMD + I/O) | [scheduled bbox exact=8/9] README.md section #2 (t=8276, 8 atoms) |
| 1.8 | 1131 | 8276 | +7145 | 1.00 | late | README key-features bullets (recursive/UX/limits) | [scheduled bbox exact=7/7] README.md section #2 (t=8276, 7 atoms) |
| 2.2 | 1514 | 3261 | +1747 | 0.93 | late | match_position_t and match_result_t | [scheduled bbox exact=6/14] c decl at krep.h:55 (t=3261, 6 atoms) |
| 2.3 | 1605 | 3182 | +1577 | 1.00 | late | search_func_t function pointer typedef | [scheduled bbox exact=4/6] c decl at krep.h:98 (t=2888, 4 atoms) |
| 2.5 | 2124 | 2839 | +715 | 1.00 | late | Helper API + select_search_algorithm + get_algorithm_name | [scheduled bbox exact=4/5] c decl names surface in krep.h (t=2839, 4 atoms) |
| 2.7 | 2786 | 4174 | +1388 | 0.94 | late | Public API: search_file / search_string / search_directory_recursive | [scheduled bbox exact=8/31] c decl doc at krep.h:161 (t=4174, 8 atoms) |
| 2.8 | 3120 | 4601 | +1481 | 1.00 | late | match_result_* lifecycle + print_matching_items | [scheduled bbox exact=12/22] c decl doc at krep.h:200 (t=4601, 12 atoms) |
| 3.1 | 3761 | 6527 | +2766 | 1.00 | late | select_search_algorithm signature | [scheduled bbox exact=1/1] c decl at krep.c:1771 (t=6592, 1 atoms) |
| 5.1 | 6621 | 585 | -6036 | 0.85 | early | aho_corasick.h — public AC API | [scheduled bbox exact=19/41] c decl names surface in aho_corasick.h (t=394, 19 atoms) |
| 5.2 | 6820 | 5072 | -1748 | 0.93 | aligned | ac_node_t + ac_trie struct definitions | [scheduled bbox exact=8/15] c decl at aho_corasick.c:17 (t=5072, 8 atoms) |
| 5.3 | 7170 | 3499 | -3671 | 0.92 | early | thread_pool_t + task_t structures | [scheduled bbox exact=12/26] c decl at krep.h:132 (t=3499, 12 atoms) |
| 5.4 | 7450 | 4418 | -3032 | 0.85 | early | thread_data_t — what each chunk task carries | [scheduled bbox exact=16/20] c decl at krep.h:104 (t=4418, 16 atoms) |
| 5.5 | 7712 | 3954 | -3758 | 0.87 | early | Whole-word match inline helpers | [scheduled bbox exact=9/23] c decl doc at krep.h:312 (t=3795, 9 atoms) |

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
| 1170 | — | — | — | +16 more rows |
