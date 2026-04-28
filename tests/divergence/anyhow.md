scores: Sim=0.412 Reached=14/44 Early=0 Late=8 Partial=5 Missing=25 Used=9944/10000

## Verdict

Verdict: wrong-slice bound
Likely primary lever: split walker batches to match NS semantic slices
Evidence: 1 ranking-recoverable (w×gap=0.01), 20 wrong-slice/granularity (w×gap=2.86), 9 no-discovered (w×gap=0.61)
Secondary intervention: free final budget for 1 too-expensive candidate
Loss reasons: 0 predecessor-gated, 1 too-expensive, 0 discovered-unscheduled
Top rows: 2.6, 2.9, 2.8, 2.1, 2.4, ...
Note: likely lever is heuristic; verify `Sim` moves, not just bucket counts.

## Top opportunities

_`w(t)×gap` is a non-additive priority score: Σ exp(-exp_t/τ) × (1 - credit) per row, τ=2000. Same time weighting and credit gap as Sim, but rows can overlap between opportunities — sums across rows are an upper bound on Sim impact, not an additive estimate._

| intervention | rows | w(t)×gap | bands ≤3k/≤6k/total | evidence | top row ids |
|:-------------|-----:|---------:|:----------------------|:---------|:------------|
| split wrong-slice walker batches | 20 | 2.86 | 6/11/20 | nearby candidates have low exact atom overlap | 2.6, 2.9, 2.8, 2.1, 2.4, ... |
| add walker candidates for no-discovered rows | 9 | 0.61 | 1/3/9 | NS rows have no discovered line candidate | 2.13, 3.2, 3.6, 4.2, 4.3, ... |
| free final budget / demote late waste | 1 | 0.01 | 0/0/1 | high-overlap candidates exceed final remaining budget, exact total=23/25 | 5.5 |

Tiers: 1=5/5 reached, 0 partial, 0 missing, avg=1.00; 2=8/15 reached, 3 partial, 4 missing, avg=0.67; 3=0/10 reached, 1 partial, 9 missing, avg=0.23; 4=1/7 reached, 1 partial, 5 missing, avg=0.23; 5=0/7 reached, 0 partial, 7 missing, avg=0.06

## Diagnosis rollup

| diagnosis | rows | missing | partial | timing | likely lever |
|:----------|-----:|--------:|--------:|-------:|:-------------|
| ranking-recoverable | 1 | 1 | 0 | 0 | value/ranking |
| wrong-slice / granularity | 20 | 15 | 5 | 0 | walker granularity / wrong slice |
| no discovered candidate | 9 | 9 | 0 | 0 | walker coverage or predecessor-gated emit |
| timing-only | 11 | 0 | 0 | 11 | usually no code change |

## Loss reason rollup (ranking-recoverable rows)

| loss reason | rows | w(t)×gap | likely lever |
|:------------|-----:|---------:|:-------------|
| too expensive at final margin | 1 | 0.01 | free final budget |

_Candidate coverage note: candidates are the walker batches discovered during this scheduled run; descendants behind unscheduled predecessors may not be present, so `no discovered candidate` is not proof that no walker emit path exists._
Candidate hint kinds: scheduled bbox=26, scheduled same-file=4, fs-only=2, no discovered candidate=9

## Exact atom overlap rollup (bbox hints)

| kind | status | exact_overlap | rows |
|:-----|:-------|:--------------|-----:|
| scheduled bbox | aligned | low | 1 |
| scheduled bbox | aligned | high | 2 |
| scheduled bbox | late | high | 1 |
| scheduled bbox | late | full | 5 |
| scheduled bbox | missing | none | 1 |
| scheduled bbox | missing | low | 11 |
| scheduled bbox | partial | low | 5 |

## Arrival ledger by diagnosis

### ranking-recoverable

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 5.5 | 9528 | — | — | 0.04 | missing | __fancy_ensure! macro body | [scheduled bbox exact=2/25] macro_export names across src (t=811, 2 atoms); better unscheduled exact=23/25: macro_export body at src/ensure.rs:886 (23 atoms, too expensive at final margin) |

### wrong-slice / granularity

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 2.1 | 424 | — | — | 0.58 | partial | lib.rs module declarations | [scheduled bbox exact=11/19] mod/use plumbing in src/lib.rs (t=1364, 11 atoms) |
| 2.2 | 458 | — | — | 0.75 | partial | Error struct definition | [scheduled bbox exact=3/4] pub item at src/lib.rs:390 (t=561, 3 atoms) |
| 2.4 | 541 | — | — | 0.60 | partial | Chain struct | [scheduled bbox exact=3/5] pub item at src/lib.rs:415 (t=580, 3 atoms) |
| 2.6 | 858 | — | — | 0.00 | missing | Error::* method signatures (locations) | [scheduled same-file] pub item at src/error.rs:934 (t=3210, 7 atoms) |
| 2.8 | 1130 | — | — | 0.33 | missing | Crate-level public items (locations) | [scheduled bbox exact=0/9] pub-item doc body at src/lib.rs:616 (t=8897, 102 atoms) |
| 2.9 | 1302 | — | — | 0.13 | missing | Macro export locations | [scheduled bbox exact=1/15] macro_export body at src/macros.rs:58 (t=1528, 11 atoms) |
| 3.1 | 3672 | — | — | 0.11 | missing | error.rs item locations | [scheduled bbox exact=1/19] pub item at src/error.rs:934 (t=3210, 7 atoms) |
| 3.3 | 4232 | — | — | 0.48 | missing | chain.rs item locations | [scheduled bbox exact=9/25] pub item at src/chain.rs:16 (t=3503, 9 atoms) |
| 3.4 | 4770 | — | — | 0.41 | missing | kind.rs tagged-dispatch types | [scheduled bbox exact=9/51] pub-item names surface in src/kind.rs (t=3576, 12 atoms) |
| 3.5 | 4950 | — | — | 0.21 | missing | wrapper.rs: MessageError / DisplayError / BoxedError | [scheduled bbox exact=3/14] pub-item names surface in src/wrapper.rs (t=3137, 6 atoms) |
| 3.7 | 5548 | — | — | 0.68 | partial | ptr.rs internal pointer newtypes | [scheduled bbox exact=8/34] pub-item names surface in src/ptr.rs (t=2937, 8 atoms) |
| 3.8 | 6006 | — | — | 0.03 | missing | backtrace.rs cfg landscape (locations) | [scheduled bbox exact=1/38] pub-item names surface in src/backtrace.rs (t=1392, 2 atoms) |
| 3.9 | 6235 | — | — | 0.19 | missing | nightly.rs locations + signatures | [scheduled bbox exact=6/16] pub-item names surface in src/nightly.rs (t=3665, 6 atoms) |
| 3.10 | 6577 | — | — | 0.24 | missing | ensure.rs orientation header (locations) | [scheduled bbox exact=3/29] macro_export names across src (t=811, 5 atoms) |
| 4.1 | 6817 | — | — | 0.52 | partial | tests/common + drop helpers | [scheduled bbox exact=3/25] impl method sigs in tests/drop/mod.rs (t=7629, 12 atoms) |
| 4.6 | 7952 | — | — | 0.12 | missing | test_autotrait + test_boxed + test_ffi + test_backtrace | [scheduled bbox exact=3/25] pub-item names surface in tests/test_ffi.rs (t=7465, 5 atoms) |
| 5.1 | 8313 | — | — | 0.39 | missing | ErrorImpl<E> layout + vtable() reader | [scheduled bbox exact=7/18] pub item at src/error.rs:934 (t=3210, 7 atoms) |
| 5.2 | 8514 | — | — | 0.00 | missing | Chain DoubleEndedIterator buffering | [scheduled same-file] pub item at src/chain.rs:16 (t=3503, 9 atoms) |
| 5.3 | 8742 | — | — | 0.00 | missing | kind.rs autoref-precedence note | [scheduled same-file] pub-item names surface in src/kind.rs (t=3576, 12 atoms) |
| 5.6 | 9798 | — | — | 0.00 | missing | ensure!: render `{msg} ({lhs} vs {rhs})` | [scheduled same-file] macro_export names across src (t=811, 6 atoms) |

### no discovered candidate

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 2.13 | 2558 | — | — | 0.00 | missing | Context impls for Result and Option | no discovered line candidate |
| 3.2 | 3934 | — | — | 0.00 | missing | context.rs item locations | no discovered line candidate |
| 3.6 | 5232 | — | — | 0.00 | missing | fmt.rs: ErrorImpl::display body + debug/Indented locations | no discovered line candidate |
| 4.2 | 7104 | — | — | 0.00 | missing | test_context: Low/Mid/High chain helper + fns | no discovered line candidate |
| 4.3 | 7307 | — | — | 0.00 | missing | test_downcast / test_repr / test_convert fns (locations) | no discovered line candidate |
| 4.4 | 7450 | — | — | 0.00 | missing | test_macros + test_chain + test_source fns | no discovered line candidate |
| 4.5 | 7686 | — | — | 0.00 | missing | test_fmt: f/g/h chain + EXPECTED_* constants (locations) | no discovered line candidate |
| 5.4 | 9288 | — | — | 0.00 | missing | ErrorImpl::debug body | no discovered line candidate |
| 5.7 | 9946 | — | — | 0.00 | missing | build.rs cfg pipeline (rustc version + cfgs) | no discovered line candidate |

### timing-only

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.2 | 79 | 1126 | +1047 | 1.00 | late | Crate-doc lede | [scheduled bbox exact=3/3] crate-doc lede in src/lib.rs (t=1126, 3 atoms) |
| 1.3 | 128 | 458 | +330 | 1.00 | late | src/ listing | fs-only |
| 1.4 | 195 | 343 | +148 | 1.00 | late | Crate identity (name, edition, MSRV) | [scheduled bbox exact=6/6] [package] in Cargo.toml (t=343, 12 atoms) |
| 1.5 | 285 | 7137 | +6852 | 1.00 | late | tests/ listing | fs-only |
| 2.5 | 697 | 1702 | +1005 | 1.00 | late | Error vs Box<dyn Error>: three differences | [scheduled bbox exact=10/10] pub-item doc lede at src/lib.rs:390 (t=1702, 10 atoms) |
| 2.7 | 1036 | 733 | -303 | 0.92 | aligned | Context trait signature | [scheduled bbox exact=12/13] pub item at src/lib.rs:616 (t=733, 12 atoms) |
| 2.10 | 1542 | 1909 | +367 | 0.88 | aligned | Cargo features + dev-deps | [scheduled bbox exact=12/17] [dependencies] in Cargo.toml (t=1909, 12 atoms) |
| 2.11 | 1757 | 9929 | +8172 | 1.00 | late | Display reprs: `{}` and `{:#}` | [scheduled bbox exact=17/17] pub-item doc body at src/lib.rs:390 (t=9929, 17 atoms) |
| 2.12 | 1979 | 2152 | +173 | 0.91 | aligned | anyhow! macro body | [scheduled bbox exact=20/22] macro_export body at src/macros.rs:204 (t=2152, 20 atoms) |
| 2.14 | 3026 | 9929 | +6903 | 1.00 | late | Debug repr `{:?}` (default for `fn main`) | [scheduled bbox exact=34/34] pub-item doc body at src/lib.rs:390 (t=9929, 34 atoms) |
| 2.15 | 3399 | 9929 | +6530 | 0.91 | late | Debug repr `{:#?}` + manual chain rendering | [scheduled bbox exact=32/35] pub-item doc body at src/lib.rs:390 (t=9929, 32 atoms) |

## Walker waste rollup (by descriptor pattern)

| n | off_tokens_total | pattern |
|--:|-----------------:|:--------|
| 10 | 1767 | README.md section #<n> |
| 3 | 1435 | pub-item doc body at src/lib.rs:<n> |
| 3 | 573 | pub-item doc lede at src/lib.rs:<n> |

## Walker waste (off-NS token spend ≥ 50)

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 1564 | 1.00 | 1564 | 5229 | crate-doc body in src/lib.rs |
| 1160 | 1.00 | 1160 | 8897 | pub-item doc body at src/lib.rs:616 |
| 286 | 1.00 | 286 | 2763 | pub-item doc lede at src/lib.rs:650 |
| 259 | 1.00 | 259 | 5488 | README.md section #9 |
| 241 | 1.00 | 241 | 7047 | README.md section #2 |
| 229 | 0.84 | 274 | 1126 | crate-doc lede in src/lib.rs |
| 222 | 1.00 | 222 | 2420 | pub-item doc lede at src/lib.rs:468 |
| 215 | 1.00 | 215 | 6806 | README.md section #4 |
| 205 | 1.00 | 205 | 6591 | README.md section #5 |
| 200 | 1.00 | 200 | 3410 | README.md section #7 |
| 1578 | — | — | — | +15 more rows |
