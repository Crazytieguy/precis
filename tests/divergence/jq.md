Score(3000)=0.699 I=0.825 C=0.591 ns_rows≤3K=16/40 grid(1000/1442/2080/3000/4327/6240/9000)=0.750/0.781/0.708/0.699/0.569/0.479/0.471

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
| walker |  | 587 | 72 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.633 |
| ns | 607 |  | 90 | README: static build and released-tarball builds | 1.6 |  | 0.595 |
| walker |  | 630 | 43 | Fs::DirListing { dir: docs/content/manual } |  |  | 0.601 |
| walker |  | 634 | 4 | Fs::DirListing { dir: docs/content/manual/dev } |  |  | 0.601 |
| walker |  | 638 | 4 | Fs::DirListing { dir: docs/content/manual/v1.3 } |  |  | 0.601 |
| walker |  | 642 | 4 | Fs::DirListing { dir: docs/content/manual/v1.4 } |  |  | 0.601 |
| walker |  | 646 | 4 | Fs::DirListing { dir: docs/content/manual/v1.5 } |  |  | 0.601 |
| walker |  | 650 | 4 | Fs::DirListing { dir: docs/content/manual/v1.6 } |  |  | 0.601 |
| walker |  | 654 | 4 | Fs::DirListing { dir: docs/content/manual/v1.7 } |  |  | 0.601 |
| walker |  | 658 | 4 | Fs::DirListing { dir: docs/content/manual/v1.8 } |  |  | 0.601 |
| ns | 840 |  | 233 | src/ listing (complete) - the flat core | 1.7 |  | 0.449 |
| walker |  | 891 | 233 | Fs::DirListing { dir: src } |  |  | 0.750 |
| walker |  | 911 | 20 | Code::CodeKey { rung: Names, file: src/jv_utf8_tables.h, decl: 0, sub: 0, line: 0 } |  |  | 0.750 |
| walker |  | 932 | 21 | Code::CodeKey { rung: Names, file: src/jv_dtoa_tsd.h, decl: 0, sub: 0, line: 0 } |  |  | 0.750 |
| walker |  | 984 | 52 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.750 |
| ns | 1005 |  | 165 | tests/ listing (complete) | 1.8 |  | 0.652 |
| walker |  | 1016 | 32 | Code::CodeKey { rung: Names, file: src/jv_private.h, decl: 0, sub: 0, line: 0 } |  |  | 0.652 |
| ns | 1113 |  | 108 | docs/ and docs/content/manual listings (complete) | 1.9 |  | 0.685 |
| ns | 1160 |  | 47 | CI workflows and vendored dependencies (complete listings) | 1.10 |  | 0.694 |
| walker |  | 1181 | 165 | Fs::DirListing { dir: tests } |  |  | 0.808 |
| walker |  | 1220 | 39 | Code::CodeKey { rung: Names, file: src/jq_parser.h, decl: 0, sub: 0, line: 0 } |  |  | 0.815 |
| walker |  | 1268 | 48 | Code::CodeKey { rung: Names, file: src/linker.h, decl: 0, sub: 0, line: 0 } |  |  | 0.819 |
| walker |  | 1321 | 53 | Code::CodeKey { rung: Names, file: src/exec_stack.h, decl: 0, sub: 0, line: 0 } |  |  | 0.819 |
| walker |  | 1362 | 41 | Code::CodeKey { rung: Decl, file: src/exec_stack.h, decl: 1, sub: 0, line: 40 } |  |  | 0.819 |
| ns | 1385 |  | 225 | jq.h: jq_state lifecycle - init, compile, start, next, teardown | 2.1 |  | 0.781 |
| walker |  | 1413 | 51 | Code::CodeKey { rung: Decl, file: src/exec_stack.h, decl: 4, sub: 0, line: 52 } |  |  | 0.781 |
| walker |  | 1470 | 57 | Code::CodeKey { rung: Names, file: src/builtin.h, decl: 0, sub: 0, line: 0 } |  |  | 0.788 |
| ns | 1503 |  | 118 | jq.h: debug-trace flags and halt / exit-code / error-message API | 2.2 |  | 0.767 |
| walker |  | 1590 | 120 | Code::CodeKey { rung: Decl, file: src/builtin.h, decl: 2, sub: 0, line: 10 } |  |  | 0.767 |
| walker |  | 1688 | 98 | Fs::DirListing { dir: sig } |  |  | 0.767 |
| ns | 1749 |  | 246 | jq.h: input/debug/stderr callbacks and the attribute store | 2.3 |  | 0.736 |
| walker |  | 1855 | 167 | Fs::DirListing { dir: vendor/decNumber } |  |  | 0.736 |
| ns | 1979 |  | 230 | jq.h: jq_util_input_* file/stdin reader and jq_set_colors | 2.4 |  | 0.708 |
| walker |  | 2201 | 346 | Plaintext::Whole { file: Dockerfile } |  |  | 0.708 |
| ns | 2202 |  | 223 | main.c usage(): synopsis and one-paragraph description | 3.1 |  | 0.691 |
| walker |  | 2287 | 86 | Code::CodeKey { rung: Names, file: src/util.h, decl: 0, sub: 0, line: 0 } |  |  | 0.691 |
| walker |  | 2302 | 15 | Code::CodeKey { rung: Decl, file: src/util.h, decl: 4, sub: 0, line: 44 } |  |  | 0.691 |
| walker |  | 2320 | 18 | Code::CodeKey { rung: Decl, file: src/util.h, decl: 5, sub: 0, line: 62 } |  |  | 0.691 |
| walker |  | 2403 | 83 | Markdown::Section { file: README.md, section_index: 4, keeps_default_concavity: false } |  |  | 0.691 |
| walker |  | 2418 | 15 | Plaintext::DeclSurface { file: docs/public/robots.txt } |  |  | 0.691 |
| walker |  | 2441 | 23 | Code::CodeKey { rung: Names, file: src/jv_file.c, decl: 0, sub: 0, line: 0 } |  |  | 0.691 |
| ns | 2653 |  | 451 | main.c usage(): input and output-formatting options (-n through --seq) | 3.2 |  | 0.657 |
| walker |  | 2661 | 220 | Code::CodeKey { rung: Names, file: src/jq.h, decl: 0, sub: 0, line: 0 } |  |  | 0.692 |
| walker |  | 2713 | 52 | Code::CodeKey { rung: Decl, file: src/jq.h, decl: 1, sub: 0, line: 11 } |  |  | 0.699 |
| walker |  | 2718 | 5 | Fs::DirListing { dir: .github/ISSUE_TEMPLATE } |  |  | 0.699 |
| walker |  | 2723 | 5 | Fs::DirListing { dir: tests/torture } |  |  | 0.699 |
| ns | 3043 |  | 390 | main.c usage(): program, argument and mode options (-f through --) | 3.3 |  | 0.668 |
| walker |  | 3051 | 328 | Code::CodeKey { rung: Names, file: src/parser.h, decl: 0, sub: 0, line: 0 } |  |  | 0.668 |
| walker |  | 3058 | 7 | Code::CodeKey { rung: Decl, file: src/parser.h, decl: 6, sub: 0, line: 123 } |  |  | 0.668 |
| walker |  | 3074 | 16 | Code::CodeKey { rung: Decl, file: src/parser.h, decl: 1, sub: 0, line: 44 } |  |  | 0.668 |
| walker |  | 3082 | 8 | Code::CodeKey { rung: Doc, file: src/parser.h, decl: 7, sub: 0, line: 126 } |  |  | 0.668 |
| ns | 3210 |  | 167 | main.c: process exit-status codes | 3.4 |  | 0.655 |
| walker |  | 3226 | 144 | Code::CodeKey { rung: Decl, file: src/parser.h, decl: 4, sub: 0, line: 54 } |  |  | 0.655 |
| walker |  | 3311 | 85 | Markdown::Section { file: README.md, section_index: 5, keeps_default_concavity: false } |  |  | 0.655 |
| walker |  | 3429 | 118 | Code::CodeKey { rung: Names, file: src/locfile.h, decl: 0, sub: 0, line: 0 } |  |  | 0.655 |
| ns | 3433 |  | 223 | main.c: the undocumented flags (--debug-dump-disasm, --debug-trace, --run-tests) | 3.5 |  | 0.640 |
| walker |  | 3437 | 8 | Code::CodeKey { rung: Decl, file: src/locfile.h, decl: 1, sub: 0, line: 6 } |  |  | 0.640 |
| walker |  | 3514 | 77 | Code::CodeKey { rung: Decl, file: src/locfile.h, decl: 2, sub: 0, line: 12 } |  |  | 0.640 |
| walker |  | 3647 | 133 | Code::CodeKey { rung: Names, file: src/jv_alloc.h, decl: 0, sub: 0, line: 0 } |  |  | 0.640 |
| ns | 3673 |  | 240 | Makefile.am: the TESTS list and test environment | 3.6 |  | 0.621 |
| ns | 3911 |  | 238 | tests/setup + tests/jqtest: how one test driver actually runs | 3.7 |  | 0.606 |
| walker |  | 3970 | 323 | Markdown::Section { file: docs/README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.606 |
| ns | 4024 |  | 113 | tests/jq.test: the three-line test format, with the first cases | 3.8 |  | 0.592 |
| walker |  | 4128 | 158 | Code::CodeKey { rung: Names, file: src/jv_unicode.h, decl: 0, sub: 0, line: 0 } |  |  | 0.592 |
| ns | 4257 |  | 233 | jv.h: jv_kind enum and the jv struct | 4.1 |  | 0.569 |
| ns | 4411 |  | 154 | jv.h: the consume/produce refcount contract | 4.2 |  | 0.558 |
| walker |  | 4795 | 667 | Plaintext::Whole { file: compile-ios.sh } |  |  | 0.558 |
| ns | 4813 |  | 402 | jv.h: comparison, invalid-with-message, constructors, numbers, arrays | 4.3 |  | 0.533 |
| walker |  | 4978 | 183 | Code::CodeKey { rung: Names, file: src/bytecode.h, decl: 0, sub: 0, line: 0 } |  |  | 0.533 |
| walker |  | 5015 | 37 | Code::CodeKey { rung: Decl, file: src/bytecode.h, decl: 6, sub: 0, line: 59 } |  |  | 0.533 |
| walker |  | 5092 | 77 | Code::CodeKey { rung: Decl, file: src/bytecode.h, decl: 2, sub: 0, line: 32 } |  |  | 0.533 |
| ns | 5142 |  | 329 | jv.h: the string API | 4.4 |  | 0.520 |
| walker |  | 5215 | 123 | Code::CodeKey { rung: Decl, file: src/bytecode.h, decl: 5, sub: 0, line: 48 } |  |  | 0.520 |
| walker |  | 5352 | 137 | Code::CodeKey { rung: Decl, file: src/bytecode.h, decl: 8, sub: 0, line: 72 } |  |  | 0.520 |
| ns | 5353 |  | 211 | jv.h: the object API and its iterator protocol | 4.5 |  | 0.510 |
| walker |  | 5496 | 144 | Code::CodeKey { rung: Decl, file: src/bytecode.h, decl: 1, sub: 0, line: 20 } |  |  | 0.510 |
| ns | 5549 |  | 196 | jv.h: path access, keys, ordering, sort/group/unique | 4.6 |  | 0.503 |
| walker |  | 5745 | 249 | Code::CodeKey { rung: Names, file: src/lexer.h, decl: 0, sub: 0, line: 0 } |  |  | 0.503 |
| walker |  | 5750 | 5 | Code::CodeKey { rung: Decl, file: src/lexer.h, decl: 13, sub: 0, line: 288 } |  |  | 0.503 |
| walker |  | 5780 | 30 | Code::CodeKey { rung: Decl, file: src/lexer.h, decl: 18, sub: 0, line: 294 } |  |  | 0.503 |
| ns | 5811 |  | 262 | jv.h: parsing - flags, one-shot parsers, streaming jv_parser, jv_load_file | 4.7 |  | 0.490 |
| walker |  | 5818 | 38 | Code::CodeKey { rung: Decl, file: src/lexer.h, decl: 7, sub: 0, line: 267 } |  |  | 0.490 |
| walker |  | 6079 | 261 | Code::CodeKey { rung: Names, file: src/compile.h, decl: 0, sub: 0, line: 0 } |  |  | 0.490 |
| walker |  | 6095 | 16 | Code::CodeKey { rung: Decl, file: src/compile.h, decl: 3, sub: 0, line: 12 } |  |  | 0.490 |
| ns | 6129 |  | 318 | jv.h: print flags and the dump/show functions | 4.8 |  | 0.479 |
| ns | 6267 |  | 138 | jv.h: the convenience macro walls (existence, not bodies) | 4.9 |  | 0.472 |
| walker |  | 6320 | 225 | Code::CodeKey { rung: Names, file: src/jv_dtoa.h, decl: 0, sub: 0, line: 0 } |  |  | 0.472 |
| walker |  | 6352 | 32 | Code::CodeKey { rung: Decl, file: src/jv_dtoa.h, decl: 3, sub: 0, line: 6 } |  |  | 0.472 |
| walker |  | 6558 | 206 | Code::CodeKey { rung: Decl, file: src/parser.h, decl: 5, sub: 0, line: 68 } |  |  | 0.472 |
| ns | 6773 |  | 506 | opcode_list.h: every opcode and its immediate kind | 5.1 |  | 0.451 |
| walker |  | 6793 | 235 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.451 |
| walker |  | 7129 | 336 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: true } |  |  | 0.510 |
| ns | 7248 |  | 475 | parser.y: the complete token list | 5.2 |  | 0.488 |
| walker |  | 7346 | 217 | Code::CodeKey { rung: Names, file: src/jq.h, decl: 0, sub: 1, line: 0 } |  |  | 0.508 |
| ns | 7435 |  | 187 | parser.y: operator precedence and associativity | 5.3 |  | 0.500 |
| walker |  | 7682 | 336 | Code::CodeKey { rung: Names, file: src/parser.h, decl: 0, sub: 1, line: 0 } |  |  | 0.500 |
| walker |  | 7767 | 85 | Code::CodeKey { rung: Decl, file: src/parser.h, decl: 57, sub: 0, line: 178 } |  |  | 0.500 |
| walker |  | 7885 | 118 | Code::CodeKey { rung: Names, file: src/parser.h, decl: 0, sub: 2, line: 0 } |  |  | 0.500 |
| walker |  | 7890 | 5 | Code::CodeKey { rung: Decl, file: src/parser.h, decl: 64, sub: 0, line: 205 } |  |  | 0.500 |
| walker |  | 7897 | 7 | Code::CodeKey { rung: Decl, file: src/parser.h, decl: 60, sub: 0, line: 191 } |  |  | 0.500 |
| walker |  | 7918 | 21 | Code::CodeKey { rung: Decl, file: src/parser.h, decl: 61, sub: 0, line: 195 } |  |  | 0.500 |
| walker |  | 7962 | 44 | Code::CodeKey { rung: Decl, file: src/parser.h, decl: 62, sub: 0, line: 197 } |  |  | 0.500 |
| ns | 8106 |  | 671 | builtin.c: function_list, part 1 - libm, binops, conversions, keys, strings, paths, sorting | 5.4 |  | 0.483 |
| walker |  | 8166 | 204 | Code::CodeKey { rung: Decl, file: src/parser.h, decl: 5, sub: 1, line: 68 } |  |  | 0.483 |
| walker |  | 8401 | 235 | Code::CodeKey { rung: Names, file: src/jv_parse.c, decl: 0, sub: 0, line: 0 } |  |  | 0.483 |
| walker |  | 8476 | 75 | Code::CodeKey { rung: Decl, file: src/jv_parse.c, decl: 3, sub: 0, line: 25 } |  |  | 0.483 |
| ns | 8607 |  | 501 | builtin.c: function_list, part 2 - search, min/max, errors, env, regex, I/O, time | 5.5 |  | 0.471 |
| walker |  | 8811 | 335 | Code::CodeKey { rung: Decl, file: src/jv_parse.c, decl: 4, sub: 0, line: 34 } |  |  | 0.471 |
| walker |  | 8826 | 15 | Code::CodeKey { rung: Names, file: docs/build_mantests.py, decl: 0, sub: 0, line: 0 } |  |  | 0.471 |
| walker |  | 8857 | 31 | Code::CodeKey { rung: Decl, file: docs/build_mantests.py, decl: 1, sub: 0, line: 5 } |  |  | 0.471 |
| ns | 9119 |  | 512 | builtin.jq: every jq-defined builtin, part 1 (lines 1-115) | 5.6 |  | 0.448 |
| walker |  | 9130 | 273 | Code::CodeKey { rung: Names, file: src/jv.h, decl: 0, sub: 0, line: 0 } |  |  | 0.457 |
| walker |  | 9200 | 70 | Code::CodeKey { rung: Decl, file: src/jv.h, decl: 1, sub: 0, line: 19 } |  |  | 0.463 |
| walker |  | 9275 | 75 | Code::CodeKey { rung: Decl, file: src/jv.h, decl: 3, sub: 0, line: 34 } |  |  | 0.477 |
| walker |  | 9302 | 27 | Code::CodeKey { rung: Doc, file: src/jv.h, decl: 3, sub: 0, line: 34 } |  |  | 0.485 |
| walker |  | 9419 | 117 | Code::CodeKey { rung: Names, file: src/jv_dtoa_tsd.c, decl: 0, sub: 0, line: 0 } |  |  | 0.485 |
| walker |  | 9468 | 49 | Code::CodeKey { rung: Body, file: src/jv_dtoa_tsd.c, decl: 4, sub: 0, line: 22 } |  |  | 0.470 |
| ns | 9468 |  | 349 | builtin.jq: every jq-defined builtin, part 2 (lines 116-244) | 5.7 |  | 0.470 |
| walker |  | 9671 | 203 | Code::CodeKey { rung: Names, file: src/jq.h, decl: 0, sub: 2, line: 0 } |  |  | 0.485 |
| walker |  | 9758 | 87 | Code::CodeKey { rung: Doc, file: src/jq.h, decl: 34, sub: 0, line: 60 } |  |  | 0.485 |
| ns | 9778 |  | 310 | execute.c: struct jq_state, the whole interpreter state | 5.8 |  | 0.474 |
| walker |  | 9970 | 212 | Code::CodeKey { rung: Names, file: src/compile.h, decl: 0, sub: 1, line: 0 } |  |  | 0.474 |
| ns | 9977 |  | 199 | manual.yml: the section titles of the jq language reference | 6.1 |  | 0.469 |
| walker |  | 9998 | 28 | Markdown::HeadingsOutline { file: SECURITY.md } |  |  | 0.469 |
