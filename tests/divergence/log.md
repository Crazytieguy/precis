scores: Sim=0.521 Reached=15/44 Early=0 Late=10 Partial=2 Missing=27 Used=8545/10000

## Verdict

Verdict: budget-pressure bound
Likely primary lever: free final budget / demote late low-value spend
Evidence: 10 ranking-recoverable (w×gap=1.58), 17 wrong-slice/granularity (w×gap=1.50), 1 no-discovered (w×gap=0.01)
Secondary intervention: split wrong-slice batches for 17 rows
Loss reasons: 0 predecessor-gated, 9 too-expensive, 1 discovered-unscheduled
Top rows: 2.6, 3.1, 3.5, 3.2, 3.9, ...
Note: likely lever is heuristic; verify `Sim` moves, not just bucket counts.

## Top opportunities

_`w(t)×gap` is a non-additive priority score: Σ exp(-exp_t/τ) × (1 - credit) per row, τ=2000. Same time weighting and credit gap as Sim, but rows can overlap between opportunities — sums across rows are an upper bound on Sim impact, not an additive estimate._

| intervention | rows | w(t)×gap | bands ≤3k/≤6k/total | evidence | top row ids |
|:-------------|-----:|---------:|:----------------------|:---------|:------------|
| split wrong-slice walker batches | 17 | 1.50 | 3/9/17 | nearby candidates have low exact atom overlap | 2.6, 3.1, 3.5, 3.2, 3.9, ... |
| free final budget / demote late waste | 9 | 1.43 | 3/4/9 | high-overlap candidates exceed final remaining budget, exact total=95/98 | 1.6, 3.3, 3.4, 4.3, 5.2, ... |
| tune ranking for discovered unscheduled candidates | 1 | 0.15 | 0/1/1 | high-overlap candidates fit but did not win, exact total=27/28 | 3.7 |
| add walker candidates for no-discovered rows | 1 | 0.01 | 0/0/1 | NS rows have no discovered line candidate | 5.11 |

Tiers: 1=5/6 reached, 0 partial, 1 missing, avg=0.83; 2=6/7 reached, 0 partial, 1 missing, avg=0.85; 3=1/9 reached, 0 partial, 8 missing, avg=0.21; 4=2/8 reached, 2 partial, 4 missing, avg=0.46; 5=1/14 reached, 0 partial, 13 missing, avg=0.10

## Diagnosis rollup

| diagnosis | rows | missing | partial | timing | likely lever |
|:----------|-----:|--------:|--------:|-------:|:-------------|
| ranking-recoverable | 10 | 10 | 0 | 0 | value/ranking |
| wrong-slice / granularity | 17 | 15 | 2 | 0 | walker granularity / wrong slice |
| no discovered candidate | 1 | 1 | 0 | 0 | walker coverage or predecessor-gated emit |
| fs/listing | 1 | 1 | 0 | 0 | filesystem/listing value |
| timing-only | 11 | 0 | 0 | 11 | usually no code change |

## Loss reason rollup (ranking-recoverable rows)

| loss reason | rows | w(t)×gap | likely lever |
|:------------|-----:|---------:|:-------------|
| too expensive at final margin | 9 | 1.43 | free final budget |
| discovered unscheduled | 1 | 0.15 | tune ranking |

_Candidate coverage note: candidates are the walker batches discovered during this scheduled run; descendants behind unscheduled predecessors may not be present, so `no discovered candidate` is not proof that no walker emit path exists._
Candidate hint kinds: scheduled bbox=23, unscheduled bbox=9, scheduled same-file=5, fs-only=2, no discovered candidate=1

## Exact atom overlap rollup (bbox hints)

| kind | status | exact_overlap | rows |
|:-----|:-------|:--------------|-----:|
| scheduled bbox | aligned | high | 1 |
| scheduled bbox | late | low | 1 |
| scheduled bbox | late | high | 2 |
| scheduled bbox | late | full | 6 |
| scheduled bbox | missing | none | 3 |
| scheduled bbox | missing | low | 8 |
| scheduled bbox | partial | low | 2 |
| unscheduled bbox | missing | high | 1 |
| unscheduled bbox | missing | full | 8 |

## Arrival ledger by diagnosis

### ranking-recoverable

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.6 | 545 | — | — | 0.00 | missing | Five-macro user-facing summary | [unscheduled bbox exact=10/10] crate-doc body in src/lib.rs (18 atoms, too expensive at final margin) |
| 3.3 | 2682 | — | — | 0.00 | missing | Level public-method index | [unscheduled bbox exact=6/6] impl method sigs in src/lib.rs (11 atoms, too expensive at final margin) |
| 3.4 | 2779 | — | — | 0.00 | missing | LevelFilter public-method index | [unscheduled bbox exact=6/6] impl method sigs in src/lib.rs (11 atoms, too expensive at final margin) |
| 3.7 | 3856 | — | — | 0.00 | missing | __log internal-macro body (the actual gate) | [scheduled bbox exact=1/28] macro_export names across src (t=3890, 1 atoms); better unscheduled exact=27/28: macro_export body at src/macros.rs:119 (27 atoms, discovered unscheduled) |
| 4.3 | 5477 | — | — | 0.00 | missing | kv capture-modifier table | [unscheduled bbox exact=10/10] crate-doc body in src/kv/mod.rs (10 atoms, too expensive at final margin) |
| 5.2 | 7149 | — | — | 0.00 | missing | Implementing-a-Logger doc snippet | [unscheduled bbox exact=18/18] crate-doc body in src/lib.rs (18 atoms, too expensive at final margin) |
| 5.3 | 7366 | — | — | 0.00 | missing | Default-Off warning + STATIC_MAX_LEVEL note | [unscheduled bbox exact=11/11] crate-doc body in src/lib.rs (11 atoms, too expensive at final margin) |
| 5.6 | 8318 | — | — | 0.00 | missing | RecordBuilder method index | [unscheduled bbox exact=12/12] impl method sigs in src/lib.rs (23 atoms, too expensive at final margin) |
| 5.7 | 8407 | — | — | 0.00 | missing | MetadataBuilder method index | [unscheduled bbox exact=4/4] impl method sigs in src/lib.rs (7 atoms, too expensive at final margin) |
| 5.8 | 8659 | — | — | 0.00 | missing | Logger blanket impls (&T, Box, Arc) | [unscheduled bbox exact=18/21] impl method sigs in src/lib.rs (26 atoms, too expensive at final margin) |

### wrong-slice / granularity

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 2.6 | 1531 | — | — | 0.01 | missing | Logger installation entry-point signatures | [scheduled bbox exact=0/14] pub-item doc lede at src/lib.rs:1396 (t=5578, 13 atoms) |
| 3.1 | 2481 | — | — | 0.24 | missing | Record struct + accessor signatures | [scheduled bbox exact=9/38] pub item at src/lib.rs:842 (t=1488, 9 atoms); better unscheduled exact=20/38: impl method sigs in src/lib.rs (26 atoms, too expensive at final margin) |
| 3.2 | 2587 | — | — | 0.40 | missing | Metadata struct + accessors | [scheduled bbox exact=4/10] pub item at src/lib.rs:1158 (t=1394, 4 atoms); better unscheduled exact=6/10: impl method sigs in src/lib.rs (7 atoms, too expensive at final margin) |
| 3.5 | 3082 | — | — | 0.00 | missing | Global state + ordering constants | [scheduled same-file] pub-item names surface in src/lib.rs (t=1336, 34 atoms) |
| 3.8 | 4415 | — | — | 0.22 | missing | __private_api log dispatcher | [scheduled bbox exact=11/55] pub item at src/__private_api.rs:84 (t=6916, 11 atoms) |
| 3.9 | 4715 | — | — | 0.00 | missing | set_logger_inner state transitions | [scheduled same-file] pub-item names surface in src/lib.rs (t=1336, 34 atoms) |
| 4.1 | 5085 | — | — | 0.33 | missing | kv module concept | [scheduled bbox exact=10/30] crate-doc lede in src/kv/mod.rs (t=730, 10 atoms); better unscheduled exact=19/30: crate-doc body in src/kv/mod.rs (19 atoms, too expensive at final margin) |
| 4.2 | 5290 | — | — | 0.55 | partial | kv module re-exports | [scheduled bbox exact=11/20] mod/use plumbing in src/kv/mod.rs (t=3343, 11 atoms) |
| 4.5 | 5889 | — | — | 0.18 | missing | kv::Value capture constructors | [scheduled bbox exact=3/17] pub item at src/kv/value.rs:119 (t=940, 3 atoms) |
| 4.6 | 6053 | — | — | 0.58 | partial | kv::Key surface | [scheduled bbox exact=4/12] pub item at src/kv/key.rs:37 (t=840, 4 atoms) |
| 4.8 | 6552 | — | — | 0.00 | missing | kv::Value to_* primitive accessors | [scheduled same-file] pub item at src/kv/value.rs:462 (t=8477, 63 atoms) |
| 5.4 | 7693 | — | — | 0.00 | missing | Compile-time max_level_* conflict guards | [scheduled same-file] pub-item names surface in src/lib.rs (t=1336, 34 atoms) |
| 5.5 | 7986 | — | — | 0.00 | missing | FromStr impls for Level/LevelFilter | [scheduled bbox exact=0/24] pub item at src/lib.rs:636 (t=1627, 14 atoms); better unscheduled exact=6/24: impl method sigs in src/lib.rs (30 atoms, too expensive at final margin) |
| 5.9 | 8977 | — | — | 0.07 | missing | non-atomic AtomicUsize fallback | [scheduled bbox exact=2/29] mod/use plumbing in src/lib.rs (t=2902, 2 atoms); better unscheduled exact=7/29: impl method sigs in src/lib.rs (7 atoms, too expensive at final margin) |
| 5.12 | 9640 | — | — | 0.00 | missing | kv::Source impl matrix | [scheduled bbox exact=0/37] pub item at src/kv/source.rs:235 (t=886, 4 atoms) |
| 5.13 | 9784 | — | — | 0.27 | missing | kv::Error variants | [scheduled bbox exact=3/15] pub item at src/kv/error.rs:5 (t=641, 3 atoms) |
| 5.14 | 9861 | — | — | 0.00 | missing | logger() global accessor | [scheduled same-file] pub-item names surface in src/lib.rs (t=1336, 34 atoms) |

### no discovered candidate

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 5.11 | 9234 | — | — | 0.00 | missing | Macro test-fn names (tests/macros.rs) | no discovered line candidate |

### fs/listing

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 5.10 | 8998 | — | — | 0.20 | missing | tests/ + benches/ + harness listings | fs-only |

### timing-only

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.2 | 102 | 355 | +253 | 1.00 | late | Cargo name + description | [scheduled bbox exact=4/4] [package] in Cargo.toml (t=355, 9 atoms) |
| 1.3 | 190 | 599 | +409 | 1.00 | late | Crate-doc one-liner | [scheduled bbox exact=6/6] crate-doc lede in src/lib.rs (t=599, 6 atoms) |
| 1.4 | 231 | 619 | +388 | 1.00 | late | src/ + src/kv/ listing | fs-only |
| 1.5 | 381 | 599 | +218 | 1.00 | late | Crate-doc target/level/body model | [scheduled bbox exact=9/9] crate-doc lede in src/lib.rs (t=599, 9 atoms) |
| 2.1 | 735 | 1336 | +601 | 1.00 | late | Public-item map of lib.rs | [scheduled bbox exact=16/17] pub-item names surface in src/lib.rs (t=1336, 31 atoms) |
| 2.2 | 831 | 3890 | +3059 | 1.00 | late | Macro names (src/macros.rs) | [scheduled bbox exact=1/10] macro_export body at src/macros.rs:75 (t=7637, 38 atoms) |
| 2.3 | 883 | 2419 | +1536 | 1.00 | late | Log trait method signatures | [scheduled bbox exact=4/4] pub item at src/lib.rs:1249 (t=2419, 17 atoms) |
| 2.4 | 1153 | 1932 | +779 | 1.00 | late | Level enum body | [scheduled bbox exact=24/24] pub item at src/lib.rs:475 (t=1932, 24 atoms) |
| 2.7 | 2033 | 7637 | +5604 | 0.93 | late | log! macro shapes (4 forms) | [scheduled bbox exact=37/40] macro_export body at src/macros.rs:75 (t=7637, 37 atoms) |
| 4.7 | 6361 | 8477 | +2116 | 1.00 | late | VisitValue trait method index | [scheduled bbox exact=14/14] pub item at src/kv/value.rs:462 (t=8477, 57 atoms) |
| 5.1 | 6964 | 4961 | -2003 | 0.85 | aligned | Cargo features list | [scheduled bbox exact=28/33] [features] in Cargo.toml (t=4961, 28 atoms) |

## Walker waste rollup (by descriptor pattern)

| n | off_tokens_total | pattern |
|--:|-----------------:|:--------|
| 12 | 1301 | pub-item doc lede at src/lib.rs:<n> |
| 4 | 402 | README.md section #<n> |

## Walker waste (off-NS token spend ≥ 50)

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 544 | 0.65 | 840 | 8477 | pub item at src/kv/value.rs:462 |
| 397 | 1.00 | 397 | 6400 | [dependencies] in Cargo.toml |
| 359 | 0.84 | 425 | 6003 | pub item at src/kv/source.rs:51 |
| 349 | 0.87 | 401 | 2419 | pub item at src/lib.rs:1249 |
| 318 | 1.00 | 318 | 6718 | macro_export body at src/macros.rs:391 |
| 213 | 1.00 | 213 | 5578 | pub-item doc lede at src/lib.rs:1396 |
| 198 | 1.00 | 198 | 3100 | README.md section #0 |
| 156 | 0.61 | 256 | 3890 | macro_export names across src |
| 128 | 0.70 | 184 | 355 | [package] in Cargo.toml |
| 126 | 1.00 | 126 | 5328 | pub-item doc lede at src/lib.rs:1611 |
| 1556 | — | — | — | +18 more rows |
