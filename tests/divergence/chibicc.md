Score(3000)=0.702 I=0.852 C=0.579 ns_rows≤3K=23/51 grid(1000/1442/2080/3000/4327/6240/9000)=0.513/0.749/0.731/0.702/0.699/0.618/0.739

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 51 | 51 | Fs::DirListing { dir: . } |  |  | 0.000 |
| ns | 75 |  | 75 | README title + what chibicc is | 1.1 |  | 0.000 |
| walker |  | 87 | 36 | Fs::DirListing { dir: include } |  |  | 0.000 |
| ns | 126 |  | 51 | Complete repository root listing | 1.2 |  | 0.595 |
| walker |  | 164 | 77 | Markdown::ReadmeHeadline { file: README.md } |  |  | 0.621 |
| walker |  | 223 | 59 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.647 |
| ns | 263 |  | 137 | README lede tail: real-world programs it compiles | 1.3 | 1.1 | 0.523 |
| ns | 356 |  | 93 | chibicc.h module section banners (the header's table of contents) | 1.4 |  | 0.440 |
| ns | 415 |  | 59 | All README H2 headings | 1.5 |  | 0.470 |
| ns | 528 |  | 113 | README Internals: tokenize and preprocess stages | 1.6 | 1.5 | 0.418 |
| ns | 602 |  | 74 | README Internals: parse and codegen stages | 1.7 | 1.6 | 0.392 |
| ns | 765 |  | 163 | Makefile: build flags and the chibicc target | 1.8 |  | 0.344 |
| walker |  | 905 | 682 | Plaintext::Whole { file: Makefile } |  |  | 0.504 |
| ns | 905 |  | 140 | Makefile: test targets | 1.9 | 1.8 | 0.504 |
| ns | 1003 |  | 98 | Makefile: stage-2 self-host targets and clean | 1.10 | 1.9 | 0.514 |
| ns | 1039 |  | 36 | Complete include/ listing (bundled freestanding headers) | 1.11 |  | 0.523 |
| walker |  | 1116 | 211 | Fs::DirListing { dir: test } |  |  | 0.563 |
| ns | 1273 |  | 234 | README Status: supported and unsupported C11 features | 1.12 | 1.5 | 0.499 |
| walker |  | 1310 | 194 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.749 |
| ns | 1484 |  | 211 | Complete test/ listing | 1.13 |  | 0.792 |
| ns | 1569 |  | 85 | The four stage entry points declared in chibicc.h | 2.1 |  | 0.775 |
| walker |  | 1588 | 278 | Code::CodeKey { rung: Names, file: chibicc.h, decl: 0, sub: 0, line: 0 } |  |  | 0.777 |
| walker |  | 1611 | 23 | Code::CodeKey { rung: Decl, file: chibicc.h, decl: 9, sub: 0, line: 38 } |  |  | 0.779 |
| walker |  | 1617 | 6 | Code::CodeKey { rung: Doc, file: chibicc.h, decl: 14, sub: 0, line: 73 } |  |  | 0.779 |
| walker |  | 1677 | 60 | Code::CodeKey { rung: Decl, file: chibicc.h, decl: 13, sub: 0, line: 62 } |  |  | 0.780 |
| walker |  | 1774 | 97 | Code::CodeKey { rung: Decl, file: chibicc.h, decl: 12, sub: 0, line: 52 } |  |  | 0.781 |
| walker |  | 1781 | 7 | Code::CodeKey { rung: Doc, file: chibicc.h, decl: 12, sub: 0, line: 52 } |  |  | 0.782 |
| ns | 1798 |  | 229 | chibicc.h: rest of the tokenize.c public API | 2.2 | 2.1 | 0.754 |
| ns | 1877 |  | 79 | chibicc.h: rest of the preprocess.c API + the unreachable() macro | 2.3 | 2.1 | 0.739 |
| ns | 1966 |  | 89 | chibicc.h: parse.c's other exports + the main.c section | 2.4 | 2.1 | 0.722 |
| ns | 2055 |  | 89 | chibicc.h: strings.c section (StringArray + strarray_push + format) | 2.5 |  | 0.728 |
| walker |  | 2061 | 280 | Code::CodeKey { rung: Decl, file: chibicc.h, decl: 15, sub: 0, line: 74 } |  |  | 0.731 |
| ns | 2210 |  | 155 | chibicc.h: type.c function API | 2.6 |  | 0.707 |
| walker |  | 2263 | 202 | Code::CodeKey { rung: Names, file: chibicc.h, decl: 0, sub: 1, line: 0 } |  |  | 0.741 |
| ns | 2358 |  | 148 | chibicc.h: the extern singleton Types | 2.7 | 2.6 | 0.711 |
| ns | 2436 |  | 78 | chibicc.h: unicode.c API | 2.8 |  | 0.702 |
| walker |  | 2555 | 292 | Code::CodeKey { rung: Names, file: chibicc.h, decl: 0, sub: 2, line: 0 } |  |  | 0.725 |
| ns | 2570 |  | 134 | chibicc.h: hashmap.c API | 2.9 |  | 0.713 |
| walker |  | 2576 | 21 | Code::CodeKey { rung: Decl, file: chibicc.h, decl: 29, sub: 0, line: 108 } |  |  | 0.719 |
| walker |  | 2617 | 41 | Code::CodeKey { rung: Decl, file: chibicc.h, decl: 38, sub: 0, line: 168 } |  |  | 0.719 |
| walker |  | 2626 | 9 | Code::CodeKey { rung: Doc, file: chibicc.h, decl: 35, sub: 0, line: 126 } |  |  | 0.719 |
| walker |  | 2736 | 110 | Code::CodeKey { rung: Decl, file: chibicc.h, decl: 46, sub: 0, line: 362 } |  |  | 0.699 |
| ns | 2736 |  | 166 | parse.c file-header comment: how to read the parser | 3.1 |  | 0.699 |
| walker |  | 2744 | 8 | Code::CodeKey { rung: Doc, file: chibicc.h, decl: 46, sub: 0, line: 362 } |  |  | 0.699 |
| walker |  | 2878 | 134 | Code::CodeKey { rung: Decl, file: chibicc.h, decl: 44, sub: 0, line: 301 } |  |  | 0.701 |
| walker |  | 2919 | 41 | Code::CodeKey { rung: Doc, file: chibicc.h, decl: 37, sub: 0, line: 167 } |  |  | 0.702 |
| ns | 3100 |  | 364 | preprocess.c file-header comment: the hideset macro-expansion algorithm | 3.2 |  | 0.668 |
| ns | 3229 |  | 129 | Internal (non-exported) helpers of type.c, unicode.c and hashmap.c | 3.3 |  | 0.654 |
| walker |  | 3239 | 320 | Code::CodeKey { rung: Decl, file: chibicc.h, decl: 36, sub: 0, line: 127 } |  |  | 0.658 |
| walker |  | 3489 | 250 | Code::CodeKey { rung: Names, file: chibicc.h, decl: 0, sub: 3, line: 0 } |  |  | 0.716 |
| ns | 3561 |  | 332 | Function-name roster: main.c (the driver) | 3.4 |  | 0.679 |
| walker |  | 3726 | 237 | Code::CodeKey { rung: Names, file: chibicc.h, decl: 0, sub: 4, line: 0 } |  |  | 0.709 |
| walker |  | 3751 | 25 | Code::CodeKey { rung: Decl, file: chibicc.h, decl: 79, sub: 0, line: 428 } |  |  | 0.709 |
| walker |  | 3776 | 25 | Code::CodeKey { rung: Decl, file: chibicc.h, decl: 80, sub: 0, line: 434 } |  |  | 0.710 |
| walker |  | 3903 | 127 | Code::CodeKey { rung: Names, file: chibicc.h, decl: 0, sub: 5, line: 0 } |  |  | 0.738 |
| ns | 3965 |  | 404 | Function-name roster: tokenize.c | 3.5 |  | 0.696 |
| walker |  | 4333 | 430 | Code::CodeKey { rung: Decl, file: chibicc.h, decl: 45, sub: 0, line: 320 } |  |  | 0.699 |
| ns | 4346 |  | 381 | Function-name roster: codegen.c | 3.6 |  | 0.664 |
| walker |  | 4615 | 282 | Code::CodeKey { rung: Decl, file: chibicc.h, decl: 40, sub: 0, line: 228 } |  |  | 0.666 |
| walker |  | 4622 | 7 | Code::CodeKey { rung: Doc, file: chibicc.h, decl: 40, sub: 0, line: 228 } |  |  | 0.666 |
| ns | 4756 |  | 410 | Function-name roster: preprocess.c, token and macro machinery | 3.7 |  | 0.636 |
| walker |  | 4928 | 306 | Code::CodeKey { rung: Decl, file: chibicc.h, decl: 40, sub: 1, line: 228 } |  |  | 0.640 |
| ns | 5022 |  | 266 | Function-name roster: preprocess.c, includes, directives and builtin macros | 3.8 | 3.7 | 0.622 |
| walker |  | 5152 | 224 | Code::CodeKey { rung: Decl, file: chibicc.h, decl: 39, sub: 0, line: 176 } |  |  | 0.623 |
| walker |  | 5158 | 6 | Code::CodeKey { rung: Doc, file: chibicc.h, decl: 39, sub: 0, line: 176 } |  |  | 0.623 |
| walker |  | 5375 | 217 | Code::CodeKey { rung: Decl, file: chibicc.h, decl: 39, sub: 1, line: 176 } |  |  | 0.625 |
| ns | 5508 |  | 486 | parse.c forward-declaration block: the parser's function index | 3.9 |  | 0.588 |
| walker |  | 5562 | 187 | Code::CodeKey { rung: Decl, file: chibicc.h, decl: 39, sub: 2, line: 176 } |  |  | 0.589 |
| walker |  | 5611 | 49 | Code::CodeKey { rung: Names, file: include/stdbool.h, decl: 0, sub: 0, line: 0 } |  |  | 0.589 |
| ns | 5631 |  | 123 | TokenKind enum | 4.1 |  | 0.597 |
| walker |  | 5667 | 56 | Code::CodeKey { rung: Names, file: include/stdalign.h, decl: 0, sub: 0, line: 0 } |  |  | 0.597 |
| ns | 5707 |  | 76 | File struct | 4.2 |  | 0.604 |
| walker |  | 5755 | 88 | Code::CodeKey { rung: Names, file: include/stddef.h, decl: 0, sub: 0, line: 0 } |  |  | 0.604 |
| walker |  | 5898 | 143 | Code::CodeKey { rung: Names, file: include/stdarg.h, decl: 0, sub: 0, line: 0 } |  |  | 0.604 |
| walker |  | 5921 | 23 | Code::CodeKey { rung: Decl, file: include/stdarg.h, decl: 3, sub: 0, line: 13 } |  |  | 0.604 |
| walker |  | 5961 | 40 | Code::CodeKey { rung: Decl, file: include/stdarg.h, decl: 1, sub: 0, line: 4 } |  |  | 0.604 |
| ns | 6011 |  | 304 | Token struct (all fields) | 4.3 |  | 0.618 |
| walker |  | 6074 | 113 | Code::CodeKey { rung: Decl, file: include/stdarg.h, decl: 5, sub: 0, line: 44 } |  |  | 0.618 |
| walker |  | 6241 | 167 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.618 |
| walker |  | 6340 | 99 | Markdown::Section { file: README.md, section_index: 11, keeps_default_concavity: false } |  |  | 0.618 |
| walker |  | 6357 | 17 | Code::CodeKey { rung: Names, file: include/stdnoreturn.h, decl: 0, sub: 0, line: 0 } |  |  | 0.618 |
| ns | 6360 |  | 349 | Obj struct: variables and functions | 4.4 |  | 0.639 |
| ns | 6461 |  | 101 | Relocation struct | 4.5 | 4.4 | 0.644 |
| walker |  | 6608 | 251 | Code::CodeKey { rung: Names, file: include/float.h, decl: 0, sub: 0, line: 0 } |  |  | 0.644 |
| ns | 6766 |  | 305 | NodeKind enum, first half (arithmetic through bit ops) | 4.6 |  | 0.656 |
| walker |  | 6957 | 349 | Code::CodeKey { rung: Names, file: include/float.h, decl: 0, sub: 1, line: 0 } |  |  | 0.656 |
| ns | 7114 |  | 348 | NodeKind enum, second half (control flow, calls, casts, atomics) | 4.7 | 4.6 | 0.666 |
| walker |  | 7239 | 282 | Code::CodeKey { rung: Names, file: include/stdatomic.h, decl: 0, sub: 0, line: 0 } |  |  | 0.666 |
| walker |  | 7298 | 59 | Code::CodeKey { rung: Decl, file: include/stdatomic.h, decl: 11, sub: 0, line: 15 } |  |  | 0.666 |
| ns | 7441 |  | 327 | Node struct, first half (operands, control flow, calls) | 4.8 |  | 0.678 |
| walker |  | 7521 | 223 | Code::CodeKey { rung: Names, file: include/stdatomic.h, decl: 0, sub: 1, line: 0 } |  |  | 0.678 |
| ns | 7718 |  | 277 | Node struct, second half (goto/switch/case, asm, atomics, literals) | 4.9 | 4.8 | 0.688 |
| walker |  | 7738 | 217 | Code::CodeKey { rung: Names, file: include/stdatomic.h, decl: 0, sub: 2, line: 0 } |  |  | 0.688 |
| walker |  | 7755 | 17 | Code::CodeKey { rung: Decl, file: include/stdatomic.h, decl: 32, sub: 0, line: 49 } |  |  | 0.688 |
| walker |  | 7772 | 17 | Code::CodeKey { rung: Decl, file: include/stdatomic.h, decl: 33, sub: 0, line: 52 } |  |  | 0.688 |
| ns | 7871 |  | 153 | TypeKind enum | 4.10 |  | 0.693 |
| walker |  | 8003 | 231 | Code::CodeKey { rung: Names, file: include/stdatomic.h, decl: 0, sub: 3, line: 0 } |  |  | 0.693 |
| ns | 8147 |  | 276 | Type struct, first half + the pointer/array duality comment | 4.11 |  | 0.699 |
| walker |  | 8214 | 211 | Code::CodeKey { rung: Names, file: include/stdatomic.h, decl: 0, sub: 4, line: 0 } |  |  | 0.699 |
| ns | 8310 |  | 163 | Type struct, second half (array, VLA, struct, function members) | 4.12 | 4.11 | 0.703 |
| walker |  | 8338 | 124 | Code::CodeKey { rung: Names, file: include/stdatomic.h, decl: 0, sub: 5, line: 0 } |  |  | 0.703 |
| ns | 8435 |  | 125 | Member struct (struct/union members incl. bitfields) | 4.13 | 4.12 | 0.706 |
| ns | 8524 |  | 89 | HashEntry / HashMap structs | 4.14 | 2.9 | 0.709 |
| walker |  | 8562 | 224 | Markdown::Section { file: README.md, section_index: 4, keeps_default_concavity: false } |  |  | 0.724 |
| ns | 8615 |  | 91 | main(): driver entry and the -cc1 self-re-exec split | 5.1 | 3.4 | 0.719 |
| walker |  | 8794 | 232 | Markdown::Section { file: README.md, section_index: 5, keeps_default_concavity: false } |  |  | 0.729 |
| ns | 8913 |  | 298 | cc1(): the compile pipeline in one function | 5.2 | 5.1 | 0.713 |
| walker |  | 8981 | 187 | Markdown::Section { file: README.md, section_index: 6, keeps_default_concavity: false } |  |  | 0.739 |
| ns | 9183 |  | 270 | parse(): the top-level program loop | 5.3 |  | 0.727 |
| walker |  | 9271 | 290 | Code::CodeKey { rung: Names, file: main.c, decl: 0, sub: 0, line: 0 } |  |  | 0.727 |
| walker |  | 9294 | 23 | Code::CodeKey { rung: Decl, file: main.c, decl: 1, sub: 0, line: 3 } |  |  | 0.727 |
| ns | 9430 |  | 247 | The statement grammar (comment above stmt()) | 6.1 |  | 0.720 |
| walker |  | 9531 | 237 | Code::CodeKey { rung: Names, file: main.c, decl: 0, sub: 1, line: 0 } |  |  | 0.726 |
| walker |  | 9540 | 9 | Code::CodeKey { rung: Doc, file: main.c, decl: 36, sub: 0, line: 367 } |  |  | 0.726 |
| walker |  | 9572 | 32 | Code::CodeKey { rung: Body, file: main.c, decl: 27, sub: 0, line: 37 } |  |  | 0.726 |
| walker |  | 9604 | 32 | Code::CodeKey { rung: Body, file: main.c, decl: 37, sub: 0, line: 375 } |  |  | 0.726 |
| ns | 9756 |  | 326 | The expression precedence chain, as grammar comments | 6.2 | 3.9 | 0.718 |
| walker |  | 9836 | 232 | Code::CodeKey { rung: Names, file: main.c, decl: 0, sub: 2, line: 0 } |  |  | 0.736 |
| walker |  | 9849 | 13 | Code::CodeKey { rung: Doc, file: main.c, decl: 49, sub: 0, line: 586 } |  |  | 0.736 |
| walker |  | 9864 | 15 | Code::CodeKey { rung: Doc, file: main.c, decl: 41, sub: 0, line: 433 } |  |  | 0.736 |
| walker |  | 9919 | 55 | Code::CodeKey { rung: Doc, file: main.c, decl: 43, sub: 0, line: 461 } |  |  | 0.736 |
| walker |  | 9982 | 63 | Code::CodeKey { rung: Names, file: tokenize.c, decl: 0, sub: 0, line: 0 } |  |  | 0.736 |
| ns | 9989 |  | 233 | Every preprocessor directive chibicc handles | 6.3 |  | 0.730 |
