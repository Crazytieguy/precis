Score(3000)=0.796 I=0.888 C=0.713 ns_rows≤3K=21/54 grid(1000/1442/2080/3000/4327/6240/9000)=0.405/0.360/0.463/0.796/0.642/0.573/0.551

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 31 | 31 | Fs::DirListing { dir: . } |  |  | 0.000 |
| ns | 94 |  | 94 | README title + the one-paragraph definition of SDS | 1.1 |  | 0.000 |
| ns | 125 |  | 31 | Complete root directory listing | 1.2 |  | 0.333 |
| walker |  | 140 | 109 | Markdown::ReadmeHeadline { file: README.md } |  |  | 0.352 |
| ns | 251 |  | 126 | The header-before-the-pointer design + ASCII layout diagram | 1.3 |  | 0.269 |
| walker |  | 257 | 117 | Plaintext::Whole { file: Makefile } |  |  | 0.288 |
| ns | 365 |  | 114 | sds.h preamble: include guard, SDS_MAX_PREALLOC, SDS_NOINIT, `typedef char *sds` | 1.4 |  | 0.232 |
| ns | 533 |  | 168 | README headings, part 1: manual chapters through Error handling | 1.5 |  | 0.184 |
| ns | 639 |  | 106 | README headings, part 2: internals chapter + back matter (696-911) | 1.6 | 1.5 | 0.167 |
| walker |  | 656 | 399 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.413 |
| ns | 756 |  | 117 | The complete Makefile (build + run the in-file unit tests) | 1.7 |  | 0.425 |
| walker |  | 778 | 122 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.425 |
| ns | 850 |  | 94 | README: embedding SDS into a project (which three files to copy) | 1.8 |  | 0.390 |
| walker |  | 984 | 206 | Code::CodeKey { rung: Names, file: sds.h, decl: 0, sub: 0, line: 0 } |  |  | 0.405 |
| walker |  | 1021 | 37 | Code::CodeKey { rung: Decl, file: sds.h, decl: 4, sub: 0, line: 47 } |  |  | 0.406 |
| ns | 1047 |  | 197 | sds.h prototypes: creation, destruction, growth, concat, copy (218-229) | 2.1 |  | 0.372 |
| walker |  | 1086 | 65 | Code::CodeKey { rung: Decl, file: sds.h, decl: 5, sub: 0, line: 51 } |  |  | 0.376 |
| walker |  | 1151 | 65 | Code::CodeKey { rung: Decl, file: sds.h, decl: 6, sub: 0, line: 57 } |  |  | 0.379 |
| ns | 1192 |  | 145 | All six `static inline` accessors in sds.h (signature lines only) | 2.2 |  | 0.363 |
| walker |  | 1216 | 65 | Code::CodeKey { rung: Decl, file: sds.h, decl: 7, sub: 0, line: 63 } |  |  | 0.367 |
| walker |  | 1281 | 65 | Code::CodeKey { rung: Decl, file: sds.h, decl: 8, sub: 0, line: 69 } |  |  | 0.372 |
| ns | 1324 |  | 132 | sds.h prototypes: the printf-family, incl. the __GNUC__ format attribute (230-238) | 2.3 | 2.1 | 0.352 |
| walker |  | 1454 | 173 | Code::CodeKey { rung: Names, file: sds.h, decl: 0, sub: 1, line: 0 } |  |  | 0.364 |
| ns | 1501 |  | 177 | sds.h prototypes: trim/range/inspect/tokenize (239-248) | 2.4 | 2.3 | 0.344 |
| ns | 1621 |  | 120 | sds.h prototypes: repr/splitargs/mapchars/join (249-253) | 2.5 | 2.4 | 0.334 |
| walker |  | 1632 | 178 | Code::CodeKey { rung: Names, file: sds.h, decl: 0, sub: 2, line: 0 } |  |  | 0.372 |
| walker |  | 1652 | 20 | Code::CodeKey { rung: Doc, file: sds.h, decl: 23, sub: 0, line: 180 } |  |  | 0.380 |
| ns | 1716 |  | 95 | sds.h low-level API block (255-260) | 2.6 | 2.5 | 0.368 |
| walker |  | 1827 | 175 | Code::CodeKey { rung: Names, file: sds.h, decl: 0, sub: 3, line: 0 } |  |  | 0.419 |
| walker |  | 1839 | 12 | Code::CodeKey { rung: Decl, file: sds.h, decl: 38, sub: 0, line: 234 } |  |  | 0.426 |
| walker |  | 1862 | 23 | Code::CodeKey { rung: Decl, file: sds.h, decl: 37, sub: 0, line: 231 } |  |  | 0.440 |
| ns | 1864 |  | 148 | sds.h allocator exports + the REDIS_TEST-guarded sdsTest declaration (262-272) | 2.7 | 2.6 | 0.420 |
| walker |  | 2024 | 162 | Code::CodeKey { rung: Names, file: sds.h, decl: 0, sub: 4, line: 0 } |  |  | 0.454 |
| ns | 2045 |  | 181 | sdshdr5 and sdshdr8 struct definitions, with the 'sdshdr5 is never used' note | 3.1 |  | 0.453 |
| walker |  | 2181 | 157 | Code::CodeKey { rung: Names, file: sds.h, decl: 0, sub: 5, line: 0 } |  |  | 0.480 |
| walker |  | 2193 | 12 | Code::CodeKey { rung: Doc, file: sds.h, decl: 55, sub: 0, line: 256 } |  |  | 0.485 |
| ns | 2294 |  | 249 | sdshdr16 / sdshdr32 / sdshdr64 struct definitions | 3.2 | 3.1 | 0.496 |
| walker |  | 2316 | 123 | Code::CodeKey { rung: Names, file: sds.h, decl: 0, sub: 6, line: 0 } |  |  | 0.515 |
| walker |  | 2331 | 15 | Code::CodeKey { rung: Decl, file: sds.h, decl: 63, sub: 0, line: 270 } |  |  | 0.518 |
| ns | 2468 |  | 174 | SDS_TYPE_* constants and the SDS_HDR / SDS_HDR_VAR / SDS_TYPE_5_LEN macros | 3.3 |  | 0.521 |
| walker |  | 2549 | 218 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.854 |
| walker |  | 2590 | 41 | Code::CodeKey { rung: Doc, file: sds.h, decl: 4, sub: 0, line: 47 } |  |  | 0.871 |
| ns | 2636 |  | 168 | sdslen() body — how a length is decoded from the flags byte | 3.4 | 2.2 | 0.833 |
| walker |  | 2656 | 66 | Code::CodeKey { rung: Doc, file: sds.h, decl: 60, sub: 0, line: 266 } |  |  | 0.860 |
| ns | 2733 |  | 97 | sdsavail() body — free space is `alloc - len`, and always 0 for type 5 | 3.5 | 2.2 | 0.836 |
| ns | 2942 |  | 209 | The type-5 write paths of sdssetlen() and sdsinclen(), rest elided | 3.6 | 2.2 | 0.796 |
| ns | 3080 |  | 138 | sdsalloc() body and the type-5 no-op in sdssetalloc() | 3.7 | 2.2 | 0.771 |
| walker |  | 3081 | 425 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.771 |
| ns | 3337 |  | 257 | Roster of every function defined in sds.c, part 1 (lines 44-440), names only | 4.1 |  | 0.735 |
| walker |  | 3499 | 418 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.737 |
| ns | 3650 |  | 313 | Roster of every function defined in sds.c, part 2 (lines 451-1325), names only | 4.2 | 4.1 | 0.699 |
| ns | 3791 |  | 141 | sds.c includes + SDS_NOINIT definition, and the sdsalloc.h allocator macros | 4.3 |  | 0.682 |
| walker |  | 3904 | 405 | Markdown::Section { file: README.md, section_index: 4, keeps_default_concavity: false } |  |  | 0.682 |
| ns | 4007 |  | 216 | sdsHdrSize() and sdsReqType() — the type-selection policy | 4.4 | 4.1 | 0.656 |
| ns | 4216 |  | 209 | sdsnewlen() doc comment — NULL vs SDS_NOINIT init, and the always-null-terminated guarantee | 4.5 |  | 0.641 |
| walker |  | 4431 | 527 | Markdown::Section { file: README.md, section_index: 5, keeps_default_concavity: false } |  |  | 0.642 |
| ns | 4537 |  | 321 | sdsnewlen() body — single allocation, header write, type-5-to-8 upgrade | 4.6 | 4.1 | 0.616 |
| walker |  | 4876 | 445 | Markdown::Section { file: README.md, section_index: 6, keeps_default_concavity: false } |  |  | 0.616 |
| ns | 4883 |  | 346 | sdsMakeRoomFor(): contract comment and the growth policy (198-222) | 4.7 | 4.1 | 0.592 |
| ns | 5253 |  | 370 | sdsMakeRoomFor(): the realloc-vs-move-header branch (224-248) | 4.8 | 4.7 | 0.571 |
| walker |  | 5380 | 504 | Markdown::Section { file: README.md, section_index: 7, keeps_default_concavity: false } |  |  | 0.571 |
| ns | 5486 |  | 233 | sdsempty/sdsnew/sdsdup/sdsfree bodies, with sdsfree's NULL contract | 4.9 | 4.1 | 0.555 |
| walker |  | 5584 | 204 | Code::CodeKey { rung: Names, file: sds.c, decl: 0, sub: 0, line: 0 } |  |  | 0.567 |
| walker |  | 5642 | 58 | Code::CodeKey { rung: Names, file: testhelp.h, decl: 0, sub: 0, line: 0 } |  |  | 0.567 |
| walker |  | 5701 | 59 | Code::CodeKey { rung: Decl, file: testhelp.h, decl: 3, sub: 0, line: 44 } |  |  | 0.567 |
| walker |  | 5800 | 99 | Code::CodeKey { rung: Decl, file: testhelp.h, decl: 4, sub: 0, line: 48 } |  |  | 0.568 |
| ns | 5808 |  | 322 | sdsIncrLen() doc comment — the zero-copy read-into-the-buffer pattern | 4.10 |  | 0.551 |
| walker |  | 5833 | 33 | Code::CodeKey { rung: Names, file: sdsalloc.h, decl: 0, sub: 0, line: 0 } |  |  | 0.554 |
| walker |  | 5845 | 12 | Code::CodeKey { rung: Doc, file: sds.c, decl: 7, sub: 0, line: 160 } |  |  | 0.554 |
| walker |  | 5865 | 20 | Code::CodeKey { rung: Doc, file: sds.c, decl: 6, sub: 0, line: 154 } |  |  | 0.556 |
| walker |  | 5888 | 23 | Code::CodeKey { rung: Doc, file: sds.c, decl: 8, sub: 0, line: 165 } |  |  | 0.558 |
| walker |  | 5901 | 13 | Code::CodeKey { rung: Body, file: sds.c, decl: 5, sub: 0, line: 149 } |  |  | 0.559 |
| ns | 6023 |  | 215 | sdscatfmt() doc comment — the supported format specifiers | 4.11 |  | 0.548 |
| walker |  | 6090 | 189 | Code::CodeKey { rung: Names, file: sds.c, decl: 0, sub: 1, line: 0 } |  |  | 0.571 |
| ns | 6118 |  | 95 | README: the error-handling contract (NULL on out of memory) | 5.1 |  | 0.567 |
| walker |  | 6258 | 168 | Code::CodeKey { rung: Names, file: sds.c, decl: 0, sub: 2, line: 0 } |  |  | 0.574 |
| walker |  | 6279 | 21 | Code::CodeKey { rung: Doc, file: sds.c, decl: 24, sub: 0, line: 494 } |  |  | 0.574 |
| walker |  | 6301 | 22 | Code::CodeKey { rung: Doc, file: sds.c, decl: 26, sub: 0, line: 534 } |  |  | 0.574 |
| ns | 6345 |  | 227 | README: the preallocation algorithm and the SDS_MAX_PREALLOC cap | 5.2 |  | 0.565 |
| walker |  | 6484 | 183 | Code::CodeKey { rung: Names, file: sds.c, decl: 0, sub: 3, line: 0 } |  |  | 0.576 |
| ns | 6485 |  | 140 | README: the internals section's `struct sdshdr` — documentation that is stale | 5.3 |  | 0.568 |
| walker |  | 6505 | 21 | Code::CodeKey { rung: Doc, file: sds.c, decl: 31, sub: 0, line: 783 } |  |  | 0.568 |
| walker |  | 6526 | 21 | Code::CodeKey { rung: Doc, file: sds.c, decl: 32, sub: 0, line: 790 } |  |  | 0.568 |
| walker |  | 6737 | 211 | Code::CodeKey { rung: Names, file: sds.c, decl: 0, sub: 4, line: 0 } |  |  | 0.590 |
| ns | 6751 |  | 266 | README: the two disadvantages — reassign the return value, and shared strings | 5.4 |  | 0.598 |
| walker |  | 6757 | 20 | Code::CodeKey { rung: Doc, file: sds.c, decl: 42, sub: 0, line: 1120 } |  |  | 0.598 |
| walker |  | 6783 | 26 | Code::CodeKey { rung: Doc, file: sds.c, decl: 35, sub: 0, line: 885 } |  |  | 0.598 |
| walker |  | 6816 | 33 | Code::CodeKey { rung: Doc, file: sds.c, decl: 37, sub: 0, line: 925 } |  |  | 0.598 |
| walker |  | 6850 | 34 | Code::CodeKey { rung: Doc, file: sds.c, decl: 14, sub: 0, line: 307 } |  |  | 0.598 |
| walker |  | 6884 | 34 | Code::CodeKey { rung: Doc, file: sds.c, decl: 38, sub: 0, line: 932 } |  |  | 0.598 |
| walker |  | 6920 | 36 | Code::CodeKey { rung: Doc, file: sds.c, decl: 5, sub: 0, line: 149 } |  |  | 0.602 |
| walker |  | 6958 | 38 | Code::CodeKey { rung: Doc, file: sds.c, decl: 41, sub: 0, line: 1108 } |  |  | 0.602 |
| ns | 6968 |  | 217 | README basics: `sds` is a `char *`, and the three rules of the minimal program | 5.5 |  | 0.608 |
| walker |  | 6999 | 41 | Code::CodeKey { rung: Doc, file: sds.c, decl: 21, sub: 0, line: 440 } |  |  | 0.608 |
| walker |  | 7041 | 42 | Code::CodeKey { rung: Doc, file: sds.c, decl: 20, sub: 0, line: 427 } |  |  | 0.608 |
| walker |  | 7092 | 51 | Code::CodeKey { rung: Doc, file: sds.c, decl: 25, sub: 0, line: 526 } |  |  | 0.608 |
| walker |  | 7159 | 67 | Code::CodeKey { rung: Doc, file: sds.c, decl: 19, sub: 0, line: 421 } |  |  | 0.608 |
| ns | 7160 |  | 192 | README: why sdstrim/sdsrange return void, and how negative indexes work | 5.6 |  | 0.601 |
| walker |  | 7227 | 68 | Code::CodeKey { rung: Doc, file: sds.c, decl: 18, sub: 0, line: 413 } |  |  | 0.601 |
| walker |  | 7296 | 69 | Code::CodeKey { rung: Doc, file: sds.c, decl: 10, sub: 0, line: 193 } |  |  | 0.601 |
| walker |  | 7369 | 73 | Code::CodeKey { rung: Doc, file: sds.c, decl: 16, sub: 0, line: 380 } |  |  | 0.601 |
| ns | 7382 |  | 222 | README: swapping the allocator, and why sds_malloc/sds_realloc/sds_free are exported | 5.7 |  | 0.594 |
| ns | 7527 |  | 145 | README: the exact escaping rules sdscatrepr applies | 5.8 |  | 0.590 |
| ns | 7716 |  | 189 | README: tokenizer ownership rules and sdssplitargs' quoting behaviour | 5.9 |  | 0.583 |
| ns | 7880 |  | 164 | README: the camelCase warning and how heap checkers see SDS strings | 5.10 |  | 0.579 |
| walker |  | 7983 | 614 | Markdown::Section { file: README.md, section_index: 8, keeps_default_concavity: false } |  |  | 0.579 |
| ns | 8197 |  | 317 | sdsrange() body — negative-index normalisation and clamping | 6.1 | 4.2 | 0.566 |
| ns | 8383 |  | 186 | sdscatlen/sdscat/sdscatsds bodies — the append path | 6.2 | 4.1 | 0.558 |
| walker |  | 8503 | 520 | Markdown::Section { file: README.md, section_index: 9, keeps_default_concavity: false } |  |  | 0.558 |
| ns | 8591 |  | 208 | sdsRemoveFreeSpace() — contract and the shrink decision | 6.3 | 4.1 | 0.550 |
| ns | 8763 |  | 172 | sdstrim() body — how both ends are walked and the survivor moved down | 6.4 | 4.2 | 0.543 |
| walker |  | 8952 | 449 | Markdown::Section { file: README.md, section_index: 10, keeps_default_concavity: false } |  |  | 0.551 |
| ns | 9015 |  | 252 | sdssplitargs() doc comment — REPL-style parsing and its failure mode | 6.5 |  | 0.542 |
| ns | 9256 |  | 241 | The small inspect/reset bodies: sdsupdatelen, sdsclear, sdsAllocSize, sdsAllocPtr | 6.6 | 4.1 | 0.540 |
| ns | 9375 |  | 119 | The test entry points in sds.c: the SDS_TEST_MAIN guard, sdsTest() and main() | 7.1 | 4.2 | 0.534 |
| walker |  | 9553 | 601 | Markdown::Section { file: README.md, section_index: 11, keeps_default_concavity: false } |  |  | 0.540 |
| ns | 9582 |  | 207 | testhelp.h: the complete test_cond / test_report macro pair | 7.2 |  | 0.547 |
| ns | 9665 |  | 83 | A representative excerpt of the sdsTest() body | 7.3 |  | 0.544 |
| ns | 9794 |  | 129 | The complete Changelog (v1.0 and v2.0) | 7.4 |  | 0.539 |
| ns | 9921 |  | 127 | Authorship and licence: README credits, LICENSE header, .gitignore | 7.5 |  | 0.536 |
| walker |  | 9967 | 414 | Markdown::Section { file: README.md, section_index: 12, keeps_default_concavity: false } |  |  | 0.536 |
| walker |  | 9995 | 28 | Markdown::Section { file: README.md, section_index: 13, keeps_default_concavity: false } |  |  | 0.536 |
