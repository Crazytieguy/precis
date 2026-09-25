Score(3000)=0.797 I=0.892 C=0.713 ns_rows≤3K=21/54 grid(1000/1442/2080/3000/4327/6240/9000)=0.222/0.194/0.657/0.797/0.692/0.617/0.603

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 31 | 31 | Fs::DirListing { dir: . } |  |  | 0.000 |
| ns | 94 |  | 94 | README title + the one-paragraph definition of SDS | 1.1 |  | 0.000 |
| ns | 125 |  | 31 | Complete root directory listing | 1.2 |  | 0.333 |
| walker |  | 148 | 117 | Plaintext::Whole { file: Makefile } |  |  | 0.360 |
| ns | 251 |  | 126 | The header-before-the-pointer design + ASCII layout diagram | 1.3 |  | 0.275 |
| walker |  | 257 | 109 | Markdown::ReadmeHeadline { file: README.md } |  |  | 0.288 |
| walker |  | 290 | 33 | Code::CodeKey { rung: Names, file: sdsalloc.h, decl: 0, sub: 0, line: 0 } |  |  | 0.289 |
| walker |  | 348 | 58 | Code::CodeKey { rung: Names, file: testhelp.h, decl: 0, sub: 0, line: 0 } |  |  | 0.289 |
| ns | 365 |  | 114 | sds.h preamble: include guard, SDS_MAX_PREALLOC, SDS_NOINIT, `typedef char *sds` | 1.4 |  | 0.232 |
| walker |  | 407 | 59 | Code::CodeKey { rung: Decl, file: testhelp.h, decl: 3, sub: 0, line: 44 } |  |  | 0.233 |
| walker |  | 506 | 99 | Code::CodeKey { rung: Decl, file: testhelp.h, decl: 4, sub: 0, line: 48 } |  |  | 0.234 |
| ns | 533 |  | 168 | README headings, part 1: manual chapters through Error handling | 1.5 |  | 0.186 |
| ns | 639 |  | 106 | README headings, part 2: internals chapter + back matter (696-911) | 1.6 | 1.5 | 0.169 |
| ns | 756 |  | 117 | The complete Makefile (build + run the in-file unit tests) | 1.7 |  | 0.216 |
| walker |  | 770 | 264 | Code::CodeKey { rung: Names, file: sds.h, decl: 0, sub: 0, line: 0 } |  |  | 0.234 |
| walker |  | 807 | 37 | Code::CodeKey { rung: Decl, file: sds.h, decl: 4, sub: 0, line: 47 } |  |  | 0.235 |
| ns | 850 |  | 94 | README: embedding SDS into a project (which three files to copy) | 1.8 |  | 0.216 |
| walker |  | 872 | 65 | Code::CodeKey { rung: Decl, file: sds.h, decl: 5, sub: 0, line: 51 } |  |  | 0.220 |
| walker |  | 937 | 65 | Code::CodeKey { rung: Decl, file: sds.h, decl: 6, sub: 0, line: 57 } |  |  | 0.222 |
| walker |  | 1002 | 65 | Code::CodeKey { rung: Decl, file: sds.h, decl: 7, sub: 0, line: 63 } |  |  | 0.225 |
| ns | 1047 |  | 197 | sds.h prototypes: creation, destruction, growth, concat, copy (218-229) | 2.1 |  | 0.206 |
| walker |  | 1067 | 65 | Code::CodeKey { rung: Decl, file: sds.h, decl: 8, sub: 0, line: 69 } |  |  | 0.211 |
| walker |  | 1108 | 41 | Code::CodeKey { rung: Doc, file: sds.h, decl: 4, sub: 0, line: 47 } |  |  | 0.214 |
| ns | 1192 |  | 145 | All six `static inline` accessors in sds.h (signature lines only) | 2.2 |  | 0.205 |
| ns | 1324 |  | 132 | sds.h prototypes: the printf-family, incl. the __GNUC__ format attribute (230-238) | 2.3 | 2.1 | 0.194 |
| ns | 1501 |  | 177 | sds.h prototypes: trim/range/inspect/tokenize (239-248) | 2.4 | 2.3 | 0.183 |
| walker |  | 1507 | 399 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.341 |
| ns | 1621 |  | 120 | sds.h prototypes: repr/splitargs/mapchars/join (249-253) | 2.5 | 2.4 | 0.331 |
| walker |  | 1629 | 122 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.331 |
| ns | 1716 |  | 95 | sds.h low-level API block (255-260) | 2.6 | 2.5 | 0.321 |
| walker |  | 1847 | 218 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: true } |  |  | 0.627 |
| ns | 1864 |  | 148 | sds.h allocator exports + the REDIS_TEST-guarded sdsTest declaration (262-272) | 2.7 | 2.6 | 0.599 |
| ns | 2045 |  | 181 | sdshdr5 and sdshdr8 struct definitions, with the 'sdshdr5 is never used' note | 3.1 |  | 0.628 |
| walker |  | 2062 | 215 | Code::CodeKey { rung: Names, file: sds.h, decl: 0, sub: 1, line: 0 } |  |  | 0.657 |
| walker |  | 2082 | 20 | Code::CodeKey { rung: Doc, file: sds.h, decl: 23, sub: 0, line: 180 } |  |  | 0.665 |
| walker |  | 2289 | 207 | Code::CodeKey { rung: Names, file: sds.h, decl: 0, sub: 2, line: 0 } |  |  | 0.728 |
| ns | 2294 |  | 249 | sdshdr16 / sdshdr32 / sdshdr64 struct definitions | 3.2 | 3.1 | 0.746 |
| walker |  | 2314 | 25 | Code::CodeKey { rung: Decl, file: sds.h, decl: 37, sub: 0, line: 231 } |  |  | 0.752 |
| ns | 2468 |  | 174 | SDS_TYPE_* constants and the SDS_HDR / SDS_HDR_VAR / SDS_TYPE_5_LEN macros | 3.3 |  | 0.760 |
| walker |  | 2510 | 196 | Code::CodeKey { rung: Names, file: sds.h, decl: 0, sub: 3, line: 0 } |  |  | 0.800 |
| walker |  | 2520 | 10 | Code::CodeKey { rung: Decl, file: sds.h, decl: 38, sub: 0, line: 234 } |  |  | 0.808 |
| ns | 2636 |  | 168 | sdslen() body — how a length is decoded from the flags byte | 3.4 | 2.2 | 0.773 |
| ns | 2733 |  | 97 | sdsavail() body — free space is `alloc - len`, and always 0 for type 5 | 3.5 | 2.2 | 0.751 |
| walker |  | 2812 | 292 | Code::CodeKey { rung: Names, file: sds.h, decl: 0, sub: 4, line: 0 } |  |  | 0.798 |
| walker |  | 2827 | 15 | Code::CodeKey { rung: Decl, file: sds.h, decl: 63, sub: 0, line: 270 } |  |  | 0.801 |
| walker |  | 2839 | 12 | Code::CodeKey { rung: Doc, file: sds.h, decl: 55, sub: 0, line: 256 } |  |  | 0.811 |
| walker |  | 2905 | 66 | Code::CodeKey { rung: Doc, file: sds.h, decl: 60, sub: 0, line: 266 } |  |  | 0.838 |
| ns | 2942 |  | 209 | The type-5 write paths of sdssetlen() and sdsinclen(), rest elided | 3.6 | 2.2 | 0.797 |
| ns | 3080 |  | 138 | sdsalloc() body and the type-5 no-op in sdssetalloc() | 3.7 | 2.2 | 0.773 |
| walker |  | 3144 | 239 | Code::CodeKey { rung: Names, file: sds.c, decl: 0, sub: 0, line: 0 } |  |  | 0.775 |
| walker |  | 3156 | 12 | Code::CodeKey { rung: Doc, file: sds.c, decl: 7, sub: 0, line: 160 } |  |  | 0.775 |
| walker |  | 3169 | 13 | Code::CodeKey { rung: Body, file: sds.c, decl: 5, sub: 0, line: 149 } |  |  | 0.775 |
| walker |  | 3186 | 17 | Code::CodeKey { rung: Body, file: sds.c, decl: 7, sub: 0, line: 160 } |  |  | 0.775 |
| walker |  | 3206 | 20 | Code::CodeKey { rung: Doc, file: sds.c, decl: 6, sub: 0, line: 154 } |  |  | 0.776 |
| walker |  | 3229 | 23 | Code::CodeKey { rung: Doc, file: sds.c, decl: 8, sub: 0, line: 165 } |  |  | 0.776 |
| walker |  | 3265 | 36 | Code::CodeKey { rung: Doc, file: sds.c, decl: 5, sub: 0, line: 149 } |  |  | 0.776 |
| walker |  | 3334 | 69 | Code::CodeKey { rung: Doc, file: sds.c, decl: 10, sub: 0, line: 193 } |  |  | 0.777 |
| ns | 3337 |  | 257 | Roster of every function defined in sds.c, part 1 (lines 44-440), names only | 4.1 |  | 0.758 |
| walker |  | 3422 | 88 | Code::CodeKey { rung: Doc, file: sds.c, decl: 13, sub: 0, line: 300 } |  |  | 0.758 |
| walker |  | 3520 | 98 | Code::CodeKey { rung: Doc, file: sds.c, decl: 12, sub: 0, line: 256 } |  |  | 0.758 |
| walker |  | 3631 | 111 | Code::CodeKey { rung: Doc, file: sds.c, decl: 11, sub: 0, line: 204 } |  |  | 0.759 |
| ns | 3650 |  | 313 | Roster of every function defined in sds.c, part 2 (lines 451-1325), names only | 4.2 | 4.1 | 0.720 |
| ns | 3791 |  | 141 | sds.c includes + SDS_NOINIT definition, and the sdsalloc.h allocator macros | 4.3 |  | 0.705 |
| walker |  | 3862 | 231 | Code::CodeKey { rung: Names, file: sds.c, decl: 0, sub: 1, line: 0 } |  |  | 0.735 |
| walker |  | 3883 | 21 | Code::CodeKey { rung: Doc, file: sds.c, decl: 24, sub: 0, line: 494 } |  |  | 0.735 |
| walker |  | 3917 | 34 | Code::CodeKey { rung: Doc, file: sds.c, decl: 14, sub: 0, line: 307 } |  |  | 0.736 |
| walker |  | 3958 | 41 | Code::CodeKey { rung: Doc, file: sds.c, decl: 21, sub: 0, line: 440 } |  |  | 0.736 |
| walker |  | 4000 | 42 | Code::CodeKey { rung: Doc, file: sds.c, decl: 20, sub: 0, line: 427 } |  |  | 0.736 |
| ns | 4007 |  | 216 | sdsHdrSize() and sdsReqType() — the type-selection policy | 4.4 | 4.1 | 0.708 |
| walker |  | 4067 | 67 | Code::CodeKey { rung: Doc, file: sds.c, decl: 19, sub: 0, line: 421 } |  |  | 0.708 |
| walker |  | 4135 | 68 | Code::CodeKey { rung: Doc, file: sds.c, decl: 18, sub: 0, line: 413 } |  |  | 0.708 |
| walker |  | 4208 | 73 | Code::CodeKey { rung: Doc, file: sds.c, decl: 16, sub: 0, line: 380 } |  |  | 0.708 |
| ns | 4216 |  | 209 | sdsnewlen() doc comment — NULL vs SDS_NOINIT init, and the always-null-terminated guarantee | 4.5 |  | 0.692 |
| walker |  | 4293 | 85 | Code::CodeKey { rung: Doc, file: sds.c, decl: 17, sub: 0, line: 398 } |  |  | 0.692 |
| walker |  | 4379 | 86 | Code::CodeKey { rung: Doc, file: sds.c, decl: 22, sub: 0, line: 450 } |  |  | 0.692 |
| ns | 4537 |  | 321 | sdsnewlen() body — single allocation, header write, type-5-to-8 upgrade | 4.6 | 4.1 | 0.665 |
| walker |  | 4605 | 226 | Code::CodeKey { rung: Names, file: sds.c, decl: 0, sub: 2, line: 0 } |  |  | 0.675 |
| walker |  | 4626 | 21 | Code::CodeKey { rung: Doc, file: sds.c, decl: 31, sub: 0, line: 783 } |  |  | 0.675 |
| walker |  | 4647 | 21 | Code::CodeKey { rung: Doc, file: sds.c, decl: 32, sub: 0, line: 790 } |  |  | 0.675 |
| walker |  | 4669 | 22 | Code::CodeKey { rung: Doc, file: sds.c, decl: 26, sub: 0, line: 534 } |  |  | 0.675 |
| walker |  | 4720 | 51 | Code::CodeKey { rung: Doc, file: sds.c, decl: 25, sub: 0, line: 526 } |  |  | 0.675 |
| ns | 4883 |  | 346 | sdsMakeRoomFor(): contract comment and the growth policy (198-222) | 4.7 | 4.1 | 0.654 |
| walker |  | 4979 | 259 | Code::CodeKey { rung: Names, file: sds.c, decl: 0, sub: 3, line: 0 } |  |  | 0.684 |
| walker |  | 4999 | 20 | Code::CodeKey { rung: Doc, file: sds.c, decl: 42, sub: 0, line: 1120 } |  |  | 0.684 |
| walker |  | 5025 | 26 | Code::CodeKey { rung: Doc, file: sds.c, decl: 35, sub: 0, line: 885 } |  |  | 0.684 |
| walker |  | 5058 | 33 | Code::CodeKey { rung: Doc, file: sds.c, decl: 37, sub: 0, line: 925 } |  |  | 0.684 |
| walker |  | 5092 | 34 | Code::CodeKey { rung: Doc, file: sds.c, decl: 38, sub: 0, line: 932 } |  |  | 0.684 |
| walker |  | 5130 | 38 | Code::CodeKey { rung: Doc, file: sds.c, decl: 41, sub: 0, line: 1108 } |  |  | 0.684 |
| walker |  | 5233 | 103 | Code::CodeKey { rung: Doc, file: sds.c, decl: 43, sub: 0, line: 1136 } |  |  | 0.684 |
| ns | 5253 |  | 370 | sdsMakeRoomFor(): the realloc-vs-move-header branch (224-248) | 4.8 | 4.7 | 0.660 |
| walker |  | 5343 | 110 | Code::CodeKey { rung: Doc, file: sds.c, decl: 36, sub: 0, line: 898 } |  |  | 0.660 |
| walker |  | 5483 | 140 | Code::CodeKey { rung: Doc, file: sds.c, decl: 33, sub: 0, line: 807 } |  |  | 0.660 |
| ns | 5486 |  | 233 | sdsempty/sdsnew/sdsdup/sdsfree bodies, with sdsfree's NULL contract | 4.9 | 4.1 | 0.653 |
| walker |  | 5632 | 149 | Code::CodeKey { rung: Doc, file: sds.c, decl: 40, sub: 0, line: 1092 } |  |  | 0.653 |
| ns | 5808 |  | 322 | sdsIncrLen() doc comment — the zero-copy read-into-the-buffer pattern | 4.10 |  | 0.634 |
| walker |  | 5809 | 177 | Code::CodeKey { rung: Doc, file: sds.c, decl: 29, sub: 0, line: 725 } |  |  | 0.634 |
| walker |  | 5999 | 190 | Code::CodeKey { rung: Doc, file: sds.c, decl: 30, sub: 0, line: 756 } |  |  | 0.634 |
| ns | 6023 |  | 215 | sdscatfmt() doc comment — the supported format specifiers | 4.11 |  | 0.621 |
| ns | 6118 |  | 95 | README: the error-handling contract (NULL on out of memory) | 5.1 |  | 0.617 |
| walker |  | 6197 | 198 | Code::CodeKey { rung: Doc, file: sds.c, decl: 9, sub: 0, line: 184 } |  |  | 0.617 |
| ns | 6345 |  | 227 | README: the preallocation algorithm and the SDS_MAX_PREALLOC cap | 5.2 |  | 0.607 |
| walker |  | 6406 | 209 | Code::CodeKey { rung: Doc, file: sds.c, decl: 34, sub: 0, line: 835 } |  |  | 0.607 |
| ns | 6485 |  | 140 | README: the internals section's `struct sdshdr` — documentation that is stale | 5.3 |  | 0.599 |
| walker |  | 6617 | 211 | Code::CodeKey { rung: Doc, file: sds.c, decl: 4, sub: 0, line: 89 } |  |  | 0.619 |
| ns | 6751 |  | 266 | README: the two disadvantages — reassign the return value, and shared strings | 5.4 |  | 0.610 |
| walker |  | 6832 | 215 | Code::CodeKey { rung: Doc, file: sds.c, decl: 27, sub: 0, line: 591 } |  |  | 0.610 |
| ns | 6968 |  | 217 | README basics: `sds` is a `char *`, and the three rules of the minimal program | 5.5 |  | 0.603 |
| walker |  | 7047 | 215 | Code::CodeKey { rung: Doc, file: sds.c, decl: 28, sub: 0, line: 616 } |  |  | 0.625 |
| ns | 7160 |  | 192 | README: why sdstrim/sdsrange return void, and how negative indexes work | 5.6 |  | 0.618 |
| walker |  | 7302 | 255 | Code::CodeKey { rung: Doc, file: sds.c, decl: 39, sub: 0, line: 973 } |  |  | 0.619 |
| ns | 7382 |  | 222 | README: swapping the allocator, and why sds_malloc/sds_realloc/sds_free are exported | 5.7 |  | 0.612 |
| ns | 7527 |  | 145 | README: the exact escaping rules sdscatrepr applies | 5.8 |  | 0.608 |
| walker |  | 7624 | 322 | Code::CodeKey { rung: Doc, file: sds.c, decl: 15, sub: 0, line: 334 } |  |  | 0.638 |
| ns | 7716 |  | 189 | README: tokenizer ownership rules and sdssplitargs' quoting behaviour | 5.9 |  | 0.631 |
| ns | 7880 |  | 164 | README: the camelCase warning and how heap checkers see SDS strings | 5.10 |  | 0.627 |
| walker |  | 8049 | 425 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.627 |
| ns | 8197 |  | 317 | sdsrange() body — negative-index normalisation and clamping | 6.1 | 4.2 | 0.612 |
| ns | 8383 |  | 186 | sdscatlen/sdscat/sdscatsds bodies — the append path | 6.2 | 4.1 | 0.603 |
| walker |  | 8467 | 418 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.618 |
| ns | 8591 |  | 208 | sdsRemoveFreeSpace() — contract and the shrink decision | 6.3 | 4.1 | 0.609 |
| ns | 8763 |  | 172 | sdstrim() body — how both ends are walked and the survivor moved down | 6.4 | 4.2 | 0.601 |
| walker |  | 8875 | 408 | Markdown::Section { file: README.md, section_index: 4, keeps_default_concavity: false } |  |  | 0.603 |
| ns | 9015 |  | 252 | sdssplitargs() doc comment — REPL-style parsing and its failure mode | 6.5 |  | 0.612 |
| ns | 9256 |  | 241 | The small inspect/reset bodies: sdsupdatelen, sdsclear, sdsAllocSize, sdsAllocPtr | 6.6 | 4.1 | 0.608 |
| ns | 9375 |  | 119 | The test entry points in sds.c: the SDS_TEST_MAIN guard, sdsTest() and main() | 7.1 | 4.2 | 0.601 |
| walker |  | 9402 | 527 | Markdown::Section { file: README.md, section_index: 5, keeps_default_concavity: false } |  |  | 0.610 |
| ns | 9582 |  | 207 | testhelp.h: the complete test_cond / test_report macro pair | 7.2 |  | 0.616 |
| ns | 9665 |  | 83 | A representative excerpt of the sdsTest() body | 7.3 |  | 0.612 |
| ns | 9794 |  | 129 | The complete Changelog (v1.0 and v2.0) | 7.4 |  | 0.607 |
| walker |  | 9847 | 445 | Markdown::Section { file: README.md, section_index: 6, keeps_default_concavity: false } |  |  | 0.607 |
| ns | 9921 |  | 127 | Authorship and licence: README credits, LICENSE header, .gitignore | 7.5 |  | 0.603 |
