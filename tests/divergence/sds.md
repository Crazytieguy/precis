Score(3000)=0.799 I=0.890 C=0.717 ns_rows≤3K=21/54 grid(1000/1442/2080/3000/4327/6240/9000)=0.405/0.360/0.458/0.799/0.671/0.617/0.555

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
| ns | 1716 |  | 95 | sds.h low-level API block (255-260) | 2.6 | 2.5 | 0.361 |
| walker |  | 1798 | 166 | Code::CodeKey { rung: Names, file: sds.h, decl: 0, sub: 3, line: 0 } |  |  | 0.413 |
| walker |  | 1805 | 7 | Code::CodeKey { rung: Decl, file: sds.h, decl: 38, sub: 0, line: 234 } |  |  | 0.416 |
| walker |  | 1821 | 16 | Code::CodeKey { rung: Decl, file: sds.h, decl: 37, sub: 0, line: 231 } |  |  | 0.424 |
| ns | 1864 |  | 148 | sds.h allocator exports + the REDIS_TEST-guarded sdsTest declaration (262-272) | 2.7 | 2.6 | 0.405 |
| walker |  | 1979 | 158 | Code::CodeKey { rung: Names, file: sds.h, decl: 0, sub: 4, line: 0 } |  |  | 0.438 |
| ns | 2045 |  | 181 | sdshdr5 and sdshdr8 struct definitions, with the 'sdshdr5 is never used' note | 3.1 |  | 0.437 |
| walker |  | 2139 | 160 | Code::CodeKey { rung: Names, file: sds.h, decl: 0, sub: 5, line: 0 } |  |  | 0.473 |
| walker |  | 2159 | 20 | Code::CodeKey { rung: Doc, file: sds.h, decl: 23, sub: 0, line: 180 } |  |  | 0.479 |
| ns | 2294 |  | 249 | sdshdr16 / sdshdr32 / sdshdr64 struct definitions | 3.2 | 3.1 | 0.490 |
| walker |  | 2317 | 158 | Code::CodeKey { rung: Names, file: sds.h, decl: 0, sub: 6, line: 0 } |  |  | 0.508 |
| walker |  | 2324 | 7 | Code::CodeKey { rung: Decl, file: sds.h, decl: 63, sub: 0, line: 270 } |  |  | 0.509 |
| walker |  | 2336 | 12 | Code::CodeKey { rung: Doc, file: sds.h, decl: 55, sub: 0, line: 256 } |  |  | 0.519 |
| ns | 2468 |  | 174 | SDS_TYPE_* constants and the SDS_HDR / SDS_HDR_VAR / SDS_TYPE_5_LEN macros | 3.3 |  | 0.522 |
| walker |  | 2554 | 218 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.855 |
| walker |  | 2595 | 41 | Code::CodeKey { rung: Doc, file: sds.h, decl: 4, sub: 0, line: 47 } |  |  | 0.872 |
| ns | 2636 |  | 168 | sdslen() body — how a length is decoded from the flags byte | 3.4 | 2.2 | 0.834 |
| walker |  | 2661 | 66 | Code::CodeKey { rung: Doc, file: sds.h, decl: 60, sub: 0, line: 266 } |  |  | 0.864 |
| ns | 2733 |  | 97 | sdsavail() body — free space is `alloc - len`, and always 0 for type 5 | 3.5 | 2.2 | 0.840 |
| ns | 2942 |  | 209 | The type-5 write paths of sdssetlen() and sdsinclen(), rest elided | 3.6 | 2.2 | 0.799 |
| ns | 3080 |  | 138 | sdsalloc() body and the type-5 no-op in sdssetalloc() | 3.7 | 2.2 | 0.774 |
| walker |  | 3086 | 425 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.774 |
| ns | 3337 |  | 257 | Roster of every function defined in sds.c, part 1 (lines 44-440), names only | 4.1 |  | 0.738 |
| walker |  | 3504 | 418 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.739 |
| ns | 3650 |  | 313 | Roster of every function defined in sds.c, part 2 (lines 451-1325), names only | 4.2 | 4.1 | 0.702 |
| ns | 3791 |  | 141 | sds.c includes + SDS_NOINIT definition, and the sdsalloc.h allocator macros | 4.3 |  | 0.684 |
| walker |  | 3909 | 405 | Markdown::Section { file: README.md, section_index: 4, keeps_default_concavity: false } |  |  | 0.684 |
| ns | 4007 |  | 216 | sdsHdrSize() and sdsReqType() — the type-selection policy | 4.4 | 4.1 | 0.658 |
| walker |  | 4113 | 204 | Code::CodeKey { rung: Names, file: sds.c, decl: 0, sub: 0, line: 0 } |  |  | 0.671 |
| walker |  | 4171 | 58 | Code::CodeKey { rung: Names, file: testhelp.h, decl: 0, sub: 0, line: 0 } |  |  | 0.671 |
| walker |  | 4204 | 33 | Code::CodeKey { rung: Names, file: sdsalloc.h, decl: 0, sub: 0, line: 0 } |  |  | 0.675 |
| walker |  | 4216 | 12 | Code::CodeKey { rung: Doc, file: sds.c, decl: 7, sub: 0, line: 160 } |  |  | 0.659 |
| ns | 4216 |  | 209 | sdsnewlen() doc comment — NULL vs SDS_NOINIT init, and the always-null-terminated guarantee | 4.5 |  | 0.659 |
| walker |  | 4236 | 20 | Code::CodeKey { rung: Doc, file: sds.c, decl: 6, sub: 0, line: 154 } |  |  | 0.660 |
| walker |  | 4425 | 189 | Code::CodeKey { rung: Names, file: sds.c, decl: 0, sub: 1, line: 0 } |  |  | 0.687 |
| ns | 4537 |  | 321 | sdsnewlen() body — single allocation, header write, type-5-to-8 upgrade | 4.6 | 4.1 | 0.660 |
| walker |  | 4593 | 168 | Code::CodeKey { rung: Names, file: sds.c, decl: 0, sub: 2, line: 0 } |  |  | 0.668 |
| walker |  | 4776 | 183 | Code::CodeKey { rung: Names, file: sds.c, decl: 0, sub: 3, line: 0 } |  |  | 0.681 |
| walker |  | 4797 | 21 | Code::CodeKey { rung: Doc, file: sds.c, decl: 24, sub: 0, line: 494 } |  |  | 0.681 |
| walker |  | 4818 | 21 | Code::CodeKey { rung: Doc, file: sds.c, decl: 31, sub: 0, line: 783 } |  |  | 0.681 |
| walker |  | 4839 | 21 | Code::CodeKey { rung: Doc, file: sds.c, decl: 32, sub: 0, line: 790 } |  |  | 0.681 |
| walker |  | 4861 | 22 | Code::CodeKey { rung: Doc, file: sds.c, decl: 26, sub: 0, line: 534 } |  |  | 0.681 |
| ns | 4883 |  | 346 | sdsMakeRoomFor(): contract comment and the growth policy (198-222) | 4.7 | 4.1 | 0.654 |
| walker |  | 4884 | 23 | Code::CodeKey { rung: Doc, file: sds.c, decl: 8, sub: 0, line: 165 } |  |  | 0.654 |
| walker |  | 5166 | 282 | Code::CodeKey { rung: Names, file: sds.c, decl: 0, sub: 4, line: 0 } |  |  | 0.686 |
| walker |  | 5174 | 8 | Code::CodeKey { rung: Decl, file: sds.c, decl: 47, sub: 0, line: 1146 } |  |  | 0.686 |
| walker |  | 5182 | 8 | Code::CodeKey { rung: Decl, file: sds.c, decl: 48, sub: 0, line: 1324 } |  |  | 0.686 |
| walker |  | 5202 | 20 | Code::CodeKey { rung: Doc, file: sds.c, decl: 42, sub: 0, line: 1120 } |  |  | 0.686 |
| walker |  | 5228 | 26 | Code::CodeKey { rung: Doc, file: sds.c, decl: 35, sub: 0, line: 885 } |  |  | 0.686 |
| ns | 5253 |  | 370 | sdsMakeRoomFor(): the realloc-vs-move-header branch (224-248) | 4.8 | 4.7 | 0.662 |
| walker |  | 5261 | 33 | Code::CodeKey { rung: Doc, file: sds.c, decl: 37, sub: 0, line: 925 } |  |  | 0.662 |
| walker |  | 5295 | 34 | Code::CodeKey { rung: Doc, file: sds.c, decl: 14, sub: 0, line: 307 } |  |  | 0.662 |
| walker |  | 5329 | 34 | Code::CodeKey { rung: Doc, file: sds.c, decl: 38, sub: 0, line: 932 } |  |  | 0.662 |
| walker |  | 5365 | 36 | Code::CodeKey { rung: Doc, file: sds.c, decl: 5, sub: 0, line: 149 } |  |  | 0.662 |
| walker |  | 5424 | 59 | Code::CodeKey { rung: Body, file: testhelp.h, decl: 3, sub: 0, line: 44 } |  |  | 0.662 |
| walker |  | 5462 | 38 | Code::CodeKey { rung: Doc, file: sds.c, decl: 41, sub: 0, line: 1108 } |  |  | 0.662 |
| ns | 5486 |  | 233 | sdsempty/sdsnew/sdsdup/sdsfree bodies, with sdsfree's NULL contract | 4.9 | 4.1 | 0.651 |
| walker |  | 5503 | 41 | Code::CodeKey { rung: Doc, file: sds.c, decl: 21, sub: 0, line: 440 } |  |  | 0.651 |
| walker |  | 5545 | 42 | Code::CodeKey { rung: Doc, file: sds.c, decl: 20, sub: 0, line: 427 } |  |  | 0.651 |
| walker |  | 5596 | 51 | Code::CodeKey { rung: Doc, file: sds.c, decl: 25, sub: 0, line: 526 } |  |  | 0.651 |
| ns | 5808 |  | 322 | sdsIncrLen() doc comment — the zero-copy read-into-the-buffer pattern | 4.10 |  | 0.632 |
| ns | 6023 |  | 215 | sdscatfmt() doc comment — the supported format specifiers | 4.11 |  | 0.620 |
| ns | 6118 |  | 95 | README: the error-handling contract (NULL on out of memory) | 5.1 |  | 0.616 |
| walker |  | 6123 | 527 | Markdown::Section { file: README.md, section_index: 5, keeps_default_concavity: false } |  |  | 0.617 |
| ns | 6345 |  | 227 | README: the preallocation algorithm and the SDS_MAX_PREALLOC cap | 5.2 |  | 0.607 |
| ns | 6485 |  | 140 | README: the internals section's `struct sdshdr` — documentation that is stale | 5.3 |  | 0.599 |
| walker |  | 6568 | 445 | Markdown::Section { file: README.md, section_index: 6, keeps_default_concavity: false } |  |  | 0.599 |
| ns | 6751 |  | 266 | README: the two disadvantages — reassign the return value, and shared strings | 5.4 |  | 0.607 |
| ns | 6968 |  | 217 | README basics: `sds` is a `char *`, and the three rules of the minimal program | 5.5 |  | 0.612 |
| walker |  | 7072 | 504 | Markdown::Section { file: README.md, section_index: 7, keeps_default_concavity: false } |  |  | 0.612 |
| walker |  | 7139 | 67 | Code::CodeKey { rung: Doc, file: sds.c, decl: 19, sub: 0, line: 421 } |  |  | 0.612 |
| ns | 7160 |  | 192 | README: why sdstrim/sdsrange return void, and how negative indexes work | 5.6 |  | 0.605 |
| walker |  | 7207 | 68 | Code::CodeKey { rung: Doc, file: sds.c, decl: 18, sub: 0, line: 413 } |  |  | 0.605 |
| walker |  | 7276 | 69 | Code::CodeKey { rung: Doc, file: sds.c, decl: 10, sub: 0, line: 193 } |  |  | 0.605 |
| ns | 7382 |  | 222 | README: swapping the allocator, and why sds_malloc/sds_realloc/sds_free are exported | 5.7 |  | 0.598 |
| ns | 7527 |  | 145 | README: the exact escaping rules sdscatrepr applies | 5.8 |  | 0.594 |
| ns | 7716 |  | 189 | README: tokenizer ownership rules and sdssplitargs' quoting behaviour | 5.9 |  | 0.588 |
| ns | 7880 |  | 164 | README: the camelCase warning and how heap checkers see SDS strings | 5.10 |  | 0.584 |
| walker |  | 7890 | 614 | Markdown::Section { file: README.md, section_index: 8, keeps_default_concavity: false } |  |  | 0.584 |
| ns | 8197 |  | 317 | sdsrange() body — negative-index normalisation and clamping | 6.1 | 4.2 | 0.570 |
| ns | 8383 |  | 186 | sdscatlen/sdscat/sdscatsds bodies — the append path | 6.2 | 4.1 | 0.562 |
| walker |  | 8410 | 520 | Markdown::Section { file: README.md, section_index: 9, keeps_default_concavity: false } |  |  | 0.562 |
| ns | 8591 |  | 208 | sdsRemoveFreeSpace() — contract and the shrink decision | 6.3 | 4.1 | 0.554 |
| ns | 8763 |  | 172 | sdstrim() body — how both ends are walked and the survivor moved down | 6.4 | 4.2 | 0.547 |
| walker |  | 8859 | 449 | Markdown::Section { file: README.md, section_index: 10, keeps_default_concavity: false } |  |  | 0.555 |
| ns | 9015 |  | 252 | sdssplitargs() doc comment — REPL-style parsing and its failure mode | 6.5 |  | 0.546 |
| ns | 9256 |  | 241 | The small inspect/reset bodies: sdsupdatelen, sdsclear, sdsAllocSize, sdsAllocPtr | 6.6 | 4.1 | 0.544 |
| ns | 9375 |  | 119 | The test entry points in sds.c: the SDS_TEST_MAIN guard, sdsTest() and main() | 7.1 | 4.2 | 0.540 |
| walker |  | 9460 | 601 | Markdown::Section { file: README.md, section_index: 11, keeps_default_concavity: false } |  |  | 0.546 |
| ns | 9582 |  | 207 | testhelp.h: the complete test_cond / test_report macro pair | 7.2 |  | 0.544 |
| ns | 9665 |  | 83 | A representative excerpt of the sdsTest() body | 7.3 |  | 0.541 |
| ns | 9794 |  | 129 | The complete Changelog (v1.0 and v2.0) | 7.4 |  | 0.536 |
| walker |  | 9874 | 414 | Markdown::Section { file: README.md, section_index: 12, keeps_default_concavity: false } |  |  | 0.536 |
| ns | 9921 |  | 127 | Authorship and licence: README credits, LICENSE header, .gitignore | 7.5 |  | 0.533 |
| walker |  | 9992 | 118 | Markdown::Section { file: README.md, section_index: 13, keeps_default_concavity: false } |  |  | 0.533 |
