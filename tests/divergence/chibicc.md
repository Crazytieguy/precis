Score(3000)=0.669 I=0.831 C=0.539 ns_rows≤3K=23/51 grid(1000/1442/2080/3000/4327/6240/9000)=0.504/0.749/0.727/0.669/0.585/0.537/0.741

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 51 | 51 | Fs::DirListing { dir: . } |  |  | 0.000 |
| ns | 75 |  | 75 | README title + what chibicc is | 1.1 |  | 0.000 |
| walker |  | 87 | 36 | Fs::DirListing { dir: include } |  |  | 0.000 |
| ns | 126 |  | 51 | Complete repository root listing | 1.2 |  | 0.595 |
| walker |  | 164 | 77 | Markdown::ReadmeHeadline { file: README.md } |  |  | 0.621 |
| walker |  | 223 | 59 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.647 |
| walker |  | 240 | 17 | Code::CodeKey { rung: Names, file: include/stdnoreturn.h, decl: 0, sub: 0, line: 0 } |  |  | 0.647 |
| ns | 263 |  | 137 | README lede tail: real-world programs it compiles | 1.3 | 1.1 | 0.523 |
| ns | 356 |  | 93 | chibicc.h module section banners (the header's table of contents) | 1.4 |  | 0.440 |
| ns | 415 |  | 59 | All README H2 headings | 1.5 |  | 0.470 |
| ns | 528 |  | 113 | README Internals: tokenize and preprocess stages | 1.6 | 1.5 | 0.418 |
| ns | 602 |  | 74 | README Internals: parse and codegen stages | 1.7 | 1.6 | 0.392 |
| ns | 765 |  | 163 | Makefile: build flags and the chibicc target | 1.8 |  | 0.344 |
| ns | 905 |  | 140 | Makefile: test targets | 1.9 | 1.8 | 0.320 |
| walker |  | 922 | 682 | Plaintext::Whole { file: Makefile } |  |  | 0.504 |
| ns | 1003 |  | 98 | Makefile: stage-2 self-host targets and clean | 1.10 | 1.9 | 0.514 |
| ns | 1039 |  | 36 | Complete include/ listing (bundled freestanding headers) | 1.11 |  | 0.523 |
| walker |  | 1133 | 211 | Fs::DirListing { dir: test } |  |  | 0.563 |
| ns | 1273 |  | 234 | README Status: supported and unsupported C11 features | 1.12 | 1.5 | 0.499 |
| walker |  | 1327 | 194 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.749 |
| ns | 1484 |  | 211 | Complete test/ listing | 1.13 |  | 0.792 |
| walker |  | 1494 | 167 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: true } |  |  | 0.792 |
| walker |  | 1543 | 49 | Code::CodeKey { rung: Names, file: include/stdbool.h, decl: 0, sub: 0, line: 0 } |  |  | 0.792 |
| ns | 1569 |  | 85 | The four stage entry points declared in chibicc.h | 2.1 |  | 0.775 |
| walker |  | 1599 | 56 | Code::CodeKey { rung: Names, file: include/stdalign.h, decl: 0, sub: 0, line: 0 } |  |  | 0.775 |
| walker |  | 1636 | 37 | Code::CodeKey { rung: Names, file: strings.c, decl: 0, sub: 0, line: 0 } |  |  | 0.775 |
| walker |  | 1653 | 17 | Code::CodeKey { rung: Doc, file: strings.c, decl: 2, sub: 0, line: 20 } |  |  | 0.775 |
| ns | 1798 |  | 229 | chibicc.h: rest of the tokenize.c public API | 2.2 | 2.1 | 0.747 |
| ns | 1877 |  | 79 | chibicc.h: rest of the preprocess.c API + the unreachable() macro | 2.3 | 2.1 | 0.733 |
| walker |  | 1931 | 278 | Code::CodeKey { rung: Names, file: chibicc.h, decl: 0, sub: 0, line: 0 } |  |  | 0.735 |
| walker |  | 1954 | 23 | Code::CodeKey { rung: Decl, file: chibicc.h, decl: 9, sub: 0, line: 38 } |  |  | 0.737 |
| walker |  | 1960 | 6 | Code::CodeKey { rung: Doc, file: chibicc.h, decl: 14, sub: 0, line: 73 } |  |  | 0.737 |
| ns | 1966 |  | 89 | chibicc.h: parse.c's other exports + the main.c section | 2.4 | 2.1 | 0.719 |
| walker |  | 2020 | 60 | Code::CodeKey { rung: Decl, file: chibicc.h, decl: 13, sub: 0, line: 62 } |  |  | 0.720 |
| ns | 2055 |  | 89 | chibicc.h: strings.c section (StringArray + strarray_push + format) | 2.5 |  | 0.727 |
| walker |  | 2117 | 97 | Code::CodeKey { rung: Decl, file: chibicc.h, decl: 12, sub: 0, line: 52 } |  |  | 0.728 |
| walker |  | 2124 | 7 | Code::CodeKey { rung: Doc, file: chibicc.h, decl: 12, sub: 0, line: 52 } |  |  | 0.728 |
| ns | 2210 |  | 155 | chibicc.h: type.c function API | 2.6 |  | 0.704 |
| ns | 2358 |  | 148 | chibicc.h: the extern singleton Types | 2.7 | 2.6 | 0.675 |
| walker |  | 2404 | 280 | Code::CodeKey { rung: Decl, file: chibicc.h, decl: 15, sub: 0, line: 74 } |  |  | 0.678 |
| ns | 2436 |  | 78 | chibicc.h: unicode.c API | 2.8 |  | 0.669 |
| walker |  | 2492 | 88 | Code::CodeKey { rung: Names, file: include/stddef.h, decl: 0, sub: 0, line: 0 } |  |  | 0.669 |
| ns | 2570 |  | 134 | chibicc.h: hashmap.c API | 2.9 |  | 0.658 |
| ns | 2736 |  | 166 | parse.c file-header comment: how to read the parser | 3.1 |  | 0.639 |
| walker |  | 2743 | 251 | Code::CodeKey { rung: Names, file: include/float.h, decl: 0, sub: 0, line: 0 } |  |  | 0.639 |
| walker |  | 2945 | 202 | Code::CodeKey { rung: Names, file: chibicc.h, decl: 0, sub: 1, line: 0 } |  |  | 0.669 |
| walker |  | 3088 | 143 | Code::CodeKey { rung: Names, file: include/stdarg.h, decl: 0, sub: 0, line: 0 } |  |  | 0.669 |
| ns | 3100 |  | 364 | preprocess.c file-header comment: the hideset macro-expansion algorithm | 3.2 |  | 0.637 |
| walker |  | 3111 | 23 | Code::CodeKey { rung: Decl, file: include/stdarg.h, decl: 3, sub: 0, line: 13 } |  |  | 0.637 |
| walker |  | 3151 | 40 | Code::CodeKey { rung: Decl, file: include/stdarg.h, decl: 1, sub: 0, line: 4 } |  |  | 0.637 |
| ns | 3229 |  | 129 | Internal (non-exported) helpers of type.c, unicode.c and hashmap.c | 3.3 |  | 0.624 |
| walker |  | 3264 | 113 | Code::CodeKey { rung: Decl, file: include/stdarg.h, decl: 5, sub: 0, line: 44 } |  |  | 0.624 |
| walker |  | 3363 | 99 | Markdown::Section { file: README.md, section_index: 11, keeps_default_concavity: false } |  |  | 0.624 |
| ns | 3561 |  | 332 | Function-name roster: main.c (the driver) | 3.4 |  | 0.591 |
| walker |  | 3605 | 242 | Code::CodeKey { rung: Names, file: tokenize.c, decl: 0, sub: 0, line: 0 } |  |  | 0.592 |
| walker |  | 3614 | 9 | Code::CodeKey { rung: Doc, file: tokenize.c, decl: 5, sub: 0, line: 16 } |  |  | 0.592 |
| walker |  | 3632 | 18 | Code::CodeKey { rung: Decl, file: tokenize.c, decl: 6, sub: 0, line: 28 } |  |  | 0.592 |
| walker |  | 3646 | 14 | Code::CodeKey { rung: Doc, file: tokenize.c, decl: 11, sub: 0, line: 84 } |  |  | 0.592 |
| walker |  | 3662 | 16 | Code::CodeKey { rung: Doc, file: tokenize.c, decl: 10, sub: 0, line: 79 } |  |  | 0.592 |
| walker |  | 3670 | 8 | Code::CodeKey { rung: Doc, file: tokenize.c, decl: 1, sub: 0, line: 4 } |  |  | 0.592 |
| walker |  | 3680 | 10 | Code::CodeKey { rung: Doc, file: tokenize.c, decl: 2, sub: 0, line: 7 } |  |  | 0.592 |
| walker |  | 3690 | 10 | Code::CodeKey { rung: Doc, file: tokenize.c, decl: 13, sub: 0, line: 100 } |  |  | 0.592 |
| walker |  | 3703 | 13 | Code::CodeKey { rung: Doc, file: tokenize.c, decl: 4, sub: 0, line: 13 } |  |  | 0.592 |
| walker |  | 3914 | 211 | Code::CodeKey { rung: Names, file: type.c, decl: 0, sub: 0, line: 0 } |  |  | 0.592 |
| walker |  | 3930 | 16 | Code::CodeKey { rung: Doc, file: tokenize.c, decl: 3, sub: 0, line: 10 } |  |  | 0.592 |
| ns | 3965 |  | 404 | Function-name roster: tokenize.c | 3.5 |  | 0.562 |
| walker |  | 4222 | 292 | Code::CodeKey { rung: Names, file: chibicc.h, decl: 0, sub: 2, line: 0 } |  |  | 0.580 |
| walker |  | 4243 | 21 | Code::CodeKey { rung: Decl, file: chibicc.h, decl: 29, sub: 0, line: 108 } |  |  | 0.585 |
| walker |  | 4284 | 41 | Code::CodeKey { rung: Decl, file: chibicc.h, decl: 38, sub: 0, line: 168 } |  |  | 0.585 |
| walker |  | 4293 | 9 | Code::CodeKey { rung: Doc, file: chibicc.h, decl: 35, sub: 0, line: 126 } |  |  | 0.585 |
| ns | 4346 |  | 381 | Function-name roster: codegen.c | 3.6 |  | 0.556 |
| walker |  | 4403 | 110 | Code::CodeKey { rung: Decl, file: chibicc.h, decl: 46, sub: 0, line: 362 } |  |  | 0.557 |
| walker |  | 4411 | 8 | Code::CodeKey { rung: Doc, file: chibicc.h, decl: 46, sub: 0, line: 362 } |  |  | 0.557 |
| walker |  | 4545 | 134 | Code::CodeKey { rung: Decl, file: chibicc.h, decl: 44, sub: 0, line: 301 } |  |  | 0.558 |
| walker |  | 4586 | 41 | Code::CodeKey { rung: Doc, file: chibicc.h, decl: 37, sub: 0, line: 167 } |  |  | 0.559 |
| ns | 4756 |  | 410 | Function-name roster: preprocess.c, token and macro machinery | 3.7 |  | 0.534 |
| walker |  | 4906 | 320 | Code::CodeKey { rung: Decl, file: chibicc.h, decl: 36, sub: 0, line: 127 } |  |  | 0.537 |
| ns | 5022 |  | 266 | Function-name roster: preprocess.c, includes, directives and builtin macros | 3.8 | 3.7 | 0.522 |
| walker |  | 5336 | 430 | Code::CodeKey { rung: Decl, file: chibicc.h, decl: 45, sub: 0, line: 320 } |  |  | 0.525 |
| ns | 5508 |  | 486 | parse.c forward-declaration block: the parser's function index | 3.9 |  | 0.494 |
| walker |  | 5618 | 282 | Code::CodeKey { rung: Decl, file: chibicc.h, decl: 40, sub: 0, line: 228 } |  |  | 0.496 |
| walker |  | 5625 | 7 | Code::CodeKey { rung: Doc, file: chibicc.h, decl: 40, sub: 0, line: 228 } |  |  | 0.496 |
| ns | 5631 |  | 123 | TokenKind enum | 4.1 |  | 0.506 |
| ns | 5707 |  | 76 | File struct | 4.2 |  | 0.515 |
| walker |  | 5931 | 306 | Code::CodeKey { rung: Decl, file: chibicc.h, decl: 40, sub: 1, line: 228 } |  |  | 0.518 |
| ns | 6011 |  | 304 | Token struct (all fields) | 4.3 |  | 0.536 |
| walker |  | 6155 | 224 | Code::CodeKey { rung: Decl, file: chibicc.h, decl: 39, sub: 0, line: 176 } |  |  | 0.537 |
| walker |  | 6161 | 6 | Code::CodeKey { rung: Doc, file: chibicc.h, decl: 39, sub: 0, line: 176 } |  |  | 0.537 |
| ns | 6360 |  | 349 | Obj struct: variables and functions | 4.4 |  | 0.566 |
| walker |  | 6378 | 217 | Code::CodeKey { rung: Decl, file: chibicc.h, decl: 39, sub: 1, line: 176 } |  |  | 0.567 |
| ns | 6461 |  | 101 | Relocation struct | 4.5 | 4.4 | 0.573 |
| walker |  | 6565 | 187 | Code::CodeKey { rung: Decl, file: chibicc.h, decl: 39, sub: 2, line: 176 } |  |  | 0.575 |
| ns | 6766 |  | 305 | NodeKind enum, first half (arithmetic through bit ops) | 4.6 |  | 0.591 |
| walker |  | 6815 | 250 | Code::CodeKey { rung: Names, file: chibicc.h, decl: 0, sub: 3, line: 0 } |  |  | 0.623 |
| walker |  | 7052 | 237 | Code::CodeKey { rung: Names, file: chibicc.h, decl: 0, sub: 4, line: 0 } |  |  | 0.641 |
| walker |  | 7077 | 25 | Code::CodeKey { rung: Decl, file: chibicc.h, decl: 79, sub: 0, line: 428 } |  |  | 0.641 |
| walker |  | 7102 | 25 | Code::CodeKey { rung: Decl, file: chibicc.h, decl: 80, sub: 0, line: 434 } |  |  | 0.641 |
| ns | 7114 |  | 348 | NodeKind enum, second half (control flow, calls, casts, atomics) | 4.7 | 4.6 | 0.652 |
| walker |  | 7229 | 127 | Code::CodeKey { rung: Names, file: chibicc.h, decl: 0, sub: 5, line: 0 } |  |  | 0.668 |
| ns | 7441 |  | 327 | Node struct, first half (operands, control flow, calls) | 4.8 |  | 0.680 |
| walker |  | 7578 | 349 | Code::CodeKey { rung: Names, file: include/float.h, decl: 0, sub: 1, line: 0 } |  |  | 0.680 |
| ns | 7718 |  | 277 | Node struct, second half (goto/switch/case, asm, atomics, literals) | 4.9 | 4.8 | 0.690 |
| walker |  | 7860 | 282 | Code::CodeKey { rung: Names, file: include/stdatomic.h, decl: 0, sub: 0, line: 0 } |  |  | 0.690 |
| ns | 7871 |  | 153 | TypeKind enum | 4.10 |  | 0.695 |
| walker |  | 7919 | 59 | Code::CodeKey { rung: Decl, file: include/stdatomic.h, decl: 11, sub: 0, line: 15 } |  |  | 0.695 |
| ns | 8147 |  | 276 | Type struct, first half + the pointer/array duality comment | 4.11 |  | 0.701 |
| walker |  | 8209 | 290 | Code::CodeKey { rung: Names, file: main.c, decl: 0, sub: 0, line: 0 } |  |  | 0.701 |
| walker |  | 8232 | 23 | Code::CodeKey { rung: Decl, file: main.c, decl: 1, sub: 0, line: 3 } |  |  | 0.701 |
| ns | 8310 |  | 163 | Type struct, second half (array, VLA, struct, function members) | 4.12 | 4.11 | 0.705 |
| ns | 8435 |  | 125 | Member struct (struct/union members incl. bitfields) | 4.13 | 4.12 | 0.708 |
| walker |  | 8456 | 224 | Markdown::Section { file: README.md, section_index: 4, keeps_default_concavity: false } |  |  | 0.724 |
| ns | 8524 |  | 89 | HashEntry / HashMap structs | 4.14 | 2.9 | 0.726 |
| ns | 8615 |  | 91 | main(): driver entry and the -cc1 self-re-exec split | 5.1 | 3.4 | 0.720 |
| walker |  | 8688 | 232 | Markdown::Section { file: README.md, section_index: 5, keeps_default_concavity: true } |  |  | 0.731 |
| walker |  | 8875 | 187 | Markdown::Section { file: README.md, section_index: 6, keeps_default_concavity: false } |  |  | 0.758 |
| ns | 8913 |  | 298 | cc1(): the compile pipeline in one function | 5.2 | 5.1 | 0.741 |
| walker |  | 9006 | 131 | Code::CodeKey { rung: Names, file: unicode.c, decl: 0, sub: 0, line: 0 } |  |  | 0.741 |
| walker |  | 9020 | 14 | Code::CodeKey { rung: Doc, file: unicode.c, decl: 1, sub: 0, line: 4 } |  |  | 0.741 |
| walker |  | 9048 | 28 | Code::CodeKey { rung: Doc, file: unicode.c, decl: 5, sub: 0, line: 110 } |  |  | 0.741 |
| walker |  | 9076 | 28 | Code::CodeKey { rung: Doc, file: unicode.c, decl: 7, sub: 0, line: 181 } |  |  | 0.741 |
| walker |  | 9155 | 79 | Code::CodeKey { rung: Body, file: unicode.c, decl: 7, sub: 0, line: 181 } |  |  | 0.741 |
| ns | 9183 |  | 270 | parse(): the top-level program loop | 5.3 |  | 0.729 |
| walker |  | 9378 | 223 | Code::CodeKey { rung: Names, file: include/stdatomic.h, decl: 0, sub: 1, line: 0 } |  |  | 0.729 |
| ns | 9430 |  | 247 | The statement grammar (comment above stmt()) | 6.1 |  | 0.722 |
| walker |  | 9595 | 217 | Code::CodeKey { rung: Names, file: include/stdatomic.h, decl: 0, sub: 2, line: 0 } |  |  | 0.722 |
| walker |  | 9612 | 17 | Code::CodeKey { rung: Decl, file: include/stdatomic.h, decl: 32, sub: 0, line: 49 } |  |  | 0.722 |
| walker |  | 9629 | 17 | Code::CodeKey { rung: Decl, file: include/stdatomic.h, decl: 33, sub: 0, line: 52 } |  |  | 0.722 |
| ns | 9756 |  | 326 | The expression precedence chain, as grammar comments | 6.2 | 3.9 | 0.714 |
| walker |  | 9866 | 237 | Code::CodeKey { rung: Names, file: main.c, decl: 0, sub: 1, line: 0 } |  |  | 0.720 |
| walker |  | 9875 | 9 | Code::CodeKey { rung: Doc, file: main.c, decl: 36, sub: 0, line: 367 } |  |  | 0.720 |
| ns | 9989 |  | 233 | Every preprocessor directive chibicc handles | 6.3 |  | 0.713 |
