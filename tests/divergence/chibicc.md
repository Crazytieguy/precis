Score(3000)=0.639 I=0.821 C=0.497 ns_rows≤3K=23/51 grid(1000/1442/2080/3000/4327/6240/9000)=0.504/0.749/0.727/0.639/0.562/0.577/0.485

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
| walker |  | 2886 | 143 | Code::CodeKey { rung: Names, file: include/stdarg.h, decl: 0, sub: 0, line: 0 } |  |  | 0.639 |
| walker |  | 2909 | 23 | Code::CodeKey { rung: Decl, file: include/stdarg.h, decl: 3, sub: 0, line: 13 } |  |  | 0.639 |
| walker |  | 2949 | 40 | Code::CodeKey { rung: Decl, file: include/stdarg.h, decl: 1, sub: 0, line: 4 } |  |  | 0.639 |
| walker |  | 3062 | 113 | Code::CodeKey { rung: Decl, file: include/stdarg.h, decl: 5, sub: 0, line: 44 } |  |  | 0.639 |
| ns | 3100 |  | 364 | preprocess.c file-header comment: the hideset macro-expansion algorithm | 3.2 |  | 0.607 |
| walker |  | 3161 | 99 | Markdown::Section { file: README.md, section_index: 11, keeps_default_concavity: false } |  |  | 0.607 |
| ns | 3229 |  | 129 | Internal (non-exported) helpers of type.c, unicode.c and hashmap.c | 3.3 |  | 0.595 |
| walker |  | 3403 | 242 | Code::CodeKey { rung: Names, file: tokenize.c, decl: 0, sub: 0, line: 0 } |  |  | 0.595 |
| walker |  | 3412 | 9 | Code::CodeKey { rung: Doc, file: tokenize.c, decl: 5, sub: 0, line: 16 } |  |  | 0.595 |
| walker |  | 3430 | 18 | Code::CodeKey { rung: Decl, file: tokenize.c, decl: 6, sub: 0, line: 28 } |  |  | 0.595 |
| walker |  | 3444 | 14 | Code::CodeKey { rung: Doc, file: tokenize.c, decl: 11, sub: 0, line: 84 } |  |  | 0.595 |
| walker |  | 3460 | 16 | Code::CodeKey { rung: Doc, file: tokenize.c, decl: 10, sub: 0, line: 79 } |  |  | 0.595 |
| walker |  | 3468 | 8 | Code::CodeKey { rung: Doc, file: tokenize.c, decl: 1, sub: 0, line: 4 } |  |  | 0.595 |
| walker |  | 3478 | 10 | Code::CodeKey { rung: Doc, file: tokenize.c, decl: 2, sub: 0, line: 7 } |  |  | 0.595 |
| walker |  | 3488 | 10 | Code::CodeKey { rung: Doc, file: tokenize.c, decl: 13, sub: 0, line: 100 } |  |  | 0.595 |
| walker |  | 3501 | 13 | Code::CodeKey { rung: Doc, file: tokenize.c, decl: 4, sub: 0, line: 13 } |  |  | 0.595 |
| ns | 3561 |  | 332 | Function-name roster: main.c (the driver) | 3.4 |  | 0.564 |
| walker |  | 3712 | 211 | Code::CodeKey { rung: Names, file: type.c, decl: 0, sub: 0, line: 0 } |  |  | 0.564 |
| walker |  | 3728 | 16 | Code::CodeKey { rung: Doc, file: tokenize.c, decl: 3, sub: 0, line: 10 } |  |  | 0.564 |
| walker |  | 3930 | 202 | Code::CodeKey { rung: Names, file: chibicc.h, decl: 0, sub: 1, line: 0 } |  |  | 0.592 |
| ns | 3965 |  | 404 | Function-name roster: tokenize.c | 3.5 |  | 0.562 |
| walker |  | 4212 | 282 | Code::CodeKey { rung: Names, file: include/stdatomic.h, decl: 0, sub: 0, line: 0 } |  |  | 0.562 |
| walker |  | 4271 | 59 | Code::CodeKey { rung: Decl, file: include/stdatomic.h, decl: 11, sub: 0, line: 15 } |  |  | 0.562 |
| ns | 4346 |  | 381 | Function-name roster: codegen.c | 3.6 |  | 0.533 |
| walker |  | 4561 | 290 | Code::CodeKey { rung: Names, file: main.c, decl: 0, sub: 0, line: 0 } |  |  | 0.533 |
| walker |  | 4584 | 23 | Code::CodeKey { rung: Decl, file: main.c, decl: 1, sub: 0, line: 3 } |  |  | 0.533 |
| ns | 4756 |  | 410 | Function-name roster: preprocess.c, token and macro machinery | 3.7 |  | 0.509 |
| walker |  | 4808 | 224 | Markdown::Section { file: README.md, section_index: 4, keeps_default_concavity: false } |  |  | 0.537 |
| ns | 5022 |  | 266 | Function-name roster: preprocess.c, includes, directives and builtin macros | 3.8 | 3.7 | 0.522 |
| walker |  | 5040 | 232 | Markdown::Section { file: README.md, section_index: 5, keeps_default_concavity: true } |  |  | 0.539 |
| walker |  | 5227 | 187 | Markdown::Section { file: README.md, section_index: 6, keeps_default_concavity: false } |  |  | 0.577 |
| walker |  | 5358 | 131 | Code::CodeKey { rung: Names, file: unicode.c, decl: 0, sub: 0, line: 0 } |  |  | 0.577 |
| walker |  | 5372 | 14 | Code::CodeKey { rung: Doc, file: unicode.c, decl: 1, sub: 0, line: 4 } |  |  | 0.577 |
| walker |  | 5400 | 28 | Code::CodeKey { rung: Doc, file: unicode.c, decl: 5, sub: 0, line: 110 } |  |  | 0.577 |
| walker |  | 5428 | 28 | Code::CodeKey { rung: Doc, file: unicode.c, decl: 7, sub: 0, line: 181 } |  |  | 0.577 |
| walker |  | 5507 | 79 | Code::CodeKey { rung: Body, file: unicode.c, decl: 7, sub: 0, line: 181 } |  |  | 0.577 |
| ns | 5508 |  | 486 | parse.c forward-declaration block: the parser's function index | 3.9 |  | 0.543 |
| ns | 5631 |  | 123 | TokenKind enum | 4.1 |  | 0.552 |
| ns | 5707 |  | 76 | File struct | 4.2 |  | 0.559 |
| walker |  | 5856 | 349 | Code::CodeKey { rung: Names, file: include/float.h, decl: 0, sub: 1, line: 0 } |  |  | 0.559 |
| ns | 6011 |  | 304 | Token struct (all fields) | 4.3 |  | 0.576 |
| walker |  | 6142 | 286 | Code::CodeKey { rung: Names, file: preprocess.c, decl: 0, sub: 0, line: 0 } |  |  | 0.577 |
| walker |  | 6164 | 22 | Code::CodeKey { rung: Decl, file: preprocess.c, decl: 2, sub: 0, line: 28 } |  |  | 0.577 |
| walker |  | 6187 | 23 | Code::CodeKey { rung: Decl, file: preprocess.c, decl: 11, sub: 0, line: 63 } |  |  | 0.577 |
| walker |  | 6228 | 41 | Code::CodeKey { rung: Decl, file: preprocess.c, decl: 4, sub: 0, line: 34 } |  |  | 0.577 |
| walker |  | 6279 | 51 | Code::CodeKey { rung: Decl, file: preprocess.c, decl: 9, sub: 0, line: 55 } |  |  | 0.577 |
| walker |  | 6351 | 72 | Code::CodeKey { rung: Decl, file: preprocess.c, decl: 7, sub: 0, line: 44 } |  |  | 0.577 |
| ns | 6360 |  | 349 | Obj struct: variables and functions | 4.4 |  | 0.554 |
| walker |  | 6375 | 24 | Code::CodeKey { rung: Doc, file: preprocess.c, decl: 8, sub: 0, line: 54 } |  |  | 0.554 |
| ns | 6461 |  | 101 | Relocation struct | 4.5 | 4.4 | 0.548 |
| walker |  | 6476 | 101 | Code::CodeKey { rung: Body, file: strings.c, decl: 2, sub: 0, line: 20 } |  |  | 0.548 |
| walker |  | 6508 | 32 | Code::CodeKey { rung: Doc, file: preprocess.c, decl: 19, sub: 0, line: 82 } |  |  | 0.548 |
| walker |  | 6618 | 110 | Code::CodeKey { rung: Doc, file: unicode.c, decl: 2, sub: 0, line: 37 } |  |  | 0.548 |
| ns | 6766 |  | 305 | NodeKind enum, first half (arithmetic through bit ops) | 4.6 |  | 0.534 |
| walker |  | 6979 | 361 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.534 |
| ns | 7114 |  | 348 | NodeKind enum, second half (control flow, calls, casts, atomics) | 4.7 | 4.6 | 0.522 |
| walker |  | 7196 | 217 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.522 |
| ns | 7441 |  | 327 | Node struct, first half (operands, control flow, calls) | 4.8 |  | 0.506 |
| walker |  | 7456 | 260 | Markdown::Section { file: README.md, section_index: 7, keeps_default_concavity: false } |  |  | 0.506 |
| walker |  | 7700 | 244 | Markdown::Section { file: README.md, section_index: 8, keeps_default_concavity: false } |  |  | 0.506 |
| ns | 7718 |  | 277 | Node struct, second half (goto/switch/case, asm, atomics, literals) | 4.9 | 4.8 | 0.493 |
| ns | 7871 |  | 153 | TypeKind enum | 4.10 |  | 0.486 |
| walker |  | 7956 | 256 | Markdown::Section { file: README.md, section_index: 9, keeps_default_concavity: true } |  |  | 0.486 |
| ns | 8147 |  | 276 | Type struct, first half + the pointer/array duality comment | 4.11 |  | 0.477 |
| walker |  | 8288 | 332 | Code::CodeKey { rung: Names, file: hashmap.c, decl: 0, sub: 0, line: 0 } |  |  | 0.483 |
| walker |  | 8297 | 9 | Code::CodeKey { rung: Doc, file: hashmap.c, decl: 4, sub: 0, line: 15 } |  |  | 0.483 |
| walker |  | 8307 | 10 | Code::CodeKey { rung: Doc, file: hashmap.c, decl: 1, sub: 0, line: 6 } |  |  | 0.483 |
| ns | 8310 |  | 163 | Type struct, second half (array, VLA, struct, function members) | 4.12 | 4.11 | 0.476 |
| walker |  | 8319 | 12 | Code::CodeKey { rung: Doc, file: hashmap.c, decl: 2, sub: 0, line: 9 } |  |  | 0.476 |
| walker |  | 8334 | 15 | Code::CodeKey { rung: Body, file: hashmap.c, decl: 14, sub: 0, line: 126 } |  |  | 0.476 |
| walker |  | 8350 | 16 | Code::CodeKey { rung: Doc, file: hashmap.c, decl: 3, sub: 0, line: 12 } |  |  | 0.476 |
| walker |  | 8366 | 16 | Code::CodeKey { rung: Body, file: hashmap.c, decl: 10, sub: 0, line: 108 } |  |  | 0.476 |
| walker |  | 8383 | 17 | Code::CodeKey { rung: Body, file: hashmap.c, decl: 12, sub: 0, line: 117 } |  |  | 0.476 |
| walker |  | 8415 | 32 | Code::CodeKey { rung: Doc, file: hashmap.c, decl: 6, sub: 0, line: 28 } |  |  | 0.476 |
| ns | 8435 |  | 125 | Member struct (struct/union members incl. bitfields) | 4.13 | 4.12 | 0.471 |
| ns | 8524 |  | 89 | HashEntry / HashMap structs | 4.14 | 2.9 | 0.467 |
| ns | 8615 |  | 91 | main(): driver entry and the -cc1 self-re-exec split | 5.1 | 3.4 | 0.463 |
| walker |  | 8707 | 292 | Code::CodeKey { rung: Names, file: chibicc.h, decl: 0, sub: 2, line: 0 } |  |  | 0.474 |
| walker |  | 8728 | 21 | Code::CodeKey { rung: Decl, file: chibicc.h, decl: 29, sub: 0, line: 108 } |  |  | 0.477 |
| walker |  | 8769 | 41 | Code::CodeKey { rung: Decl, file: chibicc.h, decl: 38, sub: 0, line: 168 } |  |  | 0.480 |
| walker |  | 8778 | 9 | Code::CodeKey { rung: Doc, file: chibicc.h, decl: 35, sub: 0, line: 126 } |  |  | 0.480 |
| walker |  | 8888 | 110 | Code::CodeKey { rung: Decl, file: chibicc.h, decl: 46, sub: 0, line: 362 } |  |  | 0.494 |
| walker |  | 8896 | 8 | Code::CodeKey { rung: Doc, file: chibicc.h, decl: 46, sub: 0, line: 362 } |  |  | 0.496 |
| ns | 8913 |  | 298 | cc1(): the compile pipeline in one function | 5.2 | 5.1 | 0.485 |
| walker |  | 9030 | 134 | Code::CodeKey { rung: Decl, file: chibicc.h, decl: 44, sub: 0, line: 301 } |  |  | 0.504 |
| walker |  | 9071 | 41 | Code::CodeKey { rung: Doc, file: chibicc.h, decl: 37, sub: 0, line: 167 } |  |  | 0.511 |
| ns | 9183 |  | 270 | parse(): the top-level program loop | 5.3 |  | 0.502 |
| walker |  | 9391 | 320 | Code::CodeKey { rung: Decl, file: chibicc.h, decl: 36, sub: 0, line: 127 } |  |  | 0.539 |
| ns | 9430 |  | 247 | The statement grammar (comment above stmt()) | 6.1 |  | 0.534 |
| ns | 9756 |  | 326 | The expression precedence chain, as grammar comments | 6.2 | 3.9 | 0.528 |
| walker |  | 9821 | 430 | Code::CodeKey { rung: Decl, file: chibicc.h, decl: 45, sub: 0, line: 320 } |  |  | 0.564 |
| ns | 9989 |  | 233 | Every preprocessor directive chibicc handles | 6.3 |  | 0.559 |
