scores: Score(3000)=0.607 ns_rows≤3K=17/44 (reached=9 partial=0 missing=8)

## Per-budget scores

| B | A_B | I(B) | C(B) | Score(B) | walker_used |
|--:|----:|-----:|-----:|---------:|------------:|
| 1000 | 83 | 0.868 | 0.506 | 0.663 | 969 |
| 1442 | 120 | 0.858 | 0.480 | 0.642 | 1436 |
| 2080 | 174 | 0.869 | 0.547 | 0.690 | 2064 |
| 3000 | 234 | 0.843 | 0.437 | 0.607 | 2991 |
| 4327 | 297 | 0.830 | 0.428 | 0.596 | 4076 |
| 6240 | 476 | 0.800 | 0.317 | 0.504 | 6020 |
| 9000 | 674 | 0.804 | 0.363 | 0.540 | 8999 |

## Verdict

Verdict: wrong-slice bound
Likely primary lever: split walker batches to match NS semantic slices
Evidence: 11 ranking-recoverable (gap@3k=0.26), 18 wrong-slice/granularity (gap@3k=0.47), 1 no-discovered (gap@3k=0.00)
Secondary intervention: free T_max budget for 9 too-expensive candidates
Top rows: 3.1, 2.2, 2.6, 3.2, 3.5, ...

## Top opportunities

_`gap@B` is a non-additive priority score: `Σ over atoms with rank ≤ |A_B|: (1 − damped_credit(a)) / rank(a)`. `gap@3k` is the primary sort key — direct proxy for `Score(3000)` headroom. `gap@1k` and `gap@9k` show how the same intervention scales across the budget vector. Rows can overlap between opportunities; sums are upper bounds on Score(B) impact, not additive estimates._

| intervention | rows | gap@1k | gap@3k | gap@9k | evidence | top row ids |
|:-------------|-----:|-------:|-------:|-------:|:---------|:------------|
| split wrong-slice walker batches | 18 | 0.13 | 0.47 | 0.85 | nearby candidates have low exact atom overlap | 3.1, 2.2, 2.6, 3.2, 3.5, ... |
| finish partially-delivered NS batches | 5 | 0.00 | 0.33 | 0.28 | avg batch completion=0.33 | 3.1, 2.6, 3.2, 4.4, 4.6 |
| free T_max budget / demote late waste | 9 | 0.21 | 0.26 | 0.40 | high-overlap candidates exceed remaining budget at T_max (caveat: not 3K-budget — see below), exact total=95/98 | 1.6, 3.3, 3.4, 4.3, 5.2, ... |

## Diagnosis rollup

| diagnosis | rows | missing | partial | likely lever |
|:----------|-----:|--------:|--------:|:-------------|
| ranking-recoverable | 11 | 11 | 0 | value/ranking |
| wrong-slice / granularity | 18 | 16 | 2 | walker granularity / wrong slice |
| no discovered candidate | 1 | 1 | 0 | walker coverage or predecessor-gated emit |
| fs/listing | 1 | 1 | 0 | filesystem/listing value |
| mixed/unknown | 3 | 3 | 0 | inspect row |

## Loss reason rollup (ranking-recoverable rows)

| loss reason | rows | gap@3k | likely lever |
|:------------|-----:|-------:|:-------------|
| too expensive at final margin | 9 | 0.26 | free T_max budget |
| discovered unscheduled | 2 | 0.00 | tune ranking |

_Candidate coverage note: candidates are the walker batches discovered during this scheduled run; descendants behind unscheduled predecessors may not be present, so `no discovered candidate` is not proof that no walker emit path exists._
Candidate hint kinds: scheduled bbox=18, unscheduled bbox=10, scheduled same-file=4, fs-only=1, no discovered candidate=1

## Exact atom overlap rollup (bbox hints)

| kind | status | exact_overlap | rows |
|:-----|:-------|:--------------|-----:|
| scheduled bbox | missing | none | 3 |
| scheduled bbox | missing | low | 10 |
| scheduled bbox | missing | high | 2 |
| scheduled bbox | missing | full | 1 |
| scheduled bbox | partial | low | 2 |
| unscheduled bbox | missing | high | 2 |
| unscheduled bbox | missing | full | 8 |

## Arrival ledger by diagnosis

### ranking-recoverable

| id | exp_t | credit | comp | status | descriptor | candidate hint |
|----|------:|-------:|-----:|:-------|:-----------|:--------------------|
| 1.6 | 545 | 0.00 | 0.00 | missing | Five-macro user-facing summary | [unscheduled bbox exact=10/10] crate-doc body in src/lib.rs (18 atoms, too expensive at final margin) |
| 3.3 | 2682 | 0.00 | 0.00 | missing | Level public-method index | [unscheduled bbox exact=6/6] impl method sigs in src/lib.rs (11 atoms, too expensive at final margin) |
| 3.4 | 2779 | 0.00 | 0.00 | missing | LevelFilter public-method index | [unscheduled bbox exact=6/6] impl method sigs in src/lib.rs (11 atoms, too expensive at final margin) |
| 3.7 | 3856 | 0.00 | 0.00 | missing | __log internal-macro body (the actual gate) | [scheduled bbox exact=1/28] macro_export names across src (t=4332, 1 atoms); better unscheduled exact=27/28: macro_export body at src/macros.rs:119 (27 atoms, discovered unscheduled) |
| 4.3 | 5477 | 0.00 | 0.00 | missing | kv capture-modifier table | [unscheduled bbox exact=10/10] crate-doc body in src/kv/mod.rs (10 atoms, too expensive at final margin) |
| 5.2 | 7149 | 0.00 | 0.00 | missing | Implementing-a-Logger doc snippet | [unscheduled bbox exact=18/18] crate-doc body in src/lib.rs (18 atoms, too expensive at final margin) |
| 5.3 | 7366 | 0.00 | 0.00 | missing | Default-Off warning + STATIC_MAX_LEVEL note | [unscheduled bbox exact=11/11] crate-doc body in src/lib.rs (11 atoms, too expensive at final margin) |
| 5.6 | 8318 | 0.00 | 0.00 | missing | RecordBuilder method index | [unscheduled bbox exact=12/12] impl method sigs in src/lib.rs (23 atoms, too expensive at final margin) |
| 5.7 | 8407 | 0.00 | 0.00 | missing | MetadataBuilder method index | [unscheduled bbox exact=4/4] impl method sigs in src/lib.rs (7 atoms, too expensive at final margin) |
| 5.8 | 8659 | 0.00 | 0.00 | missing | Logger blanket impls (&T, Box, Arc) | [unscheduled bbox exact=18/21] impl method sigs in src/lib.rs (26 atoms, too expensive at final margin) |
| 5.14 | 9861 | 0.00 | 0.00 | missing | logger() global accessor | [unscheduled bbox exact=6/7] pub item body at src/lib.rs:1581 body 1590 (6 atoms, discovered unscheduled) |

### wrong-slice / granularity

| id | exp_t | credit | comp | status | descriptor | candidate hint |
|----|------:|-------:|-----:|:-------|:-----------|:--------------------|
| 2.2 | 831 | 0.00 | 0.00 | missing | Macro names (src/macros.rs) | [scheduled bbox exact=1/10] macro_export body at src/macros.rs:75 (t=8159, 38 atoms) |
| 2.6 | 1531 | 0.22 | 0.22 | missing | Logger installation entry-point signatures | [scheduled bbox exact=0/14] pub-item doc lede at src/lib.rs:1396 (t=6020, 13 atoms) |
| 3.1 | 2481 | 0.24 | 0.23 | missing | Record struct + accessor signatures | [scheduled bbox exact=9/38] pub item at src/lib.rs:842 (t=1530, 9 atoms); better unscheduled exact=20/38: impl method sigs in src/lib.rs (26 atoms, too expensive at final margin) |
| 3.2 | 2587 | 0.40 | 0.26 | missing | Metadata struct + accessors | [scheduled bbox exact=4/10] pub item at src/lib.rs:1158 (t=1394, 4 atoms); better unscheduled exact=6/10: impl method sigs in src/lib.rs (7 atoms, too expensive at final margin) |
| 3.5 | 3082 | 0.00 | 0.00 | missing | Global state + ordering constants | [scheduled same-file] pub-item names surface in src/lib.rs (t=1336, 34 atoms) |
| 3.8 | 4415 | 0.00 | 0.00 | missing | __private_api log dispatcher | [scheduled bbox exact=11/55] pub item at src/__private_api.rs:84 (t=7383, 11 atoms) |
| 3.9 | 4715 | 0.00 | 0.00 | missing | set_logger_inner state transitions | [scheduled same-file] pub-item names surface in src/lib.rs (t=1336, 34 atoms) |
| 4.1 | 5085 | 0.33 | 0.16 | missing | kv module concept | [scheduled bbox exact=10/30] crate-doc lede in src/kv/mod.rs (t=240, 10 atoms); better unscheduled exact=19/30: crate-doc body in src/kv/mod.rs (19 atoms, too expensive at final margin) |
| 4.2 | 5290 | 0.00 | 0.00 | missing | kv module re-exports | [scheduled bbox exact=11/20] mod/use plumbing in src/kv/mod.rs (t=3785, 11 atoms) |
| 4.4 | 5612 | 0.62 | 0.49 | partial | kv::Source trait surface | [scheduled bbox exact=4/8] pub item at src/kv/source.rs:51 (t=6445, 36 atoms) |
| 4.5 | 5889 | 0.18 | 0.08 | missing | kv::Value capture constructors | [scheduled bbox exact=3/17] pub item at src/kv/value.rs:119 (t=940, 3 atoms) |
| 4.6 | 6053 | 0.58 | 0.45 | partial | kv::Key surface | [scheduled bbox exact=4/12] pub item at src/kv/key.rs:37 (t=596, 4 atoms) |
| 4.8 | 6552 | 0.00 | 0.00 | missing | kv::Value to_* primitive accessors | [scheduled same-file] pub item at src/kv/value.rs:462 (t=8999, 63 atoms) |
| 5.4 | 7693 | 0.00 | 0.00 | missing | Compile-time max_level_* conflict guards | [scheduled same-file] pub-item names surface in src/lib.rs (t=1336, 34 atoms) |
| 5.5 | 7986 | 0.00 | 0.00 | missing | FromStr impls for Level/LevelFilter | [scheduled bbox exact=0/24] pub item at src/lib.rs:636 (t=1669, 14 atoms); better unscheduled exact=6/24: impl method sigs in src/lib.rs (30 atoms, too expensive at final margin) |
| 5.9 | 8977 | 0.00 | 0.00 | missing | non-atomic AtomicUsize fallback | [scheduled bbox exact=2/29] mod/use plumbing in src/lib.rs (t=3344, 2 atoms); better unscheduled exact=7/29: impl method sigs in src/lib.rs (7 atoms, too expensive at final margin) |
| 5.12 | 9640 | 0.00 | 0.00 | missing | kv::Source impl matrix | [scheduled bbox exact=0/37] pub item at src/kv/source.rs:235 (t=642, 4 atoms) |
| 5.13 | 9784 | 0.20 | 0.12 | missing | kv::Error variants | [scheduled bbox exact=3/15] pub item at src/kv/error.rs:5 (t=151, 3 atoms) |

### no discovered candidate

| id | exp_t | credit | comp | status | descriptor | candidate hint |
|----|------:|-------:|-----:|:-------|:-----------|:--------------------|
| 5.11 | 9234 | 0.00 | 0.00 | missing | Macro test-fn names (tests/macros.rs) | no discovered line candidate |

### fs/listing

| id | exp_t | credit | comp | status | descriptor | candidate hint |
|----|------:|-------:|-----:|:-------|:-----------|:--------------------|
| 5.10 | 8998 | 0.00 | 0.00 | missing | tests/ + benches/ + harness listings | fs-only |

### mixed/unknown

| id | exp_t | credit | comp | status | descriptor | candidate hint |
|----|------:|-------:|-----:|:-------|:-----------|:--------------------|
| 2.7 | 2033 | 0.00 | 0.00 | missing | log! macro shapes (4 forms) | [scheduled bbox exact=37/40] macro_export body at src/macros.rs:75 (t=8159, 37 atoms) |
| 4.7 | 6361 | 0.07 | 0.03 | missing | VisitValue trait method index | [scheduled bbox exact=14/14] pub item at src/kv/value.rs:462 (t=8999, 57 atoms) |
| 5.1 | 6964 | 0.00 | 0.00 | missing | Cargo features list | [scheduled bbox exact=28/33] [features] in Cargo.toml (t=5403, 28 atoms) |

## Walker waste rollup (by descriptor pattern)

| n | off_tokens_total | pattern |
|--:|-----------------:|:--------|
| 12 | 1301 | pub-item doc lede at src/lib.rs:<n> |
| 4 | 402 | README.md section #<n> |

## Walker waste (off-NS token spend ≥ 50)

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 544 | 0.65 | 840 | 8999 | pub item at src/kv/value.rs:462 |
| 397 | 1.00 | 397 | 6842 | [dependencies] in Cargo.toml |
| 359 | 0.84 | 425 | 6445 | pub item at src/kv/source.rs:51 |
| 349 | 0.87 | 401 | 2551 | pub item at src/lib.rs:1249 |
| 318 | 1.00 | 318 | 7160 | macro_export body at src/macros.rs:391 |
| 213 | 1.00 | 213 | 6020 | pub-item doc lede at src/lib.rs:1396 |
| 198 | 1.00 | 198 | 3542 | README.md section #0 |
| 156 | 0.61 | 256 | 4332 | macro_export names across src |
| 147 | 1.00 | 147 | 2715 | pub item body at src/lib.rs:1529 body 1530 |
| 128 | 0.70 | 184 | 486 | [package] in Cargo.toml |
| 1772 | — | — | — | +20 more rows |
