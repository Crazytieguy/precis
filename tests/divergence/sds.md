Score(3000)=0.662 I=0.849 C=0.516 ns_rows≤3K=21/54 grid(1000/1442/2080/3000/4327/6240/9000)=0.222/0.236/0.349/0.662/0.704/0.591/0.542

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
| walker |  | 1282 | 215 | c names sds.h #1 |  |  | 0.239 |
| walker |  | 1302 | 20 | c doc sds.h:180 |  |  | 0.249 |
| ns | 1324 |  | 132 | sds.h prototypes: the printf-family, incl. the __GNUC__ format attribute (230-238) | 2.3 | 2.1 | 0.236 |
| ns | 1501 |  | 177 | sds.h prototypes: trim/range/inspect/tokenize (239-248) | 2.4 | 2.3 | 0.223 |
| walker |  | 1541 | 239 | c names sds.c |  |  | 0.226 |
| walker |  | 1553 | 12 | c doc sds.c:160 |  |  | 0.226 |
| walker |  | 1566 | 13 | c body sds.c:149 |  |  | 0.226 |
| walker |  | 1583 | 17 | c body sds.c:160 |  |  | 0.226 |
| walker |  | 1603 | 20 | c doc sds.c:154 |  |  | 0.227 |
| ns | 1621 |  | 120 | sds.h prototypes: repr/splitargs/mapchars/join (249-253) | 2.5 | 2.4 | 0.220 |
| ns | 1716 |  | 95 | sds.h low-level API block (255-260) | 2.6 | 2.5 | 0.214 |
| ns | 1864 |  | 148 | sds.h allocator exports + the REDIS_TEST-guarded sdsTest declaration (262-272) | 2.7 | 2.6 | 0.204 |
| walker |  | 2002 | 399 | headings outline in README.md |  |  | 0.342 |
| ns | 2045 |  | 181 | sdshdr5 and sdshdr8 struct definitions, with the 'sdshdr5 is never used' note | 3.1 |  | 0.349 |
| walker |  | 2124 | 122 | README.md section #0 |  |  | 0.349 |
| ns | 2294 |  | 249 | sdshdr16 / sdshdr32 / sdshdr64 struct definitions | 3.2 | 3.1 | 0.374 |
| walker |  | 2342 | 218 | README.md section #1 |  |  | 0.673 |
| ns | 2468 |  | 174 | SDS_TYPE_* constants and the SDS_HDR / SDS_HDR_VAR / SDS_TYPE_5_LEN macros | 3.3 |  | 0.685 |
| walker |  | 2573 | 231 | c names sds.c #1 |  |  | 0.689 |
| ns | 2636 |  | 168 | sdslen() body — how a length is decoded from the flags byte | 3.4 | 2.2 | 0.658 |
| ns | 2733 |  | 97 | sdsavail() body — free space is `alloc - len`, and always 0 for type 5 | 3.5 | 2.2 | 0.640 |
| walker |  | 2780 | 207 | c names sds.h #2 |  |  | 0.690 |
| walker |  | 2805 | 25 | c decl sds.h:231 |  |  | 0.696 |
| walker |  | 2822 | 17 | c body sds.c:413 |  |  | 0.696 |
| ns | 2942 |  | 209 | The type-5 write paths of sdssetlen() and sdsinclen(), rest elided | 3.6 | 2.2 | 0.662 |
| walker |  | 3048 | 226 | c names sds.c #2 |  |  | 0.663 |
| ns | 3080 |  | 138 | sdsalloc() body and the type-5 no-op in sdssetalloc() | 3.7 | 2.2 | 0.643 |
| walker |  | 3244 | 196 | c names sds.h #3 |  |  | 0.678 |
| walker |  | 3254 | 10 | c decl sds.h:234 |  |  | 0.685 |
| ns | 3337 |  | 257 | Roster of every function defined in sds.c, part 1 (lines 44-440), names only | 4.1 |  | 0.703 |
| walker |  | 3513 | 259 | c names sds.c #3 |  |  | 0.707 |
| ns | 3650 |  | 313 | Roster of every function defined in sds.c, part 2 (lines 451-1325), names only | 4.2 | 4.1 | 0.717 |
| ns | 3791 |  | 141 | sds.c includes + SDS_NOINIT definition, and the sdsalloc.h allocator macros | 4.3 |  | 0.702 |
| walker |  | 3805 | 292 | c names sds.h #4 |  |  | 0.738 |
| walker |  | 3820 | 15 | c decl sds.h:270 |  |  | 0.740 |
| walker |  | 3832 | 12 | c doc sds.h:256 |  |  | 0.748 |
| walker |  | 3849 | 17 | c body sds.c:440 |  |  | 0.748 |
| walker |  | 3868 | 19 | c body sds.c:307 |  |  | 0.748 |
| ns | 4007 |  | 216 | sdsHdrSize() and sdsReqType() — the type-selection policy | 4.4 | 4.1 | 0.720 |
| ns | 4216 |  | 209 | sdsnewlen() doc comment — NULL vs SDS_NOINIT init, and the always-null-terminated guarantee | 4.5 |  | 0.704 |
| walker |  | 4293 | 425 | README.md section #2 |  |  | 0.704 |
| ns | 4537 |  | 321 | sdsnewlen() body — single allocation, header write, type-5-to-8 upgrade | 4.6 | 4.1 | 0.676 |
| walker |  | 4711 | 418 | README.md section #3 |  |  | 0.677 |
| ns | 4883 |  | 346 | sdsMakeRoomFor(): contract comment and the growth policy (198-222) | 4.7 | 4.1 | 0.651 |
| walker |  | 5119 | 408 | README.md section #4 |  |  | 0.651 |
| walker |  | 5160 | 41 | c doc sds.h:47 |  |  | 0.661 |
| walker |  | 5179 | 19 | c body sds.c:421 |  |  | 0.661 |
| ns | 5253 |  | 370 | sdsMakeRoomFor(): the realloc-vs-move-header branch (224-248) | 4.8 | 4.7 | 0.638 |
| ns | 5486 |  | 233 | sdsempty/sdsnew/sdsdup/sdsfree bodies, with sdsfree's NULL contract | 4.9 | 4.1 | 0.625 |
| walker |  | 5706 | 527 | README.md section #5 |  |  | 0.626 |
| ns | 5808 |  | 322 | sdsIncrLen() doc comment — the zero-copy read-into-the-buffer pattern | 4.10 |  | 0.607 |
| ns | 6023 |  | 215 | sdscatfmt() doc comment — the supported format specifiers | 4.11 |  | 0.596 |
| ns | 6118 |  | 95 | README: the error-handling contract (NULL on out of memory) | 5.1 |  | 0.591 |
| walker |  | 6151 | 445 | README.md section #6 |  |  | 0.591 |
| ns | 6345 |  | 227 | README: the preallocation algorithm and the SDS_MAX_PREALLOC cap | 5.2 |  | 0.582 |
| ns | 6485 |  | 140 | README: the internals section's `struct sdshdr` — documentation that is stale | 5.3 |  | 0.574 |
| walker |  | 6655 | 504 | README.md section #7 |  |  | 0.574 |
| walker |  | 6675 | 20 | c doc sds.c:1120 |  |  | 0.574 |
| ns | 6751 |  | 266 | README: the two disadvantages — reassign the return value, and shared strings | 5.4 |  | 0.583 |
| ns | 6968 |  | 217 | README basics: `sds` is a `char *`, and the three rules of the minimal program | 5.5 |  | 0.589 |
| ns | 7160 |  | 192 | README: why sdstrim/sdsrange return void, and how negative indexes work | 5.6 |  | 0.582 |
| walker |  | 7289 | 614 | README.md section #8 |  |  | 0.582 |
| ns | 7382 |  | 222 | README: swapping the allocator, and why sds_malloc/sds_realloc/sds_free are exported | 5.7 |  | 0.575 |
| ns | 7527 |  | 145 | README: the exact escaping rules sdscatrepr applies | 5.8 |  | 0.571 |
| ns | 7716 |  | 189 | README: tokenizer ownership rules and sdssplitargs' quoting behaviour | 5.9 |  | 0.565 |
| walker |  | 7809 | 520 | README.md section #9 |  |  | 0.565 |
| ns | 7880 |  | 164 | README: the camelCase warning and how heap checkers see SDS strings | 5.10 |  | 0.561 |
| ns | 8197 |  | 317 | sdsrange() body — negative-index normalisation and clamping | 6.1 | 4.2 | 0.548 |
| walker |  | 8258 | 449 | README.md section #10 |  |  | 0.556 |
| ns | 8383 |  | 186 | sdscatlen/sdscat/sdscatsds bodies — the append path | 6.2 | 4.1 | 0.550 |
| ns | 8591 |  | 208 | sdsRemoveFreeSpace() — contract and the shrink decision | 6.3 | 4.1 | 0.542 |
| ns | 8763 |  | 172 | sdstrim() body — how both ends are walked and the survivor moved down | 6.4 | 4.2 | 0.536 |
| walker |  | 8859 | 601 | README.md section #11 |  |  | 0.542 |
| ns | 9015 |  | 252 | sdssplitargs() doc comment — REPL-style parsing and its failure mode | 6.5 |  | 0.533 |
| ns | 9256 |  | 241 | The small inspect/reset bodies: sdsupdatelen, sdsclear, sdsAllocSize, sdsAllocPtr | 6.6 | 4.1 | 0.525 |
| walker |  | 9273 | 414 | README.md section #12 |  |  | 0.525 |
| ns | 9375 |  | 119 | The test entry points in sds.c: the SDS_TEST_MAIN guard, sdsTest() and main() | 7.1 | 4.2 | 0.519 |
| ns | 9582 |  | 207 | testhelp.h: the complete test_cond / test_report macro pair | 7.2 |  | 0.527 |
| ns | 9665 |  | 83 | A representative excerpt of the sdsTest() body | 7.3 |  | 0.524 |
| walker |  | 9790 | 517 | README.md section #13 |  |  | 0.524 |
| ns | 9794 |  | 129 | The complete Changelog (v1.0 and v2.0) | 7.4 |  | 0.519 |
| ns | 9921 |  | 127 | Authorship and licence: README credits, LICENSE header, .gitignore | 7.5 |  | 0.517 |
