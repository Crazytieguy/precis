Score(3000)=0.534 I=0.810 C=0.353 ns_rows≤3K=20/56 grid(1000/1442/2080/3000/4327/6240/9000)=0.539/0.659/0.635/0.534/0.721/0.694/0.702

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 35 | 35 | Fs::DirListing { dir: . } |  |  | 0.000 |
| ns | 66 |  | 66 | Repository identity: title line and the one-paragraph what-it-is | 1.1 |  | 0.000 |
| walker |  | 101 | 66 | Markdown::ReadmeHeadline { file: README.md } |  |  | 1.000 |
| ns | 101 |  | 35 | Complete root directory listing | 1.2 |  | 1.000 |
| walker |  | 114 | 13 | Fs::DirListing { dir: .github } |  |  | 1.000 |
| walker |  | 122 | 8 | Fs::DirListing { dir: .github/workflows } |  |  | 1.000 |
| ns | 164 |  | 63 | Complete listings of test/, .github/ and .github/workflows/ | 1.3 |  | 0.705 |
| ns | 215 |  | 51 | Makefile: binary name, source list, and the complete .PHONY target list | 1.4 |  | 0.655 |
| ns | 333 |  | 118 | README invocation synopsis: all six supported command forms | 1.5 |  | 0.567 |
| walker |  | 338 | 216 | Code::CodeKey { rung: Names, file: krep.h, decl: 0, sub: 0, line: 0 } |  |  | 0.569 |
| walker |  | 347 | 9 | Code::CodeKey { rung: Doc, file: krep.h, decl: 10, sub: 0, line: 42 } |  |  | 0.569 |
| walker |  | 361 | 14 | Code::CodeKey { rung: Doc, file: krep.h, decl: 1, sub: 0, line: 19 } |  |  | 0.569 |
| walker |  | 375 | 14 | Code::CodeKey { rung: Doc, file: krep.h, decl: 2, sub: 0, line: 22 } |  |  | 0.569 |
| walker |  | 395 | 20 | Code::CodeKey { rung: Doc, file: krep.h, decl: 4, sub: 0, line: 34 } |  |  | 0.569 |
| ns | 463 |  | 130 | Every H2 heading location in README.md | 1.6 |  | 0.472 |
| ns | 559 |  | 96 | krep.h public API roster: the three search entry points, match-result management, printing | 1.7 |  | 0.423 |
| walker |  | 579 | 184 | Code::CodeKey { rung: Names, file: aho_corasick.h, decl: 0, sub: 0, line: 0 } |  |  | 0.426 |
| walker |  | 589 | 10 | Code::CodeKey { rung: Doc, file: aho_corasick.h, decl: 5, sub: 0, line: 23 } |  |  | 0.426 |
| walker |  | 601 | 12 | Code::CodeKey { rung: Doc, file: aho_corasick.h, decl: 9, sub: 0, line: 33 } |  |  | 0.427 |
| walker |  | 613 | 12 | Code::CodeKey { rung: Doc, file: aho_corasick.h, decl: 11, sub: 0, line: 39 } |  |  | 0.428 |
| walker |  | 627 | 14 | Code::CodeKey { rung: Doc, file: aho_corasick.h, decl: 6, sub: 0, line: 26 } |  |  | 0.428 |
| walker |  | 642 | 15 | Code::CodeKey { rung: Doc, file: aho_corasick.h, decl: 8, sub: 0, line: 30 } |  |  | 0.429 |
| walker |  | 684 | 42 | Fs::DirListing { dir: test } |  |  | 0.607 |
| walker |  | 702 | 18 | Code::CodeKey { rung: Doc, file: aho_corasick.h, decl: 1, sub: 0, line: 14 } |  |  | 0.607 |
| walker |  | 722 | 20 | Code::CodeKey { rung: Doc, file: aho_corasick.h, decl: 10, sub: 0, line: 36 } |  |  | 0.610 |
| ns | 789 |  | 230 | krep.h search-algorithm roster with the SIMD #if guards intact | 1.8 |  | 0.501 |
| ns | 996 |  | 207 | aho_corasick.h: the complete second-module interface | 1.9 |  | 0.539 |
| walker |  | 1004 | 282 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.650 |
| walker |  | 1119 | 115 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.652 |
| ns | 1173 |  | 177 | README Key Features, first half | 2.1 |  | 0.624 |
| walker |  | 1251 | 132 | Markdown::Section { file: README.md, section_index: 5, keeps_default_concavity: true } |  |  | 0.687 |
| walker |  | 1272 | 21 | Markdown::Section { file: README.md, section_index: 12, keeps_default_concavity: false } |  |  | 0.687 |
| walker |  | 1301 | 29 | Markdown::Section { file: README.md, section_index: 13, keeps_default_concavity: false } |  |  | 0.687 |
| ns | 1343 |  | 170 | README Key Features, second half | 2.2 |  | 0.659 |
| ns | 1534 |  | 191 | README command-line options table, first half (-i through -F) | 2.3 |  | 0.631 |
| walker |  | 1653 | 352 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: true } |  |  | 0.721 |
| walker |  | 1717 | 64 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.721 |
| ns | 1753 |  | 219 | README command-line options table, second half (-r through -h) | 2.4 |  | 0.685 |
| walker |  | 1761 | 44 | Markdown::Section { file: README.md, section_index: 14, keeps_default_concavity: false } |  |  | 0.685 |
| ns | 1963 |  | 210 | README: the documented smart algorithm-selection policy | 2.5 |  | 0.649 |
| walker |  | 2039 | 278 | Code::CodeKey { rung: Names, file: krep.h, decl: 0, sub: 1, line: 0 } |  |  | 0.651 |
| ns | 2069 |  | 106 | README usage examples: the six concrete invocations, command lines only | 2.6 |  | 0.635 |
| walker |  | 2071 | 32 | Code::CodeKey { rung: Decl, file: krep.h, decl: 17, sub: 0, line: 98 } |  |  | 0.635 |
| walker |  | 2111 | 40 | Code::CodeKey { rung: Decl, file: krep.h, decl: 14, sub: 0, line: 49 } |  |  | 0.636 |
| walker |  | 2162 | 51 | Code::CodeKey { rung: Decl, file: krep.h, decl: 19, sub: 0, line: 125 } |  |  | 0.636 |
| ns | 2181 |  | 112 | README: the multi-threading architecture section | 2.7 |  | 0.612 |
| walker |  | 2224 | 62 | Code::CodeKey { rung: Decl, file: krep.h, decl: 15, sub: 0, line: 55 } |  |  | 0.614 |
| ns | 2287 |  | 106 | README: the recursive-search skipping rules | 2.8 |  | 0.596 |
| walker |  | 2385 | 161 | Code::CodeKey { rung: Decl, file: krep.h, decl: 20, sub: 0, line: 132 } |  |  | 0.596 |
| ns | 2402 |  | 115 | README scope note: explicitly not a grep/ripgrep replacement | 2.9 |  | 0.605 |
| ns | 2512 |  | 110 | README install and build-from-source commands | 2.10 |  | 0.582 |
| walker |  | 2635 | 250 | Code::CodeKey { rung: Decl, file: krep.h, decl: 18, sub: 0, line: 104 } |  |  | 0.586 |
| ns | 2902 |  | 390 | search_params_t: the struct every search function takes | 3.1 |  | 0.534 |
| walker |  | 3003 | 368 | Code::CodeKey { rung: Decl, file: krep.h, decl: 16, sub: 0, line: 65 } |  |  | 0.647 |
| walker |  | 3012 | 9 | Code::CodeKey { rung: Doc, file: krep.h, decl: 19, sub: 0, line: 125 } |  |  | 0.647 |
| ns | 3089 |  | 187 | match_position_t and match_result_t | 3.2 |  | 0.653 |
| ns | 3180 |  | 91 | search_func_t: the algorithm function-pointer type | 3.3 |  | 0.649 |
| ns | 3282 |  | 102 | krep.h thread-pool API: all four functions in full | 3.4 |  | 0.644 |
| walker |  | 3427 | 415 | Markdown::Section { file: README.md, section_index: 7, keeps_default_concavity: true } |  |  | 0.703 |
| walker |  | 3549 | 122 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.703 |
| walker |  | 3558 | 9 | Code::CodeKey { rung: Doc, file: krep.h, decl: 21, sub: 0, line: 146 } |  |  | 0.706 |
| ns | 3562 |  | 280 | thread_data_t: per-thread search state, including the false-sharing padding | 3.5 |  | 0.716 |
| ns | 3659 |  | 97 | krep.h helper roster: line finding, word matching, table preparation | 3.6 |  | 0.703 |
| walker |  | 3784 | 226 | Code::CodeKey { rung: Names, file: krep.h, decl: 0, sub: 2, line: 0 } |  |  | 0.740 |
| walker |  | 3819 | 35 | Code::CodeKey { rung: Doc, file: aho_corasick.h, decl: 3, sub: 0, line: 19 } |  |  | 0.740 |
| ns | 3826 |  | 167 | skip_directories: the complete recursive-search directory blocklist | 3.7 |  | 0.725 |
| ns | 4011 |  | 185 | skip_extensions: head, ellipsis, and the count line | 3.8 |  | 0.713 |
| walker |  | 4027 | 208 | Code::CodeKey { rung: Names, file: krep.c, decl: 0, sub: 0, line: 0 } |  |  | 0.713 |
| walker |  | 4034 | 7 | Code::CodeKey { rung: Doc, file: krep.c, decl: 4, sub: 0, line: 77 } |  |  | 0.713 |
| walker |  | 4045 | 11 | Code::CodeKey { rung: Doc, file: krep.c, decl: 2, sub: 0, line: 42 } |  |  | 0.713 |
| walker |  | 4059 | 14 | Code::CodeKey { rung: Doc, file: krep.c, decl: 3, sub: 0, line: 44 } |  |  | 0.713 |
| walker |  | 4075 | 16 | Code::CodeKey { rung: Doc, file: krep.c, decl: 1, sub: 0, line: 39 } |  |  | 0.713 |
| ns | 4236 |  | 225 | ANSI colour macro block, output palette and help palette | 3.9 |  | 0.720 |
| walker |  | 4321 | 246 | Code::CodeKey { rung: Names, file: aho_corasick.c, decl: 0, sub: 0, line: 0 } |  |  | 0.721 |
| walker |  | 4326 | 5 | Code::CodeKey { rung: Decl, file: aho_corasick.c, decl: 6, sub: 0, line: 111 } |  |  | 0.721 |
| walker |  | 4331 | 5 | Code::CodeKey { rung: Decl, file: aho_corasick.c, decl: 7, sub: 0, line: 274 } |  |  | 0.721 |
| walker |  | 4336 | 5 | Code::CodeKey { rung: Decl, file: aho_corasick.c, decl: 8, sub: 0, line: 287 } |  |  | 0.721 |
| walker |  | 4341 | 5 | Code::CodeKey { rung: Decl, file: aho_corasick.c, decl: 3, sub: 0, line: 34 } |  |  | 0.721 |
| walker |  | 4346 | 5 | Code::CodeKey { rung: Decl, file: aho_corasick.c, decl: 4, sub: 0, line: 55 } |  |  | 0.721 |
| walker |  | 4351 | 5 | Code::CodeKey { rung: Decl, file: aho_corasick.c, decl: 5, sub: 0, line: 86 } |  |  | 0.721 |
| walker |  | 4388 | 37 | Code::CodeKey { rung: Decl, file: aho_corasick.c, decl: 12, sub: 0, line: 299 } |  |  | 0.721 |
| ns | 4395 |  | 159 | Documented contracts of the three entry points and the printer | 3.10 | 1.7 | 0.708 |
| walker |  | 4451 | 63 | Code::CodeKey { rung: Decl, file: aho_corasick.c, decl: 2, sub: 0, line: 26 } |  |  | 0.709 |
| walker |  | 4553 | 102 | Code::CodeKey { rung: Decl, file: aho_corasick.c, decl: 1, sub: 0, line: 17 } |  |  | 0.710 |
| ns | 4634 |  | 239 | krep.c file-level section banner map, all sixteen top-level banners | 4.1 |  | 0.688 |
| walker |  | 4750 | 197 | Code::CodeKey { rung: Names, file: krep.h, decl: 0, sub: 3, line: 0 } |  |  | 0.698 |
| walker |  | 4757 | 7 | Code::CodeKey { rung: Doc, file: krep.h, decl: 33, sub: 0, line: 203 } |  |  | 0.698 |
| ns | 4758 |  | 124 | krep.c definition roster, lines 139-461: match results, line finding, printing | 4.2 |  | 0.685 |
| ns | 4932 |  | 174 | krep.c definition roster, lines 1084-1964: utilities, the four scalar algorithms, orchestration | 4.3 |  | 0.670 |
| walker |  | 4986 | 229 | Code::CodeKey { rung: Names, file: krep.c, decl: 0, sub: 1, line: 0 } |  |  | 0.670 |
| walker |  | 4995 | 9 | Code::CodeKey { rung: Doc, file: krep.c, decl: 15, sub: 0, line: 93 } |  |  | 0.670 |
| walker |  | 5005 | 10 | Code::CodeKey { rung: Doc, file: krep.h, decl: 18, sub: 0, line: 104 } |  |  | 0.674 |
| walker |  | 5021 | 16 | Code::CodeKey { rung: Doc, file: krep.c, decl: 20, sub: 0, line: 116 } |  |  | 0.674 |
| walker |  | 5031 | 10 | Code::CodeKey { rung: Doc, file: krep.h, decl: 28, sub: 0, line: 183 } |  |  | 0.674 |
| ns | 5124 |  | 192 | krep.c definition roster, lines 1999-3442: public API bodies, skip filters, gitignore | 4.4 |  | 0.658 |
| walker |  | 5213 | 182 | Code::CodeKey { rung: Names, file: krep.h, decl: 0, sub: 4, line: 0 } |  |  | 0.668 |
| walker |  | 5232 | 19 | Code::CodeKey { rung: Decl, file: krep.h, decl: 40, sub: 0, line: 218 } |  |  | 0.672 |
| walker |  | 5251 | 19 | Code::CodeKey { rung: Decl, file: krep.h, decl: 39, sub: 0, line: 214 } |  |  | 0.678 |
| walker |  | 5270 | 19 | Code::CodeKey { rung: Decl, file: krep.h, decl: 42, sub: 0, line: 226 } |  |  | 0.684 |
| walker |  | 5294 | 24 | Code::CodeKey { rung: Decl, file: krep.h, decl: 41, sub: 0, line: 222 } |  |  | 0.694 |
| ns | 5363 |  | 239 | krep.c definition roster, lines 3451-5108: main, thread pool, and the guarded SIMD kernels | 4.5 |  | 0.675 |
| walker |  | 5514 | 220 | Code::CodeKey { rung: Names, file: krep.c, decl: 0, sub: 2, line: 0 } |  |  | 0.679 |
| walker |  | 5519 | 5 | Code::CodeKey { rung: Decl, file: krep.c, decl: 29, sub: 0, line: 139 } |  |  | 0.679 |
| walker |  | 5524 | 5 | Code::CodeKey { rung: Decl, file: krep.c, decl: 30, sub: 0, line: 175 } |  |  | 0.679 |
| walker |  | 5529 | 5 | Code::CodeKey { rung: Decl, file: krep.c, decl: 31, sub: 0, line: 244 } |  |  | 0.679 |
| walker |  | 5534 | 5 | Code::CodeKey { rung: Decl, file: krep.c, decl: 32, sub: 0, line: 256 } |  |  | 0.679 |
| walker |  | 5539 | 5 | Code::CodeKey { rung: Decl, file: krep.c, decl: 28, sub: 0, line: 128 } |  |  | 0.679 |
| ns | 5546 |  | 183 | search_file internal section map, all eleven banners inside lines 2274-3070 | 4.6 |  | 0.667 |
| walker |  | 5549 | 10 | Code::CodeKey { rung: Doc, file: krep.c, decl: 29, sub: 0, line: 139 } |  |  | 0.667 |
| walker |  | 5562 | 13 | Code::CodeKey { rung: Doc, file: aho_corasick.c, decl: 12, sub: 0, line: 299 } |  |  | 0.667 |
| walker |  | 5574 | 12 | Code::CodeKey { rung: Doc, file: krep.c, decl: 27, sub: 0, line: 125 } |  |  | 0.668 |
| walker |  | 5587 | 13 | Code::CodeKey { rung: Doc, file: krep.c, decl: 31, sub: 0, line: 244 } |  |  | 0.668 |
| ns | 5668 |  | 122 | print_matching_items internal section map: the -o and full-line output modes | 4.7 |  | 0.660 |
| walker |  | 5799 | 212 | Code::CodeKey { rung: Names, file: krep.c, decl: 0, sub: 3, line: 0 } |  |  | 0.676 |
| walker |  | 5804 | 5 | Code::CodeKey { rung: Decl, file: krep.c, decl: 34, sub: 0, line: 363 } |  |  | 0.676 |
| walker |  | 5809 | 5 | Code::CodeKey { rung: Decl, file: krep.c, decl: 35, sub: 0, line: 401 } |  |  | 0.676 |
| walker |  | 5814 | 5 | Code::CodeKey { rung: Decl, file: krep.c, decl: 38, sub: 0, line: 461 } |  |  | 0.676 |
| walker |  | 5819 | 5 | Code::CodeKey { rung: Decl, file: krep.c, decl: 36, sub: 0, line: 420 } |  |  | 0.676 |
| ns | 5822 |  | 154 | aho_corasick.c: complete definition roster | 4.8 |  | 0.681 |
| walker |  | 5824 | 5 | Code::CodeKey { rung: Decl, file: krep.c, decl: 37, sub: 0, line: 438 } |  |  | 0.681 |
| walker |  | 5830 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 39, sub: 0, line: 1084 } |  |  | 0.681 |
| walker |  | 5867 | 37 | Code::CodeKey { rung: Decl, file: krep.c, decl: 33, sub: 0, line: 329 } |  |  | 0.681 |
| walker |  | 5875 | 8 | Code::CodeKey { rung: Doc, file: krep.h, decl: 43, sub: 0, line: 231 } |  |  | 0.681 |
| walker |  | 5889 | 14 | Code::CodeKey { rung: Doc, file: aho_corasick.c, decl: 6, sub: 0, line: 111 } |  |  | 0.681 |
| ns | 6014 |  | 192 | aho_corasick.c trie data model: ac_node_t and struct ac_trie | 4.9 |  | 0.680 |
| walker |  | 6048 | 159 | Code::CodeKey { rung: Names, file: krep.h, decl: 0, sub: 5, line: 0 } |  |  | 0.694 |
| walker |  | 6053 | 5 | Code::CodeKey { rung: Decl, file: krep.h, decl: 49, sub: 0, line: 298 } |  |  | 0.694 |
| walker |  | 6058 | 5 | Code::CodeKey { rung: Decl, file: krep.h, decl: 50, sub: 0, line: 312 } |  |  | 0.694 |
| walker |  | 6243 | 185 | Markdown::Section { file: README.md, section_index: 9, keeps_default_concavity: false } |  |  | 0.710 |
| ns | 6272 |  | 258 | test/test_krep.c: complete roster of test functions, helpers and main | 4.10 |  | 0.691 |
| walker |  | 6441 | 198 | Markdown::Section { file: README.md, section_index: 10, keeps_default_concavity: true } |  |  | 0.714 |
| ns | 6497 |  | 225 | test/test_regex.c and test/test_multiple_patterns.c rosters | 4.11 |  | 0.699 |
| walker |  | 6570 | 129 | Markdown::Section { file: README.md, section_index: 11, keeps_default_concavity: false } |  |  | 0.712 |
| walker |  | 6584 | 14 | Code::CodeKey { rung: Doc, file: aho_corasick.c, decl: 7, sub: 0, line: 274 } |  |  | 0.712 |
| ns | 6608 |  | 111 | test/test_directory.c roster: the separate recursive-search integration binary | 4.12 |  | 0.704 |
| ns | 6841 |  | 233 | test/test_compat.h: the TESTING wrapper roster and its guards | 4.13 |  | 0.692 |
| walker |  | 6876 | 292 | Markdown::Section { file: README.md, section_index: 4, keeps_default_concavity: false } |  |  | 0.711 |
| walker |  | 6890 | 14 | Code::CodeKey { rung: Doc, file: aho_corasick.c, decl: 11, sub: 0, line: 296 } |  |  | 0.711 |
| ns | 6972 |  | 131 | test/test_krep.h: shared test helper declarations | 4.14 |  | 0.701 |
| ns | 7080 |  | 108 | Makefile compiler configuration: CC, CFLAGS, LDFLAGS, PREFIX | 5.1 |  | 0.695 |
| walker |  | 7187 | 297 | Markdown::Section { file: README.md, section_index: 6, keeps_default_concavity: false } |  |  | 0.706 |
| ns | 7316 |  | 236 | .github/workflows/ci.yml in full | 5.2 |  | 0.687 |
| walker |  | 7427 | 240 | Code::CodeKey { rung: Names, file: krep.c, decl: 0, sub: 4, line: 0 } |  |  | 0.700 |
| walker |  | 7433 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 40, sub: 0, line: 1125 } |  |  | 0.700 |
| walker |  | 7439 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 41, sub: 0, line: 1137 } |  |  | 0.700 |
| walker |  | 7445 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 42, sub: 0, line: 1198 } |  |  | 0.700 |
| walker |  | 7451 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 43, sub: 0, line: 1213 } |  |  | 0.700 |
| walker |  | 7457 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 48, sub: 0, line: 1771 } |  |  | 0.700 |
| walker |  | 7463 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 46, sub: 0, line: 1585 } |  |  | 0.700 |
| walker |  | 7469 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 49, sub: 0, line: 1873 } |  |  | 0.700 |
| walker |  | 7510 | 41 | Code::CodeKey { rung: Decl, file: krep.c, decl: 45, sub: 0, line: 1389 } |  |  | 0.700 |
| walker |  | 7560 | 50 | Code::CodeKey { rung: Decl, file: krep.c, decl: 44, sub: 0, line: 1259 } |  |  | 0.700 |
| ns | 7580 |  | 264 | Makefile architecture detection: which SIMD flags each arch gets | 5.3 |  | 0.687 |
| walker |  | 7610 | 50 | Code::CodeKey { rung: Decl, file: krep.c, decl: 47, sub: 0, line: 1628 } |  |  | 0.687 |
| ns | 7883 |  | 303 | Makefile compile/link rules and the parallel -DTESTING build | 5.4 |  | 0.676 |
| walker |  | 7900 | 290 | Markdown::Section { file: README.md, section_index: 8, keeps_default_concavity: false } |  |  | 0.676 |
| walker |  | 7913 | 13 | Code::CodeKey { rung: Doc, file: krep.h, decl: 16, sub: 0, line: 65 } |  |  | 0.676 |
| walker |  | 7934 | 21 | Code::CodeKey { rung: Doc, file: aho_corasick.c, decl: 9, sub: 0, line: 293 } |  |  | 0.676 |
| walker |  | 7956 | 22 | Code::CodeKey { rung: Doc, file: aho_corasick.c, decl: 8, sub: 0, line: 287 } |  |  | 0.676 |
| ns | 8052 |  | 169 | Makefile run targets: test, test-directory, ci, bench-rg, all-tests | 5.5 |  | 0.665 |
| ns | 8209 |  | 157 | .github/workflows/release.yml: tag trigger and release artifacts | 5.6 |  | 0.653 |
| walker |  | 8237 | 281 | Code::CodeKey { rung: Names, file: krep.c, decl: 0, sub: 5, line: 0 } |  |  | 0.665 |
| walker |  | 8243 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 50, sub: 0, line: 1919 } |  |  | 0.665 |
| walker |  | 8249 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 51, sub: 0, line: 1964 } |  |  | 0.665 |
| walker |  | 8255 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 52, sub: 0, line: 1999 } |  |  | 0.665 |
| walker |  | 8261 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 56, sub: 0, line: 2274 } |  |  | 0.665 |
| walker |  | 8267 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 54, sub: 0, line: 2252 } |  |  | 0.665 |
| walker |  | 8273 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 55, sub: 0, line: 2265 } |  |  | 0.665 |
| walker |  | 8279 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 57, sub: 0, line: 3071 } |  |  | 0.665 |
| walker |  | 8285 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 58, sub: 0, line: 3090 } |  |  | 0.665 |
| walker |  | 8291 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 59, sub: 0, line: 3122 } |  |  | 0.665 |
| walker |  | 8297 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 62, sub: 0, line: 3163 } |  |  | 0.665 |
| walker |  | 8303 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 63, sub: 0, line: 3180 } |  |  | 0.665 |
| walker |  | 8337 | 34 | Code::CodeKey { rung: Decl, file: krep.c, decl: 60, sub: 0, line: 3146 } |  |  | 0.666 |
| walker |  | 8394 | 57 | Code::CodeKey { rung: Decl, file: krep.c, decl: 61, sub: 0, line: 3154 } |  |  | 0.666 |
| walker |  | 8413 | 19 | Code::CodeKey { rung: Body, file: krep.h, decl: 49, sub: 0, line: 298 } |  |  | 0.666 |
| ns | 8462 |  | 253 | krep.c constants: VERSION and every performance tunable | 6.1 |  | 0.669 |
| walker |  | 8649 | 236 | Plaintext::Whole { file: .github/workflows/ci.yml } |  |  | 0.696 |
| ns | 8696 |  | 234 | krep.c global option state and the lower_table constructor | 6.2 |  | 0.695 |
| walker |  | 8952 | 303 | Code::CodeKey { rung: Names, file: krep.c, decl: 0, sub: 6, line: 0 } |  |  | 0.710 |
| walker |  | 8958 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 68, sub: 0, line: 3442 } |  |  | 0.710 |
| walker |  | 8964 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 71, sub: 0, line: 4104 } |  |  | 0.710 |
| ns | 8966 |  | 270 | main: the getopt_long table and short-option string | 6.3 |  | 0.702 |
| walker |  | 8970 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 72, sub: 0, line: 4209 } |  |  | 0.702 |
| walker |  | 8976 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 74, sub: 0, line: 4313 } |  |  | 0.702 |
| walker |  | 8982 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 75, sub: 0, line: 4332 } |  |  | 0.702 |
| walker |  | 8988 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 64, sub: 0, line: 3240 } |  |  | 0.702 |
| walker |  | 8994 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 65, sub: 0, line: 3272 } |  |  | 0.702 |
| walker |  | 9000 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 66, sub: 0, line: 3297 } |  |  | 0.702 |
| walker |  | 9006 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 70, sub: 0, line: 4046 } |  |  | 0.702 |
| walker |  | 9012 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 73, sub: 0, line: 4251 } |  |  | 0.702 |
| walker |  | 9053 | 41 | Code::CodeKey { rung: Decl, file: krep.c, decl: 69, sub: 0, line: 3891 } |  |  | 0.702 |
| walker |  | 9094 | 41 | Code::CodeKey { rung: Decl, file: krep.c, decl: 76, sub: 0, line: 4371 } |  |  | 0.702 |
| walker |  | 9118 | 24 | Code::CodeKey { rung: Decl, file: krep.c, decl: 67, sub: 0, line: 3310 } |  |  | 0.702 |
| walker |  | 9141 | 23 | Code::CodeKey { rung: Doc, file: krep.h, decl: 14, sub: 0, line: 49 } |  |  | 0.707 |
| walker |  | 9169 | 28 | Code::CodeKey { rung: Body, file: aho_corasick.c, decl: 8, sub: 0, line: 287 } |  |  | 0.707 |
| walker |  | 9178 | 9 | Code::CodeKey { rung: Doc, file: aho_corasick.c, decl: 3, sub: 0, line: 34 } |  |  | 0.707 |
| walker |  | 9215 | 37 | Code::CodeKey { rung: Doc, file: krep.h, decl: 17, sub: 0, line: 98 } |  |  | 0.711 |
| walker |  | 9227 | 12 | Code::CodeKey { rung: Doc, file: aho_corasick.c, decl: 4, sub: 0, line: 55 } |  |  | 0.711 |
| ns | 9299 |  | 333 | select_search_algorithm: dispatch head through the short-pattern branch | 6.4 |  | 0.696 |
| ns | 9685 |  | 386 | select_search_algorithm: SIMD length limits and the KMP/Boyer-Moore fallback | 6.5 | 6.4 | 0.679 |
| walker |  | 9824 | 597 | Plaintext::Whole { file: .github/workflows/release.yml } |  |  | 0.698 |
| walker |  | 9839 | 15 | Code::CodeKey { rung: Doc, file: aho_corasick.c, decl: 5, sub: 0, line: 86 } |  |  | 0.698 |
| ns | 9868 |  | 183 | gitignore data model: pattern record and parent-chained context | 6.6 |  | 0.696 |
| walker |  | 9874 | 35 | Plaintext::Whole { file: .gitignore } |  |  | 0.696 |
| ns | 9993 |  | 125 | Licence header, dependabot config and .gitignore | 7.1 |  | 0.690 |
