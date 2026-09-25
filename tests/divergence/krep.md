Score(3000)=0.608 I=0.835 C=0.443 ns_rows≤3K=20/56 grid(1000/1442/2080/3000/4327/6240/9000)=0.651/0.697/0.727/0.608/0.763/0.748/0.668

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
| walker |  | 585 | 115 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.773 |
| walker |  | 717 | 132 | Markdown::Section { file: README.md, section_index: 5, keeps_default_concavity: true } |  |  | 0.867 |
| ns | 789 |  | 230 | krep.h search-algorithm roster with the SIMD #if guards intact | 1.8 |  | 0.713 |
| ns | 996 |  | 207 | aho_corasick.h: the complete second-module interface | 1.9 |  | 0.644 |
| walker |  | 1069 | 352 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: true } |  |  | 0.655 |
| walker |  | 1133 | 64 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.655 |
| ns | 1173 |  | 177 | README Key Features, first half | 2.1 |  | 0.674 |
| ns | 1343 |  | 170 | README Key Features, second half | 2.2 |  | 0.690 |
| ns | 1534 |  | 191 | README command-line options table, first half (-i through -F) | 2.3 |  | 0.661 |
| walker |  | 1548 | 415 | Markdown::Section { file: README.md, section_index: 7, keeps_default_concavity: true } |  |  | 0.717 |
| walker |  | 1670 | 122 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.717 |
| ns | 1753 |  | 219 | README command-line options table, second half (-r through -h) | 2.4 |  | 0.732 |
| walker |  | 1854 | 184 | Code::CodeKey { rung: Names, file: aho_corasick.h, decl: 0, sub: 0, line: 0 } |  |  | 0.753 |
| walker |  | 1864 | 10 | Code::CodeKey { rung: Doc, file: aho_corasick.h, decl: 5, sub: 0, line: 23 } |  |  | 0.753 |
| walker |  | 1876 | 12 | Code::CodeKey { rung: Doc, file: aho_corasick.h, decl: 9, sub: 0, line: 33 } |  |  | 0.758 |
| walker |  | 1888 | 12 | Code::CodeKey { rung: Doc, file: aho_corasick.h, decl: 11, sub: 0, line: 39 } |  |  | 0.766 |
| walker |  | 1902 | 14 | Code::CodeKey { rung: Doc, file: aho_corasick.h, decl: 6, sub: 0, line: 26 } |  |  | 0.766 |
| walker |  | 1917 | 15 | Code::CodeKey { rung: Doc, file: aho_corasick.h, decl: 8, sub: 0, line: 30 } |  |  | 0.775 |
| walker |  | 1935 | 18 | Code::CodeKey { rung: Doc, file: aho_corasick.h, decl: 1, sub: 0, line: 14 } |  |  | 0.775 |
| walker |  | 1955 | 20 | Code::CodeKey { rung: Doc, file: aho_corasick.h, decl: 10, sub: 0, line: 36 } |  |  | 0.788 |
| ns | 1963 |  | 210 | README: the documented smart algorithm-selection policy | 2.5 |  | 0.746 |
| walker |  | 1990 | 35 | Code::CodeKey { rung: Doc, file: aho_corasick.h, decl: 3, sub: 0, line: 19 } |  |  | 0.746 |
| ns | 2069 |  | 106 | README usage examples: the six concrete invocations, command lines only | 2.6 |  | 0.727 |
| ns | 2181 |  | 112 | README: the multi-threading architecture section | 2.7 |  | 0.698 |
| walker |  | 2206 | 216 | Code::CodeKey { rung: Names, file: krep.h, decl: 0, sub: 0, line: 0 } |  |  | 0.700 |
| walker |  | 2215 | 9 | Code::CodeKey { rung: Doc, file: krep.h, decl: 10, sub: 0, line: 42 } |  |  | 0.700 |
| walker |  | 2229 | 14 | Code::CodeKey { rung: Doc, file: krep.h, decl: 1, sub: 0, line: 19 } |  |  | 0.700 |
| walker |  | 2243 | 14 | Code::CodeKey { rung: Doc, file: krep.h, decl: 2, sub: 0, line: 22 } |  |  | 0.700 |
| walker |  | 2263 | 20 | Code::CodeKey { rung: Doc, file: krep.h, decl: 4, sub: 0, line: 34 } |  |  | 0.700 |
| ns | 2287 |  | 106 | README: the recursive-search skipping rules | 2.8 |  | 0.679 |
| ns | 2402 |  | 115 | README scope note: explicitly not a grep/ripgrep replacement | 2.9 |  | 0.684 |
| ns | 2512 |  | 110 | README install and build-from-source commands | 2.10 |  | 0.659 |
| walker |  | 2541 | 278 | Code::CodeKey { rung: Names, file: krep.h, decl: 0, sub: 1, line: 0 } |  |  | 0.661 |
| walker |  | 2573 | 32 | Code::CodeKey { rung: Decl, file: krep.h, decl: 17, sub: 0, line: 98 } |  |  | 0.662 |
| walker |  | 2613 | 40 | Code::CodeKey { rung: Decl, file: krep.h, decl: 14, sub: 0, line: 49 } |  |  | 0.662 |
| walker |  | 2664 | 51 | Code::CodeKey { rung: Decl, file: krep.h, decl: 19, sub: 0, line: 125 } |  |  | 0.662 |
| walker |  | 2673 | 9 | Code::CodeKey { rung: Doc, file: krep.h, decl: 19, sub: 0, line: 125 } |  |  | 0.662 |
| walker |  | 2682 | 9 | Code::CodeKey { rung: Doc, file: krep.h, decl: 21, sub: 0, line: 146 } |  |  | 0.663 |
| walker |  | 2744 | 62 | Code::CodeKey { rung: Decl, file: krep.h, decl: 15, sub: 0, line: 55 } |  |  | 0.665 |
| walker |  | 2767 | 23 | Code::CodeKey { rung: Doc, file: krep.h, decl: 14, sub: 0, line: 49 } |  |  | 0.666 |
| ns | 2902 |  | 390 | search_params_t: the struct every search function takes | 3.1 |  | 0.607 |
| walker |  | 2928 | 161 | Code::CodeKey { rung: Decl, file: krep.h, decl: 20, sub: 0, line: 132 } |  |  | 0.607 |
| walker |  | 2965 | 37 | Code::CodeKey { rung: Doc, file: krep.h, decl: 17, sub: 0, line: 98 } |  |  | 0.608 |
| ns | 3089 |  | 187 | match_position_t and match_result_t | 3.2 |  | 0.629 |
| ns | 3180 |  | 91 | search_func_t: the algorithm function-pointer type | 3.3 |  | 0.636 |
| walker |  | 3215 | 250 | Code::CodeKey { rung: Decl, file: krep.h, decl: 18, sub: 0, line: 104 } |  |  | 0.641 |
| walker |  | 3225 | 10 | Code::CodeKey { rung: Doc, file: krep.h, decl: 18, sub: 0, line: 104 } |  |  | 0.642 |
| ns | 3282 |  | 102 | krep.h thread-pool API: all four functions in full | 3.4 |  | 0.640 |
| ns | 3562 |  | 280 | thread_data_t: per-thread search state, including the false-sharing padding | 3.5 |  | 0.661 |
| walker |  | 3593 | 368 | Code::CodeKey { rung: Decl, file: krep.h, decl: 16, sub: 0, line: 65 } |  |  | 0.743 |
| walker |  | 3606 | 13 | Code::CodeKey { rung: Doc, file: krep.h, decl: 16, sub: 0, line: 65 } |  |  | 0.743 |
| ns | 3659 |  | 97 | krep.h helper roster: line finding, word matching, table preparation | 3.6 |  | 0.729 |
| ns | 3826 |  | 167 | skip_directories: the complete recursive-search directory blocklist | 3.7 |  | 0.715 |
| walker |  | 3832 | 226 | Code::CodeKey { rung: Names, file: krep.h, decl: 0, sub: 2, line: 0 } |  |  | 0.750 |
| walker |  | 3842 | 10 | Code::CodeKey { rung: Doc, file: krep.h, decl: 28, sub: 0, line: 183 } |  |  | 0.750 |
| walker |  | 3932 | 90 | Code::CodeKey { rung: Doc, file: krep.h, decl: 26, sub: 0, line: 170 } |  |  | 0.750 |
| ns | 4011 |  | 185 | skip_extensions: head, ellipsis, and the count line | 3.8 |  | 0.738 |
| walker |  | 4129 | 197 | Code::CodeKey { rung: Names, file: krep.h, decl: 0, sub: 3, line: 0 } |  |  | 0.748 |
| walker |  | 4136 | 7 | Code::CodeKey { rung: Doc, file: krep.h, decl: 33, sub: 0, line: 203 } |  |  | 0.748 |
| ns | 4236 |  | 225 | ANSI colour macro block, output palette and help palette | 3.9 |  | 0.753 |
| walker |  | 4318 | 182 | Code::CodeKey { rung: Names, file: krep.h, decl: 0, sub: 4, line: 0 } |  |  | 0.763 |
| walker |  | 4337 | 19 | Code::CodeKey { rung: Decl, file: krep.h, decl: 40, sub: 0, line: 218 } |  |  | 0.768 |
| walker |  | 4356 | 19 | Code::CodeKey { rung: Decl, file: krep.h, decl: 39, sub: 0, line: 214 } |  |  | 0.775 |
| walker |  | 4375 | 19 | Code::CodeKey { rung: Decl, file: krep.h, decl: 42, sub: 0, line: 226 } |  |  | 0.782 |
| ns | 4395 |  | 159 | Documented contracts of the three entry points and the printer | 3.10 | 1.7 | 0.768 |
| walker |  | 4399 | 24 | Code::CodeKey { rung: Decl, file: krep.h, decl: 41, sub: 0, line: 222 } |  |  | 0.779 |
| walker |  | 4407 | 8 | Code::CodeKey { rung: Doc, file: krep.h, decl: 43, sub: 0, line: 231 } |  |  | 0.779 |
| walker |  | 4503 | 96 | Code::CodeKey { rung: Doc, file: krep.h, decl: 27, sub: 0, line: 180 } |  |  | 0.780 |
| ns | 4634 |  | 239 | krep.c file-level section banner map, all sixteen top-level banners | 4.1 |  | 0.756 |
| walker |  | 4662 | 159 | Code::CodeKey { rung: Names, file: krep.h, decl: 0, sub: 5, line: 0 } |  |  | 0.773 |
| walker |  | 4667 | 5 | Code::CodeKey { rung: Decl, file: krep.h, decl: 49, sub: 0, line: 298 } |  |  | 0.773 |
| walker |  | 4672 | 5 | Code::CodeKey { rung: Decl, file: krep.h, decl: 50, sub: 0, line: 312 } |  |  | 0.773 |
| walker |  | 4740 | 68 | Code::CodeKey { rung: Doc, file: krep.h, decl: 49, sub: 0, line: 298 } |  |  | 0.773 |
| ns | 4758 |  | 124 | krep.c definition roster, lines 139-461: match results, line finding, printing | 4.2 |  | 0.759 |
| walker |  | 4837 | 97 | Code::CodeKey { rung: Doc, file: krep.h, decl: 47, sub: 0, line: 278 } |  |  | 0.759 |
| ns | 4932 |  | 174 | krep.c definition roster, lines 1084-1964: utilities, the four scalar algorithms, orchestration | 4.3 |  | 0.742 |
| walker |  | 4939 | 102 | Code::CodeKey { rung: Doc, file: krep.h, decl: 48, sub: 0, line: 288 } |  |  | 0.742 |
| walker |  | 5046 | 107 | Code::CodeKey { rung: Doc, file: krep.h, decl: 50, sub: 0, line: 312 } |  |  | 0.742 |
| ns | 5124 |  | 192 | krep.c definition roster, lines 1999-3442: public API bodies, skip filters, gitignore | 4.4 |  | 0.724 |
| walker |  | 5162 | 116 | Code::CodeKey { rung: Doc, file: krep.h, decl: 25, sub: 0, line: 161 } |  |  | 0.732 |
| walker |  | 5331 | 169 | Code::CodeKey { rung: Doc, file: krep.h, decl: 32, sub: 0, line: 200 } |  |  | 0.741 |
| ns | 5363 |  | 239 | krep.c definition roster, lines 3451-5108: main, thread pool, and the guarded SIMD kernels | 4.5 |  | 0.720 |
| walker |  | 5516 | 185 | Markdown::Section { file: README.md, section_index: 9, keeps_default_concavity: false } |  |  | 0.738 |
| ns | 5546 |  | 183 | search_file internal section map, all eleven banners inside lines 2274-3070 | 4.6 |  | 0.726 |
| ns | 5668 |  | 122 | print_matching_items internal section map: the -o and full-line output modes | 4.7 |  | 0.717 |
| walker |  | 5714 | 198 | Markdown::Section { file: README.md, section_index: 10, keeps_default_concavity: false } |  |  | 0.742 |
| ns | 5822 |  | 154 | aho_corasick.c: complete definition roster | 4.8 |  | 0.732 |
| walker |  | 5843 | 129 | Markdown::Section { file: README.md, section_index: 11, keeps_default_concavity: false } |  |  | 0.748 |
| ns | 6014 |  | 192 | aho_corasick.c trie data model: ac_node_t and struct ac_trie | 4.9 |  | 0.730 |
| walker |  | 6089 | 246 | Code::CodeKey { rung: Names, file: aho_corasick.c, decl: 0, sub: 0, line: 0 } |  |  | 0.745 |
| walker |  | 6094 | 5 | Code::CodeKey { rung: Decl, file: aho_corasick.c, decl: 3, sub: 0, line: 34 } |  |  | 0.745 |
| walker |  | 6099 | 5 | Code::CodeKey { rung: Decl, file: aho_corasick.c, decl: 4, sub: 0, line: 55 } |  |  | 0.745 |
| walker |  | 6104 | 5 | Code::CodeKey { rung: Decl, file: aho_corasick.c, decl: 5, sub: 0, line: 86 } |  |  | 0.745 |
| walker |  | 6109 | 5 | Code::CodeKey { rung: Decl, file: aho_corasick.c, decl: 6, sub: 0, line: 111 } |  |  | 0.745 |
| walker |  | 6114 | 5 | Code::CodeKey { rung: Decl, file: aho_corasick.c, decl: 7, sub: 0, line: 274 } |  |  | 0.745 |
| walker |  | 6119 | 5 | Code::CodeKey { rung: Decl, file: aho_corasick.c, decl: 8, sub: 0, line: 287 } |  |  | 0.745 |
| walker |  | 6156 | 37 | Code::CodeKey { rung: Decl, file: aho_corasick.c, decl: 12, sub: 0, line: 299 } |  |  | 0.745 |
| walker |  | 6219 | 63 | Code::CodeKey { rung: Decl, file: aho_corasick.c, decl: 2, sub: 0, line: 26 } |  |  | 0.748 |
| walker |  | 6228 | 9 | Code::CodeKey { rung: Doc, file: aho_corasick.c, decl: 3, sub: 0, line: 34 } |  |  | 0.748 |
| walker |  | 6240 | 12 | Code::CodeKey { rung: Doc, file: aho_corasick.c, decl: 4, sub: 0, line: 55 } |  |  | 0.748 |
| walker |  | 6253 | 13 | Code::CodeKey { rung: Doc, file: aho_corasick.c, decl: 12, sub: 0, line: 299 } |  |  | 0.748 |
| walker |  | 6267 | 14 | Code::CodeKey { rung: Doc, file: aho_corasick.c, decl: 6, sub: 0, line: 111 } |  |  | 0.748 |
| ns | 6272 |  | 258 | test/test_krep.c: complete roster of test functions, helpers and main | 4.10 |  | 0.728 |
| walker |  | 6281 | 14 | Code::CodeKey { rung: Doc, file: aho_corasick.c, decl: 7, sub: 0, line: 274 } |  |  | 0.728 |
| walker |  | 6295 | 14 | Code::CodeKey { rung: Doc, file: aho_corasick.c, decl: 11, sub: 0, line: 296 } |  |  | 0.728 |
| walker |  | 6310 | 15 | Code::CodeKey { rung: Doc, file: aho_corasick.c, decl: 5, sub: 0, line: 86 } |  |  | 0.728 |
| walker |  | 6412 | 102 | Code::CodeKey { rung: Decl, file: aho_corasick.c, decl: 1, sub: 0, line: 17 } |  |  | 0.739 |
| walker |  | 6433 | 21 | Code::CodeKey { rung: Doc, file: aho_corasick.c, decl: 9, sub: 0, line: 293 } |  |  | 0.739 |
| walker |  | 6455 | 22 | Code::CodeKey { rung: Doc, file: aho_corasick.c, decl: 8, sub: 0, line: 287 } |  |  | 0.739 |
| ns | 6497 |  | 225 | test/test_regex.c and test/test_multiple_patterns.c rosters | 4.11 |  | 0.723 |
| ns | 6608 |  | 111 | test/test_directory.c roster: the separate recursive-search integration binary | 4.12 |  | 0.715 |
| walker |  | 6747 | 292 | Markdown::Section { file: README.md, section_index: 4, keeps_default_concavity: false } |  |  | 0.734 |
| ns | 6841 |  | 233 | test/test_compat.h: the TESTING wrapper roster and its guards | 4.13 |  | 0.721 |
| walker |  | 6955 | 208 | Code::CodeKey { rung: Names, file: krep.c, decl: 0, sub: 0, line: 0 } |  |  | 0.721 |
| walker |  | 6962 | 7 | Code::CodeKey { rung: Doc, file: krep.c, decl: 4, sub: 0, line: 77 } |  |  | 0.721 |
| ns | 6972 |  | 131 | test/test_krep.h: shared test helper declarations | 4.14 |  | 0.712 |
| walker |  | 6973 | 11 | Code::CodeKey { rung: Doc, file: krep.c, decl: 2, sub: 0, line: 42 } |  |  | 0.712 |
| walker |  | 6987 | 14 | Code::CodeKey { rung: Doc, file: krep.c, decl: 3, sub: 0, line: 44 } |  |  | 0.712 |
| walker |  | 7003 | 16 | Code::CodeKey { rung: Doc, file: krep.c, decl: 1, sub: 0, line: 39 } |  |  | 0.712 |
| ns | 7080 |  | 108 | Makefile compiler configuration: CC, CFLAGS, LDFLAGS, PREFIX | 5.1 |  | 0.706 |
| walker |  | 7233 | 230 | Code::CodeKey { rung: Names, file: krep.c, decl: 0, sub: 1, line: 0 } |  |  | 0.707 |
| walker |  | 7246 | 13 | Code::CodeKey { rung: Decl, file: krep.c, decl: 21, sub: 0, line: 104 } |  |  | 0.707 |
| walker |  | 7261 | 15 | Code::CodeKey { rung: Decl, file: krep.c, decl: 20, sub: 0, line: 101 } |  |  | 0.707 |
| walker |  | 7270 | 9 | Code::CodeKey { rung: Doc, file: krep.c, decl: 15, sub: 0, line: 93 } |  |  | 0.707 |
| walker |  | 7283 | 13 | Code::CodeKey { rung: Doc, file: krep.c, decl: 21, sub: 0, line: 104 } |  |  | 0.707 |
| walker |  | 7297 | 14 | Code::CodeKey { rung: Doc, file: krep.c, decl: 20, sub: 0, line: 101 } |  |  | 0.707 |
| ns | 7316 |  | 236 | .github/workflows/ci.yml in full | 5.2 |  | 0.688 |
| walker |  | 7530 | 233 | Code::CodeKey { rung: Names, file: krep.c, decl: 0, sub: 2, line: 0 } |  |  | 0.689 |
| walker |  | 7535 | 5 | Code::CodeKey { rung: Decl, file: krep.c, decl: 33, sub: 0, line: 128 } |  |  | 0.689 |
| walker |  | 7540 | 5 | Code::CodeKey { rung: Decl, file: krep.c, decl: 34, sub: 0, line: 139 } |  |  | 0.689 |
| walker |  | 7550 | 10 | Code::CodeKey { rung: Decl, file: krep.c, decl: 23, sub: 0, line: 109 } |  |  | 0.689 |
| walker |  | 7561 | 11 | Code::CodeKey { rung: Decl, file: krep.c, decl: 22, sub: 0, line: 107 } |  |  | 0.689 |
| walker |  | 7573 | 12 | Code::CodeKey { rung: Decl, file: krep.c, decl: 24, sub: 0, line: 111 } |  |  | 0.689 |
| ns | 7580 |  | 264 | Makefile architecture detection: which SIMD flags each arch gets | 5.3 |  | 0.676 |
| walker |  | 7583 | 10 | Code::CodeKey { rung: Doc, file: krep.c, decl: 34, sub: 0, line: 139 } |  |  | 0.676 |
| walker |  | 7594 | 11 | Code::CodeKey { rung: Doc, file: krep.c, decl: 33, sub: 0, line: 128 } |  |  | 0.676 |
| walker |  | 7606 | 12 | Code::CodeKey { rung: Doc, file: krep.c, decl: 32, sub: 0, line: 125 } |  |  | 0.676 |
| walker |  | 7620 | 14 | Code::CodeKey { rung: Doc, file: krep.c, decl: 25, sub: 0, line: 116 } |  |  | 0.676 |
| walker |  | 7830 | 210 | Code::CodeKey { rung: Names, file: krep.c, decl: 0, sub: 3, line: 0 } |  |  | 0.687 |
| walker |  | 7835 | 5 | Code::CodeKey { rung: Decl, file: krep.c, decl: 35, sub: 0, line: 175 } |  |  | 0.687 |
| walker |  | 7840 | 5 | Code::CodeKey { rung: Decl, file: krep.c, decl: 36, sub: 0, line: 244 } |  |  | 0.687 |
| walker |  | 7845 | 5 | Code::CodeKey { rung: Decl, file: krep.c, decl: 37, sub: 0, line: 256 } |  |  | 0.687 |
| walker |  | 7850 | 5 | Code::CodeKey { rung: Decl, file: krep.c, decl: 39, sub: 0, line: 363 } |  |  | 0.687 |
| walker |  | 7855 | 5 | Code::CodeKey { rung: Decl, file: krep.c, decl: 40, sub: 0, line: 401 } |  |  | 0.687 |
| walker |  | 7860 | 5 | Code::CodeKey { rung: Decl, file: krep.c, decl: 41, sub: 0, line: 420 } |  |  | 0.687 |
| walker |  | 7865 | 5 | Code::CodeKey { rung: Decl, file: krep.c, decl: 42, sub: 0, line: 438 } |  |  | 0.687 |
| ns | 7883 |  | 303 | Makefile compile/link rules and the parallel -DTESTING build | 5.4 |  | 0.676 |
| walker |  | 7902 | 37 | Code::CodeKey { rung: Decl, file: krep.c, decl: 38, sub: 0, line: 329 } |  |  | 0.676 |
| walker |  | 7915 | 13 | Code::CodeKey { rung: Doc, file: krep.c, decl: 36, sub: 0, line: 244 } |  |  | 0.676 |
| walker |  | 7931 | 16 | Code::CodeKey { rung: Doc, file: krep.c, decl: 40, sub: 0, line: 401 } |  |  | 0.676 |
| walker |  | 7949 | 18 | Code::CodeKey { rung: Doc, file: krep.c, decl: 35, sub: 0, line: 175 } |  |  | 0.676 |
| walker |  | 7967 | 18 | Code::CodeKey { rung: Doc, file: krep.c, decl: 41, sub: 0, line: 420 } |  |  | 0.676 |
| walker |  | 7987 | 20 | Code::CodeKey { rung: Doc, file: krep.c, decl: 38, sub: 0, line: 329 } |  |  | 0.676 |
| walker |  | 8019 | 32 | Code::CodeKey { rung: Doc, file: krep.c, decl: 42, sub: 0, line: 438 } |  |  | 0.676 |
| ns | 8052 |  | 169 | Makefile run targets: test, test-directory, ci, bench-rg, all-tests | 5.5 |  | 0.666 |
| walker |  | 8059 | 40 | Code::CodeKey { rung: Doc, file: krep.c, decl: 39, sub: 0, line: 363 } |  |  | 0.666 |
| walker |  | 8108 | 49 | Code::CodeKey { rung: Doc, file: krep.c, decl: 37, sub: 0, line: 256 } |  |  | 0.666 |
| ns | 8209 |  | 157 | .github/workflows/release.yml: tag trigger and release artifacts | 5.6 |  | 0.654 |
| walker |  | 8324 | 216 | Code::CodeKey { rung: Names, file: krep.c, decl: 0, sub: 4, line: 0 } |  |  | 0.661 |
| walker |  | 8329 | 5 | Code::CodeKey { rung: Decl, file: krep.c, decl: 43, sub: 0, line: 461 } |  |  | 0.661 |
| walker |  | 8335 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 44, sub: 0, line: 1084 } |  |  | 0.661 |
| walker |  | 8341 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 45, sub: 0, line: 1125 } |  |  | 0.661 |
| walker |  | 8347 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 46, sub: 0, line: 1137 } |  |  | 0.661 |
| walker |  | 8353 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 47, sub: 0, line: 1198 } |  |  | 0.661 |
| walker |  | 8359 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 48, sub: 0, line: 1213 } |  |  | 0.661 |
| walker |  | 8400 | 41 | Code::CodeKey { rung: Decl, file: krep.c, decl: 50, sub: 0, line: 1389 } |  |  | 0.661 |
| walker |  | 8450 | 50 | Code::CodeKey { rung: Decl, file: krep.c, decl: 49, sub: 0, line: 1259 } |  |  | 0.661 |
| walker |  | 8460 | 10 | Code::CodeKey { rung: Doc, file: krep.c, decl: 46, sub: 0, line: 1137 } |  |  | 0.661 |
| ns | 8462 |  | 253 | krep.c constants: VERSION and every performance tunable | 6.1 |  | 0.664 |
| walker |  | 8471 | 11 | Code::CodeKey { rung: Doc, file: krep.c, decl: 45, sub: 0, line: 1125 } |  |  | 0.664 |
| walker |  | 8486 | 15 | Code::CodeKey { rung: Doc, file: krep.c, decl: 48, sub: 0, line: 1213 } |  |  | 0.664 |
| walker |  | 8503 | 17 | Code::CodeKey { rung: Doc, file: krep.c, decl: 47, sub: 0, line: 1198 } |  |  | 0.664 |
| walker |  | 8539 | 36 | Code::CodeKey { rung: Doc, file: krep.c, decl: 49, sub: 0, line: 1259 } |  |  | 0.664 |
| walker |  | 8576 | 37 | Code::CodeKey { rung: Doc, file: krep.c, decl: 44, sub: 0, line: 1084 } |  |  | 0.664 |
| ns | 8696 |  | 234 | krep.c global option state and the lower_table constructor | 6.2 |  | 0.663 |
| walker |  | 8809 | 233 | Code::CodeKey { rung: Names, file: krep.c, decl: 0, sub: 5, line: 0 } |  |  | 0.676 |
| walker |  | 8815 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 51, sub: 0, line: 1585 } |  |  | 0.676 |
| walker |  | 8821 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 53, sub: 0, line: 1771 } |  |  | 0.676 |
| walker |  | 8827 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 54, sub: 0, line: 1873 } |  |  | 0.676 |
| walker |  | 8833 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 55, sub: 0, line: 1919 } |  |  | 0.676 |
| walker |  | 8839 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 56, sub: 0, line: 1964 } |  |  | 0.676 |
| walker |  | 8845 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 57, sub: 0, line: 1999 } |  |  | 0.676 |
| walker |  | 8851 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 59, sub: 0, line: 2252 } |  |  | 0.676 |
| walker |  | 8857 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 60, sub: 0, line: 2265 } |  |  | 0.676 |
| walker |  | 8863 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 61, sub: 0, line: 2274 } |  |  | 0.676 |
| walker |  | 8913 | 50 | Code::CodeKey { rung: Decl, file: krep.c, decl: 52, sub: 0, line: 1628 } |  |  | 0.676 |
| walker |  | 8923 | 10 | Code::CodeKey { rung: Doc, file: krep.c, decl: 58, sub: 0, line: 2249 } |  |  | 0.676 |
| walker |  | 8936 | 13 | Code::CodeKey { rung: Doc, file: krep.c, decl: 60, sub: 0, line: 2265 } |  |  | 0.676 |
| walker |  | 8952 | 16 | Code::CodeKey { rung: Doc, file: krep.c, decl: 57, sub: 0, line: 1999 } |  |  | 0.676 |
| ns | 8966 |  | 270 | main: the getopt_long table and short-option string | 6.3 |  | 0.668 |
| walker |  | 8968 | 16 | Code::CodeKey { rung: Doc, file: krep.c, decl: 59, sub: 0, line: 2252 } |  |  | 0.668 |
| walker |  | 8985 | 17 | Code::CodeKey { rung: Doc, file: krep.c, decl: 56, sub: 0, line: 1964 } |  |  | 0.668 |
| walker |  | 9004 | 19 | Code::CodeKey { rung: Doc, file: krep.c, decl: 54, sub: 0, line: 1873 } |  |  | 0.668 |
| walker |  | 9023 | 19 | Code::CodeKey { rung: Doc, file: krep.c, decl: 55, sub: 0, line: 1919 } |  |  | 0.668 |
| walker |  | 9079 | 56 | Code::CodeKey { rung: Doc, file: krep.c, decl: 52, sub: 0, line: 1628 } |  |  | 0.668 |
| walker |  | 9138 | 59 | Code::CodeKey { rung: Doc, file: krep.c, decl: 51, sub: 0, line: 1585 } |  |  | 0.668 |
| ns | 9299 |  | 333 | select_search_algorithm: dispatch head through the short-pattern branch | 6.4 |  | 0.654 |
| walker |  | 9384 | 246 | Code::CodeKey { rung: Names, file: krep.c, decl: 0, sub: 6, line: 0 } |  |  | 0.667 |
| walker |  | 9390 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 62, sub: 0, line: 3071 } |  |  | 0.667 |
| walker |  | 9396 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 63, sub: 0, line: 3090 } |  |  | 0.667 |
| walker |  | 9402 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 64, sub: 0, line: 3122 } |  |  | 0.667 |
| walker |  | 9408 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 67, sub: 0, line: 3163 } |  |  | 0.667 |
| walker |  | 9414 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 68, sub: 0, line: 3180 } |  |  | 0.667 |
| walker |  | 9420 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 69, sub: 0, line: 3240 } |  |  | 0.667 |
| walker |  | 9426 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 70, sub: 0, line: 3272 } |  |  | 0.667 |
| walker |  | 9432 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 71, sub: 0, line: 3297 } |  |  | 0.667 |
| walker |  | 9456 | 24 | Code::CodeKey { rung: Decl, file: krep.c, decl: 72, sub: 0, line: 3310 } |  |  | 0.667 |
| walker |  | 9490 | 34 | Code::CodeKey { rung: Decl, file: krep.c, decl: 65, sub: 0, line: 3146 } |  |  | 0.667 |
| walker |  | 9547 | 57 | Code::CodeKey { rung: Decl, file: krep.c, decl: 66, sub: 0, line: 3154 } |  |  | 0.668 |
| walker |  | 9561 | 14 | Code::CodeKey { rung: Doc, file: krep.c, decl: 67, sub: 0, line: 3163 } |  |  | 0.668 |
| walker |  | 9576 | 15 | Code::CodeKey { rung: Doc, file: krep.c, decl: 62, sub: 0, line: 3071 } |  |  | 0.668 |
| walker |  | 9591 | 15 | Code::CodeKey { rung: Doc, file: krep.c, decl: 63, sub: 0, line: 3090 } |  |  | 0.668 |
| walker |  | 9606 | 15 | Code::CodeKey { rung: Doc, file: krep.c, decl: 64, sub: 0, line: 3122 } |  |  | 0.668 |
| walker |  | 9621 | 15 | Code::CodeKey { rung: Doc, file: krep.c, decl: 65, sub: 0, line: 3146 } |  |  | 0.668 |
| walker |  | 9636 | 15 | Code::CodeKey { rung: Doc, file: krep.c, decl: 72, sub: 0, line: 3310 } |  |  | 0.668 |
| walker |  | 9654 | 18 | Code::CodeKey { rung: Doc, file: krep.c, decl: 68, sub: 0, line: 3180 } |  |  | 0.668 |
| walker |  | 9674 | 20 | Code::CodeKey { rung: Doc, file: krep.c, decl: 66, sub: 0, line: 3154 } |  |  | 0.668 |
| ns | 9685 |  | 386 | select_search_algorithm: SIMD length limits and the KMP/Boyer-Moore fallback | 6.5 | 6.4 | 0.652 |
| walker |  | 9694 | 20 | Code::CodeKey { rung: Doc, file: krep.c, decl: 70, sub: 0, line: 3272 } |  |  | 0.652 |
| walker |  | 9715 | 21 | Code::CodeKey { rung: Doc, file: krep.c, decl: 71, sub: 0, line: 3297 } |  |  | 0.652 |
| walker |  | 9739 | 24 | Code::CodeKey { rung: Doc, file: krep.c, decl: 69, sub: 0, line: 3240 } |  |  | 0.652 |
| ns | 9868 |  | 183 | gitignore data model: pattern record and parent-chained context | 6.6 |  | 0.657 |
| walker |  | 9941 | 202 | Code::CodeKey { rung: Names, file: krep.c, decl: 0, sub: 7, line: 0 } |  |  | 0.664 |
| walker |  | 9947 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 73, sub: 0, line: 3442 } |  |  | 0.664 |
| walker |  | 9953 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 75, sub: 0, line: 4046 } |  |  | 0.664 |
| walker |  | 9959 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 76, sub: 0, line: 4104 } |  |  | 0.664 |
| walker |  | 9965 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 77, sub: 0, line: 4209 } |  |  | 0.664 |
| walker |  | 9971 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 78, sub: 0, line: 4251 } |  |  | 0.664 |
| walker |  | 9977 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 79, sub: 0, line: 4313 } |  |  | 0.664 |
| walker |  | 9983 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 80, sub: 0, line: 4332 } |  |  | 0.664 |
| ns | 9993 |  | 125 | Licence header, dependabot config and .gitignore | 7.1 |  | 0.658 |
| walker |  | 9995 | 12 | Code::CodeKey { rung: Decl, file: krep.c, decl: 74, sub: 0, line: 3891 } |  |  | 0.658 |
