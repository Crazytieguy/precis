Score(3000)=0.698 I=0.849 C=0.573 ns_rows≤3K=20/56 grid(1000/1442/2080/3000/4327/6240/9000)=0.651/0.698/0.679/0.698/0.799/0.722/0.660

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 35 | 35 | Fs::DirListing { dir: . } |  |  | 0.000 |
| ns | 66 |  | 66 | Repository identity: title line and the one-paragraph what-it-is | 1.1 |  | 0.000 |
| walker |  | 101 | 66 | Markdown::ReadmeHeadline { file: README.md } |  |  | 1.000 |
| ns | 101 |  | 35 | Complete root directory listing | 1.2 |  | 1.000 |
| walker |  | 125 | 24 | Plaintext::Whole { file: Makefile } |  |  | 1.000 |
| walker |  | 138 | 13 | Fs::DirListing { dir: .github } |  |  | 1.000 |
| walker |  | 146 | 8 | Fs::DirListing { dir: .github/workflows } |  |  | 1.000 |
| ns | 164 |  | 63 | Complete listings of test/, .github/ and .github/workflows/ | 1.3 |  | 0.708 |
| walker |  | 188 | 42 | Fs::DirListing { dir: test } |  |  | 1.000 |
| ns | 215 |  | 51 | Makefile: binary name, source list, and the complete .PHONY target list | 1.4 |  | 0.943 |
| ns | 333 |  | 118 | README invocation synopsis: all six supported command forms | 1.5 |  | 0.817 |
| ns | 463 |  | 130 | Every H2 heading location in README.md | 1.6 |  | 0.679 |
| walker |  | 470 | 282 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.860 |
| ns | 559 |  | 96 | krep.h public API roster: the three search entry points, match-result management, printing | 1.7 |  | 0.771 |
| walker |  | 572 | 102 | Markdown::CommandBlock { file: README.md, row: 54 } |  |  | 0.774 |
| walker |  | 687 | 115 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.776 |
| ns | 789 |  | 230 | krep.h search-algorithm roster with the SIMD #if guards intact | 1.8 |  | 0.638 |
| walker |  | 819 | 132 | Markdown::Section { file: README.md, section_index: 5, keeps_default_concavity: true } |  |  | 0.716 |
| ns | 996 |  | 207 | aho_corasick.h: the complete second-module interface | 1.9 |  | 0.647 |
| walker |  | 1171 | 352 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: true } |  |  | 0.657 |
| ns | 1173 |  | 177 | README Key Features, first half | 2.1 |  | 0.677 |
| walker |  | 1235 | 64 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.677 |
| ns | 1343 |  | 170 | README Key Features, second half | 2.2 |  | 0.693 |
| ns | 1534 |  | 191 | README command-line options table, first half (-i through -F) | 2.3 |  | 0.663 |
| walker |  | 1650 | 415 | Markdown::Section { file: README.md, section_index: 7, keeps_default_concavity: true } |  |  | 0.720 |
| ns | 1753 |  | 219 | README command-line options table, second half (-r through -h) | 2.4 |  | 0.734 |
| walker |  | 1772 | 122 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.734 |
| walker |  | 1963 | 191 | Code::CodeKey { rung: Names, file: krep.h, decl: 0, sub: 0, line: 0 } |  |  | 0.696 |
| ns | 1963 |  | 210 | README: the documented smart algorithm-selection policy | 2.5 |  | 0.696 |
| walker |  | 1977 | 14 | Code::CodeKey { rung: Doc, file: krep.h, decl: 1, sub: 0, line: 19 } |  |  | 0.696 |
| walker |  | 1991 | 14 | Code::CodeKey { rung: Doc, file: krep.h, decl: 2, sub: 0, line: 22 } |  |  | 0.696 |
| walker |  | 2011 | 20 | Code::CodeKey { rung: Doc, file: krep.h, decl: 4, sub: 0, line: 34 } |  |  | 0.696 |
| ns | 2069 |  | 106 | README usage examples: the six concrete invocations, command lines only | 2.6 |  | 0.678 |
| ns | 2181 |  | 112 | README: the multi-threading architecture section | 2.7 |  | 0.652 |
| walker |  | 2208 | 197 | Code::CodeKey { rung: Names, file: krep.h, decl: 0, sub: 1, line: 0 } |  |  | 0.654 |
| walker |  | 2240 | 32 | Code::CodeKey { rung: Decl, file: krep.h, decl: 17, sub: 0, line: 98 } |  |  | 0.654 |
| walker |  | 2280 | 40 | Code::CodeKey { rung: Decl, file: krep.h, decl: 14, sub: 0, line: 49 } |  |  | 0.655 |
| ns | 2287 |  | 106 | README: the recursive-search skipping rules | 2.8 |  | 0.636 |
| walker |  | 2342 | 62 | Code::CodeKey { rung: Decl, file: krep.h, decl: 15, sub: 0, line: 55 } |  |  | 0.638 |
| walker |  | 2351 | 9 | Code::CodeKey { rung: Doc, file: krep.h, decl: 10, sub: 0, line: 42 } |  |  | 0.638 |
| ns | 2402 |  | 115 | README scope note: explicitly not a grep/ripgrep replacement | 2.9 |  | 0.645 |
| ns | 2512 |  | 110 | README install and build-from-source commands | 2.10 |  | 0.642 |
| walker |  | 2719 | 368 | Code::CodeKey { rung: Decl, file: krep.h, decl: 16, sub: 0, line: 65 } |  |  | 0.652 |
| walker |  | 2732 | 13 | Code::CodeKey { rung: Doc, file: krep.h, decl: 16, sub: 0, line: 65 } |  |  | 0.652 |
| walker |  | 2755 | 23 | Code::CodeKey { rung: Doc, file: krep.h, decl: 14, sub: 0, line: 49 } |  |  | 0.654 |
| ns | 2902 |  | 390 | search_params_t: the struct every search function takes | 3.1 |  | 0.692 |
| walker |  | 2964 | 209 | Code::CodeKey { rung: Names, file: krep.h, decl: 0, sub: 2, line: 0 } |  |  | 0.698 |
| walker |  | 3015 | 51 | Code::CodeKey { rung: Decl, file: krep.h, decl: 19, sub: 0, line: 125 } |  |  | 0.698 |
| ns | 3089 |  | 187 | match_position_t and match_result_t | 3.2 |  | 0.710 |
| walker |  | 3176 | 161 | Code::CodeKey { rung: Decl, file: krep.h, decl: 20, sub: 0, line: 132 } |  |  | 0.710 |
| ns | 3180 |  | 91 | search_func_t: the algorithm function-pointer type | 3.3 |  | 0.705 |
| walker |  | 3185 | 9 | Code::CodeKey { rung: Doc, file: krep.h, decl: 19, sub: 0, line: 125 } |  |  | 0.705 |
| walker |  | 3194 | 9 | Code::CodeKey { rung: Doc, file: krep.h, decl: 21, sub: 0, line: 146 } |  |  | 0.706 |
| ns | 3282 |  | 102 | krep.h thread-pool API: all four functions in full | 3.4 |  | 0.711 |
| walker |  | 3444 | 250 | Code::CodeKey { rung: Decl, file: krep.h, decl: 18, sub: 0, line: 104 } |  |  | 0.716 |
| walker |  | 3454 | 10 | Code::CodeKey { rung: Doc, file: krep.h, decl: 18, sub: 0, line: 104 } |  |  | 0.717 |
| ns | 3562 |  | 280 | thread_data_t: per-thread search state, including the false-sharing padding | 3.5 |  | 0.730 |
| walker |  | 3631 | 177 | Code::CodeKey { rung: Names, file: krep.h, decl: 0, sub: 3, line: 0 } |  |  | 0.760 |
| walker |  | 3638 | 7 | Code::CodeKey { rung: Doc, file: krep.h, decl: 33, sub: 0, line: 203 } |  |  | 0.760 |
| walker |  | 3648 | 10 | Code::CodeKey { rung: Doc, file: krep.h, decl: 28, sub: 0, line: 183 } |  |  | 0.760 |
| ns | 3659 |  | 97 | krep.h helper roster: line finding, word matching, table preparation | 3.6 |  | 0.746 |
| walker |  | 3791 | 143 | Code::CodeKey { rung: Names, file: krep.h, decl: 0, sub: 4, line: 0 } |  |  | 0.751 |
| ns | 3826 |  | 167 | skip_directories: the complete recursive-search directory blocklist | 3.7 |  | 0.736 |
| walker |  | 3943 | 152 | Code::CodeKey { rung: Names, file: krep.h, decl: 0, sub: 5, line: 0 } |  |  | 0.747 |
| walker |  | 3962 | 19 | Code::CodeKey { rung: Decl, file: krep.h, decl: 40, sub: 0, line: 218 } |  |  | 0.752 |
| walker |  | 3981 | 19 | Code::CodeKey { rung: Decl, file: krep.h, decl: 39, sub: 0, line: 214 } |  |  | 0.759 |
| walker |  | 4000 | 19 | Code::CodeKey { rung: Decl, file: krep.h, decl: 42, sub: 0, line: 226 } |  |  | 0.766 |
| ns | 4011 |  | 185 | skip_extensions: head, ellipsis, and the count line | 3.8 |  | 0.753 |
| walker |  | 4024 | 24 | Code::CodeKey { rung: Decl, file: krep.h, decl: 41, sub: 0, line: 222 } |  |  | 0.765 |
| walker |  | 4213 | 189 | Code::CodeKey { rung: Names, file: krep.h, decl: 0, sub: 6, line: 0 } |  |  | 0.783 |
| walker |  | 4218 | 5 | Code::CodeKey { rung: Decl, file: krep.h, decl: 49, sub: 0, line: 298 } |  |  | 0.783 |
| walker |  | 4223 | 5 | Code::CodeKey { rung: Decl, file: krep.h, decl: 50, sub: 0, line: 312 } |  |  | 0.783 |
| walker |  | 4231 | 8 | Code::CodeKey { rung: Doc, file: krep.h, decl: 43, sub: 0, line: 231 } |  |  | 0.783 |
| ns | 4236 |  | 225 | ANSI colour macro block, output palette and help palette | 3.9 |  | 0.787 |
| walker |  | 4268 | 37 | Code::CodeKey { rung: Doc, file: krep.h, decl: 17, sub: 0, line: 98 } |  |  | 0.796 |
| ns | 4395 |  | 159 | Documented contracts of the three entry points and the printer | 3.10 | 1.7 | 0.781 |
| walker |  | 4458 | 190 | Markdown::Section { file: README.md, section_index: 4, keeps_default_concavity: false } |  |  | 0.794 |
| walker |  | 4526 | 68 | Code::CodeKey { rung: Doc, file: krep.h, decl: 49, sub: 0, line: 298 } |  |  | 0.794 |
| ns | 4634 |  | 239 | krep.c file-level section banner map, all sixteen top-level banners | 4.1 |  | 0.770 |
| walker |  | 4711 | 185 | Markdown::Section { file: README.md, section_index: 9, keeps_default_concavity: false } |  |  | 0.790 |
| ns | 4758 |  | 124 | krep.c definition roster, lines 139-461: match results, line finding, printing | 4.2 |  | 0.776 |
| walker |  | 4909 | 198 | Markdown::Section { file: README.md, section_index: 10, keeps_default_concavity: false } |  |  | 0.804 |
| ns | 4932 |  | 174 | krep.c definition roster, lines 1084-1964: utilities, the four scalar algorithms, orchestration | 4.3 |  | 0.785 |
| walker |  | 5038 | 129 | Markdown::Section { file: README.md, section_index: 11, keeps_default_concavity: false } |  |  | 0.802 |
| ns | 5124 |  | 192 | krep.c definition roster, lines 1999-3442: public API bodies, skip filters, gitignore | 4.4 |  | 0.783 |
| walker |  | 5128 | 90 | Code::CodeKey { rung: Doc, file: krep.h, decl: 26, sub: 0, line: 170 } |  |  | 0.784 |
| walker |  | 5316 | 188 | Code::CodeKey { rung: Names, file: krep.c, decl: 0, sub: 0, line: 0 } |  |  | 0.784 |
| walker |  | 5333 | 17 | Code::CodeKey { rung: Decl, file: krep.c, decl: 1, sub: 0, line: 9 } |  |  | 0.784 |
| walker |  | 5340 | 7 | Code::CodeKey { rung: Doc, file: krep.c, decl: 5, sub: 0, line: 77 } |  |  | 0.784 |
| walker |  | 5351 | 11 | Code::CodeKey { rung: Doc, file: krep.c, decl: 3, sub: 0, line: 42 } |  |  | 0.784 |
| ns | 5363 |  | 239 | krep.c definition roster, lines 3451-5108: main, thread pool, and the guarded SIMD kernels | 4.5 |  | 0.763 |
| walker |  | 5365 | 14 | Code::CodeKey { rung: Doc, file: krep.c, decl: 4, sub: 0, line: 44 } |  |  | 0.763 |
| walker |  | 5381 | 16 | Code::CodeKey { rung: Doc, file: krep.c, decl: 2, sub: 0, line: 39 } |  |  | 0.763 |
| ns | 5546 |  | 183 | search_file internal section map, all eleven banners inside lines 2274-3070 | 4.6 |  | 0.750 |
| walker |  | 5576 | 195 | Code::CodeKey { rung: Names, file: krep.c, decl: 0, sub: 1, line: 0 } |  |  | 0.751 |
| walker |  | 5588 | 12 | Code::CodeKey { rung: Decl, file: krep.c, decl: 12, sub: 0, line: 84 } |  |  | 0.751 |
| walker |  | 5597 | 9 | Code::CodeKey { rung: Doc, file: krep.c, decl: 17, sub: 0, line: 93 } |  |  | 0.751 |
| ns | 5668 |  | 122 | print_matching_items internal section map: the -o and full-line output modes | 4.7 |  | 0.742 |
| walker |  | 5794 | 197 | Code::CodeKey { rung: Names, file: krep.c, decl: 0, sub: 2, line: 0 } |  |  | 0.742 |
| walker |  | 5804 | 10 | Code::CodeKey { rung: Decl, file: krep.c, decl: 25, sub: 0, line: 109 } |  |  | 0.742 |
| walker |  | 5815 | 11 | Code::CodeKey { rung: Decl, file: krep.c, decl: 24, sub: 0, line: 107 } |  |  | 0.742 |
| ns | 5822 |  | 154 | aho_corasick.c: complete definition roster | 4.8 |  | 0.733 |
| walker |  | 5827 | 12 | Code::CodeKey { rung: Decl, file: krep.c, decl: 26, sub: 0, line: 111 } |  |  | 0.733 |
| walker |  | 5840 | 13 | Code::CodeKey { rung: Decl, file: krep.c, decl: 23, sub: 0, line: 104 } |  |  | 0.733 |
| walker |  | 5855 | 15 | Code::CodeKey { rung: Decl, file: krep.c, decl: 22, sub: 0, line: 101 } |  |  | 0.733 |
| walker |  | 5868 | 13 | Code::CodeKey { rung: Doc, file: krep.c, decl: 23, sub: 0, line: 104 } |  |  | 0.733 |
| walker |  | 5882 | 14 | Code::CodeKey { rung: Doc, file: krep.c, decl: 22, sub: 0, line: 101 } |  |  | 0.733 |
| walker |  | 5896 | 14 | Code::CodeKey { rung: Doc, file: krep.c, decl: 27, sub: 0, line: 116 } |  |  | 0.733 |
| ns | 6014 |  | 192 | aho_corasick.c trie data model: ac_node_t and struct ac_trie | 4.9 |  | 0.715 |
| walker |  | 6084 | 188 | Code::CodeKey { rung: Names, file: krep.c, decl: 0, sub: 3, line: 0 } |  |  | 0.718 |
| walker |  | 6089 | 5 | Code::CodeKey { rung: Decl, file: krep.c, decl: 35, sub: 0, line: 128 } |  |  | 0.718 |
| walker |  | 6094 | 5 | Code::CodeKey { rung: Decl, file: krep.c, decl: 36, sub: 0, line: 139 } |  |  | 0.718 |
| walker |  | 6099 | 5 | Code::CodeKey { rung: Decl, file: krep.c, decl: 37, sub: 0, line: 175 } |  |  | 0.718 |
| walker |  | 6104 | 5 | Code::CodeKey { rung: Decl, file: krep.c, decl: 38, sub: 0, line: 244 } |  |  | 0.718 |
| walker |  | 6109 | 5 | Code::CodeKey { rung: Decl, file: krep.c, decl: 39, sub: 0, line: 256 } |  |  | 0.718 |
| walker |  | 6119 | 10 | Code::CodeKey { rung: Doc, file: krep.c, decl: 36, sub: 0, line: 139 } |  |  | 0.718 |
| walker |  | 6130 | 11 | Code::CodeKey { rung: Doc, file: krep.c, decl: 35, sub: 0, line: 128 } |  |  | 0.718 |
| walker |  | 6142 | 12 | Code::CodeKey { rung: Doc, file: krep.c, decl: 34, sub: 0, line: 125 } |  |  | 0.718 |
| walker |  | 6155 | 13 | Code::CodeKey { rung: Doc, file: krep.c, decl: 38, sub: 0, line: 244 } |  |  | 0.718 |
| walker |  | 6173 | 18 | Code::CodeKey { rung: Doc, file: krep.c, decl: 37, sub: 0, line: 175 } |  |  | 0.718 |
| ns | 6272 |  | 258 | test/test_krep.c: complete roster of test functions, helpers and main | 4.10 |  | 0.699 |
| walker |  | 6352 | 179 | Code::CodeKey { rung: Names, file: krep.c, decl: 0, sub: 4, line: 0 } |  |  | 0.713 |
| walker |  | 6357 | 5 | Code::CodeKey { rung: Decl, file: krep.c, decl: 41, sub: 0, line: 363 } |  |  | 0.713 |
| walker |  | 6362 | 5 | Code::CodeKey { rung: Decl, file: krep.c, decl: 42, sub: 0, line: 401 } |  |  | 0.713 |
| walker |  | 6367 | 5 | Code::CodeKey { rung: Decl, file: krep.c, decl: 43, sub: 0, line: 420 } |  |  | 0.713 |
| walker |  | 6372 | 5 | Code::CodeKey { rung: Decl, file: krep.c, decl: 44, sub: 0, line: 438 } |  |  | 0.713 |
| walker |  | 6377 | 5 | Code::CodeKey { rung: Decl, file: krep.c, decl: 45, sub: 0, line: 461 } |  |  | 0.713 |
| walker |  | 6414 | 37 | Code::CodeKey { rung: Decl, file: krep.c, decl: 40, sub: 0, line: 329 } |  |  | 0.713 |
| walker |  | 6430 | 16 | Code::CodeKey { rung: Doc, file: krep.c, decl: 42, sub: 0, line: 401 } |  |  | 0.713 |
| walker |  | 6448 | 18 | Code::CodeKey { rung: Doc, file: krep.c, decl: 43, sub: 0, line: 420 } |  |  | 0.713 |
| walker |  | 6468 | 20 | Code::CodeKey { rung: Doc, file: krep.c, decl: 40, sub: 0, line: 329 } |  |  | 0.713 |
| ns | 6497 |  | 225 | test/test_regex.c and test/test_multiple_patterns.c rosters | 4.11 |  | 0.697 |
| ns | 6608 |  | 111 | test/test_directory.c roster: the separate recursive-search integration binary | 4.12 |  | 0.689 |
| walker |  | 6644 | 176 | Code::CodeKey { rung: Names, file: krep.c, decl: 0, sub: 5, line: 0 } |  |  | 0.695 |
| walker |  | 6650 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 46, sub: 0, line: 1084 } |  |  | 0.695 |
| walker |  | 6656 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 47, sub: 0, line: 1125 } |  |  | 0.695 |
| walker |  | 6662 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 48, sub: 0, line: 1137 } |  |  | 0.695 |
| walker |  | 6668 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 49, sub: 0, line: 1198 } |  |  | 0.695 |
| walker |  | 6674 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 50, sub: 0, line: 1213 } |  |  | 0.695 |
| walker |  | 6715 | 41 | Code::CodeKey { rung: Decl, file: krep.c, decl: 52, sub: 0, line: 1389 } |  |  | 0.695 |
| walker |  | 6765 | 50 | Code::CodeKey { rung: Decl, file: krep.c, decl: 51, sub: 0, line: 1259 } |  |  | 0.695 |
| walker |  | 6775 | 10 | Code::CodeKey { rung: Doc, file: krep.c, decl: 48, sub: 0, line: 1137 } |  |  | 0.695 |
| walker |  | 6786 | 11 | Code::CodeKey { rung: Doc, file: krep.c, decl: 47, sub: 0, line: 1125 } |  |  | 0.695 |
| walker |  | 6801 | 15 | Code::CodeKey { rung: Doc, file: krep.c, decl: 50, sub: 0, line: 1213 } |  |  | 0.695 |
| walker |  | 6818 | 17 | Code::CodeKey { rung: Doc, file: krep.c, decl: 49, sub: 0, line: 1198 } |  |  | 0.695 |
| ns | 6841 |  | 233 | test/test_compat.h: the TESTING wrapper roster and its guards | 4.13 |  | 0.683 |
| ns | 6972 |  | 131 | test/test_krep.h: shared test helper declarations | 4.14 |  | 0.674 |
| walker |  | 7007 | 189 | Code::CodeKey { rung: Names, file: krep.c, decl: 0, sub: 6, line: 0 } |  |  | 0.688 |
| walker |  | 7013 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 53, sub: 0, line: 1585 } |  |  | 0.688 |
| walker |  | 7019 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 55, sub: 0, line: 1771 } |  |  | 0.688 |
| walker |  | 7025 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 56, sub: 0, line: 1873 } |  |  | 0.688 |
| walker |  | 7031 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 57, sub: 0, line: 1919 } |  |  | 0.688 |
| walker |  | 7037 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 58, sub: 0, line: 1964 } |  |  | 0.688 |
| walker |  | 7043 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 59, sub: 0, line: 1999 } |  |  | 0.688 |
| walker |  | 7049 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 61, sub: 0, line: 2252 } |  |  | 0.688 |
| ns | 7080 |  | 108 | Makefile compiler configuration: CC, CFLAGS, LDFLAGS, PREFIX | 5.1 |  | 0.682 |
| walker |  | 7099 | 50 | Code::CodeKey { rung: Decl, file: krep.c, decl: 54, sub: 0, line: 1628 } |  |  | 0.682 |
| walker |  | 7109 | 10 | Code::CodeKey { rung: Doc, file: krep.c, decl: 60, sub: 0, line: 2249 } |  |  | 0.682 |
| walker |  | 7125 | 16 | Code::CodeKey { rung: Doc, file: krep.c, decl: 59, sub: 0, line: 1999 } |  |  | 0.682 |
| walker |  | 7141 | 16 | Code::CodeKey { rung: Doc, file: krep.c, decl: 61, sub: 0, line: 2252 } |  |  | 0.682 |
| walker |  | 7158 | 17 | Code::CodeKey { rung: Doc, file: krep.c, decl: 58, sub: 0, line: 1964 } |  |  | 0.682 |
| walker |  | 7177 | 19 | Code::CodeKey { rung: Doc, file: krep.c, decl: 56, sub: 0, line: 1873 } |  |  | 0.682 |
| walker |  | 7196 | 19 | Code::CodeKey { rung: Doc, file: krep.c, decl: 57, sub: 0, line: 1919 } |  |  | 0.682 |
| ns | 7316 |  | 236 | .github/workflows/ci.yml in full | 5.2 |  | 0.665 |
| walker |  | 7411 | 215 | Code::CodeKey { rung: Names, file: krep.c, decl: 0, sub: 7, line: 0 } |  |  | 0.674 |
| walker |  | 7417 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 62, sub: 0, line: 2265 } |  |  | 0.674 |
| walker |  | 7423 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 63, sub: 0, line: 2274 } |  |  | 0.674 |
| walker |  | 7429 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 64, sub: 0, line: 3071 } |  |  | 0.674 |
| walker |  | 7435 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 65, sub: 0, line: 3090 } |  |  | 0.674 |
| walker |  | 7441 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 66, sub: 0, line: 3122 } |  |  | 0.674 |
| walker |  | 7447 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 69, sub: 0, line: 3163 } |  |  | 0.674 |
| walker |  | 7453 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 70, sub: 0, line: 3180 } |  |  | 0.674 |
| walker |  | 7459 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 71, sub: 0, line: 3240 } |  |  | 0.674 |
| walker |  | 7493 | 34 | Code::CodeKey { rung: Decl, file: krep.c, decl: 67, sub: 0, line: 3146 } |  |  | 0.675 |
| walker |  | 7550 | 57 | Code::CodeKey { rung: Decl, file: krep.c, decl: 68, sub: 0, line: 3154 } |  |  | 0.675 |
| walker |  | 7563 | 13 | Code::CodeKey { rung: Doc, file: krep.c, decl: 62, sub: 0, line: 2265 } |  |  | 0.675 |
| walker |  | 7577 | 14 | Code::CodeKey { rung: Doc, file: krep.c, decl: 69, sub: 0, line: 3163 } |  |  | 0.675 |
| ns | 7580 |  | 264 | Makefile architecture detection: which SIMD flags each arch gets | 5.3 |  | 0.662 |
| walker |  | 7592 | 15 | Code::CodeKey { rung: Doc, file: krep.c, decl: 64, sub: 0, line: 3071 } |  |  | 0.662 |
| walker |  | 7607 | 15 | Code::CodeKey { rung: Doc, file: krep.c, decl: 65, sub: 0, line: 3090 } |  |  | 0.662 |
| walker |  | 7622 | 15 | Code::CodeKey { rung: Doc, file: krep.c, decl: 66, sub: 0, line: 3122 } |  |  | 0.662 |
| walker |  | 7637 | 15 | Code::CodeKey { rung: Doc, file: krep.c, decl: 67, sub: 0, line: 3146 } |  |  | 0.662 |
| walker |  | 7655 | 18 | Code::CodeKey { rung: Doc, file: krep.c, decl: 70, sub: 0, line: 3180 } |  |  | 0.662 |
| walker |  | 7675 | 20 | Code::CodeKey { rung: Doc, file: krep.c, decl: 68, sub: 0, line: 3154 } |  |  | 0.662 |
| walker |  | 7878 | 203 | Code::CodeKey { rung: Names, file: krep.c, decl: 0, sub: 8, line: 0 } |  |  | 0.674 |
| ns | 7883 |  | 303 | Makefile compile/link rules and the parallel -DTESTING build | 5.4 |  | 0.663 |
| walker |  | 7884 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 72, sub: 0, line: 3272 } |  |  | 0.663 |
| walker |  | 7890 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 73, sub: 0, line: 3297 } |  |  | 0.663 |
| walker |  | 7896 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 75, sub: 0, line: 3442 } |  |  | 0.663 |
| walker |  | 7902 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 78, sub: 0, line: 4046 } |  |  | 0.663 |
| walker |  | 7908 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 79, sub: 0, line: 4104 } |  |  | 0.663 |
| walker |  | 7914 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 80, sub: 0, line: 4209 } |  |  | 0.663 |
| walker |  | 7938 | 24 | Code::CodeKey { rung: Decl, file: krep.c, decl: 74, sub: 0, line: 3310 } |  |  | 0.663 |
| walker |  | 7971 | 33 | Code::CodeKey { rung: Decl, file: krep.c, decl: 76, sub: 0, line: 3450 } |  |  | 0.663 |
| walker |  | 8012 | 41 | Code::CodeKey { rung: Decl, file: krep.c, decl: 77, sub: 0, line: 3891 } |  |  | 0.663 |
| walker |  | 8023 | 11 | Code::CodeKey { rung: Doc, file: krep.c, decl: 77, sub: 0, line: 3891 } |  |  | 0.663 |
| walker |  | 8037 | 14 | Code::CodeKey { rung: Doc, file: krep.c, decl: 80, sub: 0, line: 4209 } |  |  | 0.663 |
| walker |  | 8052 | 15 | Code::CodeKey { rung: Doc, file: krep.c, decl: 74, sub: 0, line: 3310 } |  |  | 0.653 |
| ns | 8052 |  | 169 | Makefile run targets: test, test-directory, ci, bench-rg, all-tests | 5.5 |  | 0.653 |
| walker |  | 8068 | 16 | Code::CodeKey { rung: Doc, file: krep.c, decl: 78, sub: 0, line: 4046 } |  |  | 0.653 |
| walker |  | 8086 | 18 | Code::CodeKey { rung: Doc, file: krep.c, decl: 79, sub: 0, line: 4104 } |  |  | 0.653 |
| walker |  | 8106 | 20 | Code::CodeKey { rung: Doc, file: krep.c, decl: 72, sub: 0, line: 3272 } |  |  | 0.653 |
| walker |  | 8127 | 21 | Code::CodeKey { rung: Doc, file: krep.c, decl: 73, sub: 0, line: 3297 } |  |  | 0.653 |
| ns | 8209 |  | 157 | .github/workflows/release.yml: tag trigger and release artifacts | 5.6 |  | 0.641 |
| walker |  | 8313 | 186 | Code::CodeKey { rung: Names, file: krep.c, decl: 0, sub: 9, line: 0 } |  |  | 0.653 |
| walker |  | 8319 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 81, sub: 0, line: 4251 } |  |  | 0.653 |
| walker |  | 8325 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 82, sub: 0, line: 4313 } |  |  | 0.653 |
| walker |  | 8331 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 83, sub: 0, line: 4332 } |  |  | 0.653 |
| walker |  | 8372 | 41 | Code::CodeKey { rung: Decl, file: krep.c, decl: 84, sub: 0, line: 4371 } |  |  | 0.653 |
| walker |  | 8435 | 63 | Code::CodeKey { rung: Decl, file: krep.c, decl: 85, sub: 0, line: 4505 } |  |  | 0.655 |
| ns | 8462 |  | 253 | krep.c constants: VERSION and every performance tunable | 6.1 |  | 0.662 |
| walker |  | 8500 | 65 | Code::CodeKey { rung: Decl, file: krep.c, decl: 88, sub: 0, line: 5104 } |  |  | 0.664 |
| walker |  | 8565 | 65 | Code::CodeKey { rung: Decl, file: krep.c, decl: 87, sub: 0, line: 4872 } |  |  | 0.666 |
| walker |  | 8630 | 65 | Code::CodeKey { rung: Decl, file: krep.c, decl: 86, sub: 0, line: 4699 } |  |  | 0.668 |
| walker |  | 8641 | 11 | Code::CodeKey { rung: Doc, file: krep.c, decl: 83, sub: 0, line: 4332 } |  |  | 0.668 |
| walker |  | 8654 | 13 | Code::CodeKey { rung: Doc, file: krep.c, decl: 82, sub: 0, line: 4313 } |  |  | 0.668 |
| walker |  | 8669 | 15 | Code::CodeKey { rung: Doc, file: krep.c, decl: 81, sub: 0, line: 4251 } |  |  | 0.668 |
| walker |  | 8690 | 21 | Code::CodeKey { rung: Doc, file: krep.c, decl: 84, sub: 0, line: 4371 } |  |  | 0.668 |
| ns | 8696 |  | 234 | krep.c global option state and the lower_table constructor | 6.2 |  | 0.668 |
| walker |  | 8714 | 24 | Code::CodeKey { rung: Doc, file: krep.c, decl: 71, sub: 0, line: 3240 } |  |  | 0.668 |
| walker |  | 8746 | 32 | Code::CodeKey { rung: Doc, file: krep.c, decl: 44, sub: 0, line: 438 } |  |  | 0.668 |
| walker |  | 8779 | 33 | Code::CodeKey { rung: Doc, file: krep.c, decl: 86, sub: 0, line: 4699 } |  |  | 0.668 |
| walker |  | 8814 | 35 | Code::CodeKey { rung: Doc, file: krep.c, decl: 75, sub: 0, line: 3442 } |  |  | 0.668 |
| walker |  | 8850 | 36 | Code::CodeKey { rung: Doc, file: krep.c, decl: 51, sub: 0, line: 1259 } |  |  | 0.668 |
| walker |  | 8887 | 37 | Code::CodeKey { rung: Doc, file: krep.c, decl: 46, sub: 0, line: 1084 } |  |  | 0.668 |
| walker |  | 8926 | 39 | Code::CodeKey { rung: Doc, file: krep.c, decl: 88, sub: 0, line: 5104 } |  |  | 0.668 |
| walker |  | 8966 | 40 | Code::CodeKey { rung: Doc, file: krep.c, decl: 41, sub: 0, line: 363 } |  |  | 0.660 |
| ns | 8966 |  | 270 | main: the getopt_long table and short-option string | 6.3 |  | 0.660 |
| walker |  | 9212 | 246 | Code::CodeKey { rung: Names, file: aho_corasick.c, decl: 0, sub: 0, line: 0 } |  |  | 0.671 |
| walker |  | 9217 | 5 | Code::CodeKey { rung: Decl, file: aho_corasick.c, decl: 3, sub: 0, line: 34 } |  |  | 0.671 |
| walker |  | 9222 | 5 | Code::CodeKey { rung: Decl, file: aho_corasick.c, decl: 4, sub: 0, line: 55 } |  |  | 0.671 |
| walker |  | 9227 | 5 | Code::CodeKey { rung: Decl, file: aho_corasick.c, decl: 5, sub: 0, line: 86 } |  |  | 0.671 |
| walker |  | 9232 | 5 | Code::CodeKey { rung: Decl, file: aho_corasick.c, decl: 6, sub: 0, line: 111 } |  |  | 0.671 |
| walker |  | 9237 | 5 | Code::CodeKey { rung: Decl, file: aho_corasick.c, decl: 7, sub: 0, line: 274 } |  |  | 0.671 |
| walker |  | 9242 | 5 | Code::CodeKey { rung: Decl, file: aho_corasick.c, decl: 8, sub: 0, line: 287 } |  |  | 0.671 |
| walker |  | 9279 | 37 | Code::CodeKey { rung: Decl, file: aho_corasick.c, decl: 12, sub: 0, line: 299 } |  |  | 0.671 |
| ns | 9299 |  | 333 | select_search_algorithm: dispatch head through the short-pattern branch | 6.4 |  | 0.657 |
| walker |  | 9342 | 63 | Code::CodeKey { rung: Decl, file: aho_corasick.c, decl: 2, sub: 0, line: 26 } |  |  | 0.659 |
| walker |  | 9444 | 102 | Code::CodeKey { rung: Decl, file: aho_corasick.c, decl: 1, sub: 0, line: 17 } |  |  | 0.667 |
| walker |  | 9628 | 184 | Code::CodeKey { rung: Names, file: aho_corasick.h, decl: 0, sub: 0, line: 0 } |  |  | 0.673 |
| walker |  | 9638 | 10 | Code::CodeKey { rung: Doc, file: aho_corasick.h, decl: 5, sub: 0, line: 23 } |  |  | 0.673 |
| walker |  | 9650 | 12 | Code::CodeKey { rung: Doc, file: aho_corasick.h, decl: 9, sub: 0, line: 33 } |  |  | 0.675 |
| walker |  | 9662 | 12 | Code::CodeKey { rung: Doc, file: aho_corasick.h, decl: 11, sub: 0, line: 39 } |  |  | 0.677 |
| walker |  | 9676 | 14 | Code::CodeKey { rung: Doc, file: aho_corasick.h, decl: 6, sub: 0, line: 26 } |  |  | 0.677 |
| ns | 9685 |  | 386 | select_search_algorithm: SIMD length limits and the KMP/Boyer-Moore fallback | 6.5 | 6.4 | 0.661 |
| walker |  | 9691 | 15 | Code::CodeKey { rung: Doc, file: aho_corasick.h, decl: 8, sub: 0, line: 30 } |  |  | 0.663 |
| walker |  | 9709 | 18 | Code::CodeKey { rung: Doc, file: aho_corasick.h, decl: 1, sub: 0, line: 14 } |  |  | 0.663 |
| walker |  | 9729 | 20 | Code::CodeKey { rung: Doc, file: aho_corasick.h, decl: 10, sub: 0, line: 36 } |  |  | 0.667 |
| walker |  | 9764 | 35 | Code::CodeKey { rung: Doc, file: aho_corasick.h, decl: 3, sub: 0, line: 19 } |  |  | 0.667 |
| walker |  | 9773 | 9 | Code::CodeKey { rung: Doc, file: aho_corasick.c, decl: 3, sub: 0, line: 34 } |  |  | 0.667 |
| walker |  | 9785 | 12 | Code::CodeKey { rung: Doc, file: aho_corasick.c, decl: 4, sub: 0, line: 55 } |  |  | 0.667 |
| walker |  | 9798 | 13 | Code::CodeKey { rung: Doc, file: aho_corasick.c, decl: 12, sub: 0, line: 299 } |  |  | 0.667 |
| walker |  | 9812 | 14 | Code::CodeKey { rung: Doc, file: aho_corasick.c, decl: 6, sub: 0, line: 111 } |  |  | 0.667 |
| walker |  | 9826 | 14 | Code::CodeKey { rung: Doc, file: aho_corasick.c, decl: 7, sub: 0, line: 274 } |  |  | 0.667 |
| walker |  | 9840 | 14 | Code::CodeKey { rung: Doc, file: aho_corasick.c, decl: 11, sub: 0, line: 296 } |  |  | 0.667 |
| walker |  | 9855 | 15 | Code::CodeKey { rung: Doc, file: aho_corasick.c, decl: 5, sub: 0, line: 86 } |  |  | 0.667 |
| ns | 9868 |  | 183 | gitignore data model: pattern record and parent-chained context | 6.6 |  | 0.672 |
| walker |  | 9904 | 49 | Code::CodeKey { rung: Doc, file: krep.c, decl: 39, sub: 0, line: 256 } |  |  | 0.672 |
| walker |  | 9925 | 21 | Code::CodeKey { rung: Doc, file: aho_corasick.c, decl: 9, sub: 0, line: 293 } |  |  | 0.672 |
| walker |  | 9947 | 22 | Code::CodeKey { rung: Doc, file: aho_corasick.c, decl: 8, sub: 0, line: 287 } |  |  | 0.672 |
| ns | 9993 |  | 125 | Licence header, dependabot config and .gitignore | 7.1 |  | 0.665 |
