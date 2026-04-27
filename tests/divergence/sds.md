scores: Sim=0.306 Reached=5/34 Early=2 Late=1 Partial=0 Missing=29 Used=540/10000

## Tier rollup

| tier | batches | reached | partial | missing | avg_credit |
|-----:|--------:|--------:|--------:|--------:|-----------:|
| 1 | 4 | 3 | 0 | 1 | 0.75 |
| 2 | 12 | 0 | 0 | 12 | 0.03 |
| 3 | 9 | 2 | 0 | 7 | 0.28 |
| 4 | 5 | 0 | 0 | 5 | 0.00 |
| 5 | 4 | 0 | 0 | 4 | 0.00 |

## Arrival ledger (non-aligned or partial-credit NS batches)

| id | exp_t | reached_t | delta_t | credit | status | descriptor | nearby walker batch |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.2 | 39 | 133 | +94 | 1.00 | late | README title | README headline in README.md (t=133, 1 atoms) |
| 1.4 | 252 | — | — | 0.00 | missing | README v2 lede — perf note + sdscatfmt headline |  |
| 2.1 | 262 | — | — | 0.00 | missing | sds typedef — sds IS char* |  |
| 2.2 | 293 | — | — | 0.00 | missing | SDS_MAX_PREALLOC + SDS_NOINIT constants |  |
| 2.3 | 469 | — | — | 0.00 | missing | SDS_TYPE_* tag constants + SDS_HDR macros |  |
| 2.4 | 657 | — | — | 0.00 | missing | Cardinal usage rule — must reassign return value |  |
| 2.5 | 847 | — | — | 0.00 | missing | Public fn declarations — high-level API (creation/length/free/concat/copy) |  |
| 2.6 | 979 | — | — | 0.00 | missing | Public fn declarations — printf family (sdscatvprintf/printf/fmt) |  |
| 2.7 | 1276 | — | — | 0.00 | missing | Public fn declarations — utility fns (trim/range/cmp/split/case/repr/join) |  |
| 2.8 | 1487 | — | — | 0.00 | missing | Public fn declarations — low-level + allocator-export API |  |
| 2.9 | 1917 | — | — | 0.00 | missing | Five packed sdshdr structs |  |
| 2.10 | 2230 | — | — | 0.12 | missing | Canonical Hello World | headings outline in README.md (t=540, 3 atoms) |
| 2.11 | 2584 | — | — | 0.21 | missing | Embedding + allocator-swap instructions | headings outline in README.md (t=540, 6 atoms) |
| 2.12 | 2706 | — | — | 0.00 | missing | sdsalloc.h — full file |  |
| 3.1 | 2823 | — | — | 0.30 | missing | Error handling — NULL on OOM | headings outline in README.md (t=540, 3 atoms) |
| 3.2 | 2912 | 540 | -2372 | 1.00 | early | README section TOC — first half (intro through copying) | headings outline in README.md (t=540, 31 atoms) |
| 3.3 | 3011 | 540 | -2471 | 1.00 | early | README section TOC — second half (quoting through credits) | headings outline in README.md (t=540, 40 atoms) |
| 3.4 | 3492 | — | — | 0.06 | missing | How SDS strings work — design narrative + ASCII diagram | headings outline in README.md (t=540, 2 atoms) |
| 3.5 | 3896 | — | — | 0.00 | missing | Preallocation + SDS_MAX_PREALLOC growth strategy |  |
| 3.6 | 4166 | — | — | 0.10 | missing | Zero-copy append idiom | headings outline in README.md (t=540, 2 atoms) |
| 3.7 | 4516 | — | — | 0.00 | missing | sdsReqType + sdsHdrSize — internal dispatch helpers |  |
| 3.8 | 4974 | — | — | 0.00 | missing | sds.h inline accessors — sdslen + sdsavail |  |
| 3.9 | 5456 | — | — | 0.03 | missing | Outdated v1 internals diagram in README | headings outline in README.md (t=540, 1 atoms) |
| 4.1 | 6189 | — | — | 0.00 | missing | sdsMakeRoomFor body — growth/realloc engine |  |
| 4.2 | 6852 | — | — | 0.00 | missing | sdsnewlen body — canonical allocator (no doc-comment) |  |
| 4.3 | 7067 | — | — | 0.00 | missing | sdscatfmt — fast subset of printf, format spec list |  |
| 4.4 | 7347 | — | — | 0.00 | missing | Constructor family + sdsfree (short bodies) |  |
| 4.5 | 8081 | — | — | 0.02 | missing | README — Concatenating strings + sdsgrowzero | headings outline in README.md (t=540, 1 atoms) |
| 5.1 | 8792 | — | — | 0.00 | missing | Append/copy primitive bodies (sdscatlen/cat/catsds/cpylen/cpy) |  |
| 5.2 | 9412 | — | — | 0.00 | missing | sdsIncrLen body — zero-copy idiom's other half (no doc-comment) |  |
| 5.3 | 9663 | — | — | 0.00 | missing | testhelp.h — minimal test framework macros |  |
| 5.4 | 9909 | — | — | 0.00 | missing | Makefile + Changelog |  |

## Walker waste (off-NS token spend ≥ 50)

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 129 | 0.32 | 399 | 540 | headings outline in README.md |
