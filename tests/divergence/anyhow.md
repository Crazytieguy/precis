scores: Score(3000)=0.548 ns_rows≤3K=18/44 (reached=8 partial=2 missing=8)

## Per-budget scores

| B | A_B | I(B) | C(B) | Score(B) | walker_used |
|--:|----:|-----:|-----:|---------:|------------:|
| 1000 | 102 | 0.759 | 0.319 | 0.492 | 860 |
| 1442 | 139 | 0.781 | 0.394 | 0.554 | 1400 |
| 2080 | 195 | 0.774 | 0.409 | 0.562 | 1940 |
| 3000 | 246 | 0.761 | 0.395 | 0.548 | 2992 |
| 4327 | 380 | 0.718 | 0.270 | 0.441 | 3706 |
| 6240 | 557 | 0.681 | 0.230 | 0.396 | 6228 |
| 9000 | 794 | 0.716 | 0.231 | 0.407 | 7854 |

## Verdict

Verdict: wrong-slice bound
Likely primary lever: split walker batches to match NS semantic slices
Evidence: 4 ranking-recoverable (gap@3k=0.38), 20 wrong-slice/granularity (gap@3k=1.30), 9 no-discovered (gap@3k=0.55)
Secondary intervention: free T_max budget for 3 too-expensive candidates
Top rows: 2.1, 2.6, 3.4, 2.9, 3.8, ...

## Top opportunities

_`gap@B` is a non-additive priority score: `Σ over atoms in row: (1 − damped_credit(a)) / rank(a)` evaluated at budget B's walker state. Approximates how much closing the row would lift `Score(B)` (via the Importance numerator); not an exact delta. `gap@3k` is the primary sort key. `gap@1k` and `gap@9k` show how the same intervention scales across the budget vector — gap is monotone non-increasing in B (walker has more budget at higher B). Rows can overlap between opportunities; sums are upper bounds on Score(B) impact, not additive estimates._

| intervention | rows | gap@1k | gap@3k | gap@9k | evidence | top row ids |
|:-------------|-----:|-------:|-------:|-------:|:---------|:------------|
| split wrong-slice walker batches | 20 | 1.41 | 1.30 | 1.21 | nearby candidates have low exact atom overlap | 2.1, 2.6, 3.4, 2.9, 3.8, ... |
| add walker candidates for no-discovered rows | 9 | 0.55 | 0.55 | 0.55 | NS rows have no discovered line candidate | 2.13, 3.2, 5.4, 3.6, 4.2, ... |
| finish partially-delivered NS batches | 5 | 0.54 | 0.43 | 0.41 | avg batch completion=0.48 | 2.1, 2.8, 3.7, 2.4, 2.2 |
| free T_max budget / demote late waste | 3 | 0.35 | 0.35 | 0.35 | high-overlap candidates exceed remaining budget at T_max (caveat: not 3K-budget — see below), exact total=83/86 | 2.14, 2.15, 2.11 |
| tune ranking for discovered unscheduled candidates | 1 | 0.03 | 0.03 | 0.03 | high-overlap candidates fit but did not win, exact total=23/25 | 5.5 |

## Diagnosis rollup

| diagnosis | rows | missing | partial | likely lever |
|:----------|-----:|--------:|--------:|:-------------|
| ranking-recoverable | 4 | 4 | 0 | value/ranking |
| wrong-slice / granularity | 20 | 19 | 1 | walker granularity / wrong slice |
| no discovered candidate | 9 | 9 | 0 | walker coverage or predecessor-gated emit |
| fs/listing | 2 | 2 | 0 | filesystem/listing value |
| mixed/unknown | 1 | 0 | 1 | inspect row |

## Loss reason rollup (ranking-recoverable rows)

| loss reason | rows | gap@3k | likely lever |
|:------------|-----:|-------:|:-------------|
| too expensive at final margin | 3 | 0.35 | free T_max budget |
| discovered unscheduled | 1 | 0.03 | tune ranking |

_Candidate coverage note: candidates are the walker batches discovered during this scheduled run; descendants behind unscheduled predecessors may not be present, so `no discovered candidate` is not proof that no walker emit path exists._
Candidate hint kinds: scheduled bbox=18, unscheduled bbox=3, scheduled same-file=4, fs-only=2, no discovered candidate=9

## Exact atom overlap rollup (bbox hints)

| kind | status | exact_overlap | rows |
|:-----|:-------|:--------------|-----:|
| scheduled bbox | missing | none | 1 |
| scheduled bbox | missing | low | 15 |
| scheduled bbox | partial | low | 1 |
| scheduled bbox | partial | high | 1 |
| unscheduled bbox | missing | high | 1 |
| unscheduled bbox | missing | full | 2 |

## Arrival ledger by diagnosis

### ranking-recoverable

| id | exp_t | credit | comp | status | descriptor | candidate hint |
|----|------:|-------:|-----:|:-------|:-----------|:--------------------|
| 2.11 | 1757 | 0.00 | 0.00 | missing | Display reprs: `{}` and `{:#}` | [unscheduled bbox exact=17/17] pub-item doc body at src/lib.rs:390 (17 atoms, too expensive at final margin) |
| 2.14 | 3026 | 0.00 | 0.00 | missing | Debug repr `{:?}` (default for `fn main`) | [unscheduled bbox exact=34/34] pub-item doc body at src/lib.rs:390 (34 atoms, too expensive at final margin) |
| 2.15 | 3399 | 0.00 | 0.00 | missing | Debug repr `{:#?}` + manual chain rendering | [unscheduled bbox exact=32/35] pub-item doc body at src/lib.rs:390 (32 atoms, too expensive at final margin) |
| 5.5 | 9528 | 0.04 | 0.04 | missing | __fancy_ensure! macro body | [scheduled bbox exact=2/25] macro_export names across src (t=819, 2 atoms); better unscheduled exact=23/25: macro_export body at src/ensure.rs:886 (23 atoms, discovered unscheduled) |

### wrong-slice / granularity

| id | exp_t | credit | comp | status | descriptor | candidate hint |
|----|------:|-------:|-----:|:-------|:-----------|:--------------------|
| 2.1 | 424 | 0.58 | 0.52 | missing | lib.rs module declarations | [scheduled bbox exact=11/19] mod/use plumbing in src/lib.rs (t=1372, 11 atoms) |
| 2.2 | 458 | 0.75 | 0.69 | partial | Error struct definition | [scheduled bbox exact=3/4] pub item at src/lib.rs:390 (t=561, 3 atoms) |
| 2.4 | 541 | 0.60 | 0.47 | missing | Chain struct | [scheduled bbox exact=3/5] pub item at src/lib.rs:415 (t=580, 3 atoms) |
| 2.6 | 858 | 0.00 | 0.00 | missing | Error::* method signatures (locations) | [scheduled same-file] pub item at src/error.rs:934 (t=3218, 7 atoms) |
| 2.8 | 1130 | 0.33 | 0.45 | missing | Crate-level public items (locations) | [scheduled bbox exact=0/9] pub-item doc body at src/lib.rs:616 (t=9014, 102 atoms) |
| 2.9 | 1302 | 0.13 | 0.10 | missing | Macro export locations | [scheduled bbox exact=1/15] macro_export body at src/macros.rs:58 (t=1536, 11 atoms) |
| 3.1 | 3672 | 0.11 | 0.12 | missing | error.rs item locations | [scheduled bbox exact=1/19] pub item at src/error.rs:934 (t=3218, 7 atoms) |
| 3.3 | 4232 | 0.16 | 0.14 | missing | chain.rs item locations | [scheduled bbox exact=9/25] pub item at src/chain.rs:16 (t=3511, 9 atoms) |
| 3.4 | 4770 | 0.00 | 0.00 | missing | kind.rs tagged-dispatch types | [scheduled bbox exact=9/51] pub-item names surface in src/kind.rs (t=3584, 12 atoms) |
| 3.5 | 4950 | 0.00 | 0.00 | missing | wrapper.rs: MessageError / DisplayError / BoxedError | [scheduled bbox exact=3/14] pub-item names surface in src/wrapper.rs (t=3145, 6 atoms) |
| 3.7 | 5548 | 0.34 | 0.29 | missing | ptr.rs internal pointer newtypes | [scheduled bbox exact=8/34] pub-item names surface in src/ptr.rs (t=2945, 8 atoms) |
| 3.8 | 6006 | 0.03 | 0.02 | missing | backtrace.rs cfg landscape (locations) | [scheduled bbox exact=1/38] pub-item names surface in src/backtrace.rs (t=1400, 2 atoms) |
| 3.9 | 6235 | 0.00 | 0.00 | missing | nightly.rs locations + signatures | [scheduled bbox exact=6/16] pub-item names surface in src/nightly.rs (t=3673, 6 atoms) |
| 3.10 | 6577 | 0.17 | 0.19 | missing | ensure.rs orientation header (locations) | [scheduled bbox exact=3/29] macro_export names across src (t=819, 5 atoms) |
| 4.1 | 6817 | 0.00 | 0.00 | missing | tests/common + drop helpers | [scheduled bbox exact=3/25] impl method sigs in tests/drop/mod.rs (t=7746, 12 atoms) |
| 4.6 | 7952 | 0.00 | 0.00 | missing | test_autotrait + test_boxed + test_ffi + test_backtrace | [scheduled bbox exact=3/25] pub-item names surface in tests/test_ffi.rs (t=7550, 5 atoms) |
| 5.1 | 8313 | 0.06 | 0.05 | missing | ErrorImpl<E> layout + vtable() reader | [scheduled bbox exact=7/18] pub item at src/error.rs:934 (t=3218, 7 atoms) |
| 5.2 | 8514 | 0.00 | 0.00 | missing | Chain DoubleEndedIterator buffering | [scheduled same-file] pub item at src/chain.rs:16 (t=3511, 9 atoms) |
| 5.3 | 8742 | 0.00 | 0.00 | missing | kind.rs autoref-precedence note | [scheduled same-file] pub-item names surface in src/kind.rs (t=3584, 12 atoms) |
| 5.6 | 9798 | 0.00 | 0.00 | missing | ensure!: render `{msg} ({lhs} vs {rhs})` | [scheduled same-file] macro_export names across src (t=819, 6 atoms) |

### no discovered candidate

| id | exp_t | credit | comp | status | descriptor | candidate hint |
|----|------:|-------:|-----:|:-------|:-----------|:--------------------|
| 2.13 | 2558 | 0.00 | 0.00 | missing | Context impls for Result and Option | no discovered line candidate |
| 3.2 | 3934 | 0.00 | 0.00 | missing | context.rs item locations | no discovered line candidate |
| 3.6 | 5232 | 0.00 | 0.00 | missing | fmt.rs: ErrorImpl::display body + debug/Indented locations | no discovered line candidate |
| 4.2 | 7104 | 0.00 | 0.00 | missing | test_context: Low/Mid/High chain helper + fns | no discovered line candidate |
| 4.3 | 7307 | 0.00 | 0.00 | missing | test_downcast / test_repr / test_convert fns (locations) | no discovered line candidate |
| 4.4 | 7450 | 0.00 | 0.00 | missing | test_macros + test_chain + test_source fns | no discovered line candidate |
| 4.5 | 7686 | 0.00 | 0.00 | missing | test_fmt: f/g/h chain + EXPECTED_* constants (locations) | no discovered line candidate |
| 5.4 | 9288 | 0.00 | 0.00 | missing | ErrorImpl::debug body | no discovered line candidate |
| 5.7 | 9946 | 0.00 | 0.00 | missing | build.rs cfg pipeline (rustc version + cfgs) | no discovered line candidate |

### fs/listing

| id | exp_t | credit | comp | status | descriptor | candidate hint |
|----|------:|-------:|-----:|:-------|:-----------|:--------------------|
| 1.5 | 285 | 0.00 | 0.00 | missing | tests/ listing | fs-only |
| 4.7 | 8056 | 0.00 | 0.00 | missing | tests/ui + tests/crate listings | fs-only |

### mixed/unknown

| id | exp_t | credit | comp | status | descriptor | candidate hint |
|----|------:|-------:|-----:|:-------|:-----------|:--------------------|
| 2.12 | 1979 | 0.91 | 0.87 | partial | anyhow! macro body | [scheduled bbox exact=20/22] macro_export body at src/macros.rs:204 (t=2160, 20 atoms) |

## Walker waste rollup (by descriptor pattern)

| n | off_tokens_total | pattern |
|--:|-----------------:|:--------|
| 10 | 1767 | README.md section #<n> |
| 3 | 1435 | pub-item doc body at src/lib.rs:<n> |
| 3 | 573 | pub-item doc lede at src/lib.rs:<n> |

## Walker waste (off-NS token spend ≥ 50)

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 1564 | 1.00 | 1564 | 5270 | crate-doc body in src/lib.rs |
| 1160 | 1.00 | 1160 | 9014 | pub-item doc body at src/lib.rs:616 |
| 286 | 1.00 | 286 | 2771 | pub-item doc lede at src/lib.rs:650 |
| 259 | 1.00 | 259 | 5529 | README.md section #9 |
| 241 | 1.00 | 241 | 7088 | README.md section #2 |
| 229 | 0.84 | 274 | 1134 | crate-doc lede in src/lib.rs |
| 222 | 1.00 | 222 | 2428 | pub-item doc lede at src/lib.rs:468 |
| 215 | 1.00 | 215 | 6847 | README.md section #4 |
| 205 | 1.00 | 205 | 6632 | README.md section #5 |
| 200 | 1.00 | 200 | 3418 | README.md section #7 |
| 1578 | — | — | — | +15 more rows |
