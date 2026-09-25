Score(3000)=0.643 I=0.796 C=0.519 ns_rows≤3K=16/40 grid(1000/1442/2080/3000/4327/6240/9000)=0.750/0.771/0.693/0.643/0.524/0.537/0.456

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
| walker |  | 377 | 12 | Fs::DirListing { dir: vendor } |  |  | 0.789 |
| walker |  | 398 | 21 | Fs::DirListing { dir: docs/templates/shared } |  |  | 0.789 |
| walker |  | 412 | 14 | Fs::DirListing { dir: scripts } |  |  | 0.789 |
| walker |  | 459 | 47 | Fs::DirListing { dir: config/m4 } |  |  | 0.789 |
| walker |  | 515 | 56 | Markdown::ReadmeHeadline { file: docs/README.md } |  |  | 0.789 |
| ns | 517 |  | 183 | README: build-from-source dependencies and the exact command sequence | 1.5 |  | 0.631 |
| walker |  | 587 | 72 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.632 |
| ns | 607 |  | 90 | README: static build and released-tarball builds | 1.6 |  | 0.594 |
| walker |  | 630 | 43 | Fs::DirListing { dir: docs/content/manual } |  |  | 0.600 |
| walker |  | 634 | 4 | Fs::DirListing { dir: docs/content/manual/dev } |  |  | 0.600 |
| walker |  | 638 | 4 | Fs::DirListing { dir: docs/content/manual/v1.3 } |  |  | 0.600 |
| walker |  | 642 | 4 | Fs::DirListing { dir: docs/content/manual/v1.4 } |  |  | 0.600 |
| walker |  | 646 | 4 | Fs::DirListing { dir: docs/content/manual/v1.5 } |  |  | 0.600 |
| walker |  | 650 | 4 | Fs::DirListing { dir: docs/content/manual/v1.6 } |  |  | 0.600 |
| walker |  | 654 | 4 | Fs::DirListing { dir: docs/content/manual/v1.7 } |  |  | 0.600 |
| walker |  | 658 | 4 | Fs::DirListing { dir: docs/content/manual/v1.8 } |  |  | 0.600 |
| ns | 840 |  | 233 | src/ listing (complete) - the flat core | 1.7 |  | 0.449 |
| walker |  | 891 | 233 | Fs::DirListing { dir: src } |  |  | 0.749 |
| walker |  | 943 | 52 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.749 |
| ns | 1005 |  | 165 | tests/ listing (complete) | 1.8 |  | 0.652 |
| walker |  | 1108 | 165 | Fs::DirListing { dir: tests } |  |  | 0.794 |
| ns | 1113 |  | 108 | docs/ and docs/content/manual listings (complete) | 1.9 |  | 0.805 |
| ns | 1160 |  | 47 | CI workflows and vendored dependencies (complete listings) | 1.10 |  | 0.808 |
| walker |  | 1206 | 98 | Fs::DirListing { dir: sig } |  |  | 0.808 |
| walker |  | 1373 | 167 | Fs::DirListing { dir: vendor/decNumber } |  |  | 0.808 |
| ns | 1385 |  | 225 | jq.h: jq_state lifecycle - init, compile, start, next, teardown | 2.1 |  | 0.771 |
| ns | 1503 |  | 118 | jq.h: debug-trace flags and halt / exit-code / error-message API | 2.2 |  | 0.750 |
| walker |  | 1719 | 346 | Plaintext::Whole { file: Dockerfile } |  |  | 0.750 |
| ns | 1749 |  | 246 | jq.h: input/debug/stderr callbacks and the attribute store | 2.3 |  | 0.720 |
| walker |  | 1802 | 83 | Markdown::Section { file: README.md, section_index: 4, keeps_default_concavity: false } |  |  | 0.720 |
| walker |  | 1817 | 15 | Plaintext::DeclSurface { file: docs/public/robots.txt } |  |  | 0.720 |
| walker |  | 1822 | 5 | Fs::DirListing { dir: .github/ISSUE_TEMPLATE } |  |  | 0.720 |
| walker |  | 1827 | 5 | Fs::DirListing { dir: tests/torture } |  |  | 0.720 |
| walker |  | 1912 | 85 | Markdown::Section { file: README.md, section_index: 5, keeps_default_concavity: false } |  |  | 0.720 |
| ns | 1979 |  | 230 | jq.h: jq_util_input_* file/stdin reader and jq_set_colors | 2.4 |  | 0.693 |
| ns | 2202 |  | 223 | main.c usage(): synopsis and one-paragraph description | 3.1 |  | 0.676 |
| walker |  | 2240 | 328 | Code::CodeKey { rung: Names, file: src/parser.h, decl: 0, sub: 0, line: 0 } |  |  | 0.676 |
| walker |  | 2247 | 7 | Code::CodeKey { rung: Decl, file: src/parser.h, decl: 6, sub: 0, line: 123 } |  |  | 0.676 |
| walker |  | 2263 | 16 | Code::CodeKey { rung: Decl, file: src/parser.h, decl: 1, sub: 0, line: 44 } |  |  | 0.676 |
| walker |  | 2271 | 8 | Code::CodeKey { rung: Doc, file: src/parser.h, decl: 7, sub: 0, line: 126 } |  |  | 0.676 |
| walker |  | 2415 | 144 | Code::CodeKey { rung: Decl, file: src/parser.h, decl: 4, sub: 0, line: 54 } |  |  | 0.676 |
| ns | 2653 |  | 451 | main.c usage(): input and output-formatting options (-n through --seq) | 3.2 |  | 0.643 |
| walker |  | 2751 | 336 | Code::CodeKey { rung: Names, file: src/parser.h, decl: 0, sub: 1, line: 0 } |  |  | 0.643 |
| walker |  | 2836 | 85 | Code::CodeKey { rung: Decl, file: src/parser.h, decl: 57, sub: 0, line: 178 } |  |  | 0.643 |
| walker |  | 2954 | 118 | Code::CodeKey { rung: Names, file: src/parser.h, decl: 0, sub: 2, line: 0 } |  |  | 0.643 |
| walker |  | 2959 | 5 | Code::CodeKey { rung: Decl, file: src/parser.h, decl: 64, sub: 0, line: 205 } |  |  | 0.643 |
| walker |  | 2966 | 7 | Code::CodeKey { rung: Decl, file: src/parser.h, decl: 60, sub: 0, line: 191 } |  |  | 0.643 |
| walker |  | 2987 | 21 | Code::CodeKey { rung: Decl, file: src/parser.h, decl: 61, sub: 0, line: 195 } |  |  | 0.643 |
| walker |  | 3031 | 44 | Code::CodeKey { rung: Decl, file: src/parser.h, decl: 62, sub: 0, line: 197 } |  |  | 0.643 |
| ns | 3043 |  | 390 | main.c usage(): program, argument and mode options (-f through --) | 3.3 |  | 0.615 |
| walker |  | 3084 | 53 | Code::CodeKey { rung: Names, file: src/exec_stack.h, decl: 0, sub: 0, line: 0 } |  |  | 0.615 |
| walker |  | 3125 | 41 | Code::CodeKey { rung: Decl, file: src/exec_stack.h, decl: 1, sub: 0, line: 40 } |  |  | 0.615 |
| walker |  | 3176 | 51 | Code::CodeKey { rung: Decl, file: src/exec_stack.h, decl: 4, sub: 0, line: 52 } |  |  | 0.615 |
| ns | 3210 |  | 167 | main.c: process exit-status codes | 3.4 |  | 0.603 |
| walker |  | 3359 | 183 | Code::CodeKey { rung: Names, file: src/bytecode.h, decl: 0, sub: 0, line: 0 } |  |  | 0.603 |
| walker |  | 3396 | 37 | Code::CodeKey { rung: Decl, file: src/bytecode.h, decl: 6, sub: 0, line: 59 } |  |  | 0.603 |
| ns | 3433 |  | 223 | main.c: the undocumented flags (--debug-dump-disasm, --debug-trace, --run-tests) | 3.5 |  | 0.589 |
| walker |  | 3473 | 77 | Code::CodeKey { rung: Decl, file: src/bytecode.h, decl: 2, sub: 0, line: 32 } |  |  | 0.589 |
| walker |  | 3596 | 123 | Code::CodeKey { rung: Decl, file: src/bytecode.h, decl: 5, sub: 0, line: 48 } |  |  | 0.589 |
| ns | 3673 |  | 240 | Makefile.am: the TESTS list and test environment | 3.6 |  | 0.571 |
| walker |  | 3733 | 137 | Code::CodeKey { rung: Decl, file: src/bytecode.h, decl: 8, sub: 0, line: 72 } |  |  | 0.571 |
| walker |  | 3877 | 144 | Code::CodeKey { rung: Decl, file: src/bytecode.h, decl: 1, sub: 0, line: 20 } |  |  | 0.571 |
| walker |  | 3909 | 32 | Code::CodeKey { rung: Names, file: src/jv_private.h, decl: 0, sub: 0, line: 0 } |  |  | 0.571 |
| ns | 3911 |  | 238 | tests/setup + tests/jqtest: how one test driver actually runs | 3.7 |  | 0.558 |
| ns | 4024 |  | 113 | tests/jq.test: the three-line test format, with the first cases | 3.8 |  | 0.545 |
| walker |  | 4042 | 133 | Code::CodeKey { rung: Names, file: src/jv_alloc.h, decl: 0, sub: 0, line: 0 } |  |  | 0.545 |
| ns | 4257 |  | 233 | jv.h: jv_kind enum and the jv struct | 4.1 |  | 0.524 |
| walker |  | 4365 | 323 | Markdown::Section { file: docs/README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.524 |
| ns | 4411 |  | 154 | jv.h: the consume/produce refcount contract | 4.2 |  | 0.514 |
| walker |  | 4483 | 118 | Code::CodeKey { rung: Names, file: src/locfile.h, decl: 0, sub: 0, line: 0 } |  |  | 0.514 |
| walker |  | 4491 | 8 | Code::CodeKey { rung: Decl, file: src/locfile.h, decl: 1, sub: 0, line: 6 } |  |  | 0.514 |
| walker |  | 4568 | 77 | Code::CodeKey { rung: Decl, file: src/locfile.h, decl: 2, sub: 0, line: 12 } |  |  | 0.514 |
| walker |  | 4654 | 86 | Code::CodeKey { rung: Names, file: src/util.h, decl: 0, sub: 0, line: 0 } |  |  | 0.514 |
| walker |  | 4669 | 15 | Code::CodeKey { rung: Decl, file: src/util.h, decl: 4, sub: 0, line: 44 } |  |  | 0.514 |
| walker |  | 4687 | 18 | Code::CodeKey { rung: Decl, file: src/util.h, decl: 5, sub: 0, line: 62 } |  |  | 0.514 |
| ns | 4813 |  | 402 | jv.h: comparison, invalid-with-message, constructors, numbers, arrays | 4.3 |  | 0.490 |
| walker |  | 4907 | 220 | Code::CodeKey { rung: Names, file: src/jq.h, decl: 0, sub: 0, line: 0 } |  |  | 0.517 |
| walker |  | 4959 | 52 | Code::CodeKey { rung: Decl, file: src/jq.h, decl: 1, sub: 0, line: 11 } |  |  | 0.522 |
| ns | 5142 |  | 329 | jv.h: the string API | 4.4 |  | 0.509 |
| walker |  | 5176 | 217 | Code::CodeKey { rung: Names, file: src/jq.h, decl: 0, sub: 1, line: 0 } |  |  | 0.533 |
| ns | 5353 |  | 211 | jv.h: the object API and its iterator protocol | 4.5 |  | 0.523 |
| walker |  | 5379 | 203 | Code::CodeKey { rung: Names, file: src/jq.h, decl: 0, sub: 2, line: 0 } |  |  | 0.545 |
| walker |  | 5511 | 132 | Code::CodeKey { rung: Names, file: src/jq.h, decl: 0, sub: 3, line: 0 } |  |  | 0.567 |
| ns | 5549 |  | 196 | jv.h: path access, keys, ordering, sort/group/unique | 4.6 |  | 0.558 |
| walker |  | 5598 | 87 | Code::CodeKey { rung: Doc, file: src/jq.h, decl: 34, sub: 0, line: 60 } |  |  | 0.558 |
| walker |  | 5655 | 57 | Code::CodeKey { rung: Names, file: src/builtin.h, decl: 0, sub: 0, line: 0 } |  |  | 0.559 |
| walker |  | 5775 | 120 | Code::CodeKey { rung: Decl, file: src/builtin.h, decl: 2, sub: 0, line: 10 } |  |  | 0.559 |
| ns | 5811 |  | 262 | jv.h: parsing - flags, one-shot parsers, streaming jv_parser, jv_load_file | 4.7 |  | 0.545 |
| walker |  | 5814 | 39 | Code::CodeKey { rung: Names, file: src/jq_parser.h, decl: 0, sub: 0, line: 0 } |  |  | 0.549 |
| walker |  | 5834 | 20 | Code::CodeKey { rung: Names, file: src/jv_utf8_tables.h, decl: 0, sub: 0, line: 0 } |  |  | 0.549 |
| walker |  | 6095 | 261 | Code::CodeKey { rung: Names, file: src/compile.h, decl: 0, sub: 0, line: 0 } |  |  | 0.549 |
| walker |  | 6111 | 16 | Code::CodeKey { rung: Decl, file: src/compile.h, decl: 3, sub: 0, line: 12 } |  |  | 0.549 |
| ns | 6129 |  | 318 | jv.h: print flags and the dump/show functions | 4.8 |  | 0.537 |
| ns | 6267 |  | 138 | jv.h: the convenience macro walls (existence, not bodies) | 4.9 |  | 0.529 |
| walker |  | 6323 | 212 | Code::CodeKey { rung: Names, file: src/compile.h, decl: 0, sub: 1, line: 0 } |  |  | 0.529 |
| walker |  | 6554 | 231 | Code::CodeKey { rung: Names, file: src/compile.h, decl: 0, sub: 2, line: 0 } |  |  | 0.529 |
| ns | 6773 |  | 506 | opcode_list.h: every opcode and its immediate kind | 5.1 |  | 0.505 |
| walker |  | 6783 | 229 | Code::CodeKey { rung: Names, file: src/compile.h, decl: 0, sub: 3, line: 0 } |  |  | 0.513 |
| walker |  | 6982 | 199 | Code::CodeKey { rung: Names, file: src/compile.h, decl: 0, sub: 4, line: 0 } |  |  | 0.513 |
| walker |  | 7124 | 142 | Code::CodeKey { rung: Names, file: src/compile.h, decl: 0, sub: 5, line: 0 } |  |  | 0.513 |
| walker |  | 7170 | 46 | Code::CodeKey { rung: Decl, file: src/compile.h, decl: 72, sub: 0, line: 101 } |  |  | 0.513 |
| walker |  | 7191 | 21 | Code::CodeKey { rung: Names, file: src/jv_dtoa_tsd.h, decl: 0, sub: 0, line: 0 } |  |  | 0.513 |
| ns | 7248 |  | 475 | parser.y: the complete token list | 5.2 |  | 0.491 |
| ns | 7435 |  | 187 | parser.y: operator precedence and associativity | 5.3 |  | 0.484 |
| walker |  | 7440 | 249 | Code::CodeKey { rung: Names, file: src/lexer.h, decl: 0, sub: 0, line: 0 } |  |  | 0.484 |
| walker |  | 7445 | 5 | Code::CodeKey { rung: Decl, file: src/lexer.h, decl: 13, sub: 0, line: 288 } |  |  | 0.484 |
| walker |  | 7475 | 30 | Code::CodeKey { rung: Decl, file: src/lexer.h, decl: 18, sub: 0, line: 294 } |  |  | 0.484 |
| walker |  | 7513 | 38 | Code::CodeKey { rung: Decl, file: src/lexer.h, decl: 7, sub: 0, line: 267 } |  |  | 0.484 |
| walker |  | 7763 | 250 | Code::CodeKey { rung: Names, file: src/lexer.h, decl: 0, sub: 1, line: 0 } |  |  | 0.484 |
| walker |  | 7786 | 23 | Code::CodeKey { rung: Decl, file: src/lexer.h, decl: 29, sub: 0, line: 374 } |  |  | 0.484 |
| walker |  | 7807 | 21 | Code::CodeKey { rung: Decl, file: src/lexer.h, decl: 30, sub: 0, line: 379 } |  |  | 0.484 |
| walker |  | 7831 | 24 | Code::CodeKey { rung: Decl, file: src/lexer.h, decl: 20, sub: 0, line: 345 } |  |  | 0.484 |
| walker |  | 7847 | 16 | Code::CodeKey { rung: Doc, file: src/lexer.h, decl: 19, sub: 0, line: 336 } |  |  | 0.484 |
| walker |  | 7878 | 31 | Code::CodeKey { rung: Doc, file: src/lexer.h, decl: 21, sub: 0, line: 352 } |  |  | 0.484 |
| walker |  | 8077 | 199 | Code::CodeKey { rung: Names, file: src/lexer.h, decl: 0, sub: 2, line: 0 } |  |  | 0.484 |
| ns | 8106 |  | 671 | builtin.c: function_list, part 1 - libm, binops, conversions, keys, strings, paths, sorting | 5.4 |  | 0.467 |
| walker |  | 8339 | 262 | Code::CodeKey { rung: Decl, file: src/lexer.h, decl: 31, sub: 0, line: 384 } |  |  | 0.467 |
| walker |  | 8593 | 254 | Code::CodeKey { rung: Decl, file: src/lexer.h, decl: 31, sub: 1, line: 384 } |  |  | 0.467 |
| ns | 8607 |  | 501 | builtin.c: function_list, part 2 - search, min/max, errors, env, regex, I/O, time | 5.5 |  | 0.456 |
| walker |  | 8828 | 235 | Code::CodeKey { rung: Names, file: src/lexer.h, decl: 0, sub: 3, line: 0 } |  |  | 0.456 |
| walker |  | 9074 | 246 | Code::CodeKey { rung: Names, file: src/lexer.h, decl: 0, sub: 4, line: 0 } |  |  | 0.456 |
| ns | 9119 |  | 512 | builtin.jq: every jq-defined builtin, part 1 (lines 1-115) | 5.6 |  | 0.433 |
| walker |  | 9259 | 185 | Code::CodeKey { rung: Names, file: src/lexer.h, decl: 0, sub: 5, line: 0 } |  |  | 0.433 |
| walker |  | 9268 | 9 | Code::CodeKey { rung: Decl, file: src/lexer.h, decl: 68, sub: 0, line: 566 } |  |  | 0.433 |
| walker |  | 9298 | 30 | Code::CodeKey { rung: Decl, file: src/lexer.h, decl: 69, sub: 0, line: 569 } |  |  | 0.433 |
| walker |  | 9341 | 43 | Code::CodeKey { rung: Decl, file: src/lexer.h, decl: 70, sub: 0, line: 572 } |  |  | 0.433 |
| ns | 9468 |  | 349 | builtin.jq: every jq-defined builtin, part 2 (lines 116-244) | 5.7 |  | 0.420 |
| walker |  | 9566 | 225 | Code::CodeKey { rung: Names, file: src/jv_dtoa.h, decl: 0, sub: 0, line: 0 } |  |  | 0.420 |
| walker |  | 9598 | 32 | Code::CodeKey { rung: Decl, file: src/jv_dtoa.h, decl: 3, sub: 0, line: 6 } |  |  | 0.420 |
| walker |  | 9756 | 158 | Code::CodeKey { rung: Names, file: src/jv_unicode.h, decl: 0, sub: 0, line: 0 } |  |  | 0.420 |
| ns | 9778 |  | 310 | execute.c: struct jq_state, the whole interpreter state | 5.8 |  | 0.411 |
| walker |  | 9804 | 48 | Code::CodeKey { rung: Names, file: src/linker.h, decl: 0, sub: 0, line: 0 } |  |  | 0.415 |
| ns | 9977 |  | 199 | manual.yml: the section titles of the jq language reference | 6.1 |  | 0.411 |
| walker |  | 9998 | 194 | Code::CodeKey { rung: Names, file: src/jv.h, decl: 0, sub: 0, line: 0 } |  |  | 0.415 |
