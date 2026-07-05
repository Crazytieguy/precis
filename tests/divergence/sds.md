Score(3000)=0.662 I=0.803 C=0.546 ns_rows≤3K=18/34 (reached=11 partial=1 missing=6)

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 31 | 31 | listing of '.' |  |  | 1.000 |
| ns | 31 |  | 31 | Repo file listing | 1.1 |  | 1.000 |
| ns | 41 |  | 10 | README title | 1.2 |  | 0.932 |
| ns | 132 |  | 91 | README v2 lede — first paragraph (binary-compat warning) | 1.3 |  | 0.748 |
| walker |  | 140 | 109 | README headline in README.md |  |  | 1.000 |
| walker |  | 173 | 33 | c decl names surface in sdsalloc.h |  |  | 1.000 |
| ns | 256 |  | 124 | README v2 lede — perf note + sdscatfmt headline | 1.4 |  | 0.749 |
| ns | 270 |  | 14 | sds typedef — sds IS char* | 2.1 |  | 0.728 |
| ns | 303 |  | 33 | SDS_MAX_PREALLOC + SDS_NOINIT constants | 2.2 |  | 0.691 |
| ns | 481 |  | 178 | SDS_TYPE_* tag constants + SDS_HDR macros | 2.3 |  | 0.560 |
| walker |  | 572 | 399 | headings outline in README.md |  |  | 0.569 |
| walker |  | 630 | 58 | c decl names surface in testhelp.h |  |  | 0.569 |
| ns | 671 |  | 190 | Cardinal usage rule — must reassign return value | 2.4 |  | 0.481 |
| ns | 863 |  | 192 | Public fn declarations — high-level API (creation/length/free/concat/copy) | 2.5 |  | 0.422 |
| ns | 995 |  | 132 | Public fn declarations — printf family (sdscatvprintf/printf/fmt) | 2.6 |  | 0.386 |
| walker |  | 1073 | 443 | c decl names surface in sds.h |  |  | 0.573 |
| walker |  | 1073 | 0 | c decl at sds.h:87 |  |  | 0.573 |
| walker |  | 1073 | 0 | c decl at sds.h:104 |  |  | 0.573 |
| walker |  | 1073 | 0 | c decl at sds.h:130 |  |  | 0.573 |
| walker |  | 1073 | 0 | c decl at sds.h:154 |  |  | 0.573 |
| walker |  | 1073 | 0 | c decl at sds.h:180 |  |  | 0.573 |
| walker |  | 1073 | 0 | c decl at sds.h:197 |  |  | 0.573 |
| walker |  | 1110 | 37 | c decl at sds.h:47 |  |  | 0.574 |
| walker |  | 1175 | 65 | c decl at sds.h:51 |  |  | 0.576 |
| walker |  | 1240 | 65 | c decl at sds.h:57 |  |  | 0.579 |
| ns | 1292 |  | 297 | Public fn declarations — utility fns (trim/range/cmp/split/case/repr/join) | 2.7 |  | 0.512 |
| walker |  | 1305 | 65 | c decl at sds.h:63 |  |  | 0.516 |
| walker |  | 1370 | 65 | c decl at sds.h:69 |  |  | 0.520 |
| walker |  | 1400 | 30 | c includes in sds.h |  |  | 0.520 |
| walker |  | 1420 | 20 | c decl doc at sds.h:180 |  |  | 0.520 |
| walker |  | 1461 | 41 | c decl doc at sds.h:47 |  |  | 0.523 |
| ns | 1503 |  | 211 | Public fn declarations — low-level + allocator-export API | 2.8 |  | 0.476 |
| walker |  | 1912 | 451 | c decl names surface #1 in sds.h |  |  | 0.645 |
| walker |  | 1930 | 18 | c decl at sds.h:232 |  |  | 0.657 |
| ns | 1931 |  | 428 | Five packed sdshdr structs | 2.9 |  | 0.713 |
| walker |  | 2210 | 280 | c decl names surface #2 in sds.h |  |  | 0.794 |
| walker |  | 2210 | 0 | c decl at sds.h:256 |  |  | 0.794 |
| walker |  | 2210 | 0 | c decl at sds.h:266 |  |  | 0.794 |
| walker |  | 2222 | 12 | c decl doc at sds.h:256 |  |  | 0.801 |
| ns | 2246 |  | 315 | Canonical Hello World | 2.10 |  | 0.722 |
| ns | 2602 |  | 356 | Embedding + allocator-swap instructions | 2.11 |  | 0.654 |
| walker |  | 2715 | 493 | c header banner in sds.h |  |  | 0.654 |
| ns | 2726 |  | 124 | sdsalloc.h — full file | 2.12 | 2.11 | 0.636 |
| walker |  | 2781 | 66 | c decl doc at sds.h:266 |  |  | 0.670 |
| walker |  | 2840 | 59 | c decl at testhelp.h:44 |  |  | 0.670 |
| ns | 2845 |  | 119 | Error handling — NULL on OOM | 3.1 |  | 0.650 |
| ns | 2950 |  | 105 | README section TOC — first half (intro through copying) | 3.2 |  | 0.662 |
| walker |  | 3005 | 165 | c decl body at sds.h:87 |  |  | 0.663 |
| ns | 3067 |  | 117 | README section TOC — second half (quoting through credits) | 3.3 |  | 0.674 |
| walker |  | 3170 | 165 | c decl body at sds.h:180 |  |  | 0.674 |
| ns | 3546 |  | 479 | How SDS strings work — design narrative + ASCII diagram | 3.4 |  | 0.618 |
| walker |  | 3646 | 476 | c decl names surface in sds.c |  |  | 0.618 |
| walker |  | 3646 | 0 | c decl at sds.c:89 |  |  | 0.618 |
| walker |  | 3646 | 0 | c decl at sds.c:149 |  |  | 0.618 |
| walker |  | 3646 | 0 | c decl at sds.c:154 |  |  | 0.618 |
| walker |  | 3646 | 0 | c decl at sds.c:160 |  |  | 0.618 |
| walker |  | 3646 | 0 | c decl at sds.c:165 |  |  | 0.618 |
| walker |  | 3646 | 0 | c decl at sds.c:184 |  |  | 0.618 |
| walker |  | 3646 | 0 | c decl at sds.c:193 |  |  | 0.618 |
| walker |  | 3646 | 0 | c decl at sds.c:204 |  |  | 0.618 |
| walker |  | 3646 | 0 | c decl at sds.c:256 |  |  | 0.618 |
| walker |  | 3646 | 0 | c decl at sds.c:300 |  |  | 0.618 |
| walker |  | 3646 | 0 | c decl at sds.c:307 |  |  | 0.618 |
| walker |  | 3646 | 0 | c decl at sds.c:334 |  |  | 0.618 |
| walker |  | 3646 | 0 | c decl at sds.c:380 |  |  | 0.618 |
| walker |  | 3646 | 0 | c decl at sds.c:398 |  |  | 0.618 |
| walker |  | 3646 | 0 | c decl at sds.c:413 |  |  | 0.618 |
| walker |  | 3646 | 0 | c decl at sds.c:421 |  |  | 0.618 |
| walker |  | 3646 | 0 | c decl at sds.c:427 |  |  | 0.618 |
| walker |  | 3646 | 0 | c decl at sds.c:440 |  |  | 0.618 |
| walker |  | 3646 | 0 | c decl at sds.c:450 |  |  | 0.618 |
| walker |  | 3646 | 0 | c decl at sds.c:451 |  |  | 0.618 |
| walker |  | 3646 | 0 | c decl at sds.c:494 |  |  | 0.618 |
| walker |  | 3646 | 0 | c decl at sds.c:526 |  |  | 0.618 |
| walker |  | 3646 | 0 | c decl at sds.c:534 |  |  | 0.618 |
| walker |  | 3659 | 13 | c decl body at sds.c:149 |  |  | 0.618 |
| walker |  | 3671 | 12 | c decl doc at sds.c:160 |  |  | 0.618 |
| walker |  | 3688 | 17 | c decl body at sds.c:160 |  |  | 0.618 |
| walker |  | 3705 | 17 | c decl body at sds.c:413 |  |  | 0.618 |
| walker |  | 3722 | 17 | c decl body at sds.c:440 |  |  | 0.618 |
| walker |  | 3741 | 19 | c decl body at sds.c:307 |  |  | 0.618 |
| walker |  | 3760 | 19 | c decl body at sds.c:421 |  |  | 0.619 |
| walker |  | 3780 | 20 | c decl doc at sds.c:154 |  |  | 0.619 |
| walker |  | 3807 | 27 | c decl body at sds.c:184 |  |  | 0.619 |
| walker |  | 3834 | 27 | c decl body at sds.c:193 |  |  | 0.619 |
| walker |  | 3855 | 21 | c decl doc at sds.c:494 |  |  | 0.619 |
| walker |  | 3877 | 22 | c decl doc at sds.c:534 |  |  | 0.619 |
| walker |  | 3900 | 23 | c decl doc at sds.c:165 |  |  | 0.619 |
| walker |  | 3932 | 32 | c decl body at sds.c:165 |  |  | 0.619 |
| ns | 3950 |  | 404 | Preallocation + SDS_MAX_PREALLOC growth strategy | 3.5 |  | 0.585 |
| walker |  | 3964 | 32 | c decl body at sds.c:300 |  |  | 0.585 |
| walker |  | 4001 | 37 | c decl body at sds.c:154 |  |  | 0.585 |
| walker |  | 4084 | 83 | c includes in sds.c |  |  | 0.585 |
| walker |  | 4118 | 34 | c decl doc at sds.c:307 |  |  | 0.585 |
| walker |  | 4165 | 47 | c decl body at sds.c:526 |  |  | 0.585 |
| walker |  | 4201 | 36 | c decl doc at sds.c:149 |  |  | 0.586 |
| ns | 4218 |  | 268 | Zero-copy append idiom | 3.6 |  | 0.563 |
| walker |  | 4242 | 41 | c decl doc at sds.c:440 |  |  | 0.563 |
| walker |  | 4284 | 42 | c decl doc at sds.c:427 |  |  | 0.563 |
| walker |  | 4335 | 51 | c decl doc at sds.c:526 |  |  | 0.563 |
| walker |  | 4434 | 99 | c decl at testhelp.h:48 |  |  | 0.564 |
| ns | 4572 |  | 354 | sdsReqType + sdsHdrSize — internal dispatch helpers | 3.7 |  | 0.532 |
| walker |  | 4639 | 205 | c decl body at sds.h:197 |  |  | 0.532 |
| walker |  | 4876 | 237 | c decl body at sds.h:130 |  |  | 0.532 |
| ns | 5030 |  | 458 | sds.h inline accessors — sdslen + sdsavail | 3.8 |  | 0.512 |
| walker |  | 5120 | 244 | c decl body at sds.h:104 |  |  | 0.575 |
| walker |  | 5372 | 252 | c decl body at sds.h:154 |  |  | 0.575 |
| walker |  | 5439 | 67 | c decl doc at sds.c:421 |  |  | 0.575 |
| walker |  | 5507 | 68 | c decl doc at sds.c:413 |  |  | 0.575 |
| ns | 5512 |  | 482 | Outdated v1 internals diagram in README | 3.9 |  | 0.546 |
| walker |  | 5576 | 69 | c decl doc at sds.c:193 |  |  | 0.546 |
| walker |  | 5649 | 73 | c decl doc at sds.c:380 |  |  | 0.546 |
| walker |  | 5749 | 100 | c decl body at sds.c:427 |  |  | 0.547 |
| walker |  | 5853 | 104 | c decl body at sds.c:398 |  |  | 0.548 |
| ns | 6247 |  | 735 | sdsMakeRoomFor body — growth/realloc engine | 4.1 |  | 0.512 |
| walker |  | 6293 | 440 | c decl names surface #1 in sds.c |  |  | 0.512 |
| walker |  | 6293 | 0 | c decl at sds.c:591 |  |  | 0.512 |
| walker |  | 6293 | 0 | c decl at sds.c:616 |  |  | 0.512 |
| walker |  | 6293 | 0 | c decl at sds.c:725 |  |  | 0.512 |
| walker |  | 6293 | 0 | c decl at sds.c:756 |  |  | 0.512 |
| walker |  | 6293 | 0 | c decl at sds.c:783 |  |  | 0.512 |
| walker |  | 6293 | 0 | c decl at sds.c:790 |  |  | 0.512 |
| walker |  | 6293 | 0 | c decl at sds.c:807 |  |  | 0.512 |
| walker |  | 6293 | 0 | c decl at sds.c:835 |  |  | 0.512 |
| walker |  | 6293 | 0 | c decl at sds.c:885 |  |  | 0.512 |
| walker |  | 6293 | 0 | c decl at sds.c:898 |  |  | 0.512 |
| walker |  | 6293 | 0 | c decl at sds.c:925 |  |  | 0.512 |
| walker |  | 6293 | 0 | c decl at sds.c:932 |  |  | 0.512 |
| walker |  | 6293 | 0 | c decl at sds.c:973 |  |  | 0.512 |
| walker |  | 6293 | 0 | c decl at sds.c:1092 |  |  | 0.512 |
| walker |  | 6293 | 0 | c decl at sds.c:1108 |  |  | 0.512 |
| walker |  | 6293 | 0 | c decl at sds.c:1120 |  |  | 0.512 |
| walker |  | 6293 | 0 | c decl at sds.c:1136 |  |  | 0.512 |
| walker |  | 6313 | 20 | c decl doc at sds.c:1120 |  |  | 0.512 |
| walker |  | 6334 | 21 | c decl doc at sds.c:783 |  |  | 0.512 |
| walker |  | 6355 | 21 | c decl doc at sds.c:790 |  |  | 0.512 |
| walker |  | 6381 | 26 | c decl doc at sds.c:885 |  |  | 0.512 |
| walker |  | 6420 | 39 | c decl body at sds.c:885 |  |  | 0.512 |
| walker |  | 6453 | 33 | c decl doc at sds.c:925 |  |  | 0.512 |
| walker |  | 6487 | 34 | c decl doc at sds.c:932 |  |  | 0.512 |
| walker |  | 6535 | 48 | c decl body at sds.c:783 |  |  | 0.512 |
| walker |  | 6583 | 48 | c decl body at sds.c:790 |  |  | 0.512 |
| walker |  | 6632 | 49 | c decl body at sds.c:925 |  |  | 0.512 |
| walker |  | 6670 | 38 | c decl doc at sds.c:1108 |  |  | 0.512 |
| walker |  | 6734 | 64 | c decl body at sds.c:591 |  |  | 0.512 |
| walker |  | 6838 | 104 | c decl body at sds.c:1108 |  |  | 0.512 |
| ns | 6912 |  | 665 | sdsnewlen body — canonical allocator (no doc-comment) | 4.2 |  | 0.479 |
| walker |  | 6946 | 108 | c decl body at sds.c:1120 |  |  | 0.479 |
| walker |  | 7031 | 85 | c decl doc at sds.c:398 |  |  | 0.480 |
| walker |  | 7119 | 88 | c decl doc at sds.c:300 |  |  | 0.480 |
| ns | 7129 |  | 217 | sdscatfmt — fast subset of printf, format spec list | 4.3 |  | 0.471 |
| walker |  | 7205 | 86 | c decl doc at sds.c:450 |  |  | 0.471 |
| walker |  | 7303 | 98 | c decl doc at sds.c:256 |  |  | 0.471 |
| ns | 7409 |  | 280 | Constructor family + sdsfree (short bodies) | 4.4 |  | 0.481 |
| walker |  | 7436 | 133 | c decl body at sds.c:1092 |  |  | 0.481 |
| walker |  | 7571 | 135 | c decl body at sds.c:380 |  |  | 0.481 |
| walker |  | 7709 | 138 | c decl body at sds.c:807 |  |  | 0.481 |
| walker |  | 7812 | 103 | c decl doc at sds.c:1136 |  |  | 0.481 |
| walker |  | 7922 | 110 | c decl doc at sds.c:898 |  |  | 0.481 |
| walker |  | 8033 | 111 | c decl doc at sds.c:204 |  |  | 0.483 |
| ns | 8143 |  | 734 | README — Concatenating strings + sdsgrowzero | 4.5 |  | 0.454 |
| walker |  | 8190 | 157 | c decl body at sds.c:725 |  |  | 0.454 |
| walker |  | 8330 | 140 | c decl doc at sds.c:807 |  |  | 0.454 |
| ns | 8856 |  | 713 | Append/copy primitive bodies (sdscatlen/cat/catsds/cpylen/cpy) | 5.1 |  | 0.481 |
| walker |  | 8864 | 534 | c header banner in testhelp.h |  |  | 0.481 |
| walker |  | 9013 | 149 | c decl doc at sds.c:1092 |  |  | 0.481 |
| ns | 9478 |  | 622 | sdsIncrLen body — zero-copy idiom's other half (no doc-comment) | 5.2 | 3.6 | 0.465 |
| walker |  | 9595 | 582 | c header banner in sdsalloc.h |  |  | 0.476 |
| ns | 9731 |  | 253 | testhelp.h — minimal test framework macros | 5.3 |  | 0.482 |
| ns | 9977 |  | 246 | Makefile + Changelog | 5.4 |  | 0.475 |
