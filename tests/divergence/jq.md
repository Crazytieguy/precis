Score(3000)=0.847 I=0.898 C=0.798 ns_rows≤3K=16/40 grid(1000/1442/2080/3000/4327/6240/9000)=0.805/0.826/0.857/0.847/0.707/0.595/0.498

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| ns | 71 |  | 71 | README lede: what jq is | 1.1 |  | 0.000 |
| walker |  | 81 | 81 | Fs::DirListing { dir: . } |  |  | 0.000 |
| walker |  | 152 | 71 | Markdown::ReadmeHeadline { file: README.md } |  |  | 1.000 |
| ns | 152 |  | 81 | Repository root listing (complete) | 1.2 |  | 1.000 |
| walker |  | 161 | 9 | Fs::DirListing { dir: config } |  |  | 1.000 |
| walker |  | 184 | 23 | Fs::DirListing { dir: m4 } |  |  | 1.000 |
| ns | 215 |  | 63 | Front-end landmarks: jq_parse / jq_parse_library / block_compile | 1.3 |  | 0.909 |
| ns | 334 |  | 119 | Back-end landmarks: load_program, builtins_bind, and who owns the bytecode format | 1.4 |  | 0.778 |
| walker |  | 417 | 233 | Fs::DirListing { dir: src } |  |  | 0.816 |
| walker |  | 469 | 52 | Fs::DirListing { dir: docs } |  |  | 0.816 |
| walker |  | 482 | 13 | Fs::DirListing { dir: docs/content } |  |  | 0.816 |
| walker |  | 486 | 4 | Fs::DirListing { dir: docs/content/download } |  |  | 0.816 |
| walker |  | 490 | 4 | Fs::DirListing { dir: docs/content/tutorial } |  |  | 0.816 |
| walker |  | 511 | 21 | Fs::DirListing { dir: docs/templates } |  |  | 0.816 |
| ns | 517 |  | 183 | README: build-from-source dependencies and the exact command sequence | 1.5 |  | 0.664 |
| walker |  | 542 | 31 | Fs::DirListing { dir: docs/public } |  |  | 0.664 |
| walker |  | 546 | 4 | Fs::DirListing { dir: docs/public/css } |  |  | 0.664 |
| walker |  | 551 | 5 | Fs::DirListing { dir: docs/public/js } |  |  | 0.664 |
| walker |  | 563 | 12 | Fs::DirListing { dir: .github } |  |  | 0.664 |
| walker |  | 598 | 35 | Fs::DirListing { dir: .github/workflows } |  |  | 0.666 |
| walker |  | 603 | 5 | Fs::DirListing { dir: .github/ISSUE_TEMPLATE } |  |  | 0.666 |
| ns | 607 |  | 90 | README: static build and released-tarball builds | 1.6 |  | 0.626 |
| walker |  | 615 | 12 | Fs::DirListing { dir: vendor } |  |  | 0.627 |
| walker |  | 636 | 21 | Fs::DirListing { dir: docs/templates/shared } |  |  | 0.627 |
| walker |  | 650 | 14 | Fs::DirListing { dir: scripts } |  |  | 0.627 |
| walker |  | 697 | 47 | Fs::DirListing { dir: config/m4 } |  |  | 0.627 |
| walker |  | 769 | 72 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.628 |
| ns | 840 |  | 233 | src/ listing (complete) - the flat core | 1.7 |  | 0.743 |
| walker |  | 924 | 155 | Markdown::CommandBlock { file: README.md, row: 45 } |  |  | 0.802 |
| walker |  | 976 | 52 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.802 |
| ns | 1005 |  | 165 | tests/ listing (complete) | 1.8 |  | 0.698 |
| walker |  | 1019 | 43 | Fs::DirListing { dir: docs/content/manual } |  |  | 0.704 |
| walker |  | 1023 | 4 | Fs::DirListing { dir: docs/content/manual/dev } |  |  | 0.704 |
| walker |  | 1027 | 4 | Fs::DirListing { dir: docs/content/manual/v1.3 } |  |  | 0.704 |
| walker |  | 1031 | 4 | Fs::DirListing { dir: docs/content/manual/v1.4 } |  |  | 0.704 |
| walker |  | 1035 | 4 | Fs::DirListing { dir: docs/content/manual/v1.5 } |  |  | 0.704 |
| walker |  | 1039 | 4 | Fs::DirListing { dir: docs/content/manual/v1.6 } |  |  | 0.704 |
| walker |  | 1043 | 4 | Fs::DirListing { dir: docs/content/manual/v1.7 } |  |  | 0.704 |
| walker |  | 1047 | 4 | Fs::DirListing { dir: docs/content/manual/v1.8 } |  |  | 0.704 |
| ns | 1113 |  | 108 | docs/ and docs/content/manual listings (complete) | 1.9 |  | 0.729 |
| ns | 1160 |  | 47 | CI workflows and vendored dependencies (complete listings) | 1.10 |  | 0.736 |
| walker |  | 1212 | 165 | Fs::DirListing { dir: tests } |  |  | 0.848 |
| walker |  | 1295 | 83 | Markdown::Section { file: README.md, section_index: 4, keeps_default_concavity: false } |  |  | 0.848 |
| ns | 1385 |  | 225 | jq.h: jq_state lifecycle - init, compile, start, next, teardown | 2.1 |  | 0.809 |
| walker |  | 1471 | 176 | Code::CodeKey { rung: Names, file: src/jq.h, decl: 0, sub: 0, line: 0 } |  |  | 0.836 |
| ns | 1503 |  | 118 | jq.h: debug-trace flags and halt / exit-code / error-message API | 2.2 |  | 0.814 |
| walker |  | 1523 | 52 | Code::CodeKey { rung: Decl, file: src/jq.h, decl: 1, sub: 0, line: 11 } |  |  | 0.821 |
| walker |  | 1704 | 181 | Code::CodeKey { rung: Names, file: src/jq.h, decl: 0, sub: 1, line: 0 } |  |  | 0.861 |
| ns | 1749 |  | 246 | jq.h: input/debug/stderr callbacks and the attribute store | 2.3 |  | 0.828 |
| walker |  | 1860 | 156 | Code::CodeKey { rung: Names, file: src/jq.h, decl: 0, sub: 2, line: 0 } |  |  | 0.857 |
| ns | 1979 |  | 230 | jq.h: jq_util_input_* file/stdin reader and jq_set_colors | 2.4 |  | 0.825 |
| walker |  | 2026 | 166 | Code::CodeKey { rung: Names, file: src/jq.h, decl: 0, sub: 3, line: 0 } |  |  | 0.846 |
| walker |  | 2124 | 98 | Code::CodeKey { rung: Names, file: src/jq.h, decl: 0, sub: 4, line: 0 } |  |  | 0.872 |
| ns | 2202 |  | 223 | main.c usage(): synopsis and one-paragraph description | 3.1 |  | 0.851 |
| walker |  | 2222 | 98 | Fs::DirListing { dir: sig } |  |  | 0.851 |
| walker |  | 2568 | 346 | Plaintext::Whole { file: Dockerfile } |  |  | 0.851 |
| walker |  | 2583 | 15 | Plaintext::DeclSurface { file: docs/public/robots.txt } |  |  | 0.851 |
| walker |  | 2588 | 5 | Fs::DirListing { dir: tests/torture } |  |  | 0.851 |
| ns | 2653 |  | 451 | main.c usage(): input and output-formatting options (-n through --seq) | 3.2 |  | 0.809 |
| walker |  | 2703 | 115 | Plaintext::DeclSurface { file: compile-ios.sh } |  |  | 0.809 |
| walker |  | 2938 | 235 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.809 |
| ns | 3043 |  | 390 | main.c usage(): program, argument and mode options (-f through --) | 3.3 |  | 0.775 |
| walker |  | 3119 | 181 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.830 |
| walker |  | 3206 | 87 | Code::CodeKey { rung: Doc, file: src/jq.h, decl: 34, sub: 0, line: 60 } |  |  | 0.830 |
| ns | 3210 |  | 167 | main.c: process exit-status codes | 3.4 |  | 0.814 |
| ns | 3433 |  | 223 | main.c: the undocumented flags (--debug-dump-disasm, --debug-trace, --run-tests) | 3.5 |  | 0.795 |
| walker |  | 3436 | 230 | Code::CodeKey { rung: Names, file: src/parser.c, decl: 0, sub: 0, line: 0 } |  |  | 0.795 |
| walker |  | 3443 | 7 | Code::CodeKey { rung: Decl, file: src/parser.c, decl: 15, sub: 0, line: 192 } |  |  | 0.795 |
| walker |  | 3459 | 16 | Code::CodeKey { rung: Decl, file: src/parser.c, decl: 10, sub: 0, line: 113 } |  |  | 0.795 |
| walker |  | 3490 | 31 | Code::CodeKey { rung: Decl, file: src/parser.c, decl: 9, sub: 0, line: 107 } |  |  | 0.795 |
| walker |  | 3665 | 175 | Code::CodeKey { rung: Decl, file: src/parser.c, decl: 14, sub: 0, line: 137 } |  |  | 0.795 |
| ns | 3673 |  | 240 | Makefile.am: the TESTS list and test environment | 3.6 |  | 0.771 |
| walker |  | 3832 | 167 | Code::CodeKey { rung: Decl, file: src/parser.c, decl: 14, sub: 1, line: 137 } |  |  | 0.771 |
| ns | 3911 |  | 238 | tests/setup + tests/jqtest: how one test driver actually runs | 3.7 |  | 0.753 |
| walker |  | 3988 | 156 | Code::CodeKey { rung: Decl, file: src/parser.c, decl: 14, sub: 2, line: 137 } |  |  | 0.753 |
| ns | 4024 |  | 113 | tests/jq.test: the three-line test format, with the first cases | 3.8 |  | 0.735 |
| walker |  | 4145 | 157 | Code::CodeKey { rung: Decl, file: src/parser.c, decl: 14, sub: 3, line: 137 } |  |  | 0.735 |
| ns | 4257 |  | 233 | jv.h: jv_kind enum and the jv struct | 4.1 |  | 0.707 |
| walker |  | 4385 | 240 | Code::CodeKey { rung: Decl, file: src/parser.c, decl: 14, sub: 4, line: 137 } |  |  | 0.707 |
| walker |  | 4393 | 8 | Code::CodeKey { rung: Doc, file: src/parser.c, decl: 3, sub: 0, line: 55 } |  |  | 0.707 |
| walker |  | 4402 | 9 | Code::CodeKey { rung: Doc, file: src/parser.c, decl: 4, sub: 0, line: 58 } |  |  | 0.707 |
| walker |  | 4411 | 9 | Code::CodeKey { rung: Doc, file: src/parser.c, decl: 5, sub: 0, line: 61 } |  |  | 0.693 |
| ns | 4411 |  | 154 | jv.h: the consume/produce refcount contract | 4.2 |  | 0.693 |
| walker |  | 4420 | 9 | Code::CodeKey { rung: Doc, file: src/parser.c, decl: 6, sub: 0, line: 64 } |  |  | 0.693 |
| walker |  | 4430 | 10 | Code::CodeKey { rung: Doc, file: src/parser.c, decl: 2, sub: 0, line: 52 } |  |  | 0.693 |
| walker |  | 4440 | 10 | Code::CodeKey { rung: Doc, file: src/parser.c, decl: 9, sub: 0, line: 107 } |  |  | 0.693 |
| walker |  | 4457 | 17 | Code::CodeKey { rung: Doc, file: src/parser.c, decl: 1, sub: 0, line: 49 } |  |  | 0.693 |
| walker |  | 4736 | 279 | Code::CodeKey { rung: Names, file: src/jv_dtoa.c, decl: 0, sub: 0, line: 0 } |  |  | 0.693 |
| walker |  | 4743 | 7 | Code::CodeKey { rung: Decl, file: src/jv_dtoa.c, decl: 10, sub: 0, line: 231 } |  |  | 0.693 |
| walker |  | 4750 | 7 | Code::CodeKey { rung: Decl, file: src/jv_dtoa.c, decl: 17, sub: 0, line: 322 } |  |  | 0.693 |
| walker |  | 4763 | 13 | Code::CodeKey { rung: Decl, file: src/jv_dtoa.c, decl: 7, sub: 0, line: 204 } |  |  | 0.693 |
| walker |  | 4777 | 14 | Code::CodeKey { rung: Decl, file: src/jv_dtoa.c, decl: 8, sub: 0, line: 207 } |  |  | 0.693 |
| walker |  | 4792 | 15 | Code::CodeKey { rung: Decl, file: src/jv_dtoa.c, decl: 12, sub: 0, line: 297 } |  |  | 0.693 |
| walker |  | 4807 | 15 | Code::CodeKey { rung: Decl, file: src/jv_dtoa.c, decl: 15, sub: 0, line: 316 } |  |  | 0.693 |
| ns | 4813 |  | 402 | jv.h: comparison, invalid-with-message, constructors, numbers, arrays | 4.3 |  | 0.662 |
| walker |  | 4854 | 47 | Code::CodeKey { rung: Decl, file: src/jv_dtoa.c, decl: 11, sub: 0, line: 256 } |  |  | 0.662 |
| walker |  | 5056 | 202 | Code::CodeKey { rung: Names, file: src/jv_dtoa.c, decl: 0, sub: 1, line: 0 } |  |  | 0.662 |
| walker |  | 5061 | 5 | Code::CodeKey { rung: Decl, file: src/jv_dtoa.c, decl: 22, sub: 0, line: 475 } |  |  | 0.662 |
| walker |  | 5074 | 13 | Code::CodeKey { rung: Decl, file: src/jv_dtoa.c, decl: 25, sub: 0, line: 481 } |  |  | 0.662 |
| walker |  | 5112 | 38 | Code::CodeKey { rung: Decl, file: src/jv_dtoa.c, decl: 27, sub: 0, line: 486 } |  |  | 0.662 |
| ns | 5142 |  | 329 | jv.h: the string API | 4.4 |  | 0.645 |
| ns | 5353 |  | 211 | jv.h: the object API and its iterator protocol | 4.5 |  | 0.634 |
| walker |  | 5358 | 246 | Code::CodeKey { rung: Names, file: src/jv_dtoa.c, decl: 0, sub: 2, line: 0 } |  |  | 0.634 |
| walker |  | 5363 | 5 | Code::CodeKey { rung: Decl, file: src/jv_dtoa.c, decl: 34, sub: 0, line: 530 } |  |  | 0.634 |
| walker |  | 5377 | 14 | Code::CodeKey { rung: Decl, file: src/jv_dtoa.c, decl: 29, sub: 0, line: 491 } |  |  | 0.634 |
| walker |  | 5398 | 21 | Code::CodeKey { rung: Decl, file: src/jv_dtoa.c, decl: 40, sub: 0, line: 676 } |  |  | 0.634 |
| walker |  | 5420 | 22 | Code::CodeKey { rung: Decl, file: src/jv_dtoa.c, decl: 35, sub: 0, line: 550 } |  |  | 0.634 |
| walker |  | 5444 | 24 | Code::CodeKey { rung: Decl, file: src/jv_dtoa.c, decl: 30, sub: 0, line: 505 } |  |  | 0.634 |
| walker |  | 5481 | 37 | Code::CodeKey { rung: Decl, file: src/jv_dtoa.c, decl: 38, sub: 0, line: 589 } |  |  | 0.634 |
| walker |  | 5519 | 38 | Code::CodeKey { rung: Decl, file: src/jv_dtoa.c, decl: 39, sub: 0, line: 642 } |  |  | 0.634 |
| ns | 5549 |  | 196 | jv.h: path access, keys, ordering, sort/group/unique | 4.6 |  | 0.625 |
| walker |  | 5567 | 48 | Code::CodeKey { rung: Decl, file: src/jv_dtoa.c, decl: 31, sub: 0, line: 512 } |  |  | 0.625 |
| ns | 5811 |  | 262 | jv.h: parsing - flags, one-shot parsers, streaming jv_parser, jv_load_file | 4.7 |  | 0.609 |
| walker |  | 5906 | 339 | Code::CodeKey { rung: Names, file: src/jv_dtoa.c, decl: 0, sub: 3, line: 0 } |  |  | 0.609 |
| walker |  | 5926 | 20 | Code::CodeKey { rung: Decl, file: src/jv_dtoa.c, decl: 42, sub: 0, line: 750 } |  |  | 0.609 |
| walker |  | 5948 | 22 | Code::CodeKey { rung: Decl, file: src/jv_dtoa.c, decl: 41, sub: 0, line: 706 } |  |  | 0.609 |
| walker |  | 5973 | 25 | Code::CodeKey { rung: Decl, file: src/jv_dtoa.c, decl: 44, sub: 0, line: 869 } |  |  | 0.609 |
| walker |  | 5998 | 25 | Code::CodeKey { rung: Decl, file: src/jv_dtoa.c, decl: 45, sub: 0, line: 904 } |  |  | 0.609 |
| walker |  | 6025 | 27 | Code::CodeKey { rung: Decl, file: src/jv_dtoa.c, decl: 43, sub: 0, line: 762 } |  |  | 0.609 |
| walker |  | 6052 | 27 | Code::CodeKey { rung: Decl, file: src/jv_dtoa.c, decl: 46, sub: 0, line: 960 } |  |  | 0.609 |
| walker |  | 6079 | 27 | Code::CodeKey { rung: Decl, file: src/jv_dtoa.c, decl: 47, sub: 0, line: 990 } |  |  | 0.609 |
| walker |  | 6107 | 28 | Code::CodeKey { rung: Decl, file: src/jv_dtoa.c, decl: 49, sub: 0, line: 1118 } |  |  | 0.609 |
| ns | 6129 |  | 318 | jv.h: print flags and the dump/show functions | 4.8 |  | 0.595 |
| walker |  | 6136 | 29 | Code::CodeKey { rung: Decl, file: src/jv_dtoa.c, decl: 51, sub: 0, line: 1314 } |  |  | 0.595 |
| walker |  | 6167 | 31 | Code::CodeKey { rung: Decl, file: src/jv_dtoa.c, decl: 50, sub: 0, line: 1184 } |  |  | 0.595 |
| walker |  | 6259 | 92 | Code::CodeKey { rung: Decl, file: src/jv_dtoa.c, decl: 54, sub: 0, line: 1362 } |  |  | 0.595 |
| ns | 6267 |  | 138 | jv.h: the convenience macro walls (existence, not bodies) | 4.9 |  | 0.586 |
| walker |  | 6444 | 185 | Code::CodeKey { rung: Decl, file: src/jv_dtoa.c, decl: 52, sub: 0, line: 1351 } |  |  | 0.586 |
| walker |  | 6710 | 266 | Code::CodeKey { rung: Names, file: src/jv_dtoa.c, decl: 0, sub: 4, line: 0 } |  |  | 0.586 |
| walker |  | 6716 | 6 | Code::CodeKey { rung: Decl, file: src/jv_dtoa.c, decl: 62, sub: 0, line: 1404 } |  |  | 0.586 |
| walker |  | 6722 | 6 | Code::CodeKey { rung: Decl, file: src/jv_dtoa.c, decl: 69, sub: 0, line: 1557 } |  |  | 0.586 |
| walker |  | 6736 | 14 | Code::CodeKey { rung: Decl, file: src/jv_dtoa.c, decl: 61, sub: 0, line: 1401 } |  |  | 0.586 |
| walker |  | 6752 | 16 | Code::CodeKey { rung: Decl, file: src/jv_dtoa.c, decl: 60, sub: 0, line: 1384 } |  |  | 0.586 |
| walker |  | 6769 | 17 | Code::CodeKey { rung: Decl, file: src/jv_dtoa.c, decl: 65, sub: 0, line: 1428 } |  |  | 0.586 |
| ns | 6773 |  | 506 | opcode_list.h: every opcode and its immediate kind | 5.1 |  | 0.560 |
| walker |  | 6791 | 22 | Code::CodeKey { rung: Decl, file: src/jv_dtoa.c, decl: 63, sub: 0, line: 1412 } |  |  | 0.560 |
| walker |  | 6820 | 29 | Code::CodeKey { rung: Decl, file: src/jv_dtoa.c, decl: 66, sub: 0, line: 1432 } |  |  | 0.560 |
| walker |  | 6853 | 33 | Code::CodeKey { rung: Decl, file: src/jv_dtoa.c, decl: 64, sub: 0, line: 1422 } |  |  | 0.560 |
| walker |  | 6896 | 43 | Code::CodeKey { rung: Decl, file: src/jv_dtoa.c, decl: 68, sub: 0, line: 1525 } |  |  | 0.560 |
| walker |  | 6969 | 73 | Code::CodeKey { rung: Decl, file: src/jv_dtoa.c, decl: 67, sub: 0, line: 1449 } |  |  | 0.560 |
| walker |  | 7177 | 208 | Code::CodeKey { rung: Names, file: src/jv_dtoa.c, decl: 0, sub: 5, line: 0 } |  |  | 0.560 |
| walker |  | 7183 | 6 | Code::CodeKey { rung: Decl, file: src/jv_dtoa.c, decl: 70, sub: 0, line: 1586 } |  |  | 0.560 |
| walker |  | 7189 | 6 | Code::CodeKey { rung: Decl, file: src/jv_dtoa.c, decl: 78, sub: 0, line: 1976 } |  |  | 0.560 |
| walker |  | 7202 | 13 | Code::CodeKey { rung: Decl, file: src/jv_dtoa.c, decl: 76, sub: 0, line: 1627 } |  |  | 0.560 |
| ns | 7248 |  | 475 | parser.y: the complete token list | 5.2 |  | 0.536 |
| walker |  | 7258 | 56 | Code::CodeKey { rung: Decl, file: src/jv_dtoa.c, decl: 71, sub: 0, line: 1612 } |  |  | 0.536 |
| walker |  | 7427 | 169 | Code::CodeKey { rung: Decl, file: src/jv_dtoa.c, decl: 77, sub: 0, line: 1630 } |  |  | 0.536 |
| ns | 7435 |  | 187 | parser.y: operator precedence and associativity | 5.3 |  | 0.529 |
| walker |  | 7621 | 194 | Code::CodeKey { rung: Names, file: src/jv_dtoa.c, decl: 0, sub: 6, line: 0 } |  |  | 0.529 |
| walker |  | 7640 | 19 | Code::CodeKey { rung: Decl, file: src/jv_dtoa.c, decl: 85, sub: 0, line: 2333 } |  |  | 0.529 |
| walker |  | 7665 | 25 | Code::CodeKey { rung: Decl, file: src/jv_dtoa.c, decl: 82, sub: 0, line: 1995 } |  |  | 0.529 |
| walker |  | 7731 | 66 | Code::CodeKey { rung: Decl, file: src/jv_dtoa.c, decl: 83, sub: 0, line: 2105 } |  |  | 0.529 |
| walker |  | 7949 | 218 | Code::CodeKey { rung: Names, file: src/jv_dtoa.c, decl: 0, sub: 7, line: 0 } |  |  | 0.529 |
| walker |  | 7955 | 6 | Code::CodeKey { rung: Decl, file: src/jv_dtoa.c, decl: 98, sub: 0, line: 3394 } |  |  | 0.529 |
| walker |  | 7961 | 6 | Code::CodeKey { rung: Decl, file: src/jv_dtoa.c, decl: 99, sub: 0, line: 3410 } |  |  | 0.529 |
| walker |  | 7979 | 18 | Code::CodeKey { rung: Decl, file: src/jv_dtoa.c, decl: 94, sub: 0, line: 2344 } |  |  | 0.529 |
| walker |  | 7997 | 18 | Code::CodeKey { rung: Decl, file: src/jv_dtoa.c, decl: 95, sub: 0, line: 2347 } |  |  | 0.529 |
| walker |  | 8016 | 19 | Code::CodeKey { rung: Decl, file: src/jv_dtoa.c, decl: 97, sub: 0, line: 2365 } |  |  | 0.529 |
| walker |  | 8036 | 20 | Code::CodeKey { rung: Decl, file: src/jv_dtoa.c, decl: 96, sub: 0, line: 2350 } |  |  | 0.529 |
| ns | 8106 |  | 671 | builtin.c: function_list, part 1 - libm, binops, conversions, keys, strings, paths, sorting | 5.4 |  | 0.510 |
| walker |  | 8306 | 270 | Code::CodeKey { rung: Names, file: src/jv_dtoa.c, decl: 0, sub: 8, line: 0 } |  |  | 0.510 |
| walker |  | 8312 | 6 | Code::CodeKey { rung: Decl, file: src/jv_dtoa.c, decl: 100, sub: 0, line: 3428 } |  |  | 0.510 |
| walker |  | 8318 | 6 | Code::CodeKey { rung: Decl, file: src/jv_dtoa.c, decl: 104, sub: 0, line: 3514 } |  |  | 0.510 |
| walker |  | 8324 | 6 | Code::CodeKey { rung: Decl, file: src/jv_dtoa.c, decl: 111, sub: 0, line: 4205 } |  |  | 0.510 |
| walker |  | 8335 | 11 | Code::CodeKey { rung: Decl, file: src/jv_dtoa.c, decl: 103, sub: 0, line: 3512 } |  |  | 0.510 |
| walker |  | 8355 | 20 | Code::CodeKey { rung: Decl, file: src/jv_dtoa.c, decl: 110, sub: 0, line: 3525 } |  |  | 0.510 |
| walker |  | 8392 | 37 | Code::CodeKey { rung: Decl, file: src/jv_dtoa.c, decl: 109, sub: 0, line: 3520 } |  |  | 0.510 |
| ns | 8607 |  | 501 | builtin.c: function_list, part 2 - search, min/max, errors, env, regex, I/O, time | 5.5 |  | 0.498 |
| walker |  | 8670 | 278 | Code::CodeKey { rung: Names, file: src/parser.c, decl: 0, sub: 1, line: 0 } |  |  | 0.498 |
| walker |  | 8678 | 8 | Code::CodeKey { rung: Doc, file: src/parser.c, decl: 16, sub: 0, line: 195 } |  |  | 0.498 |
| walker |  | 8937 | 259 | Code::CodeKey { rung: Names, file: src/parser.c, decl: 0, sub: 2, line: 0 } |  |  | 0.498 |
| ns | 9119 |  | 512 | builtin.jq: every jq-defined builtin, part 1 (lines 1-115) | 5.6 |  | 0.473 |
| walker |  | 9146 | 209 | Code::CodeKey { rung: Names, file: src/parser.c, decl: 0, sub: 3, line: 0 } |  |  | 0.473 |
| walker |  | 9151 | 5 | Code::CodeKey { rung: Decl, file: src/parser.c, decl: 73, sub: 0, line: 274 } |  |  | 0.473 |
| walker |  | 9158 | 7 | Code::CodeKey { rung: Decl, file: src/parser.c, decl: 69, sub: 0, line: 260 } |  |  | 0.473 |
| walker |  | 9172 | 14 | Code::CodeKey { rung: Decl, file: src/parser.c, decl: 77, sub: 0, line: 396 } |  |  | 0.473 |
| walker |  | 9193 | 21 | Code::CodeKey { rung: Decl, file: src/parser.c, decl: 70, sub: 0, line: 264 } |  |  | 0.473 |
| walker |  | 9221 | 28 | Code::CodeKey { rung: Decl, file: src/parser.c, decl: 74, sub: 0, line: 280 } |  |  | 0.473 |
| walker |  | 9265 | 44 | Code::CodeKey { rung: Decl, file: src/parser.c, decl: 71, sub: 0, line: 266 } |  |  | 0.473 |
| walker |  | 9350 | 85 | Code::CodeKey { rung: Decl, file: src/parser.c, decl: 66, sub: 0, line: 247 } |  |  | 0.473 |
| walker |  | 9444 | 94 | Code::CodeKey { rung: Decl, file: src/parser.c, decl: 78, sub: 0, line: 399 } |  |  | 0.473 |
| walker |  | 9452 | 8 | Code::CodeKey { rung: Doc, file: src/parser.c, decl: 66, sub: 0, line: 247 } |  |  | 0.473 |
| walker |  | 9460 | 8 | Code::CodeKey { rung: Doc, file: src/parser.c, decl: 70, sub: 0, line: 264 } |  |  | 0.473 |
| ns | 9468 |  | 349 | builtin.jq: every jq-defined builtin, part 2 (lines 116-244) | 5.7 |  | 0.459 |
| walker |  | 9616 | 156 | Code::CodeKey { rung: Decl, file: src/parser.c, decl: 75, sub: 0, line: 285 } |  |  | 0.459 |
| walker |  | 9624 | 8 | Code::CodeKey { rung: Doc, file: src/parser.c, decl: 75, sub: 0, line: 285 } |  |  | 0.459 |
| ns | 9778 |  | 310 | execute.c: struct jq_state, the whole interpreter state | 5.8 |  | 0.449 |
| walker |  | 9788 | 164 | Code::CodeKey { rung: Decl, file: src/parser.c, decl: 75, sub: 1, line: 285 } |  |  | 0.449 |
| walker |  | 9936 | 148 | Code::CodeKey { rung: Decl, file: src/parser.c, decl: 75, sub: 2, line: 285 } |  |  | 0.449 |
| ns | 9977 |  | 199 | manual.yml: the section titles of the jq language reference | 6.1 |  | 0.445 |
| walker |  | 9981 | 45 | Code::CodeKey { rung: Decl, file: src/parser.c, decl: 75, sub: 3, line: 285 } |  |  | 0.445 |
