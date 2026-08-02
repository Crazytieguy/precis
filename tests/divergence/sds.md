Score(3000)=0.763 I=0.889 C=0.655 ns_rows≤3K=21/54 (reached=13 partial=3 missing=5)

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 31 | 31 | listing of '.' |  |  | 0.000 |
| ns | 94 |  | 94 | README title + the one-paragraph definition of SDS | 1.1 |  | 0.000 |
| ns | 125 |  | 31 | Complete root directory listing | 1.2 |  | 0.333 |
| walker |  | 148 | 117 | plaintext config Makefile |  |  | 0.360 |
| ns | 251 |  | 126 | The header-before-the-pointer design + ASCII layout diagram | 1.3 |  | 0.275 |
| walker |  | 257 | 109 | README headline in README.md |  |  | 0.288 |
| walker |  | 290 | 33 | c decl names surface in sdsalloc.h |  |  | 0.289 |
| walker |  | 348 | 58 | c decl names surface in testhelp.h |  |  | 0.289 |
| ns | 365 |  | 114 | sds.h preamble: include guard, SDS_MAX_PREALLOC, SDS_NOINIT, `typedef char *sds` | 1.4 |  | 0.232 |
| ns | 533 |  | 168 | README headings, part 1: manual chapters through Error handling | 1.5 |  | 0.184 |
| ns | 639 |  | 106 | README headings, part 2: internals chapter + back matter (696-911) | 1.6 | 1.5 | 0.168 |
| ns | 756 |  | 117 | The complete Makefile (build + run the in-file unit tests) | 1.7 |  | 0.214 |
| walker |  | 791 | 443 | c decl names surface in sds.h |  |  | 0.246 |
| walker |  | 791 | 0 | c decl at sds.h:87 |  |  | 0.246 |
| walker |  | 791 | 0 | c decl at sds.h:104 |  |  | 0.246 |
| walker |  | 791 | 0 | c decl at sds.h:130 |  |  | 0.246 |
| walker |  | 791 | 0 | c decl at sds.h:154 |  |  | 0.246 |
| walker |  | 791 | 0 | c decl at sds.h:180 |  |  | 0.246 |
| walker |  | 791 | 0 | c decl at sds.h:197 |  |  | 0.246 |
| walker |  | 828 | 37 | c decl at sds.h:47 |  |  | 0.247 |
| ns | 850 |  | 94 | README: embedding SDS into a project (which three files to copy) | 1.8 |  | 0.227 |
| walker |  | 893 | 65 | c decl at sds.h:51 |  |  | 0.231 |
| walker |  | 958 | 65 | c decl at sds.h:57 |  |  | 0.232 |
| walker |  | 1023 | 65 | c decl at sds.h:63 |  |  | 0.236 |
| ns | 1047 |  | 197 | sds.h prototypes: creation, destruction, growth, concat, copy (218-229) | 2.1 |  | 0.216 |
| walker |  | 1088 | 65 | c decl at sds.h:69 |  |  | 0.220 |
| walker |  | 1118 | 30 | c includes in sds.h |  |  | 0.252 |
| ns | 1192 |  | 145 | All six `static inline` accessors in sds.h (signature lines only) | 2.2 |  | 0.268 |
| ns | 1324 |  | 132 | sds.h prototypes: the printf-family, incl. the __GNUC__ format attribute (230-238) | 2.3 | 2.1 | 0.254 |
| ns | 1501 |  | 177 | sds.h prototypes: trim/range/inspect/tokenize (239-248) | 2.4 | 2.3 | 0.240 |
| walker |  | 1517 | 399 | headings outline in README.md |  |  | 0.392 |
| ns | 1621 |  | 120 | sds.h prototypes: repr/splitargs/mapchars/join (249-253) | 2.5 | 2.4 | 0.382 |
| ns | 1716 |  | 95 | sds.h low-level API block (255-260) | 2.6 | 2.5 | 0.370 |
| walker |  | 1862 | 345 | README.md section #0 |  |  | 0.686 |
| ns | 1864 |  | 148 | sds.h allocator exports + the REDIS_TEST-guarded sdsTest declaration (262-272) | 2.7 | 2.6 | 0.655 |
| walker |  | 1882 | 20 | c decl doc at sds.h:180 |  |  | 0.664 |
| ns | 2045 |  | 181 | sdshdr5 and sdshdr8 struct definitions, with the 'sdshdr5 is never used' note | 3.1 |  | 0.665 |
| ns | 2294 |  | 249 | sdshdr16 / sdshdr32 / sdshdr64 struct definitions | 3.2 | 3.1 | 0.691 |
| walker |  | 2333 | 451 | c decl names surface #1 in sds.h |  |  | 0.793 |
| walker |  | 2351 | 18 | c decl at sds.h:232 |  |  | 0.800 |
| ns | 2468 |  | 174 | SDS_TYPE_* constants and the SDS_HDR / SDS_HDR_VAR / SDS_TYPE_5_LEN macros | 3.3 |  | 0.805 |
| walker |  | 2631 | 280 | c decl names surface #2 in sds.h |  |  | 0.852 |
| walker |  | 2631 | 0 | c decl at sds.h:256 |  |  | 0.852 |
| walker |  | 2631 | 0 | c decl at sds.h:266 |  |  | 0.852 |
| ns | 2636 |  | 168 | sdslen() body — how a length is decoded from the flags byte | 3.4 | 2.2 | 0.815 |
| walker |  | 2643 | 12 | c decl doc at sds.h:256 |  |  | 0.825 |
| ns | 2733 |  | 97 | sdsavail() body — free space is `alloc - len`, and always 0 for type 5 | 3.5 | 2.2 | 0.802 |
| ns | 2942 |  | 209 | The type-5 write paths of sdssetlen() and sdsinclen(), rest elided | 3.6 | 2.2 | 0.763 |
| ns | 3080 |  | 138 | sdsalloc() body and the type-5 no-op in sdssetalloc() | 3.7 | 2.2 | 0.739 |
| walker |  | 3136 | 493 | c header banner in sds.h |  |  | 0.739 |
| walker |  | 3195 | 59 | c decl at testhelp.h:44 |  |  | 0.740 |
| walker |  | 3261 | 66 | c decl doc at sds.h:266 |  |  | 0.763 |
| walker |  | 3302 | 41 | c decl doc at sds.h:47 |  |  | 0.777 |
| ns | 3337 |  | 257 | Roster of every function defined in sds.c, part 1 (lines 44-440), names only | 4.1 |  | 0.741 |
| walker |  | 3401 | 99 | c decl at testhelp.h:48 |  |  | 0.742 |
| ns | 3650 |  | 313 | Roster of every function defined in sds.c, part 2 (lines 451-1325), names only | 4.2 | 4.1 | 0.704 |
| ns | 3791 |  | 141 | sds.c includes + SDS_NOINIT definition, and the sdsalloc.h allocator macros | 4.3 |  | 0.688 |
| walker |  | 3877 | 476 | c decl names surface in sds.c |  |  | 0.727 |
| walker |  | 3877 | 0 | c decl at sds.c:89 |  |  | 0.727 |
| walker |  | 3877 | 0 | c decl at sds.c:149 |  |  | 0.727 |
| walker |  | 3877 | 0 | c decl at sds.c:154 |  |  | 0.727 |
| walker |  | 3877 | 0 | c decl at sds.c:160 |  |  | 0.727 |
| walker |  | 3877 | 0 | c decl at sds.c:165 |  |  | 0.727 |
| walker |  | 3877 | 0 | c decl at sds.c:184 |  |  | 0.727 |
| walker |  | 3877 | 0 | c decl at sds.c:193 |  |  | 0.727 |
| walker |  | 3877 | 0 | c decl at sds.c:204 |  |  | 0.727 |
| walker |  | 3877 | 0 | c decl at sds.c:256 |  |  | 0.727 |
| walker |  | 3877 | 0 | c decl at sds.c:300 |  |  | 0.727 |
| walker |  | 3877 | 0 | c decl at sds.c:307 |  |  | 0.727 |
| walker |  | 3877 | 0 | c decl at sds.c:334 |  |  | 0.727 |
| walker |  | 3877 | 0 | c decl at sds.c:380 |  |  | 0.727 |
| walker |  | 3877 | 0 | c decl at sds.c:398 |  |  | 0.727 |
| walker |  | 3877 | 0 | c decl at sds.c:413 |  |  | 0.727 |
| walker |  | 3877 | 0 | c decl at sds.c:421 |  |  | 0.727 |
| walker |  | 3877 | 0 | c decl at sds.c:427 |  |  | 0.727 |
| walker |  | 3877 | 0 | c decl at sds.c:440 |  |  | 0.727 |
| walker |  | 3877 | 0 | c decl at sds.c:450 |  |  | 0.727 |
| walker |  | 3877 | 0 | c decl at sds.c:451 |  |  | 0.727 |
| walker |  | 3877 | 0 | c decl at sds.c:494 |  |  | 0.727 |
| walker |  | 3877 | 0 | c decl at sds.c:526 |  |  | 0.727 |
| walker |  | 3877 | 0 | c decl at sds.c:534 |  |  | 0.727 |
| walker |  | 3890 | 13 | c decl body at sds.c:149 |  |  | 0.727 |
| walker |  | 3902 | 12 | c decl doc at sds.c:160 |  |  | 0.727 |
| walker |  | 3919 | 17 | c decl body at sds.c:160 |  |  | 0.728 |
| walker |  | 3936 | 17 | c decl body at sds.c:413 |  |  | 0.728 |
| walker |  | 3953 | 17 | c decl body at sds.c:440 |  |  | 0.728 |
| walker |  | 3972 | 19 | c decl body at sds.c:307 |  |  | 0.728 |
| walker |  | 3991 | 19 | c decl body at sds.c:421 |  |  | 0.728 |
| ns | 4007 |  | 216 | sdsHdrSize() and sdsReqType() — the type-selection policy | 4.4 | 4.1 | 0.700 |
| walker |  | 4074 | 83 | c includes in sds.c |  |  | 0.722 |
| walker |  | 4094 | 20 | c decl doc at sds.c:154 |  |  | 0.722 |
| walker |  | 4121 | 27 | c decl body at sds.c:184 |  |  | 0.722 |
| walker |  | 4148 | 27 | c decl body at sds.c:193 |  |  | 0.722 |
| walker |  | 4169 | 21 | c decl doc at sds.c:494 |  |  | 0.722 |
| walker |  | 4191 | 22 | c decl doc at sds.c:534 |  |  | 0.722 |
| walker |  | 4214 | 23 | c decl doc at sds.c:165 |  |  | 0.722 |
| ns | 4216 |  | 209 | sdsnewlen() doc comment — NULL vs SDS_NOINIT init, and the always-null-terminated guarantee | 4.5 |  | 0.706 |
| walker |  | 4246 | 32 | c decl body at sds.c:165 |  |  | 0.707 |
| walker |  | 4278 | 32 | c decl body at sds.c:300 |  |  | 0.707 |
| walker |  | 4315 | 37 | c decl body at sds.c:154 |  |  | 0.707 |
| walker |  | 4349 | 34 | c decl doc at sds.c:307 |  |  | 0.708 |
| walker |  | 4385 | 36 | c decl doc at sds.c:149 |  |  | 0.708 |
| walker |  | 4432 | 47 | c decl body at sds.c:526 |  |  | 0.708 |
| walker |  | 4473 | 41 | c decl doc at sds.c:440 |  |  | 0.708 |
| ns | 4537 |  | 321 | sdsnewlen() body — single allocation, header write, type-5-to-8 upgrade | 4.6 | 4.1 | 0.680 |
| ns | 4883 |  | 346 | sdsMakeRoomFor(): contract comment and the growth policy (198-222) | 4.7 | 4.1 | 0.653 |
| walker |  | 4913 | 440 | c decl names surface #1 in sds.c |  |  | 0.693 |
| walker |  | 4913 | 0 | c decl at sds.c:591 |  |  | 0.693 |
| walker |  | 4913 | 0 | c decl at sds.c:616 |  |  | 0.693 |
| walker |  | 4913 | 0 | c decl at sds.c:725 |  |  | 0.693 |
| walker |  | 4913 | 0 | c decl at sds.c:756 |  |  | 0.693 |
| walker |  | 4913 | 0 | c decl at sds.c:783 |  |  | 0.693 |
| walker |  | 4913 | 0 | c decl at sds.c:790 |  |  | 0.693 |
| walker |  | 4913 | 0 | c decl at sds.c:807 |  |  | 0.693 |
| walker |  | 4913 | 0 | c decl at sds.c:835 |  |  | 0.693 |
| walker |  | 4913 | 0 | c decl at sds.c:885 |  |  | 0.693 |
| walker |  | 4913 | 0 | c decl at sds.c:898 |  |  | 0.693 |
| walker |  | 4913 | 0 | c decl at sds.c:925 |  |  | 0.693 |
| walker |  | 4913 | 0 | c decl at sds.c:932 |  |  | 0.693 |
| walker |  | 4913 | 0 | c decl at sds.c:973 |  |  | 0.693 |
| walker |  | 4913 | 0 | c decl at sds.c:1092 |  |  | 0.693 |
| walker |  | 4913 | 0 | c decl at sds.c:1108 |  |  | 0.693 |
| walker |  | 4913 | 0 | c decl at sds.c:1120 |  |  | 0.693 |
| walker |  | 4913 | 0 | c decl at sds.c:1136 |  |  | 0.693 |
| walker |  | 4933 | 20 | c decl doc at sds.c:1120 |  |  | 0.693 |
| walker |  | 4954 | 21 | c decl doc at sds.c:783 |  |  | 0.693 |
| walker |  | 4975 | 21 | c decl doc at sds.c:790 |  |  | 0.693 |
| walker |  | 5001 | 26 | c decl doc at sds.c:885 |  |  | 0.693 |
| walker |  | 5040 | 39 | c decl body at sds.c:885 |  |  | 0.693 |
| walker |  | 5073 | 33 | c decl doc at sds.c:925 |  |  | 0.693 |
| walker |  | 5107 | 34 | c decl doc at sds.c:932 |  |  | 0.693 |
| walker |  | 5155 | 48 | c decl body at sds.c:783 |  |  | 0.693 |
| walker |  | 5203 | 48 | c decl body at sds.c:790 |  |  | 0.693 |
| walker |  | 5252 | 49 | c decl body at sds.c:925 |  |  | 0.693 |
| ns | 5253 |  | 370 | sdsMakeRoomFor(): the realloc-vs-move-header branch (224-248) | 4.8 | 4.7 | 0.668 |
| walker |  | 5290 | 38 | c decl doc at sds.c:1108 |  |  | 0.668 |
| walker |  | 5332 | 42 | c decl doc at sds.c:427 |  |  | 0.668 |
| ns | 5486 |  | 233 | sdsempty/sdsnew/sdsdup/sdsfree bodies, with sdsfree's NULL contract | 4.9 | 4.1 | 0.671 |
| walker |  | 5497 | 165 | c decl body at sds.h:87 |  |  | 0.695 |
| walker |  | 5662 | 165 | c decl body at sds.h:180 |  |  | 0.701 |
| walker |  | 5726 | 64 | c decl body at sds.c:591 |  |  | 0.701 |
| walker |  | 5777 | 51 | c decl doc at sds.c:526 |  |  | 0.701 |
| ns | 5808 |  | 322 | sdsIncrLen() doc comment — the zero-copy read-into-the-buffer pattern | 4.10 |  | 0.681 |
| walker |  | 5982 | 205 | c decl body at sds.h:197 |  |  | 0.696 |
| ns | 6023 |  | 215 | sdscatfmt() doc comment — the supported format specifiers | 4.11 |  | 0.682 |
| walker |  | 6049 | 67 | c decl doc at sds.c:421 |  |  | 0.682 |
| ns | 6118 |  | 95 | README: the error-handling contract (NULL on out of memory) | 5.1 |  | 0.677 |
| walker |  | 6286 | 237 | c decl body at sds.h:130 |  |  | 0.684 |
| ns | 6345 |  | 227 | README: the preallocation algorithm and the SDS_MAX_PREALLOC cap | 5.2 |  | 0.673 |
| walker |  | 6354 | 68 | c decl doc at sds.c:413 |  |  | 0.673 |
| walker |  | 6423 | 69 | c decl doc at sds.c:193 |  |  | 0.674 |
| ns | 6485 |  | 140 | README: the internals section's `struct sdshdr` — documentation that is stale | 5.3 |  | 0.665 |
| walker |  | 6667 | 244 | c decl body at sds.h:104 |  |  | 0.680 |
| ns | 6751 |  | 266 | README: the two disadvantages — reassign the return value, and shared strings | 5.4 |  | 0.671 |
| walker |  | 6919 | 252 | c decl body at sds.h:154 |  |  | 0.694 |
| ns | 6968 |  | 217 | README basics: `sds` is a `char *`, and the three rules of the minimal program | 5.5 |  | 0.685 |
| walker |  | 6992 | 73 | c decl doc at sds.c:380 |  |  | 0.685 |
| walker |  | 7092 | 100 | c decl body at sds.c:427 |  |  | 0.685 |
| ns | 7160 |  | 192 | README: why sdstrim/sdsrange return void, and how negative indexes work | 5.6 |  | 0.677 |
| walker |  | 7196 | 104 | c decl body at sds.c:398 |  |  | 0.678 |
| walker |  | 7300 | 104 | c decl body at sds.c:1108 |  |  | 0.678 |
| ns | 7382 |  | 222 | README: swapping the allocator, and why sds_malloc/sds_realloc/sds_free are exported | 5.7 |  | 0.670 |
| walker |  | 7408 | 108 | c decl body at sds.c:1120 |  |  | 0.670 |
| walker |  | 7493 | 85 | c decl doc at sds.c:398 |  |  | 0.670 |
| ns | 7527 |  | 145 | README: the exact escaping rules sdscatrepr applies | 5.8 |  | 0.666 |
| walker |  | 7581 | 88 | c decl doc at sds.c:300 |  |  | 0.666 |
| walker |  | 7667 | 86 | c decl doc at sds.c:450 |  |  | 0.666 |
| ns | 7716 |  | 189 | README: tokenizer ownership rules and sdssplitargs' quoting behaviour | 5.9 |  | 0.658 |
| walker |  | 7765 | 98 | c decl doc at sds.c:256 |  |  | 0.658 |
| ns | 7880 |  | 164 | README: the camelCase warning and how heap checkers see SDS strings | 5.10 |  | 0.654 |
| walker |  | 7898 | 133 | c decl body at sds.c:1092 |  |  | 0.654 |
| walker |  | 8001 | 103 | c decl doc at sds.c:1136 |  |  | 0.654 |
| ns | 8197 |  | 317 | sdsrange() body — negative-index normalisation and clamping | 6.1 | 4.2 | 0.638 |
| ns | 8383 |  | 186 | sdscatlen/sdscat/sdscatsds bodies — the append path | 6.2 | 4.1 | 0.643 |
| walker |  | 8426 | 425 | README.md section #1 |  |  | 0.643 |
| ns | 8591 |  | 208 | sdsRemoveFreeSpace() — contract and the shrink decision | 6.3 | 4.1 | 0.633 |
| ns | 8763 |  | 172 | sdstrim() body — how both ends are walked and the survivor moved down | 6.4 | 4.2 | 0.626 |
| walker |  | 8844 | 418 | README.md section #2 |  |  | 0.639 |
| ns | 9015 |  | 252 | sdssplitargs() doc comment — REPL-style parsing and its failure mode | 6.5 |  | 0.629 |
| walker |  | 9252 | 408 | README.md section #3 |  |  | 0.631 |
| ns | 9256 |  | 241 | The small inspect/reset bodies: sdsupdatelen, sdsclear, sdsAllocSize, sdsAllocPtr | 6.6 | 4.1 | 0.636 |
| ns | 9375 |  | 119 | The test entry points in sds.c: the SDS_TEST_MAIN guard, sdsTest() and main() | 7.1 | 4.2 | 0.629 |
| walker |  | 9387 | 135 | c decl body at sds.c:380 |  |  | 0.629 |
| walker |  | 9525 | 138 | c decl body at sds.c:807 |  |  | 0.629 |
| ns | 9582 |  | 207 | testhelp.h: the complete test_cond / test_report macro pair | 7.2 |  | 0.635 |
| walker |  | 9635 | 110 | c decl doc at sds.c:898 |  |  | 0.635 |
| ns | 9665 |  | 83 | A representative excerpt of the sdsTest() body | 7.3 |  | 0.632 |
| walker |  | 9746 | 111 | c decl doc at sds.c:204 |  |  | 0.635 |
| ns | 9794 |  | 129 | The complete Changelog (v1.0 and v2.0) | 7.4 |  | 0.629 |
| walker |  | 9903 | 157 | c decl body at sds.c:725 |  |  | 0.642 |
| ns | 9921 |  | 127 | Authorship and licence: README credits, LICENSE header, .gitignore | 7.5 |  | 0.638 |
