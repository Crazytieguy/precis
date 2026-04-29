scores: Score(3000)=0.647 ns_rows≤3K=18/34 (reached=11 partial=0 missing=7)

## Per-budget scores

| B | A_B | I(B) | C(B) | Score(B) | walker_used |
|--:|----:|-----:|-----:|---------:|------------:|
| 1000 | 66 | 0.707 | 0.212 | 0.387 | 815 |
| 1442 | 81 | 0.679 | 0.173 | 0.342 | 815 |
| 2080 | 125 | 0.810 | 0.521 | 0.650 | 2055 |
| 3000 | 207 | 0.794 | 0.527 | 0.647 | 2854 |
| 4327 | 299 | 0.758 | 0.430 | 0.571 | 4213 |
| 6240 | 458 | 0.724 | 0.365 | 0.514 | 6232 |
| 9000 | 667 | 0.694 | 0.304 | 0.460 | 8832 |

## Verdict

Verdict: ranking-race bound
Likely primary lever: raise high-overlap discovered candidates over competing batches
Evidence: 8 ranking-recoverable (gap@3k=1.29), 12 wrong-slice/granularity (gap@3k=1.15), 1 no-discovered (gap@3k=0.03)
Secondary intervention: split wrong-slice batches for 12 rows
Top rows: 1.4, 2.4, 3.4, 4.2, 3.5, ...

## Top opportunities

_`gap@B` is a non-additive priority score: `Σ over atoms in row: (1 − damped_credit(a)) / rank(a)` evaluated at budget B's walker state. Approximates how much closing the row would lift `Score(B)` (via the Importance numerator); not an exact delta. `gap@3k` is the primary sort key. `gap@1k` and `gap@9k` show how the same intervention scales across the budget vector — gap is monotone non-increasing in B (walker has more budget at higher B). Rows can overlap between opportunities; sums are upper bounds on Score(B) impact, not additive estimates._

| intervention | rows | gap@1k | gap@3k | gap@9k | evidence | top row ids |
|:-------------|-----:|-------:|-------:|-------:|:---------|:------------|
| tune ranking for high-overlap unscheduled candidates | 8 | 1.29 | 1.29 | 1.29 | high-overlap candidates not in the schedule by T_max, exact total=203/232 | 1.4, 2.4, 3.4, 4.2, 3.5, ... |
| split wrong-slice walker batches | 12 | 1.16 | 1.15 | 0.93 | nearby candidates have low exact atom overlap | 2.10, 2.11, 2.6, 3.8, 4.1, ... |
| add walker candidates for no-discovered rows | 1 | 0.03 | 0.03 | 0.03 | NS rows have no discovered line candidate | 5.4 |

## Diagnosis rollup

| diagnosis | rows | missing | partial | likely lever |
|:----------|-----:|--------:|--------:|:-------------|
| ranking-recoverable | 8 | 8 | 0 | value/ranking |
| wrong-slice / granularity | 12 | 11 | 1 | walker granularity / wrong slice |
| no discovered candidate | 1 | 1 | 0 | walker coverage or predecessor-gated emit |
| mixed/unknown | 1 | 1 | 0 | inspect row |

## Loss reason rollup (ranking-recoverable rows)

| loss reason | rows | gap@3k | likely lever |
|:------------|-----:|-------:|:-------------|
| too expensive at final margin | 8 | 1.29 | tune ranking |

Candidate hint kinds: scheduled bbox=17, unscheduled bbox=3, scheduled same-file=1, no discovered candidate=1 _(candidates are walker batches discovered this run; descendants behind unscheduled predecessors may not be present, so `no discovered candidate` isn't proof that no emit path exists)._

## Exact atom overlap rollup (bbox hints)

| kind | status | exact_overlap | rows |
|:-----|:-------|:--------------|-----:|
| scheduled bbox | missing | low | 15 |
| scheduled bbox | missing | full | 1 |
| scheduled bbox | partial | low | 1 |
| unscheduled bbox | missing | high | 3 |

## Arrival ledger by diagnosis

### ranking-recoverable

| id | exp_t | credit | comp | status | descriptor | candidate hint |
|----|------:|-------:|-----:|:-------|:-----------|:--------------------|
| 1.4 | 252 | 0.00 | 0.00 | missing | README v2 lede — perf note + sdscatfmt headline | [unscheduled bbox exact=7/8] README.md section #0 (7 atoms, too expensive at final margin) |
| 2.4 | 657 | 0.00 | 0.00 | missing | Cardinal usage rule — must reassign return value | [unscheduled bbox exact=9/11] README.md section #0 (9 atoms, too expensive at final margin) |
| 3.4 | 3492 | 0.06 | 0.00 | missing | How SDS strings work — design narrative + ASCII diagram | [scheduled bbox exact=2/35] headings outline in README.md (t=815, 2 atoms); better unscheduled exact=28/35: README.md section #0 (28 atoms, too expensive at final margin) |
| 3.5 | 3896 | 0.00 | 0.00 | missing | Preallocation + SDS_MAX_PREALLOC growth strategy | [unscheduled bbox exact=22/26] README.md section #0 (22 atoms, too expensive at final margin) |
| 3.6 | 4166 | 0.10 | 0.01 | missing | Zero-copy append idiom | [scheduled bbox exact=2/20] headings outline in README.md (t=815, 2 atoms); better unscheduled exact=16/20: README.md section #0 (16 atoms, too expensive at final margin) |
| 3.9 | 5456 | 0.03 | 0.00 | missing | Outdated v1 internals diagram in README | [scheduled bbox exact=1/35] headings outline in README.md (t=815, 1 atoms); better unscheduled exact=29/35: README.md section #0 (29 atoms, too expensive at final margin) |
| 4.2 | 6852 | 0.00 | 0.00 | missing | sdsnewlen body — canonical allocator (no doc-comment) | [scheduled bbox exact=2/57] c decl at sds.c:89 (t=6220, 2 atoms); better unscheduled exact=54/57: c decl body at sds.c:89 (54 atoms, too expensive at final margin) |
| 5.2 | 9412 | 0.00 | 0.00 | missing | sdsIncrLen body — zero-copy idiom's other half (no doc-comment) | [scheduled bbox exact=2/40] c decl at sds.c:334 (t=6220, 2 atoms); better unscheduled exact=38/40: c decl body at sds.c:334 (38 atoms, too expensive at final margin) |

### wrong-slice / granularity

| id | exp_t | credit | comp | status | descriptor | candidate hint |
|----|------:|-------:|-----:|:-------|:-----------|:--------------------|
| 2.6 | 979 | 0.23 | 0.38 | missing | Public fn declarations — printf family (sdscatvprintf/printf/fmt) | [scheduled bbox exact=3/9] c decl names surface in sds.h (t=1919, 3 atoms) |
| 2.10 | 2230 | 0.12 | 0.02 | missing | Canonical Hello World | [scheduled bbox exact=3/24] headings outline in README.md (t=815, 3 atoms); better unscheduled exact=17/24: README.md section #0 (17 atoms, too expensive at final margin) |
| 2.11 | 2584 | 0.21 | 0.07 | missing | Embedding + allocator-swap instructions | [scheduled bbox exact=6/28] headings outline in README.md (t=815, 6 atoms); better unscheduled exact=21/28: README.md section #0 (21 atoms, too expensive at final margin) |
| 2.12 | 2706 | 0.30 | 0.19 | missing | sdsalloc.h — full file | [scheduled bbox exact=6/10] c header banner in sdsalloc.h (t=4050, 6 atoms) |
| 3.1 | 2823 | 0.30 | 0.05 | missing | Error handling — NULL on OOM | [scheduled bbox exact=3/10] headings outline in README.md (t=815, 3 atoms); better unscheduled exact=7/10: README.md section #0 (7 atoms, too expensive at final margin) |
| 3.7 | 4516 | 0.00 | 0.00 | missing | sdsReqType + sdsHdrSize — internal dispatch helpers | [scheduled same-file] c decl names surface in sds.c (t=6220, 83 atoms) |
| 3.8 | 4974 | 0.05 | 0.08 | missing | sds.h inline accessors — sdslen + sdsavail | [scheduled bbox exact=23/42] c decl body at sds.h:104 (t=5056, 23 atoms) |
| 4.1 | 6189 | 0.00 | 0.00 | missing | sdsMakeRoomFor body — growth/realloc engine | [scheduled bbox exact=6/51] c decl doc at sds.c:204 (t=8114, 6 atoms); better unscheduled exact=38/51: c decl body at sds.c:204 (38 atoms, too expensive at final margin) |
| 4.4 | 7347 | 0.00 | 0.00 | missing | Constructor family + sdsfree (short bodies) | [scheduled bbox exact=8/22] c decl names surface in sds.c (t=6220, 8 atoms) |
| 4.5 | 8081 | 0.02 | 0.00 | missing | README — Concatenating strings + sdsgrowzero | [scheduled bbox exact=1/64] headings outline in README.md (t=815, 1 atoms); better unscheduled exact=49/64: README.md section #0 (49 atoms, too expensive at final margin) |
| 5.1 | 8792 | 0.00 | 0.00 | missing | Append/copy primitive bodies (sdscatlen/cat/catsds/cpylen/cpy) | [scheduled bbox exact=10/50] c decl names surface in sds.c (t=6220, 10 atoms) |
| 5.3 | 9663 | 0.74 | 0.91 | partial | testhelp.h — minimal test framework macros | [scheduled bbox exact=8/19] c decl at testhelp.h:48 (t=416, 8 atoms) |

### no discovered candidate

| id | exp_t | credit | comp | status | descriptor | candidate hint |
|----|------:|-------:|-----:|:-------|:-----------|:--------------------|
| 5.4 | 9909 | 0.00 | 0.00 | missing | Makefile + Changelog | no discovered line candidate |

### mixed/unknown

| id | exp_t | credit | comp | status | descriptor | candidate hint |
|----|------:|-------:|-----:|:-------|:-----------|:--------------------|
| 4.3 | 7067 | 0.00 | 0.00 | missing | sdscatfmt — fast subset of printf, format spec list | [scheduled bbox exact=16/16] c decl doc at sds.c:616 (t=9880, 16 atoms) |

## Walker waste, primary-actionable (first_t ≤ 3000, off-NS spend ≥ 50)

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 534 | 1.00 | 534 | 3388 | c header banner in testhelp.h |
| 493 | 1.00 | 493 | 2854 | c header banner in sds.h |
| 129 | 0.32 | 399 | 815 | headings outline in README.md |
| 99 | 0.09 | 1104 | 1919 | c decl names surface in sds.h |

## Walker waste, late (first_t > 3000, off-NS spend ≥ 50) — higher-budget calibration only

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 668 | 0.73 | 914 | 6220 | c decl names surface in sds.c |
| 493 | 0.85 | 579 | 4050 | c header banner in sdsalloc.h |
| 250 | 1.00 | 250 | 5306 | c decl body at sds.h:154 |
| 235 | 1.00 | 235 | 4814 | c decl body at sds.h:130 |
| 215 | 1.00 | 215 | 9665 | c decl doc at sds.c:591 |
| 211 | 1.00 | 211 | 9450 | c decl doc at sds.c:89 |
| 209 | 1.00 | 209 | 9239 | c decl doc at sds.c:835 |
| 203 | 1.00 | 203 | 4579 | c decl body at sds.h:197 |
| 198 | 1.00 | 198 | 9030 | c decl doc at sds.c:184 |
| 190 | 1.00 | 190 | 8832 | c decl doc at sds.c:756 |
| 1548 | — | — | — | +15 more rows |
