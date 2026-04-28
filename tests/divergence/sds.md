scores: Sim=0.446 Reached=15/34 Early=2 Late=10 Partial=3 Missing=16 Used=9976/10000

## Verdict

Verdict: budget-pressure bound
Likely primary lever: free final budget / demote late low-value spend
Evidence: 8 ranking-recoverable (w×gap=2.12), 10 wrong-slice/granularity (w×gap=1.32), 1 no-discovered (w×gap=0.01)
Secondary intervention: split wrong-slice batches for 10 rows
Loss reasons: 0 predecessor-gated, 8 too-expensive, 0 discovered-unscheduled
Top rows: 1.4, 2.4, 3.4, 3.5, 3.6, ...
Note: likely lever is heuristic; verify `Sim` moves, not just bucket counts.

## Top opportunities

_`w(t)×gap` is a non-additive priority score: Σ exp(-exp_t/τ) × (1 - credit) per row, τ=2000. Same time weighting and credit gap as Sim, but rows can overlap between opportunities — sums across rows are an upper bound on Sim impact, not an additive estimate._

| intervention | rows | w(t)×gap | bands ≤3k/≤6k/total | evidence | top row ids |
|:-------------|-----:|---------:|:----------------------|:---------|:------------|
| free final budget / demote late waste | 8 | 2.12 | 2/6/8 | high-overlap candidates exceed final remaining budget, exact total=203/232 | 1.4, 2.4, 3.4, 3.5, 3.6, ... |
| split wrong-slice walker batches | 10 | 1.32 | 4/5/10 | nearby candidates have low exact atom overlap | 2.6, 2.10, 2.11, 3.1, 3.7, ... |
| add walker candidates for no-discovered rows | 1 | 0.01 | 0/0/1 | NS rows have no discovered line candidate | 5.4 |

Tiers: 1=3/4 reached, 0 partial, 1 missing, avg=0.75; 2=8/12 reached, 0 partial, 4 missing, avg=0.71; 3=3/9 reached, 0 partial, 6 missing, avg=0.38; 4=1/5 reached, 1 partial, 3 missing, avg=0.37; 5=0/4 reached, 2 partial, 2 missing, avg=0.32

## Diagnosis rollup

| diagnosis | rows | missing | partial | timing | likely lever |
|:----------|-----:|--------:|--------:|-------:|:-------------|
| ranking-recoverable | 8 | 8 | 0 | 0 | value/ranking |
| wrong-slice / granularity | 10 | 7 | 3 | 0 | walker granularity / wrong slice |
| no discovered candidate | 1 | 1 | 0 | 0 | walker coverage or predecessor-gated emit |
| timing-only | 13 | 0 | 0 | 13 | usually no code change |

## Loss reason rollup (ranking-recoverable rows)

| loss reason | rows | w(t)×gap | likely lever |
|:------------|-----:|---------:|:-------------|
| too expensive at final margin | 8 | 2.12 | free final budget |

_Candidate coverage note: candidates are the walker batches discovered during this scheduled run; descendants behind unscheduled predecessors may not be present, so `no discovered candidate` is not proof that no walker emit path exists._
Candidate hint kinds: scheduled bbox=27, unscheduled bbox=3, scheduled same-file=1, no discovered candidate=1

## Exact atom overlap rollup (bbox hints)

| kind | status | exact_overlap | rows |
|:-----|:-------|:--------------|-----:|
| scheduled bbox | aligned | low | 1 |
| scheduled bbox | early | full | 2 |
| scheduled bbox | late | low | 2 |
| scheduled bbox | late | full | 8 |
| scheduled bbox | missing | low | 11 |
| scheduled bbox | partial | low | 3 |
| unscheduled bbox | missing | high | 3 |

## Arrival ledger by diagnosis

### ranking-recoverable

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.4 | 252 | — | — | 0.00 | missing | README v2 lede — perf note + sdscatfmt headline | [unscheduled bbox exact=7/8] README.md section #0 (7 atoms, too expensive at final margin) |
| 2.4 | 657 | — | — | 0.00 | missing | Cardinal usage rule — must reassign return value | [unscheduled bbox exact=9/11] README.md section #0 (9 atoms, too expensive at final margin) |
| 3.4 | 3492 | — | — | 0.06 | missing | How SDS strings work — design narrative + ASCII diagram | [scheduled bbox exact=2/35] headings outline in README.md (t=815, 2 atoms); better unscheduled exact=28/35: README.md section #0 (28 atoms, too expensive at final margin) |
| 3.5 | 3896 | — | — | 0.00 | missing | Preallocation + SDS_MAX_PREALLOC growth strategy | [unscheduled bbox exact=22/26] README.md section #0 (22 atoms, too expensive at final margin) |
| 3.6 | 4166 | — | — | 0.10 | missing | Zero-copy append idiom | [scheduled bbox exact=2/20] headings outline in README.md (t=815, 2 atoms); better unscheduled exact=16/20: README.md section #0 (16 atoms, too expensive at final margin) |
| 3.9 | 5456 | — | — | 0.03 | missing | Outdated v1 internals diagram in README | [scheduled bbox exact=1/35] headings outline in README.md (t=815, 1 atoms); better unscheduled exact=29/35: README.md section #0 (29 atoms, too expensive at final margin) |
| 4.2 | 6852 | — | — | 0.02 | missing | sdsnewlen body — canonical allocator (no doc-comment) | [scheduled bbox exact=2/57] c decl at sds.c:89 (t=6220, 2 atoms); better unscheduled exact=54/57: c decl body at sds.c:89 (54 atoms, too expensive at final margin) |
| 5.2 | 9412 | — | — | 0.03 | missing | sdsIncrLen body — zero-copy idiom's other half (no doc-comment) | [scheduled bbox exact=2/40] c decl at sds.c:334 (t=6220, 2 atoms); better unscheduled exact=38/40: c decl body at sds.c:334 (38 atoms, too expensive at final margin) |

### wrong-slice / granularity

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 2.6 | 979 | — | — | 0.23 | missing | Public fn declarations — printf family (sdscatvprintf/printf/fmt) | [scheduled bbox exact=3/9] c decl names surface in sds.h (t=1919, 3 atoms) |
| 2.10 | 2230 | — | — | 0.12 | missing | Canonical Hello World | [scheduled bbox exact=3/24] headings outline in README.md (t=815, 3 atoms); better unscheduled exact=17/24: README.md section #0 (17 atoms, too expensive at final margin) |
| 2.11 | 2584 | — | — | 0.21 | missing | Embedding + allocator-swap instructions | [scheduled bbox exact=6/28] headings outline in README.md (t=815, 6 atoms); better unscheduled exact=21/28: README.md section #0 (21 atoms, too expensive at final margin) |
| 3.1 | 2823 | — | — | 0.30 | missing | Error handling — NULL on OOM | [scheduled bbox exact=3/10] headings outline in README.md (t=815, 3 atoms); better unscheduled exact=7/10: README.md section #0 (7 atoms, too expensive at final margin) |
| 3.7 | 4516 | — | — | 0.00 | missing | sdsReqType + sdsHdrSize — internal dispatch helpers | [scheduled same-file] c decl names surface in sds.c (t=6220, 83 atoms) |
| 4.1 | 6189 | — | — | 0.14 | missing | sdsMakeRoomFor body — growth/realloc engine | [scheduled bbox exact=6/51] c decl doc at sds.c:204 (t=8114, 6 atoms); better unscheduled exact=38/51: c decl body at sds.c:204 (38 atoms, too expensive at final margin) |
| 4.4 | 7347 | — | — | 0.68 | partial | Constructor family + sdsfree (short bodies) | [scheduled bbox exact=8/22] c decl names surface in sds.c (t=6220, 8 atoms) |
| 4.5 | 8081 | — | — | 0.02 | missing | README — Concatenating strings + sdsgrowzero | [scheduled bbox exact=1/64] headings outline in README.md (t=815, 1 atoms); better unscheduled exact=49/64: README.md section #0 (49 atoms, too expensive at final margin) |
| 5.1 | 8792 | — | — | 0.50 | partial | Append/copy primitive bodies (sdscatlen/cat/catsds/cpylen/cpy) | [scheduled bbox exact=10/50] c decl names surface in sds.c (t=6220, 10 atoms) |
| 5.3 | 9663 | — | — | 0.74 | partial | testhelp.h — minimal test framework macros | [scheduled bbox exact=8/19] c decl at testhelp.h:48 (t=416, 8 atoms) |

### no discovered candidate

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 5.4 | 9909 | — | — | 0.00 | missing | Makefile + Changelog | no discovered line candidate |

### timing-only

| id | exp_t | reached_t | delta_t | credit | status | descriptor | candidate hint |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.2 | 39 | 220 | +181 | 1.00 | late | README title | [scheduled bbox exact=1/1] README headline in README.md (t=220, 1 atoms) |
| 1.3 | 128 | 220 | +92 | 1.00 | late | README v2 lede — first paragraph (binary-compat warning) | [scheduled bbox exact=4/4] README headline in README.md (t=220, 4 atoms) |
| 2.1 | 262 | 1919 | +1657 | 1.00 | late | sds typedef — sds IS char* | [scheduled bbox exact=1/1] c decl at sds.h:43 (t=1919, 1 atoms) |
| 2.2 | 293 | 1919 | +1626 | 1.00 | late | SDS_MAX_PREALLOC + SDS_NOINIT constants | [scheduled bbox exact=2/2] c decl names surface in sds.h (t=1919, 2 atoms) |
| 2.3 | 469 | 1919 | +1450 | 1.00 | late | SDS_TYPE_* tag constants + SDS_HDR macros | [scheduled bbox exact=10/10] c decl names surface in sds.h (t=1919, 10 atoms) |
| 2.5 | 847 | 1919 | +1072 | 1.00 | late | Public fn declarations — high-level API (creation/length/free/concat/copy) | [scheduled bbox exact=11/11] c decl names surface in sds.h (t=1919, 11 atoms) |
| 2.7 | 1276 | 1919 | +643 | 1.00 | late | Public fn declarations — utility fns (trim/range/cmp/split/case/repr/join) | [scheduled bbox exact=15/15] c decl names surface in sds.h (t=1919, 15 atoms) |
| 2.8 | 1487 | 2361 | +874 | 1.00 | late | Public fn declarations — low-level + allocator-export API | [scheduled bbox exact=9/14] c decl names surface in sds.h (t=1919, 9 atoms) |
| 2.12 | 2706 | 4050 | +1344 | 0.90 | late | sdsalloc.h — full file | [scheduled bbox exact=6/10] c header banner in sdsalloc.h (t=4050, 6 atoms) |
| 3.2 | 2912 | 815 | -2097 | 1.00 | early | README section TOC — first half (intro through copying) | [scheduled bbox exact=10/10] headings outline in README.md (t=815, 31 atoms) |
| 3.3 | 3011 | 815 | -2196 | 1.00 | early | README section TOC — second half (quoting through credits) | [scheduled bbox exact=11/11] headings outline in README.md (t=815, 40 atoms) |
| 3.8 | 4974 | 5056 | +82 | 0.93 | aligned | sds.h inline accessors — sdslen + sdsavail | [scheduled bbox exact=23/42] c decl body at sds.h:104 (t=5056, 23 atoms) |
| 4.3 | 7067 | 9880 | +2813 | 1.00 | late | sdscatfmt — fast subset of printf, format spec list | [scheduled bbox exact=16/16] c decl doc at sds.c:616 (t=9880, 16 atoms) |

## Walker waste (off-NS token spend ≥ 50)

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 668 | 0.73 | 914 | 6220 | c decl names surface in sds.c |
| 534 | 1.00 | 534 | 3388 | c header banner in testhelp.h |
| 493 | 1.00 | 493 | 2854 | c header banner in sds.h |
| 493 | 0.85 | 579 | 4050 | c header banner in sdsalloc.h |
| 250 | 1.00 | 250 | 5306 | c decl body at sds.h:154 |
| 235 | 1.00 | 235 | 4814 | c decl body at sds.h:130 |
| 215 | 1.00 | 215 | 9665 | c decl doc at sds.c:591 |
| 211 | 1.00 | 211 | 9450 | c decl doc at sds.c:89 |
| 209 | 1.00 | 209 | 9239 | c decl doc at sds.c:835 |
| 203 | 1.00 | 203 | 4579 | c decl body at sds.h:197 |
| 2164 | — | — | — | +19 more rows |
