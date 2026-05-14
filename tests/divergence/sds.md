Score(3000)=0.646 I=0.792 C=0.527 ns_rows≤3K=18/34 (reached=11 partial=0 missing=7)

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 31 | 31 | listing of '.' |  |  | 1.000 |
| ns | 31 |  | 31 | Repo file listing | 1.1 |  | 1.000 |
| ns | 39 |  | 8 | README title | 1.2 |  | 0.932 |
| ns | 128 |  | 89 | README v2 lede — first paragraph (binary-compat warning) | 1.3 |  | 0.748 |
| walker |  | 133 | 102 | README headline in README.md |  |  | 1.000 |
| walker |  | 165 | 32 | c includes in sds.h |  |  | 1.000 |
| walker |  | 173 | 8 | plaintext config .gitignore |  |  | 1.000 |
| walker |  | 204 | 31 | c decl names surface in sdsalloc.h |  |  | 1.000 |
| ns | 252 |  | 124 | README v2 lede — perf note + sdscatfmt headline | 1.4 |  | 0.749 |
| walker |  | 260 | 56 | c decl names surface in testhelp.h |  |  | 0.749 |
| ns | 262 |  | 10 | sds typedef — sds IS char* | 2.1 |  | 0.728 |
| ns | 293 |  | 31 | SDS_MAX_PREALLOC + SDS_NOINIT constants | 2.2 |  | 0.691 |
| ns | 469 |  | 176 | SDS_TYPE_* tag constants + SDS_HDR macros | 2.3 |  | 0.560 |
| ns | 657 |  | 188 | Cardinal usage rule — must reassign return value | 2.4 |  | 0.474 |
| walker |  | 659 | 399 | headings outline in README.md |  |  | 0.481 |
| ns | 847 |  | 190 | Public fn declarations — high-level API (creation/length/free/concat/copy) | 2.5 |  | 0.422 |
| ns | 979 |  | 132 | Public fn declarations — printf family (sdscatvprintf/printf/fmt) | 2.6 |  | 0.386 |
| ns | 1276 |  | 297 | Public fn declarations — utility fns (trim/range/cmp/split/case/repr/join) | 2.7 |  | 0.342 |
| ns | 1487 |  | 211 | Public fn declarations — low-level + allocator-export API | 2.8 |  | 0.311 |
| walker |  | 1763 | 1104 | c decl names surface in sds.h |  |  | 0.716 |
| walker |  | 1763 | 0 | c decl at sds.h:87 |  |  | 0.716 |
| walker |  | 1763 | 0 | c decl at sds.h:104 |  |  | 0.716 |
| walker |  | 1763 | 0 | c decl at sds.h:130 |  |  | 0.716 |
| walker |  | 1763 | 0 | c decl at sds.h:154 |  |  | 0.716 |
| walker |  | 1763 | 0 | c decl at sds.h:180 |  |  | 0.716 |
| walker |  | 1763 | 0 | c decl at sds.h:197 |  |  | 0.716 |
| walker |  | 1763 | 0 | c decl at sds.h:256 |  |  | 0.716 |
| walker |  | 1763 | 0 | c decl at sds.h:266 |  |  | 0.716 |
| walker |  | 1800 | 37 | c decl at sds.h:47 |  |  | 0.717 |
| walker |  | 1814 | 14 | c decl doc at sds.h:256 |  |  | 0.727 |
| walker |  | 1834 | 20 | c decl doc at sds.h:180 |  |  | 0.727 |
| walker |  | 1899 | 65 | c decl at sds.h:51 |  |  | 0.729 |
| ns | 1917 |  | 430 | Five packed sdshdr structs | 2.9 |  | 0.648 |
| walker |  | 1964 | 65 | c decl at sds.h:57 |  |  | 0.675 |
| walker |  | 2029 | 65 | c decl at sds.h:63 |  |  | 0.710 |
| walker |  | 2094 | 65 | c decl at sds.h:69 |  |  | 0.751 |
| walker |  | 2137 | 43 | c decl doc at sds.h:47 |  |  | 0.779 |
| walker |  | 2205 | 68 | c decl doc at sds.h:266 |  |  | 0.822 |
| ns | 2230 |  | 313 | Canonical Hello World | 2.10 |  | 0.742 |
| ns | 2584 |  | 354 | Embedding + allocator-swap instructions | 2.11 |  | 0.672 |
| walker |  | 2698 | 493 | c header banner in sds.h |  |  | 0.672 |
| ns | 2706 |  | 122 | sdsalloc.h — full file | 2.12 | 2.11 | 0.652 |
| walker |  | 2757 | 59 | c decl at testhelp.h:44 |  |  | 0.653 |
| ns | 2823 |  | 117 | Error handling — NULL on OOM | 3.1 |  | 0.633 |
| walker |  | 2840 | 83 | c includes in sds.c |  |  | 0.633 |
| ns | 2912 |  | 89 | README section TOC — first half (intro through copying) | 3.2 |  | 0.646 |
| walker |  | 3003 | 163 | c decl body at sds.h:87 |  |  | 0.647 |
| ns | 3011 |  | 99 | README section TOC — second half (quoting through credits) | 3.3 |  | 0.659 |
| walker |  | 3166 | 163 | c decl body at sds.h:180 |  |  | 0.659 |
| walker |  | 3263 | 97 | c decl at testhelp.h:48 |  |  | 0.660 |
| walker |  | 3466 | 203 | c decl body at sds.h:197 |  |  | 0.660 |
| ns | 3492 |  | 481 | How SDS strings work — design narrative + ASCII diagram | 3.4 |  | 0.605 |
| walker |  | 3701 | 235 | c decl body at sds.h:130 |  |  | 0.605 |
| ns | 3896 |  | 404 | Preallocation + SDS_MAX_PREALLOC growth strategy | 3.5 |  | 0.572 |
| walker |  | 3943 | 242 | c decl body at sds.h:104 |  |  | 0.577 |
| ns | 4166 |  | 270 | Zero-copy append idiom | 3.6 |  | 0.554 |
| walker |  | 4193 | 250 | c decl body at sds.h:154 |  |  | 0.554 |
| ns | 4516 |  | 350 | sdsReqType + sdsHdrSize — internal dispatch helpers | 3.7 |  | 0.523 |
| ns | 4974 |  | 458 | sds.h inline accessors — sdslen + sdsavail | 3.8 |  | 0.562 |
| walker |  | 5107 | 914 | c decl names surface in sds.c |  |  | 0.562 |
| walker |  | 5107 | 0 | c decl at sds.c:89 |  |  | 0.562 |
| walker |  | 5107 | 0 | c decl at sds.c:149 |  |  | 0.562 |
| walker |  | 5107 | 0 | c decl at sds.c:154 |  |  | 0.562 |
| walker |  | 5107 | 0 | c decl at sds.c:160 |  |  | 0.562 |
| walker |  | 5107 | 0 | c decl at sds.c:165 |  |  | 0.562 |
| walker |  | 5107 | 0 | c decl at sds.c:184 |  |  | 0.562 |
| walker |  | 5107 | 0 | c decl at sds.c:193 |  |  | 0.562 |
| walker |  | 5107 | 0 | c decl at sds.c:204 |  |  | 0.562 |
| walker |  | 5107 | 0 | c decl at sds.c:256 |  |  | 0.562 |
| walker |  | 5107 | 0 | c decl at sds.c:300 |  |  | 0.562 |
| walker |  | 5107 | 0 | c decl at sds.c:307 |  |  | 0.562 |
| walker |  | 5107 | 0 | c decl at sds.c:334 |  |  | 0.562 |
| walker |  | 5107 | 0 | c decl at sds.c:380 |  |  | 0.562 |
| walker |  | 5107 | 0 | c decl at sds.c:398 |  |  | 0.562 |
| walker |  | 5107 | 0 | c decl at sds.c:413 |  |  | 0.562 |
| walker |  | 5107 | 0 | c decl at sds.c:421 |  |  | 0.562 |
| walker |  | 5107 | 0 | c decl at sds.c:427 |  |  | 0.562 |
| walker |  | 5107 | 0 | c decl at sds.c:440 |  |  | 0.562 |
| walker |  | 5107 | 0 | c decl at sds.c:450 |  |  | 0.562 |
| walker |  | 5107 | 0 | c decl at sds.c:451 |  |  | 0.562 |
| walker |  | 5107 | 0 | c decl at sds.c:494 |  |  | 0.562 |
| walker |  | 5107 | 0 | c decl at sds.c:526 |  |  | 0.562 |
| walker |  | 5107 | 0 | c decl at sds.c:534 |  |  | 0.562 |
| walker |  | 5107 | 0 | c decl at sds.c:591 |  |  | 0.562 |
| walker |  | 5107 | 0 | c decl at sds.c:616 |  |  | 0.562 |
| walker |  | 5107 | 0 | c decl at sds.c:725 |  |  | 0.562 |
| walker |  | 5107 | 0 | c decl at sds.c:756 |  |  | 0.562 |
| walker |  | 5107 | 0 | c decl at sds.c:783 |  |  | 0.562 |
| walker |  | 5107 | 0 | c decl at sds.c:790 |  |  | 0.562 |
| walker |  | 5107 | 0 | c decl at sds.c:807 |  |  | 0.562 |
| walker |  | 5107 | 0 | c decl at sds.c:835 |  |  | 0.562 |
| walker |  | 5107 | 0 | c decl at sds.c:885 |  |  | 0.562 |
| walker |  | 5107 | 0 | c decl at sds.c:898 |  |  | 0.562 |
| walker |  | 5107 | 0 | c decl at sds.c:925 |  |  | 0.562 |
| walker |  | 5107 | 0 | c decl at sds.c:932 |  |  | 0.562 |
| walker |  | 5107 | 0 | c decl at sds.c:973 |  |  | 0.562 |
| walker |  | 5107 | 0 | c decl at sds.c:1092 |  |  | 0.562 |
| walker |  | 5107 | 0 | c decl at sds.c:1108 |  |  | 0.562 |
| walker |  | 5107 | 0 | c decl at sds.c:1120 |  |  | 0.562 |
| walker |  | 5107 | 0 | c decl at sds.c:1136 |  |  | 0.562 |
| walker |  | 5119 | 12 | c decl doc at sds.c:160 |  |  | 0.562 |
| walker |  | 5130 | 11 | c decl body at sds.c:149 |  |  | 0.562 |
| walker |  | 5150 | 20 | c decl doc at sds.c:154 |  |  | 0.562 |
| walker |  | 5170 | 20 | c decl doc at sds.c:1120 |  |  | 0.562 |
| walker |  | 5191 | 21 | c decl doc at sds.c:494 |  |  | 0.562 |
| walker |  | 5212 | 21 | c decl doc at sds.c:783 |  |  | 0.562 |
| walker |  | 5233 | 21 | c decl doc at sds.c:790 |  |  | 0.562 |
| walker |  | 5255 | 22 | c decl doc at sds.c:534 |  |  | 0.562 |
| walker |  | 5270 | 15 | c decl body at sds.c:160 |  |  | 0.562 |
| walker |  | 5285 | 15 | c decl body at sds.c:413 |  |  | 0.563 |
| walker |  | 5300 | 15 | c decl body at sds.c:440 |  |  | 0.563 |
| walker |  | 5323 | 23 | c decl doc at sds.c:165 |  |  | 0.563 |
| walker |  | 5349 | 26 | c decl doc at sds.c:885 |  |  | 0.563 |
| walker |  | 5366 | 17 | c decl body at sds.c:307 |  |  | 0.563 |
| walker |  | 5383 | 17 | c decl body at sds.c:421 |  |  | 0.563 |
| walker |  | 5416 | 33 | c decl doc at sds.c:925 |  |  | 0.563 |
| walker |  | 5450 | 34 | c decl doc at sds.c:307 |  |  | 0.563 |
| ns | 5456 |  | 482 | Outdated v1 internals diagram in README | 3.9 |  | 0.534 |
| walker |  | 5484 | 34 | c decl doc at sds.c:932 |  |  | 0.534 |
| walker |  | 5520 | 36 | c decl doc at sds.c:149 |  |  | 0.535 |
| walker |  | 5558 | 38 | c decl doc at sds.c:1108 |  |  | 0.535 |
| walker |  | 5599 | 41 | c decl doc at sds.c:440 |  |  | 0.535 |
| walker |  | 5641 | 42 | c decl doc at sds.c:427 |  |  | 0.535 |
| walker |  | 5666 | 25 | c decl body at sds.c:184 |  |  | 0.535 |
| walker |  | 5691 | 25 | c decl body at sds.c:193 |  |  | 0.535 |
| walker |  | 5742 | 51 | c decl doc at sds.c:526 |  |  | 0.535 |
| walker |  | 5772 | 30 | c decl body at sds.c:165 |  |  | 0.535 |
| walker |  | 5802 | 30 | c decl body at sds.c:300 |  |  | 0.535 |
| walker |  | 5837 | 35 | c decl body at sds.c:154 |  |  | 0.536 |
| walker |  | 5904 | 67 | c decl doc at sds.c:421 |  |  | 0.536 |
| walker |  | 5972 | 68 | c decl doc at sds.c:413 |  |  | 0.536 |
| walker |  | 6041 | 69 | c decl doc at sds.c:193 |  |  | 0.536 |
| walker |  | 6078 | 37 | c decl body at sds.c:885 |  |  | 0.536 |
| walker |  | 6151 | 73 | c decl doc at sds.c:380 |  |  | 0.536 |
| ns | 6189 |  | 733 | sdsMakeRoomFor body — growth/realloc engine | 4.1 |  | 0.501 |
| walker |  | 6191 | 40 | c decl body at sds.c:526 |  |  | 0.501 |
| walker |  | 6232 | 41 | c decl body at sds.c:783 |  |  | 0.501 |
| walker |  | 6273 | 41 | c decl body at sds.c:790 |  |  | 0.501 |
| walker |  | 6358 | 85 | c decl doc at sds.c:398 |  |  | 0.502 |
| walker |  | 6446 | 88 | c decl doc at sds.c:300 |  |  | 0.502 |
| walker |  | 6493 | 47 | c decl body at sds.c:925 |  |  | 0.502 |
| walker |  | 6591 | 98 | c decl doc at sds.c:256 |  |  | 0.502 |
| walker |  | 6677 | 86 | c decl doc at sds.c:450 |  |  | 0.502 |
| walker |  | 6780 | 103 | c decl doc at sds.c:1136 |  |  | 0.502 |
| ns | 6852 |  | 663 | sdsnewlen body — canonical allocator (no doc-comment) | 4.2 |  | 0.469 |
| walker |  | 6890 | 110 | c decl doc at sds.c:898 |  |  | 0.469 |
| walker |  | 7001 | 111 | c decl doc at sds.c:204 |  |  | 0.472 |
| walker |  | 7063 | 62 | c decl body at sds.c:591 |  |  | 0.472 |
| ns | 7067 |  | 215 | sdscatfmt — fast subset of printf, format spec list | 4.3 |  | 0.463 |
| walker |  | 7203 | 140 | c decl doc at sds.c:807 |  |  | 0.463 |
| ns | 7347 |  | 280 | Constructor family + sdsfree (short bodies) | 4.4 |  | 0.473 |
| walker |  | 7352 | 149 | c decl doc at sds.c:1092 |  |  | 0.473 |
| walker |  | 7529 | 177 | c decl doc at sds.c:725 |  |  | 0.473 |
| walker |  | 7719 | 190 | c decl doc at sds.c:756 |  |  | 0.473 |
| walker |  | 7917 | 198 | c decl doc at sds.c:184 |  |  | 0.473 |
| ns | 8081 |  | 734 | README — Concatenating strings + sdsgrowzero | 4.5 |  | 0.444 |
| walker |  | 8126 | 209 | c decl doc at sds.c:835 |  |  | 0.444 |
| walker |  | 8337 | 211 | c decl doc at sds.c:89 |  |  | 0.444 |
| walker |  | 8552 | 215 | c decl doc at sds.c:591 |  |  | 0.444 |
| walker |  | 8767 | 215 | c decl doc at sds.c:616 |  |  | 0.466 |
| ns | 8792 |  | 711 | Append/copy primitive bodies (sdscatlen/cat/catsds/cpylen/cpy) | 5.1 |  | 0.467 |
| walker |  | 8863 | 96 | c decl body at sds.c:1108 |  |  | 0.467 |
| walker |  | 8960 | 97 | c decl body at sds.c:398 |  |  | 0.478 |
| walker |  | 9058 | 98 | c decl body at sds.c:427 |  |  | 0.491 |
| walker |  | 9158 | 100 | c decl body at sds.c:1120 |  |  | 0.491 |
| ns | 9412 |  | 620 | sdsIncrLen body — zero-copy idiom's other half (no doc-comment) | 5.2 | 3.6 | 0.475 |
| walker |  | 9413 | 255 | c decl doc at sds.c:973 |  |  | 0.475 |
| ns | 9663 |  | 251 | testhelp.h — minimal test framework macros | 5.3 |  | 0.481 |
| walker |  | 9735 | 322 | c decl doc at sds.c:334 |  |  | 0.481 |
| walker |  | 9858 | 123 | c decl body at sds.c:380 |  |  | 0.481 |
| ns | 9909 |  | 246 | Makefile + Changelog | 5.4 |  | 0.473 |
| walker |  | 9983 | 125 | c decl body at sds.c:1092 |  |  | 0.473 |
