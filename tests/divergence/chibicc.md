Score(3000)=0.702 I=0.851 C=0.579 ns_rows≤3K=23/51 grid(1000/1442/2080/3000/4327/6240/9000)=0.753/0.750/0.737/0.702/0.597/0.652/0.735

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 51 | 51 | Fs::DirListing { dir: . } |  |  | 0.000 |
| ns | 75 |  | 75 | README title + what chibicc is | 1.1 |  | 0.000 |
| ns | 126 |  | 51 | Complete repository root listing | 1.2 |  | 0.580 |
| walker |  | 128 | 77 | Markdown::ReadmeHeadline { file: README.md } |  |  | 0.607 |
| walker |  | 164 | 36 | Fs::DirListing { dir: include } |  |  | 0.621 |
| walker |  | 223 | 59 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.647 |
| ns | 263 |  | 137 | README lede tail: real-world programs it compiles | 1.3 | 1.1 | 0.523 |
| ns | 356 |  | 93 | chibicc.h module section banners (the header's table of contents) | 1.4 |  | 0.440 |
| ns | 415 |  | 59 | All README H2 headings | 1.5 |  | 0.470 |
| ns | 528 |  | 113 | README Internals: tokenize and preprocess stages | 1.6 | 1.5 | 0.418 |
| ns | 602 |  | 74 | README Internals: parse and codegen stages | 1.7 | 1.6 | 0.392 |
| ns | 765 |  | 163 | Makefile: build flags and the chibicc target | 1.8 |  | 0.344 |
| walker |  | 848 | 625 | Plaintext::Whole { file: Makefile } |  |  | 0.484 |
| ns | 905 |  | 140 | Makefile: test targets | 1.9 | 1.8 | 0.504 |
| ns | 1003 |  | 98 | Makefile: stage-2 self-host targets and clean | 1.10 | 1.9 | 0.514 |
| ns | 1039 |  | 36 | Complete include/ listing (bundled freestanding headers) | 1.11 |  | 0.523 |
| walker |  | 1042 | 194 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.813 |
| walker |  | 1253 | 211 | Fs::DirListing { dir: test } |  |  | 0.844 |
| ns | 1273 |  | 234 | README Status: supported and unsupported C11 features | 1.12 | 1.5 | 0.749 |
| walker |  | 1444 | 191 | Code::CodeKey { rung: Names, file: chibicc.h, decl: 0, sub: 0, line: 0 } |  |  | 0.750 |
| walker |  | 1467 | 23 | Code::CodeKey { rung: Decl, file: chibicc.h, decl: 9, sub: 0, line: 38 } |  |  | 0.752 |
| ns | 1484 |  | 211 | Complete test/ listing | 1.13 |  | 0.795 |
| ns | 1569 |  | 85 | The four stage entry points declared in chibicc.h | 2.1 |  | 0.779 |
| walker |  | 1647 | 180 | Code::CodeKey { rung: Names, file: chibicc.h, decl: 0, sub: 1, line: 0 } |  |  | 0.780 |
| walker |  | 1707 | 60 | Code::CodeKey { rung: Decl, file: chibicc.h, decl: 13, sub: 0, line: 62 } |  |  | 0.781 |
| ns | 1798 |  | 229 | chibicc.h: rest of the tokenize.c public API | 2.2 | 2.1 | 0.759 |
| walker |  | 1804 | 97 | Code::CodeKey { rung: Decl, file: chibicc.h, decl: 12, sub: 0, line: 52 } |  |  | 0.761 |
| walker |  | 1810 | 6 | Code::CodeKey { rung: Doc, file: chibicc.h, decl: 14, sub: 0, line: 73 } |  |  | 0.761 |
| walker |  | 1817 | 7 | Code::CodeKey { rung: Doc, file: chibicc.h, decl: 12, sub: 0, line: 52 } |  |  | 0.761 |
| ns | 1877 |  | 79 | chibicc.h: rest of the preprocess.c API + the unreachable() macro | 2.3 | 2.1 | 0.747 |
| ns | 1966 |  | 89 | chibicc.h: parse.c's other exports + the main.c section | 2.4 | 2.1 | 0.728 |
| ns | 2055 |  | 89 | chibicc.h: strings.c section (StringArray + strarray_push + format) | 2.5 |  | 0.735 |
| walker |  | 2097 | 280 | Code::CodeKey { rung: Decl, file: chibicc.h, decl: 15, sub: 0, line: 74 } |  |  | 0.738 |
| ns | 2210 |  | 155 | chibicc.h: type.c function API | 2.6 |  | 0.713 |
| walker |  | 2285 | 188 | Code::CodeKey { rung: Names, file: chibicc.h, decl: 0, sub: 2, line: 0 } |  |  | 0.751 |
| ns | 2358 |  | 148 | chibicc.h: the extern singleton Types | 2.7 | 2.6 | 0.720 |
| ns | 2436 |  | 78 | chibicc.h: unicode.c API | 2.8 |  | 0.711 |
| walker |  | 2528 | 243 | Code::CodeKey { rung: Names, file: chibicc.h, decl: 0, sub: 3, line: 0 } |  |  | 0.731 |
| walker |  | 2569 | 41 | Code::CodeKey { rung: Decl, file: chibicc.h, decl: 38, sub: 0, line: 168 } |  |  | 0.732 |
| ns | 2570 |  | 134 | chibicc.h: hashmap.c API | 2.9 |  | 0.719 |
| walker |  | 2703 | 134 | Code::CodeKey { rung: Decl, file: chibicc.h, decl: 44, sub: 0, line: 301 } |  |  | 0.721 |
| ns | 2736 |  | 166 | parse.c file-header comment: how to read the parser | 3.1 |  | 0.699 |
| walker |  | 2813 | 110 | Code::CodeKey { rung: Decl, file: chibicc.h, decl: 46, sub: 0, line: 362 } |  |  | 0.701 |
| walker |  | 2989 | 176 | Code::CodeKey { rung: Decl, file: chibicc.h, decl: 39, sub: 0, line: 176 } |  |  | 0.702 |
| ns | 3100 |  | 364 | preprocess.c file-header comment: the hideset macro-expansion algorithm | 3.2 |  | 0.667 |
| walker |  | 3170 | 181 | Code::CodeKey { rung: Decl, file: chibicc.h, decl: 39, sub: 1, line: 176 } |  |  | 0.669 |
| ns | 3229 |  | 129 | Internal (non-exported) helpers of type.c, unicode.c and hashmap.c | 3.3 |  | 0.656 |
| walker |  | 3441 | 271 | Code::CodeKey { rung: Decl, file: chibicc.h, decl: 39, sub: 2, line: 176 } |  |  | 0.658 |
| ns | 3561 |  | 332 | Function-name roster: main.c (the driver) | 3.4 |  | 0.624 |
| walker |  | 3763 | 322 | Code::CodeKey { rung: Decl, file: chibicc.h, decl: 36, sub: 0, line: 127 } |  |  | 0.628 |
| walker |  | 3769 | 6 | Code::CodeKey { rung: Doc, file: chibicc.h, decl: 39, sub: 0, line: 176 } |  |  | 0.628 |
| ns | 3965 |  | 404 | Function-name roster: tokenize.c | 3.5 |  | 0.592 |
| walker |  | 3978 | 209 | Code::CodeKey { rung: Decl, file: chibicc.h, decl: 40, sub: 0, line: 228 } |  |  | 0.593 |
| walker |  | 3985 | 7 | Code::CodeKey { rung: Doc, file: chibicc.h, decl: 40, sub: 0, line: 228 } |  |  | 0.593 |
| ns | 4346 |  | 381 | Function-name roster: codegen.c | 3.6 |  | 0.563 |
| walker |  | 4359 | 374 | Code::CodeKey { rung: Decl, file: chibicc.h, decl: 40, sub: 1, line: 228 } |  |  | 0.567 |
| walker |  | 4367 | 8 | Code::CodeKey { rung: Doc, file: chibicc.h, decl: 46, sub: 0, line: 362 } |  |  | 0.567 |
| walker |  | 4376 | 9 | Code::CodeKey { rung: Doc, file: chibicc.h, decl: 35, sub: 0, line: 126 } |  |  | 0.568 |
| ns | 4756 |  | 410 | Function-name roster: preprocess.c, token and macro machinery | 3.7 |  | 0.542 |
| walker |  | 4806 | 430 | Code::CodeKey { rung: Decl, file: chibicc.h, decl: 45, sub: 0, line: 320 } |  |  | 0.545 |
| walker |  | 5011 | 205 | Code::CodeKey { rung: Names, file: chibicc.h, decl: 0, sub: 4, line: 0 } |  |  | 0.583 |
| ns | 5022 |  | 266 | Function-name roster: preprocess.c, includes, directives and builtin macros | 3.8 | 3.7 | 0.567 |
| walker |  | 5215 | 204 | Code::CodeKey { rung: Names, file: chibicc.h, decl: 0, sub: 5, line: 0 } |  |  | 0.593 |
| walker |  | 5240 | 25 | Code::CodeKey { rung: Decl, file: chibicc.h, decl: 79, sub: 0, line: 428 } |  |  | 0.593 |
| walker |  | 5265 | 25 | Code::CodeKey { rung: Decl, file: chibicc.h, decl: 80, sub: 0, line: 434 } |  |  | 0.593 |
| walker |  | 5435 | 170 | Code::CodeKey { rung: Names, file: chibicc.h, decl: 0, sub: 6, line: 0 } |  |  | 0.620 |
| ns | 5508 |  | 486 | parse.c forward-declaration block: the parser's function index | 3.9 |  | 0.583 |
| walker |  | 5602 | 167 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.583 |
| ns | 5631 |  | 123 | TokenKind enum | 4.1 |  | 0.591 |
| walker |  | 5701 | 99 | Markdown::Section { file: README.md, section_index: 10, keeps_default_concavity: false } |  |  | 0.591 |
| ns | 5707 |  | 76 | File struct | 4.2 |  | 0.597 |
| walker |  | 5925 | 224 | Markdown::Section { file: README.md, section_index: 4, keeps_default_concavity: false } |  |  | 0.620 |
| ns | 6011 |  | 304 | Token struct (all fields) | 4.3 |  | 0.633 |
| walker |  | 6157 | 232 | Markdown::Section { file: README.md, section_index: 5, keeps_default_concavity: false } |  |  | 0.648 |
| walker |  | 6344 | 187 | Markdown::Section { file: README.md, section_index: 6, keeps_default_concavity: false } |  |  | 0.681 |
| ns | 6360 |  | 349 | Obj struct: variables and functions | 4.4 |  | 0.699 |
| walker |  | 6383 | 39 | Code::CodeKey { rung: Doc, file: chibicc.h, decl: 37, sub: 0, line: 167 } |  |  | 0.699 |
| ns | 6461 |  | 101 | Relocation struct | 4.5 | 4.4 | 0.704 |
| walker |  | 6608 | 225 | Code::CodeKey { rung: Names, file: include/stdatomic.h, decl: 0, sub: 0, line: 0 } |  |  | 0.704 |
| walker |  | 6667 | 59 | Code::CodeKey { rung: Decl, file: include/stdatomic.h, decl: 11, sub: 0, line: 15 } |  |  | 0.704 |
| ns | 6766 |  | 305 | NodeKind enum, first half (arithmetic through bit ops) | 4.6 |  | 0.714 |
| walker |  | 6810 | 143 | Code::CodeKey { rung: Names, file: include/stdarg.h, decl: 0, sub: 0, line: 0 } |  |  | 0.714 |
| walker |  | 6850 | 40 | Code::CodeKey { rung: Decl, file: include/stdarg.h, decl: 1, sub: 0, line: 4 } |  |  | 0.714 |
| walker |  | 7036 | 186 | Code::CodeKey { rung: Names, file: include/stdatomic.h, decl: 0, sub: 1, line: 0 } |  |  | 0.714 |
| ns | 7114 |  | 348 | NodeKind enum, second half (control flow, calls, casts, atomics) | 4.7 | 4.6 | 0.723 |
| walker |  | 7203 | 167 | Code::CodeKey { rung: Names, file: include/stdatomic.h, decl: 0, sub: 2, line: 0 } |  |  | 0.723 |
| walker |  | 7382 | 179 | Code::CodeKey { rung: Names, file: include/stdatomic.h, decl: 0, sub: 3, line: 0 } |  |  | 0.723 |
| ns | 7441 |  | 327 | Node struct, first half (operands, control flow, calls) | 4.8 |  | 0.732 |
| walker |  | 7568 | 186 | Code::CodeKey { rung: Names, file: include/stdatomic.h, decl: 0, sub: 4, line: 0 } |  |  | 0.732 |
| ns | 7718 |  | 277 | Node struct, second half (goto/switch/case, asm, atomics, literals) | 4.9 | 4.8 | 0.740 |
| walker |  | 7736 | 168 | Code::CodeKey { rung: Names, file: include/stdatomic.h, decl: 0, sub: 5, line: 0 } |  |  | 0.740 |
| ns | 7871 |  | 153 | TypeKind enum | 4.10 |  | 0.744 |
| walker |  | 7903 | 167 | Code::CodeKey { rung: Names, file: include/stdatomic.h, decl: 0, sub: 6, line: 0 } |  |  | 0.744 |
| walker |  | 8108 | 205 | Code::CodeKey { rung: Names, file: include/float.h, decl: 0, sub: 0, line: 0 } |  |  | 0.744 |
| ns | 8147 |  | 276 | Type struct, first half + the pointer/array duality comment | 4.11 |  | 0.749 |
| walker |  | 8196 | 88 | Code::CodeKey { rung: Names, file: include/stddef.h, decl: 0, sub: 0, line: 0 } |  |  | 0.749 |
| walker |  | 8252 | 56 | Code::CodeKey { rung: Names, file: include/stdalign.h, decl: 0, sub: 0, line: 0 } |  |  | 0.749 |
| walker |  | 8301 | 49 | Code::CodeKey { rung: Names, file: include/stdbool.h, decl: 0, sub: 0, line: 0 } |  |  | 0.749 |
| ns | 8310 |  | 163 | Type struct, second half (array, VLA, struct, function members) | 4.12 | 4.11 | 0.753 |
| walker |  | 8318 | 17 | Code::CodeKey { rung: Names, file: include/stdnoreturn.h, decl: 0, sub: 0, line: 0 } |  |  | 0.753 |
| ns | 8435 |  | 125 | Member struct (struct/union members incl. bitfields) | 4.13 | 4.12 | 0.756 |
| ns | 8524 |  | 89 | HashEntry / HashMap structs | 4.14 | 2.9 | 0.758 |
| walker |  | 8536 | 218 | Code::CodeKey { rung: Names, file: include/float.h, decl: 0, sub: 1, line: 0 } |  |  | 0.758 |
| ns | 8615 |  | 91 | main(): driver entry and the -cc1 self-re-exec split | 5.1 | 3.4 | 0.752 |
| walker |  | 8713 | 177 | Code::CodeKey { rung: Names, file: include/float.h, decl: 0, sub: 2, line: 0 } |  |  | 0.752 |
| ns | 8913 |  | 298 | cc1(): the compile pipeline in one function | 5.2 | 5.1 | 0.735 |
| walker |  | 9074 | 361 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.735 |
| ns | 9183 |  | 270 | parse(): the top-level program loop | 5.3 |  | 0.723 |
| walker |  | 9291 | 217 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.723 |
| ns | 9430 |  | 247 | The statement grammar (comment above stmt()) | 6.1 |  | 0.716 |
| walker |  | 9535 | 244 | Markdown::Section { file: README.md, section_index: 7, keeps_default_concavity: false } |  |  | 0.716 |
| ns | 9756 |  | 326 | The expression precedence chain, as grammar comments | 6.2 | 3.9 | 0.708 |
| walker |  | 9791 | 256 | Markdown::Section { file: README.md, section_index: 8, keeps_default_concavity: false } |  |  | 0.708 |
| walker |  | 9814 | 23 | Code::CodeKey { rung: Body, file: include/stdarg.h, decl: 3, sub: 0, line: 13 } |  |  | 0.708 |
| ns | 9989 |  | 233 | Every preprocessor directive chibicc handles | 6.3 |  | 0.701 |
| walker |  | 9996 | 182 | Markdown::Section { file: README.md, section_index: 9, keeps_default_concavity: false } |  |  | 0.701 |
