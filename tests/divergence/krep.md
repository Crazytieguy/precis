Score(3000)=0.695 I=0.847 C=0.570 ns_rows≤3K=20/56 grid(1000/1442/2080/3000/4327/6240/9000)=0.649/0.696/0.678/0.695/0.798/0.727/0.667

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 35 | 35 | Fs::DirListing { dir: . } |  |  | 0.000 |
| ns | 66 |  | 66 | Repository identity: title line and the one-paragraph what-it-is | 1.1 |  | 0.000 |
| walker |  | 101 | 66 | Markdown::ReadmeHeadline { file: README.md } |  |  | 1.000 |
| ns | 101 |  | 35 | Complete root directory listing | 1.2 |  | 1.000 |
| walker |  | 114 | 13 | Fs::DirListing { dir: .github } |  |  | 1.000 |
| walker |  | 122 | 8 | Fs::DirListing { dir: .github/workflows } |  |  | 1.000 |
| ns | 164 |  | 63 | Complete listings of test/, .github/ and .github/workflows/ | 1.3 |  | 0.705 |
| walker |  | 193 | 71 | Plaintext::Whole { file: Makefile } |  |  | 0.708 |
| ns | 215 |  | 51 | Makefile: binary name, source list, and the complete .PHONY target list | 1.4 |  | 0.671 |
| walker |  | 235 | 42 | Fs::DirListing { dir: test } |  |  | 0.944 |
| ns | 333 |  | 118 | README invocation synopsis: all six supported command forms | 1.5 |  | 0.817 |
| ns | 463 |  | 130 | Every H2 heading location in README.md | 1.6 |  | 0.679 |
| walker |  | 517 | 282 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.860 |
| ns | 559 |  | 96 | krep.h public API roster: the three search entry points, match-result management, printing | 1.7 |  | 0.771 |
| walker |  | 619 | 102 | Markdown::CommandBlock { file: README.md, row: 54 } |  |  | 0.774 |
| walker |  | 734 | 115 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.776 |
| ns | 789 |  | 230 | krep.h search-algorithm roster with the SIMD #if guards intact | 1.8 |  | 0.638 |
| walker |  | 866 | 132 | Markdown::Section { file: README.md, section_index: 5, keeps_default_concavity: true } |  |  | 0.716 |
| ns | 996 |  | 207 | aho_corasick.h: the complete second-module interface | 1.9 |  | 0.647 |
| ns | 1173 |  | 177 | README Key Features, first half | 2.1 |  | 0.618 |
| walker |  | 1218 | 352 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: true } |  |  | 0.677 |
| walker |  | 1282 | 64 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.677 |
| ns | 1343 |  | 170 | README Key Features, second half | 2.2 |  | 0.693 |
| ns | 1534 |  | 191 | README command-line options table, first half (-i through -F) | 2.3 |  | 0.663 |
| walker |  | 1697 | 415 | Markdown::Section { file: README.md, section_index: 7, keeps_default_concavity: true } |  |  | 0.720 |
| ns | 1753 |  | 219 | README command-line options table, second half (-r through -h) | 2.4 |  | 0.735 |
| walker |  | 1819 | 122 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.735 |
| ns | 1963 |  | 210 | README: the documented smart algorithm-selection policy | 2.5 |  | 0.695 |
| walker |  | 2010 | 191 | Code::CodeKey { rung: Names, file: krep.h, decl: 0, sub: 0, line: 0 } |  |  | 0.696 |
| walker |  | 2024 | 14 | Code::CodeKey { rung: Doc, file: krep.h, decl: 1, sub: 0, line: 19 } |  |  | 0.696 |
| walker |  | 2038 | 14 | Code::CodeKey { rung: Doc, file: krep.h, decl: 2, sub: 0, line: 22 } |  |  | 0.696 |
| walker |  | 2058 | 20 | Code::CodeKey { rung: Doc, file: krep.h, decl: 4, sub: 0, line: 34 } |  |  | 0.696 |
| ns | 2069 |  | 106 | README usage examples: the six concrete invocations, command lines only | 2.6 |  | 0.678 |
| ns | 2181 |  | 112 | README: the multi-threading architecture section | 2.7 |  | 0.652 |
| walker |  | 2255 | 197 | Code::CodeKey { rung: Names, file: krep.h, decl: 0, sub: 1, line: 0 } |  |  | 0.654 |
| walker |  | 2287 | 32 | Code::CodeKey { rung: Decl, file: krep.h, decl: 17, sub: 0, line: 98 } |  |  | 0.635 |
| ns | 2287 |  | 106 | README: the recursive-search skipping rules | 2.8 |  | 0.635 |
| walker |  | 2327 | 40 | Code::CodeKey { rung: Decl, file: krep.h, decl: 14, sub: 0, line: 49 } |  |  | 0.636 |
| walker |  | 2389 | 62 | Code::CodeKey { rung: Decl, file: krep.h, decl: 15, sub: 0, line: 55 } |  |  | 0.638 |
| walker |  | 2398 | 9 | Code::CodeKey { rung: Doc, file: krep.h, decl: 10, sub: 0, line: 42 } |  |  | 0.638 |
| ns | 2402 |  | 115 | README scope note: explicitly not a grep/ripgrep replacement | 2.9 |  | 0.645 |
| ns | 2512 |  | 110 | README install and build-from-source commands | 2.10 |  | 0.642 |
| walker |  | 2766 | 368 | Code::CodeKey { rung: Decl, file: krep.h, decl: 16, sub: 0, line: 65 } |  |  | 0.652 |
| walker |  | 2779 | 13 | Code::CodeKey { rung: Doc, file: krep.h, decl: 16, sub: 0, line: 65 } |  |  | 0.652 |
| walker |  | 2802 | 23 | Code::CodeKey { rung: Doc, file: krep.h, decl: 14, sub: 0, line: 49 } |  |  | 0.654 |
| ns | 2902 |  | 390 | search_params_t: the struct every search function takes | 3.1 |  | 0.692 |
| walker |  | 3011 | 209 | Code::CodeKey { rung: Names, file: krep.h, decl: 0, sub: 2, line: 0 } |  |  | 0.698 |
| walker |  | 3062 | 51 | Code::CodeKey { rung: Decl, file: krep.h, decl: 19, sub: 0, line: 125 } |  |  | 0.698 |
| ns | 3089 |  | 187 | match_position_t and match_result_t | 3.2 |  | 0.710 |
| ns | 3180 |  | 91 | search_func_t: the algorithm function-pointer type | 3.3 |  | 0.705 |
| walker |  | 3223 | 161 | Code::CodeKey { rung: Decl, file: krep.h, decl: 20, sub: 0, line: 132 } |  |  | 0.705 |
| walker |  | 3232 | 9 | Code::CodeKey { rung: Doc, file: krep.h, decl: 19, sub: 0, line: 125 } |  |  | 0.705 |
| walker |  | 3241 | 9 | Code::CodeKey { rung: Doc, file: krep.h, decl: 21, sub: 0, line: 146 } |  |  | 0.706 |
| ns | 3282 |  | 102 | krep.h thread-pool API: all four functions in full | 3.4 |  | 0.711 |
| walker |  | 3491 | 250 | Code::CodeKey { rung: Decl, file: krep.h, decl: 18, sub: 0, line: 104 } |  |  | 0.716 |
| walker |  | 3501 | 10 | Code::CodeKey { rung: Doc, file: krep.h, decl: 18, sub: 0, line: 104 } |  |  | 0.717 |
| ns | 3562 |  | 280 | thread_data_t: per-thread search state, including the false-sharing padding | 3.5 |  | 0.730 |
| ns | 3659 |  | 97 | krep.h helper roster: line finding, word matching, table preparation | 3.6 |  | 0.717 |
| walker |  | 3678 | 177 | Code::CodeKey { rung: Names, file: krep.h, decl: 0, sub: 3, line: 0 } |  |  | 0.746 |
| walker |  | 3685 | 7 | Code::CodeKey { rung: Doc, file: krep.h, decl: 33, sub: 0, line: 203 } |  |  | 0.746 |
| walker |  | 3695 | 10 | Code::CodeKey { rung: Doc, file: krep.h, decl: 28, sub: 0, line: 183 } |  |  | 0.746 |
| ns | 3826 |  | 167 | skip_directories: the complete recursive-search directory blocklist | 3.7 |  | 0.732 |
| walker |  | 3838 | 143 | Code::CodeKey { rung: Names, file: krep.h, decl: 0, sub: 4, line: 0 } |  |  | 0.736 |
| walker |  | 3990 | 152 | Code::CodeKey { rung: Names, file: krep.h, decl: 0, sub: 5, line: 0 } |  |  | 0.747 |
| walker |  | 4009 | 19 | Code::CodeKey { rung: Decl, file: krep.h, decl: 40, sub: 0, line: 218 } |  |  | 0.752 |
| ns | 4011 |  | 185 | skip_extensions: head, ellipsis, and the count line | 3.8 |  | 0.739 |
| walker |  | 4028 | 19 | Code::CodeKey { rung: Decl, file: krep.h, decl: 39, sub: 0, line: 214 } |  |  | 0.746 |
| walker |  | 4047 | 19 | Code::CodeKey { rung: Decl, file: krep.h, decl: 42, sub: 0, line: 226 } |  |  | 0.753 |
| walker |  | 4071 | 24 | Code::CodeKey { rung: Decl, file: krep.h, decl: 41, sub: 0, line: 222 } |  |  | 0.765 |
| walker |  | 4085 | 14 | Code::CodeKey { rung: Doc, file: krep.h, decl: 39, sub: 0, line: 214 } |  |  | 0.776 |
| ns | 4236 |  | 225 | ANSI colour macro block, output palette and help palette | 3.9 |  | 0.780 |
| walker |  | 4274 | 189 | Code::CodeKey { rung: Names, file: krep.h, decl: 0, sub: 6, line: 0 } |  |  | 0.798 |
| walker |  | 4279 | 5 | Code::CodeKey { rung: Decl, file: krep.h, decl: 49, sub: 0, line: 298 } |  |  | 0.798 |
| walker |  | 4284 | 5 | Code::CodeKey { rung: Decl, file: krep.h, decl: 50, sub: 0, line: 312 } |  |  | 0.798 |
| walker |  | 4292 | 8 | Code::CodeKey { rung: Doc, file: krep.h, decl: 43, sub: 0, line: 231 } |  |  | 0.798 |
| walker |  | 4329 | 37 | Code::CodeKey { rung: Doc, file: krep.h, decl: 17, sub: 0, line: 98 } |  |  | 0.807 |
| ns | 4395 |  | 159 | Documented contracts of the three entry points and the printer | 3.10 | 1.7 | 0.792 |
| walker |  | 4519 | 190 | Markdown::Section { file: README.md, section_index: 4, keeps_default_concavity: false } |  |  | 0.805 |
| walker |  | 4587 | 68 | Code::CodeKey { rung: Doc, file: krep.h, decl: 49, sub: 0, line: 298 } |  |  | 0.805 |
| ns | 4634 |  | 239 | krep.c file-level section banner map, all sixteen top-level banners | 4.1 |  | 0.781 |
| ns | 4758 |  | 124 | krep.c definition roster, lines 139-461: match results, line finding, printing | 4.2 |  | 0.766 |
| walker |  | 4772 | 185 | Markdown::Section { file: README.md, section_index: 9, keeps_default_concavity: false } |  |  | 0.786 |
| ns | 4932 |  | 174 | krep.c definition roster, lines 1084-1964: utilities, the four scalar algorithms, orchestration | 4.3 |  | 0.768 |
| walker |  | 4970 | 198 | Markdown::Section { file: README.md, section_index: 10, keeps_default_concavity: false } |  |  | 0.795 |
| walker |  | 5099 | 129 | Markdown::Section { file: README.md, section_index: 11, keeps_default_concavity: false } |  |  | 0.812 |
| ns | 5124 |  | 192 | krep.c definition roster, lines 1999-3442: public API bodies, skip filters, gitignore | 4.4 |  | 0.793 |
| walker |  | 5189 | 90 | Code::CodeKey { rung: Doc, file: krep.h, decl: 26, sub: 0, line: 170 } |  |  | 0.794 |
| ns | 5363 |  | 239 | krep.c definition roster, lines 3451-5108: main, thread pool, and the guarded SIMD kernels | 4.5 |  | 0.772 |
| walker |  | 5377 | 188 | Code::CodeKey { rung: Names, file: krep.c, decl: 0, sub: 0, line: 0 } |  |  | 0.772 |
| walker |  | 5394 | 17 | Code::CodeKey { rung: Decl, file: krep.c, decl: 1, sub: 0, line: 9 } |  |  | 0.772 |
| walker |  | 5401 | 7 | Code::CodeKey { rung: Doc, file: krep.c, decl: 5, sub: 0, line: 77 } |  |  | 0.772 |
| walker |  | 5412 | 11 | Code::CodeKey { rung: Doc, file: krep.c, decl: 3, sub: 0, line: 42 } |  |  | 0.772 |
| walker |  | 5426 | 14 | Code::CodeKey { rung: Doc, file: krep.c, decl: 4, sub: 0, line: 44 } |  |  | 0.772 |
| walker |  | 5442 | 16 | Code::CodeKey { rung: Doc, file: krep.c, decl: 2, sub: 0, line: 39 } |  |  | 0.772 |
| walker |  | 5463 | 21 | Code::CodeKey { rung: Doc, file: krep.c, decl: 1, sub: 0, line: 9 } |  |  | 0.772 |
| ns | 5546 |  | 183 | search_file internal section map, all eleven banners inside lines 2274-3070 | 4.6 |  | 0.759 |
| walker |  | 5658 | 195 | Code::CodeKey { rung: Names, file: krep.c, decl: 0, sub: 1, line: 0 } |  |  | 0.760 |
| ns | 5668 |  | 122 | print_matching_items internal section map: the -o and full-line output modes | 4.7 |  | 0.751 |
| walker |  | 5670 | 12 | Code::CodeKey { rung: Decl, file: krep.c, decl: 12, sub: 0, line: 84 } |  |  | 0.751 |
| walker |  | 5679 | 9 | Code::CodeKey { rung: Doc, file: krep.c, decl: 17, sub: 0, line: 93 } |  |  | 0.751 |
| ns | 5822 |  | 154 | aho_corasick.c: complete definition roster | 4.8 |  | 0.741 |
| walker |  | 5882 | 203 | Code::CodeKey { rung: Names, file: krep.c, decl: 0, sub: 2, line: 0 } |  |  | 0.741 |
| walker |  | 5889 | 7 | Code::CodeKey { rung: Decl, file: krep.c, decl: 26, sub: 0, line: 111 } |  |  | 0.741 |
| walker |  | 5902 | 13 | Code::CodeKey { rung: Doc, file: krep.c, decl: 23, sub: 0, line: 104 } |  |  | 0.741 |
| walker |  | 5916 | 14 | Code::CodeKey { rung: Doc, file: krep.c, decl: 22, sub: 0, line: 101 } |  |  | 0.741 |
| walker |  | 5930 | 14 | Code::CodeKey { rung: Doc, file: krep.c, decl: 27, sub: 0, line: 116 } |  |  | 0.741 |
| ns | 6014 |  | 192 | aho_corasick.c trie data model: ac_node_t and struct ac_trie | 4.9 |  | 0.724 |
| walker |  | 6122 | 192 | Code::CodeKey { rung: Names, file: krep.c, decl: 0, sub: 3, line: 0 } |  |  | 0.725 |
| walker |  | 6127 | 5 | Code::CodeKey { rung: Decl, file: krep.c, decl: 35, sub: 0, line: 128 } |  |  | 0.725 |
| walker |  | 6132 | 5 | Code::CodeKey { rung: Decl, file: krep.c, decl: 36, sub: 0, line: 139 } |  |  | 0.725 |
| walker |  | 6137 | 5 | Code::CodeKey { rung: Decl, file: krep.c, decl: 37, sub: 0, line: 175 } |  |  | 0.725 |
| walker |  | 6147 | 10 | Code::CodeKey { rung: Doc, file: krep.c, decl: 36, sub: 0, line: 139 } |  |  | 0.725 |
| walker |  | 6158 | 11 | Code::CodeKey { rung: Doc, file: krep.c, decl: 35, sub: 0, line: 128 } |  |  | 0.725 |
| walker |  | 6170 | 12 | Code::CodeKey { rung: Doc, file: krep.c, decl: 34, sub: 0, line: 125 } |  |  | 0.725 |
| walker |  | 6188 | 18 | Code::CodeKey { rung: Doc, file: krep.c, decl: 37, sub: 0, line: 175 } |  |  | 0.725 |
| ns | 6272 |  | 258 | test/test_krep.c: complete roster of test functions, helpers and main | 4.10 |  | 0.706 |
| walker |  | 6371 | 183 | Code::CodeKey { rung: Names, file: krep.c, decl: 0, sub: 4, line: 0 } |  |  | 0.718 |
| walker |  | 6376 | 5 | Code::CodeKey { rung: Decl, file: krep.c, decl: 38, sub: 0, line: 244 } |  |  | 0.718 |
| walker |  | 6381 | 5 | Code::CodeKey { rung: Decl, file: krep.c, decl: 39, sub: 0, line: 256 } |  |  | 0.718 |
| walker |  | 6386 | 5 | Code::CodeKey { rung: Decl, file: krep.c, decl: 41, sub: 0, line: 363 } |  |  | 0.718 |
| walker |  | 6391 | 5 | Code::CodeKey { rung: Decl, file: krep.c, decl: 42, sub: 0, line: 401 } |  |  | 0.718 |
| walker |  | 6396 | 5 | Code::CodeKey { rung: Decl, file: krep.c, decl: 43, sub: 0, line: 420 } |  |  | 0.718 |
| walker |  | 6401 | 5 | Code::CodeKey { rung: Decl, file: krep.c, decl: 44, sub: 0, line: 438 } |  |  | 0.718 |
| walker |  | 6438 | 37 | Code::CodeKey { rung: Decl, file: krep.c, decl: 40, sub: 0, line: 329 } |  |  | 0.718 |
| walker |  | 6451 | 13 | Code::CodeKey { rung: Doc, file: krep.c, decl: 38, sub: 0, line: 244 } |  |  | 0.718 |
| walker |  | 6467 | 16 | Code::CodeKey { rung: Doc, file: krep.c, decl: 42, sub: 0, line: 401 } |  |  | 0.718 |
| walker |  | 6485 | 18 | Code::CodeKey { rung: Doc, file: krep.c, decl: 43, sub: 0, line: 420 } |  |  | 0.718 |
| ns | 6497 |  | 225 | test/test_regex.c and test/test_multiple_patterns.c rosters | 4.11 |  | 0.702 |
| ns | 6608 |  | 111 | test/test_directory.c roster: the separate recursive-search integration binary | 4.12 |  | 0.694 |
| walker |  | 6651 | 166 | Code::CodeKey { rung: Names, file: krep.c, decl: 0, sub: 5, line: 0 } |  |  | 0.700 |
| walker |  | 6656 | 5 | Code::CodeKey { rung: Decl, file: krep.c, decl: 45, sub: 0, line: 461 } |  |  | 0.700 |
| walker |  | 6662 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 46, sub: 0, line: 1084 } |  |  | 0.700 |
| walker |  | 6668 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 47, sub: 0, line: 1125 } |  |  | 0.700 |
| walker |  | 6674 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 48, sub: 0, line: 1137 } |  |  | 0.700 |
| walker |  | 6680 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 49, sub: 0, line: 1198 } |  |  | 0.700 |
| walker |  | 6686 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 50, sub: 0, line: 1213 } |  |  | 0.700 |
| walker |  | 6696 | 10 | Code::CodeKey { rung: Doc, file: krep.c, decl: 48, sub: 0, line: 1137 } |  |  | 0.700 |
| walker |  | 6707 | 11 | Code::CodeKey { rung: Doc, file: krep.c, decl: 47, sub: 0, line: 1125 } |  |  | 0.700 |
| walker |  | 6722 | 15 | Code::CodeKey { rung: Doc, file: krep.c, decl: 50, sub: 0, line: 1213 } |  |  | 0.700 |
| walker |  | 6739 | 17 | Code::CodeKey { rung: Doc, file: krep.c, decl: 49, sub: 0, line: 1198 } |  |  | 0.700 |
| walker |  | 6759 | 20 | Code::CodeKey { rung: Doc, file: krep.c, decl: 40, sub: 0, line: 329 } |  |  | 0.700 |
| ns | 6841 |  | 233 | test/test_compat.h: the TESTING wrapper roster and its guards | 4.13 |  | 0.688 |
| walker |  | 6962 | 203 | Code::CodeKey { rung: Names, file: krep.c, decl: 0, sub: 6, line: 0 } |  |  | 0.705 |
| walker |  | 6968 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 53, sub: 0, line: 1585 } |  |  | 0.705 |
| ns | 6972 |  | 131 | test/test_krep.h: shared test helper declarations | 4.14 |  | 0.696 |
| walker |  | 6974 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 55, sub: 0, line: 1771 } |  |  | 0.696 |
| walker |  | 6980 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 56, sub: 0, line: 1873 } |  |  | 0.696 |
| walker |  | 6986 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 57, sub: 0, line: 1919 } |  |  | 0.696 |
| walker |  | 6992 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 58, sub: 0, line: 1964 } |  |  | 0.696 |
| walker |  | 6998 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 59, sub: 0, line: 1999 } |  |  | 0.696 |
| walker |  | 7039 | 41 | Code::CodeKey { rung: Decl, file: krep.c, decl: 52, sub: 0, line: 1389 } |  |  | 0.696 |
| ns | 7080 |  | 108 | Makefile compiler configuration: CC, CFLAGS, LDFLAGS, PREFIX | 5.1 |  | 0.690 |
| walker |  | 7089 | 50 | Code::CodeKey { rung: Decl, file: krep.c, decl: 51, sub: 0, line: 1259 } |  |  | 0.690 |
| walker |  | 7139 | 50 | Code::CodeKey { rung: Decl, file: krep.c, decl: 54, sub: 0, line: 1628 } |  |  | 0.690 |
| walker |  | 7155 | 16 | Code::CodeKey { rung: Doc, file: krep.c, decl: 59, sub: 0, line: 1999 } |  |  | 0.690 |
| walker |  | 7172 | 17 | Code::CodeKey { rung: Doc, file: krep.c, decl: 58, sub: 0, line: 1964 } |  |  | 0.690 |
| walker |  | 7191 | 19 | Code::CodeKey { rung: Doc, file: krep.c, decl: 56, sub: 0, line: 1873 } |  |  | 0.690 |
| walker |  | 7210 | 19 | Code::CodeKey { rung: Doc, file: krep.c, decl: 57, sub: 0, line: 1919 } |  |  | 0.690 |
| ns | 7316 |  | 236 | .github/workflows/ci.yml in full | 5.2 |  | 0.672 |
| walker |  | 7435 | 225 | Code::CodeKey { rung: Names, file: krep.c, decl: 0, sub: 7, line: 0 } |  |  | 0.680 |
| walker |  | 7441 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 61, sub: 0, line: 2252 } |  |  | 0.680 |
| walker |  | 7447 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 62, sub: 0, line: 2265 } |  |  | 0.680 |
| walker |  | 7453 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 63, sub: 0, line: 2274 } |  |  | 0.680 |
| walker |  | 7459 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 64, sub: 0, line: 3071 } |  |  | 0.680 |
| walker |  | 7465 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 65, sub: 0, line: 3090 } |  |  | 0.680 |
| walker |  | 7471 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 66, sub: 0, line: 3122 } |  |  | 0.680 |
| walker |  | 7477 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 69, sub: 0, line: 3163 } |  |  | 0.680 |
| walker |  | 7483 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 70, sub: 0, line: 3180 } |  |  | 0.680 |
| walker |  | 7517 | 34 | Code::CodeKey { rung: Decl, file: krep.c, decl: 67, sub: 0, line: 3146 } |  |  | 0.680 |
| walker |  | 7574 | 57 | Code::CodeKey { rung: Decl, file: krep.c, decl: 68, sub: 0, line: 3154 } |  |  | 0.681 |
| ns | 7580 |  | 264 | Makefile architecture detection: which SIMD flags each arch gets | 5.3 |  | 0.667 |
| walker |  | 7584 | 10 | Code::CodeKey { rung: Doc, file: krep.c, decl: 60, sub: 0, line: 2249 } |  |  | 0.667 |
| walker |  | 7597 | 13 | Code::CodeKey { rung: Doc, file: krep.c, decl: 62, sub: 0, line: 2265 } |  |  | 0.667 |
| walker |  | 7611 | 14 | Code::CodeKey { rung: Doc, file: krep.c, decl: 69, sub: 0, line: 3163 } |  |  | 0.667 |
| walker |  | 7626 | 15 | Code::CodeKey { rung: Doc, file: krep.c, decl: 64, sub: 0, line: 3071 } |  |  | 0.667 |
| walker |  | 7641 | 15 | Code::CodeKey { rung: Doc, file: krep.c, decl: 65, sub: 0, line: 3090 } |  |  | 0.667 |
| walker |  | 7656 | 15 | Code::CodeKey { rung: Doc, file: krep.c, decl: 66, sub: 0, line: 3122 } |  |  | 0.667 |
| walker |  | 7671 | 15 | Code::CodeKey { rung: Doc, file: krep.c, decl: 67, sub: 0, line: 3146 } |  |  | 0.668 |
| walker |  | 7687 | 16 | Code::CodeKey { rung: Doc, file: krep.c, decl: 61, sub: 0, line: 2252 } |  |  | 0.668 |
| walker |  | 7705 | 18 | Code::CodeKey { rung: Doc, file: krep.c, decl: 70, sub: 0, line: 3180 } |  |  | 0.668 |
| walker |  | 7725 | 20 | Code::CodeKey { rung: Doc, file: krep.c, decl: 68, sub: 0, line: 3154 } |  |  | 0.668 |
| ns | 7883 |  | 303 | Makefile compile/link rules and the parallel -DTESTING build | 5.4 |  | 0.657 |
| walker |  | 7907 | 182 | Code::CodeKey { rung: Names, file: krep.c, decl: 0, sub: 8, line: 0 } |  |  | 0.669 |
| walker |  | 7913 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 71, sub: 0, line: 3240 } |  |  | 0.669 |
| walker |  | 7919 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 72, sub: 0, line: 3272 } |  |  | 0.669 |
| walker |  | 7925 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 73, sub: 0, line: 3297 } |  |  | 0.669 |
| walker |  | 7931 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 75, sub: 0, line: 3442 } |  |  | 0.669 |
| walker |  | 7937 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 78, sub: 0, line: 4046 } |  |  | 0.669 |
| walker |  | 7961 | 24 | Code::CodeKey { rung: Decl, file: krep.c, decl: 74, sub: 0, line: 3310 } |  |  | 0.669 |
| walker |  | 7994 | 33 | Code::CodeKey { rung: Decl, file: krep.c, decl: 76, sub: 0, line: 3450 } |  |  | 0.669 |
| walker |  | 8035 | 41 | Code::CodeKey { rung: Decl, file: krep.c, decl: 77, sub: 0, line: 3891 } |  |  | 0.669 |
| walker |  | 8046 | 11 | Code::CodeKey { rung: Doc, file: krep.c, decl: 77, sub: 0, line: 3891 } |  |  | 0.669 |
| ns | 8052 |  | 169 | Makefile run targets: test, test-directory, ci, bench-rg, all-tests | 5.5 |  | 0.659 |
| walker |  | 8061 | 15 | Code::CodeKey { rung: Doc, file: krep.c, decl: 74, sub: 0, line: 3310 } |  |  | 0.659 |
| walker |  | 8077 | 16 | Code::CodeKey { rung: Doc, file: krep.c, decl: 78, sub: 0, line: 4046 } |  |  | 0.659 |
| walker |  | 8097 | 20 | Code::CodeKey { rung: Doc, file: krep.c, decl: 72, sub: 0, line: 3272 } |  |  | 0.659 |
| walker |  | 8118 | 21 | Code::CodeKey { rung: Doc, file: krep.c, decl: 73, sub: 0, line: 3297 } |  |  | 0.659 |
| walker |  | 8139 | 21 | Code::CodeKey { rung: Doc, file: krep.c, decl: 76, sub: 0, line: 3450 } |  |  | 0.659 |
| ns | 8209 |  | 157 | .github/workflows/release.yml: tag trigger and release artifacts | 5.6 |  | 0.647 |
| walker |  | 8372 | 233 | Code::CodeKey { rung: Names, file: krep.c, decl: 0, sub: 9, line: 0 } |  |  | 0.661 |
| walker |  | 8378 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 79, sub: 0, line: 4104 } |  |  | 0.661 |
| walker |  | 8384 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 80, sub: 0, line: 4209 } |  |  | 0.661 |
| walker |  | 8390 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 81, sub: 0, line: 4251 } |  |  | 0.661 |
| walker |  | 8396 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 82, sub: 0, line: 4313 } |  |  | 0.661 |
| walker |  | 8402 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 83, sub: 0, line: 4332 } |  |  | 0.661 |
| walker |  | 8443 | 41 | Code::CodeKey { rung: Decl, file: krep.c, decl: 84, sub: 0, line: 4371 } |  |  | 0.661 |
| ns | 8462 |  | 253 | krep.c constants: VERSION and every performance tunable | 6.1 |  | 0.668 |
| walker |  | 8506 | 63 | Code::CodeKey { rung: Decl, file: krep.c, decl: 85, sub: 0, line: 4505 } |  |  | 0.669 |
| walker |  | 8571 | 65 | Code::CodeKey { rung: Decl, file: krep.c, decl: 88, sub: 0, line: 5104 } |  |  | 0.671 |
| walker |  | 8636 | 65 | Code::CodeKey { rung: Decl, file: krep.c, decl: 87, sub: 0, line: 4872 } |  |  | 0.673 |
| ns | 8696 |  | 234 | krep.c global option state and the lower_table constructor | 6.2 |  | 0.673 |
| walker |  | 8701 | 65 | Code::CodeKey { rung: Decl, file: krep.c, decl: 86, sub: 0, line: 4699 } |  |  | 0.675 |
| walker |  | 8712 | 11 | Code::CodeKey { rung: Doc, file: krep.c, decl: 83, sub: 0, line: 4332 } |  |  | 0.675 |
| walker |  | 8725 | 13 | Code::CodeKey { rung: Doc, file: krep.c, decl: 82, sub: 0, line: 4313 } |  |  | 0.675 |
| walker |  | 8739 | 14 | Code::CodeKey { rung: Doc, file: krep.c, decl: 80, sub: 0, line: 4209 } |  |  | 0.675 |
| walker |  | 8754 | 15 | Code::CodeKey { rung: Doc, file: krep.c, decl: 81, sub: 0, line: 4251 } |  |  | 0.675 |
| walker |  | 8772 | 18 | Code::CodeKey { rung: Doc, file: krep.c, decl: 79, sub: 0, line: 4104 } |  |  | 0.675 |
| walker |  | 8793 | 21 | Code::CodeKey { rung: Doc, file: krep.c, decl: 84, sub: 0, line: 4371 } |  |  | 0.675 |
| walker |  | 8817 | 24 | Code::CodeKey { rung: Doc, file: krep.c, decl: 71, sub: 0, line: 3240 } |  |  | 0.675 |
| walker |  | 8849 | 32 | Code::CodeKey { rung: Doc, file: krep.c, decl: 44, sub: 0, line: 438 } |  |  | 0.675 |
| walker |  | 8882 | 33 | Code::CodeKey { rung: Doc, file: krep.c, decl: 86, sub: 0, line: 4699 } |  |  | 0.675 |
| walker |  | 8917 | 35 | Code::CodeKey { rung: Doc, file: krep.c, decl: 75, sub: 0, line: 3442 } |  |  | 0.675 |
| walker |  | 8953 | 36 | Code::CodeKey { rung: Doc, file: krep.c, decl: 51, sub: 0, line: 1259 } |  |  | 0.675 |
| ns | 8966 |  | 270 | main: the getopt_long table and short-option string | 6.3 |  | 0.667 |
| walker |  | 8990 | 37 | Code::CodeKey { rung: Doc, file: krep.c, decl: 46, sub: 0, line: 1084 } |  |  | 0.667 |
| walker |  | 9029 | 39 | Code::CodeKey { rung: Doc, file: krep.c, decl: 88, sub: 0, line: 5104 } |  |  | 0.667 |
| walker |  | 9069 | 40 | Code::CodeKey { rung: Doc, file: krep.c, decl: 41, sub: 0, line: 363 } |  |  | 0.667 |
| ns | 9299 |  | 333 | select_search_algorithm: dispatch head through the short-pattern branch | 6.4 |  | 0.653 |
| walker |  | 9315 | 246 | Code::CodeKey { rung: Names, file: aho_corasick.c, decl: 0, sub: 0, line: 0 } |  |  | 0.664 |
| walker |  | 9320 | 5 | Code::CodeKey { rung: Decl, file: aho_corasick.c, decl: 3, sub: 0, line: 34 } |  |  | 0.664 |
| walker |  | 9325 | 5 | Code::CodeKey { rung: Decl, file: aho_corasick.c, decl: 4, sub: 0, line: 55 } |  |  | 0.664 |
| walker |  | 9330 | 5 | Code::CodeKey { rung: Decl, file: aho_corasick.c, decl: 5, sub: 0, line: 86 } |  |  | 0.664 |
| walker |  | 9335 | 5 | Code::CodeKey { rung: Decl, file: aho_corasick.c, decl: 6, sub: 0, line: 111 } |  |  | 0.664 |
| walker |  | 9340 | 5 | Code::CodeKey { rung: Decl, file: aho_corasick.c, decl: 7, sub: 0, line: 274 } |  |  | 0.664 |
| walker |  | 9345 | 5 | Code::CodeKey { rung: Decl, file: aho_corasick.c, decl: 8, sub: 0, line: 287 } |  |  | 0.664 |
| walker |  | 9382 | 37 | Code::CodeKey { rung: Decl, file: aho_corasick.c, decl: 12, sub: 0, line: 299 } |  |  | 0.664 |
| walker |  | 9445 | 63 | Code::CodeKey { rung: Decl, file: aho_corasick.c, decl: 2, sub: 0, line: 26 } |  |  | 0.666 |
| walker |  | 9547 | 102 | Code::CodeKey { rung: Decl, file: aho_corasick.c, decl: 1, sub: 0, line: 17 } |  |  | 0.674 |
| ns | 9685 |  | 386 | select_search_algorithm: SIMD length limits and the KMP/Boyer-Moore fallback | 6.5 | 6.4 | 0.658 |
| walker |  | 9731 | 184 | Code::CodeKey { rung: Names, file: aho_corasick.h, decl: 0, sub: 0, line: 0 } |  |  | 0.664 |
| walker |  | 9741 | 10 | Code::CodeKey { rung: Doc, file: aho_corasick.h, decl: 5, sub: 0, line: 23 } |  |  | 0.664 |
| walker |  | 9753 | 12 | Code::CodeKey { rung: Doc, file: aho_corasick.h, decl: 9, sub: 0, line: 33 } |  |  | 0.665 |
| walker |  | 9765 | 12 | Code::CodeKey { rung: Doc, file: aho_corasick.h, decl: 11, sub: 0, line: 39 } |  |  | 0.668 |
| walker |  | 9779 | 14 | Code::CodeKey { rung: Doc, file: aho_corasick.h, decl: 6, sub: 0, line: 26 } |  |  | 0.668 |
| walker |  | 9794 | 15 | Code::CodeKey { rung: Doc, file: aho_corasick.h, decl: 8, sub: 0, line: 30 } |  |  | 0.670 |
| walker |  | 9812 | 18 | Code::CodeKey { rung: Doc, file: aho_corasick.h, decl: 1, sub: 0, line: 14 } |  |  | 0.670 |
| walker |  | 9832 | 20 | Code::CodeKey { rung: Doc, file: aho_corasick.h, decl: 10, sub: 0, line: 36 } |  |  | 0.674 |
| walker |  | 9867 | 35 | Code::CodeKey { rung: Doc, file: aho_corasick.h, decl: 3, sub: 0, line: 19 } |  |  | 0.674 |
| ns | 9868 |  | 183 | gitignore data model: pattern record and parent-chained context | 6.6 |  | 0.679 |
| walker |  | 9876 | 9 | Code::CodeKey { rung: Doc, file: aho_corasick.c, decl: 3, sub: 0, line: 34 } |  |  | 0.679 |
| walker |  | 9888 | 12 | Code::CodeKey { rung: Doc, file: aho_corasick.c, decl: 4, sub: 0, line: 55 } |  |  | 0.679 |
| walker |  | 9901 | 13 | Code::CodeKey { rung: Doc, file: aho_corasick.c, decl: 12, sub: 0, line: 299 } |  |  | 0.679 |
| walker |  | 9915 | 14 | Code::CodeKey { rung: Doc, file: aho_corasick.c, decl: 6, sub: 0, line: 111 } |  |  | 0.679 |
| walker |  | 9929 | 14 | Code::CodeKey { rung: Doc, file: aho_corasick.c, decl: 7, sub: 0, line: 274 } |  |  | 0.679 |
| walker |  | 9943 | 14 | Code::CodeKey { rung: Doc, file: aho_corasick.c, decl: 11, sub: 0, line: 296 } |  |  | 0.679 |
| walker |  | 9958 | 15 | Code::CodeKey { rung: Doc, file: aho_corasick.c, decl: 5, sub: 0, line: 86 } |  |  | 0.679 |
| ns | 9993 |  | 125 | Licence header, dependabot config and .gitignore | 7.1 |  | 0.672 |
