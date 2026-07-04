Score(3000)=0.662 I=0.803 C=0.546 ns_rows≤3K=18/34 (reached=11 partial=1 missing=6)

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 31 | 31 | listing of '.' |  |  | 1.000 |
| ns | 31 |  | 31 | Repo file listing | 1.1 |  | 1.000 |
| ns | 39 |  | 8 | README title | 1.2 |  | 0.932 |
| ns | 128 |  | 89 | README v2 lede — first paragraph (binary-compat warning) | 1.3 |  | 0.748 |
| walker |  | 138 | 107 | README headline in README.md |  |  | 1.000 |
| walker |  | 169 | 31 | c decl names surface in sdsalloc.h |  |  | 1.000 |
| ns | 252 |  | 124 | README v2 lede — perf note + sdscatfmt headline | 1.4 |  | 0.749 |
| ns | 262 |  | 10 | sds typedef — sds IS char* | 2.1 |  | 0.728 |
| ns | 293 |  | 31 | SDS_MAX_PREALLOC + SDS_NOINIT constants | 2.2 |  | 0.691 |
| ns | 469 |  | 176 | SDS_TYPE_* tag constants + SDS_HDR macros | 2.3 |  | 0.560 |
| walker |  | 568 | 399 | headings outline in README.md |  |  | 0.569 |
| walker |  | 624 | 56 | c decl names surface in testhelp.h |  |  | 0.569 |
| ns | 657 |  | 188 | Cardinal usage rule — must reassign return value | 2.4 |  | 0.481 |
| ns | 847 |  | 190 | Public fn declarations — high-level API (creation/length/free/concat/copy) | 2.5 |  | 0.422 |
| ns | 979 |  | 132 | Public fn declarations — printf family (sdscatvprintf/printf/fmt) | 2.6 |  | 0.386 |
| walker |  | 1065 | 441 | c decl names surface in sds.h |  |  | 0.573 |
| walker |  | 1065 | 0 | c decl at sds.h:87 |  |  | 0.573 |
| walker |  | 1065 | 0 | c decl at sds.h:104 |  |  | 0.573 |
| walker |  | 1065 | 0 | c decl at sds.h:130 |  |  | 0.573 |
| walker |  | 1065 | 0 | c decl at sds.h:154 |  |  | 0.573 |
| walker |  | 1065 | 0 | c decl at sds.h:180 |  |  | 0.573 |
| walker |  | 1065 | 0 | c decl at sds.h:197 |  |  | 0.573 |
| walker |  | 1102 | 37 | c decl at sds.h:47 |  |  | 0.574 |
| walker |  | 1122 | 20 | c decl doc at sds.h:180 |  |  | 0.574 |
| walker |  | 1187 | 65 | c decl at sds.h:51 |  |  | 0.576 |
| walker |  | 1252 | 65 | c decl at sds.h:57 |  |  | 0.579 |
| ns | 1276 |  | 297 | Public fn declarations — utility fns (trim/range/cmp/split/case/repr/join) | 2.7 |  | 0.512 |
| walker |  | 1317 | 65 | c decl at sds.h:63 |  |  | 0.516 |
| walker |  | 1382 | 65 | c decl at sds.h:69 |  |  | 0.520 |
| walker |  | 1425 | 43 | c decl doc at sds.h:47 |  |  | 0.523 |
| walker |  | 1457 | 32 | c includes in sds.h |  |  | 0.523 |
| ns | 1487 |  | 211 | Public fn declarations — low-level + allocator-export API | 2.8 |  | 0.476 |
| walker |  | 1906 | 449 | c decl names surface #1 in sds.h |  |  | 0.645 |
| ns | 1917 |  | 430 | Five packed sdshdr structs | 2.9 |  | 0.704 |
| walker |  | 1922 | 16 | c decl at sds.h:232 |  |  | 0.713 |
| walker |  | 2204 | 282 | c decl names surface #2 in sds.h |  |  | 0.794 |
| walker |  | 2204 | 0 | c decl at sds.h:256 |  |  | 0.794 |
| walker |  | 2204 | 0 | c decl at sds.h:266 |  |  | 0.794 |
| walker |  | 2218 | 14 | c decl doc at sds.h:256 |  |  | 0.801 |
| ns | 2230 |  | 313 | Canonical Hello World | 2.10 |  | 0.722 |
| walker |  | 2286 | 68 | c decl doc at sds.h:266 |  |  | 0.762 |
| ns | 2584 |  | 354 | Embedding + allocator-swap instructions | 2.11 |  | 0.690 |
| ns | 2706 |  | 122 | sdsalloc.h — full file | 2.12 | 2.11 | 0.670 |
| walker |  | 2779 | 493 | c header banner in sds.h |  |  | 0.670 |
| ns | 2823 |  | 117 | Error handling — NULL on OOM | 3.1 |  | 0.650 |
| walker |  | 2838 | 59 | c decl at testhelp.h:44 |  |  | 0.650 |
| ns | 2912 |  | 89 | README section TOC — first half (intro through copying) | 3.2 |  | 0.662 |
| walker |  | 3001 | 163 | c decl body at sds.h:87 |  |  | 0.663 |
| ns | 3011 |  | 99 | README section TOC — second half (quoting through credits) | 3.3 |  | 0.674 |
| walker |  | 3164 | 163 | c decl body at sds.h:180 |  |  | 0.674 |
| walker |  | 3261 | 97 | c decl at testhelp.h:48 |  |  | 0.675 |
| ns | 3492 |  | 481 | How SDS strings work — design narrative + ASCII diagram | 3.4 |  | 0.619 |
| walker |  | 3735 | 474 | c decl names surface in sds.c |  |  | 0.619 |
| walker |  | 3735 | 0 | c decl at sds.c:89 |  |  | 0.619 |
| walker |  | 3735 | 0 | c decl at sds.c:149 |  |  | 0.619 |
| walker |  | 3735 | 0 | c decl at sds.c:154 |  |  | 0.619 |
| walker |  | 3735 | 0 | c decl at sds.c:160 |  |  | 0.619 |
| walker |  | 3735 | 0 | c decl at sds.c:165 |  |  | 0.619 |
| walker |  | 3735 | 0 | c decl at sds.c:184 |  |  | 0.619 |
| walker |  | 3735 | 0 | c decl at sds.c:193 |  |  | 0.619 |
| walker |  | 3735 | 0 | c decl at sds.c:204 |  |  | 0.619 |
| walker |  | 3735 | 0 | c decl at sds.c:256 |  |  | 0.619 |
| walker |  | 3735 | 0 | c decl at sds.c:300 |  |  | 0.619 |
| walker |  | 3735 | 0 | c decl at sds.c:307 |  |  | 0.619 |
| walker |  | 3735 | 0 | c decl at sds.c:334 |  |  | 0.619 |
| walker |  | 3735 | 0 | c decl at sds.c:380 |  |  | 0.619 |
| walker |  | 3735 | 0 | c decl at sds.c:398 |  |  | 0.619 |
| walker |  | 3735 | 0 | c decl at sds.c:413 |  |  | 0.619 |
| walker |  | 3735 | 0 | c decl at sds.c:421 |  |  | 0.619 |
| walker |  | 3735 | 0 | c decl at sds.c:427 |  |  | 0.619 |
| walker |  | 3735 | 0 | c decl at sds.c:440 |  |  | 0.619 |
| walker |  | 3735 | 0 | c decl at sds.c:450 |  |  | 0.619 |
| walker |  | 3735 | 0 | c decl at sds.c:451 |  |  | 0.619 |
| walker |  | 3735 | 0 | c decl at sds.c:494 |  |  | 0.619 |
| walker |  | 3735 | 0 | c decl at sds.c:526 |  |  | 0.619 |
| walker |  | 3735 | 0 | c decl at sds.c:534 |  |  | 0.619 |
| walker |  | 3747 | 12 | c decl doc at sds.c:160 |  |  | 0.619 |
| walker |  | 3758 | 11 | c decl body at sds.c:149 |  |  | 0.619 |
| walker |  | 3778 | 20 | c decl doc at sds.c:154 |  |  | 0.619 |
| walker |  | 3799 | 21 | c decl doc at sds.c:494 |  |  | 0.619 |
| walker |  | 3821 | 22 | c decl doc at sds.c:534 |  |  | 0.619 |
| walker |  | 3836 | 15 | c decl body at sds.c:160 |  |  | 0.619 |
| walker |  | 3851 | 15 | c decl body at sds.c:413 |  |  | 0.619 |
| walker |  | 3866 | 15 | c decl body at sds.c:440 |  |  | 0.619 |
| walker |  | 3889 | 23 | c decl doc at sds.c:165 |  |  | 0.620 |
| ns | 3896 |  | 404 | Preallocation + SDS_MAX_PREALLOC growth strategy | 3.5 |  | 0.585 |
| walker |  | 3906 | 17 | c decl body at sds.c:307 |  |  | 0.585 |
| walker |  | 3923 | 17 | c decl body at sds.c:421 |  |  | 0.585 |
| walker |  | 3957 | 34 | c decl doc at sds.c:307 |  |  | 0.585 |
| walker |  | 3993 | 36 | c decl doc at sds.c:149 |  |  | 0.586 |
| walker |  | 4034 | 41 | c decl doc at sds.c:440 |  |  | 0.586 |
| walker |  | 4076 | 42 | c decl doc at sds.c:427 |  |  | 0.586 |
| walker |  | 4101 | 25 | c decl body at sds.c:184 |  |  | 0.586 |
| walker |  | 4126 | 25 | c decl body at sds.c:193 |  |  | 0.586 |
| ns | 4166 |  | 270 | Zero-copy append idiom | 3.6 |  | 0.563 |
| walker |  | 4177 | 51 | c decl doc at sds.c:526 |  |  | 0.563 |
| walker |  | 4207 | 30 | c decl body at sds.c:165 |  |  | 0.563 |
| walker |  | 4237 | 30 | c decl body at sds.c:300 |  |  | 0.563 |
| walker |  | 4272 | 35 | c decl body at sds.c:154 |  |  | 0.564 |
| walker |  | 4339 | 67 | c decl doc at sds.c:421 |  |  | 0.564 |
| walker |  | 4407 | 68 | c decl doc at sds.c:413 |  |  | 0.564 |
| walker |  | 4476 | 69 | c decl doc at sds.c:193 |  |  | 0.564 |
| ns | 4516 |  | 350 | sdsReqType + sdsHdrSize — internal dispatch helpers | 3.7 |  | 0.533 |
| walker |  | 4549 | 73 | c decl doc at sds.c:380 |  |  | 0.533 |
| walker |  | 4634 | 85 | c decl doc at sds.c:398 |  |  | 0.534 |
| walker |  | 4717 | 83 | c includes in sds.c |  |  | 0.534 |
| walker |  | 4805 | 88 | c decl doc at sds.c:300 |  |  | 0.534 |
| walker |  | 4850 | 45 | c decl body at sds.c:526 |  |  | 0.534 |
| walker |  | 4948 | 98 | c decl doc at sds.c:256 |  |  | 0.534 |
| ns | 4974 |  | 458 | sds.h inline accessors — sdslen + sdsavail | 3.8 |  | 0.513 |
| walker |  | 5034 | 86 | c decl doc at sds.c:450 |  |  | 0.513 |
| walker |  | 5145 | 111 | c decl doc at sds.c:204 |  |  | 0.513 |
| walker |  | 5348 | 203 | c decl body at sds.h:197 |  |  | 0.513 |
| ns | 5456 |  | 482 | Outdated v1 internals diagram in README | 3.9 |  | 0.487 |
| walker |  | 5546 | 198 | c decl doc at sds.c:184 |  |  | 0.487 |
| walker |  | 5781 | 235 | c decl body at sds.h:130 |  |  | 0.487 |
| walker |  | 5992 | 211 | c decl doc at sds.c:89 |  |  | 0.487 |
| ns | 6189 |  | 733 | sdsMakeRoomFor body — growth/realloc engine | 4.1 |  | 0.458 |
| walker |  | 6234 | 242 | c decl body at sds.h:104 |  |  | 0.514 |
| walker |  | 6484 | 250 | c decl body at sds.h:154 |  |  | 0.514 |
| walker |  | 6582 | 98 | c decl body at sds.c:427 |  |  | 0.515 |
| walker |  | 6684 | 102 | c decl body at sds.c:398 |  |  | 0.516 |
| ns | 6852 |  | 663 | sdsnewlen body — canonical allocator (no doc-comment) | 4.2 |  | 0.482 |
| ns | 7067 |  | 215 | sdscatfmt — fast subset of printf, format spec list | 4.3 |  | 0.474 |
| walker |  | 7124 | 440 | c decl names surface #1 in sds.c |  |  | 0.474 |
| walker |  | 7124 | 0 | c decl at sds.c:591 |  |  | 0.474 |
| walker |  | 7124 | 0 | c decl at sds.c:616 |  |  | 0.474 |
| walker |  | 7124 | 0 | c decl at sds.c:725 |  |  | 0.474 |
| walker |  | 7124 | 0 | c decl at sds.c:756 |  |  | 0.474 |
| walker |  | 7124 | 0 | c decl at sds.c:783 |  |  | 0.474 |
| walker |  | 7124 | 0 | c decl at sds.c:790 |  |  | 0.474 |
| walker |  | 7124 | 0 | c decl at sds.c:807 |  |  | 0.474 |
| walker |  | 7124 | 0 | c decl at sds.c:835 |  |  | 0.474 |
| walker |  | 7124 | 0 | c decl at sds.c:885 |  |  | 0.474 |
| walker |  | 7124 | 0 | c decl at sds.c:898 |  |  | 0.474 |
| walker |  | 7124 | 0 | c decl at sds.c:925 |  |  | 0.474 |
| walker |  | 7124 | 0 | c decl at sds.c:932 |  |  | 0.474 |
| walker |  | 7124 | 0 | c decl at sds.c:973 |  |  | 0.474 |
| walker |  | 7124 | 0 | c decl at sds.c:1092 |  |  | 0.474 |
| walker |  | 7124 | 0 | c decl at sds.c:1108 |  |  | 0.474 |
| walker |  | 7124 | 0 | c decl at sds.c:1120 |  |  | 0.474 |
| walker |  | 7124 | 0 | c decl at sds.c:1136 |  |  | 0.474 |
| walker |  | 7144 | 20 | c decl doc at sds.c:1120 |  |  | 0.474 |
| walker |  | 7165 | 21 | c decl doc at sds.c:783 |  |  | 0.474 |
| walker |  | 7186 | 21 | c decl doc at sds.c:790 |  |  | 0.474 |
| walker |  | 7212 | 26 | c decl doc at sds.c:885 |  |  | 0.474 |
| walker |  | 7245 | 33 | c decl doc at sds.c:925 |  |  | 0.474 |
| walker |  | 7279 | 34 | c decl doc at sds.c:932 |  |  | 0.474 |
| walker |  | 7317 | 38 | c decl doc at sds.c:1108 |  |  | 0.474 |
| ns | 7347 |  | 280 | Constructor family + sdsfree (short bodies) | 4.4 |  | 0.483 |
| walker |  | 7354 | 37 | c decl body at sds.c:885 |  |  | 0.483 |
| walker |  | 7400 | 46 | c decl body at sds.c:783 |  |  | 0.483 |
| walker |  | 7446 | 46 | c decl body at sds.c:790 |  |  | 0.483 |
| walker |  | 7493 | 47 | c decl body at sds.c:925 |  |  | 0.483 |
| walker |  | 7596 | 103 | c decl doc at sds.c:1136 |  |  | 0.483 |
| walker |  | 7706 | 110 | c decl doc at sds.c:898 |  |  | 0.483 |
| walker |  | 7768 | 62 | c decl body at sds.c:591 |  |  | 0.483 |
| walker |  | 7908 | 140 | c decl doc at sds.c:807 |  |  | 0.483 |
| walker |  | 8057 | 149 | c decl doc at sds.c:1092 |  |  | 0.483 |
| ns | 8081 |  | 734 | README — Concatenating strings + sdsgrowzero | 4.5 |  | 0.454 |
| walker |  | 8234 | 177 | c decl doc at sds.c:725 |  |  | 0.454 |
| walker |  | 8424 | 190 | c decl doc at sds.c:756 |  |  | 0.454 |
| walker |  | 8633 | 209 | c decl doc at sds.c:835 |  |  | 0.454 |
| ns | 8792 |  | 711 | Append/copy primitive bodies (sdscatlen/cat/catsds/cpylen/cpy) | 5.1 |  | 0.481 |
| walker |  | 8848 | 215 | c decl doc at sds.c:591 |  |  | 0.481 |
| walker |  | 9063 | 215 | c decl doc at sds.c:616 |  |  | 0.500 |
| walker |  | 9318 | 255 | c decl doc at sds.c:973 |  |  | 0.500 |
| ns | 9412 |  | 620 | sdsIncrLen body — zero-copy idiom's other half (no doc-comment) | 5.2 | 3.6 | 0.483 |
| walker |  | 9420 | 102 | c decl body at sds.c:1108 |  |  | 0.483 |
| walker |  | 9526 | 106 | c decl body at sds.c:1120 |  |  | 0.483 |
| ns | 9663 |  | 251 | testhelp.h — minimal test framework macros | 5.3 |  | 0.489 |
| walker |  | 9848 | 322 | c decl doc at sds.c:334 |  |  | 0.489 |
| ns | 9909 |  | 246 | Makefile + Changelog | 5.4 |  | 0.481 |
| walker |  | 9979 | 131 | c decl body at sds.c:1092 |  |  | 0.481 |
