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
| walker |  | 1114 | 20 | c decl doc at sds.h:180 |  |  | 0.574 |
| walker |  | 1179 | 65 | c decl at sds.h:51 |  |  | 0.576 |
| walker |  | 1244 | 65 | c decl at sds.h:57 |  |  | 0.579 |
| ns | 1276 |  | 297 | Public fn declarations — utility fns (trim/range/cmp/split/case/repr/join) | 2.7 |  | 0.512 |
| walker |  | 1309 | 65 | c decl at sds.h:63 |  |  | 0.516 |
| walker |  | 1374 | 65 | c decl at sds.h:69 |  |  | 0.520 |
| walker |  | 1417 | 43 | c decl doc at sds.h:47 |  |  | 0.523 |
| walker |  | 1449 | 32 | c includes in sds.h |  |  | 0.523 |
| ns | 1487 |  | 211 | Public fn declarations — low-level + allocator-export API | 2.8 |  | 0.476 |
| walker |  | 1895 | 446 | c decl names surface #1 in sds.h |  |  | 0.645 |
| walker |  | 1911 | 16 | c decl at sds.h:232 |  |  | 0.657 |
| ns | 1917 |  | 430 | Five packed sdshdr structs | 2.9 |  | 0.713 |
| walker |  | 2193 | 282 | c decl names surface #2 in sds.h |  |  | 0.794 |
| walker |  | 2193 | 0 | c decl at sds.h:256 |  |  | 0.794 |
| walker |  | 2193 | 0 | c decl at sds.h:266 |  |  | 0.794 |
| walker |  | 2207 | 14 | c decl doc at sds.h:256 |  |  | 0.801 |
| ns | 2230 |  | 313 | Canonical Hello World | 2.10 |  | 0.722 |
| walker |  | 2275 | 68 | c decl doc at sds.h:266 |  |  | 0.762 |
| ns | 2584 |  | 354 | Embedding + allocator-swap instructions | 2.11 |  | 0.690 |
| ns | 2706 |  | 122 | sdsalloc.h — full file | 2.12 | 2.11 | 0.670 |
| walker |  | 2768 | 493 | c header banner in sds.h |  |  | 0.670 |
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
| walker |  | 3736 | 12 | c decl doc at sds.c:160 |  |  | 0.619 |
| walker |  | 3747 | 11 | c decl body at sds.c:149 |  |  | 0.619 |
| walker |  | 3767 | 20 | c decl doc at sds.c:154 |  |  | 0.619 |
| walker |  | 3788 | 21 | c decl doc at sds.c:494 |  |  | 0.619 |
| walker |  | 3810 | 22 | c decl doc at sds.c:534 |  |  | 0.619 |
| walker |  | 3825 | 15 | c decl body at sds.c:160 |  |  | 0.619 |
| walker |  | 3840 | 15 | c decl body at sds.c:413 |  |  | 0.619 |
| walker |  | 3855 | 15 | c decl body at sds.c:440 |  |  | 0.619 |
| walker |  | 3878 | 23 | c decl doc at sds.c:165 |  |  | 0.620 |
| walker |  | 3895 | 17 | c decl body at sds.c:307 |  |  | 0.620 |
| ns | 3896 |  | 404 | Preallocation + SDS_MAX_PREALLOC growth strategy | 3.5 |  | 0.585 |
| walker |  | 3912 | 17 | c decl body at sds.c:421 |  |  | 0.585 |
| walker |  | 3946 | 34 | c decl doc at sds.c:307 |  |  | 0.585 |
| walker |  | 3982 | 36 | c decl doc at sds.c:149 |  |  | 0.586 |
| walker |  | 4023 | 41 | c decl doc at sds.c:440 |  |  | 0.586 |
| walker |  | 4065 | 42 | c decl doc at sds.c:427 |  |  | 0.586 |
| walker |  | 4090 | 25 | c decl body at sds.c:184 |  |  | 0.586 |
| walker |  | 4115 | 25 | c decl body at sds.c:193 |  |  | 0.586 |
| walker |  | 4166 | 51 | c decl doc at sds.c:526 |  |  | 0.563 |
| ns | 4166 |  | 270 | Zero-copy append idiom | 3.6 |  | 0.563 |
| walker |  | 4196 | 30 | c decl body at sds.c:165 |  |  | 0.563 |
| walker |  | 4226 | 30 | c decl body at sds.c:300 |  |  | 0.563 |
| walker |  | 4261 | 35 | c decl body at sds.c:154 |  |  | 0.564 |
| walker |  | 4328 | 67 | c decl doc at sds.c:421 |  |  | 0.564 |
| walker |  | 4396 | 68 | c decl doc at sds.c:413 |  |  | 0.564 |
| walker |  | 4465 | 69 | c decl doc at sds.c:193 |  |  | 0.564 |
| ns | 4516 |  | 350 | sdsReqType + sdsHdrSize — internal dispatch helpers | 3.7 |  | 0.533 |
| walker |  | 4538 | 73 | c decl doc at sds.c:380 |  |  | 0.533 |
| walker |  | 4578 | 40 | c decl body at sds.c:526 |  |  | 0.533 |
| walker |  | 4663 | 85 | c decl doc at sds.c:398 |  |  | 0.534 |
| walker |  | 4746 | 83 | c includes in sds.c |  |  | 0.534 |
| walker |  | 4834 | 88 | c decl doc at sds.c:300 |  |  | 0.534 |
| walker |  | 4932 | 98 | c decl doc at sds.c:256 |  |  | 0.534 |
| ns | 4974 |  | 458 | sds.h inline accessors — sdslen + sdsavail | 3.8 |  | 0.513 |
| walker |  | 5018 | 86 | c decl doc at sds.c:450 |  |  | 0.513 |
| walker |  | 5129 | 111 | c decl doc at sds.c:204 |  |  | 0.513 |
| walker |  | 5332 | 203 | c decl body at sds.h:197 |  |  | 0.513 |
| ns | 5456 |  | 482 | Outdated v1 internals diagram in README | 3.9 |  | 0.487 |
| walker |  | 5530 | 198 | c decl doc at sds.c:184 |  |  | 0.487 |
| walker |  | 5765 | 235 | c decl body at sds.h:130 |  |  | 0.487 |
| walker |  | 5976 | 211 | c decl doc at sds.c:89 |  |  | 0.487 |
| ns | 6189 |  | 733 | sdsMakeRoomFor body — growth/realloc engine | 4.1 |  | 0.458 |
| walker |  | 6218 | 242 | c decl body at sds.h:104 |  |  | 0.514 |
| walker |  | 6468 | 250 | c decl body at sds.h:154 |  |  | 0.514 |
| walker |  | 6565 | 97 | c decl body at sds.c:398 |  |  | 0.515 |
| walker |  | 6663 | 98 | c decl body at sds.c:427 |  |  | 0.515 |
| ns | 6852 |  | 663 | sdsnewlen body — canonical allocator (no doc-comment) | 4.2 |  | 0.482 |
| ns | 7067 |  | 215 | sdscatfmt — fast subset of printf, format spec list | 4.3 |  | 0.474 |
| walker |  | 7103 | 440 | c decl names surface #1 in sds.c |  |  | 0.474 |
| walker |  | 7103 | 0 | c decl at sds.c:591 |  |  | 0.474 |
| walker |  | 7103 | 0 | c decl at sds.c:616 |  |  | 0.474 |
| walker |  | 7103 | 0 | c decl at sds.c:725 |  |  | 0.474 |
| walker |  | 7103 | 0 | c decl at sds.c:756 |  |  | 0.474 |
| walker |  | 7103 | 0 | c decl at sds.c:783 |  |  | 0.474 |
| walker |  | 7103 | 0 | c decl at sds.c:790 |  |  | 0.474 |
| walker |  | 7103 | 0 | c decl at sds.c:807 |  |  | 0.474 |
| walker |  | 7103 | 0 | c decl at sds.c:835 |  |  | 0.474 |
| walker |  | 7103 | 0 | c decl at sds.c:885 |  |  | 0.474 |
| walker |  | 7103 | 0 | c decl at sds.c:898 |  |  | 0.474 |
| walker |  | 7103 | 0 | c decl at sds.c:925 |  |  | 0.474 |
| walker |  | 7103 | 0 | c decl at sds.c:932 |  |  | 0.474 |
| walker |  | 7103 | 0 | c decl at sds.c:973 |  |  | 0.474 |
| walker |  | 7103 | 0 | c decl at sds.c:1092 |  |  | 0.474 |
| walker |  | 7103 | 0 | c decl at sds.c:1108 |  |  | 0.474 |
| walker |  | 7103 | 0 | c decl at sds.c:1120 |  |  | 0.474 |
| walker |  | 7103 | 0 | c decl at sds.c:1136 |  |  | 0.474 |
| walker |  | 7123 | 20 | c decl doc at sds.c:1120 |  |  | 0.474 |
| walker |  | 7144 | 21 | c decl doc at sds.c:783 |  |  | 0.474 |
| walker |  | 7165 | 21 | c decl doc at sds.c:790 |  |  | 0.474 |
| walker |  | 7191 | 26 | c decl doc at sds.c:885 |  |  | 0.474 |
| walker |  | 7224 | 33 | c decl doc at sds.c:925 |  |  | 0.474 |
| walker |  | 7258 | 34 | c decl doc at sds.c:932 |  |  | 0.474 |
| walker |  | 7296 | 38 | c decl doc at sds.c:1108 |  |  | 0.474 |
| walker |  | 7333 | 37 | c decl body at sds.c:885 |  |  | 0.474 |
| ns | 7347 |  | 280 | Constructor family + sdsfree (short bodies) | 4.4 |  | 0.483 |
| walker |  | 7374 | 41 | c decl body at sds.c:783 |  |  | 0.483 |
| walker |  | 7415 | 41 | c decl body at sds.c:790 |  |  | 0.483 |
| walker |  | 7462 | 47 | c decl body at sds.c:925 |  |  | 0.483 |
| walker |  | 7565 | 103 | c decl doc at sds.c:1136 |  |  | 0.483 |
| walker |  | 7675 | 110 | c decl doc at sds.c:898 |  |  | 0.483 |
| walker |  | 7737 | 62 | c decl body at sds.c:591 |  |  | 0.483 |
| walker |  | 7877 | 140 | c decl doc at sds.c:807 |  |  | 0.483 |
| walker |  | 8026 | 149 | c decl doc at sds.c:1092 |  |  | 0.483 |
| ns | 8081 |  | 734 | README — Concatenating strings + sdsgrowzero | 4.5 |  | 0.454 |
| walker |  | 8203 | 177 | c decl doc at sds.c:725 |  |  | 0.454 |
| walker |  | 8393 | 190 | c decl doc at sds.c:756 |  |  | 0.454 |
| walker |  | 8602 | 209 | c decl doc at sds.c:835 |  |  | 0.454 |
| ns | 8792 |  | 711 | Append/copy primitive bodies (sdscatlen/cat/catsds/cpylen/cpy) | 5.1 |  | 0.480 |
| walker |  | 8817 | 215 | c decl doc at sds.c:591 |  |  | 0.480 |
| walker |  | 9032 | 215 | c decl doc at sds.c:616 |  |  | 0.498 |
| walker |  | 9128 | 96 | c decl body at sds.c:1108 |  |  | 0.498 |
| walker |  | 9228 | 100 | c decl body at sds.c:1120 |  |  | 0.498 |
| ns | 9412 |  | 620 | sdsIncrLen body — zero-copy idiom's other half (no doc-comment) | 5.2 | 3.6 | 0.482 |
| walker |  | 9483 | 255 | c decl doc at sds.c:973 |  |  | 0.482 |
| ns | 9663 |  | 251 | testhelp.h — minimal test framework macros | 5.3 |  | 0.488 |
| walker |  | 9805 | 322 | c decl doc at sds.c:334 |  |  | 0.488 |
| ns | 9909 |  | 246 | Makefile + Changelog | 5.4 |  | 0.480 |
| walker |  | 9928 | 123 | c decl body at sds.c:380 |  |  | 0.480 |
