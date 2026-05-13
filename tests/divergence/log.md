scores: Score(3000)=0.607 ns_rows≤3K=17/44 (reached=9 partial=0 missing=8)

## Per-budget scores

| B | A_B | I(B) | C(B) | compl(B) | Score(B) | walker_used |
|--:|----:|-----:|-----:|---------:|---------:|------------:|
| 1000 | 83 | 0.868 | 0.506 | 1.000 | 0.663 | 969 |
| 1442 | 120 | 0.858 | 0.480 | 0.747 | 0.642 | 1436 |
| 2080 | 174 | 0.869 | 0.547 | 0.911 | 0.690 | 2064 |
| 3000 | 234 | 0.843 | 0.437 | 0.807 | 0.607 | 2991 |
| 4327 | 297 | 0.830 | 0.428 | 0.883 | 0.596 | 4076 |
| 6240 | 476 | 0.800 | 0.317 | 0.679 | 0.504 | 6020 |
| 9000 | 674 | 0.804 | 0.363 | 0.710 | 0.540 | 8999 |

## Top opportunities

### Additive (close partial / missing rows)

| intervention | rows | gap@1k | gap@3k | gap@9k | evidence | top row ids |
|:-------------|-----:|-------:|-------:|-------:|:---------|:------------|
| split wrong-slice walker batches | 18 | 1.21 | 1.19 | 0.92 | nearby candidates have low exact atom overlap | 3.1, 3.8, 2.2, 2.6, 3.5, ... |
| tune ranking for high-overlap unscheduled candidates | 11 | 0.51 | 0.51 | 0.51 | high-overlap candidates not in the schedule by T_max, exact total=128/133 | 1.6, 3.7, 5.8, 5.2, 3.3, ... |
| add walker candidates for no-discovered rows | 1 | 0.03 | 0.03 | 0.03 | NS rows have no discovered line candidate | 5.11 |

### Subtractive (suppress consistently off-NS batches)

| pattern | batches | freed@1k | freed@3k | freed@9k | evidence | top batch ids |
|:--------|--------:|---------:|---------:|---------:|:---------|:--------------|
| pub item at src/lib.rs:<n> | 2 | 0 | 349 | 349 | off_3k=625 | pub item at src/lib.rs:1249, pub item at src/lib.rs:1611 |
| pub item body at src/lib.rs:1529 body 1530 | 1 | 0 | 147 | 147 | off_3k=147 | pub item body at src/lib.rs:1529 body 1530 |
| [package] in Cargo.toml | 1 | 128 | 128 | 128 | off_3k=128 | [package] in Cargo.toml |
| pub item body at src/lib.rs:1375 body 1376 | 1 | 0 | 90 | 90 | off_3k=90 | pub item body at src/lib.rs:1375 body 1376 |
| macro_export names across src/kv | 1 | 67 | 67 | 67 | off_3k=67 | macro_export names across src/kv |

Top missed paths (NS rows ≤ 3K): src/lib.rs (6 rows, 84 atoms), src/macros.rs (2 rows, 50 atoms)

## Arrival ledger by diagnosis

### ranking-recoverable

| id | ns_t | credit | status | descriptor | candidate hint |
|----|-----:|-------:|:-------|:-----------|:---------------|
| 1.6 | 545 | 0.00 | missing | Five-macro user-facing summary | [unscheduled bbox exact=10/10] crate-doc body in src/lib.rs (18 atoms, too expensive at final margin) |
| 3.3 | 2682 | 0.00 | missing | Level public-method index | [unscheduled bbox exact=6/6] impl method sigs in src/lib.rs (11 atoms, too expensive at final margin) |
| 3.4 | 2779 | 0.00 | missing | LevelFilter public-method index | [unscheduled bbox exact=6/6] impl method sigs in src/lib.rs (11 atoms, too expensive at final margin) |
| 3.7 | 3856 | 0.00 | missing | __log internal-macro body (the actual gate) | [scheduled bbox exact=1/28] macro_export names across src (t=4332, 1 atoms); better unscheduled exact=27/28: macro_export body at src/macros.rs:119 (27 atoms, discovered unscheduled) |
| 4.3 | 5477 | 0.00 | missing | kv capture-modifier table | [unscheduled bbox exact=10/10] crate-doc body in src/kv/mod.rs (10 atoms, too expensive at final margin) |
| 5.2 | 7149 | 0.00 | missing | Implementing-a-Logger doc snippet | [unscheduled bbox exact=18/18] crate-doc body in src/lib.rs (18 atoms, too expensive at final margin) |
| 5.3 | 7366 | 0.00 | missing | Default-Off warning + STATIC_MAX_LEVEL note | [unscheduled bbox exact=11/11] crate-doc body in src/lib.rs (11 atoms, too expensive at final margin) |
| 5.6 | 8318 | 0.00 | missing | RecordBuilder method index | [unscheduled bbox exact=12/12] impl method sigs in src/lib.rs (23 atoms, too expensive at final margin) |
| 5.7 | 8407 | 0.00 | missing | MetadataBuilder method index | [unscheduled bbox exact=4/4] impl method sigs in src/lib.rs (7 atoms, too expensive at final margin) |
| 5.8 | 8659 | 0.00 | missing | Logger blanket impls (&T, Box, Arc) | [unscheduled bbox exact=18/21] impl method sigs in src/lib.rs (26 atoms, too expensive at final margin) |
| 5.14 | 9861 | 0.00 | missing | logger() global accessor | [unscheduled bbox exact=6/7] pub item body at src/lib.rs:1581 body 1590 (6 atoms, discovered unscheduled) |

### wrong-slice / granularity

| id | ns_t | credit | status | descriptor | candidate hint |
|----|-----:|-------:|:-------|:-----------|:---------------|
| 2.2 | 831 | 0.00 | missing | Macro names (src/macros.rs) | [scheduled bbox exact=1/10] macro_export body at src/macros.rs:75 (t=8159, 38 atoms) |
| 2.6 | 1531 | 0.22 | missing | Logger installation entry-point signatures | [scheduled bbox exact=0/14] pub-item doc lede at src/lib.rs:1396 (t=6020, 13 atoms) |
| 3.1 | 2481 | 0.24 | missing | Record struct + accessor signatures | [scheduled bbox exact=9/38] pub item at src/lib.rs:842 (t=1530, 9 atoms); better unscheduled exact=20/38: impl method sigs in src/lib.rs (26 atoms, too expensive at final margin) |
| 3.2 | 2587 | 0.40 | missing | Metadata struct + accessors | [scheduled bbox exact=4/10] pub item at src/lib.rs:1158 (t=1394, 4 atoms); better unscheduled exact=6/10: impl method sigs in src/lib.rs (7 atoms, too expensive at final margin) |
| 3.5 | 3082 | 0.00 | missing | Global state + ordering constants | [scheduled same-file] pub-item names surface in src/lib.rs (t=1336, 34 atoms) |
| 3.8 | 4415 | 0.00 | missing | __private_api log dispatcher | [scheduled bbox exact=11/55] pub item at src/__private_api.rs:84 (t=7383, 11 atoms) |
| 3.9 | 4715 | 0.00 | missing | set_logger_inner state transitions | [scheduled same-file] pub-item names surface in src/lib.rs (t=1336, 34 atoms) |
| 4.1 | 5085 | 0.33 | missing | kv module concept | [scheduled bbox exact=10/30] crate-doc lede in src/kv/mod.rs (t=240, 10 atoms); better unscheduled exact=19/30: crate-doc body in src/kv/mod.rs (19 atoms, too expensive at final margin) |
| 4.2 | 5290 | 0.00 | missing | kv module re-exports | [scheduled bbox exact=11/20] mod/use plumbing in src/kv/mod.rs (t=3785, 11 atoms) |
| 4.4 | 5612 | 0.62 | missing | kv::Source trait surface | [scheduled bbox exact=4/8] pub item at src/kv/source.rs:51 (t=6445, 36 atoms) |
| 4.5 | 5889 | 0.18 | missing | kv::Value capture constructors | [scheduled bbox exact=3/17] pub item at src/kv/value.rs:119 (t=940, 3 atoms) |
| 4.6 | 6053 | 0.58 | missing | kv::Key surface | [scheduled bbox exact=4/12] pub item at src/kv/key.rs:37 (t=596, 4 atoms) |
| 4.8 | 6552 | 0.00 | missing | kv::Value to_* primitive accessors | [scheduled same-file] pub item at src/kv/value.rs:462 (t=8999, 63 atoms) |
| 5.4 | 7693 | 0.00 | missing | Compile-time max_level_* conflict guards | [scheduled same-file] pub-item names surface in src/lib.rs (t=1336, 34 atoms) |
| 5.5 | 7986 | 0.00 | missing | FromStr impls for Level/LevelFilter | [scheduled bbox exact=0/24] pub item at src/lib.rs:636 (t=1669, 14 atoms); better unscheduled exact=6/24: impl method sigs in src/lib.rs (30 atoms, too expensive at final margin) |
| 5.9 | 8977 | 0.00 | missing | non-atomic AtomicUsize fallback | [scheduled bbox exact=2/29] mod/use plumbing in src/lib.rs (t=3344, 2 atoms); better unscheduled exact=7/29: impl method sigs in src/lib.rs (7 atoms, too expensive at final margin) |
| 5.12 | 9640 | 0.00 | missing | kv::Source impl matrix | [scheduled bbox exact=0/37] pub item at src/kv/source.rs:235 (t=642, 4 atoms) |
| 5.13 | 9784 | 0.20 | missing | kv::Error variants | [scheduled bbox exact=3/15] pub item at src/kv/error.rs:5 (t=151, 3 atoms) |

### no discovered candidate

| id | ns_t | credit | status | descriptor | candidate hint |
|----|-----:|-------:|:-------|:-----------|:---------------|
| 5.11 | 9234 | 0.00 | missing | Macro test-fn names (tests/macros.rs) | no discovered line candidate |

### fs/listing

| id | ns_t | credit | status | descriptor | candidate hint |
|----|-----:|-------:|:-------|:-----------|:---------------|
| 5.10 | 8998 | 0.00 | missing | tests/ + benches/ + harness listings | fs-only |

### mixed/unknown

| id | ns_t | credit | status | descriptor | candidate hint |
|----|-----:|-------:|:-------|:-----------|:---------------|
| 2.7 | 2033 | 0.00 | missing | log! macro shapes (4 forms) | [scheduled bbox exact=37/40] macro_export body at src/macros.rs:75 (t=8159, 37 atoms) |
| 4.7 | 6361 | 0.07 | missing | VisitValue trait method index | [scheduled bbox exact=14/14] pub item at src/kv/value.rs:462 (t=8999, 57 atoms) |
| 5.1 | 6964 | 0.00 | missing | Cargo features list | [scheduled bbox exact=28/33] [features] in Cargo.toml (t=5403, 28 atoms) |

Top wasted paths (off-NS at 3K): src/lib.rs (925t, 5 batches), Cargo.toml (128t, 1 batch), src/kv/mod.rs (89t, 1 batch), src/kv/value.rs (67t, 1 batch), README.md (53t, 1 batch)

## Walker waste rollup (by descriptor pattern)

| n | off_3k_total | pattern |
|--:|-------------:|:--------|
| 2 | 625 | pub item at src/lib.rs:<n> |

## Walker waste (walker_t ≤ 3000, off_3k ≥ 50)

| off_3k | off_3k_ratio | off_any | cost | walker_t | batch |
|-------:|-------------:|--------:|-----:|---------:|:------|
| 349 | 0.87 | 349 | 401 | 2150 | pub item at src/lib.rs:1249 |
| 276 | 1.00 | 0 | 276 | 2715 | pub item at src/lib.rs:1611 |
| 147 | 1.00 | 147 | 147 | 2568 | pub item body at src/lib.rs:1529 body 1530 |
| 128 | 0.70 | 128 | 184 | 302 | [package] in Cargo.toml |
| 90 | 1.00 | 90 | 90 | 1974 | pub item body at src/lib.rs:1375 body 1376 |
| 89 | 1.00 | 0 | 89 | 151 | crate-doc lede in src/kv/mod.rs |
| 67 | 1.00 | 67 | 67 | 969 | macro_export names across src/kv |
| 63 | 1.00 | 63 | 63 | 2991 | pub-item doc lede at src/lib.rs:1566 |
| 53 | 1.00 | 53 | 53 | 240 | headings outline in README.md |
