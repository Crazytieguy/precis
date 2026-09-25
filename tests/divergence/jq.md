Score(3000)=0.778 I=0.839 C=0.722 ns_rows≤3K=16/40 grid(1000/1442/2080/3000/4327/6240/9000)=0.756/0.819/0.839/0.778/0.634/0.538/0.457

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| ns | 71 |  | 71 | README lede: what jq is | 1.1 |  | 0.000 |
| walker |  | 81 | 81 | Fs::DirListing { dir: . } |  |  | 0.000 |
| walker |  | 90 | 9 | Fs::DirListing { dir: config } |  |  | 0.000 |
| walker |  | 113 | 23 | Fs::DirListing { dir: m4 } |  |  | 0.000 |
| ns | 152 |  | 81 | Repository root listing (complete) | 1.2 |  | 0.698 |
| walker |  | 165 | 52 | Fs::DirListing { dir: docs } |  |  | 0.703 |
| walker |  | 178 | 13 | Fs::DirListing { dir: docs/content } |  |  | 0.708 |
| walker |  | 182 | 4 | Fs::DirListing { dir: docs/content/download } |  |  | 0.708 |
| walker |  | 186 | 4 | Fs::DirListing { dir: docs/content/tutorial } |  |  | 0.708 |
| walker |  | 207 | 21 | Fs::DirListing { dir: docs/templates } |  |  | 0.708 |
| ns | 215 |  | 63 | Front-end landmarks: jq_parse / jq_parse_library / block_compile | 1.3 |  | 0.643 |
| walker |  | 278 | 71 | Markdown::ReadmeHeadline { file: README.md } |  |  | 0.916 |
| walker |  | 309 | 31 | Fs::DirListing { dir: docs/public } |  |  | 0.916 |
| walker |  | 313 | 4 | Fs::DirListing { dir: docs/public/css } |  |  | 0.916 |
| walker |  | 318 | 5 | Fs::DirListing { dir: docs/public/js } |  |  | 0.916 |
| walker |  | 330 | 12 | Fs::DirListing { dir: .github } |  |  | 0.916 |
| ns | 334 |  | 119 | Back-end landmarks: load_program, builtins_bind, and who owns the bytecode format | 1.4 |  | 0.785 |
| walker |  | 365 | 35 | Fs::DirListing { dir: .github/workflows } |  |  | 0.788 |
| walker |  | 370 | 5 | Fs::DirListing { dir: .github/ISSUE_TEMPLATE } |  |  | 0.788 |
| walker |  | 382 | 12 | Fs::DirListing { dir: vendor } |  |  | 0.789 |
| walker |  | 403 | 21 | Fs::DirListing { dir: docs/templates/shared } |  |  | 0.789 |
| walker |  | 417 | 14 | Fs::DirListing { dir: scripts } |  |  | 0.789 |
| walker |  | 464 | 47 | Fs::DirListing { dir: config/m4 } |  |  | 0.789 |
| ns | 517 |  | 183 | README: build-from-source dependencies and the exact command sequence | 1.5 |  | 0.631 |
| walker |  | 536 | 72 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.632 |
| walker |  | 588 | 52 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.632 |
| ns | 607 |  | 90 | README: static build and released-tarball builds | 1.6 |  | 0.594 |
| walker |  | 631 | 43 | Fs::DirListing { dir: docs/content/manual } |  |  | 0.600 |
| walker |  | 635 | 4 | Fs::DirListing { dir: docs/content/manual/dev } |  |  | 0.600 |
| walker |  | 639 | 4 | Fs::DirListing { dir: docs/content/manual/v1.3 } |  |  | 0.600 |
| walker |  | 643 | 4 | Fs::DirListing { dir: docs/content/manual/v1.4 } |  |  | 0.600 |
| walker |  | 647 | 4 | Fs::DirListing { dir: docs/content/manual/v1.5 } |  |  | 0.600 |
| walker |  | 651 | 4 | Fs::DirListing { dir: docs/content/manual/v1.6 } |  |  | 0.600 |
| walker |  | 655 | 4 | Fs::DirListing { dir: docs/content/manual/v1.7 } |  |  | 0.600 |
| walker |  | 659 | 4 | Fs::DirListing { dir: docs/content/manual/v1.8 } |  |  | 0.600 |
| ns | 840 |  | 233 | src/ listing (complete) - the flat core | 1.7 |  | 0.449 |
| walker |  | 892 | 233 | Fs::DirListing { dir: src } |  |  | 0.749 |
| ns | 1005 |  | 165 | tests/ listing (complete) | 1.8 |  | 0.652 |
| walker |  | 1057 | 165 | Fs::DirListing { dir: tests } |  |  | 0.794 |
| ns | 1113 |  | 108 | docs/ and docs/content/manual listings (complete) | 1.9 |  | 0.805 |
| walker |  | 1140 | 83 | Markdown::Section { file: README.md, section_index: 4, keeps_default_concavity: false } |  |  | 0.805 |
| ns | 1160 |  | 47 | CI workflows and vendored dependencies (complete listings) | 1.10 |  | 0.808 |
| walker |  | 1316 | 176 | Code::CodeKey { rung: Names, file: src/jq.h, decl: 0, sub: 0, line: 0 } |  |  | 0.811 |
| walker |  | 1368 | 52 | Code::CodeKey { rung: Decl, file: src/jq.h, decl: 1, sub: 0, line: 11 } |  |  | 0.812 |
| ns | 1385 |  | 225 | jq.h: jq_state lifecycle - init, compile, start, next, teardown | 2.1 |  | 0.798 |
| ns | 1503 |  | 118 | jq.h: debug-trace flags and halt / exit-code / error-message API | 2.2 |  | 0.784 |
| walker |  | 1549 | 181 | Code::CodeKey { rung: Names, file: src/jq.h, decl: 0, sub: 1, line: 0 } |  |  | 0.824 |
| walker |  | 1705 | 156 | Code::CodeKey { rung: Names, file: src/jq.h, decl: 0, sub: 2, line: 0 } |  |  | 0.828 |
| ns | 1749 |  | 246 | jq.h: input/debug/stderr callbacks and the attribute store | 2.3 |  | 0.822 |
| walker |  | 1871 | 166 | Code::CodeKey { rung: Names, file: src/jq.h, decl: 0, sub: 3, line: 0 } |  |  | 0.833 |
| walker |  | 1969 | 98 | Code::CodeKey { rung: Names, file: src/jq.h, decl: 0, sub: 4, line: 0 } |  |  | 0.837 |
| ns | 1979 |  | 230 | jq.h: jq_util_input_* file/stdin reader and jq_set_colors | 2.4 |  | 0.839 |
| walker |  | 2067 | 98 | Fs::DirListing { dir: sig } |  |  | 0.839 |
| ns | 2202 |  | 223 | main.c usage(): synopsis and one-paragraph description | 3.1 |  | 0.818 |
| walker |  | 2413 | 346 | Plaintext::Whole { file: Dockerfile } |  |  | 0.818 |
| walker |  | 2580 | 167 | Fs::DirListing { dir: vendor/decNumber } |  |  | 0.818 |
| walker |  | 2595 | 15 | Plaintext::DeclSurface { file: docs/public/robots.txt } |  |  | 0.818 |
| ns | 2653 |  | 451 | main.c usage(): input and output-formatting options (-n through --seq) | 3.2 |  | 0.778 |
| walker |  | 2853 | 258 | Code::CodeKey { rung: Names, file: src/parser.h, decl: 0, sub: 0, line: 0 } |  |  | 0.778 |
| walker |  | 2860 | 7 | Code::CodeKey { rung: Decl, file: src/parser.h, decl: 6, sub: 0, line: 123 } |  |  | 0.778 |
| walker |  | 2876 | 16 | Code::CodeKey { rung: Decl, file: src/parser.h, decl: 1, sub: 0, line: 44 } |  |  | 0.778 |
| walker |  | 3020 | 144 | Code::CodeKey { rung: Decl, file: src/parser.h, decl: 4, sub: 0, line: 54 } |  |  | 0.778 |
| walker |  | 3028 | 8 | Code::CodeKey { rung: Doc, file: src/parser.h, decl: 7, sub: 0, line: 126 } |  |  | 0.778 |
| ns | 3043 |  | 390 | main.c usage(): program, argument and mode options (-f through --) | 3.3 |  | 0.745 |
| walker |  | 3081 | 53 | Code::CodeKey { rung: Names, file: src/exec_stack.h, decl: 0, sub: 0, line: 0 } |  |  | 0.745 |
| walker |  | 3122 | 41 | Code::CodeKey { rung: Decl, file: src/exec_stack.h, decl: 1, sub: 0, line: 40 } |  |  | 0.745 |
| walker |  | 3173 | 51 | Code::CodeKey { rung: Decl, file: src/exec_stack.h, decl: 4, sub: 0, line: 52 } |  |  | 0.745 |
| walker |  | 3178 | 5 | Fs::DirListing { dir: tests/torture } |  |  | 0.745 |
| ns | 3210 |  | 167 | main.c: process exit-status codes | 3.4 |  | 0.730 |
| ns | 3433 |  | 223 | main.c: the undocumented flags (--debug-dump-disasm, --debug-trace, --run-tests) | 3.5 |  | 0.714 |
| walker |  | 3463 | 285 | Code::CodeKey { rung: Names, file: src/parser.h, decl: 0, sub: 1, line: 0 } |  |  | 0.714 |
| ns | 3673 |  | 240 | Makefile.am: the TESTS list and test environment | 3.6 |  | 0.692 |
| walker |  | 3702 | 239 | Code::CodeKey { rung: Names, file: src/parser.h, decl: 0, sub: 2, line: 0 } |  |  | 0.692 |
| walker |  | 3707 | 5 | Code::CodeKey { rung: Decl, file: src/parser.h, decl: 64, sub: 0, line: 205 } |  |  | 0.692 |
| walker |  | 3714 | 7 | Code::CodeKey { rung: Decl, file: src/parser.h, decl: 60, sub: 0, line: 191 } |  |  | 0.692 |
| walker |  | 3735 | 21 | Code::CodeKey { rung: Decl, file: src/parser.h, decl: 61, sub: 0, line: 195 } |  |  | 0.692 |
| walker |  | 3779 | 44 | Code::CodeKey { rung: Decl, file: src/parser.h, decl: 62, sub: 0, line: 197 } |  |  | 0.692 |
| walker |  | 3864 | 85 | Code::CodeKey { rung: Decl, file: src/parser.h, decl: 57, sub: 0, line: 178 } |  |  | 0.692 |
| ns | 3911 |  | 238 | tests/setup + tests/jqtest: how one test driver actually runs | 3.7 |  | 0.675 |
| walker |  | 3982 | 118 | Code::CodeKey { rung: Names, file: src/locfile.h, decl: 0, sub: 0, line: 0 } |  |  | 0.675 |
| walker |  | 3990 | 8 | Code::CodeKey { rung: Decl, file: src/locfile.h, decl: 1, sub: 0, line: 6 } |  |  | 0.675 |
| ns | 4024 |  | 113 | tests/jq.test: the three-line test format, with the first cases | 3.8 |  | 0.660 |
| walker |  | 4067 | 77 | Code::CodeKey { rung: Decl, file: src/locfile.h, decl: 2, sub: 0, line: 12 } |  |  | 0.660 |
| walker |  | 4250 | 183 | Code::CodeKey { rung: Names, file: src/bytecode.h, decl: 0, sub: 0, line: 0 } |  |  | 0.660 |
| ns | 4257 |  | 233 | jv.h: jv_kind enum and the jv struct | 4.1 |  | 0.634 |
| walker |  | 4287 | 37 | Code::CodeKey { rung: Decl, file: src/bytecode.h, decl: 6, sub: 0, line: 59 } |  |  | 0.634 |
| walker |  | 4364 | 77 | Code::CodeKey { rung: Decl, file: src/bytecode.h, decl: 2, sub: 0, line: 32 } |  |  | 0.634 |
| ns | 4411 |  | 154 | jv.h: the consume/produce refcount contract | 4.2 |  | 0.622 |
| walker |  | 4487 | 123 | Code::CodeKey { rung: Decl, file: src/bytecode.h, decl: 5, sub: 0, line: 48 } |  |  | 0.622 |
| walker |  | 4624 | 137 | Code::CodeKey { rung: Decl, file: src/bytecode.h, decl: 8, sub: 0, line: 72 } |  |  | 0.622 |
| walker |  | 4768 | 144 | Code::CodeKey { rung: Decl, file: src/bytecode.h, decl: 1, sub: 0, line: 20 } |  |  | 0.622 |
| walker |  | 4800 | 32 | Code::CodeKey { rung: Names, file: src/jv_private.h, decl: 0, sub: 0, line: 0 } |  |  | 0.622 |
| ns | 4813 |  | 402 | jv.h: comparison, invalid-with-message, constructors, numbers, arrays | 4.3 |  | 0.594 |
| walker |  | 4933 | 133 | Code::CodeKey { rung: Names, file: src/jv_alloc.h, decl: 0, sub: 0, line: 0 } |  |  | 0.594 |
| walker |  | 5019 | 86 | Code::CodeKey { rung: Names, file: src/util.h, decl: 0, sub: 0, line: 0 } |  |  | 0.594 |
| walker |  | 5034 | 15 | Code::CodeKey { rung: Decl, file: src/util.h, decl: 4, sub: 0, line: 44 } |  |  | 0.594 |
| walker |  | 5052 | 18 | Code::CodeKey { rung: Decl, file: src/util.h, decl: 5, sub: 0, line: 62 } |  |  | 0.594 |
| ns | 5142 |  | 329 | jv.h: the string API | 4.4 |  | 0.579 |
| walker |  | 5164 | 112 | Plaintext::DeclSurface { file: compile-ios.sh } |  |  | 0.579 |
| walker |  | 5221 | 57 | Code::CodeKey { rung: Names, file: src/builtin.h, decl: 0, sub: 0, line: 0 } |  |  | 0.580 |
| walker |  | 5341 | 120 | Code::CodeKey { rung: Decl, file: src/builtin.h, decl: 2, sub: 0, line: 10 } |  |  | 0.580 |
| ns | 5353 |  | 211 | jv.h: the object API and its iterator protocol | 4.5 |  | 0.569 |
| walker |  | 5380 | 39 | Code::CodeKey { rung: Names, file: src/jq_parser.h, decl: 0, sub: 0, line: 0 } |  |  | 0.573 |
| ns | 5549 |  | 196 | jv.h: path access, keys, ordering, sort/group/unique | 4.6 |  | 0.565 |
| walker |  | 5615 | 235 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.565 |
| walker |  | 5635 | 20 | Code::CodeKey { rung: Names, file: src/jv_utf8_tables.h, decl: 0, sub: 0, line: 0 } |  |  | 0.565 |
| walker |  | 5656 | 21 | Code::CodeKey { rung: Names, file: src/jv_dtoa_tsd.h, decl: 0, sub: 0, line: 0 } |  |  | 0.565 |
| ns | 5811 |  | 262 | jv.h: parsing - flags, one-shot parsers, streaming jv_parser, jv_load_file | 4.7 |  | 0.551 |
| walker |  | 5869 | 213 | Code::CodeKey { rung: Names, file: src/compile.h, decl: 0, sub: 0, line: 0 } |  |  | 0.551 |
| walker |  | 5885 | 16 | Code::CodeKey { rung: Decl, file: src/compile.h, decl: 3, sub: 0, line: 12 } |  |  | 0.551 |
| walker |  | 6062 | 177 | Code::CodeKey { rung: Names, file: src/compile.h, decl: 0, sub: 1, line: 0 } |  |  | 0.551 |
| ns | 6129 |  | 318 | jv.h: print flags and the dump/show functions | 4.8 |  | 0.538 |
| walker |  | 6244 | 182 | Code::CodeKey { rung: Names, file: src/compile.h, decl: 0, sub: 2, line: 0 } |  |  | 0.538 |
| ns | 6267 |  | 138 | jv.h: the convenience macro walls (existence, not bodies) | 4.9 |  | 0.530 |
| walker |  | 6433 | 189 | Code::CodeKey { rung: Names, file: src/compile.h, decl: 0, sub: 3, line: 0 } |  |  | 0.530 |
| walker |  | 6615 | 182 | Code::CodeKey { rung: Names, file: src/compile.h, decl: 0, sub: 4, line: 0 } |  |  | 0.538 |
| walker |  | 6771 | 156 | Code::CodeKey { rung: Names, file: src/compile.h, decl: 0, sub: 5, line: 0 } |  |  | 0.538 |
| ns | 6773 |  | 506 | opcode_list.h: every opcode and its immediate kind | 5.1 |  | 0.514 |
| walker |  | 6956 | 185 | Code::CodeKey { rung: Names, file: src/compile.h, decl: 0, sub: 6, line: 0 } |  |  | 0.514 |
| walker |  | 7002 | 46 | Code::CodeKey { rung: Decl, file: src/compile.h, decl: 72, sub: 0, line: 101 } |  |  | 0.514 |
| walker |  | 7202 | 200 | Code::CodeKey { rung: Names, file: src/lexer.h, decl: 0, sub: 0, line: 0 } |  |  | 0.514 |
| walker |  | 7207 | 5 | Code::CodeKey { rung: Decl, file: src/lexer.h, decl: 13, sub: 0, line: 288 } |  |  | 0.514 |
| walker |  | 7245 | 38 | Code::CodeKey { rung: Decl, file: src/lexer.h, decl: 7, sub: 0, line: 267 } |  |  | 0.514 |
| ns | 7248 |  | 475 | parser.y: the complete token list | 5.2 |  | 0.492 |
| ns | 7435 |  | 187 | parser.y: operator precedence and associativity | 5.3 |  | 0.485 |
| walker |  | 7438 | 193 | Code::CodeKey { rung: Names, file: src/lexer.h, decl: 0, sub: 1, line: 0 } |  |  | 0.485 |
| walker |  | 7462 | 24 | Code::CodeKey { rung: Decl, file: src/lexer.h, decl: 20, sub: 0, line: 345 } |  |  | 0.485 |
| walker |  | 7492 | 30 | Code::CodeKey { rung: Decl, file: src/lexer.h, decl: 18, sub: 0, line: 294 } |  |  | 0.485 |
| walker |  | 7508 | 16 | Code::CodeKey { rung: Doc, file: src/lexer.h, decl: 19, sub: 0, line: 336 } |  |  | 0.485 |
| walker |  | 7684 | 176 | Code::CodeKey { rung: Names, file: src/lexer.h, decl: 0, sub: 2, line: 0 } |  |  | 0.485 |
| walker |  | 7707 | 23 | Code::CodeKey { rung: Decl, file: src/lexer.h, decl: 29, sub: 0, line: 374 } |  |  | 0.485 |
| walker |  | 7728 | 21 | Code::CodeKey { rung: Decl, file: src/lexer.h, decl: 30, sub: 0, line: 379 } |  |  | 0.485 |
| walker |  | 7883 | 155 | Code::CodeKey { rung: Names, file: src/lexer.h, decl: 0, sub: 3, line: 0 } |  |  | 0.485 |
| ns | 8106 |  | 671 | builtin.c: function_list, part 1 - libm, binops, conversions, keys, strings, paths, sorting | 5.4 |  | 0.469 |
| walker |  | 8145 | 262 | Code::CodeKey { rung: Decl, file: src/lexer.h, decl: 31, sub: 0, line: 384 } |  |  | 0.469 |
| walker |  | 8399 | 254 | Code::CodeKey { rung: Decl, file: src/lexer.h, decl: 31, sub: 1, line: 384 } |  |  | 0.469 |
| walker |  | 8589 | 190 | Code::CodeKey { rung: Names, file: src/lexer.h, decl: 0, sub: 4, line: 0 } |  |  | 0.469 |
| ns | 8607 |  | 501 | builtin.c: function_list, part 2 - search, min/max, errors, env, regex, I/O, time | 5.5 |  | 0.457 |
| walker |  | 8787 | 198 | Code::CodeKey { rung: Names, file: src/lexer.h, decl: 0, sub: 5, line: 0 } |  |  | 0.457 |
| walker |  | 9044 | 257 | Code::CodeKey { rung: Names, file: src/lexer.h, decl: 0, sub: 6, line: 0 } |  |  | 0.457 |
| walker |  | 9053 | 9 | Code::CodeKey { rung: Decl, file: src/lexer.h, decl: 68, sub: 0, line: 566 } |  |  | 0.457 |
| walker |  | 9083 | 30 | Code::CodeKey { rung: Decl, file: src/lexer.h, decl: 69, sub: 0, line: 569 } |  |  | 0.457 |
| ns | 9119 |  | 512 | builtin.jq: every jq-defined builtin, part 1 (lines 1-115) | 5.6 |  | 0.435 |
| walker |  | 9126 | 43 | Code::CodeKey { rung: Decl, file: src/lexer.h, decl: 70, sub: 0, line: 572 } |  |  | 0.435 |
| walker |  | 9351 | 225 | Code::CodeKey { rung: Names, file: src/jv_dtoa.h, decl: 0, sub: 0, line: 0 } |  |  | 0.435 |
| walker |  | 9383 | 32 | Code::CodeKey { rung: Decl, file: src/jv_dtoa.h, decl: 3, sub: 0, line: 6 } |  |  | 0.435 |
| ns | 9468 |  | 349 | builtin.jq: every jq-defined builtin, part 2 (lines 116-244) | 5.7 |  | 0.422 |
| walker |  | 9541 | 158 | Code::CodeKey { rung: Names, file: src/jv_unicode.h, decl: 0, sub: 0, line: 0 } |  |  | 0.422 |
| walker |  | 9589 | 48 | Code::CodeKey { rung: Names, file: src/linker.h, decl: 0, sub: 0, line: 0 } |  |  | 0.426 |
| walker |  | 9620 | 31 | Code::CodeKey { rung: Doc, file: src/lexer.h, decl: 21, sub: 0, line: 352 } |  |  | 0.426 |
| walker |  | 9707 | 87 | Code::CodeKey { rung: Doc, file: src/jq.h, decl: 34, sub: 0, line: 60 } |  |  | 0.426 |
| ns | 9778 |  | 310 | execute.c: struct jq_state, the whole interpreter state | 5.8 |  | 0.416 |
| walker |  | 9929 | 222 | Code::CodeKey { rung: Names, file: src/jv.h, decl: 0, sub: 0, line: 0 } |  |  | 0.421 |
| ns | 9977 |  | 199 | manual.yml: the section titles of the jq language reference | 6.1 |  | 0.417 |
| walker |  | 9999 | 70 | Code::CodeKey { rung: Decl, file: src/jv.h, decl: 1, sub: 0, line: 19 } |  |  | 0.423 |
| walker |  | 9999 | 0 | Code::CodeKey { rung: Decl, file: src/jv.h, decl: 3, sub: 0, line: 34 } |  |  | 0.423 |
