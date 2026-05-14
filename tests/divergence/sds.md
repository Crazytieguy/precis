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
| walker |  | 698 | 438 | c decl names surface in sds.h |  |  | 0.703 |
| walker |  | 698 | 0 | c decl at sds.h:87 |  |  | 0.703 |
| walker |  | 698 | 0 | c decl at sds.h:104 |  |  | 0.703 |
| walker |  | 698 | 0 | c decl at sds.h:130 |  |  | 0.703 |
| walker |  | 698 | 0 | c decl at sds.h:154 |  |  | 0.703 |
| walker |  | 698 | 0 | c decl at sds.h:180 |  |  | 0.703 |
| walker |  | 698 | 0 | c decl at sds.h:197 |  |  | 0.703 |
| walker |  | 735 | 37 | c decl at sds.h:47 |  |  | 0.704 |
| walker |  | 755 | 20 | c decl doc at sds.h:180 |  |  | 0.704 |
| walker |  | 820 | 65 | c decl at sds.h:51 |  |  | 0.707 |
| ns | 847 |  | 190 | Public fn declarations — high-level API (creation/length/free/concat/copy) | 2.5 |  | 0.621 |
| walker |  | 885 | 65 | c decl at sds.h:57 |  |  | 0.624 |
| walker |  | 950 | 65 | c decl at sds.h:63 |  |  | 0.629 |
| ns | 979 |  | 132 | Public fn declarations — printf family (sdscatvprintf/printf/fmt) | 2.6 |  | 0.576 |
| walker |  | 1015 | 65 | c decl at sds.h:69 |  |  | 0.581 |
| walker |  | 1058 | 43 | c decl doc at sds.h:47 |  |  | 0.584 |
| ns | 1276 |  | 297 | Public fn declarations — utility fns (trim/range/cmp/split/case/repr/join) | 2.7 |  | 0.517 |
| walker |  | 1457 | 399 | headings outline in README.md |  |  | 0.523 |
| ns | 1487 |  | 211 | Public fn declarations — low-level + allocator-export API | 2.8 |  | 0.476 |
| walker |  | 1684 | 227 | c decl names surface #2 in sds.h |  |  | 0.528 |
| walker |  | 1684 | 0 | c decl at sds.h:256 |  |  | 0.528 |
| walker |  | 1684 | 0 | c decl at sds.h:266 |  |  | 0.528 |
| walker |  | 1698 | 14 | c decl doc at sds.h:256 |  |  | 0.540 |
| walker |  | 1766 | 68 | c decl doc at sds.h:266 |  |  | 0.609 |
| ns | 1917 |  | 430 | Five packed sdshdr structs | 2.9 |  | 0.676 |
| walker |  | 2205 | 439 | c decl names surface #1 in sds.h |  |  | 0.822 |
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
| ns | 3492 |  | 481 | How SDS strings work — design narrative + ASCII diagram | 3.4 |  | 0.605 |
| walker |  | 3737 | 474 | c decl names surface in sds.c |  |  | 0.605 |
| walker |  | 3737 | 0 | c decl at sds.c:89 |  |  | 0.605 |
| walker |  | 3737 | 0 | c decl at sds.c:149 |  |  | 0.605 |
| walker |  | 3737 | 0 | c decl at sds.c:154 |  |  | 0.605 |
| walker |  | 3737 | 0 | c decl at sds.c:160 |  |  | 0.605 |
| walker |  | 3737 | 0 | c decl at sds.c:165 |  |  | 0.605 |
| walker |  | 3737 | 0 | c decl at sds.c:184 |  |  | 0.605 |
| walker |  | 3737 | 0 | c decl at sds.c:193 |  |  | 0.605 |
| walker |  | 3737 | 0 | c decl at sds.c:204 |  |  | 0.605 |
| walker |  | 3737 | 0 | c decl at sds.c:256 |  |  | 0.605 |
| walker |  | 3737 | 0 | c decl at sds.c:300 |  |  | 0.605 |
| walker |  | 3737 | 0 | c decl at sds.c:307 |  |  | 0.605 |
| walker |  | 3737 | 0 | c decl at sds.c:334 |  |  | 0.605 |
| walker |  | 3737 | 0 | c decl at sds.c:380 |  |  | 0.605 |
| walker |  | 3737 | 0 | c decl at sds.c:398 |  |  | 0.605 |
| walker |  | 3737 | 0 | c decl at sds.c:413 |  |  | 0.605 |
| walker |  | 3737 | 0 | c decl at sds.c:421 |  |  | 0.605 |
| walker |  | 3737 | 0 | c decl at sds.c:427 |  |  | 0.605 |
| walker |  | 3737 | 0 | c decl at sds.c:440 |  |  | 0.605 |
| walker |  | 3737 | 0 | c decl at sds.c:450 |  |  | 0.605 |
| walker |  | 3737 | 0 | c decl at sds.c:451 |  |  | 0.605 |
| walker |  | 3737 | 0 | c decl at sds.c:494 |  |  | 0.605 |
| walker |  | 3737 | 0 | c decl at sds.c:526 |  |  | 0.605 |
| walker |  | 3737 | 0 | c decl at sds.c:534 |  |  | 0.605 |
| walker |  | 3749 | 12 | c decl doc at sds.c:160 |  |  | 0.605 |
| walker |  | 3760 | 11 | c decl body at sds.c:149 |  |  | 0.605 |
| walker |  | 3780 | 20 | c decl doc at sds.c:154 |  |  | 0.605 |
| walker |  | 3801 | 21 | c decl doc at sds.c:494 |  |  | 0.605 |
| walker |  | 3823 | 22 | c decl doc at sds.c:534 |  |  | 0.605 |
| walker |  | 3838 | 15 | c decl body at sds.c:160 |  |  | 0.605 |
| walker |  | 3853 | 15 | c decl body at sds.c:413 |  |  | 0.605 |
| walker |  | 3868 | 15 | c decl body at sds.c:440 |  |  | 0.605 |
| walker |  | 3891 | 23 | c decl doc at sds.c:165 |  |  | 0.606 |
| ns | 3896 |  | 404 | Preallocation + SDS_MAX_PREALLOC growth strategy | 3.5 |  | 0.572 |
| walker |  | 3908 | 17 | c decl body at sds.c:307 |  |  | 0.572 |
| walker |  | 3925 | 17 | c decl body at sds.c:421 |  |  | 0.572 |
| walker |  | 3959 | 34 | c decl doc at sds.c:307 |  |  | 0.572 |
| walker |  | 3995 | 36 | c decl doc at sds.c:149 |  |  | 0.573 |
| walker |  | 4036 | 41 | c decl doc at sds.c:440 |  |  | 0.573 |
| walker |  | 4078 | 42 | c decl doc at sds.c:427 |  |  | 0.573 |
| walker |  | 4103 | 25 | c decl body at sds.c:184 |  |  | 0.573 |
| walker |  | 4128 | 25 | c decl body at sds.c:193 |  |  | 0.573 |
| ns | 4166 |  | 270 | Zero-copy append idiom | 3.6 |  | 0.550 |
| walker |  | 4179 | 51 | c decl doc at sds.c:526 |  |  | 0.550 |
| walker |  | 4209 | 30 | c decl body at sds.c:165 |  |  | 0.551 |
| walker |  | 4239 | 30 | c decl body at sds.c:300 |  |  | 0.551 |
| walker |  | 4274 | 35 | c decl body at sds.c:154 |  |  | 0.551 |
| walker |  | 4341 | 67 | c decl doc at sds.c:421 |  |  | 0.551 |
| walker |  | 4409 | 68 | c decl doc at sds.c:413 |  |  | 0.552 |
| walker |  | 4478 | 69 | c decl doc at sds.c:193 |  |  | 0.552 |
| ns | 4516 |  | 350 | sdsReqType + sdsHdrSize — internal dispatch helpers | 3.7 |  | 0.521 |
| walker |  | 4551 | 73 | c decl doc at sds.c:380 |  |  | 0.521 |
| walker |  | 4591 | 40 | c decl body at sds.c:526 |  |  | 0.521 |
| walker |  | 4676 | 85 | c decl doc at sds.c:398 |  |  | 0.522 |
| walker |  | 4764 | 88 | c decl doc at sds.c:300 |  |  | 0.522 |
| walker |  | 4862 | 98 | c decl doc at sds.c:256 |  |  | 0.522 |
| walker |  | 4948 | 86 | c decl doc at sds.c:450 |  |  | 0.522 |
| ns | 4974 |  | 458 | sds.h inline accessors — sdslen + sdsavail | 3.8 |  | 0.502 |
| walker |  | 5059 | 111 | c decl doc at sds.c:204 |  |  | 0.502 |
| walker |  | 5262 | 203 | c decl body at sds.h:197 |  |  | 0.502 |
| ns | 5456 |  | 482 | Outdated v1 internals diagram in README | 3.9 |  | 0.477 |
| walker |  | 5460 | 198 | c decl doc at sds.c:184 |  |  | 0.477 |
| walker |  | 5695 | 235 | c decl body at sds.h:130 |  |  | 0.477 |
| walker |  | 5906 | 211 | c decl doc at sds.c:89 |  |  | 0.477 |
| walker |  | 6148 | 242 | c decl body at sds.h:104 |  |  | 0.537 |
| ns | 6189 |  | 733 | sdsMakeRoomFor body — growth/realloc engine | 4.1 |  | 0.504 |
| walker |  | 6398 | 250 | c decl body at sds.h:154 |  |  | 0.504 |
| walker |  | 6495 | 97 | c decl body at sds.c:398 |  |  | 0.505 |
| walker |  | 6593 | 98 | c decl body at sds.c:427 |  |  | 0.506 |
| ns | 6852 |  | 663 | sdsnewlen body — canonical allocator (no doc-comment) | 4.2 |  | 0.473 |
| walker |  | 7033 | 440 | c decl names surface #1 in sds.c |  |  | 0.473 |
| walker |  | 7033 | 0 | c decl at sds.c:591 |  |  | 0.473 |
| walker |  | 7033 | 0 | c decl at sds.c:616 |  |  | 0.473 |
| walker |  | 7033 | 0 | c decl at sds.c:725 |  |  | 0.473 |
| walker |  | 7033 | 0 | c decl at sds.c:756 |  |  | 0.473 |
| walker |  | 7033 | 0 | c decl at sds.c:783 |  |  | 0.473 |
| walker |  | 7033 | 0 | c decl at sds.c:790 |  |  | 0.473 |
| walker |  | 7033 | 0 | c decl at sds.c:807 |  |  | 0.473 |
| walker |  | 7033 | 0 | c decl at sds.c:835 |  |  | 0.473 |
| walker |  | 7033 | 0 | c decl at sds.c:885 |  |  | 0.473 |
| walker |  | 7033 | 0 | c decl at sds.c:898 |  |  | 0.473 |
| walker |  | 7033 | 0 | c decl at sds.c:925 |  |  | 0.473 |
| walker |  | 7033 | 0 | c decl at sds.c:932 |  |  | 0.473 |
| walker |  | 7033 | 0 | c decl at sds.c:973 |  |  | 0.473 |
| walker |  | 7033 | 0 | c decl at sds.c:1092 |  |  | 0.473 |
| walker |  | 7033 | 0 | c decl at sds.c:1108 |  |  | 0.473 |
| walker |  | 7033 | 0 | c decl at sds.c:1120 |  |  | 0.473 |
| walker |  | 7033 | 0 | c decl at sds.c:1136 |  |  | 0.473 |
| walker |  | 7053 | 20 | c decl doc at sds.c:1120 |  |  | 0.473 |
| ns | 7067 |  | 215 | sdscatfmt — fast subset of printf, format spec list | 4.3 |  | 0.465 |
| walker |  | 7074 | 21 | c decl doc at sds.c:783 |  |  | 0.465 |
| walker |  | 7095 | 21 | c decl doc at sds.c:790 |  |  | 0.465 |
| walker |  | 7121 | 26 | c decl doc at sds.c:885 |  |  | 0.465 |
| walker |  | 7154 | 33 | c decl doc at sds.c:925 |  |  | 0.465 |
| walker |  | 7188 | 34 | c decl doc at sds.c:932 |  |  | 0.465 |
| walker |  | 7226 | 38 | c decl doc at sds.c:1108 |  |  | 0.465 |
| walker |  | 7263 | 37 | c decl body at sds.c:885 |  |  | 0.465 |
| walker |  | 7304 | 41 | c decl body at sds.c:783 |  |  | 0.465 |
| walker |  | 7345 | 41 | c decl body at sds.c:790 |  |  | 0.465 |
| ns | 7347 |  | 280 | Constructor family + sdsfree (short bodies) | 4.4 |  | 0.475 |
| walker |  | 7392 | 47 | c decl body at sds.c:925 |  |  | 0.475 |
| walker |  | 7495 | 103 | c decl doc at sds.c:1136 |  |  | 0.475 |
| walker |  | 7605 | 110 | c decl doc at sds.c:898 |  |  | 0.475 |
| walker |  | 7667 | 62 | c decl body at sds.c:591 |  |  | 0.475 |
| walker |  | 7807 | 140 | c decl doc at sds.c:807 |  |  | 0.475 |
| walker |  | 7956 | 149 | c decl doc at sds.c:1092 |  |  | 0.475 |
| ns | 8081 |  | 734 | README — Concatenating strings + sdsgrowzero | 4.5 |  | 0.446 |
| walker |  | 8133 | 177 | c decl doc at sds.c:725 |  |  | 0.446 |
| walker |  | 8323 | 190 | c decl doc at sds.c:756 |  |  | 0.446 |
| walker |  | 8532 | 209 | c decl doc at sds.c:835 |  |  | 0.446 |
| walker |  | 8747 | 215 | c decl doc at sds.c:591 |  |  | 0.446 |
| ns | 8792 |  | 711 | Append/copy primitive bodies (sdscatlen/cat/catsds/cpylen/cpy) | 5.1 |  | 0.472 |
| walker |  | 8962 | 215 | c decl doc at sds.c:616 |  |  | 0.491 |
| walker |  | 9058 | 96 | c decl body at sds.c:1108 |  |  | 0.491 |
| walker |  | 9158 | 100 | c decl body at sds.c:1120 |  |  | 0.491 |
| ns | 9412 |  | 620 | sdsIncrLen body — zero-copy idiom's other half (no doc-comment) | 5.2 | 3.6 | 0.475 |
| walker |  | 9413 | 255 | c decl doc at sds.c:973 |  |  | 0.475 |
| ns | 9663 |  | 251 | testhelp.h — minimal test framework macros | 5.3 |  | 0.481 |
| walker |  | 9735 | 322 | c decl doc at sds.c:334 |  |  | 0.481 |
| walker |  | 9858 | 123 | c decl body at sds.c:380 |  |  | 0.481 |
| ns | 9909 |  | 246 | Makefile + Changelog | 5.4 |  | 0.473 |
| walker |  | 9983 | 125 | c decl body at sds.c:1092 |  |  | 0.473 |
