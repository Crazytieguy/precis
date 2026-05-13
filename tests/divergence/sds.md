Score(3000)=0.647 I=0.794 C=0.527 ns_rows≤3K=18/34 (reached=11 partial=0 missing=7)

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 31 | 31 | listing of '.' |  |  | 1.000 |
| ns | 31 |  | 31 | Repo file listing | 1.1 |  | 1.000 |
| ns | 39 |  | 8 | README title | 1.2 |  | 0.932 |
| walker |  | 62 | 31 | c decl names surface in sdsalloc.h |  |  | 0.933 |
| walker |  | 118 | 56 | c decl names surface in testhelp.h |  |  | 0.933 |
| ns | 128 |  | 89 | README v2 lede — first paragraph (binary-compat warning) | 1.3 |  | 0.748 |
| walker |  | 220 | 102 | README headline in README.md |  |  | 1.000 |
| ns | 252 |  | 124 | README v2 lede — perf note + sdscatfmt headline | 1.4 |  | 0.749 |
| ns | 262 |  | 10 | sds typedef — sds IS char* | 2.1 |  | 0.728 |
| walker |  | 279 | 59 | c decl at testhelp.h:44 |  |  | 0.729 |
| ns | 293 |  | 31 | SDS_MAX_PREALLOC + SDS_NOINIT constants | 2.2 |  | 0.692 |
| walker |  | 311 | 32 | c includes in sds.h |  |  | 0.692 |
| walker |  | 319 | 8 | plaintext config .gitignore |  |  | 0.692 |
| walker |  | 416 | 97 | c decl at testhelp.h:48 |  |  | 0.693 |
| ns | 469 |  | 176 | SDS_TYPE_* tag constants + SDS_HDR macros | 2.3 |  | 0.562 |
| ns | 657 |  | 188 | Cardinal usage rule — must reassign return value | 2.4 |  | 0.475 |
| walker |  | 815 | 399 | headings outline in README.md |  |  | 0.482 |
| ns | 847 |  | 190 | Public fn declarations — high-level API (creation/length/free/concat/copy) | 2.5 |  | 0.423 |
| ns | 979 |  | 132 | Public fn declarations — printf family (sdscatvprintf/printf/fmt) | 2.6 |  | 0.387 |
| ns | 1276 |  | 297 | Public fn declarations — utility fns (trim/range/cmp/split/case/repr/join) | 2.7 |  | 0.342 |
| ns | 1487 |  | 211 | Public fn declarations — low-level + allocator-export API | 2.8 |  | 0.311 |
| ns | 1917 |  | 430 | Five packed sdshdr structs | 2.9 |  | 0.264 |
| walker |  | 1919 | 1104 | c decl names surface in sds.h |  |  | 0.615 |
| walker |  | 1919 | 0 | c decl at sds.h:87 |  |  | 0.615 |
| walker |  | 1919 | 0 | c decl at sds.h:104 |  |  | 0.615 |
| walker |  | 1919 | 0 | c decl at sds.h:130 |  |  | 0.615 |
| walker |  | 1919 | 0 | c decl at sds.h:154 |  |  | 0.615 |
| walker |  | 1919 | 0 | c decl at sds.h:180 |  |  | 0.615 |
| walker |  | 1919 | 0 | c decl at sds.h:197 |  |  | 0.615 |
| walker |  | 1919 | 0 | c decl at sds.h:256 |  |  | 0.615 |
| walker |  | 1919 | 0 | c decl at sds.h:266 |  |  | 0.615 |
| walker |  | 1956 | 37 | c decl at sds.h:47 |  |  | 0.622 |
| walker |  | 1970 | 14 | c decl doc at sds.h:256 |  |  | 0.630 |
| walker |  | 1990 | 20 | c decl doc at sds.h:180 |  |  | 0.630 |
| walker |  | 2055 | 65 | c decl at sds.h:51 |  |  | 0.650 |
| walker |  | 2120 | 65 | c decl at sds.h:57 |  |  | 0.677 |
| walker |  | 2185 | 65 | c decl at sds.h:63 |  |  | 0.711 |
| ns | 2230 |  | 313 | Canonical Hello World | 2.10 |  | 0.641 |
| walker |  | 2250 | 65 | c decl at sds.h:69 |  |  | 0.678 |
| walker |  | 2293 | 43 | c decl doc at sds.h:47 |  |  | 0.704 |
| walker |  | 2361 | 68 | c decl doc at sds.h:266 |  |  | 0.743 |
| ns | 2584 |  | 354 | Embedding + allocator-swap instructions | 2.11 |  | 0.673 |
| ns | 2706 |  | 122 | sdsalloc.h — full file | 2.12 | 2.11 | 0.653 |
| ns | 2823 |  | 117 | Error handling — NULL on OOM | 3.1 |  | 0.634 |
| walker |  | 2854 | 493 | c header banner in sds.h |  |  | 0.634 |
| ns | 2912 |  | 89 | README section TOC — first half (intro through copying) | 3.2 |  | 0.647 |
| ns | 3011 |  | 99 | README section TOC — second half (quoting through credits) | 3.3 |  | 0.658 |
| walker |  | 3388 | 534 | c header banner in testhelp.h |  |  | 0.658 |
| walker |  | 3471 | 83 | c includes in sds.c |  |  | 0.658 |
| ns | 3492 |  | 481 | How SDS strings work — design narrative + ASCII diagram | 3.4 |  | 0.604 |
| ns | 3896 |  | 404 | Preallocation + SDS_MAX_PREALLOC growth strategy | 3.5 |  | 0.570 |
| walker |  | 4050 | 579 | c header banner in sdsalloc.h |  |  | 0.593 |
| ns | 4166 |  | 270 | Zero-copy append idiom | 3.6 |  | 0.569 |
| walker |  | 4213 | 163 | c decl body at sds.h:87 |  |  | 0.571 |
| walker |  | 4376 | 163 | c decl body at sds.h:180 |  |  | 0.571 |
| ns | 4516 |  | 350 | sdsReqType + sdsHdrSize — internal dispatch helpers | 3.7 |  | 0.539 |
| walker |  | 4579 | 203 | c decl body at sds.h:197 |  |  | 0.539 |
| walker |  | 4814 | 235 | c decl body at sds.h:130 |  |  | 0.539 |
| ns | 4974 |  | 458 | sds.h inline accessors — sdslen + sdsavail | 3.8 |  | 0.517 |
| walker |  | 5056 | 242 | c decl body at sds.h:104 |  |  | 0.579 |
| walker |  | 5306 | 250 | c decl body at sds.h:154 |  |  | 0.579 |
| ns | 5456 |  | 482 | Outdated v1 internals diagram in README | 3.9 |  | 0.550 |
| ns | 6189 |  | 733 | sdsMakeRoomFor body — growth/realloc engine | 4.1 |  | 0.514 |
| walker |  | 6220 | 914 | c decl names surface in sds.c |  |  | 0.514 |
| walker |  | 6220 | 0 | c decl at sds.c:89 |  |  | 0.514 |
| walker |  | 6220 | 0 | c decl at sds.c:149 |  |  | 0.514 |
| walker |  | 6220 | 0 | c decl at sds.c:154 |  |  | 0.514 |
| walker |  | 6220 | 0 | c decl at sds.c:160 |  |  | 0.514 |
| walker |  | 6220 | 0 | c decl at sds.c:165 |  |  | 0.514 |
| walker |  | 6220 | 0 | c decl at sds.c:184 |  |  | 0.514 |
| walker |  | 6220 | 0 | c decl at sds.c:193 |  |  | 0.514 |
| walker |  | 6220 | 0 | c decl at sds.c:204 |  |  | 0.514 |
| walker |  | 6220 | 0 | c decl at sds.c:256 |  |  | 0.514 |
| walker |  | 6220 | 0 | c decl at sds.c:300 |  |  | 0.514 |
| walker |  | 6220 | 0 | c decl at sds.c:307 |  |  | 0.514 |
| walker |  | 6220 | 0 | c decl at sds.c:334 |  |  | 0.514 |
| walker |  | 6220 | 0 | c decl at sds.c:380 |  |  | 0.514 |
| walker |  | 6220 | 0 | c decl at sds.c:398 |  |  | 0.514 |
| walker |  | 6220 | 0 | c decl at sds.c:413 |  |  | 0.514 |
| walker |  | 6220 | 0 | c decl at sds.c:421 |  |  | 0.514 |
| walker |  | 6220 | 0 | c decl at sds.c:427 |  |  | 0.514 |
| walker |  | 6220 | 0 | c decl at sds.c:440 |  |  | 0.514 |
| walker |  | 6220 | 0 | c decl at sds.c:450 |  |  | 0.514 |
| walker |  | 6220 | 0 | c decl at sds.c:451 |  |  | 0.514 |
| walker |  | 6220 | 0 | c decl at sds.c:494 |  |  | 0.514 |
| walker |  | 6220 | 0 | c decl at sds.c:526 |  |  | 0.514 |
| walker |  | 6220 | 0 | c decl at sds.c:534 |  |  | 0.514 |
| walker |  | 6220 | 0 | c decl at sds.c:591 |  |  | 0.514 |
| walker |  | 6220 | 0 | c decl at sds.c:616 |  |  | 0.514 |
| walker |  | 6220 | 0 | c decl at sds.c:725 |  |  | 0.514 |
| walker |  | 6220 | 0 | c decl at sds.c:756 |  |  | 0.514 |
| walker |  | 6220 | 0 | c decl at sds.c:783 |  |  | 0.514 |
| walker |  | 6220 | 0 | c decl at sds.c:790 |  |  | 0.514 |
| walker |  | 6220 | 0 | c decl at sds.c:807 |  |  | 0.514 |
| walker |  | 6220 | 0 | c decl at sds.c:835 |  |  | 0.514 |
| walker |  | 6220 | 0 | c decl at sds.c:885 |  |  | 0.514 |
| walker |  | 6220 | 0 | c decl at sds.c:898 |  |  | 0.514 |
| walker |  | 6220 | 0 | c decl at sds.c:925 |  |  | 0.514 |
| walker |  | 6220 | 0 | c decl at sds.c:932 |  |  | 0.514 |
| walker |  | 6220 | 0 | c decl at sds.c:973 |  |  | 0.514 |
| walker |  | 6220 | 0 | c decl at sds.c:1092 |  |  | 0.514 |
| walker |  | 6220 | 0 | c decl at sds.c:1108 |  |  | 0.514 |
| walker |  | 6220 | 0 | c decl at sds.c:1120 |  |  | 0.514 |
| walker |  | 6220 | 0 | c decl at sds.c:1136 |  |  | 0.514 |
| walker |  | 6232 | 12 | c decl doc at sds.c:160 |  |  | 0.514 |
| walker |  | 6243 | 11 | c decl body at sds.c:149 |  |  | 0.514 |
| walker |  | 6263 | 20 | c decl doc at sds.c:154 |  |  | 0.514 |
| walker |  | 6283 | 20 | c decl doc at sds.c:1120 |  |  | 0.514 |
| walker |  | 6304 | 21 | c decl doc at sds.c:494 |  |  | 0.514 |
| walker |  | 6325 | 21 | c decl doc at sds.c:783 |  |  | 0.514 |
| walker |  | 6346 | 21 | c decl doc at sds.c:790 |  |  | 0.514 |
| walker |  | 6368 | 22 | c decl doc at sds.c:534 |  |  | 0.514 |
| walker |  | 6383 | 15 | c decl body at sds.c:160 |  |  | 0.515 |
| walker |  | 6398 | 15 | c decl body at sds.c:413 |  |  | 0.515 |
| walker |  | 6413 | 15 | c decl body at sds.c:440 |  |  | 0.515 |
| walker |  | 6436 | 23 | c decl doc at sds.c:165 |  |  | 0.515 |
| walker |  | 6462 | 26 | c decl doc at sds.c:885 |  |  | 0.515 |
| walker |  | 6479 | 17 | c decl body at sds.c:307 |  |  | 0.515 |
| walker |  | 6496 | 17 | c decl body at sds.c:421 |  |  | 0.515 |
| walker |  | 6529 | 33 | c decl doc at sds.c:925 |  |  | 0.515 |
| walker |  | 6563 | 34 | c decl doc at sds.c:307 |  |  | 0.515 |
| walker |  | 6597 | 34 | c decl doc at sds.c:932 |  |  | 0.515 |
| walker |  | 6633 | 36 | c decl doc at sds.c:149 |  |  | 0.515 |
| walker |  | 6671 | 38 | c decl doc at sds.c:1108 |  |  | 0.515 |
| walker |  | 6712 | 41 | c decl doc at sds.c:440 |  |  | 0.515 |
| walker |  | 6754 | 42 | c decl doc at sds.c:427 |  |  | 0.515 |
| walker |  | 6779 | 25 | c decl body at sds.c:184 |  |  | 0.515 |
| walker |  | 6804 | 25 | c decl body at sds.c:193 |  |  | 0.515 |
| ns | 6852 |  | 663 | sdsnewlen body — canonical allocator (no doc-comment) | 4.2 |  | 0.482 |
| walker |  | 6855 | 51 | c decl doc at sds.c:526 |  |  | 0.482 |
| walker |  | 6885 | 30 | c decl body at sds.c:165 |  |  | 0.482 |
| walker |  | 6915 | 30 | c decl body at sds.c:300 |  |  | 0.482 |
| walker |  | 6950 | 35 | c decl body at sds.c:154 |  |  | 0.482 |
| walker |  | 7017 | 67 | c decl doc at sds.c:421 |  |  | 0.483 |
| ns | 7067 |  | 215 | sdscatfmt — fast subset of printf, format spec list | 4.3 |  | 0.474 |
| walker |  | 7085 | 68 | c decl doc at sds.c:413 |  |  | 0.475 |
| walker |  | 7154 | 69 | c decl doc at sds.c:193 |  |  | 0.475 |
| walker |  | 7191 | 37 | c decl body at sds.c:885 |  |  | 0.475 |
| walker |  | 7264 | 73 | c decl doc at sds.c:380 |  |  | 0.475 |
| walker |  | 7304 | 40 | c decl body at sds.c:526 |  |  | 0.475 |
| walker |  | 7345 | 41 | c decl body at sds.c:783 |  |  | 0.475 |
| ns | 7347 |  | 280 | Constructor family + sdsfree (short bodies) | 4.4 |  | 0.484 |
| walker |  | 7386 | 41 | c decl body at sds.c:790 |  |  | 0.484 |
| walker |  | 7471 | 85 | c decl doc at sds.c:398 |  |  | 0.484 |
| walker |  | 7559 | 88 | c decl doc at sds.c:300 |  |  | 0.484 |
| walker |  | 7606 | 47 | c decl body at sds.c:925 |  |  | 0.484 |
| walker |  | 7704 | 98 | c decl doc at sds.c:256 |  |  | 0.484 |
| walker |  | 7790 | 86 | c decl doc at sds.c:450 |  |  | 0.484 |
| walker |  | 7893 | 103 | c decl doc at sds.c:1136 |  |  | 0.484 |
| walker |  | 8003 | 110 | c decl doc at sds.c:898 |  |  | 0.484 |
| ns | 8081 |  | 734 | README — Concatenating strings + sdsgrowzero | 4.5 |  | 0.455 |
| walker |  | 8114 | 111 | c decl doc at sds.c:204 |  |  | 0.457 |
| walker |  | 8176 | 62 | c decl body at sds.c:591 |  |  | 0.457 |
| walker |  | 8316 | 140 | c decl doc at sds.c:807 |  |  | 0.457 |
| walker |  | 8465 | 149 | c decl doc at sds.c:1092 |  |  | 0.457 |
| walker |  | 8642 | 177 | c decl doc at sds.c:725 |  |  | 0.457 |
| ns | 8792 |  | 711 | Append/copy primitive bodies (sdscatlen/cat/catsds/cpylen/cpy) | 5.1 |  | 0.460 |
| walker |  | 8832 | 190 | c decl doc at sds.c:756 |  |  | 0.460 |
| walker |  | 9030 | 198 | c decl doc at sds.c:184 |  |  | 0.460 |
| walker |  | 9239 | 209 | c decl doc at sds.c:835 |  |  | 0.460 |
| ns | 9412 |  | 620 | sdsIncrLen body — zero-copy idiom's other half (no doc-comment) | 5.2 | 3.6 | 0.445 |
| walker |  | 9450 | 211 | c decl doc at sds.c:89 |  |  | 0.445 |
| ns | 9663 |  | 251 | testhelp.h — minimal test framework macros | 5.3 |  | 0.451 |
| walker |  | 9665 | 215 | c decl doc at sds.c:591 |  |  | 0.451 |
| walker |  | 9880 | 215 | c decl doc at sds.c:616 |  |  | 0.469 |
| ns | 9909 |  | 246 | Makefile + Changelog | 5.4 |  | 0.462 |
| walker |  | 9976 | 96 | c decl body at sds.c:1108 |  |  | 0.462 |
