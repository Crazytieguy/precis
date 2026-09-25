Score(3000)=0.639 I=0.821 C=0.497 ns_rows≤3K=23/51 grid(1000/1442/2080/3000/4327/6240/9000)=0.327/0.304/0.361/0.639/0.562/0.530/0.610

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 51 | 51 | Fs::DirListing { dir: . } |  |  | 0.000 |
| ns | 75 |  | 75 | README title + what chibicc is | 1.1 |  | 0.000 |
| walker |  | 87 | 36 | Fs::DirListing { dir: include } |  |  | 0.000 |
| walker |  | 104 | 17 | Code::CodeKey { rung: Names, file: include/stdnoreturn.h, decl: 0, sub: 0, line: 0 } |  |  | 0.000 |
| ns | 126 |  | 51 | Complete repository root listing | 1.2 |  | 0.595 |
| walker |  | 181 | 77 | Markdown::ReadmeHeadline { file: README.md } |  |  | 0.621 |
| walker |  | 240 | 59 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.647 |
| ns | 263 |  | 137 | README lede tail: real-world programs it compiles | 1.3 | 1.1 | 0.523 |
| walker |  | 289 | 49 | Code::CodeKey { rung: Names, file: include/stdbool.h, decl: 0, sub: 0, line: 0 } |  |  | 0.523 |
| walker |  | 345 | 56 | Code::CodeKey { rung: Names, file: include/stdalign.h, decl: 0, sub: 0, line: 0 } |  |  | 0.523 |
| ns | 356 |  | 93 | chibicc.h module section banners (the header's table of contents) | 1.4 |  | 0.440 |
| walker |  | 382 | 37 | Code::CodeKey { rung: Names, file: strings.c, decl: 0, sub: 0, line: 0 } |  |  | 0.440 |
| ns | 415 |  | 59 | All README H2 headings | 1.5 |  | 0.470 |
| ns | 528 |  | 113 | README Internals: tokenize and preprocess stages | 1.6 | 1.5 | 0.418 |
| ns | 602 |  | 74 | README Internals: parse and codegen stages | 1.7 | 1.6 | 0.392 |
| walker |  | 660 | 278 | Code::CodeKey { rung: Names, file: chibicc.h, decl: 0, sub: 0, line: 0 } |  |  | 0.394 |
| walker |  | 683 | 23 | Code::CodeKey { rung: Decl, file: chibicc.h, decl: 9, sub: 0, line: 38 } |  |  | 0.396 |
| walker |  | 689 | 6 | Code::CodeKey { rung: Doc, file: chibicc.h, decl: 14, sub: 0, line: 73 } |  |  | 0.396 |
| walker |  | 749 | 60 | Code::CodeKey { rung: Decl, file: chibicc.h, decl: 13, sub: 0, line: 62 } |  |  | 0.398 |
| ns | 765 |  | 163 | Makefile: build flags and the chibicc target | 1.8 |  | 0.349 |
| walker |  | 846 | 97 | Code::CodeKey { rung: Decl, file: chibicc.h, decl: 12, sub: 0, line: 52 } |  |  | 0.351 |
| walker |  | 853 | 7 | Code::CodeKey { rung: Doc, file: chibicc.h, decl: 12, sub: 0, line: 52 } |  |  | 0.351 |
| ns | 905 |  | 140 | Makefile: test targets | 1.9 | 1.8 | 0.327 |
| ns | 1003 |  | 98 | Makefile: stage-2 self-host targets and clean | 1.10 | 1.9 | 0.313 |
| ns | 1039 |  | 36 | Complete include/ listing (bundled freestanding headers) | 1.11 |  | 0.343 |
| ns | 1273 |  | 234 | README Status: supported and unsupported C11 features | 1.12 | 1.5 | 0.304 |
| ns | 1484 |  | 211 | Complete test/ listing | 1.13 |  | 0.247 |
| walker |  | 1535 | 682 | Plaintext::Whole { file: Makefile } |  |  | 0.383 |
| ns | 1569 |  | 85 | The four stage entry points declared in chibicc.h | 2.1 |  | 0.375 |
| walker |  | 1623 | 88 | Code::CodeKey { rung: Names, file: include/stddef.h, decl: 0, sub: 0, line: 0 } |  |  | 0.375 |
| ns | 1798 |  | 229 | chibicc.h: rest of the tokenize.c public API | 2.2 | 2.1 | 0.362 |
| walker |  | 1874 | 251 | Code::CodeKey { rung: Names, file: include/float.h, decl: 0, sub: 0, line: 0 } |  |  | 0.362 |
| ns | 1877 |  | 79 | chibicc.h: rest of the preprocess.c API + the unreachable() macro | 2.3 | 2.1 | 0.355 |
| ns | 1966 |  | 89 | chibicc.h: parse.c's other exports + the main.c section | 2.4 | 2.1 | 0.346 |
| ns | 2055 |  | 89 | chibicc.h: strings.c section (StringArray + strarray_push + format) | 2.5 |  | 0.361 |
| walker |  | 2085 | 211 | Fs::DirListing { dir: test } |  |  | 0.510 |
| ns | 2210 |  | 155 | chibicc.h: type.c function API | 2.6 |  | 0.493 |
| ns | 2358 |  | 148 | chibicc.h: the extern singleton Types | 2.7 | 2.6 | 0.473 |
| walker |  | 2365 | 280 | Code::CodeKey { rung: Decl, file: chibicc.h, decl: 15, sub: 0, line: 74 } |  |  | 0.477 |
| ns | 2436 |  | 78 | chibicc.h: unicode.c API | 2.8 |  | 0.471 |
| walker |  | 2559 | 194 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.669 |
| ns | 2570 |  | 134 | chibicc.h: hashmap.c API | 2.9 |  | 0.658 |
| walker |  | 2726 | 167 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: true } |  |  | 0.658 |
| ns | 2736 |  | 166 | parse.c file-header comment: how to read the parser | 3.1 |  | 0.639 |
| walker |  | 2869 | 143 | Code::CodeKey { rung: Names, file: include/stdarg.h, decl: 0, sub: 0, line: 0 } |  |  | 0.639 |
| walker |  | 2892 | 23 | Code::CodeKey { rung: Decl, file: include/stdarg.h, decl: 3, sub: 0, line: 13 } |  |  | 0.639 |
| walker |  | 2932 | 40 | Code::CodeKey { rung: Decl, file: include/stdarg.h, decl: 1, sub: 0, line: 4 } |  |  | 0.639 |
| walker |  | 3045 | 113 | Code::CodeKey { rung: Decl, file: include/stdarg.h, decl: 5, sub: 0, line: 44 } |  |  | 0.639 |
| ns | 3100 |  | 364 | preprocess.c file-header comment: the hideset macro-expansion algorithm | 3.2 |  | 0.607 |
| ns | 3229 |  | 129 | Internal (non-exported) helpers of type.c, unicode.c and hashmap.c | 3.3 |  | 0.595 |
| walker |  | 3287 | 242 | Code::CodeKey { rung: Names, file: tokenize.c, decl: 0, sub: 0, line: 0 } |  |  | 0.595 |
| walker |  | 3296 | 9 | Code::CodeKey { rung: Doc, file: tokenize.c, decl: 5, sub: 0, line: 16 } |  |  | 0.595 |
| walker |  | 3314 | 18 | Code::CodeKey { rung: Decl, file: tokenize.c, decl: 6, sub: 0, line: 28 } |  |  | 0.595 |
| walker |  | 3328 | 14 | Code::CodeKey { rung: Doc, file: tokenize.c, decl: 11, sub: 0, line: 84 } |  |  | 0.595 |
| walker |  | 3539 | 211 | Code::CodeKey { rung: Names, file: type.c, decl: 0, sub: 0, line: 0 } |  |  | 0.595 |
| walker |  | 3555 | 16 | Code::CodeKey { rung: Doc, file: tokenize.c, decl: 10, sub: 0, line: 79 } |  |  | 0.595 |
| ns | 3561 |  | 332 | Function-name roster: main.c (the driver) | 3.4 |  | 0.564 |
| walker |  | 3572 | 17 | Code::CodeKey { rung: Doc, file: strings.c, decl: 2, sub: 0, line: 20 } |  |  | 0.564 |
| walker |  | 3774 | 202 | Code::CodeKey { rung: Names, file: chibicc.h, decl: 0, sub: 1, line: 0 } |  |  | 0.592 |
| ns | 3965 |  | 404 | Function-name roster: tokenize.c | 3.5 |  | 0.562 |
| walker |  | 4056 | 282 | Code::CodeKey { rung: Names, file: include/stdatomic.h, decl: 0, sub: 0, line: 0 } |  |  | 0.562 |
| walker |  | 4115 | 59 | Code::CodeKey { rung: Decl, file: include/stdatomic.h, decl: 11, sub: 0, line: 15 } |  |  | 0.562 |
| ns | 4346 |  | 381 | Function-name roster: codegen.c | 3.6 |  | 0.533 |
| walker |  | 4405 | 290 | Code::CodeKey { rung: Names, file: main.c, decl: 0, sub: 0, line: 0 } |  |  | 0.533 |
| walker |  | 4428 | 23 | Code::CodeKey { rung: Decl, file: main.c, decl: 1, sub: 0, line: 3 } |  |  | 0.533 |
| walker |  | 4559 | 131 | Code::CodeKey { rung: Names, file: unicode.c, decl: 0, sub: 0, line: 0 } |  |  | 0.534 |
| walker |  | 4573 | 14 | Code::CodeKey { rung: Doc, file: unicode.c, decl: 1, sub: 0, line: 4 } |  |  | 0.534 |
| ns | 4756 |  | 410 | Function-name roster: preprocess.c, token and macro machinery | 3.7 |  | 0.510 |
| walker |  | 4922 | 349 | Code::CodeKey { rung: Names, file: include/float.h, decl: 0, sub: 1, line: 0 } |  |  | 0.510 |
| walker |  | 4950 | 28 | Code::CodeKey { rung: Doc, file: unicode.c, decl: 5, sub: 0, line: 110 } |  |  | 0.510 |
| walker |  | 4978 | 28 | Code::CodeKey { rung: Doc, file: unicode.c, decl: 7, sub: 0, line: 181 } |  |  | 0.510 |
| ns | 5022 |  | 266 | Function-name roster: preprocess.c, includes, directives and builtin macros | 3.8 | 3.7 | 0.496 |
| walker |  | 5264 | 286 | Code::CodeKey { rung: Names, file: preprocess.c, decl: 0, sub: 0, line: 0 } |  |  | 0.497 |
| walker |  | 5286 | 22 | Code::CodeKey { rung: Decl, file: preprocess.c, decl: 2, sub: 0, line: 28 } |  |  | 0.497 |
| walker |  | 5309 | 23 | Code::CodeKey { rung: Decl, file: preprocess.c, decl: 11, sub: 0, line: 63 } |  |  | 0.497 |
| walker |  | 5350 | 41 | Code::CodeKey { rung: Decl, file: preprocess.c, decl: 4, sub: 0, line: 34 } |  |  | 0.497 |
| walker |  | 5401 | 51 | Code::CodeKey { rung: Decl, file: preprocess.c, decl: 9, sub: 0, line: 55 } |  |  | 0.497 |
| walker |  | 5473 | 72 | Code::CodeKey { rung: Decl, file: preprocess.c, decl: 7, sub: 0, line: 44 } |  |  | 0.497 |
| ns | 5508 |  | 486 | parse.c forward-declaration block: the parser's function index | 3.9 |  | 0.467 |
| ns | 5631 |  | 123 | TokenKind enum | 4.1 |  | 0.478 |
| ns | 5707 |  | 76 | File struct | 4.2 |  | 0.487 |
| walker |  | 5805 | 332 | Code::CodeKey { rung: Names, file: hashmap.c, decl: 0, sub: 0, line: 0 } |  |  | 0.495 |
| walker |  | 5814 | 9 | Code::CodeKey { rung: Doc, file: hashmap.c, decl: 4, sub: 0, line: 15 } |  |  | 0.495 |
| walker |  | 5824 | 10 | Code::CodeKey { rung: Doc, file: hashmap.c, decl: 1, sub: 0, line: 6 } |  |  | 0.495 |
| walker |  | 5836 | 12 | Code::CodeKey { rung: Doc, file: hashmap.c, decl: 2, sub: 0, line: 9 } |  |  | 0.495 |
| walker |  | 5851 | 15 | Code::CodeKey { rung: Body, file: hashmap.c, decl: 14, sub: 0, line: 126 } |  |  | 0.495 |
| walker |  | 5867 | 16 | Code::CodeKey { rung: Doc, file: hashmap.c, decl: 3, sub: 0, line: 12 } |  |  | 0.495 |
| walker |  | 5883 | 16 | Code::CodeKey { rung: Body, file: hashmap.c, decl: 10, sub: 0, line: 108 } |  |  | 0.495 |
| walker |  | 5891 | 8 | Code::CodeKey { rung: Doc, file: tokenize.c, decl: 1, sub: 0, line: 4 } |  |  | 0.495 |
| ns | 6011 |  | 304 | Token struct (all fields) | 4.3 |  | 0.513 |
| walker |  | 6183 | 292 | Code::CodeKey { rung: Names, file: chibicc.h, decl: 0, sub: 2, line: 0 } |  |  | 0.526 |
| walker |  | 6204 | 21 | Code::CodeKey { rung: Decl, file: chibicc.h, decl: 29, sub: 0, line: 108 } |  |  | 0.530 |
| walker |  | 6245 | 41 | Code::CodeKey { rung: Decl, file: chibicc.h, decl: 38, sub: 0, line: 168 } |  |  | 0.530 |
| walker |  | 6355 | 110 | Code::CodeKey { rung: Decl, file: chibicc.h, decl: 46, sub: 0, line: 362 } |  |  | 0.531 |
| ns | 6360 |  | 349 | Obj struct: variables and functions | 4.4 |  | 0.511 |
| ns | 6461 |  | 101 | Relocation struct | 4.5 | 4.4 | 0.509 |
| walker |  | 6489 | 134 | Code::CodeKey { rung: Decl, file: chibicc.h, decl: 44, sub: 0, line: 301 } |  |  | 0.511 |
| ns | 6766 |  | 305 | NodeKind enum, first half (arithmetic through bit ops) | 4.6 |  | 0.497 |
| walker |  | 6811 | 322 | Code::CodeKey { rung: Decl, file: chibicc.h, decl: 36, sub: 0, line: 127 } |  |  | 0.544 |
| ns | 7114 |  | 348 | NodeKind enum, second half (control flow, calls, casts, atomics) | 4.7 | 4.6 | 0.531 |
| walker |  | 7243 | 432 | Code::CodeKey { rung: Decl, file: chibicc.h, decl: 45, sub: 0, line: 320 } |  |  | 0.534 |
| ns | 7441 |  | 327 | Node struct, first half (operands, control flow, calls) | 4.8 |  | 0.518 |
| walker |  | 7525 | 282 | Code::CodeKey { rung: Decl, file: chibicc.h, decl: 40, sub: 0, line: 228 } |  |  | 0.547 |
| ns | 7718 |  | 277 | Node struct, second half (goto/switch/case, asm, atomics, literals) | 4.9 | 4.8 | 0.532 |
| walker |  | 7749 | 224 | Code::CodeKey { rung: Decl, file: chibicc.h, decl: 39, sub: 0, line: 176 } |  |  | 0.548 |
| walker |  | 7755 | 6 | Code::CodeKey { rung: Doc, file: chibicc.h, decl: 39, sub: 0, line: 176 } |  |  | 0.549 |
| walker |  | 7761 | 6 | Code::CodeKey { rung: Doc, file: chibicc.h, decl: 46, sub: 0, line: 362 } |  |  | 0.550 |
| ns | 7871 |  | 153 | TypeKind enum | 4.10 |  | 0.559 |
| walker |  | 7978 | 217 | Code::CodeKey { rung: Decl, file: chibicc.h, decl: 39, sub: 1, line: 176 } |  |  | 0.575 |
| ns | 8147 |  | 276 | Type struct, first half + the pointer/array duality comment | 4.11 |  | 0.586 |
| walker |  | 8284 | 306 | Code::CodeKey { rung: Decl, file: chibicc.h, decl: 40, sub: 1, line: 228 } |  |  | 0.623 |
| ns | 8310 |  | 163 | Type struct, second half (array, VLA, struct, function members) | 4.12 | 4.11 | 0.629 |
| walker |  | 8383 | 99 | Markdown::Section { file: README.md, section_index: 11, keeps_default_concavity: false } |  |  | 0.629 |
| ns | 8435 |  | 125 | Member struct (struct/union members incl. bitfields) | 4.13 | 4.12 | 0.634 |
| ns | 8524 |  | 89 | HashEntry / HashMap structs | 4.14 | 2.9 | 0.629 |
| walker |  | 8606 | 223 | Code::CodeKey { rung: Names, file: include/stdatomic.h, decl: 0, sub: 1, line: 0 } |  |  | 0.629 |
| ns | 8615 |  | 91 | main(): driver entry and the -cc1 self-re-exec split | 5.1 | 3.4 | 0.624 |
| walker |  | 8800 | 194 | Code::CodeKey { rung: Names, file: codegen.c, decl: 0, sub: 0, line: 0 } |  |  | 0.624 |
| ns | 8913 |  | 298 | cc1(): the compile pipeline in one function | 5.2 | 5.1 | 0.610 |
| walker |  | 9037 | 237 | Code::CodeKey { rung: Names, file: main.c, decl: 0, sub: 1, line: 0 } |  |  | 0.617 |
| walker |  | 9046 | 9 | Code::CodeKey { rung: Doc, file: main.c, decl: 36, sub: 0, line: 367 } |  |  | 0.617 |
| ns | 9183 |  | 270 | parse(): the top-level program loop | 5.3 |  | 0.606 |
| walker |  | 9273 | 227 | Code::CodeKey { rung: Names, file: tokenize.c, decl: 0, sub: 1, line: 0 } |  |  | 0.617 |
| walker |  | 9297 | 24 | Code::CodeKey { rung: Doc, file: preprocess.c, decl: 8, sub: 0, line: 54 } |  |  | 0.617 |
| walker |  | 9314 | 17 | Code::CodeKey { rung: Body, file: hashmap.c, decl: 12, sub: 0, line: 117 } |  |  | 0.617 |
| ns | 9430 |  | 247 | The statement grammar (comment above stmt()) | 6.1 |  | 0.611 |
| walker |  | 9564 | 250 | Code::CodeKey { rung: Names, file: chibicc.h, decl: 0, sub: 3, line: 0 } |  |  | 0.634 |
| ns | 9756 |  | 326 | The expression precedence chain, as grammar comments | 6.2 | 3.9 | 0.627 |
| walker |  | 9793 | 229 | Code::CodeKey { rung: Names, file: type.c, decl: 0, sub: 1, line: 0 } |  |  | 0.628 |
| walker |  | 9812 | 19 | Code::CodeKey { rung: Body, file: type.c, decl: 17, sub: 0, line: 39 } |  |  | 0.628 |
| walker |  | 9845 | 33 | Code::CodeKey { rung: Body, file: type.c, decl: 16, sub: 0, line: 34 } |  |  | 0.628 |
| walker |  | 9891 | 46 | Code::CodeKey { rung: Body, file: type.c, decl: 19, sub: 0, line: 90 } |  |  | 0.628 |
| ns | 9989 |  | 233 | Every preprocessor directive chibicc handles | 6.3 |  | 0.622 |
