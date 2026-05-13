scores: Score(3000)=0.647 ns_rows≤3K=18/34 (reached=11 partial=0 missing=7)

## Per-budget scores

| B | A_B | I(B) | C(B) | compl(B) | Score(B) | walker_used |
|--:|----:|-----:|-----:|---------:|---------:|------------:|
| 1000 | 66 | 0.707 | 0.212 | 1.000 | 0.387 | 815 |
| 1442 | 81 | 0.679 | 0.173 | 1.000 | 0.342 | 815 |
| 2080 | 125 | 0.810 | 0.521 | 0.853 | 0.650 | 2055 |
| 3000 | 207 | 0.794 | 0.527 | 0.732 | 0.647 | 2854 |
| 4327 | 299 | 0.758 | 0.430 | 0.712 | 0.571 | 4213 |
| 6240 | 458 | 0.724 | 0.365 | 0.662 | 0.514 | 6232 |
| 9000 | 667 | 0.694 | 0.304 | 0.636 | 0.460 | 8832 |

## Top opportunities

### Additive (close partial / missing rows)

| intervention | rows | gap@1k | gap@3k | gap@9k | evidence | top row ids |
|:-------------|-----:|-------:|-------:|-------:|:---------|:------------|
| tune ranking for high-overlap unscheduled candidates | 8 | 1.29 | 1.29 | 1.29 | high-overlap candidates not in the schedule by T_max, exact total=203/232 | 1.4, 2.4, 3.4, 4.2, 3.5, ... |
| split wrong-slice walker batches | 12 | 1.16 | 1.15 | 0.93 | nearby candidates have low exact atom overlap | 2.10, 2.11, 2.6, 3.8, 4.1, ... |
| add walker candidates for no-discovered rows | 1 | 0.03 | 0.03 | 0.03 | NS rows have no discovered line candidate | 5.4 |

### Subtractive (suppress consistently off-NS batches)

| pattern | batches | freed@1k | freed@3k | freed@9k | evidence | top batch ids |
|:--------|--------:|---------:|---------:|---------:|:---------|:--------------|
| c header banner in testhelp.h | 1 | 0 | 534 | 534 | off_3k=534 | c header banner in testhelp.h |
| c header banner in sds.h | 1 | 0 | 493 | 493 | off_3k=493 | c header banner in sds.h |
| headings outline in README.md | 1 | 129 | 129 | 129 | off_3k=246 | headings outline in README.md |
| c decl names surface in sds.h | 1 | 99 | 99 | 99 | off_3k=137 | c decl names surface in sds.h |

Top missed paths (NS rows ≤ 3K): README.md (5 rows, 81 atoms), sdsalloc.h (1 row, 10 atoms), sds.h (1 row, 9 atoms)

## Arrival ledger by diagnosis

### ranking-recoverable

| id | ns_t | credit | status | descriptor | candidate hint |
|----|-----:|-------:|:-------|:-----------|:---------------|
| 1.4 | 252 | 0.00 | missing | README v2 lede — perf note + sdscatfmt headline | [unscheduled bbox exact=7/8] README.md section #0 (7 atoms, too expensive at final margin) |
| 2.4 | 657 | 0.00 | missing | Cardinal usage rule — must reassign return value | [unscheduled bbox exact=9/11] README.md section #0 (9 atoms, too expensive at final margin) |
| 3.4 | 3492 | 0.06 | missing | How SDS strings work — design narrative + ASCII diagram | [scheduled bbox exact=2/35] headings outline in README.md (t=815, 2 atoms); better unscheduled exact=28/35: README.md section #0 (28 atoms, too expensive at final margin) |
| 3.5 | 3896 | 0.00 | missing | Preallocation + SDS_MAX_PREALLOC growth strategy | [unscheduled bbox exact=22/26] README.md section #0 (22 atoms, too expensive at final margin) |
| 3.6 | 4166 | 0.10 | missing | Zero-copy append idiom | [scheduled bbox exact=2/20] headings outline in README.md (t=815, 2 atoms); better unscheduled exact=16/20: README.md section #0 (16 atoms, too expensive at final margin) |
| 3.9 | 5456 | 0.03 | missing | Outdated v1 internals diagram in README | [scheduled bbox exact=1/35] headings outline in README.md (t=815, 1 atoms); better unscheduled exact=29/35: README.md section #0 (29 atoms, too expensive at final margin) |
| 4.2 | 6852 | 0.00 | missing | sdsnewlen body — canonical allocator (no doc-comment) | [scheduled bbox exact=2/57] c decl at sds.c:89 (t=6220, 2 atoms); better unscheduled exact=54/57: c decl body at sds.c:89 (54 atoms, too expensive at final margin) |
| 5.2 | 9412 | 0.00 | missing | sdsIncrLen body — zero-copy idiom's other half (no doc-comment) | [scheduled bbox exact=2/40] c decl at sds.c:334 (t=6220, 2 atoms); better unscheduled exact=38/40: c decl body at sds.c:334 (38 atoms, too expensive at final margin) |

### wrong-slice / granularity

| id | ns_t | credit | status | descriptor | candidate hint |
|----|-----:|-------:|:-------|:-----------|:---------------|
| 2.6 | 979 | 0.23 | missing | Public fn declarations — printf family (sdscatvprintf/printf/fmt) | [scheduled bbox exact=3/9] c decl names surface in sds.h (t=1919, 3 atoms) |
| 2.10 | 2230 | 0.12 | missing | Canonical Hello World | [scheduled bbox exact=3/24] headings outline in README.md (t=815, 3 atoms); better unscheduled exact=17/24: README.md section #0 (17 atoms, too expensive at final margin) |
| 2.11 | 2584 | 0.21 | missing | Embedding + allocator-swap instructions | [scheduled bbox exact=6/28] headings outline in README.md (t=815, 6 atoms); better unscheduled exact=21/28: README.md section #0 (21 atoms, too expensive at final margin) |
| 2.12 | 2706 | 0.30 | missing | sdsalloc.h — full file | [scheduled bbox exact=6/10] c header banner in sdsalloc.h (t=4050, 6 atoms) |
| 3.1 | 2823 | 0.30 | missing | Error handling — NULL on OOM | [scheduled bbox exact=3/10] headings outline in README.md (t=815, 3 atoms); better unscheduled exact=7/10: README.md section #0 (7 atoms, too expensive at final margin) |
| 3.7 | 4516 | 0.00 | missing | sdsReqType + sdsHdrSize — internal dispatch helpers | [scheduled same-file] c decl names surface in sds.c (t=6220, 83 atoms) |
| 3.8 | 4974 | 0.05 | missing | sds.h inline accessors — sdslen + sdsavail | [scheduled bbox exact=23/42] c decl body at sds.h:104 (t=5056, 23 atoms) |
| 4.1 | 6189 | 0.00 | missing | sdsMakeRoomFor body — growth/realloc engine | [scheduled bbox exact=6/51] c decl doc at sds.c:204 (t=8114, 6 atoms); better unscheduled exact=38/51: c decl body at sds.c:204 (38 atoms, too expensive at final margin) |
| 4.4 | 7347 | 0.00 | missing | Constructor family + sdsfree (short bodies) | [scheduled bbox exact=8/22] c decl names surface in sds.c (t=6220, 8 atoms) |
| 4.5 | 8081 | 0.02 | missing | README — Concatenating strings + sdsgrowzero | [scheduled bbox exact=1/64] headings outline in README.md (t=815, 1 atoms); better unscheduled exact=49/64: README.md section #0 (49 atoms, too expensive at final margin) |
| 5.1 | 8792 | 0.00 | missing | Append/copy primitive bodies (sdscatlen/cat/catsds/cpylen/cpy) | [scheduled bbox exact=10/50] c decl names surface in sds.c (t=6220, 10 atoms) |
| 5.3 | 9663 | 0.74 | partial | testhelp.h — minimal test framework macros | [scheduled bbox exact=8/19] c decl at testhelp.h:48 (t=416, 8 atoms) |

### no discovered candidate

| id | ns_t | credit | status | descriptor | candidate hint |
|----|-----:|-------:|:-------|:-----------|:---------------|
| 5.4 | 9909 | 0.00 | missing | Makefile + Changelog | no discovered line candidate |

### mixed/unknown

| id | ns_t | credit | status | descriptor | candidate hint |
|----|-----:|-------:|:-------|:-----------|:---------------|
| 4.3 | 7067 | 0.00 | missing | sdscatfmt — fast subset of printf, format spec list | [scheduled bbox exact=16/16] c decl doc at sds.c:616 (t=9880, 16 atoms) |

Top wasted paths (off-NS at 3K): testhelp.h (746t, 4 batches), sds.h (630t, 2 batches), README.md (246t, 1 batch)

## Walker waste (walker_t ≤ 3000, off_3k ≥ 50)

| off_3k | off_3k_ratio | off_any | cost | walker_t | batch |
|-------:|-------------:|--------:|-----:|---------:|:------|
| 534 | 1.00 | 534 | 534 | 2854 | c header banner in testhelp.h |
| 493 | 1.00 | 493 | 493 | 2361 | c header banner in sds.h |
| 246 | 0.62 | 129 | 399 | 416 | headings outline in README.md |
| 137 | 0.12 | 99 | 1104 | 815 | c decl names surface in sds.h |
| 97 | 1.00 | 0 | 97 | 319 | c decl at testhelp.h:48 |
| 59 | 1.00 | 0 | 59 | 220 | c decl at testhelp.h:44 |
| 56 | 1.00 | 0 | 56 | 62 | c decl names surface in testhelp.h |
