Score(3000)=0.694 I=0.839 C=0.573 ns_rows≤3K=23/51 grid(1000/1442/2080/3000/4327/6240/9000)=0.504/0.749/0.747/0.694/0.589/0.645/0.673

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 51 | 51 | listing of '.' |  |  | 0.000 |
| ns | 75 |  | 75 | README title + what chibicc is | 1.1 |  | 0.000 |
| walker |  | 87 | 36 | listing of 'include' |  |  | 0.000 |
| ns | 126 |  | 51 | Complete repository root listing | 1.2 |  | 0.595 |
| walker |  | 164 | 77 | README headline in README.md |  |  | 0.621 |
| walker |  | 223 | 59 | headings outline in README.md |  |  | 0.647 |
| ns | 263 |  | 137 | README lede tail: real-world programs it compiles | 1.3 | 1.1 | 0.523 |
| ns | 356 |  | 93 | chibicc.h module section banners (the header's table of contents) | 1.4 |  | 0.440 |
| ns | 415 |  | 59 | All README H2 headings | 1.5 |  | 0.470 |
| ns | 528 |  | 113 | README Internals: tokenize and preprocess stages | 1.6 | 1.5 | 0.418 |
| ns | 602 |  | 74 | README Internals: parse and codegen stages | 1.7 | 1.6 | 0.392 |
| ns | 765 |  | 163 | Makefile: build flags and the chibicc target | 1.8 |  | 0.344 |
| walker |  | 905 | 682 | plaintext config Makefile |  |  | 0.504 |
| ns | 905 |  | 140 | Makefile: test targets | 1.9 | 1.8 | 0.504 |
| ns | 1003 |  | 98 | Makefile: stage-2 self-host targets and clean | 1.10 | 1.9 | 0.514 |
| ns | 1039 |  | 36 | Complete include/ listing (bundled freestanding headers) | 1.11 |  | 0.523 |
| walker |  | 1116 | 211 | listing of 'test' |  |  | 0.563 |
| ns | 1273 |  | 234 | README Status: supported and unsupported C11 features | 1.12 | 1.5 | 0.499 |
| walker |  | 1310 | 194 | README.md section #0 |  |  | 0.749 |
| walker |  | 1477 | 167 | README.md section #1 |  |  | 0.749 |
| ns | 1484 |  | 211 | Complete test/ listing | 1.13 |  | 0.792 |
| ns | 1569 |  | 85 | The four stage entry points declared in chibicc.h | 2.1 |  | 0.775 |
| ns | 1798 |  | 229 | chibicc.h: rest of the tokenize.c public API | 2.2 | 2.1 | 0.747 |
| ns | 1877 |  | 79 | chibicc.h: rest of the preprocess.c API + the unreachable() macro | 2.3 | 2.1 | 0.733 |
| walker |  | 1897 | 420 | c decl names surface in chibicc.h |  |  | 0.760 |
| walker |  | 1897 | 0 | c decl at chibicc.h:73 |  |  | 0.760 |
| walker |  | 1928 | 31 | c decl at chibicc.h:38 |  |  | 0.762 |
| walker |  | 1936 | 8 | c decl doc at chibicc.h:73 |  |  | 0.762 |
| ns | 1966 |  | 89 | chibicc.h: parse.c's other exports + the main.c section | 2.4 | 2.1 | 0.743 |
| walker |  | 2003 | 67 | c decl at chibicc.h:62 |  |  | 0.744 |
| ns | 2055 |  | 89 | chibicc.h: strings.c section (StringArray + strarray_push + format) | 2.5 |  | 0.747 |
| walker |  | 2108 | 105 | c decl at chibicc.h:52 |  |  | 0.748 |
| walker |  | 2115 | 7 | c decl doc at chibicc.h:52 |  |  | 0.748 |
| ns | 2210 |  | 155 | chibicc.h: type.c function API | 2.6 |  | 0.724 |
| ns | 2358 |  | 148 | chibicc.h: the extern singleton Types | 2.7 | 2.6 | 0.694 |
| walker |  | 2430 | 315 | c decl names surface #1 in chibicc.h |  |  | 0.727 |
| walker |  | 2430 | 0 | c decl at chibicc.h:126 |  |  | 0.727 |
| walker |  | 2430 | 0 | c decl at chibicc.h:167 |  |  | 0.727 |
| walker |  | 2435 | 5 | c decl at chibicc.h:228 |  |  | 0.727 |
| ns | 2436 |  | 78 | chibicc.h: unicode.c API | 2.8 |  | 0.718 |
| walker |  | 2442 | 7 | c decl at chibicc.h:127 |  |  | 0.718 |
| walker |  | 2449 | 7 | c decl at chibicc.h:320 |  |  | 0.718 |
| walker |  | 2459 | 10 | c decl at chibicc.h:176 |  |  | 0.718 |
| walker |  | 2480 | 21 | c decl at chibicc.h:108 |  |  | 0.724 |
| walker |  | 2521 | 41 | c decl at chibicc.h:168 |  |  | 0.725 |
| walker |  | 2527 | 6 | c decl doc at chibicc.h:176 |  |  | 0.725 |
| walker |  | 2534 | 7 | c decl doc at chibicc.h:228 |  |  | 0.725 |
| ns | 2570 |  | 134 | chibicc.h: hashmap.c API | 2.9 |  | 0.713 |
| walker |  | 2581 | 47 | c aggregate member group at chibicc.h:228 group 238 |  |  | 0.713 |
| walker |  | 2691 | 110 | c decl at chibicc.h:362 |  |  | 0.714 |
| ns | 2736 |  | 166 | parse.c file-header comment: how to read the parser | 3.1 |  | 0.693 |
| walker |  | 2744 | 53 | c aggregate member group at chibicc.h:228 group 261 |  |  | 0.693 |
| walker |  | 2797 | 53 | c aggregate member group at chibicc.h:228 group 282 |  |  | 0.693 |
| walker |  | 2859 | 62 | c aggregate member group at chibicc.h:228 group 270 |  |  | 0.694 |
| walker |  | 3001 | 142 | c decl at chibicc.h:301 |  |  | 0.695 |
| walker |  | 3070 | 69 | c aggregate member group at chibicc.h:320 group 350 |  |  | 0.696 |
| ns | 3100 |  | 364 | preprocess.c file-header comment: the hideset macro-expansion algorithm | 3.2 |  | 0.662 |
| walker |  | 3143 | 73 | c aggregate member group at chibicc.h:320 group 321 |  |  | 0.662 |
| walker |  | 3220 | 77 | c aggregate member group at chibicc.h:127 group 128 |  |  | 0.662 |
| ns | 3229 |  | 129 | Internal (non-exported) helpers of type.c, unicode.c and hashmap.c | 3.3 |  | 0.649 |
| walker |  | 3297 | 77 | c aggregate member group at chibicc.h:320 group 336 |  |  | 0.649 |
| walker |  | 3378 | 81 | c aggregate member group at chibicc.h:127 group 136 |  |  | 0.650 |
| walker |  | 3463 | 85 | c aggregate member group at chibicc.h:228 group 229 |  |  | 0.651 |
| walker |  | 3549 | 86 | c aggregate member group at chibicc.h:228 group 245 |  |  | 0.652 |
| ns | 3561 |  | 332 | Function-name roster: main.c (the driver) | 3.4 |  | 0.618 |
| walker |  | 3644 | 95 | c aggregate member group at chibicc.h:127 group 150 |  |  | 0.619 |
| walker |  | 3654 | 10 | c whole header in test/include3.h |  |  | 0.619 |
| walker |  | 3664 | 10 | c whole header in test/include4.h |  |  | 0.619 |
| walker |  | 3675 | 11 | c whole header in test/include2.h |  |  | 0.619 |
| walker |  | 3941 | 266 | c decl at chibicc.h:74 |  |  | 0.621 |
| ns | 3965 |  | 404 | Function-name roster: tokenize.c | 3.5 |  | 0.586 |
| walker |  | 3978 | 37 | c decl names surface in strings.c |  |  | 0.586 |
| walker |  | 3978 | 0 | c decl at strings.c:3 |  |  | 0.586 |
| walker |  | 3978 | 0 | c decl at strings.c:20 |  |  | 0.586 |
| walker |  | 4120 | 142 | c aggregate member group at chibicc.h:176 group 177 |  |  | 0.586 |
| walker |  | 4268 | 148 | c aggregate member group at chibicc.h:176 group 189 |  |  | 0.588 |
| walker |  | 4308 | 40 | c decl names surface #1 in unicode.c |  |  | 0.589 |
| walker |  | 4308 | 0 | c decl at unicode.c:70 |  |  | 0.589 |
| walker |  | 4308 | 0 | c decl at unicode.c:123 |  |  | 0.589 |
| ns | 4346 |  | 381 | Function-name roster: codegen.c | 3.6 |  | 0.559 |
| walker |  | 4407 | 99 | README.md section #10 |  |  | 0.559 |
| walker |  | 4706 | 299 | c decl names surface #2 in chibicc.h |  |  | 0.620 |
| ns | 4756 |  | 410 | Function-name roster: preprocess.c, token and macro machinery | 3.7 |  | 0.592 |
| walker |  | 4893 | 187 | c includes in chibicc.h |  |  | 0.592 |
| ns | 5022 |  | 266 | Function-name roster: preprocess.c, includes, directives and builtin macros | 3.8 | 3.7 | 0.575 |
| walker |  | 5058 | 165 | c aggregate member group at chibicc.h:176 group 201 |  |  | 0.576 |
| walker |  | 5231 | 173 | c aggregate member group at chibicc.h:176 group 213 |  |  | 0.578 |
| walker |  | 5241 | 10 | c includes in strings.c |  |  | 0.578 |
| walker |  | 5428 | 187 | README.md section #5 |  |  | 0.615 |
| ns | 5508 |  | 486 | parse.c forward-declaration block: the parser's function index | 3.9 |  | 0.579 |
| ns | 5631 |  | 123 | TokenKind enum | 4.1 |  | 0.587 |
| ns | 5707 |  | 76 | File struct | 4.2 |  | 0.594 |
| walker |  | 5727 | 299 | c decl names surface #3 in chibicc.h |  |  | 0.628 |
| walker |  | 5760 | 33 | c decl at chibicc.h:428 |  |  | 0.628 |
| walker |  | 5793 | 33 | c decl at chibicc.h:434 |  |  | 0.629 |
| walker |  | 5861 | 68 | c decl names surface #1 in type.c |  |  | 0.632 |
| walker |  | 5861 | 0 | c decl at type.c:20 |  |  | 0.632 |
| walker |  | 5861 | 0 | c decl at type.c:134 |  |  | 0.632 |
| walker |  | 5861 | 0 | c decl at type.c:170 |  |  | 0.632 |
| walker |  | 5917 | 56 | c whole header in include/stdnoreturn.h |  |  | 0.632 |
| walker |  | 6000 | 83 | c decl names surface in main.c |  |  | 0.632 |
| walker |  | 6000 | 0 | c decl at main.c:586 |  |  | 0.632 |
| walker |  | 6000 | 0 | c decl at main.c:700 |  |  | 0.632 |
| walker |  | 6010 | 10 | c includes in main.c |  |  | 0.632 |
| ns | 6011 |  | 304 | Token struct (all fields) | 4.3 |  | 0.645 |
| walker |  | 6041 | 31 | c decl at main.c:3 |  |  | 0.645 |
| walker |  | 6132 | 91 | c decl names surface in unicode.c |  |  | 0.645 |
| walker |  | 6132 | 0 | c decl at unicode.c:4 |  |  | 0.645 |
| walker |  | 6132 | 0 | c decl at unicode.c:37 |  |  | 0.645 |
| walker |  | 6132 | 0 | c decl at unicode.c:87 |  |  | 0.645 |
| walker |  | 6132 | 0 | c decl at unicode.c:110 |  |  | 0.645 |
| walker |  | 6132 | 0 | c decl at unicode.c:181 |  |  | 0.645 |
| walker |  | 6144 | 12 | c includes in unicode.c |  |  | 0.645 |
| walker |  | 6156 | 12 | c decl doc at unicode.c:4 |  |  | 0.645 |
| walker |  | 6169 | 13 | c decl doc at main.c:586 |  |  | 0.645 |
| ns | 6360 |  | 349 | Obj struct: variables and functions | 4.4 |  | 0.645 |
| walker |  | 6429 | 260 | README.md section #6 |  |  | 0.645 |
| walker |  | 6435 | 6 | c decl doc at chibicc.h:362 |  |  | 0.645 |
| ns | 6461 |  | 101 | Relocation struct | 4.5 | 4.4 | 0.642 |
| walker |  | 6551 | 116 | c decl names surface #1 in hashmap.c |  |  | 0.653 |
| walker |  | 6551 | 0 | c decl at hashmap.c:17 |  |  | 0.653 |
| walker |  | 6551 | 0 | c decl at hashmap.c:28 |  |  | 0.653 |
| walker |  | 6551 | 0 | c decl at hashmap.c:55 |  |  | 0.653 |
| walker |  | 6551 | 0 | c decl at hashmap.c:60 |  |  | 0.653 |
| walker |  | 6551 | 0 | c decl at hashmap.c:76 |  |  | 0.653 |
| walker |  | 6568 | 17 | c header banner in hashmap.c |  |  | 0.653 |
| ns | 6766 |  | 305 | NodeKind enum, first half (arithmetic through bit ops) | 4.6 |  | 0.665 |
| walker |  | 6812 | 244 | README.md section #7 |  |  | 0.665 |
| walker |  | 7068 | 256 | README.md section #8 |  |  | 0.665 |
| ns | 7114 |  | 348 | NodeKind enum, second half (control flow, calls, casts, atomics) | 4.7 | 4.6 | 0.675 |
| walker |  | 7154 | 86 | c whole header in include/stdbool.h |  |  | 0.675 |
| walker |  | 7245 | 91 | c whole header in include/stdalign.h |  |  | 0.675 |
| walker |  | 7383 | 138 | c decl names surface in parse.c |  |  | 0.675 |
| walker |  | 7383 | 0 | c decl at parse.c:31 |  |  | 0.675 |
| walker |  | 7383 | 0 | c decl at parse.c:54 |  |  | 0.675 |
| walker |  | 7383 | 0 | c decl at parse.c:75 |  |  | 0.675 |
| walker |  | 7383 | 0 | c decl at parse.c:244 |  |  | 0.675 |
| walker |  | 7383 | 0 | c decl at parse.c:1987 |  |  | 0.675 |
| walker |  | 7383 | 0 | c decl at parse.c:3337 |  |  | 0.675 |
| walker |  | 7397 | 14 | c includes in parse.c |  |  | 0.675 |
| walker |  | 7439 | 42 | c decl at parse.c:76 |  |  | 0.675 |
| ns | 7441 |  | 327 | Node struct, first half (operands, control flow, calls) | 4.8 |  | 0.671 |
| walker |  | 7485 | 46 | c decl at parse.c:23 |  |  | 0.671 |
| walker |  | 7493 | 8 | c decl doc at parse.c:31 |  |  | 0.671 |
| walker |  | 7556 | 63 | c decl at parse.c:42 |  |  | 0.671 |
| walker |  | 7631 | 75 | c decl at parse.c:32 |  |  | 0.671 |
| ns | 7718 |  | 277 | Node struct, second half (goto/switch/case, asm, atomics, literals) | 4.9 | 4.8 | 0.662 |
| walker |  | 7776 | 145 | c decl names surface in codegen.c |  |  | 0.662 |
| walker |  | 7776 | 0 | c decl at codegen.c:55 |  |  | 0.662 |
| walker |  | 7776 | 0 | c decl at codegen.c:1585 |  |  | 0.662 |
| walker |  | 7786 | 10 | c includes in codegen.c |  |  | 0.662 |
| walker |  | 7805 | 19 | c decl body at codegen.c:55 |  |  | 0.662 |
| ns | 7871 |  | 153 | TypeKind enum | 4.10 |  | 0.669 |
| ns | 8147 |  | 276 | Type struct, first half + the pointer/array duality comment | 4.11 |  | 0.660 |
| walker |  | 8266 | 461 | README.md section #4 |  |  | 0.689 |
| ns | 8310 |  | 163 | Type struct, second half (array, VLA, struct, function members) | 4.12 | 4.11 | 0.687 |
| walker |  | 8323 | 57 | c whole header in test/include1.h |  |  | 0.687 |
| walker |  | 8345 | 22 | c decl body at main.c:586 |  |  | 0.687 |
| walker |  | 8362 | 17 | c decl doc at strings.c:20 |  |  | 0.687 |
| ns | 8435 |  | 125 | Member struct (struct/union members incl. bitfields) | 4.13 | 4.12 | 0.691 |
| walker |  | 8485 | 123 | c whole header in include/stddef.h |  |  | 0.691 |
| ns | 8524 |  | 89 | HashEntry / HashMap structs | 4.14 | 2.9 | 0.693 |
| ns | 8615 |  | 91 | main(): driver entry and the -cc1 self-re-exec split | 5.1 | 3.4 | 0.688 |
| walker |  | 8846 | 361 | README.md section #2 |  |  | 0.688 |
| ns | 8913 |  | 298 | cc1(): the compile pipeline in one function | 5.2 | 5.1 | 0.673 |
| walker |  | 9063 | 217 | README.md section #3 |  |  | 0.673 |
| walker |  | 9073 | 10 | c decl doc at parse.c:75 |  |  | 0.673 |
| walker |  | 9082 | 9 | c decl doc at chibicc.h:126 |  |  | 0.675 |
| ns | 9183 |  | 270 | parse(): the top-level program loop | 5.3 |  | 0.663 |
| walker |  | 9293 | 211 | c decl names surface in hashmap.c |  |  | 0.663 |
| walker |  | 9293 | 0 | c decl at hashmap.c:6 |  |  | 0.663 |
| walker |  | 9293 | 0 | c decl at hashmap.c:9 |  |  | 0.663 |
| walker |  | 9293 | 0 | c decl at hashmap.c:12 |  |  | 0.663 |
| walker |  | 9293 | 0 | c decl at hashmap.c:15 |  |  | 0.663 |
| walker |  | 9293 | 0 | c decl at hashmap.c:108 |  |  | 0.663 |
| walker |  | 9293 | 0 | c decl at hashmap.c:112 |  |  | 0.663 |
| walker |  | 9293 | 0 | c decl at hashmap.c:117 |  |  | 0.663 |
| walker |  | 9293 | 0 | c decl at hashmap.c:121 |  |  | 0.663 |
| walker |  | 9293 | 0 | c decl at hashmap.c:126 |  |  | 0.663 |
| walker |  | 9293 | 0 | c decl at hashmap.c:130 |  |  | 0.663 |
| walker |  | 9293 | 0 | c decl at hashmap.c:136 |  |  | 0.663 |
| walker |  | 9305 | 12 | c includes in hashmap.c |  |  | 0.663 |
| walker |  | 9313 | 8 | c decl doc at hashmap.c:6 |  |  | 0.663 |
| walker |  | 9322 | 9 | c decl doc at hashmap.c:15 |  |  | 0.663 |
| walker |  | 9337 | 15 | c decl body at hashmap.c:126 |  |  | 0.663 |
| walker |  | 9353 | 16 | c decl body at hashmap.c:108 |  |  | 0.663 |
| ns | 9430 |  | 247 | The statement grammar (comment above stmt()) | 6.1 |  | 0.657 |
| walker |  | 9565 | 212 | c decl names surface in preprocess.c |  |  | 0.657 |
| walker |  | 9565 | 0 | c decl at preprocess.c:54 |  |  | 0.657 |
| walker |  | 9565 | 0 | c decl at preprocess.c:685 |  |  | 0.657 |
| walker |  | 9565 | 0 | c decl at preprocess.c:993 |  |  | 0.657 |
| walker |  | 9565 | 0 | c decl at preprocess.c:998 |  |  | 0.657 |
| walker |  | 9565 | 0 | c decl at preprocess.c:1060 |  |  | 0.657 |
| walker |  | 9565 | 0 | c decl at preprocess.c:1198 |  |  | 0.657 |
| walker |  | 9587 | 22 | c decl at preprocess.c:28 |  |  | 0.657 |
| walker |  | 9612 | 25 | c decl at preprocess.c:63 |  |  | 0.657 |
| walker |  | 9624 | 12 | c includes in preprocess.c |  |  | 0.657 |
| walker |  | 9637 | 13 | c decl body at preprocess.c:998 |  |  | 0.657 |
| walker |  | 9671 | 34 | c decl at preprocess.c:1115 |  |  | 0.657 |
| walker |  | 9712 | 41 | c decl at preprocess.c:34 |  |  | 0.657 |
| ns | 9756 |  | 326 | The expression precedence chain, as grammar comments | 6.2 | 3.9 | 0.650 |
| walker |  | 9763 | 51 | c decl at preprocess.c:55 |  |  | 0.650 |
| walker |  | 9835 | 72 | c decl at preprocess.c:44 |  |  | 0.650 |
| walker |  | 9964 | 129 | c decl names surface in include/stdarg.h |  |  | 0.650 |
| walker |  | 9987 | 23 | c decl at include/stdarg.h:13 |  |  | 0.650 |
| ns | 9989 |  | 233 | Every preprocessor directive chibicc handles | 6.3 |  | 0.644 |
