Score(3000)=0.850 I=0.901 C=0.802 ns_rows≤3K=16/40 grid(1000/1442/2080/3000/4327/6240/9000)=0.750/0.803/0.839/0.850/0.707/0.595/0.498

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| ns | 71 |  | 71 | README lede: what jq is | 1.1 |  | 0.000 |
| walker |  | 81 | 81 | Fs::DirListing { dir: . } |  |  | 0.000 |
| walker |  | 152 | 71 | Markdown::ReadmeHeadline { file: README.md } |  |  | 1.000 |
| ns | 152 |  | 81 | Repository root listing (complete) | 1.2 |  | 1.000 |
| walker |  | 161 | 9 | Fs::DirListing { dir: config } |  |  | 1.000 |
| walker |  | 184 | 23 | Fs::DirListing { dir: m4 } |  |  | 1.000 |
| ns | 215 |  | 63 | Front-end landmarks: jq_parse / jq_parse_library / block_compile | 1.3 |  | 0.909 |
| walker |  | 236 | 52 | Fs::DirListing { dir: docs } |  |  | 0.913 |
| walker |  | 249 | 13 | Fs::DirListing { dir: docs/content } |  |  | 0.916 |
| walker |  | 253 | 4 | Fs::DirListing { dir: docs/content/download } |  |  | 0.916 |
| walker |  | 257 | 4 | Fs::DirListing { dir: docs/content/tutorial } |  |  | 0.916 |
| walker |  | 278 | 21 | Fs::DirListing { dir: docs/templates } |  |  | 0.916 |
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
| walker |  | 588 | 52 | Markdown::CommandBlock { file: README.md, row: 24 } |  |  | 0.632 |
| ns | 607 |  | 90 | README: static build and released-tarball builds | 1.6 |  | 0.594 |
| walker |  | 640 | 52 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.594 |
| walker |  | 683 | 43 | Fs::DirListing { dir: docs/content/manual } |  |  | 0.600 |
| walker |  | 687 | 4 | Fs::DirListing { dir: docs/content/manual/dev } |  |  | 0.600 |
| walker |  | 691 | 4 | Fs::DirListing { dir: docs/content/manual/v1.3 } |  |  | 0.600 |
| walker |  | 695 | 4 | Fs::DirListing { dir: docs/content/manual/v1.4 } |  |  | 0.600 |
| walker |  | 699 | 4 | Fs::DirListing { dir: docs/content/manual/v1.5 } |  |  | 0.600 |
| walker |  | 703 | 4 | Fs::DirListing { dir: docs/content/manual/v1.6 } |  |  | 0.600 |
| walker |  | 707 | 4 | Fs::DirListing { dir: docs/content/manual/v1.7 } |  |  | 0.600 |
| walker |  | 711 | 4 | Fs::DirListing { dir: docs/content/manual/v1.8 } |  |  | 0.600 |
| ns | 840 |  | 233 | src/ listing (complete) - the flat core | 1.7 |  | 0.449 |
| walker |  | 944 | 233 | Fs::DirListing { dir: src } |  |  | 0.749 |
| ns | 1005 |  | 165 | tests/ listing (complete) | 1.8 |  | 0.652 |
| walker |  | 1109 | 165 | Fs::DirListing { dir: tests } |  |  | 0.794 |
| ns | 1113 |  | 108 | docs/ and docs/content/manual listings (complete) | 1.9 |  | 0.805 |
| ns | 1160 |  | 47 | CI workflows and vendored dependencies (complete listings) | 1.10 |  | 0.808 |
| walker |  | 1192 | 83 | Markdown::Section { file: README.md, section_index: 4, keeps_default_concavity: false } |  |  | 0.808 |
| walker |  | 1368 | 176 | Code::CodeKey { rung: Names, file: src/jq.h, decl: 0, sub: 0, line: 0 } |  |  | 0.811 |
| ns | 1385 |  | 225 | jq.h: jq_state lifecycle - init, compile, start, next, teardown | 2.1 |  | 0.797 |
| walker |  | 1420 | 52 | Code::CodeKey { rung: Decl, file: src/jq.h, decl: 1, sub: 0, line: 11 } |  |  | 0.798 |
| ns | 1503 |  | 118 | jq.h: debug-trace flags and halt / exit-code / error-message API | 2.2 |  | 0.784 |
| walker |  | 1601 | 181 | Code::CodeKey { rung: Names, file: src/jq.h, decl: 0, sub: 1, line: 0 } |  |  | 0.824 |
| ns | 1749 |  | 246 | jq.h: input/debug/stderr callbacks and the attribute store | 2.3 |  | 0.792 |
| walker |  | 1757 | 156 | Code::CodeKey { rung: Names, file: src/jq.h, decl: 0, sub: 2, line: 0 } |  |  | 0.822 |
| walker |  | 1923 | 166 | Code::CodeKey { rung: Names, file: src/jq.h, decl: 0, sub: 3, line: 0 } |  |  | 0.833 |
| ns | 1979 |  | 230 | jq.h: jq_util_input_* file/stdin reader and jq_set_colors | 2.4 |  | 0.812 |
| walker |  | 2021 | 98 | Code::CodeKey { rung: Names, file: src/jq.h, decl: 0, sub: 4, line: 0 } |  |  | 0.839 |
| walker |  | 2119 | 98 | Fs::DirListing { dir: sig } |  |  | 0.839 |
| ns | 2202 |  | 223 | main.c usage(): synopsis and one-paragraph description | 3.1 |  | 0.818 |
| walker |  | 2465 | 346 | Plaintext::Whole { file: Dockerfile } |  |  | 0.818 |
| walker |  | 2480 | 15 | Plaintext::DeclSurface { file: docs/public/robots.txt } |  |  | 0.818 |
| walker |  | 2485 | 5 | Fs::DirListing { dir: tests/torture } |  |  | 0.818 |
| ns | 2653 |  | 451 | main.c usage(): input and output-formatting options (-n through --seq) | 3.2 |  | 0.778 |
| walker |  | 2668 | 183 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.778 |
| walker |  | 2780 | 112 | Plaintext::DeclSurface { file: compile-ios.sh } |  |  | 0.778 |
| ns | 3043 |  | 390 | main.c usage(): program, argument and mode options (-f through --) | 3.3 |  | 0.745 |
| walker |  | 3116 | 336 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.830 |
| walker |  | 3203 | 87 | Code::CodeKey { rung: Doc, file: src/jq.h, decl: 34, sub: 0, line: 60 } |  |  | 0.830 |
| ns | 3210 |  | 167 | main.c: process exit-status codes | 3.4 |  | 0.814 |
| walker |  | 3433 | 230 | Code::CodeKey { rung: Names, file: src/parser.c, decl: 0, sub: 0, line: 0 } |  |  | 0.795 |
| ns | 3433 |  | 223 | main.c: the undocumented flags (--debug-dump-disasm, --debug-trace, --run-tests) | 3.5 |  | 0.795 |
| walker |  | 3440 | 7 | Code::CodeKey { rung: Decl, file: src/parser.c, decl: 15, sub: 0, line: 192 } |  |  | 0.795 |
| walker |  | 3456 | 16 | Code::CodeKey { rung: Decl, file: src/parser.c, decl: 10, sub: 0, line: 113 } |  |  | 0.795 |
| walker |  | 3487 | 31 | Code::CodeKey { rung: Decl, file: src/parser.c, decl: 9, sub: 0, line: 107 } |  |  | 0.795 |
| walker |  | 3631 | 144 | Code::CodeKey { rung: Decl, file: src/parser.c, decl: 13, sub: 0, line: 123 } |  |  | 0.795 |
| ns | 3673 |  | 240 | Makefile.am: the TESTS list and test environment | 3.6 |  | 0.771 |
| walker |  | 3800 | 169 | Code::CodeKey { rung: Decl, file: src/parser.c, decl: 14, sub: 0, line: 137 } |  |  | 0.771 |
| ns | 3911 |  | 238 | tests/setup + tests/jqtest: how one test driver actually runs | 3.7 |  | 0.753 |
| walker |  | 3967 | 167 | Code::CodeKey { rung: Decl, file: src/parser.c, decl: 14, sub: 1, line: 137 } |  |  | 0.753 |
| ns | 4024 |  | 113 | tests/jq.test: the three-line test format, with the first cases | 3.8 |  | 0.735 |
| walker |  | 4123 | 156 | Code::CodeKey { rung: Decl, file: src/parser.c, decl: 14, sub: 2, line: 137 } |  |  | 0.735 |
| ns | 4257 |  | 233 | jv.h: jv_kind enum and the jv struct | 4.1 |  | 0.707 |
| walker |  | 4280 | 157 | Code::CodeKey { rung: Decl, file: src/parser.c, decl: 14, sub: 3, line: 137 } |  |  | 0.707 |
| ns | 4411 |  | 154 | jv.h: the consume/produce refcount contract | 4.2 |  | 0.693 |
| walker |  | 4526 | 246 | Code::CodeKey { rung: Decl, file: src/parser.c, decl: 14, sub: 4, line: 137 } |  |  | 0.693 |
| walker |  | 4534 | 8 | Code::CodeKey { rung: Doc, file: src/parser.c, decl: 3, sub: 0, line: 55 } |  |  | 0.693 |
| walker |  | 4543 | 9 | Code::CodeKey { rung: Doc, file: src/parser.c, decl: 4, sub: 0, line: 58 } |  |  | 0.693 |
| walker |  | 4552 | 9 | Code::CodeKey { rung: Doc, file: src/parser.c, decl: 5, sub: 0, line: 61 } |  |  | 0.693 |
| walker |  | 4561 | 9 | Code::CodeKey { rung: Doc, file: src/parser.c, decl: 6, sub: 0, line: 64 } |  |  | 0.693 |
| walker |  | 4571 | 10 | Code::CodeKey { rung: Doc, file: src/parser.c, decl: 2, sub: 0, line: 52 } |  |  | 0.693 |
| walker |  | 4588 | 17 | Code::CodeKey { rung: Doc, file: src/parser.c, decl: 1, sub: 0, line: 49 } |  |  | 0.693 |
| ns | 4813 |  | 402 | jv.h: comparison, invalid-with-message, constructors, numbers, arrays | 4.3 |  | 0.662 |
| walker |  | 4846 | 258 | Code::CodeKey { rung: Names, file: src/jv_dtoa.c, decl: 0, sub: 0, line: 0 } |  |  | 0.662 |
| walker |  | 4855 | 9 | Code::CodeKey { rung: Decl, file: src/jv_dtoa.c, decl: 9, sub: 0, line: 229 } |  |  | 0.662 |
| walker |  | 4866 | 11 | Code::CodeKey { rung: Decl, file: src/jv_dtoa.c, decl: 16, sub: 0, line: 320 } |  |  | 0.662 |
| walker |  | 4878 | 12 | Code::CodeKey { rung: Decl, file: src/jv_dtoa.c, decl: 10, sub: 0, line: 231 } |  |  | 0.662 |
| walker |  | 4890 | 12 | Code::CodeKey { rung: Decl, file: src/jv_dtoa.c, decl: 17, sub: 0, line: 322 } |  |  | 0.662 |
| walker |  | 4903 | 13 | Code::CodeKey { rung: Decl, file: src/jv_dtoa.c, decl: 7, sub: 0, line: 204 } |  |  | 0.662 |
| walker |  | 4916 | 13 | Code::CodeKey { rung: Decl, file: src/jv_dtoa.c, decl: 18, sub: 0, line: 469 } |  |  | 0.662 |
| walker |  | 4930 | 14 | Code::CodeKey { rung: Decl, file: src/jv_dtoa.c, decl: 8, sub: 0, line: 207 } |  |  | 0.662 |
| walker |  | 4945 | 15 | Code::CodeKey { rung: Decl, file: src/jv_dtoa.c, decl: 12, sub: 0, line: 297 } |  |  | 0.662 |
| walker |  | 4960 | 15 | Code::CodeKey { rung: Decl, file: src/jv_dtoa.c, decl: 15, sub: 0, line: 316 } |  |  | 0.662 |
| walker |  | 5019 | 59 | Code::CodeKey { rung: Decl, file: src/jv_dtoa.c, decl: 11, sub: 0, line: 256 } |  |  | 0.662 |
| ns | 5142 |  | 329 | jv.h: the string API | 4.4 |  | 0.645 |
| walker |  | 5231 | 212 | Code::CodeKey { rung: Names, file: src/jv_dtoa.c, decl: 0, sub: 1, line: 0 } |  |  | 0.645 |
| walker |  | 5236 | 5 | Code::CodeKey { rung: Decl, file: src/jv_dtoa.c, decl: 21, sub: 0, line: 473 } |  |  | 0.645 |
| walker |  | 5241 | 5 | Code::CodeKey { rung: Decl, file: src/jv_dtoa.c, decl: 22, sub: 0, line: 475 } |  |  | 0.645 |
| walker |  | 5254 | 13 | Code::CodeKey { rung: Decl, file: src/jv_dtoa.c, decl: 25, sub: 0, line: 481 } |  |  | 0.645 |
| walker |  | 5280 | 26 | Code::CodeKey { rung: Decl, file: src/jv_dtoa.c, decl: 30, sub: 0, line: 505 } |  |  | 0.645 |
| walker |  | 5316 | 36 | Code::CodeKey { rung: Decl, file: src/jv_dtoa.c, decl: 29, sub: 0, line: 491 } |  |  | 0.645 |
| ns | 5353 |  | 211 | jv.h: the object API and its iterator protocol | 4.5 |  | 0.634 |
| walker |  | 5354 | 38 | Code::CodeKey { rung: Decl, file: src/jv_dtoa.c, decl: 27, sub: 0, line: 486 } |  |  | 0.634 |
| walker |  | 5402 | 48 | Code::CodeKey { rung: Decl, file: src/jv_dtoa.c, decl: 31, sub: 0, line: 512 } |  |  | 0.634 |
| ns | 5549 |  | 196 | jv.h: path access, keys, ordering, sort/group/unique | 4.6 |  | 0.625 |
| walker |  | 5651 | 249 | Code::CodeKey { rung: Names, file: src/jv_dtoa.c, decl: 0, sub: 2, line: 0 } |  |  | 0.625 |
| walker |  | 5656 | 5 | Code::CodeKey { rung: Decl, file: src/jv_dtoa.c, decl: 34, sub: 0, line: 530 } |  |  | 0.625 |
| walker |  | 5671 | 15 | Code::CodeKey { rung: Decl, file: src/jv_dtoa.c, decl: 37, sub: 0, line: 586 } |  |  | 0.625 |
| walker |  | 5691 | 20 | Code::CodeKey { rung: Decl, file: src/jv_dtoa.c, decl: 42, sub: 0, line: 750 } |  |  | 0.625 |
| walker |  | 5712 | 21 | Code::CodeKey { rung: Decl, file: src/jv_dtoa.c, decl: 40, sub: 0, line: 676 } |  |  | 0.625 |
| walker |  | 5734 | 22 | Code::CodeKey { rung: Decl, file: src/jv_dtoa.c, decl: 35, sub: 0, line: 550 } |  |  | 0.625 |
| walker |  | 5756 | 22 | Code::CodeKey { rung: Decl, file: src/jv_dtoa.c, decl: 41, sub: 0, line: 706 } |  |  | 0.625 |
| walker |  | 5781 | 25 | Code::CodeKey { rung: Decl, file: src/jv_dtoa.c, decl: 44, sub: 0, line: 869 } |  |  | 0.625 |
| walker |  | 5806 | 25 | Code::CodeKey { rung: Decl, file: src/jv_dtoa.c, decl: 45, sub: 0, line: 904 } |  |  | 0.625 |
| ns | 5811 |  | 262 | jv.h: parsing - flags, one-shot parsers, streaming jv_parser, jv_load_file | 4.7 |  | 0.609 |
| walker |  | 5833 | 27 | Code::CodeKey { rung: Decl, file: src/jv_dtoa.c, decl: 43, sub: 0, line: 762 } |  |  | 0.609 |
| walker |  | 5870 | 37 | Code::CodeKey { rung: Decl, file: src/jv_dtoa.c, decl: 38, sub: 0, line: 589 } |  |  | 0.609 |
| walker |  | 5908 | 38 | Code::CodeKey { rung: Decl, file: src/jv_dtoa.c, decl: 39, sub: 0, line: 642 } |  |  | 0.609 |
| ns | 6129 |  | 318 | jv.h: print flags and the dump/show functions | 4.8 |  | 0.595 |
| walker |  | 6229 | 321 | Code::CodeKey { rung: Names, file: src/jv_dtoa.c, decl: 0, sub: 3, line: 0 } |  |  | 0.595 |
| walker |  | 6235 | 6 | Code::CodeKey { rung: Decl, file: src/jv_dtoa.c, decl: 55, sub: 0, line: 1404 } |  |  | 0.595 |
| walker |  | 6241 | 6 | Code::CodeKey { rung: Decl, file: src/jv_dtoa.c, decl: 62, sub: 0, line: 1557 } |  |  | 0.595 |
| walker |  | 6255 | 14 | Code::CodeKey { rung: Decl, file: src/jv_dtoa.c, decl: 54, sub: 0, line: 1401 } |  |  | 0.595 |
| ns | 6267 |  | 138 | jv.h: the convenience macro walls (existence, not bodies) | 4.9 |  | 0.586 |
| walker |  | 6272 | 17 | Code::CodeKey { rung: Decl, file: src/jv_dtoa.c, decl: 58, sub: 0, line: 1428 } |  |  | 0.586 |
| walker |  | 6294 | 22 | Code::CodeKey { rung: Decl, file: src/jv_dtoa.c, decl: 56, sub: 0, line: 1412 } |  |  | 0.586 |
| walker |  | 6321 | 27 | Code::CodeKey { rung: Decl, file: src/jv_dtoa.c, decl: 46, sub: 0, line: 960 } |  |  | 0.586 |
| walker |  | 6348 | 27 | Code::CodeKey { rung: Decl, file: src/jv_dtoa.c, decl: 47, sub: 0, line: 990 } |  |  | 0.586 |
| walker |  | 6376 | 28 | Code::CodeKey { rung: Decl, file: src/jv_dtoa.c, decl: 49, sub: 0, line: 1118 } |  |  | 0.586 |
| walker |  | 6405 | 29 | Code::CodeKey { rung: Decl, file: src/jv_dtoa.c, decl: 51, sub: 0, line: 1314 } |  |  | 0.586 |
| walker |  | 6434 | 29 | Code::CodeKey { rung: Decl, file: src/jv_dtoa.c, decl: 59, sub: 0, line: 1432 } |  |  | 0.586 |
| walker |  | 6465 | 31 | Code::CodeKey { rung: Decl, file: src/jv_dtoa.c, decl: 50, sub: 0, line: 1184 } |  |  | 0.586 |
| walker |  | 6498 | 33 | Code::CodeKey { rung: Decl, file: src/jv_dtoa.c, decl: 57, sub: 0, line: 1422 } |  |  | 0.586 |
| walker |  | 6541 | 43 | Code::CodeKey { rung: Decl, file: src/jv_dtoa.c, decl: 61, sub: 0, line: 1525 } |  |  | 0.586 |
| walker |  | 6614 | 73 | Code::CodeKey { rung: Decl, file: src/jv_dtoa.c, decl: 60, sub: 0, line: 1449 } |  |  | 0.586 |
| ns | 6773 |  | 506 | opcode_list.h: every opcode and its immediate kind | 5.1 |  | 0.560 |
| walker |  | 6799 | 185 | Code::CodeKey { rung: Decl, file: src/jv_dtoa.c, decl: 52, sub: 0, line: 1351 } |  |  | 0.560 |
| walker |  | 7007 | 208 | Code::CodeKey { rung: Names, file: src/jv_dtoa.c, decl: 0, sub: 4, line: 0 } |  |  | 0.560 |
| walker |  | 7013 | 6 | Code::CodeKey { rung: Decl, file: src/jv_dtoa.c, decl: 63, sub: 0, line: 1586 } |  |  | 0.560 |
| walker |  | 7019 | 6 | Code::CodeKey { rung: Decl, file: src/jv_dtoa.c, decl: 71, sub: 0, line: 1976 } |  |  | 0.560 |
| walker |  | 7032 | 13 | Code::CodeKey { rung: Decl, file: src/jv_dtoa.c, decl: 69, sub: 0, line: 1627 } |  |  | 0.560 |
| walker |  | 7088 | 56 | Code::CodeKey { rung: Decl, file: src/jv_dtoa.c, decl: 64, sub: 0, line: 1612 } |  |  | 0.560 |
| ns | 7248 |  | 475 | parser.y: the complete token list | 5.2 |  | 0.536 |
| walker |  | 7257 | 169 | Code::CodeKey { rung: Decl, file: src/jv_dtoa.c, decl: 70, sub: 0, line: 1630 } |  |  | 0.536 |
| ns | 7435 |  | 187 | parser.y: operator precedence and associativity | 5.3 |  | 0.529 |
| walker |  | 7449 | 192 | Code::CodeKey { rung: Names, file: src/jv_dtoa.c, decl: 0, sub: 5, line: 0 } |  |  | 0.529 |
| walker |  | 7455 | 6 | Code::CodeKey { rung: Decl, file: src/jv_dtoa.c, decl: 74, sub: 0, line: 1993 } |  |  | 0.529 |
| walker |  | 7464 | 9 | Code::CodeKey { rung: Decl, file: src/jv_dtoa.c, decl: 73, sub: 0, line: 1991 } |  |  | 0.529 |
| walker |  | 7483 | 19 | Code::CodeKey { rung: Decl, file: src/jv_dtoa.c, decl: 78, sub: 0, line: 2333 } |  |  | 0.529 |
| walker |  | 7508 | 25 | Code::CodeKey { rung: Decl, file: src/jv_dtoa.c, decl: 75, sub: 0, line: 1995 } |  |  | 0.529 |
| walker |  | 7574 | 66 | Code::CodeKey { rung: Decl, file: src/jv_dtoa.c, decl: 76, sub: 0, line: 2105 } |  |  | 0.529 |
| walker |  | 7779 | 205 | Code::CodeKey { rung: Names, file: src/jv_dtoa.c, decl: 0, sub: 6, line: 0 } |  |  | 0.529 |
| walker |  | 7785 | 6 | Code::CodeKey { rung: Decl, file: src/jv_dtoa.c, decl: 91, sub: 0, line: 3394 } |  |  | 0.529 |
| walker |  | 7791 | 6 | Code::CodeKey { rung: Decl, file: src/jv_dtoa.c, decl: 92, sub: 0, line: 3410 } |  |  | 0.529 |
| walker |  | 7809 | 18 | Code::CodeKey { rung: Decl, file: src/jv_dtoa.c, decl: 87, sub: 0, line: 2344 } |  |  | 0.529 |
| walker |  | 7827 | 18 | Code::CodeKey { rung: Decl, file: src/jv_dtoa.c, decl: 88, sub: 0, line: 2347 } |  |  | 0.529 |
| walker |  | 7846 | 19 | Code::CodeKey { rung: Decl, file: src/jv_dtoa.c, decl: 90, sub: 0, line: 2365 } |  |  | 0.529 |
| walker |  | 7866 | 20 | Code::CodeKey { rung: Decl, file: src/jv_dtoa.c, decl: 89, sub: 0, line: 2350 } |  |  | 0.529 |
| ns | 8106 |  | 671 | builtin.c: function_list, part 1 - libm, binops, conversions, keys, strings, paths, sorting | 5.4 |  | 0.510 |
| walker |  | 8136 | 270 | Code::CodeKey { rung: Names, file: src/jv_dtoa.c, decl: 0, sub: 7, line: 0 } |  |  | 0.510 |
| walker |  | 8142 | 6 | Code::CodeKey { rung: Decl, file: src/jv_dtoa.c, decl: 93, sub: 0, line: 3428 } |  |  | 0.510 |
| walker |  | 8148 | 6 | Code::CodeKey { rung: Decl, file: src/jv_dtoa.c, decl: 97, sub: 0, line: 3514 } |  |  | 0.510 |
| walker |  | 8154 | 6 | Code::CodeKey { rung: Decl, file: src/jv_dtoa.c, decl: 104, sub: 0, line: 4205 } |  |  | 0.510 |
| walker |  | 8165 | 11 | Code::CodeKey { rung: Decl, file: src/jv_dtoa.c, decl: 96, sub: 0, line: 3512 } |  |  | 0.510 |
| walker |  | 8185 | 20 | Code::CodeKey { rung: Decl, file: src/jv_dtoa.c, decl: 103, sub: 0, line: 3525 } |  |  | 0.510 |
| walker |  | 8222 | 37 | Code::CodeKey { rung: Decl, file: src/jv_dtoa.c, decl: 102, sub: 0, line: 3520 } |  |  | 0.510 |
| walker |  | 8500 | 278 | Code::CodeKey { rung: Names, file: src/parser.c, decl: 0, sub: 1, line: 0 } |  |  | 0.510 |
| walker |  | 8508 | 8 | Code::CodeKey { rung: Doc, file: src/parser.c, decl: 16, sub: 0, line: 195 } |  |  | 0.510 |
| ns | 8607 |  | 501 | builtin.c: function_list, part 2 - search, min/max, errors, env, regex, I/O, time | 5.5 |  | 0.498 |
| walker |  | 8767 | 259 | Code::CodeKey { rung: Names, file: src/parser.c, decl: 0, sub: 2, line: 0 } |  |  | 0.498 |
| walker |  | 8976 | 209 | Code::CodeKey { rung: Names, file: src/parser.c, decl: 0, sub: 3, line: 0 } |  |  | 0.498 |
| walker |  | 8981 | 5 | Code::CodeKey { rung: Decl, file: src/parser.c, decl: 73, sub: 0, line: 274 } |  |  | 0.498 |
| walker |  | 8988 | 7 | Code::CodeKey { rung: Decl, file: src/parser.c, decl: 69, sub: 0, line: 260 } |  |  | 0.498 |
| walker |  | 9002 | 14 | Code::CodeKey { rung: Decl, file: src/parser.c, decl: 77, sub: 0, line: 396 } |  |  | 0.498 |
| walker |  | 9023 | 21 | Code::CodeKey { rung: Decl, file: src/parser.c, decl: 70, sub: 0, line: 264 } |  |  | 0.498 |
| walker |  | 9051 | 28 | Code::CodeKey { rung: Decl, file: src/parser.c, decl: 74, sub: 0, line: 280 } |  |  | 0.498 |
| walker |  | 9095 | 44 | Code::CodeKey { rung: Decl, file: src/parser.c, decl: 71, sub: 0, line: 266 } |  |  | 0.498 |
| ns | 9119 |  | 512 | builtin.jq: every jq-defined builtin, part 1 (lines 1-115) | 5.6 |  | 0.473 |
| walker |  | 9180 | 85 | Code::CodeKey { rung: Decl, file: src/parser.c, decl: 66, sub: 0, line: 247 } |  |  | 0.473 |
| walker |  | 9274 | 94 | Code::CodeKey { rung: Decl, file: src/parser.c, decl: 78, sub: 0, line: 399 } |  |  | 0.473 |
| walker |  | 9425 | 151 | Code::CodeKey { rung: Decl, file: src/parser.c, decl: 75, sub: 0, line: 285 } |  |  | 0.473 |
| walker |  | 9433 | 8 | Code::CodeKey { rung: Doc, file: src/parser.c, decl: 75, sub: 0, line: 285 } |  |  | 0.473 |
| ns | 9468 |  | 349 | builtin.jq: every jq-defined builtin, part 2 (lines 116-244) | 5.7 |  | 0.459 |
| walker |  | 9597 | 164 | Code::CodeKey { rung: Decl, file: src/parser.c, decl: 75, sub: 1, line: 285 } |  |  | 0.459 |
| walker |  | 9745 | 148 | Code::CodeKey { rung: Decl, file: src/parser.c, decl: 75, sub: 2, line: 285 } |  |  | 0.459 |
| ns | 9778 |  | 310 | execute.c: struct jq_state, the whole interpreter state | 5.8 |  | 0.449 |
| walker |  | 9898 | 153 | Code::CodeKey { rung: Decl, file: src/parser.c, decl: 75, sub: 3, line: 285 } |  |  | 0.449 |
| ns | 9977 |  | 199 | manual.yml: the section titles of the jq language reference | 6.1 |  | 0.445 |
| walker |  | 9984 | 86 | Code::CodeKey { rung: Decl, file: src/parser.c, decl: 75, sub: 4, line: 285 } |  |  | 0.445 |
