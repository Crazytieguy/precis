scores: Score(3000)=0.625 ns_rows≤3K=18/44 (reached=9 partial=2 missing=7)

## Per-budget scores

| B | A_B | I(B) | C(B) | compl(B) | Score(B) | walker_used |
|--:|----:|-----:|-----:|---------:|---------:|------------:|
| 1000 | 102 | 0.759 | 0.319 | 0.860 | 0.492 | 860 |
| 1442 | 139 | 0.781 | 0.394 | 0.748 | 0.554 | 1400 |
| 2080 | 195 | 0.774 | 0.409 | 0.733 | 0.562 | 1940 |
| 3000 | 246 | 0.834 | 0.468 | 0.806 | 0.625 | 2985 |
| 4327 | 380 | 0.789 | 0.318 | 0.745 | 0.501 | 3878 |
| 6240 | 557 | 0.749 | 0.252 | 0.652 | 0.435 | 6192 |
| 9000 | 794 | 0.716 | 0.231 | 0.636 | 0.407 | 7854 |

## Top opportunities

### Additive (close partial / missing rows)

| intervention | rows | gap@1k | gap@3k | gap@9k | evidence | top row ids |
|:-------------|-----:|-------:|-------:|-------:|:---------|:------------|
| split wrong-slice walker batches | 20 | 1.41 | 1.30 | 1.21 | nearby candidates have low exact atom overlap | 2.1, 2.6, 3.4, 2.9, 3.8, ... |
| add walker candidates for no-discovered rows | 9 | 0.55 | 0.55 | 0.55 | NS rows have no discovered line candidate | 2.13, 3.2, 5.4, 3.6, 4.2, ... |
| tune ranking for high-overlap unscheduled candidates | 4 | 0.38 | 0.38 | 0.38 | high-overlap candidates not in the schedule by T_max, exact total=106/111 | 2.14, 2.15, 2.11, 5.5 |

### Subtractive (suppress consistently off-NS batches)

| pattern | batches | freed@1k | freed@3k | freed@9k | evidence | top batch ids |
|:--------|--------:|---------:|---------:|---------:|:---------|:--------------|
| pub-item doc lede at src/lib.rs:<n> | 3 | 0 | 573 | 573 | off_3k=573 | pub-item doc lede at src/lib.rs:650, pub-item doc lede at src/lib.rs:468, pub-item doc lede at src/lib.rs:616 |
| crate-doc lede in src/lib.rs | 1 | 229 | 229 | 229 | off_3k=229 | crate-doc lede in src/lib.rs |
| README.md section #<n> | 2 | 66 | 190 | 1767 | off_3k=190 | README.md section #8, README.md section #0 |
| macro_export body at src/macros.rs:58 | 1 | 0 | 136 | 136 | off_3k=136 | macro_export body at src/macros.rs:58 |
| [package] in Cargo.toml | 1 | 100 | 100 | 100 | off_3k=100 | [package] in Cargo.toml |

Top missed paths (NS rows ≤ 3K): src/lib.rs (5 rows, 54 atoms), src/context.rs (1 row, 51 atoms), src/macros.rs (2 rows, 37 atoms), src/error.rs (1 row, 14 atoms)

## Arrival ledger by diagnosis

### ranking-recoverable

| id | ns_t | credit | status | descriptor | candidate hint |
|----|-----:|-------:|:-------|:-----------|:---------------|
| 2.11 | 1757 | 0.00 | missing | Display reprs: `{}` and `{:#}` | [unscheduled bbox exact=17/17] pub-item doc body at src/lib.rs:390 (17 atoms, too expensive at final margin) |
| 2.14 | 3026 | 0.00 | missing | Debug repr `{:?}` (default for `fn main`) | [unscheduled bbox exact=34/34] pub-item doc body at src/lib.rs:390 (34 atoms, too expensive at final margin) |
| 2.15 | 3399 | 0.00 | missing | Debug repr `{:#?}` + manual chain rendering | [unscheduled bbox exact=32/35] pub-item doc body at src/lib.rs:390 (32 atoms, too expensive at final margin) |
| 5.5 | 9528 | 0.04 | missing | __fancy_ensure! macro body | [scheduled bbox exact=2/25] macro_export names across src (t=819, 2 atoms); better unscheduled exact=23/25: macro_export body at src/ensure.rs:886 (23 atoms, discovered unscheduled) |

### wrong-slice / granularity

| id | ns_t | credit | status | descriptor | candidate hint |
|----|-----:|-------:|:-------|:-----------|:---------------|
| 2.1 | 424 | 0.58 | missing | lib.rs module declarations | [scheduled bbox exact=11/19] mod/use plumbing in src/lib.rs (t=1372, 11 atoms) |
| 2.2 | 458 | 0.75 | partial | Error struct definition | [scheduled bbox exact=3/4] pub item at src/lib.rs:390 (t=561, 3 atoms) |
| 2.4 | 541 | 0.60 | missing | Chain struct | [scheduled bbox exact=3/5] pub item at src/lib.rs:415 (t=580, 3 atoms) |
| 2.6 | 858 | 0.00 | missing | Error::* method signatures (locations) | [scheduled same-file] pub item at src/error.rs:934 (t=3308, 7 atoms) |
| 2.8 | 1130 | 0.33 | missing | Crate-level public items (locations) | [scheduled bbox exact=0/9] pub-item doc body at src/lib.rs:616 (t=9014, 102 atoms) |
| 2.9 | 1302 | 0.13 | missing | Macro export locations | [scheduled bbox exact=1/15] macro_export body at src/macros.rs:58 (t=1536, 11 atoms) |
| 3.1 | 3672 | 0.11 | missing | error.rs item locations | [scheduled bbox exact=1/19] pub item at src/error.rs:934 (t=3308, 7 atoms) |
| 3.3 | 4232 | 0.16 | missing | chain.rs item locations | [scheduled bbox exact=9/25] pub item at src/chain.rs:16 (t=3601, 9 atoms) |
| 3.4 | 4770 | 0.00 | missing | kind.rs tagged-dispatch types | [scheduled bbox exact=9/51] pub-item names surface in src/kind.rs (t=3674, 12 atoms) |
| 3.5 | 4950 | 0.00 | missing | wrapper.rs: MessageError / DisplayError / BoxedError | [scheduled bbox exact=3/14] pub-item names surface in src/wrapper.rs (t=3235, 6 atoms) |
| 3.7 | 5548 | 0.00 | missing | ptr.rs internal pointer newtypes | [scheduled bbox exact=8/34] pub-item names surface in src/ptr.rs (t=3035, 8 atoms) |
| 3.8 | 6006 | 0.03 | missing | backtrace.rs cfg landscape (locations) | [scheduled bbox exact=1/38] pub-item names surface in src/backtrace.rs (t=1400, 2 atoms) |
| 3.9 | 6235 | 0.00 | missing | nightly.rs locations + signatures | [scheduled bbox exact=6/16] pub-item names surface in src/nightly.rs (t=3763, 6 atoms) |
| 3.10 | 6577 | 0.17 | missing | ensure.rs orientation header (locations) | [scheduled bbox exact=3/29] macro_export names across src (t=819, 5 atoms) |
| 4.1 | 6817 | 0.00 | missing | tests/common + drop helpers | [scheduled bbox exact=3/25] impl method sigs in tests/drop/mod.rs (t=7587, 12 atoms) |
| 4.6 | 7952 | 0.00 | missing | test_autotrait + test_boxed + test_ffi + test_backtrace | [scheduled bbox exact=3/25] pub-item names surface in tests/test_ffi.rs (t=6971, 5 atoms) |
| 5.1 | 8313 | 0.06 | missing | ErrorImpl<E> layout + vtable() reader | [scheduled bbox exact=7/18] pub item at src/error.rs:934 (t=3308, 7 atoms) |
| 5.2 | 8514 | 0.00 | missing | Chain DoubleEndedIterator buffering | [scheduled same-file] pub item at src/chain.rs:16 (t=3601, 9 atoms) |
| 5.3 | 8742 | 0.00 | missing | kind.rs autoref-precedence note | [scheduled same-file] pub-item names surface in src/kind.rs (t=3674, 12 atoms) |
| 5.6 | 9798 | 0.00 | missing | ensure!: render `{msg} ({lhs} vs {rhs})` | [scheduled same-file] macro_export names across src (t=819, 6 atoms) |

### no discovered candidate

| id | ns_t | credit | status | descriptor | candidate hint |
|----|-----:|-------:|:-------|:-----------|:---------------|
| 2.13 | 2558 | 0.00 | missing | Context impls for Result and Option | no discovered line candidate |
| 3.2 | 3934 | 0.00 | missing | context.rs item locations | no discovered line candidate |
| 3.6 | 5232 | 0.00 | missing | fmt.rs: ErrorImpl::display body + debug/Indented locations | no discovered line candidate |
| 4.2 | 7104 | 0.00 | missing | test_context: Low/Mid/High chain helper + fns | no discovered line candidate |
| 4.3 | 7307 | 0.00 | missing | test_downcast / test_repr / test_convert fns (locations) | no discovered line candidate |
| 4.4 | 7450 | 0.00 | missing | test_macros + test_chain + test_source fns | no discovered line candidate |
| 4.5 | 7686 | 0.00 | missing | test_fmt: f/g/h chain + EXPECTED_* constants (locations) | no discovered line candidate |
| 5.4 | 9288 | 0.00 | missing | ErrorImpl::debug body | no discovered line candidate |
| 5.7 | 9946 | 0.00 | missing | build.rs cfg pipeline (rustc version + cfgs) | no discovered line candidate |

### fs/listing

| id | ns_t | credit | status | descriptor | candidate hint |
|----|-----:|-------:|:-------|:-----------|:---------------|
| 4.7 | 8056 | 0.00 | missing | tests/ui + tests/crate listings | fs-only |

### mixed/unknown

| id | ns_t | credit | status | descriptor | candidate hint |
|----|-----:|-------:|:-------|:-----------|:---------------|
| 2.12 | 1979 | 0.91 | partial | anyhow! macro body | [scheduled bbox exact=20/22] macro_export body at src/macros.rs:204 (t=2160, 20 atoms) |

Top wasted paths (off-NS at 3K): src/lib.rs (877t, 5 batches), README.md (253t, 3 batches), src/macros.rs (136t, 1 batch), Cargo.toml (100t, 1 batch), src/ensure.rs (58t, 1 batch), src/ptr.rs (50t, 1 batch)

## Walker waste rollup (by descriptor pattern)

| n | off_3k_total | pattern |
|--:|-------------:|:--------|
| 3 | 573 | pub-item doc lede at src/lib.rs:<n> |
| 2 | 190 | README.md section #<n> |

## Walker waste (walker_t ≤ 3000, off_3k ≥ 50)

| off_3k | off_3k_ratio | off_any | cost | walker_t | batch |
|-------:|-------------:|--------:|-----:|---------:|:------|
| 286 | 1.00 | 286 | 286 | 2485 | pub-item doc lede at src/lib.rs:650 |
| 229 | 0.84 | 229 | 274 | 860 | crate-doc lede in src/lib.rs |
| 222 | 1.00 | 222 | 222 | 2206 | pub-item doc lede at src/lib.rs:468 |
| 136 | 1.00 | 136 | 136 | 1400 | macro_export body at src/macros.rs:58 |
| 124 | 1.00 | 124 | 124 | 2771 | README.md section #8 |
| 100 | 0.60 | 100 | 167 | 176 | [package] in Cargo.toml |
| 75 | 0.43 | 75 | 173 | 1199 | mod/use plumbing in src/lib.rs |
| 66 | 1.00 | 66 | 66 | 343 | README.md section #0 |
| 65 | 1.00 | 65 | 65 | 1134 | pub-item doc lede at src/lib.rs:616 |
| 63 | 1.00 | 63 | 63 | 69 | README headline in README.md |
| 108 | — | — | — | — | +2 more rows |
