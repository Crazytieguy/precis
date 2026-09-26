Score(3000)=0.698 I=0.849 C=0.573 ns_rows≤3K=20/56 grid(1000/1442/2080/3000/4327/6240/9000)=0.651/0.698/0.679/0.698/0.807/0.738/0.667

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
| walker |  | 3955 | 164 | Code::CodeKey { rung: Names, file: krep.h, decl: 0, sub: 5, line: 0 } |  |  | 0.756 |
| walker |  | 3960 | 5 | Code::CodeKey { rung: Decl, file: krep.h, decl: 39, sub: 0, line: 214 } |  |  | 0.759 |
| walker |  | 3965 | 5 | Code::CodeKey { rung: Decl, file: krep.h, decl: 40, sub: 0, line: 218 } |  |  | 0.762 |
| walker |  | 3972 | 7 | Code::CodeKey { rung: Decl, file: krep.h, decl: 41, sub: 0, line: 222 } |  |  | 0.765 |
| walker |  | 3986 | 14 | Code::CodeKey { rung: Doc, file: krep.h, decl: 39, sub: 0, line: 214 } |  |  | 0.775 |
| ns | 4011 |  | 185 | skip_extensions: head, ellipsis, and the count line | 3.8 |  | 0.761 |
| walker |  | 4220 | 234 | Code::CodeKey { rung: Names, file: krep.h, decl: 0, sub: 6, line: 0 } |  |  | 0.791 |
| walker |  | 4225 | 5 | Code::CodeKey { rung: Decl, file: krep.h, decl: 49, sub: 0, line: 298 } |  |  | 0.791 |
| walker |  | 4230 | 5 | Code::CodeKey { rung: Decl, file: krep.h, decl: 50, sub: 0, line: 312 } |  |  | 0.791 |
| ns | 4236 |  | 225 | ANSI colour macro block, output palette and help palette | 3.9 |  | 0.795 |
| walker |  | 4237 | 7 | Code::CodeKey { rung: Decl, file: krep.h, decl: 42, sub: 0, line: 226 } |  |  | 0.798 |
| walker |  | 4245 | 8 | Code::CodeKey { rung: Doc, file: krep.h, decl: 43, sub: 0, line: 231 } |  |  | 0.798 |
| walker |  | 4282 | 37 | Code::CodeKey { rung: Doc, file: krep.h, decl: 17, sub: 0, line: 98 } |  |  | 0.807 |
| ns | 4395 |  | 159 | Documented contracts of the three entry points and the printer | 3.10 | 1.7 | 0.792 |
| walker |  | 4472 | 190 | Markdown::Section { file: README.md, section_index: 4, keeps_default_concavity: false } |  |  | 0.805 |
| walker |  | 4540 | 68 | Code::CodeKey { rung: Doc, file: krep.h, decl: 49, sub: 0, line: 298 } |  |  | 0.805 |
| ns | 4634 |  | 239 | krep.c file-level section banner map, all sixteen top-level banners | 4.1 |  | 0.781 |
| walker |  | 4725 | 185 | Markdown::Section { file: README.md, section_index: 9, keeps_default_concavity: false } |  |  | 0.800 |
| ns | 4758 |  | 124 | krep.c definition roster, lines 139-461: match results, line finding, printing | 4.2 |  | 0.786 |
| walker |  | 4923 | 198 | Markdown::Section { file: README.md, section_index: 10, keeps_default_concavity: false } |  |  | 0.814 |
| ns | 4932 |  | 174 | krep.c definition roster, lines 1084-1964: utilities, the four scalar algorithms, orchestration | 4.3 |  | 0.795 |
| walker |  | 5052 | 129 | Markdown::Section { file: README.md, section_index: 11, keeps_default_concavity: false } |  |  | 0.812 |
| ns | 5124 |  | 192 | krep.c definition roster, lines 1999-3442: public API bodies, skip filters, gitignore | 4.4 |  | 0.793 |
| walker |  | 5250 | 198 | Code::CodeKey { rung: Names, file: krep.c, decl: 0, sub: 0, line: 0 } |  |  | 0.793 |
| walker |  | 5257 | 7 | Code::CodeKey { rung: Decl, file: krep.c, decl: 1, sub: 0, line: 9 } |  |  | 0.793 |
| walker |  | 5264 | 7 | Code::CodeKey { rung: Doc, file: krep.c, decl: 5, sub: 0, line: 77 } |  |  | 0.793 |
| walker |  | 5275 | 11 | Code::CodeKey { rung: Doc, file: krep.c, decl: 3, sub: 0, line: 42 } |  |  | 0.793 |
| walker |  | 5289 | 14 | Code::CodeKey { rung: Doc, file: krep.c, decl: 4, sub: 0, line: 44 } |  |  | 0.793 |
| walker |  | 5305 | 16 | Code::CodeKey { rung: Doc, file: krep.c, decl: 2, sub: 0, line: 39 } |  |  | 0.793 |
| walker |  | 5326 | 21 | Code::CodeKey { rung: Doc, file: krep.c, decl: 1, sub: 0, line: 9 } |  |  | 0.793 |
| ns | 5363 |  | 239 | krep.c definition roster, lines 3451-5108: main, thread pool, and the guarded SIMD kernels | 4.5 |  | 0.772 |
| walker |  | 5528 | 202 | Code::CodeKey { rung: Names, file: krep.c, decl: 0, sub: 1, line: 0 } |  |  | 0.773 |
| walker |  | 5533 | 5 | Code::CodeKey { rung: Decl, file: krep.c, decl: 12, sub: 0, line: 84 } |  |  | 0.773 |
| walker |  | 5542 | 9 | Code::CodeKey { rung: Doc, file: krep.c, decl: 17, sub: 0, line: 93 } |  |  | 0.773 |
| ns | 5546 |  | 183 | search_file internal section map, all eleven banners inside lines 2274-3070 | 4.6 |  | 0.760 |
| ns | 5668 |  | 122 | print_matching_items internal section map: the -o and full-line output modes | 4.7 |  | 0.751 |
| walker |  | 5745 | 203 | Code::CodeKey { rung: Names, file: krep.c, decl: 0, sub: 2, line: 0 } |  |  | 0.751 |
| walker |  | 5752 | 7 | Code::CodeKey { rung: Decl, file: krep.c, decl: 26, sub: 0, line: 111 } |  |  | 0.751 |
| walker |  | 5765 | 13 | Code::CodeKey { rung: Doc, file: krep.c, decl: 23, sub: 0, line: 104 } |  |  | 0.751 |
| walker |  | 5779 | 14 | Code::CodeKey { rung: Doc, file: krep.c, decl: 22, sub: 0, line: 101 } |  |  | 0.751 |
| walker |  | 5793 | 14 | Code::CodeKey { rung: Doc, file: krep.c, decl: 27, sub: 0, line: 116 } |  |  | 0.751 |
| ns | 5822 |  | 154 | aho_corasick.c: complete definition roster | 4.8 |  | 0.741 |
| walker |  | 5985 | 192 | Code::CodeKey { rung: Names, file: krep.c, decl: 0, sub: 3, line: 0 } |  |  | 0.742 |
| walker |  | 5990 | 5 | Code::CodeKey { rung: Decl, file: krep.c, decl: 35, sub: 0, line: 128 } |  |  | 0.742 |
| walker |  | 5995 | 5 | Code::CodeKey { rung: Decl, file: krep.c, decl: 36, sub: 0, line: 139 } |  |  | 0.742 |
| walker |  | 6000 | 5 | Code::CodeKey { rung: Decl, file: krep.c, decl: 37, sub: 0, line: 175 } |  |  | 0.742 |
| walker |  | 6010 | 10 | Code::CodeKey { rung: Doc, file: krep.c, decl: 36, sub: 0, line: 139 } |  |  | 0.742 |
| ns | 6014 |  | 192 | aho_corasick.c trie data model: ac_node_t and struct ac_trie | 4.9 |  | 0.725 |
| walker |  | 6021 | 11 | Code::CodeKey { rung: Doc, file: krep.c, decl: 35, sub: 0, line: 128 } |  |  | 0.725 |
| walker |  | 6033 | 12 | Code::CodeKey { rung: Doc, file: krep.c, decl: 34, sub: 0, line: 125 } |  |  | 0.725 |
| walker |  | 6051 | 18 | Code::CodeKey { rung: Doc, file: krep.c, decl: 37, sub: 0, line: 175 } |  |  | 0.725 |
| walker |  | 6234 | 183 | Code::CodeKey { rung: Names, file: krep.c, decl: 0, sub: 4, line: 0 } |  |  | 0.738 |
| walker |  | 6239 | 5 | Code::CodeKey { rung: Decl, file: krep.c, decl: 38, sub: 0, line: 244 } |  |  | 0.738 |
| walker |  | 6244 | 5 | Code::CodeKey { rung: Decl, file: krep.c, decl: 39, sub: 0, line: 256 } |  |  | 0.738 |
| walker |  | 6249 | 5 | Code::CodeKey { rung: Decl, file: krep.c, decl: 41, sub: 0, line: 363 } |  |  | 0.738 |
| walker |  | 6254 | 5 | Code::CodeKey { rung: Decl, file: krep.c, decl: 42, sub: 0, line: 401 } |  |  | 0.738 |
| walker |  | 6259 | 5 | Code::CodeKey { rung: Decl, file: krep.c, decl: 43, sub: 0, line: 420 } |  |  | 0.738 |
| walker |  | 6264 | 5 | Code::CodeKey { rung: Decl, file: krep.c, decl: 44, sub: 0, line: 438 } |  |  | 0.738 |
| ns | 6272 |  | 258 | test/test_krep.c: complete roster of test functions, helpers and main | 4.10 |  | 0.718 |
| walker |  | 6301 | 37 | Code::CodeKey { rung: Decl, file: krep.c, decl: 40, sub: 0, line: 329 } |  |  | 0.718 |
| walker |  | 6314 | 13 | Code::CodeKey { rung: Doc, file: krep.c, decl: 38, sub: 0, line: 244 } |  |  | 0.718 |
| walker |  | 6330 | 16 | Code::CodeKey { rung: Doc, file: krep.c, decl: 42, sub: 0, line: 401 } |  |  | 0.718 |
| walker |  | 6348 | 18 | Code::CodeKey { rung: Doc, file: krep.c, decl: 43, sub: 0, line: 420 } |  |  | 0.718 |
| ns | 6497 |  | 225 | test/test_regex.c and test/test_multiple_patterns.c rosters | 4.11 |  | 0.702 |
| walker |  | 6514 | 166 | Code::CodeKey { rung: Names, file: krep.c, decl: 0, sub: 5, line: 0 } |  |  | 0.708 |
| walker |  | 6519 | 5 | Code::CodeKey { rung: Decl, file: krep.c, decl: 45, sub: 0, line: 461 } |  |  | 0.708 |
| walker |  | 6525 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 46, sub: 0, line: 1084 } |  |  | 0.708 |
| walker |  | 6531 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 47, sub: 0, line: 1125 } |  |  | 0.708 |
| walker |  | 6537 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 48, sub: 0, line: 1137 } |  |  | 0.708 |
| walker |  | 6543 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 49, sub: 0, line: 1198 } |  |  | 0.708 |
| walker |  | 6549 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 50, sub: 0, line: 1213 } |  |  | 0.708 |
| walker |  | 6559 | 10 | Code::CodeKey { rung: Doc, file: krep.c, decl: 48, sub: 0, line: 1137 } |  |  | 0.708 |
| walker |  | 6570 | 11 | Code::CodeKey { rung: Doc, file: krep.c, decl: 47, sub: 0, line: 1125 } |  |  | 0.708 |
| walker |  | 6585 | 15 | Code::CodeKey { rung: Doc, file: krep.c, decl: 50, sub: 0, line: 1213 } |  |  | 0.708 |
| walker |  | 6602 | 17 | Code::CodeKey { rung: Doc, file: krep.c, decl: 49, sub: 0, line: 1198 } |  |  | 0.708 |
| ns | 6608 |  | 111 | test/test_directory.c roster: the separate recursive-search integration binary | 4.12 |  | 0.700 |
| walker |  | 6622 | 20 | Code::CodeKey { rung: Doc, file: krep.c, decl: 40, sub: 0, line: 329 } |  |  | 0.700 |
| walker |  | 6825 | 203 | Code::CodeKey { rung: Names, file: krep.c, decl: 0, sub: 6, line: 0 } |  |  | 0.717 |
| walker |  | 6831 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 53, sub: 0, line: 1585 } |  |  | 0.717 |
| walker |  | 6837 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 55, sub: 0, line: 1771 } |  |  | 0.717 |
| ns | 6841 |  | 233 | test/test_compat.h: the TESTING wrapper roster and its guards | 4.13 |  | 0.705 |
| walker |  | 6843 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 56, sub: 0, line: 1873 } |  |  | 0.705 |
| walker |  | 6849 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 57, sub: 0, line: 1919 } |  |  | 0.705 |
| walker |  | 6855 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 58, sub: 0, line: 1964 } |  |  | 0.705 |
| walker |  | 6861 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 59, sub: 0, line: 1999 } |  |  | 0.705 |
| walker |  | 6902 | 41 | Code::CodeKey { rung: Decl, file: krep.c, decl: 52, sub: 0, line: 1389 } |  |  | 0.705 |
| walker |  | 6952 | 50 | Code::CodeKey { rung: Decl, file: krep.c, decl: 51, sub: 0, line: 1259 } |  |  | 0.705 |
| ns | 6972 |  | 131 | test/test_krep.h: shared test helper declarations | 4.14 |  | 0.695 |
| walker |  | 7002 | 50 | Code::CodeKey { rung: Decl, file: krep.c, decl: 54, sub: 0, line: 1628 } |  |  | 0.695 |
| walker |  | 7018 | 16 | Code::CodeKey { rung: Doc, file: krep.c, decl: 59, sub: 0, line: 1999 } |  |  | 0.695 |
| walker |  | 7035 | 17 | Code::CodeKey { rung: Doc, file: krep.c, decl: 58, sub: 0, line: 1964 } |  |  | 0.695 |
| walker |  | 7054 | 19 | Code::CodeKey { rung: Doc, file: krep.c, decl: 56, sub: 0, line: 1873 } |  |  | 0.695 |
| walker |  | 7073 | 19 | Code::CodeKey { rung: Doc, file: krep.c, decl: 57, sub: 0, line: 1919 } |  |  | 0.695 |
| ns | 7080 |  | 108 | Makefile compiler configuration: CC, CFLAGS, LDFLAGS, PREFIX | 5.1 |  | 0.690 |
| walker |  | 7298 | 225 | Code::CodeKey { rung: Names, file: krep.c, decl: 0, sub: 7, line: 0 } |  |  | 0.698 |
| walker |  | 7304 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 61, sub: 0, line: 2252 } |  |  | 0.698 |
| walker |  | 7310 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 62, sub: 0, line: 2265 } |  |  | 0.698 |
| walker |  | 7316 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 63, sub: 0, line: 2274 } |  |  | 0.680 |
| ns | 7316 |  | 236 | .github/workflows/ci.yml in full | 5.2 |  | 0.680 |
| walker |  | 7322 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 64, sub: 0, line: 3071 } |  |  | 0.680 |
| walker |  | 7328 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 65, sub: 0, line: 3090 } |  |  | 0.680 |
| walker |  | 7334 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 66, sub: 0, line: 3122 } |  |  | 0.680 |
| walker |  | 7340 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 69, sub: 0, line: 3163 } |  |  | 0.680 |
| walker |  | 7346 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 70, sub: 0, line: 3180 } |  |  | 0.680 |
| walker |  | 7380 | 34 | Code::CodeKey { rung: Decl, file: krep.c, decl: 67, sub: 0, line: 3146 } |  |  | 0.680 |
| walker |  | 7437 | 57 | Code::CodeKey { rung: Decl, file: krep.c, decl: 68, sub: 0, line: 3154 } |  |  | 0.680 |
| walker |  | 7447 | 10 | Code::CodeKey { rung: Doc, file: krep.c, decl: 60, sub: 0, line: 2249 } |  |  | 0.680 |
| walker |  | 7460 | 13 | Code::CodeKey { rung: Doc, file: krep.c, decl: 62, sub: 0, line: 2265 } |  |  | 0.680 |
| walker |  | 7474 | 14 | Code::CodeKey { rung: Doc, file: krep.c, decl: 69, sub: 0, line: 3163 } |  |  | 0.680 |
| walker |  | 7489 | 15 | Code::CodeKey { rung: Doc, file: krep.c, decl: 64, sub: 0, line: 3071 } |  |  | 0.680 |
| walker |  | 7504 | 15 | Code::CodeKey { rung: Doc, file: krep.c, decl: 65, sub: 0, line: 3090 } |  |  | 0.680 |
| walker |  | 7519 | 15 | Code::CodeKey { rung: Doc, file: krep.c, decl: 66, sub: 0, line: 3122 } |  |  | 0.680 |
| walker |  | 7534 | 15 | Code::CodeKey { rung: Doc, file: krep.c, decl: 67, sub: 0, line: 3146 } |  |  | 0.681 |
| walker |  | 7550 | 16 | Code::CodeKey { rung: Doc, file: krep.c, decl: 61, sub: 0, line: 2252 } |  |  | 0.681 |
| walker |  | 7568 | 18 | Code::CodeKey { rung: Doc, file: krep.c, decl: 70, sub: 0, line: 3180 } |  |  | 0.681 |
| ns | 7580 |  | 264 | Makefile architecture detection: which SIMD flags each arch gets | 5.3 |  | 0.667 |
| walker |  | 7588 | 20 | Code::CodeKey { rung: Doc, file: krep.c, decl: 68, sub: 0, line: 3154 } |  |  | 0.667 |
| walker |  | 7765 | 177 | Code::CodeKey { rung: Names, file: krep.c, decl: 0, sub: 8, line: 0 } |  |  | 0.679 |
| walker |  | 7771 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 71, sub: 0, line: 3240 } |  |  | 0.679 |
| walker |  | 7777 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 72, sub: 0, line: 3272 } |  |  | 0.679 |
| walker |  | 7783 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 73, sub: 0, line: 3297 } |  |  | 0.679 |
| walker |  | 7789 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 75, sub: 0, line: 3442 } |  |  | 0.679 |
| walker |  | 7810 | 21 | Code::CodeKey { rung: Decl, file: krep.c, decl: 76, sub: 0, line: 3450 } |  |  | 0.679 |
| walker |  | 7834 | 24 | Code::CodeKey { rung: Decl, file: krep.c, decl: 74, sub: 0, line: 3310 } |  |  | 0.679 |
| walker |  | 7875 | 41 | Code::CodeKey { rung: Decl, file: krep.c, decl: 77, sub: 0, line: 3891 } |  |  | 0.679 |
| ns | 7883 |  | 303 | Makefile compile/link rules and the parallel -DTESTING build | 5.4 |  | 0.668 |
| walker |  | 7886 | 11 | Code::CodeKey { rung: Doc, file: krep.c, decl: 77, sub: 0, line: 3891 } |  |  | 0.668 |
| walker |  | 7901 | 15 | Code::CodeKey { rung: Doc, file: krep.c, decl: 74, sub: 0, line: 3310 } |  |  | 0.668 |
| walker |  | 7921 | 20 | Code::CodeKey { rung: Doc, file: krep.c, decl: 72, sub: 0, line: 3272 } |  |  | 0.668 |
| walker |  | 7942 | 21 | Code::CodeKey { rung: Doc, file: krep.c, decl: 73, sub: 0, line: 3297 } |  |  | 0.668 |
| walker |  | 7963 | 21 | Code::CodeKey { rung: Doc, file: krep.c, decl: 76, sub: 0, line: 3450 } |  |  | 0.668 |
| ns | 8052 |  | 169 | Makefile run targets: test, test-directory, ci, bench-rg, all-tests | 5.5 |  | 0.658 |
| ns | 8209 |  | 157 | .github/workflows/release.yml: tag trigger and release artifacts | 5.6 |  | 0.646 |
| walker |  | 8273 | 310 | Code::CodeKey { rung: Names, file: krep.c, decl: 0, sub: 9, line: 0 } |  |  | 0.668 |
| walker |  | 8279 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 78, sub: 0, line: 4046 } |  |  | 0.668 |
| walker |  | 8285 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 79, sub: 0, line: 4104 } |  |  | 0.668 |
| walker |  | 8291 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 80, sub: 0, line: 4209 } |  |  | 0.668 |
| walker |  | 8297 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 81, sub: 0, line: 4251 } |  |  | 0.668 |
| walker |  | 8303 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 82, sub: 0, line: 4313 } |  |  | 0.668 |
| walker |  | 8309 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 83, sub: 0, line: 4332 } |  |  | 0.668 |
| walker |  | 8350 | 41 | Code::CodeKey { rung: Decl, file: krep.c, decl: 84, sub: 0, line: 4371 } |  |  | 0.668 |
| walker |  | 8399 | 49 | Code::CodeKey { rung: Decl, file: krep.c, decl: 86, sub: 0, line: 4699 } |  |  | 0.668 |
| walker |  | 8448 | 49 | Code::CodeKey { rung: Decl, file: krep.c, decl: 87, sub: 0, line: 4872 } |  |  | 0.668 |
| ns | 8462 |  | 253 | krep.c constants: VERSION and every performance tunable | 6.1 |  | 0.675 |
| walker |  | 8497 | 49 | Code::CodeKey { rung: Decl, file: krep.c, decl: 88, sub: 0, line: 5104 } |  |  | 0.675 |
| walker |  | 8548 | 51 | Code::CodeKey { rung: Decl, file: krep.c, decl: 85, sub: 0, line: 4505 } |  |  | 0.675 |
| walker |  | 8559 | 11 | Code::CodeKey { rung: Doc, file: krep.c, decl: 83, sub: 0, line: 4332 } |  |  | 0.675 |
| walker |  | 8572 | 13 | Code::CodeKey { rung: Doc, file: krep.c, decl: 82, sub: 0, line: 4313 } |  |  | 0.675 |
| walker |  | 8586 | 14 | Code::CodeKey { rung: Doc, file: krep.c, decl: 80, sub: 0, line: 4209 } |  |  | 0.675 |
| walker |  | 8601 | 15 | Code::CodeKey { rung: Doc, file: krep.c, decl: 81, sub: 0, line: 4251 } |  |  | 0.675 |
| walker |  | 8617 | 16 | Code::CodeKey { rung: Doc, file: krep.c, decl: 78, sub: 0, line: 4046 } |  |  | 0.675 |
| walker |  | 8635 | 18 | Code::CodeKey { rung: Doc, file: krep.c, decl: 79, sub: 0, line: 4104 } |  |  | 0.675 |
| walker |  | 8656 | 21 | Code::CodeKey { rung: Doc, file: krep.c, decl: 84, sub: 0, line: 4371 } |  |  | 0.675 |
| walker |  | 8680 | 24 | Code::CodeKey { rung: Doc, file: krep.c, decl: 71, sub: 0, line: 3240 } |  |  | 0.675 |
| ns | 8696 |  | 234 | krep.c global option state and the lower_table constructor | 6.2 |  | 0.674 |
| walker |  | 8712 | 32 | Code::CodeKey { rung: Doc, file: krep.c, decl: 44, sub: 0, line: 438 } |  |  | 0.674 |
| walker |  | 8745 | 33 | Code::CodeKey { rung: Doc, file: krep.c, decl: 86, sub: 0, line: 4699 } |  |  | 0.674 |
| walker |  | 8780 | 35 | Code::CodeKey { rung: Doc, file: krep.c, decl: 75, sub: 0, line: 3442 } |  |  | 0.674 |
| walker |  | 8816 | 36 | Code::CodeKey { rung: Doc, file: krep.c, decl: 51, sub: 0, line: 1259 } |  |  | 0.674 |
| walker |  | 8853 | 37 | Code::CodeKey { rung: Doc, file: krep.c, decl: 46, sub: 0, line: 1084 } |  |  | 0.674 |
| walker |  | 8892 | 39 | Code::CodeKey { rung: Doc, file: krep.c, decl: 88, sub: 0, line: 5104 } |  |  | 0.674 |
| walker |  | 8932 | 40 | Code::CodeKey { rung: Doc, file: krep.c, decl: 41, sub: 0, line: 363 } |  |  | 0.674 |
| ns | 8966 |  | 270 | main: the getopt_long table and short-option string | 6.3 |  | 0.666 |
| walker |  | 9178 | 246 | Code::CodeKey { rung: Names, file: aho_corasick.c, decl: 0, sub: 0, line: 0 } |  |  | 0.677 |
| walker |  | 9183 | 5 | Code::CodeKey { rung: Decl, file: aho_corasick.c, decl: 3, sub: 0, line: 34 } |  |  | 0.677 |
| walker |  | 9188 | 5 | Code::CodeKey { rung: Decl, file: aho_corasick.c, decl: 4, sub: 0, line: 55 } |  |  | 0.677 |
| walker |  | 9193 | 5 | Code::CodeKey { rung: Decl, file: aho_corasick.c, decl: 5, sub: 0, line: 86 } |  |  | 0.677 |
| walker |  | 9198 | 5 | Code::CodeKey { rung: Decl, file: aho_corasick.c, decl: 6, sub: 0, line: 111 } |  |  | 0.677 |
| walker |  | 9203 | 5 | Code::CodeKey { rung: Decl, file: aho_corasick.c, decl: 7, sub: 0, line: 274 } |  |  | 0.677 |
| walker |  | 9208 | 5 | Code::CodeKey { rung: Decl, file: aho_corasick.c, decl: 8, sub: 0, line: 287 } |  |  | 0.677 |
| walker |  | 9245 | 37 | Code::CodeKey { rung: Decl, file: aho_corasick.c, decl: 12, sub: 0, line: 299 } |  |  | 0.677 |
| ns | 9299 |  | 333 | select_search_algorithm: dispatch head through the short-pattern branch | 6.4 |  | 0.663 |
| walker |  | 9308 | 63 | Code::CodeKey { rung: Decl, file: aho_corasick.c, decl: 2, sub: 0, line: 26 } |  |  | 0.665 |
| walker |  | 9410 | 102 | Code::CodeKey { rung: Decl, file: aho_corasick.c, decl: 1, sub: 0, line: 17 } |  |  | 0.673 |
| walker |  | 9594 | 184 | Code::CodeKey { rung: Names, file: aho_corasick.h, decl: 0, sub: 0, line: 0 } |  |  | 0.679 |
| walker |  | 9604 | 10 | Code::CodeKey { rung: Doc, file: aho_corasick.h, decl: 5, sub: 0, line: 23 } |  |  | 0.679 |
| walker |  | 9616 | 12 | Code::CodeKey { rung: Doc, file: aho_corasick.h, decl: 9, sub: 0, line: 33 } |  |  | 0.681 |
| walker |  | 9628 | 12 | Code::CodeKey { rung: Doc, file: aho_corasick.h, decl: 11, sub: 0, line: 39 } |  |  | 0.683 |
| walker |  | 9642 | 14 | Code::CodeKey { rung: Doc, file: aho_corasick.h, decl: 6, sub: 0, line: 26 } |  |  | 0.683 |
| walker |  | 9657 | 15 | Code::CodeKey { rung: Doc, file: aho_corasick.h, decl: 8, sub: 0, line: 30 } |  |  | 0.686 |
| walker |  | 9675 | 18 | Code::CodeKey { rung: Doc, file: aho_corasick.h, decl: 1, sub: 0, line: 14 } |  |  | 0.686 |
| ns | 9685 |  | 386 | select_search_algorithm: SIMD length limits and the KMP/Boyer-Moore fallback | 6.5 | 6.4 | 0.670 |
| walker |  | 9695 | 20 | Code::CodeKey { rung: Doc, file: aho_corasick.h, decl: 10, sub: 0, line: 36 } |  |  | 0.673 |
| walker |  | 9730 | 35 | Code::CodeKey { rung: Doc, file: aho_corasick.h, decl: 3, sub: 0, line: 19 } |  |  | 0.673 |
| walker |  | 9739 | 9 | Code::CodeKey { rung: Doc, file: aho_corasick.c, decl: 3, sub: 0, line: 34 } |  |  | 0.673 |
| walker |  | 9751 | 12 | Code::CodeKey { rung: Doc, file: aho_corasick.c, decl: 4, sub: 0, line: 55 } |  |  | 0.673 |
| walker |  | 9764 | 13 | Code::CodeKey { rung: Doc, file: aho_corasick.c, decl: 12, sub: 0, line: 299 } |  |  | 0.673 |
| walker |  | 9778 | 14 | Code::CodeKey { rung: Doc, file: aho_corasick.c, decl: 6, sub: 0, line: 111 } |  |  | 0.673 |
| walker |  | 9792 | 14 | Code::CodeKey { rung: Doc, file: aho_corasick.c, decl: 7, sub: 0, line: 274 } |  |  | 0.673 |
| walker |  | 9806 | 14 | Code::CodeKey { rung: Doc, file: aho_corasick.c, decl: 11, sub: 0, line: 296 } |  |  | 0.673 |
| walker |  | 9821 | 15 | Code::CodeKey { rung: Doc, file: aho_corasick.c, decl: 5, sub: 0, line: 86 } |  |  | 0.673 |
| ns | 9868 |  | 183 | gitignore data model: pattern record and parent-chained context | 6.6 |  | 0.678 |
| walker |  | 9870 | 49 | Code::CodeKey { rung: Doc, file: krep.c, decl: 39, sub: 0, line: 256 } |  |  | 0.678 |
| walker |  | 9960 | 90 | Code::CodeKey { rung: Doc, file: krep.h, decl: 26, sub: 0, line: 170 } |  |  | 0.678 |
| walker |  | 9981 | 21 | Code::CodeKey { rung: Doc, file: aho_corasick.c, decl: 9, sub: 0, line: 293 } |  |  | 0.678 |
| ns | 9993 |  | 125 | Licence header, dependabot config and .gitignore | 7.1 |  | 0.672 |
