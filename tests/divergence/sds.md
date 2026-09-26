Score(3000)=0.791 I=0.885 C=0.706 ns_rows≤3K=21/54 grid(1000/1442/2080/3000/4327/6240/9000)=0.395/0.352/0.452/0.791/0.668/0.611/0.550

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 31 | 31 | Fs::DirListing { dir: . } |  |  | 0.000 |
| ns | 94 |  | 94 | README title + the one-paragraph definition of SDS | 1.1 |  | 0.000 |
| ns | 125 |  | 31 | Complete root directory listing | 1.2 |  | 0.333 |
| walker |  | 140 | 109 | Markdown::ReadmeHeadline { file: README.md } |  |  | 0.352 |
| walker |  | 248 | 108 | Plaintext::Whole { file: Makefile } |  |  | 0.373 |
| ns | 251 |  | 126 | The header-before-the-pointer design + ASCII layout diagram | 1.3 |  | 0.285 |
| ns | 365 |  | 114 | sds.h preamble: include guard, SDS_MAX_PREALLOC, SDS_NOINIT, `typedef char *sds` | 1.4 |  | 0.229 |
| ns | 533 |  | 168 | README headings, part 1: manual chapters through Error handling | 1.5 |  | 0.182 |
| ns | 639 |  | 106 | README headings, part 2: internals chapter + back matter (696-911) | 1.6 | 1.5 | 0.165 |
| walker |  | 647 | 399 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.409 |
| ns | 756 |  | 117 | The complete Makefile (build + run the in-file unit tests) | 1.7 |  | 0.415 |
| walker |  | 769 | 122 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.415 |
| ns | 850 |  | 94 | README: embedding SDS into a project (which three files to copy) | 1.8 |  | 0.380 |
| walker |  | 975 | 206 | Code::CodeKey { rung: Names, file: sds.h, decl: 0, sub: 0, line: 0 } |  |  | 0.395 |
| walker |  | 1012 | 37 | Code::CodeKey { rung: Decl, file: sds.h, decl: 4, sub: 0, line: 47 } |  |  | 0.396 |
| ns | 1047 |  | 197 | sds.h prototypes: creation, destruction, growth, concat, copy (218-229) | 2.1 |  | 0.363 |
| walker |  | 1077 | 65 | Code::CodeKey { rung: Decl, file: sds.h, decl: 5, sub: 0, line: 51 } |  |  | 0.367 |
| walker |  | 1142 | 65 | Code::CodeKey { rung: Decl, file: sds.h, decl: 6, sub: 0, line: 57 } |  |  | 0.370 |
| ns | 1192 |  | 145 | All six `static inline` accessors in sds.h (signature lines only) | 2.2 |  | 0.354 |
| walker |  | 1207 | 65 | Code::CodeKey { rung: Decl, file: sds.h, decl: 7, sub: 0, line: 63 } |  |  | 0.358 |
| walker |  | 1272 | 65 | Code::CodeKey { rung: Decl, file: sds.h, decl: 8, sub: 0, line: 69 } |  |  | 0.364 |
| ns | 1324 |  | 132 | sds.h prototypes: the printf-family, incl. the __GNUC__ format attribute (230-238) | 2.3 | 2.1 | 0.344 |
| walker |  | 1445 | 173 | Code::CodeKey { rung: Names, file: sds.h, decl: 0, sub: 1, line: 0 } |  |  | 0.355 |
| ns | 1501 |  | 177 | sds.h prototypes: trim/range/inspect/tokenize (239-248) | 2.4 | 2.3 | 0.336 |
| ns | 1621 |  | 120 | sds.h prototypes: repr/splitargs/mapchars/join (249-253) | 2.5 | 2.4 | 0.327 |
| walker |  | 1623 | 178 | Code::CodeKey { rung: Names, file: sds.h, decl: 0, sub: 2, line: 0 } |  |  | 0.365 |
| ns | 1716 |  | 95 | sds.h low-level API block (255-260) | 2.6 | 2.5 | 0.354 |
| walker |  | 1789 | 166 | Code::CodeKey { rung: Names, file: sds.h, decl: 0, sub: 3, line: 0 } |  |  | 0.406 |
| walker |  | 1796 | 7 | Code::CodeKey { rung: Decl, file: sds.h, decl: 38, sub: 0, line: 234 } |  |  | 0.409 |
| walker |  | 1812 | 16 | Code::CodeKey { rung: Decl, file: sds.h, decl: 37, sub: 0, line: 231 } |  |  | 0.417 |
| ns | 1864 |  | 148 | sds.h allocator exports + the REDIS_TEST-guarded sdsTest declaration (262-272) | 2.7 | 2.6 | 0.398 |
| walker |  | 1970 | 158 | Code::CodeKey { rung: Names, file: sds.h, decl: 0, sub: 4, line: 0 } |  |  | 0.431 |
| ns | 2045 |  | 181 | sdshdr5 and sdshdr8 struct definitions, with the 'sdshdr5 is never used' note | 3.1 |  | 0.431 |
| walker |  | 2130 | 160 | Code::CodeKey { rung: Names, file: sds.h, decl: 0, sub: 5, line: 0 } |  |  | 0.467 |
| walker |  | 2275 | 145 | Code::CodeKey { rung: Names, file: sds.h, decl: 0, sub: 6, line: 0 } |  |  | 0.484 |
| walker |  | 2290 | 15 | Code::CodeKey { rung: Decl, file: sds.h, decl: 63, sub: 0, line: 270 } |  |  | 0.487 |
| ns | 2294 |  | 249 | sdshdr16 / sdshdr32 / sdshdr64 struct definitions | 3.2 | 3.1 | 0.497 |
| walker |  | 2302 | 12 | Code::CodeKey { rung: Doc, file: sds.h, decl: 55, sub: 0, line: 256 } |  |  | 0.506 |
| walker |  | 2322 | 20 | Code::CodeKey { rung: Doc, file: sds.h, decl: 23, sub: 0, line: 180 } |  |  | 0.512 |
| ns | 2468 |  | 174 | SDS_TYPE_* constants and the SDS_HDR / SDS_HDR_VAR / SDS_TYPE_5_LEN macros | 3.3 |  | 0.516 |
| walker |  | 2540 | 218 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.848 |
| walker |  | 2581 | 41 | Code::CodeKey { rung: Doc, file: sds.h, decl: 4, sub: 0, line: 47 } |  |  | 0.865 |
| ns | 2636 |  | 168 | sdslen() body — how a length is decoded from the flags byte | 3.4 | 2.2 | 0.827 |
| walker |  | 2647 | 66 | Code::CodeKey { rung: Doc, file: sds.h, decl: 60, sub: 0, line: 266 } |  |  | 0.855 |
| ns | 2733 |  | 97 | sdsavail() body — free space is `alloc - len`, and always 0 for type 5 | 3.5 | 2.2 | 0.831 |
| ns | 2942 |  | 209 | The type-5 write paths of sdssetlen() and sdsinclen(), rest elided | 3.6 | 2.2 | 0.791 |
| walker |  | 3072 | 425 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.791 |
| ns | 3080 |  | 138 | sdsalloc() body and the type-5 no-op in sdssetalloc() | 3.7 | 2.2 | 0.766 |
| ns | 3337 |  | 257 | Roster of every function defined in sds.c, part 1 (lines 44-440), names only | 4.1 |  | 0.730 |
| walker |  | 3490 | 418 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.732 |
| ns | 3650 |  | 313 | Roster of every function defined in sds.c, part 2 (lines 451-1325), names only | 4.2 | 4.1 | 0.695 |
| ns | 3791 |  | 141 | sds.c includes + SDS_NOINIT definition, and the sdsalloc.h allocator macros | 4.3 |  | 0.677 |
| walker |  | 3895 | 405 | Markdown::Section { file: README.md, section_index: 4, keeps_default_concavity: false } |  |  | 0.677 |
| ns | 4007 |  | 216 | sdsHdrSize() and sdsReqType() — the type-selection policy | 4.4 | 4.1 | 0.652 |
| walker |  | 4099 | 204 | Code::CodeKey { rung: Names, file: sds.c, decl: 0, sub: 0, line: 0 } |  |  | 0.664 |
| walker |  | 4157 | 58 | Code::CodeKey { rung: Names, file: testhelp.h, decl: 0, sub: 0, line: 0 } |  |  | 0.665 |
| walker |  | 4190 | 33 | Code::CodeKey { rung: Names, file: sdsalloc.h, decl: 0, sub: 0, line: 0 } |  |  | 0.668 |
| walker |  | 4202 | 12 | Code::CodeKey { rung: Doc, file: sds.c, decl: 7, sub: 0, line: 160 } |  |  | 0.668 |
| ns | 4216 |  | 209 | sdsnewlen() doc comment — NULL vs SDS_NOINIT init, and the always-null-terminated guarantee | 4.5 |  | 0.653 |
| walker |  | 4222 | 20 | Code::CodeKey { rung: Doc, file: sds.c, decl: 6, sub: 0, line: 154 } |  |  | 0.653 |
| walker |  | 4411 | 189 | Code::CodeKey { rung: Names, file: sds.c, decl: 0, sub: 1, line: 0 } |  |  | 0.681 |
| ns | 4537 |  | 321 | sdsnewlen() body — single allocation, header write, type-5-to-8 upgrade | 4.6 | 4.1 | 0.654 |
| walker |  | 4579 | 168 | Code::CodeKey { rung: Names, file: sds.c, decl: 0, sub: 2, line: 0 } |  |  | 0.662 |
| walker |  | 4600 | 21 | Code::CodeKey { rung: Doc, file: sds.c, decl: 24, sub: 0, line: 494 } |  |  | 0.662 |
| walker |  | 4783 | 183 | Code::CodeKey { rung: Names, file: sds.c, decl: 0, sub: 3, line: 0 } |  |  | 0.674 |
| walker |  | 4804 | 21 | Code::CodeKey { rung: Doc, file: sds.c, decl: 31, sub: 0, line: 783 } |  |  | 0.674 |
| walker |  | 4825 | 21 | Code::CodeKey { rung: Doc, file: sds.c, decl: 32, sub: 0, line: 790 } |  |  | 0.674 |
| walker |  | 4847 | 22 | Code::CodeKey { rung: Doc, file: sds.c, decl: 26, sub: 0, line: 534 } |  |  | 0.674 |
| ns | 4883 |  | 346 | sdsMakeRoomFor(): contract comment and the growth policy (198-222) | 4.7 | 4.1 | 0.648 |
| walker |  | 5099 | 252 | Code::CodeKey { rung: Names, file: sds.c, decl: 0, sub: 4, line: 0 } |  |  | 0.680 |
| walker |  | 5109 | 10 | Code::CodeKey { rung: Decl, file: sds.c, decl: 47, sub: 0, line: 1146 } |  |  | 0.680 |
| walker |  | 5122 | 13 | Code::CodeKey { rung: Decl, file: sds.c, decl: 46, sub: 0, line: 1140 } |  |  | 0.680 |
| walker |  | 5139 | 17 | Code::CodeKey { rung: Decl, file: sds.c, decl: 48, sub: 0, line: 1324 } |  |  | 0.680 |
| walker |  | 5159 | 20 | Code::CodeKey { rung: Doc, file: sds.c, decl: 42, sub: 0, line: 1120 } |  |  | 0.680 |
| walker |  | 5182 | 23 | Code::CodeKey { rung: Doc, file: sds.c, decl: 8, sub: 0, line: 165 } |  |  | 0.680 |
| walker |  | 5208 | 26 | Code::CodeKey { rung: Doc, file: sds.c, decl: 35, sub: 0, line: 885 } |  |  | 0.680 |
| walker |  | 5241 | 33 | Code::CodeKey { rung: Doc, file: sds.c, decl: 37, sub: 0, line: 925 } |  |  | 0.680 |
| ns | 5253 |  | 370 | sdsMakeRoomFor(): the realloc-vs-move-header branch (224-248) | 4.8 | 4.7 | 0.656 |
| walker |  | 5275 | 34 | Code::CodeKey { rung: Doc, file: sds.c, decl: 14, sub: 0, line: 307 } |  |  | 0.656 |
| walker |  | 5309 | 34 | Code::CodeKey { rung: Doc, file: sds.c, decl: 38, sub: 0, line: 932 } |  |  | 0.656 |
| walker |  | 5345 | 36 | Code::CodeKey { rung: Doc, file: sds.c, decl: 5, sub: 0, line: 149 } |  |  | 0.657 |
| walker |  | 5404 | 59 | Code::CodeKey { rung: Body, file: testhelp.h, decl: 3, sub: 0, line: 44 } |  |  | 0.657 |
| walker |  | 5442 | 38 | Code::CodeKey { rung: Doc, file: sds.c, decl: 41, sub: 0, line: 1108 } |  |  | 0.657 |
| walker |  | 5483 | 41 | Code::CodeKey { rung: Doc, file: sds.c, decl: 21, sub: 0, line: 440 } |  |  | 0.657 |
| ns | 5486 |  | 233 | sdsempty/sdsnew/sdsdup/sdsfree bodies, with sdsfree's NULL contract | 4.9 | 4.1 | 0.646 |
| walker |  | 5525 | 42 | Code::CodeKey { rung: Doc, file: sds.c, decl: 20, sub: 0, line: 427 } |  |  | 0.646 |
| walker |  | 5576 | 51 | Code::CodeKey { rung: Doc, file: sds.c, decl: 25, sub: 0, line: 526 } |  |  | 0.646 |
| ns | 5808 |  | 322 | sdsIncrLen() doc comment — the zero-copy read-into-the-buffer pattern | 4.10 |  | 0.627 |
| ns | 6023 |  | 215 | sdscatfmt() doc comment — the supported format specifiers | 4.11 |  | 0.615 |
| walker |  | 6103 | 527 | Markdown::Section { file: README.md, section_index: 5, keeps_default_concavity: false } |  |  | 0.616 |
| ns | 6118 |  | 95 | README: the error-handling contract (NULL on out of memory) | 5.1 |  | 0.611 |
| ns | 6345 |  | 227 | README: the preallocation algorithm and the SDS_MAX_PREALLOC cap | 5.2 |  | 0.602 |
| ns | 6485 |  | 140 | README: the internals section's `struct sdshdr` — documentation that is stale | 5.3 |  | 0.594 |
| walker |  | 6548 | 445 | Markdown::Section { file: README.md, section_index: 6, keeps_default_concavity: false } |  |  | 0.594 |
| ns | 6751 |  | 266 | README: the two disadvantages — reassign the return value, and shared strings | 5.4 |  | 0.602 |
| ns | 6968 |  | 217 | README basics: `sds` is a `char *`, and the three rules of the minimal program | 5.5 |  | 0.607 |
| walker |  | 7052 | 504 | Markdown::Section { file: README.md, section_index: 7, keeps_default_concavity: false } |  |  | 0.607 |
| walker |  | 7119 | 67 | Code::CodeKey { rung: Doc, file: sds.c, decl: 19, sub: 0, line: 421 } |  |  | 0.607 |
| ns | 7160 |  | 192 | README: why sdstrim/sdsrange return void, and how negative indexes work | 5.6 |  | 0.600 |
| walker |  | 7187 | 68 | Code::CodeKey { rung: Doc, file: sds.c, decl: 18, sub: 0, line: 413 } |  |  | 0.600 |
| walker |  | 7256 | 69 | Code::CodeKey { rung: Doc, file: sds.c, decl: 10, sub: 0, line: 193 } |  |  | 0.601 |
| ns | 7382 |  | 222 | README: swapping the allocator, and why sds_malloc/sds_realloc/sds_free are exported | 5.7 |  | 0.593 |
| ns | 7527 |  | 145 | README: the exact escaping rules sdscatrepr applies | 5.8 |  | 0.590 |
| ns | 7716 |  | 189 | README: tokenizer ownership rules and sdssplitargs' quoting behaviour | 5.9 |  | 0.583 |
| walker |  | 7870 | 614 | Markdown::Section { file: README.md, section_index: 8, keeps_default_concavity: false } |  |  | 0.583 |
| ns | 7880 |  | 164 | README: the camelCase warning and how heap checkers see SDS strings | 5.10 |  | 0.579 |
| ns | 8197 |  | 317 | sdsrange() body — negative-index normalisation and clamping | 6.1 | 4.2 | 0.565 |
| ns | 8383 |  | 186 | sdscatlen/sdscat/sdscatsds bodies — the append path | 6.2 | 4.1 | 0.558 |
| walker |  | 8390 | 520 | Markdown::Section { file: README.md, section_index: 9, keeps_default_concavity: false } |  |  | 0.558 |
| ns | 8591 |  | 208 | sdsRemoveFreeSpace() — contract and the shrink decision | 6.3 | 4.1 | 0.550 |
| ns | 8763 |  | 172 | sdstrim() body — how both ends are walked and the survivor moved down | 6.4 | 4.2 | 0.543 |
| walker |  | 8839 | 449 | Markdown::Section { file: README.md, section_index: 10, keeps_default_concavity: false } |  |  | 0.550 |
| ns | 9015 |  | 252 | sdssplitargs() doc comment — REPL-style parsing and its failure mode | 6.5 |  | 0.542 |
| ns | 9256 |  | 241 | The small inspect/reset bodies: sdsupdatelen, sdsclear, sdsAllocSize, sdsAllocPtr | 6.6 | 4.1 | 0.539 |
| ns | 9375 |  | 119 | The test entry points in sds.c: the SDS_TEST_MAIN guard, sdsTest() and main() | 7.1 | 4.2 | 0.536 |
| walker |  | 9440 | 601 | Markdown::Section { file: README.md, section_index: 11, keeps_default_concavity: false } |  |  | 0.542 |
| ns | 9582 |  | 207 | testhelp.h: the complete test_cond / test_report macro pair | 7.2 |  | 0.540 |
| ns | 9665 |  | 83 | A representative excerpt of the sdsTest() body | 7.3 |  | 0.537 |
| ns | 9794 |  | 129 | The complete Changelog (v1.0 and v2.0) | 7.4 |  | 0.532 |
| walker |  | 9854 | 414 | Markdown::Section { file: README.md, section_index: 12, keeps_default_concavity: false } |  |  | 0.532 |
| ns | 9921 |  | 127 | Authorship and licence: README credits, LICENSE header, .gitignore | 7.5 |  | 0.529 |
| walker |  | 9991 | 137 | Markdown::Section { file: README.md, section_index: 13, keeps_default_concavity: false } |  |  | 0.529 |
