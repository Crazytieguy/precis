Score(3000)=0.663 I=0.806 C=0.546 ns_rows≤3K=18/34 (reached=11 partial=1 missing=6)

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 31 | 31 | listing of '.' |  |  | 1.000 |
| ns | 31 |  | 31 | Repo file listing | 1.1 |  | 1.000 |
| ns | 39 |  | 8 | README title | 1.2 |  | 0.932 |
| ns | 128 |  | 89 | README v2 lede — first paragraph (binary-compat warning) | 1.3 |  | 0.748 |
| walker |  | 133 | 102 | README headline in README.md |  |  | 1.000 |
| walker |  | 164 | 31 | c decl names surface in sdsalloc.h |  |  | 1.000 |
| ns | 252 |  | 124 | README v2 lede — perf note + sdscatfmt headline | 1.4 |  | 0.749 |
| ns | 262 |  | 10 | sds typedef — sds IS char* | 2.1 |  | 0.728 |
| ns | 293 |  | 31 | SDS_MAX_PREALLOC + SDS_NOINIT constants | 2.2 |  | 0.691 |
| ns | 469 |  | 176 | SDS_TYPE_* tag constants + SDS_HDR macros | 2.3 |  | 0.560 |
| walker |  | 563 | 399 | headings outline in README.md |  |  | 0.569 |
| walker |  | 619 | 56 | c decl names surface in testhelp.h |  |  | 0.569 |
| ns | 657 |  | 188 | Cardinal usage rule — must reassign return value | 2.4 |  | 0.481 |
| ns | 847 |  | 190 | Public fn declarations — high-level API (creation/length/free/concat/copy) | 2.5 |  | 0.422 |
| ns | 979 |  | 132 | Public fn declarations — printf family (sdscatvprintf/printf/fmt) | 2.6 |  | 0.386 |
| walker |  | 1057 | 438 | c decl names surface in sds.h |  |  | 0.573 |
| walker |  | 1057 | 0 | c decl at sds.h:87 |  |  | 0.573 |
| walker |  | 1057 | 0 | c decl at sds.h:104 |  |  | 0.573 |
| walker |  | 1057 | 0 | c decl at sds.h:130 |  |  | 0.573 |
| walker |  | 1057 | 0 | c decl at sds.h:154 |  |  | 0.573 |
| walker |  | 1057 | 0 | c decl at sds.h:180 |  |  | 0.573 |
| walker |  | 1057 | 0 | c decl at sds.h:197 |  |  | 0.573 |
| walker |  | 1094 | 37 | c decl at sds.h:47 |  |  | 0.574 |
| walker |  | 1159 | 65 | c decl at sds.h:51 |  |  | 0.576 |
| walker |  | 1224 | 65 | c decl at sds.h:57 |  |  | 0.579 |
| ns | 1276 |  | 297 | Public fn declarations — utility fns (trim/range/cmp/split/case/repr/join) | 2.7 |  | 0.512 |
| walker |  | 1289 | 65 | c decl at sds.h:63 |  |  | 0.516 |
| walker |  | 1354 | 65 | c decl at sds.h:69 |  |  | 0.520 |
| walker |  | 1386 | 32 | c includes in sds.h |  |  | 0.520 |
| walker |  | 1406 | 20 | c decl doc at sds.h:180 |  |  | 0.520 |
| walker |  | 1449 | 43 | c decl doc at sds.h:47 |  |  | 0.523 |
| ns | 1487 |  | 211 | Public fn declarations — low-level + allocator-export API | 2.8 |  | 0.476 |
| walker |  | 1895 | 446 | c decl names surface #1 in sds.h |  |  | 0.645 |
| walker |  | 1911 | 16 | c decl at sds.h:232 |  |  | 0.657 |
| ns | 1917 |  | 430 | Five packed sdshdr structs | 2.9 |  | 0.713 |
| walker |  | 2193 | 282 | c decl names surface #2 in sds.h |  |  | 0.794 |
| walker |  | 2193 | 0 | c decl at sds.h:256 |  |  | 0.794 |
| walker |  | 2193 | 0 | c decl at sds.h:266 |  |  | 0.794 |
| walker |  | 2207 | 14 | c decl doc at sds.h:256 |  |  | 0.801 |
| ns | 2230 |  | 313 | Canonical Hello World | 2.10 |  | 0.722 |
| ns | 2584 |  | 354 | Embedding + allocator-swap instructions | 2.11 |  | 0.654 |
| walker |  | 2700 | 493 | c header banner in sds.h |  |  | 0.654 |
| ns | 2706 |  | 122 | sdsalloc.h — full file | 2.12 | 2.11 | 0.636 |
| walker |  | 2768 | 68 | c decl doc at sds.h:266 |  |  | 0.670 |
| ns | 2823 |  | 117 | Error handling — NULL on OOM | 3.1 |  | 0.650 |
| walker |  | 2827 | 59 | c decl at testhelp.h:44 |  |  | 0.650 |
| ns | 2912 |  | 89 | README section TOC — first half (intro through copying) | 3.2 |  | 0.662 |
| walker |  | 2990 | 163 | c decl body at sds.h:87 |  |  | 0.663 |
| ns | 3011 |  | 99 | README section TOC — second half (quoting through credits) | 3.3 |  | 0.674 |
| walker |  | 3153 | 163 | c decl body at sds.h:180 |  |  | 0.674 |
| walker |  | 3250 | 97 | c decl at testhelp.h:48 |  |  | 0.675 |
| ns | 3492 |  | 481 | How SDS strings work — design narrative + ASCII diagram | 3.4 |  | 0.619 |
| walker |  | 3724 | 474 | c decl names surface in sds.c |  |  | 0.619 |
| walker |  | 3724 | 0 | c decl at sds.c:89 |  |  | 0.619 |
| walker |  | 3724 | 0 | c decl at sds.c:149 |  |  | 0.619 |
| walker |  | 3724 | 0 | c decl at sds.c:154 |  |  | 0.619 |
| walker |  | 3724 | 0 | c decl at sds.c:160 |  |  | 0.619 |
| walker |  | 3724 | 0 | c decl at sds.c:165 |  |  | 0.619 |
| walker |  | 3724 | 0 | c decl at sds.c:184 |  |  | 0.619 |
| walker |  | 3724 | 0 | c decl at sds.c:193 |  |  | 0.619 |
| walker |  | 3724 | 0 | c decl at sds.c:204 |  |  | 0.619 |
| walker |  | 3724 | 0 | c decl at sds.c:256 |  |  | 0.619 |
| walker |  | 3724 | 0 | c decl at sds.c:300 |  |  | 0.619 |
| walker |  | 3724 | 0 | c decl at sds.c:307 |  |  | 0.619 |
| walker |  | 3724 | 0 | c decl at sds.c:334 |  |  | 0.619 |
| walker |  | 3724 | 0 | c decl at sds.c:380 |  |  | 0.619 |
| walker |  | 3724 | 0 | c decl at sds.c:398 |  |  | 0.619 |
| walker |  | 3724 | 0 | c decl at sds.c:413 |  |  | 0.619 |
| walker |  | 3724 | 0 | c decl at sds.c:421 |  |  | 0.619 |
| walker |  | 3724 | 0 | c decl at sds.c:427 |  |  | 0.619 |
| walker |  | 3724 | 0 | c decl at sds.c:440 |  |  | 0.619 |
| walker |  | 3724 | 0 | c decl at sds.c:450 |  |  | 0.619 |
| walker |  | 3724 | 0 | c decl at sds.c:451 |  |  | 0.619 |
| walker |  | 3724 | 0 | c decl at sds.c:494 |  |  | 0.619 |
| walker |  | 3724 | 0 | c decl at sds.c:526 |  |  | 0.619 |
| walker |  | 3724 | 0 | c decl at sds.c:534 |  |  | 0.619 |
| walker |  | 3735 | 11 | c decl body at sds.c:149 |  |  | 0.619 |
| walker |  | 3750 | 15 | c decl body at sds.c:160 |  |  | 0.619 |
| walker |  | 3765 | 15 | c decl body at sds.c:413 |  |  | 0.619 |
| walker |  | 3780 | 15 | c decl body at sds.c:440 |  |  | 0.619 |
| walker |  | 3792 | 12 | c decl doc at sds.c:160 |  |  | 0.619 |
| walker |  | 3809 | 17 | c decl body at sds.c:307 |  |  | 0.619 |
| walker |  | 3826 | 17 | c decl body at sds.c:421 |  |  | 0.619 |
| walker |  | 3851 | 25 | c decl body at sds.c:184 |  |  | 0.619 |
| walker |  | 3876 | 25 | c decl body at sds.c:193 |  |  | 0.619 |
| walker |  | 3896 | 20 | c decl doc at sds.c:154 |  |  | 0.585 |
| ns | 3896 |  | 404 | Preallocation + SDS_MAX_PREALLOC growth strategy | 3.5 |  | 0.585 |
| walker |  | 3917 | 21 | c decl doc at sds.c:494 |  |  | 0.585 |
| walker |  | 3939 | 22 | c decl doc at sds.c:534 |  |  | 0.585 |
| walker |  | 3969 | 30 | c decl body at sds.c:165 |  |  | 0.586 |
| walker |  | 3999 | 30 | c decl body at sds.c:300 |  |  | 0.586 |
| walker |  | 4022 | 23 | c decl doc at sds.c:165 |  |  | 0.586 |
| walker |  | 4057 | 35 | c decl body at sds.c:154 |  |  | 0.586 |
| walker |  | 4097 | 40 | c decl body at sds.c:526 |  |  | 0.586 |
| ns | 4166 |  | 270 | Zero-copy append idiom | 3.6 |  | 0.563 |
| walker |  | 4180 | 83 | c includes in sds.c |  |  | 0.563 |
| walker |  | 4214 | 34 | c decl doc at sds.c:307 |  |  | 0.563 |
| walker |  | 4250 | 36 | c decl doc at sds.c:149 |  |  | 0.564 |
| walker |  | 4291 | 41 | c decl doc at sds.c:440 |  |  | 0.564 |
| walker |  | 4333 | 42 | c decl doc at sds.c:427 |  |  | 0.564 |
| walker |  | 4384 | 51 | c decl doc at sds.c:526 |  |  | 0.564 |
| ns | 4516 |  | 350 | sdsReqType + sdsHdrSize — internal dispatch helpers | 3.7 |  | 0.532 |
| walker |  | 4587 | 203 | c decl body at sds.h:197 |  |  | 0.532 |
| walker |  | 4822 | 235 | c decl body at sds.h:130 |  |  | 0.532 |
| ns | 4974 |  | 458 | sds.h inline accessors — sdslen + sdsavail | 3.8 |  | 0.512 |
| walker |  | 5064 | 242 | c decl body at sds.h:104 |  |  | 0.575 |
| walker |  | 5314 | 250 | c decl body at sds.h:154 |  |  | 0.575 |
| walker |  | 5381 | 67 | c decl doc at sds.c:421 |  |  | 0.575 |
| walker |  | 5449 | 68 | c decl doc at sds.c:413 |  |  | 0.575 |
| ns | 5456 |  | 482 | Outdated v1 internals diagram in README | 3.9 |  | 0.546 |
| walker |  | 5518 | 69 | c decl doc at sds.c:193 |  |  | 0.546 |
| walker |  | 5615 | 97 | c decl body at sds.c:398 |  |  | 0.547 |
| walker |  | 5713 | 98 | c decl body at sds.c:427 |  |  | 0.548 |
| walker |  | 5786 | 73 | c decl doc at sds.c:380 |  |  | 0.548 |
| ns | 6189 |  | 733 | sdsMakeRoomFor body — growth/realloc engine | 4.1 |  | 0.512 |
| walker |  | 6226 | 440 | c decl names surface #1 in sds.c |  |  | 0.512 |
| walker |  | 6226 | 0 | c decl at sds.c:591 |  |  | 0.512 |
| walker |  | 6226 | 0 | c decl at sds.c:616 |  |  | 0.512 |
| walker |  | 6226 | 0 | c decl at sds.c:725 |  |  | 0.512 |
| walker |  | 6226 | 0 | c decl at sds.c:756 |  |  | 0.512 |
| walker |  | 6226 | 0 | c decl at sds.c:783 |  |  | 0.512 |
| walker |  | 6226 | 0 | c decl at sds.c:790 |  |  | 0.512 |
| walker |  | 6226 | 0 | c decl at sds.c:807 |  |  | 0.512 |
| walker |  | 6226 | 0 | c decl at sds.c:835 |  |  | 0.512 |
| walker |  | 6226 | 0 | c decl at sds.c:885 |  |  | 0.512 |
| walker |  | 6226 | 0 | c decl at sds.c:898 |  |  | 0.512 |
| walker |  | 6226 | 0 | c decl at sds.c:925 |  |  | 0.512 |
| walker |  | 6226 | 0 | c decl at sds.c:932 |  |  | 0.512 |
| walker |  | 6226 | 0 | c decl at sds.c:973 |  |  | 0.512 |
| walker |  | 6226 | 0 | c decl at sds.c:1092 |  |  | 0.512 |
| walker |  | 6226 | 0 | c decl at sds.c:1108 |  |  | 0.512 |
| walker |  | 6226 | 0 | c decl at sds.c:1120 |  |  | 0.512 |
| walker |  | 6226 | 0 | c decl at sds.c:1136 |  |  | 0.512 |
| walker |  | 6246 | 20 | c decl doc at sds.c:1120 |  |  | 0.512 |
| walker |  | 6267 | 21 | c decl doc at sds.c:783 |  |  | 0.512 |
| walker |  | 6288 | 21 | c decl doc at sds.c:790 |  |  | 0.512 |
| walker |  | 6314 | 26 | c decl doc at sds.c:885 |  |  | 0.512 |
| walker |  | 6351 | 37 | c decl body at sds.c:885 |  |  | 0.512 |
| walker |  | 6392 | 41 | c decl body at sds.c:783 |  |  | 0.512 |
| walker |  | 6433 | 41 | c decl body at sds.c:790 |  |  | 0.512 |
| walker |  | 6466 | 33 | c decl doc at sds.c:925 |  |  | 0.512 |
| walker |  | 6500 | 34 | c decl doc at sds.c:932 |  |  | 0.512 |
| walker |  | 6547 | 47 | c decl body at sds.c:925 |  |  | 0.512 |
| walker |  | 6585 | 38 | c decl doc at sds.c:1108 |  |  | 0.512 |
| walker |  | 6647 | 62 | c decl body at sds.c:591 |  |  | 0.512 |
| walker |  | 6743 | 96 | c decl body at sds.c:1108 |  |  | 0.512 |
| walker |  | 6843 | 100 | c decl body at sds.c:1120 |  |  | 0.512 |
| ns | 6852 |  | 663 | sdsnewlen body — canonical allocator (no doc-comment) | 4.2 |  | 0.479 |
| walker |  | 6928 | 85 | c decl doc at sds.c:398 |  |  | 0.479 |
| walker |  | 7016 | 88 | c decl doc at sds.c:300 |  |  | 0.479 |
| ns | 7067 |  | 215 | sdscatfmt — fast subset of printf, format spec list | 4.3 |  | 0.471 |
| walker |  | 7139 | 123 | c decl body at sds.c:380 |  |  | 0.471 |
| walker |  | 7264 | 125 | c decl body at sds.c:1092 |  |  | 0.471 |
| ns | 7347 |  | 280 | Constructor family + sdsfree (short bodies) | 4.4 |  | 0.481 |
| walker |  | 7350 | 86 | c decl doc at sds.c:450 |  |  | 0.481 |
| walker |  | 7481 | 131 | c decl body at sds.c:807 |  |  | 0.481 |
| walker |  | 7579 | 98 | c decl doc at sds.c:256 |  |  | 0.481 |
| walker |  | 7682 | 103 | c decl doc at sds.c:1136 |  |  | 0.481 |
| walker |  | 7792 | 110 | c decl doc at sds.c:898 |  |  | 0.481 |
| walker |  | 7903 | 111 | c decl doc at sds.c:204 |  |  | 0.483 |
| walker |  | 8053 | 150 | c decl body at sds.c:725 |  |  | 0.483 |
| ns | 8081 |  | 734 | README — Concatenating strings + sdsgrowzero | 4.5 |  | 0.454 |
| walker |  | 8193 | 140 | c decl doc at sds.c:807 |  |  | 0.454 |
| walker |  | 8727 | 534 | c header banner in testhelp.h |  |  | 0.454 |
| ns | 8792 |  | 711 | Append/copy primitive bodies (sdscatlen/cat/catsds/cpylen/cpy) | 5.1 |  | 0.480 |
| walker |  | 8876 | 149 | c decl doc at sds.c:1092 |  |  | 0.480 |
| ns | 9412 |  | 620 | sdsIncrLen body — zero-copy idiom's other half (no doc-comment) | 5.2 | 3.6 | 0.464 |
| walker |  | 9455 | 579 | c header banner in sdsalloc.h |  |  | 0.475 |
| ns | 9663 |  | 251 | testhelp.h — minimal test framework macros | 5.3 |  | 0.481 |
| walker |  | 9674 | 219 | c decl body at sds.c:494 |  |  | 0.481 |
| ns | 9909 |  | 246 | Makefile + Changelog | 5.4 |  | 0.474 |
