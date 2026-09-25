Score(3000)=0.353 I=0.312 C=0.401 ns_rows≤3K=21/54 grid(1000/1442/2080/3000/4327/6240/9000)=0.222/0.191/0.218/0.353/0.592/0.585/0.607

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 31 | 31 | listing of '.' |  |  | 0.000 |
| ns | 94 |  | 94 | README title + the one-paragraph definition of SDS | 1.1 |  | 0.000 |
| ns | 125 |  | 31 | Complete root directory listing | 1.2 |  | 0.333 |
| walker |  | 148 | 117 | plaintext config Makefile |  |  | 0.360 |
| walker |  | 181 | 33 | c names sdsalloc.h |  |  | 0.360 |
| walker |  | 239 | 58 | c names testhelp.h |  |  | 0.360 |
| ns | 251 |  | 126 | The header-before-the-pointer design + ASCII layout diagram | 1.3 |  | 0.275 |
| walker |  | 298 | 59 | c decl testhelp.h:44 |  |  | 0.276 |
| ns | 365 |  | 114 | sds.h preamble: include guard, SDS_MAX_PREALLOC, SDS_NOINIT, `typedef char *sds` | 1.4 |  | 0.222 |
| walker |  | 407 | 109 | README headline in README.md |  |  | 0.233 |
| walker |  | 506 | 99 | c decl testhelp.h:48 |  |  | 0.234 |
| ns | 533 |  | 168 | README headings, part 1: manual chapters through Error handling | 1.5 |  | 0.186 |
| ns | 639 |  | 106 | README headings, part 2: internals chapter + back matter (696-911) | 1.6 | 1.5 | 0.169 |
| ns | 756 |  | 117 | The complete Makefile (build + run the in-file unit tests) | 1.7 |  | 0.216 |
| walker |  | 770 | 264 | c names sds.h |  |  | 0.234 |
| walker |  | 807 | 37 | c decl sds.h:47 |  |  | 0.235 |
| ns | 850 |  | 94 | README: embedding SDS into a project (which three files to copy) | 1.8 |  | 0.216 |
| walker |  | 872 | 65 | c decl sds.h:51 |  |  | 0.220 |
| walker |  | 937 | 65 | c decl sds.h:57 |  |  | 0.222 |
| walker |  | 1002 | 65 | c decl sds.h:63 |  |  | 0.225 |
| ns | 1047 |  | 197 | sds.h prototypes: creation, destruction, growth, concat, copy (218-229) | 2.1 |  | 0.206 |
| walker |  | 1067 | 65 | c decl sds.h:69 |  |  | 0.211 |
| ns | 1192 |  | 145 | All six `static inline` accessors in sds.h (signature lines only) | 2.2 |  | 0.202 |
| ns | 1324 |  | 132 | sds.h prototypes: the printf-family, incl. the __GNUC__ format attribute (230-238) | 2.3 | 2.1 | 0.191 |
| walker |  | 1350 | 283 | c module doc sds.h |  |  | 0.191 |
| ns | 1501 |  | 177 | sds.h prototypes: trim/range/inspect/tokenize (239-248) | 2.4 | 2.3 | 0.181 |
| walker |  | 1565 | 215 | c names sds.h #1 |  |  | 0.214 |
| walker |  | 1585 | 20 | c doc sds.h:180 |  |  | 0.223 |
| ns | 1621 |  | 120 | sds.h prototypes: repr/splitargs/mapchars/join (249-253) | 2.5 | 2.4 | 0.217 |
| ns | 1716 |  | 95 | sds.h low-level API block (255-260) | 2.6 | 2.5 | 0.210 |
| ns | 1864 |  | 148 | sds.h allocator exports + the REDIS_TEST-guarded sdsTest declaration (262-272) | 2.7 | 2.6 | 0.201 |
| walker |  | 1909 | 324 | c module doc testhelp.h |  |  | 0.201 |
| ns | 2045 |  | 181 | sdshdr5 and sdshdr8 struct definitions, with the 'sdshdr5 is never used' note | 3.1 |  | 0.218 |
| walker |  | 2192 | 283 | c module doc sdsalloc.h |  |  | 0.218 |
| ns | 2294 |  | 249 | sdshdr16 / sdshdr32 / sdshdr64 struct definitions | 3.2 | 3.1 | 0.253 |
| walker |  | 2431 | 239 | c names sds.c |  |  | 0.257 |
| ns | 2468 |  | 174 | SDS_TYPE_* constants and the SDS_HDR / SDS_HDR_VAR / SDS_TYPE_5_LEN macros | 3.3 |  | 0.272 |
| walker |  | 2472 | 41 | c doc sds.h:47 |  |  | 0.287 |
| walker |  | 2484 | 12 | c doc sds.c:160 |  |  | 0.287 |
| walker |  | 2497 | 13 | c body sds.c:149 |  |  | 0.287 |
| walker |  | 2514 | 17 | c body sds.c:160 |  |  | 0.287 |
| walker |  | 2534 | 20 | c doc sds.c:154 |  |  | 0.287 |
| walker |  | 2557 | 23 | c doc sds.c:165 |  |  | 0.288 |
| ns | 2636 |  | 168 | sdslen() body — how a length is decoded from the flags byte | 3.4 | 2.2 | 0.275 |
| ns | 2733 |  | 97 | sdsavail() body — free space is `alloc - len`, and always 0 for type 5 | 3.5 | 2.2 | 0.267 |
| ns | 2942 |  | 209 | The type-5 write paths of sdssetlen() and sdsinclen(), rest elided | 3.6 | 2.2 | 0.254 |
| walker |  | 2956 | 399 | headings outline in README.md |  |  | 0.353 |
| walker |  | 3078 | 122 | README.md section #0 |  |  | 0.353 |
| ns | 3080 |  | 138 | sdsalloc() body and the type-5 no-op in sdssetalloc() | 3.7 | 2.2 | 0.342 |
| walker |  | 3296 | 218 | README.md section #1 |  |  | 0.603 |
| ns | 3337 |  | 257 | Roster of every function defined in sds.c, part 1 (lines 44-440), names only | 4.1 |  | 0.595 |
| walker |  | 3506 | 210 | c module doc sds.h #1 |  |  | 0.595 |
| ns | 3650 |  | 313 | Roster of every function defined in sds.c, part 2 (lines 451-1325), names only | 4.2 | 4.1 | 0.565 |
| walker |  | 3737 | 231 | c names sds.c #1 |  |  | 0.599 |
| walker |  | 3754 | 17 | c body sds.c:413 |  |  | 0.599 |
| walker |  | 3771 | 17 | c body sds.c:440 |  |  | 0.599 |
| walker |  | 3790 | 19 | c body sds.c:307 |  |  | 0.599 |
| ns | 3791 |  | 141 | sds.c includes + SDS_NOINIT definition, and the sdsalloc.h allocator macros | 4.3 |  | 0.588 |
| walker |  | 3809 | 19 | c body sds.c:421 |  |  | 0.588 |
| walker |  | 3830 | 21 | c doc sds.c:494 |  |  | 0.588 |
| walker |  | 3857 | 27 | c body sds.c:184 |  |  | 0.588 |
| walker |  | 3884 | 27 | c body sds.c:193 |  |  | 0.588 |
| ns | 4007 |  | 216 | sdsHdrSize() and sdsReqType() — the type-selection policy | 4.4 | 4.1 | 0.567 |
| walker |  | 4091 | 207 | c names sds.h #2 |  |  | 0.603 |
| walker |  | 4109 | 18 | c decl sds.h:232 |  |  | 0.606 |
| ns | 4216 |  | 209 | sdsnewlen() doc comment — NULL vs SDS_NOINIT init, and the always-null-terminated guarantee | 4.5 |  | 0.592 |
| walker |  | 4319 | 210 | c module doc testhelp.h #1 |  |  | 0.592 |
| ns | 4537 |  | 321 | sdsnewlen() body — single allocation, header write, type-5-to-8 upgrade | 4.6 | 4.1 | 0.568 |
| walker |  | 4618 | 299 | c module doc sdsalloc.h #1 |  |  | 0.568 |
| walker |  | 4650 | 32 | c body sds.c:165 |  |  | 0.569 |
| walker |  | 4682 | 32 | c body sds.c:300 |  |  | 0.569 |
| walker |  | 4716 | 34 | c doc sds.c:307 |  |  | 0.569 |
| walker |  | 4752 | 36 | c doc sds.c:149 |  |  | 0.569 |
| walker |  | 4789 | 37 | c body sds.c:154 |  |  | 0.570 |
| ns | 4883 |  | 346 | sdsMakeRoomFor(): contract comment and the growth policy (198-222) | 4.7 | 4.1 | 0.548 |
| walker |  | 4954 | 165 | c body sds.h:87 |  |  | 0.577 |
| walker |  | 5119 | 165 | c body sds.h:180 |  |  | 0.584 |
| walker |  | 5160 | 41 | c doc sds.c:440 |  |  | 0.584 |
| walker |  | 5202 | 42 | c doc sds.c:427 |  |  | 0.584 |
| ns | 5253 |  | 370 | sdsMakeRoomFor(): the realloc-vs-move-header branch (224-248) | 4.8 | 4.7 | 0.564 |
| walker |  | 5428 | 226 | c names sds.c #2 |  |  | 0.574 |
| walker |  | 5449 | 21 | c doc sds.c:783 |  |  | 0.574 |
| walker |  | 5470 | 21 | c doc sds.c:790 |  |  | 0.574 |
| ns | 5486 |  | 233 | sdsempty/sdsnew/sdsdup/sdsfree bodies, with sdsfree's NULL contract | 4.9 | 4.1 | 0.581 |
| walker |  | 5492 | 22 | c doc sds.c:534 |  |  | 0.581 |
| walker |  | 5688 | 196 | c names sds.h #3 |  |  | 0.603 |
| walker |  | 5735 | 47 | c body sds.c:526 |  |  | 0.603 |
| ns | 5808 |  | 322 | sdsIncrLen() doc comment — the zero-copy read-into-the-buffer pattern | 4.10 |  | 0.585 |
| walker |  | 5940 | 205 | c body sds.h:197 |  |  | 0.601 |
| walker |  | 5988 | 48 | c body sds.c:783 |  |  | 0.601 |
| ns | 6023 |  | 215 | sdscatfmt() doc comment — the supported format specifiers | 4.11 |  | 0.589 |
| walker |  | 6036 | 48 | c body sds.c:790 |  |  | 0.589 |
| walker |  | 6087 | 51 | c doc sds.c:526 |  |  | 0.589 |
| ns | 6118 |  | 95 | README: the error-handling contract (NULL on out of memory) | 5.1 |  | 0.585 |
| walker |  | 6324 | 237 | c body sds.h:130 |  |  | 0.592 |
| ns | 6345 |  | 227 | README: the preallocation algorithm and the SDS_MAX_PREALLOC cap | 5.2 |  | 0.583 |
| ns | 6485 |  | 140 | README: the internals section's `struct sdshdr` — documentation that is stale | 5.3 |  | 0.575 |
| walker |  | 6568 | 244 | c body sds.h:104 |  |  | 0.591 |
| ns | 6751 |  | 266 | README: the two disadvantages — reassign the return value, and shared strings | 5.4 |  | 0.583 |
| walker |  | 6820 | 252 | c body sds.h:154 |  |  | 0.606 |
| walker |  | 6887 | 67 | c doc sds.c:421 |  |  | 0.606 |
| ns | 6968 |  | 217 | README basics: `sds` is a `char *`, and the three rules of the minimal program | 5.5 |  | 0.599 |
| walker |  | 7146 | 259 | c names sds.c #3 |  |  | 0.623 |
| ns | 7160 |  | 192 | README: why sdstrim/sdsrange return void, and how negative indexes work | 5.6 |  | 0.615 |
| walker |  | 7166 | 20 | c doc sds.c:1120 |  |  | 0.615 |
| walker |  | 7192 | 26 | c doc sds.c:885 |  |  | 0.615 |
| walker |  | 7225 | 33 | c doc sds.c:925 |  |  | 0.615 |
| walker |  | 7259 | 34 | c doc sds.c:932 |  |  | 0.615 |
| walker |  | 7297 | 38 | c doc sds.c:1108 |  |  | 0.615 |
| walker |  | 7336 | 39 | c body sds.c:885 |  |  | 0.615 |
| ns | 7382 |  | 222 | README: swapping the allocator, and why sds_malloc/sds_realloc/sds_free are exported | 5.7 |  | 0.608 |
| walker |  | 7385 | 49 | c body sds.c:925 |  |  | 0.608 |
| walker |  | 7449 | 64 | c body sds.c:591 |  |  | 0.608 |
| walker |  | 7517 | 68 | c doc sds.c:413 |  |  | 0.608 |
| ns | 7527 |  | 145 | README: the exact escaping rules sdscatrepr applies | 5.8 |  | 0.604 |
| walker |  | 7586 | 69 | c doc sds.c:193 |  |  | 0.605 |
| ns | 7716 |  | 189 | README: tokenizer ownership rules and sdssplitargs' quoting behaviour | 5.9 |  | 0.598 |
| walker |  | 7878 | 292 | c names sds.h #4 |  |  | 0.621 |
| ns | 7880 |  | 164 | README: the camelCase warning and how heap checkers see SDS strings | 5.10 |  | 0.617 |
| walker |  | 7890 | 12 | c doc sds.h:256 |  |  | 0.622 |
| walker |  | 7956 | 66 | c doc sds.h:266 |  |  | 0.633 |
| walker |  | 8029 | 73 | c doc sds.c:380 |  |  | 0.633 |
| ns | 8197 |  | 317 | sdsrange() body — negative-index normalisation and clamping | 6.1 | 4.2 | 0.618 |
| walker |  | 8312 | 283 | c module doc sds.c |  |  | 0.618 |
| ns | 8383 |  | 186 | sdscatlen/sdscat/sdscatsds bodies — the append path | 6.2 | 4.1 | 0.611 |
| walker |  | 8397 | 85 | c doc sds.c:398 |  |  | 0.611 |
| walker |  | 8483 | 86 | c doc sds.c:450 |  |  | 0.611 |
| walker |  | 8571 | 88 | c doc sds.c:300 |  |  | 0.611 |
| ns | 8591 |  | 208 | sdsRemoveFreeSpace() — contract and the shrink decision | 6.3 | 4.1 | 0.602 |
| walker |  | 8669 | 98 | c doc sds.c:256 |  |  | 0.602 |
| ns | 8763 |  | 172 | sdstrim() body — how both ends are walked and the survivor moved down | 6.4 | 4.2 | 0.595 |
| walker |  | 8772 | 103 | c doc sds.c:1136 |  |  | 0.595 |
| walker |  | 8872 | 100 | c body sds.c:427 |  |  | 0.595 |
| walker |  | 8976 | 104 | c body sds.c:398 |  |  | 0.607 |
| ns | 9015 |  | 252 | sdssplitargs() doc comment — REPL-style parsing and its failure mode | 6.5 |  | 0.597 |
| walker |  | 9080 | 104 | c body sds.c:1108 |  |  | 0.597 |
| walker |  | 9190 | 110 | c doc sds.c:898 |  |  | 0.597 |
| ns | 9256 |  | 241 | The small inspect/reset bodies: sdsupdatelen, sdsclear, sdsAllocSize, sdsAllocPtr | 6.6 | 4.1 | 0.603 |
| walker |  | 9301 | 111 | c doc sds.c:204 |  |  | 0.607 |
| ns | 9375 |  | 119 | The test entry points in sds.c: the SDS_TEST_MAIN guard, sdsTest() and main() | 7.1 | 4.2 | 0.601 |
| walker |  | 9409 | 108 | c body sds.c:1120 |  |  | 0.601 |
| ns | 9582 |  | 207 | testhelp.h: the complete test_cond / test_report macro pair | 7.2 |  | 0.607 |
| walker |  | 9619 | 210 | c module doc sds.c #1 |  |  | 0.607 |
| ns | 9665 |  | 83 | A representative excerpt of the sdsTest() body | 7.3 |  | 0.603 |
| walker |  | 9752 | 133 | c body sds.c:1092 |  |  | 0.603 |
| ns | 9794 |  | 129 | The complete Changelog (v1.0 and v2.0) | 7.4 |  | 0.598 |
| walker |  | 9892 | 140 | c doc sds.c:807 |  |  | 0.598 |
| ns | 9921 |  | 127 | Authorship and licence: README credits, LICENSE header, .gitignore | 7.5 |  | 0.594 |
