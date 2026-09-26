Score(3000)=0.678 I=0.843 C=0.546 ns_rows≤3K=20/56 grid(1000/1442/2080/3000/4327/6240/9000)=0.651/0.697/0.678/0.678/0.786/0.715/0.642

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
| walker |  | 1861 | 191 | Code::CodeKey { rung: Names, file: krep.h, decl: 0, sub: 0, line: 0 } |  |  | 0.733 |
| walker |  | 1875 | 14 | Code::CodeKey { rung: Doc, file: krep.h, decl: 1, sub: 0, line: 19 } |  |  | 0.733 |
| walker |  | 1889 | 14 | Code::CodeKey { rung: Doc, file: krep.h, decl: 2, sub: 0, line: 22 } |  |  | 0.733 |
| walker |  | 1909 | 20 | Code::CodeKey { rung: Doc, file: krep.h, decl: 4, sub: 0, line: 34 } |  |  | 0.733 |
| ns | 1963 |  | 210 | README: the documented smart algorithm-selection policy | 2.5 |  | 0.694 |
| ns | 2069 |  | 106 | README usage examples: the six concrete invocations, command lines only | 2.6 |  | 0.676 |
| walker |  | 2106 | 197 | Code::CodeKey { rung: Names, file: krep.h, decl: 0, sub: 1, line: 0 } |  |  | 0.678 |
| walker |  | 2138 | 32 | Code::CodeKey { rung: Decl, file: krep.h, decl: 17, sub: 0, line: 98 } |  |  | 0.679 |
| walker |  | 2178 | 40 | Code::CodeKey { rung: Decl, file: krep.h, decl: 14, sub: 0, line: 49 } |  |  | 0.680 |
| ns | 2181 |  | 112 | README: the multi-threading architecture section | 2.7 |  | 0.653 |
| walker |  | 2240 | 62 | Code::CodeKey { rung: Decl, file: krep.h, decl: 15, sub: 0, line: 55 } |  |  | 0.655 |
| walker |  | 2249 | 9 | Code::CodeKey { rung: Doc, file: krep.h, decl: 10, sub: 0, line: 42 } |  |  | 0.655 |
| ns | 2287 |  | 106 | README: the recursive-search skipping rules | 2.8 |  | 0.636 |
| ns | 2402 |  | 115 | README scope note: explicitly not a grep/ripgrep replacement | 2.9 |  | 0.643 |
| ns | 2512 |  | 110 | README install and build-from-source commands | 2.10 |  | 0.618 |
| walker |  | 2617 | 368 | Code::CodeKey { rung: Decl, file: krep.h, decl: 16, sub: 0, line: 65 } |  |  | 0.629 |
| walker |  | 2630 | 13 | Code::CodeKey { rung: Doc, file: krep.h, decl: 16, sub: 0, line: 65 } |  |  | 0.629 |
| walker |  | 2653 | 23 | Code::CodeKey { rung: Doc, file: krep.h, decl: 14, sub: 0, line: 49 } |  |  | 0.630 |
| walker |  | 2862 | 209 | Code::CodeKey { rung: Names, file: krep.h, decl: 0, sub: 2, line: 0 } |  |  | 0.636 |
| ns | 2902 |  | 390 | search_params_t: the struct every search function takes | 3.1 |  | 0.678 |
| walker |  | 2913 | 51 | Code::CodeKey { rung: Decl, file: krep.h, decl: 19, sub: 0, line: 125 } |  |  | 0.678 |
| walker |  | 3074 | 161 | Code::CodeKey { rung: Decl, file: krep.h, decl: 20, sub: 0, line: 132 } |  |  | 0.678 |
| walker |  | 3083 | 9 | Code::CodeKey { rung: Doc, file: krep.h, decl: 19, sub: 0, line: 125 } |  |  | 0.678 |
| ns | 3089 |  | 187 | match_position_t and match_result_t | 3.2 |  | 0.692 |
| walker |  | 3092 | 9 | Code::CodeKey { rung: Doc, file: krep.h, decl: 21, sub: 0, line: 146 } |  |  | 0.693 |
| ns | 3180 |  | 91 | search_func_t: the algorithm function-pointer type | 3.3 |  | 0.688 |
| ns | 3282 |  | 102 | krep.h thread-pool API: all four functions in full | 3.4 |  | 0.694 |
| walker |  | 3342 | 250 | Code::CodeKey { rung: Decl, file: krep.h, decl: 18, sub: 0, line: 104 } |  |  | 0.700 |
| walker |  | 3352 | 10 | Code::CodeKey { rung: Doc, file: krep.h, decl: 18, sub: 0, line: 104 } |  |  | 0.700 |
| walker |  | 3529 | 177 | Code::CodeKey { rung: Names, file: krep.h, decl: 0, sub: 3, line: 0 } |  |  | 0.732 |
| walker |  | 3536 | 7 | Code::CodeKey { rung: Doc, file: krep.h, decl: 33, sub: 0, line: 203 } |  |  | 0.732 |
| walker |  | 3546 | 10 | Code::CodeKey { rung: Doc, file: krep.h, decl: 28, sub: 0, line: 183 } |  |  | 0.732 |
| ns | 3562 |  | 280 | thread_data_t: per-thread search state, including the false-sharing padding | 3.5 |  | 0.745 |
| ns | 3659 |  | 97 | krep.h helper roster: line finding, word matching, table preparation | 3.6 |  | 0.732 |
| walker |  | 3689 | 143 | Code::CodeKey { rung: Names, file: krep.h, decl: 0, sub: 4, line: 0 } |  |  | 0.736 |
| ns | 3826 |  | 167 | skip_directories: the complete recursive-search directory blocklist | 3.7 |  | 0.722 |
| walker |  | 3841 | 152 | Code::CodeKey { rung: Names, file: krep.h, decl: 0, sub: 5, line: 0 } |  |  | 0.732 |
| walker |  | 3860 | 19 | Code::CodeKey { rung: Decl, file: krep.h, decl: 40, sub: 0, line: 218 } |  |  | 0.738 |
| walker |  | 3879 | 19 | Code::CodeKey { rung: Decl, file: krep.h, decl: 39, sub: 0, line: 214 } |  |  | 0.744 |
| walker |  | 3898 | 19 | Code::CodeKey { rung: Decl, file: krep.h, decl: 42, sub: 0, line: 226 } |  |  | 0.752 |
| walker |  | 3922 | 24 | Code::CodeKey { rung: Decl, file: krep.h, decl: 41, sub: 0, line: 222 } |  |  | 0.763 |
| ns | 4011 |  | 185 | skip_extensions: head, ellipsis, and the count line | 3.8 |  | 0.751 |
| walker |  | 4111 | 189 | Code::CodeKey { rung: Names, file: krep.h, decl: 0, sub: 6, line: 0 } |  |  | 0.770 |
| walker |  | 4116 | 5 | Code::CodeKey { rung: Decl, file: krep.h, decl: 49, sub: 0, line: 298 } |  |  | 0.770 |
| walker |  | 4121 | 5 | Code::CodeKey { rung: Decl, file: krep.h, decl: 50, sub: 0, line: 312 } |  |  | 0.770 |
| walker |  | 4129 | 8 | Code::CodeKey { rung: Doc, file: krep.h, decl: 43, sub: 0, line: 231 } |  |  | 0.770 |
| walker |  | 4166 | 37 | Code::CodeKey { rung: Doc, file: krep.h, decl: 17, sub: 0, line: 98 } |  |  | 0.778 |
| walker |  | 4234 | 68 | Code::CodeKey { rung: Doc, file: krep.h, decl: 49, sub: 0, line: 298 } |  |  | 0.778 |
| ns | 4236 |  | 225 | ANSI colour macro block, output palette and help palette | 3.9 |  | 0.782 |
| ns | 4395 |  | 159 | Documented contracts of the three entry points and the printer | 3.10 | 1.7 | 0.768 |
| walker |  | 4419 | 185 | Markdown::Section { file: README.md, section_index: 9, keeps_default_concavity: false } |  |  | 0.789 |
| walker |  | 4509 | 90 | Code::CodeKey { rung: Doc, file: krep.h, decl: 26, sub: 0, line: 170 } |  |  | 0.790 |
| ns | 4634 |  | 239 | krep.c file-level section banner map, all sixteen top-level banners | 4.1 |  | 0.766 |
| walker |  | 4707 | 198 | Markdown::Section { file: README.md, section_index: 10, keeps_default_concavity: false } |  |  | 0.795 |
| ns | 4758 |  | 124 | krep.c definition roster, lines 139-461: match results, line finding, printing | 4.2 |  | 0.780 |
| walker |  | 4836 | 129 | Markdown::Section { file: README.md, section_index: 11, keeps_default_concavity: false } |  |  | 0.798 |
| walker |  | 4932 | 96 | Code::CodeKey { rung: Doc, file: krep.h, decl: 27, sub: 0, line: 180 } |  |  | 0.780 |
| ns | 4932 |  | 174 | krep.c definition roster, lines 1084-1964: utilities, the four scalar algorithms, orchestration | 4.3 |  | 0.780 |
| walker |  | 5029 | 97 | Code::CodeKey { rung: Doc, file: krep.h, decl: 47, sub: 0, line: 278 } |  |  | 0.780 |
| ns | 5124 |  | 192 | krep.c definition roster, lines 1999-3442: public API bodies, skip filters, gitignore | 4.4 |  | 0.762 |
| walker |  | 5131 | 102 | Code::CodeKey { rung: Doc, file: krep.h, decl: 48, sub: 0, line: 288 } |  |  | 0.762 |
| ns | 5363 |  | 239 | krep.c definition roster, lines 3451-5108: main, thread pool, and the guarded SIMD kernels | 4.5 |  | 0.741 |
| walker |  | 5423 | 292 | Markdown::Section { file: README.md, section_index: 4, keeps_default_concavity: false } |  |  | 0.763 |
| walker |  | 5530 | 107 | Code::CodeKey { rung: Doc, file: krep.h, decl: 50, sub: 0, line: 312 } |  |  | 0.763 |
| ns | 5546 |  | 183 | search_file internal section map, all eleven banners inside lines 2274-3070 | 4.6 |  | 0.750 |
| ns | 5668 |  | 122 | print_matching_items internal section map: the -o and full-line output modes | 4.7 |  | 0.741 |
| walker |  | 5706 | 176 | Code::CodeKey { rung: Names, file: krep.c, decl: 0, sub: 0, line: 0 } |  |  | 0.741 |
| walker |  | 5713 | 7 | Code::CodeKey { rung: Doc, file: krep.c, decl: 4, sub: 0, line: 77 } |  |  | 0.741 |
| walker |  | 5724 | 11 | Code::CodeKey { rung: Doc, file: krep.c, decl: 2, sub: 0, line: 42 } |  |  | 0.741 |
| walker |  | 5738 | 14 | Code::CodeKey { rung: Doc, file: krep.c, decl: 3, sub: 0, line: 44 } |  |  | 0.741 |
| walker |  | 5754 | 16 | Code::CodeKey { rung: Doc, file: krep.c, decl: 1, sub: 0, line: 39 } |  |  | 0.741 |
| ns | 5822 |  | 154 | aho_corasick.c: complete definition roster | 4.8 |  | 0.732 |
| walker |  | 5935 | 181 | Code::CodeKey { rung: Names, file: krep.c, decl: 0, sub: 1, line: 0 } |  |  | 0.733 |
| walker |  | 5944 | 9 | Code::CodeKey { rung: Doc, file: krep.c, decl: 15, sub: 0, line: 93 } |  |  | 0.733 |
| ns | 6014 |  | 192 | aho_corasick.c trie data model: ac_node_t and struct ac_trie | 4.9 |  | 0.715 |
| walker |  | 6141 | 197 | Code::CodeKey { rung: Names, file: krep.c, decl: 0, sub: 2, line: 0 } |  |  | 0.715 |
| walker |  | 6151 | 10 | Code::CodeKey { rung: Decl, file: krep.c, decl: 23, sub: 0, line: 109 } |  |  | 0.715 |
| walker |  | 6162 | 11 | Code::CodeKey { rung: Decl, file: krep.c, decl: 22, sub: 0, line: 107 } |  |  | 0.715 |
| walker |  | 6174 | 12 | Code::CodeKey { rung: Decl, file: krep.c, decl: 24, sub: 0, line: 111 } |  |  | 0.715 |
| walker |  | 6187 | 13 | Code::CodeKey { rung: Decl, file: krep.c, decl: 21, sub: 0, line: 104 } |  |  | 0.715 |
| walker |  | 6202 | 15 | Code::CodeKey { rung: Decl, file: krep.c, decl: 20, sub: 0, line: 101 } |  |  | 0.715 |
| walker |  | 6215 | 13 | Code::CodeKey { rung: Doc, file: krep.c, decl: 21, sub: 0, line: 104 } |  |  | 0.715 |
| walker |  | 6229 | 14 | Code::CodeKey { rung: Doc, file: krep.c, decl: 20, sub: 0, line: 101 } |  |  | 0.715 |
| walker |  | 6243 | 14 | Code::CodeKey { rung: Doc, file: krep.c, decl: 25, sub: 0, line: 116 } |  |  | 0.715 |
| ns | 6272 |  | 258 | test/test_krep.c: complete roster of test functions, helpers and main | 4.10 |  | 0.696 |
| walker |  | 6431 | 188 | Code::CodeKey { rung: Names, file: krep.c, decl: 0, sub: 3, line: 0 } |  |  | 0.699 |
| walker |  | 6436 | 5 | Code::CodeKey { rung: Decl, file: krep.c, decl: 33, sub: 0, line: 128 } |  |  | 0.699 |
| walker |  | 6441 | 5 | Code::CodeKey { rung: Decl, file: krep.c, decl: 34, sub: 0, line: 139 } |  |  | 0.699 |
| walker |  | 6446 | 5 | Code::CodeKey { rung: Decl, file: krep.c, decl: 35, sub: 0, line: 175 } |  |  | 0.699 |
| walker |  | 6451 | 5 | Code::CodeKey { rung: Decl, file: krep.c, decl: 36, sub: 0, line: 244 } |  |  | 0.699 |
| walker |  | 6456 | 5 | Code::CodeKey { rung: Decl, file: krep.c, decl: 37, sub: 0, line: 256 } |  |  | 0.699 |
| walker |  | 6466 | 10 | Code::CodeKey { rung: Doc, file: krep.c, decl: 34, sub: 0, line: 139 } |  |  | 0.699 |
| walker |  | 6477 | 11 | Code::CodeKey { rung: Doc, file: krep.c, decl: 33, sub: 0, line: 128 } |  |  | 0.699 |
| walker |  | 6489 | 12 | Code::CodeKey { rung: Doc, file: krep.c, decl: 32, sub: 0, line: 125 } |  |  | 0.699 |
| ns | 6497 |  | 225 | test/test_regex.c and test/test_multiple_patterns.c rosters | 4.11 |  | 0.684 |
| walker |  | 6502 | 13 | Code::CodeKey { rung: Doc, file: krep.c, decl: 36, sub: 0, line: 244 } |  |  | 0.684 |
| walker |  | 6520 | 18 | Code::CodeKey { rung: Doc, file: krep.c, decl: 35, sub: 0, line: 175 } |  |  | 0.684 |
| ns | 6608 |  | 111 | test/test_directory.c roster: the separate recursive-search integration binary | 4.12 |  | 0.676 |
| walker |  | 6699 | 179 | Code::CodeKey { rung: Names, file: krep.c, decl: 0, sub: 4, line: 0 } |  |  | 0.689 |
| walker |  | 6704 | 5 | Code::CodeKey { rung: Decl, file: krep.c, decl: 39, sub: 0, line: 363 } |  |  | 0.689 |
| walker |  | 6709 | 5 | Code::CodeKey { rung: Decl, file: krep.c, decl: 40, sub: 0, line: 401 } |  |  | 0.689 |
| walker |  | 6714 | 5 | Code::CodeKey { rung: Decl, file: krep.c, decl: 41, sub: 0, line: 420 } |  |  | 0.689 |
| walker |  | 6719 | 5 | Code::CodeKey { rung: Decl, file: krep.c, decl: 42, sub: 0, line: 438 } |  |  | 0.689 |
| walker |  | 6724 | 5 | Code::CodeKey { rung: Decl, file: krep.c, decl: 43, sub: 0, line: 461 } |  |  | 0.689 |
| walker |  | 6761 | 37 | Code::CodeKey { rung: Decl, file: krep.c, decl: 38, sub: 0, line: 329 } |  |  | 0.689 |
| walker |  | 6777 | 16 | Code::CodeKey { rung: Doc, file: krep.c, decl: 40, sub: 0, line: 401 } |  |  | 0.689 |
| walker |  | 6795 | 18 | Code::CodeKey { rung: Doc, file: krep.c, decl: 41, sub: 0, line: 420 } |  |  | 0.689 |
| walker |  | 6815 | 20 | Code::CodeKey { rung: Doc, file: krep.c, decl: 38, sub: 0, line: 329 } |  |  | 0.689 |
| ns | 6841 |  | 233 | test/test_compat.h: the TESTING wrapper roster and its guards | 4.13 |  | 0.677 |
| ns | 6972 |  | 131 | test/test_krep.h: shared test helper declarations | 4.14 |  | 0.668 |
| walker |  | 6991 | 176 | Code::CodeKey { rung: Names, file: krep.c, decl: 0, sub: 5, line: 0 } |  |  | 0.674 |
| walker |  | 6997 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 44, sub: 0, line: 1084 } |  |  | 0.674 |
| walker |  | 7003 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 45, sub: 0, line: 1125 } |  |  | 0.674 |
| walker |  | 7009 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 46, sub: 0, line: 1137 } |  |  | 0.674 |
| walker |  | 7015 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 47, sub: 0, line: 1198 } |  |  | 0.674 |
| walker |  | 7021 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 48, sub: 0, line: 1213 } |  |  | 0.674 |
| walker |  | 7062 | 41 | Code::CodeKey { rung: Decl, file: krep.c, decl: 50, sub: 0, line: 1389 } |  |  | 0.674 |
| ns | 7080 |  | 108 | Makefile compiler configuration: CC, CFLAGS, LDFLAGS, PREFIX | 5.1 |  | 0.668 |
| walker |  | 7112 | 50 | Code::CodeKey { rung: Decl, file: krep.c, decl: 49, sub: 0, line: 1259 } |  |  | 0.668 |
| walker |  | 7122 | 10 | Code::CodeKey { rung: Doc, file: krep.c, decl: 46, sub: 0, line: 1137 } |  |  | 0.668 |
| walker |  | 7133 | 11 | Code::CodeKey { rung: Doc, file: krep.c, decl: 45, sub: 0, line: 1125 } |  |  | 0.668 |
| walker |  | 7148 | 15 | Code::CodeKey { rung: Doc, file: krep.c, decl: 48, sub: 0, line: 1213 } |  |  | 0.668 |
| walker |  | 7165 | 17 | Code::CodeKey { rung: Doc, file: krep.c, decl: 47, sub: 0, line: 1198 } |  |  | 0.668 |
| ns | 7316 |  | 236 | .github/workflows/ci.yml in full | 5.2 |  | 0.651 |
| walker |  | 7354 | 189 | Code::CodeKey { rung: Names, file: krep.c, decl: 0, sub: 6, line: 0 } |  |  | 0.665 |
| walker |  | 7360 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 51, sub: 0, line: 1585 } |  |  | 0.665 |
| walker |  | 7366 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 53, sub: 0, line: 1771 } |  |  | 0.665 |
| walker |  | 7372 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 54, sub: 0, line: 1873 } |  |  | 0.665 |
| walker |  | 7378 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 55, sub: 0, line: 1919 } |  |  | 0.665 |
| walker |  | 7384 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 56, sub: 0, line: 1964 } |  |  | 0.665 |
| walker |  | 7390 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 57, sub: 0, line: 1999 } |  |  | 0.665 |
| walker |  | 7396 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 59, sub: 0, line: 2252 } |  |  | 0.665 |
| walker |  | 7446 | 50 | Code::CodeKey { rung: Decl, file: krep.c, decl: 52, sub: 0, line: 1628 } |  |  | 0.665 |
| walker |  | 7456 | 10 | Code::CodeKey { rung: Doc, file: krep.c, decl: 58, sub: 0, line: 2249 } |  |  | 0.665 |
| walker |  | 7472 | 16 | Code::CodeKey { rung: Doc, file: krep.c, decl: 57, sub: 0, line: 1999 } |  |  | 0.665 |
| walker |  | 7488 | 16 | Code::CodeKey { rung: Doc, file: krep.c, decl: 59, sub: 0, line: 2252 } |  |  | 0.665 |
| walker |  | 7505 | 17 | Code::CodeKey { rung: Doc, file: krep.c, decl: 56, sub: 0, line: 1964 } |  |  | 0.665 |
| walker |  | 7524 | 19 | Code::CodeKey { rung: Doc, file: krep.c, decl: 54, sub: 0, line: 1873 } |  |  | 0.665 |
| walker |  | 7543 | 19 | Code::CodeKey { rung: Doc, file: krep.c, decl: 55, sub: 0, line: 1919 } |  |  | 0.665 |
| ns | 7580 |  | 264 | Makefile architecture detection: which SIMD flags each arch gets | 5.3 |  | 0.652 |
| walker |  | 7758 | 215 | Code::CodeKey { rung: Names, file: krep.c, decl: 0, sub: 7, line: 0 } |  |  | 0.661 |
| walker |  | 7764 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 60, sub: 0, line: 2265 } |  |  | 0.661 |
| walker |  | 7770 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 61, sub: 0, line: 2274 } |  |  | 0.661 |
| walker |  | 7776 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 62, sub: 0, line: 3071 } |  |  | 0.661 |
| walker |  | 7782 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 63, sub: 0, line: 3090 } |  |  | 0.661 |
| walker |  | 7788 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 64, sub: 0, line: 3122 } |  |  | 0.661 |
| walker |  | 7794 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 67, sub: 0, line: 3163 } |  |  | 0.661 |
| walker |  | 7800 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 68, sub: 0, line: 3180 } |  |  | 0.661 |
| walker |  | 7806 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 69, sub: 0, line: 3240 } |  |  | 0.661 |
| walker |  | 7840 | 34 | Code::CodeKey { rung: Decl, file: krep.c, decl: 65, sub: 0, line: 3146 } |  |  | 0.661 |
| ns | 7883 |  | 303 | Makefile compile/link rules and the parallel -DTESTING build | 5.4 |  | 0.651 |
| walker |  | 7897 | 57 | Code::CodeKey { rung: Decl, file: krep.c, decl: 66, sub: 0, line: 3154 } |  |  | 0.651 |
| walker |  | 7910 | 13 | Code::CodeKey { rung: Doc, file: krep.c, decl: 60, sub: 0, line: 2265 } |  |  | 0.651 |
| walker |  | 7924 | 14 | Code::CodeKey { rung: Doc, file: krep.c, decl: 67, sub: 0, line: 3163 } |  |  | 0.651 |
| walker |  | 7939 | 15 | Code::CodeKey { rung: Doc, file: krep.c, decl: 62, sub: 0, line: 3071 } |  |  | 0.651 |
| walker |  | 7954 | 15 | Code::CodeKey { rung: Doc, file: krep.c, decl: 63, sub: 0, line: 3090 } |  |  | 0.651 |
| walker |  | 7969 | 15 | Code::CodeKey { rung: Doc, file: krep.c, decl: 64, sub: 0, line: 3122 } |  |  | 0.651 |
| walker |  | 7984 | 15 | Code::CodeKey { rung: Doc, file: krep.c, decl: 65, sub: 0, line: 3146 } |  |  | 0.652 |
| walker |  | 8002 | 18 | Code::CodeKey { rung: Doc, file: krep.c, decl: 68, sub: 0, line: 3180 } |  |  | 0.652 |
| walker |  | 8022 | 20 | Code::CodeKey { rung: Doc, file: krep.c, decl: 66, sub: 0, line: 3154 } |  |  | 0.652 |
| walker |  | 8046 | 24 | Code::CodeKey { rung: Doc, file: krep.c, decl: 69, sub: 0, line: 3240 } |  |  | 0.652 |
| ns | 8052 |  | 169 | Makefile run targets: test, test-directory, ci, bench-rg, all-tests | 5.5 |  | 0.642 |
| ns | 8209 |  | 157 | .github/workflows/release.yml: tag trigger and release artifacts | 5.6 |  | 0.630 |
| walker |  | 8323 | 277 | Code::CodeKey { rung: Names, file: krep.c, decl: 0, sub: 8, line: 0 } |  |  | 0.645 |
| walker |  | 8329 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 70, sub: 0, line: 3272 } |  |  | 0.645 |
| walker |  | 8335 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 71, sub: 0, line: 3297 } |  |  | 0.645 |
| walker |  | 8341 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 73, sub: 0, line: 3442 } |  |  | 0.645 |
| walker |  | 8347 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 75, sub: 0, line: 4046 } |  |  | 0.645 |
| walker |  | 8353 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 76, sub: 0, line: 4104 } |  |  | 0.645 |
| walker |  | 8359 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 77, sub: 0, line: 4209 } |  |  | 0.645 |
| walker |  | 8365 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 78, sub: 0, line: 4251 } |  |  | 0.645 |
| walker |  | 8371 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 79, sub: 0, line: 4313 } |  |  | 0.645 |
| walker |  | 8377 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 80, sub: 0, line: 4332 } |  |  | 0.645 |
| walker |  | 8401 | 24 | Code::CodeKey { rung: Decl, file: krep.c, decl: 72, sub: 0, line: 3310 } |  |  | 0.645 |
| walker |  | 8442 | 41 | Code::CodeKey { rung: Decl, file: krep.c, decl: 74, sub: 0, line: 3891 } |  |  | 0.645 |
| ns | 8462 |  | 253 | krep.c constants: VERSION and every performance tunable | 6.1 |  | 0.649 |
| walker |  | 8483 | 41 | Code::CodeKey { rung: Decl, file: krep.c, decl: 81, sub: 0, line: 4371 } |  |  | 0.649 |
| walker |  | 8494 | 11 | Code::CodeKey { rung: Doc, file: krep.c, decl: 80, sub: 0, line: 4332 } |  |  | 0.649 |
| walker |  | 8507 | 13 | Code::CodeKey { rung: Doc, file: krep.c, decl: 74, sub: 0, line: 3891 } |  |  | 0.649 |
| walker |  | 8520 | 13 | Code::CodeKey { rung: Doc, file: krep.c, decl: 79, sub: 0, line: 4313 } |  |  | 0.649 |
| walker |  | 8534 | 14 | Code::CodeKey { rung: Doc, file: krep.c, decl: 77, sub: 0, line: 4209 } |  |  | 0.649 |
| walker |  | 8549 | 15 | Code::CodeKey { rung: Doc, file: krep.c, decl: 72, sub: 0, line: 3310 } |  |  | 0.649 |
| walker |  | 8564 | 15 | Code::CodeKey { rung: Doc, file: krep.c, decl: 78, sub: 0, line: 4251 } |  |  | 0.649 |
| walker |  | 8580 | 16 | Code::CodeKey { rung: Doc, file: krep.c, decl: 75, sub: 0, line: 4046 } |  |  | 0.649 |
| walker |  | 8598 | 18 | Code::CodeKey { rung: Doc, file: krep.c, decl: 76, sub: 0, line: 4104 } |  |  | 0.649 |
| walker |  | 8618 | 20 | Code::CodeKey { rung: Doc, file: krep.c, decl: 70, sub: 0, line: 3272 } |  |  | 0.649 |
| walker |  | 8639 | 21 | Code::CodeKey { rung: Doc, file: krep.c, decl: 71, sub: 0, line: 3297 } |  |  | 0.649 |
| walker |  | 8660 | 21 | Code::CodeKey { rung: Doc, file: krep.c, decl: 81, sub: 0, line: 4371 } |  |  | 0.649 |
| walker |  | 8692 | 32 | Code::CodeKey { rung: Doc, file: krep.c, decl: 42, sub: 0, line: 438 } |  |  | 0.649 |
| ns | 8696 |  | 234 | krep.c global option state and the lower_table constructor | 6.2 |  | 0.649 |
| walker |  | 8727 | 35 | Code::CodeKey { rung: Doc, file: krep.c, decl: 73, sub: 0, line: 3442 } |  |  | 0.649 |
| walker |  | 8763 | 36 | Code::CodeKey { rung: Doc, file: krep.c, decl: 49, sub: 0, line: 1259 } |  |  | 0.649 |
| walker |  | 8800 | 37 | Code::CodeKey { rung: Doc, file: krep.c, decl: 44, sub: 0, line: 1084 } |  |  | 0.649 |
| walker |  | 8840 | 40 | Code::CodeKey { rung: Doc, file: krep.c, decl: 39, sub: 0, line: 363 } |  |  | 0.649 |
| walker |  | 8889 | 49 | Code::CodeKey { rung: Doc, file: krep.c, decl: 37, sub: 0, line: 256 } |  |  | 0.649 |
| walker |  | 8945 | 56 | Code::CodeKey { rung: Doc, file: krep.c, decl: 52, sub: 0, line: 1628 } |  |  | 0.649 |
| ns | 8966 |  | 270 | main: the getopt_long table and short-option string | 6.3 |  | 0.641 |
| walker |  | 9191 | 246 | Code::CodeKey { rung: Names, file: aho_corasick.c, decl: 0, sub: 0, line: 0 } |  |  | 0.652 |
| walker |  | 9196 | 5 | Code::CodeKey { rung: Decl, file: aho_corasick.c, decl: 3, sub: 0, line: 34 } |  |  | 0.652 |
| walker |  | 9201 | 5 | Code::CodeKey { rung: Decl, file: aho_corasick.c, decl: 4, sub: 0, line: 55 } |  |  | 0.652 |
| walker |  | 9206 | 5 | Code::CodeKey { rung: Decl, file: aho_corasick.c, decl: 5, sub: 0, line: 86 } |  |  | 0.652 |
| walker |  | 9211 | 5 | Code::CodeKey { rung: Decl, file: aho_corasick.c, decl: 6, sub: 0, line: 111 } |  |  | 0.652 |
| walker |  | 9216 | 5 | Code::CodeKey { rung: Decl, file: aho_corasick.c, decl: 7, sub: 0, line: 274 } |  |  | 0.652 |
| walker |  | 9221 | 5 | Code::CodeKey { rung: Decl, file: aho_corasick.c, decl: 8, sub: 0, line: 287 } |  |  | 0.652 |
| walker |  | 9258 | 37 | Code::CodeKey { rung: Decl, file: aho_corasick.c, decl: 12, sub: 0, line: 299 } |  |  | 0.652 |
| ns | 9299 |  | 333 | select_search_algorithm: dispatch head through the short-pattern branch | 6.4 |  | 0.638 |
| walker |  | 9321 | 63 | Code::CodeKey { rung: Decl, file: aho_corasick.c, decl: 2, sub: 0, line: 26 } |  |  | 0.641 |
| walker |  | 9423 | 102 | Code::CodeKey { rung: Decl, file: aho_corasick.c, decl: 1, sub: 0, line: 17 } |  |  | 0.649 |
| walker |  | 9607 | 184 | Code::CodeKey { rung: Names, file: aho_corasick.h, decl: 0, sub: 0, line: 0 } |  |  | 0.655 |
| walker |  | 9617 | 10 | Code::CodeKey { rung: Doc, file: aho_corasick.h, decl: 5, sub: 0, line: 23 } |  |  | 0.655 |
| walker |  | 9629 | 12 | Code::CodeKey { rung: Doc, file: aho_corasick.h, decl: 9, sub: 0, line: 33 } |  |  | 0.657 |
| walker |  | 9641 | 12 | Code::CodeKey { rung: Doc, file: aho_corasick.h, decl: 11, sub: 0, line: 39 } |  |  | 0.659 |
| walker |  | 9655 | 14 | Code::CodeKey { rung: Doc, file: aho_corasick.h, decl: 6, sub: 0, line: 26 } |  |  | 0.659 |
| walker |  | 9670 | 15 | Code::CodeKey { rung: Doc, file: aho_corasick.h, decl: 8, sub: 0, line: 30 } |  |  | 0.662 |
| ns | 9685 |  | 386 | select_search_algorithm: SIMD length limits and the KMP/Boyer-Moore fallback | 6.5 | 6.4 | 0.646 |
| walker |  | 9688 | 18 | Code::CodeKey { rung: Doc, file: aho_corasick.h, decl: 1, sub: 0, line: 14 } |  |  | 0.646 |
| walker |  | 9708 | 20 | Code::CodeKey { rung: Doc, file: aho_corasick.h, decl: 10, sub: 0, line: 36 } |  |  | 0.650 |
| walker |  | 9743 | 35 | Code::CodeKey { rung: Doc, file: aho_corasick.h, decl: 3, sub: 0, line: 19 } |  |  | 0.650 |
| walker |  | 9752 | 9 | Code::CodeKey { rung: Doc, file: aho_corasick.c, decl: 3, sub: 0, line: 34 } |  |  | 0.650 |
| walker |  | 9764 | 12 | Code::CodeKey { rung: Doc, file: aho_corasick.c, decl: 4, sub: 0, line: 55 } |  |  | 0.650 |
| walker |  | 9777 | 13 | Code::CodeKey { rung: Doc, file: aho_corasick.c, decl: 12, sub: 0, line: 299 } |  |  | 0.650 |
| walker |  | 9791 | 14 | Code::CodeKey { rung: Doc, file: aho_corasick.c, decl: 6, sub: 0, line: 111 } |  |  | 0.650 |
| walker |  | 9805 | 14 | Code::CodeKey { rung: Doc, file: aho_corasick.c, decl: 7, sub: 0, line: 274 } |  |  | 0.650 |
| walker |  | 9819 | 14 | Code::CodeKey { rung: Doc, file: aho_corasick.c, decl: 11, sub: 0, line: 296 } |  |  | 0.650 |
| walker |  | 9834 | 15 | Code::CodeKey { rung: Doc, file: aho_corasick.c, decl: 5, sub: 0, line: 86 } |  |  | 0.650 |
| walker |  | 9855 | 21 | Code::CodeKey { rung: Doc, file: aho_corasick.c, decl: 9, sub: 0, line: 293 } |  |  | 0.650 |
| ns | 9868 |  | 183 | gitignore data model: pattern record and parent-chained context | 6.6 |  | 0.655 |
| walker |  | 9877 | 22 | Code::CodeKey { rung: Doc, file: aho_corasick.c, decl: 8, sub: 0, line: 287 } |  |  | 0.655 |
| walker |  | 9936 | 59 | Code::CodeKey { rung: Doc, file: krep.c, decl: 51, sub: 0, line: 1585 } |  |  | 0.655 |
| ns | 9993 |  | 125 | Licence header, dependabot config and .gitignore | 7.1 |  | 0.648 |
