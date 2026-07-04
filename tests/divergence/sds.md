Score(3000)=0.663 I=0.806 C=0.546 ns_rows≤3K=18/34 (reached=11 partial=1 missing=6)

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 31 | 31 | listing of '.' |  |  | 1.000 |
| ns | 31 |  | 31 | Repo file listing | 1.1 |  | 1.000 |
| ns | 41 |  | 10 | README title | 1.2 |  | 0.932 |
| ns | 132 |  | 91 | README v2 lede — first paragraph (binary-compat warning) | 1.3 |  | 0.748 |
| walker |  | 135 | 104 | README headline in README.md |  |  | 1.000 |
| walker |  | 168 | 33 | c decl names surface in sdsalloc.h |  |  | 1.000 |
| ns | 256 |  | 124 | README v2 lede — perf note + sdscatfmt headline | 1.4 |  | 0.749 |
| ns | 270 |  | 14 | sds typedef — sds IS char* | 2.1 |  | 0.728 |
| ns | 303 |  | 33 | SDS_MAX_PREALLOC + SDS_NOINIT constants | 2.2 |  | 0.691 |
| ns | 481 |  | 178 | SDS_TYPE_* tag constants + SDS_HDR macros | 2.3 |  | 0.560 |
| walker |  | 567 | 399 | headings outline in README.md |  |  | 0.569 |
| walker |  | 625 | 58 | c decl names surface in testhelp.h |  |  | 0.569 |
| ns | 671 |  | 190 | Cardinal usage rule — must reassign return value | 2.4 |  | 0.481 |
| ns | 863 |  | 192 | Public fn declarations — high-level API (creation/length/free/concat/copy) | 2.5 |  | 0.422 |
| ns | 995 |  | 132 | Public fn declarations — printf family (sdscatvprintf/printf/fmt) | 2.6 |  | 0.386 |
| walker |  | 1063 | 438 | c decl names surface in sds.h |  |  | 0.573 |
| walker |  | 1063 | 0 | c decl at sds.h:87 |  |  | 0.573 |
| walker |  | 1063 | 0 | c decl at sds.h:104 |  |  | 0.573 |
| walker |  | 1063 | 0 | c decl at sds.h:130 |  |  | 0.573 |
| walker |  | 1063 | 0 | c decl at sds.h:154 |  |  | 0.573 |
| walker |  | 1063 | 0 | c decl at sds.h:180 |  |  | 0.573 |
| walker |  | 1063 | 0 | c decl at sds.h:197 |  |  | 0.573 |
| walker |  | 1100 | 37 | c decl at sds.h:47 |  |  | 0.574 |
| walker |  | 1120 | 20 | c decl doc at sds.h:180 |  |  | 0.574 |
| walker |  | 1185 | 65 | c decl at sds.h:51 |  |  | 0.576 |
| walker |  | 1250 | 65 | c decl at sds.h:57 |  |  | 0.579 |
| ns | 1292 |  | 297 | Public fn declarations — utility fns (trim/range/cmp/split/case/repr/join) | 2.7 |  | 0.512 |
| walker |  | 1315 | 65 | c decl at sds.h:63 |  |  | 0.516 |
| walker |  | 1380 | 65 | c decl at sds.h:69 |  |  | 0.520 |
| walker |  | 1421 | 41 | c decl doc at sds.h:47 |  |  | 0.523 |
| walker |  | 1451 | 30 | c includes in sds.h |  |  | 0.523 |
| ns | 1503 |  | 211 | Public fn declarations — low-level + allocator-export API | 2.8 |  | 0.476 |
| walker |  | 1897 | 446 | c decl names surface #1 in sds.h |  |  | 0.645 |
| walker |  | 1915 | 18 | c decl at sds.h:232 |  |  | 0.657 |
| ns | 1931 |  | 428 | Five packed sdshdr structs | 2.9 |  | 0.713 |
| walker |  | 2195 | 280 | c decl names surface #2 in sds.h |  |  | 0.794 |
| walker |  | 2195 | 0 | c decl at sds.h:256 |  |  | 0.794 |
| walker |  | 2195 | 0 | c decl at sds.h:266 |  |  | 0.794 |
| walker |  | 2207 | 12 | c decl doc at sds.h:256 |  |  | 0.801 |
| ns | 2246 |  | 315 | Canonical Hello World | 2.10 |  | 0.722 |
| walker |  | 2273 | 66 | c decl doc at sds.h:266 |  |  | 0.762 |
| ns | 2602 |  | 356 | Embedding + allocator-swap instructions | 2.11 |  | 0.690 |
| ns | 2726 |  | 124 | sdsalloc.h — full file | 2.12 | 2.11 | 0.670 |
| walker |  | 2766 | 493 | c header banner in sds.h |  |  | 0.670 |
| walker |  | 2825 | 59 | c decl at testhelp.h:44 |  |  | 0.670 |
| ns | 2845 |  | 119 | Error handling — NULL on OOM | 3.1 |  | 0.650 |
| ns | 2950 |  | 105 | README section TOC — first half (intro through copying) | 3.2 |  | 0.662 |
| walker |  | 2990 | 165 | c decl body at sds.h:87 |  |  | 0.663 |
| ns | 3067 |  | 117 | README section TOC — second half (quoting through credits) | 3.3 |  | 0.674 |
| walker |  | 3155 | 165 | c decl body at sds.h:180 |  |  | 0.674 |
| ns | 3546 |  | 479 | How SDS strings work — design narrative + ASCII diagram | 3.4 |  | 0.618 |
| walker |  | 3631 | 476 | c decl names surface in sds.c |  |  | 0.618 |
| walker |  | 3631 | 0 | c decl at sds.c:89 |  |  | 0.618 |
| walker |  | 3631 | 0 | c decl at sds.c:149 |  |  | 0.618 |
| walker |  | 3631 | 0 | c decl at sds.c:154 |  |  | 0.618 |
| walker |  | 3631 | 0 | c decl at sds.c:160 |  |  | 0.618 |
| walker |  | 3631 | 0 | c decl at sds.c:165 |  |  | 0.618 |
| walker |  | 3631 | 0 | c decl at sds.c:184 |  |  | 0.618 |
| walker |  | 3631 | 0 | c decl at sds.c:193 |  |  | 0.618 |
| walker |  | 3631 | 0 | c decl at sds.c:204 |  |  | 0.618 |
| walker |  | 3631 | 0 | c decl at sds.c:256 |  |  | 0.618 |
| walker |  | 3631 | 0 | c decl at sds.c:300 |  |  | 0.618 |
| walker |  | 3631 | 0 | c decl at sds.c:307 |  |  | 0.618 |
| walker |  | 3631 | 0 | c decl at sds.c:334 |  |  | 0.618 |
| walker |  | 3631 | 0 | c decl at sds.c:380 |  |  | 0.618 |
| walker |  | 3631 | 0 | c decl at sds.c:398 |  |  | 0.618 |
| walker |  | 3631 | 0 | c decl at sds.c:413 |  |  | 0.618 |
| walker |  | 3631 | 0 | c decl at sds.c:421 |  |  | 0.618 |
| walker |  | 3631 | 0 | c decl at sds.c:427 |  |  | 0.618 |
| walker |  | 3631 | 0 | c decl at sds.c:440 |  |  | 0.618 |
| walker |  | 3631 | 0 | c decl at sds.c:450 |  |  | 0.618 |
| walker |  | 3631 | 0 | c decl at sds.c:451 |  |  | 0.618 |
| walker |  | 3631 | 0 | c decl at sds.c:494 |  |  | 0.618 |
| walker |  | 3631 | 0 | c decl at sds.c:526 |  |  | 0.618 |
| walker |  | 3631 | 0 | c decl at sds.c:534 |  |  | 0.618 |
| walker |  | 3643 | 12 | c decl doc at sds.c:160 |  |  | 0.618 |
| walker |  | 3656 | 13 | c decl body at sds.c:149 |  |  | 0.618 |
| walker |  | 3676 | 20 | c decl doc at sds.c:154 |  |  | 0.618 |
| walker |  | 3697 | 21 | c decl doc at sds.c:494 |  |  | 0.618 |
| walker |  | 3719 | 22 | c decl doc at sds.c:534 |  |  | 0.618 |
| walker |  | 3742 | 23 | c decl doc at sds.c:165 |  |  | 0.619 |
| walker |  | 3759 | 17 | c decl body at sds.c:160 |  |  | 0.619 |
| walker |  | 3776 | 17 | c decl body at sds.c:413 |  |  | 0.619 |
| walker |  | 3793 | 17 | c decl body at sds.c:440 |  |  | 0.619 |
| walker |  | 3812 | 19 | c decl body at sds.c:307 |  |  | 0.619 |
| walker |  | 3831 | 19 | c decl body at sds.c:421 |  |  | 0.619 |
| walker |  | 3865 | 34 | c decl doc at sds.c:307 |  |  | 0.619 |
| walker |  | 3901 | 36 | c decl doc at sds.c:149 |  |  | 0.619 |
| walker |  | 3942 | 41 | c decl doc at sds.c:440 |  |  | 0.619 |
| ns | 3950 |  | 404 | Preallocation + SDS_MAX_PREALLOC growth strategy | 3.5 |  | 0.585 |
| walker |  | 3984 | 42 | c decl doc at sds.c:427 |  |  | 0.585 |
| walker |  | 4011 | 27 | c decl body at sds.c:184 |  |  | 0.585 |
| walker |  | 4038 | 27 | c decl body at sds.c:193 |  |  | 0.585 |
| walker |  | 4089 | 51 | c decl doc at sds.c:526 |  |  | 0.585 |
| walker |  | 4121 | 32 | c decl body at sds.c:165 |  |  | 0.586 |
| walker |  | 4153 | 32 | c decl body at sds.c:300 |  |  | 0.586 |
| ns | 4218 |  | 268 | Zero-copy append idiom | 3.6 |  | 0.563 |
| walker |  | 4220 | 67 | c decl doc at sds.c:421 |  |  | 0.563 |
| walker |  | 4288 | 68 | c decl doc at sds.c:413 |  |  | 0.563 |
| walker |  | 4357 | 69 | c decl doc at sds.c:193 |  |  | 0.563 |
| walker |  | 4394 | 37 | c decl body at sds.c:154 |  |  | 0.564 |
| walker |  | 4467 | 73 | c decl doc at sds.c:380 |  |  | 0.564 |
| walker |  | 4509 | 42 | c decl body at sds.c:526 |  |  | 0.564 |
| ns | 4572 |  | 354 | sdsReqType + sdsHdrSize — internal dispatch helpers | 3.7 |  | 0.532 |
| walker |  | 4594 | 85 | c decl doc at sds.c:398 |  |  | 0.533 |
| walker |  | 4677 | 83 | c includes in sds.c |  |  | 0.533 |
| walker |  | 4765 | 88 | c decl doc at sds.c:300 |  |  | 0.533 |
| walker |  | 4863 | 98 | c decl doc at sds.c:256 |  |  | 0.533 |
| walker |  | 4949 | 86 | c decl doc at sds.c:450 |  |  | 0.533 |
| ns | 5030 |  | 458 | sds.h inline accessors — sdslen + sdsavail | 3.8 |  | 0.512 |
| walker |  | 5060 | 111 | c decl doc at sds.c:204 |  |  | 0.512 |
| walker |  | 5159 | 99 | c decl at testhelp.h:48 |  |  | 0.513 |
| walker |  | 5364 | 205 | c decl body at sds.h:197 |  |  | 0.513 |
| ns | 5512 |  | 482 | Outdated v1 internals diagram in README | 3.9 |  | 0.487 |
| walker |  | 5562 | 198 | c decl doc at sds.c:184 |  |  | 0.487 |
| walker |  | 5799 | 237 | c decl body at sds.h:130 |  |  | 0.487 |
| walker |  | 6010 | 211 | c decl doc at sds.c:89 |  |  | 0.487 |
| ns | 6247 |  | 735 | sdsMakeRoomFor body — growth/realloc engine | 4.1 |  | 0.458 |
| walker |  | 6254 | 244 | c decl body at sds.h:104 |  |  | 0.514 |
| walker |  | 6506 | 252 | c decl body at sds.h:154 |  |  | 0.514 |
| walker |  | 6605 | 99 | c decl body at sds.c:398 |  |  | 0.515 |
| walker |  | 6705 | 100 | c decl body at sds.c:427 |  |  | 0.515 |
| ns | 6912 |  | 665 | sdsnewlen body — canonical allocator (no doc-comment) | 4.2 |  | 0.482 |
| ns | 7129 |  | 217 | sdscatfmt — fast subset of printf, format spec list | 4.3 |  | 0.474 |
| walker |  | 7145 | 440 | c decl names surface #1 in sds.c |  |  | 0.474 |
| walker |  | 7145 | 0 | c decl at sds.c:591 |  |  | 0.474 |
| walker |  | 7145 | 0 | c decl at sds.c:616 |  |  | 0.474 |
| walker |  | 7145 | 0 | c decl at sds.c:725 |  |  | 0.474 |
| walker |  | 7145 | 0 | c decl at sds.c:756 |  |  | 0.474 |
| walker |  | 7145 | 0 | c decl at sds.c:783 |  |  | 0.474 |
| walker |  | 7145 | 0 | c decl at sds.c:790 |  |  | 0.474 |
| walker |  | 7145 | 0 | c decl at sds.c:807 |  |  | 0.474 |
| walker |  | 7145 | 0 | c decl at sds.c:835 |  |  | 0.474 |
| walker |  | 7145 | 0 | c decl at sds.c:885 |  |  | 0.474 |
| walker |  | 7145 | 0 | c decl at sds.c:898 |  |  | 0.474 |
| walker |  | 7145 | 0 | c decl at sds.c:925 |  |  | 0.474 |
| walker |  | 7145 | 0 | c decl at sds.c:932 |  |  | 0.474 |
| walker |  | 7145 | 0 | c decl at sds.c:973 |  |  | 0.474 |
| walker |  | 7145 | 0 | c decl at sds.c:1092 |  |  | 0.474 |
| walker |  | 7145 | 0 | c decl at sds.c:1108 |  |  | 0.474 |
| walker |  | 7145 | 0 | c decl at sds.c:1120 |  |  | 0.474 |
| walker |  | 7145 | 0 | c decl at sds.c:1136 |  |  | 0.474 |
| walker |  | 7165 | 20 | c decl doc at sds.c:1120 |  |  | 0.474 |
| walker |  | 7186 | 21 | c decl doc at sds.c:783 |  |  | 0.474 |
| walker |  | 7207 | 21 | c decl doc at sds.c:790 |  |  | 0.474 |
| walker |  | 7233 | 26 | c decl doc at sds.c:885 |  |  | 0.474 |
| walker |  | 7266 | 33 | c decl doc at sds.c:925 |  |  | 0.474 |
| walker |  | 7300 | 34 | c decl doc at sds.c:932 |  |  | 0.474 |
| walker |  | 7338 | 38 | c decl doc at sds.c:1108 |  |  | 0.474 |
| walker |  | 7377 | 39 | c decl body at sds.c:885 |  |  | 0.474 |
| ns | 7409 |  | 280 | Constructor family + sdsfree (short bodies) | 4.4 |  | 0.483 |
| walker |  | 7420 | 43 | c decl body at sds.c:783 |  |  | 0.483 |
| walker |  | 7463 | 43 | c decl body at sds.c:790 |  |  | 0.483 |
| walker |  | 7512 | 49 | c decl body at sds.c:925 |  |  | 0.483 |
| walker |  | 7615 | 103 | c decl doc at sds.c:1136 |  |  | 0.483 |
| walker |  | 7725 | 110 | c decl doc at sds.c:898 |  |  | 0.483 |
| walker |  | 7865 | 140 | c decl doc at sds.c:807 |  |  | 0.483 |
| walker |  | 7929 | 64 | c decl body at sds.c:591 |  |  | 0.483 |
| walker |  | 8078 | 149 | c decl doc at sds.c:1092 |  |  | 0.483 |
| ns | 8143 |  | 734 | README — Concatenating strings + sdsgrowzero | 4.5 |  | 0.454 |
| walker |  | 8255 | 177 | c decl doc at sds.c:725 |  |  | 0.454 |
| walker |  | 8445 | 190 | c decl doc at sds.c:756 |  |  | 0.454 |
| walker |  | 8654 | 209 | c decl doc at sds.c:835 |  |  | 0.454 |
| ns | 8856 |  | 713 | Append/copy primitive bodies (sdscatlen/cat/catsds/cpylen/cpy) | 5.1 |  | 0.480 |
| walker |  | 8869 | 215 | c decl doc at sds.c:591 |  |  | 0.480 |
| walker |  | 9084 | 215 | c decl doc at sds.c:616 |  |  | 0.498 |
| walker |  | 9182 | 98 | c decl body at sds.c:1108 |  |  | 0.498 |
| walker |  | 9437 | 255 | c decl doc at sds.c:973 |  |  | 0.498 |
| ns | 9478 |  | 622 | sdsIncrLen body — zero-copy idiom's other half (no doc-comment) | 5.2 | 3.6 | 0.482 |
| walker |  | 9539 | 102 | c decl body at sds.c:1120 |  |  | 0.482 |
| ns | 9731 |  | 253 | testhelp.h — minimal test framework macros | 5.3 |  | 0.488 |
| walker |  | 9861 | 322 | c decl doc at sds.c:334 |  |  | 0.488 |
| ns | 9977 |  | 246 | Makefile + Changelog | 5.4 |  | 0.480 |
| walker |  | 9986 | 125 | c decl body at sds.c:380 |  |  | 0.480 |
