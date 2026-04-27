scores: Sim=0.446 Reached=15/34 Early=2 Late=10 Partial=3 Missing=16 Used=9976/10000

## Tier rollup

| tier | batches | reached | partial | missing | avg_credit |
|-----:|--------:|--------:|--------:|--------:|-----------:|
| 1 | 4 | 3 | 0 | 1 | 0.75 |
| 2 | 12 | 8 | 0 | 4 | 0.71 |
| 3 | 9 | 3 | 0 | 6 | 0.38 |
| 4 | 5 | 1 | 1 | 3 | 0.37 |
| 5 | 4 | 0 | 2 | 2 | 0.32 |

## Arrival ledger (non-aligned or partial-credit NS batches)

| id | exp_t | reached_t | delta_t | credit | status | descriptor | nearby walker batch |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.2 | 39 | 220 | +181 | 1.00 | late | README title | README headline in README.md (t=220, 1 atoms) |
| 1.3 | 128 | 220 | +92 | 1.00 | late | README v2 lede — first paragraph (binary-compat warning) | README headline in README.md (t=220, 4 atoms) |
| 1.4 | 252 | — | — | 0.00 | missing | README v2 lede — perf note + sdscatfmt headline |  |
| 2.1 | 262 | 1919 | +1657 | 1.00 | late | sds typedef — sds IS char* | c decl names surface in sds.h (t=1919, 1 atoms) |
| 2.2 | 293 | 1919 | +1626 | 1.00 | late | SDS_MAX_PREALLOC + SDS_NOINIT constants | c decl names surface in sds.h (t=1919, 2 atoms) |
| 2.3 | 469 | 1919 | +1450 | 1.00 | late | SDS_TYPE_* tag constants + SDS_HDR macros | c decl names surface in sds.h (t=1919, 10 atoms) |
| 2.4 | 657 | — | — | 0.00 | missing | Cardinal usage rule — must reassign return value |  |
| 2.5 | 847 | 1919 | +1072 | 1.00 | late | Public fn declarations — high-level API (creation/length/free/concat/copy) | c decl names surface in sds.h (t=1919, 11 atoms) |
| 2.6 | 979 | — | — | 0.23 | missing | Public fn declarations — printf family (sdscatvprintf/printf/fmt) | c decl names surface in sds.h (t=1919, 3 atoms) |
| 2.7 | 1276 | 1919 | +643 | 1.00 | late | Public fn declarations — utility fns (trim/range/cmp/split/case/repr/join) | c decl names surface in sds.h (t=1919, 15 atoms) |
| 2.8 | 1487 | 2361 | +874 | 1.00 | late | Public fn declarations — low-level + allocator-export API | c decl names surface in sds.h (t=1919, 9 atoms) |
| 2.10 | 2230 | — | — | 0.12 | missing | Canonical Hello World | headings outline in README.md (t=815, 3 atoms) |
| 2.11 | 2584 | — | — | 0.21 | missing | Embedding + allocator-swap instructions | headings outline in README.md (t=815, 6 atoms) |
| 2.12 | 2706 | 4050 | +1344 | 0.90 | late | sdsalloc.h — full file | c header banner in sdsalloc.h (t=4050, 6 atoms) |
| 3.1 | 2823 | — | — | 0.30 | missing | Error handling — NULL on OOM | headings outline in README.md (t=815, 3 atoms) |
| 3.2 | 2912 | 815 | -2097 | 1.00 | early | README section TOC — first half (intro through copying) | headings outline in README.md (t=815, 31 atoms) |
| 3.3 | 3011 | 815 | -2196 | 1.00 | early | README section TOC — second half (quoting through credits) | headings outline in README.md (t=815, 40 atoms) |
| 3.4 | 3492 | — | — | 0.06 | missing | How SDS strings work — design narrative + ASCII diagram | headings outline in README.md (t=815, 2 atoms) |
| 3.5 | 3896 | — | — | 0.00 | missing | Preallocation + SDS_MAX_PREALLOC growth strategy |  |
| 3.6 | 4166 | — | — | 0.10 | missing | Zero-copy append idiom | headings outline in README.md (t=815, 2 atoms) |
| 3.7 | 4516 | — | — | 0.00 | missing | sdsReqType + sdsHdrSize — internal dispatch helpers |  |
| 3.8 | 4974 | 5056 | +82 | 0.93 | aligned | sds.h inline accessors — sdslen + sdsavail | c decl body at sds.h:104 (t=5056, 23 atoms) |
| 3.9 | 5456 | — | — | 0.03 | missing | Outdated v1 internals diagram in README | headings outline in README.md (t=815, 1 atoms) |
| 4.1 | 6189 | — | — | 0.14 | missing | sdsMakeRoomFor body — growth/realloc engine | c decl doc at sds.c:204 (t=8114, 6 atoms) |
| 4.2 | 6852 | — | — | 0.02 | missing | sdsnewlen body — canonical allocator (no doc-comment) | c decl names surface in sds.c (t=6220, 2 atoms) |
| 4.3 | 7067 | 9880 | +2813 | 1.00 | late | sdscatfmt — fast subset of printf, format spec list | c decl doc at sds.c:616 (t=9880, 16 atoms) |
| 4.4 | 7347 | — | — | 0.68 | partial | Constructor family + sdsfree (short bodies) | c decl names surface in sds.c (t=6220, 8 atoms) |
| 4.5 | 8081 | — | — | 0.02 | missing | README — Concatenating strings + sdsgrowzero | headings outline in README.md (t=815, 1 atoms) |
| 5.1 | 8792 | — | — | 0.50 | partial | Append/copy primitive bodies (sdscatlen/cat/catsds/cpylen/cpy) | c decl names surface in sds.c (t=6220, 10 atoms) |
| 5.2 | 9412 | — | — | 0.03 | missing | sdsIncrLen body — zero-copy idiom's other half (no doc-comment) | c decl names surface in sds.c (t=6220, 2 atoms) |
| 5.3 | 9663 | — | — | 0.74 | partial | testhelp.h — minimal test framework macros | c decl at testhelp.h:48 (t=416, 8 atoms) |
| 5.4 | 9909 | — | — | 0.00 | missing | Makefile + Changelog |  |

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
| 198 | 1.00 | 198 | 9030 | c decl doc at sds.c:184 |
| 190 | 1.00 | 190 | 8832 | c decl doc at sds.c:756 |
| 177 | 1.00 | 177 | 8642 | c decl doc at sds.c:725 |
| 163 | 1.00 | 163 | 4376 | c decl body at sds.h:180 |
| 149 | 1.00 | 149 | 8465 | c decl doc at sds.c:1092 |
| 140 | 1.00 | 140 | 8316 | c decl doc at sds.c:807 |
| 129 | 0.32 | 399 | 815 | headings outline in README.md |
| 110 | 1.00 | 110 | 8003 | c decl doc at sds.c:898 |
| 103 | 1.00 | 103 | 7893 | c decl doc at sds.c:1136 |
| 99 | 0.09 | 1104 | 1919 | c decl names surface in sds.h |
| 98 | 1.00 | 98 | 7704 | c decl doc at sds.c:256 |
| 96 | 1.00 | 96 | 9976 | c decl body at sds.c:1108 |
| 88 | 1.00 | 88 | 7559 | c decl doc at sds.c:300 |
| 86 | 1.00 | 86 | 7790 | c decl doc at sds.c:450 |
| 83 | 1.00 | 83 | 3471 | c includes in sds.c |
| 73 | 1.00 | 73 | 7264 | c decl doc at sds.c:380 |
| 69 | 1.00 | 69 | 7154 | c decl doc at sds.c:193 |
| 62 | 1.00 | 62 | 8176 | c decl body at sds.c:591 |
| 51 | 1.00 | 51 | 6855 | c decl doc at sds.c:526 |
