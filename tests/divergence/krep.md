Score(3000)=0.604 I=0.828 C=0.441 ns_rows≤3K=20/56 grid(1000/1442/2080/3000/4327/6240/9000)=0.638/0.752/0.723/0.604/0.751/0.704/0.632

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 35 | 35 | Fs::DirListing { dir: . } |  |  | 0.000 |
| ns | 66 |  | 66 | Repository identity: title line and the one-paragraph what-it-is | 1.1 |  | 0.000 |
| walker |  | 101 | 66 | Markdown::ReadmeHeadline { file: README.md } |  |  | 1.000 |
| ns | 101 |  | 35 | Complete root directory listing | 1.2 |  | 1.000 |
| walker |  | 114 | 13 | Fs::DirListing { dir: .github } |  |  | 1.000 |
| walker |  | 122 | 8 | Fs::DirListing { dir: .github/workflows } |  |  | 1.000 |
| walker |  | 164 | 42 | Fs::DirListing { dir: test } |  |  | 1.000 |
| ns | 164 |  | 63 | Complete listings of test/, .github/ and .github/workflows/ | 1.3 |  | 1.000 |
| ns | 215 |  | 51 | Makefile: binary name, source list, and the complete .PHONY target list | 1.4 |  | 0.929 |
| ns | 333 |  | 118 | README invocation synopsis: all six supported command forms | 1.5 |  | 0.804 |
| walker |  | 446 | 282 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.836 |
| ns | 463 |  | 130 | Every H2 heading location in README.md | 1.6 |  | 0.850 |
| ns | 559 |  | 96 | krep.h public API roster: the three search entry points, match-result management, printing | 1.7 |  | 0.762 |
| walker |  | 561 | 115 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.765 |
| walker |  | 693 | 132 | Markdown::Section { file: README.md, section_index: 5, keeps_default_concavity: true } |  |  | 0.858 |
| walker |  | 714 | 21 | Markdown::Section { file: README.md, section_index: 12, keeps_default_concavity: false } |  |  | 0.858 |
| walker |  | 743 | 29 | Markdown::Section { file: README.md, section_index: 13, keeps_default_concavity: false } |  |  | 0.858 |
| ns | 789 |  | 230 | krep.h search-algorithm roster with the SIMD #if guards intact | 1.8 |  | 0.706 |
| ns | 996 |  | 207 | aho_corasick.h: the complete second-module interface | 1.9 |  | 0.638 |
| walker |  | 1095 | 352 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: true } |  |  | 0.648 |
| ns | 1173 |  | 177 | README Key Features, first half | 2.1 |  | 0.669 |
| walker |  | 1279 | 184 | Code::CodeKey { rung: Names, file: aho_corasick.h, decl: 0, sub: 0, line: 0 } |  |  | 0.696 |
| walker |  | 1289 | 10 | Code::CodeKey { rung: Doc, file: aho_corasick.h, decl: 5, sub: 0, line: 23 } |  |  | 0.696 |
| walker |  | 1301 | 12 | Code::CodeKey { rung: Doc, file: aho_corasick.h, decl: 9, sub: 0, line: 33 } |  |  | 0.703 |
| walker |  | 1313 | 12 | Code::CodeKey { rung: Doc, file: aho_corasick.h, decl: 11, sub: 0, line: 39 } |  |  | 0.713 |
| walker |  | 1327 | 14 | Code::CodeKey { rung: Doc, file: aho_corasick.h, decl: 6, sub: 0, line: 26 } |  |  | 0.713 |
| walker |  | 1342 | 15 | Code::CodeKey { rung: Doc, file: aho_corasick.h, decl: 8, sub: 0, line: 30 } |  |  | 0.725 |
| ns | 1343 |  | 170 | README Key Features, second half | 2.2 |  | 0.736 |
| walker |  | 1360 | 18 | Code::CodeKey { rung: Doc, file: aho_corasick.h, decl: 1, sub: 0, line: 14 } |  |  | 0.736 |
| walker |  | 1380 | 20 | Code::CodeKey { rung: Doc, file: aho_corasick.h, decl: 10, sub: 0, line: 36 } |  |  | 0.752 |
| walker |  | 1415 | 35 | Code::CodeKey { rung: Doc, file: aho_corasick.h, decl: 3, sub: 0, line: 19 } |  |  | 0.752 |
| walker |  | 1479 | 64 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.752 |
| walker |  | 1523 | 44 | Markdown::Section { file: README.md, section_index: 14, keeps_default_concavity: false } |  |  | 0.752 |
| ns | 1534 |  | 191 | README command-line options table, first half (-i through -F) | 2.3 |  | 0.720 |
| ns | 1753 |  | 219 | README command-line options table, second half (-r through -h) | 2.4 |  | 0.684 |
| walker |  | 1938 | 415 | Markdown::Section { file: README.md, section_index: 7, keeps_default_concavity: true } |  |  | 0.784 |
| ns | 1963 |  | 210 | README: the documented smart algorithm-selection policy | 2.5 |  | 0.742 |
| walker |  | 2060 | 122 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.742 |
| ns | 2069 |  | 106 | README usage examples: the six concrete invocations, command lines only | 2.6 |  | 0.723 |
| ns | 2181 |  | 112 | README: the multi-threading architecture section | 2.7 |  | 0.695 |
| walker |  | 2276 | 216 | Code::CodeKey { rung: Names, file: krep.h, decl: 0, sub: 0, line: 0 } |  |  | 0.696 |
| walker |  | 2285 | 9 | Code::CodeKey { rung: Doc, file: krep.h, decl: 10, sub: 0, line: 42 } |  |  | 0.696 |
| ns | 2287 |  | 106 | README: the recursive-search skipping rules | 2.8 |  | 0.676 |
| walker |  | 2299 | 14 | Code::CodeKey { rung: Doc, file: krep.h, decl: 1, sub: 0, line: 19 } |  |  | 0.676 |
| walker |  | 2313 | 14 | Code::CodeKey { rung: Doc, file: krep.h, decl: 2, sub: 0, line: 22 } |  |  | 0.676 |
| walker |  | 2333 | 20 | Code::CodeKey { rung: Doc, file: krep.h, decl: 4, sub: 0, line: 34 } |  |  | 0.676 |
| ns | 2402 |  | 115 | README scope note: explicitly not a grep/ripgrep replacement | 2.9 |  | 0.682 |
| ns | 2512 |  | 110 | README install and build-from-source commands | 2.10 |  | 0.656 |
| walker |  | 2611 | 278 | Code::CodeKey { rung: Names, file: krep.h, decl: 0, sub: 1, line: 0 } |  |  | 0.658 |
| walker |  | 2643 | 32 | Code::CodeKey { rung: Decl, file: krep.h, decl: 17, sub: 0, line: 98 } |  |  | 0.659 |
| walker |  | 2683 | 40 | Code::CodeKey { rung: Decl, file: krep.h, decl: 14, sub: 0, line: 49 } |  |  | 0.660 |
| walker |  | 2734 | 51 | Code::CodeKey { rung: Decl, file: krep.h, decl: 19, sub: 0, line: 125 } |  |  | 0.660 |
| walker |  | 2743 | 9 | Code::CodeKey { rung: Doc, file: krep.h, decl: 19, sub: 0, line: 125 } |  |  | 0.660 |
| walker |  | 2752 | 9 | Code::CodeKey { rung: Doc, file: krep.h, decl: 21, sub: 0, line: 146 } |  |  | 0.660 |
| walker |  | 2814 | 62 | Code::CodeKey { rung: Decl, file: krep.h, decl: 15, sub: 0, line: 55 } |  |  | 0.662 |
| walker |  | 2837 | 23 | Code::CodeKey { rung: Doc, file: krep.h, decl: 14, sub: 0, line: 49 } |  |  | 0.663 |
| ns | 2902 |  | 390 | search_params_t: the struct every search function takes | 3.1 |  | 0.604 |
| walker |  | 2998 | 161 | Code::CodeKey { rung: Decl, file: krep.h, decl: 20, sub: 0, line: 132 } |  |  | 0.604 |
| walker |  | 3035 | 37 | Code::CodeKey { rung: Doc, file: krep.h, decl: 17, sub: 0, line: 98 } |  |  | 0.605 |
| ns | 3089 |  | 187 | match_position_t and match_result_t | 3.2 |  | 0.626 |
| ns | 3180 |  | 91 | search_func_t: the algorithm function-pointer type | 3.3 |  | 0.634 |
| ns | 3282 |  | 102 | krep.h thread-pool API: all four functions in full | 3.4 |  | 0.632 |
| walker |  | 3285 | 250 | Code::CodeKey { rung: Decl, file: krep.h, decl: 18, sub: 0, line: 104 } |  |  | 0.637 |
| walker |  | 3295 | 10 | Code::CodeKey { rung: Doc, file: krep.h, decl: 18, sub: 0, line: 104 } |  |  | 0.637 |
| ns | 3562 |  | 280 | thread_data_t: per-thread search state, including the false-sharing padding | 3.5 |  | 0.659 |
| ns | 3659 |  | 97 | krep.h helper roster: line finding, word matching, table preparation | 3.6 |  | 0.647 |
| walker |  | 3663 | 368 | Code::CodeKey { rung: Decl, file: krep.h, decl: 16, sub: 0, line: 65 } |  |  | 0.727 |
| walker |  | 3676 | 13 | Code::CodeKey { rung: Doc, file: krep.h, decl: 16, sub: 0, line: 65 } |  |  | 0.727 |
| ns | 3826 |  | 167 | skip_directories: the complete recursive-search directory blocklist | 3.7 |  | 0.713 |
| walker |  | 3902 | 226 | Code::CodeKey { rung: Names, file: krep.h, decl: 0, sub: 2, line: 0 } |  |  | 0.748 |
| walker |  | 3912 | 10 | Code::CodeKey { rung: Doc, file: krep.h, decl: 28, sub: 0, line: 183 } |  |  | 0.748 |
| walker |  | 4002 | 90 | Code::CodeKey { rung: Doc, file: krep.h, decl: 26, sub: 0, line: 170 } |  |  | 0.748 |
| ns | 4011 |  | 185 | skip_extensions: head, ellipsis, and the count line | 3.8 |  | 0.736 |
| walker |  | 4098 | 96 | Code::CodeKey { rung: Doc, file: krep.h, decl: 27, sub: 0, line: 180 } |  |  | 0.736 |
| ns | 4236 |  | 225 | ANSI colour macro block, output palette and help palette | 3.9 |  | 0.741 |
| walker |  | 4295 | 197 | Code::CodeKey { rung: Names, file: krep.h, decl: 0, sub: 3, line: 0 } |  |  | 0.751 |
| walker |  | 4302 | 7 | Code::CodeKey { rung: Doc, file: krep.h, decl: 33, sub: 0, line: 203 } |  |  | 0.751 |
| ns | 4395 |  | 159 | Documented contracts of the three entry points and the printer | 3.10 | 1.7 | 0.739 |
| walker |  | 4484 | 182 | Code::CodeKey { rung: Names, file: krep.h, decl: 0, sub: 4, line: 0 } |  |  | 0.749 |
| walker |  | 4503 | 19 | Code::CodeKey { rung: Decl, file: krep.h, decl: 40, sub: 0, line: 218 } |  |  | 0.754 |
| walker |  | 4522 | 19 | Code::CodeKey { rung: Decl, file: krep.h, decl: 39, sub: 0, line: 214 } |  |  | 0.760 |
| walker |  | 4541 | 19 | Code::CodeKey { rung: Decl, file: krep.h, decl: 42, sub: 0, line: 226 } |  |  | 0.767 |
| walker |  | 4565 | 24 | Code::CodeKey { rung: Decl, file: krep.h, decl: 41, sub: 0, line: 222 } |  |  | 0.778 |
| walker |  | 4573 | 8 | Code::CodeKey { rung: Doc, file: krep.h, decl: 43, sub: 0, line: 231 } |  |  | 0.778 |
| ns | 4634 |  | 239 | krep.c file-level section banner map, all sixteen top-level banners | 4.1 |  | 0.754 |
| walker |  | 4732 | 159 | Code::CodeKey { rung: Names, file: krep.h, decl: 0, sub: 5, line: 0 } |  |  | 0.771 |
| walker |  | 4737 | 5 | Code::CodeKey { rung: Decl, file: krep.h, decl: 49, sub: 0, line: 298 } |  |  | 0.771 |
| walker |  | 4742 | 5 | Code::CodeKey { rung: Decl, file: krep.h, decl: 50, sub: 0, line: 312 } |  |  | 0.771 |
| ns | 4758 |  | 124 | krep.c definition roster, lines 139-461: match results, line finding, printing | 4.2 |  | 0.757 |
| walker |  | 4810 | 68 | Code::CodeKey { rung: Doc, file: krep.h, decl: 49, sub: 0, line: 298 } |  |  | 0.757 |
| walker |  | 4907 | 97 | Code::CodeKey { rung: Doc, file: krep.h, decl: 47, sub: 0, line: 278 } |  |  | 0.757 |
| ns | 4932 |  | 174 | krep.c definition roster, lines 1084-1964: utilities, the four scalar algorithms, orchestration | 4.3 |  | 0.740 |
| walker |  | 5009 | 102 | Code::CodeKey { rung: Doc, file: krep.h, decl: 48, sub: 0, line: 288 } |  |  | 0.740 |
| walker |  | 5116 | 107 | Code::CodeKey { rung: Doc, file: krep.h, decl: 50, sub: 0, line: 312 } |  |  | 0.740 |
| ns | 5124 |  | 192 | krep.c definition roster, lines 1999-3442: public API bodies, skip filters, gitignore | 4.4 |  | 0.722 |
| walker |  | 5232 | 116 | Code::CodeKey { rung: Doc, file: krep.h, decl: 25, sub: 0, line: 161 } |  |  | 0.730 |
| ns | 5363 |  | 239 | krep.c definition roster, lines 3451-5108: main, thread pool, and the guarded SIMD kernels | 4.5 |  | 0.710 |
| walker |  | 5401 | 169 | Code::CodeKey { rung: Doc, file: krep.h, decl: 32, sub: 0, line: 200 } |  |  | 0.719 |
| ns | 5546 |  | 183 | search_file internal section map, all eleven banners inside lines 2274-3070 | 4.6 |  | 0.706 |
| walker |  | 5647 | 246 | Code::CodeKey { rung: Names, file: aho_corasick.c, decl: 0, sub: 0, line: 0 } |  |  | 0.708 |
| walker |  | 5652 | 5 | Code::CodeKey { rung: Decl, file: aho_corasick.c, decl: 3, sub: 0, line: 34 } |  |  | 0.708 |
| walker |  | 5657 | 5 | Code::CodeKey { rung: Decl, file: aho_corasick.c, decl: 4, sub: 0, line: 55 } |  |  | 0.708 |
| walker |  | 5662 | 5 | Code::CodeKey { rung: Decl, file: aho_corasick.c, decl: 5, sub: 0, line: 86 } |  |  | 0.708 |
| walker |  | 5667 | 5 | Code::CodeKey { rung: Decl, file: aho_corasick.c, decl: 6, sub: 0, line: 111 } |  |  | 0.708 |
| ns | 5668 |  | 122 | print_matching_items internal section map: the -o and full-line output modes | 4.7 |  | 0.699 |
| walker |  | 5672 | 5 | Code::CodeKey { rung: Decl, file: aho_corasick.c, decl: 7, sub: 0, line: 274 } |  |  | 0.699 |
| walker |  | 5677 | 5 | Code::CodeKey { rung: Decl, file: aho_corasick.c, decl: 8, sub: 0, line: 287 } |  |  | 0.699 |
| walker |  | 5714 | 37 | Code::CodeKey { rung: Decl, file: aho_corasick.c, decl: 12, sub: 0, line: 299 } |  |  | 0.699 |
| walker |  | 5777 | 63 | Code::CodeKey { rung: Decl, file: aho_corasick.c, decl: 2, sub: 0, line: 26 } |  |  | 0.700 |
| walker |  | 5786 | 9 | Code::CodeKey { rung: Doc, file: aho_corasick.c, decl: 3, sub: 0, line: 34 } |  |  | 0.700 |
| walker |  | 5798 | 12 | Code::CodeKey { rung: Doc, file: aho_corasick.c, decl: 4, sub: 0, line: 55 } |  |  | 0.700 |
| walker |  | 5811 | 13 | Code::CodeKey { rung: Doc, file: aho_corasick.c, decl: 12, sub: 0, line: 299 } |  |  | 0.700 |
| ns | 5822 |  | 154 | aho_corasick.c: complete definition roster | 4.8 |  | 0.704 |
| walker |  | 5825 | 14 | Code::CodeKey { rung: Doc, file: aho_corasick.c, decl: 6, sub: 0, line: 111 } |  |  | 0.704 |
| walker |  | 5839 | 14 | Code::CodeKey { rung: Doc, file: aho_corasick.c, decl: 7, sub: 0, line: 274 } |  |  | 0.704 |
| walker |  | 5853 | 14 | Code::CodeKey { rung: Doc, file: aho_corasick.c, decl: 11, sub: 0, line: 296 } |  |  | 0.704 |
| walker |  | 5868 | 15 | Code::CodeKey { rung: Doc, file: aho_corasick.c, decl: 5, sub: 0, line: 86 } |  |  | 0.704 |
| walker |  | 5970 | 102 | Code::CodeKey { rung: Decl, file: aho_corasick.c, decl: 1, sub: 0, line: 17 } |  |  | 0.706 |
| walker |  | 5991 | 21 | Code::CodeKey { rung: Doc, file: aho_corasick.c, decl: 9, sub: 0, line: 293 } |  |  | 0.706 |
| walker |  | 6013 | 22 | Code::CodeKey { rung: Doc, file: aho_corasick.c, decl: 8, sub: 0, line: 287 } |  |  | 0.706 |
| ns | 6014 |  | 192 | aho_corasick.c trie data model: ac_node_t and struct ac_trie | 4.9 |  | 0.703 |
| walker |  | 6221 | 208 | Code::CodeKey { rung: Names, file: krep.c, decl: 0, sub: 0, line: 0 } |  |  | 0.704 |
| walker |  | 6228 | 7 | Code::CodeKey { rung: Doc, file: krep.c, decl: 4, sub: 0, line: 77 } |  |  | 0.704 |
| walker |  | 6239 | 11 | Code::CodeKey { rung: Doc, file: krep.c, decl: 2, sub: 0, line: 42 } |  |  | 0.704 |
| walker |  | 6253 | 14 | Code::CodeKey { rung: Doc, file: krep.c, decl: 3, sub: 0, line: 44 } |  |  | 0.704 |
| walker |  | 6269 | 16 | Code::CodeKey { rung: Doc, file: krep.c, decl: 1, sub: 0, line: 39 } |  |  | 0.704 |
| ns | 6272 |  | 258 | test/test_krep.c: complete roster of test functions, helpers and main | 4.10 |  | 0.685 |
| ns | 6497 |  | 225 | test/test_regex.c and test/test_multiple_patterns.c rosters | 4.11 |  | 0.669 |
| walker |  | 6498 | 229 | Code::CodeKey { rung: Names, file: krep.c, decl: 0, sub: 1, line: 0 } |  |  | 0.670 |
| walker |  | 6507 | 9 | Code::CodeKey { rung: Doc, file: krep.c, decl: 15, sub: 0, line: 93 } |  |  | 0.670 |
| walker |  | 6523 | 16 | Code::CodeKey { rung: Doc, file: krep.c, decl: 20, sub: 0, line: 116 } |  |  | 0.670 |
| ns | 6608 |  | 111 | test/test_directory.c roster: the separate recursive-search integration binary | 4.12 |  | 0.663 |
| walker |  | 6743 | 220 | Code::CodeKey { rung: Names, file: krep.c, decl: 0, sub: 2, line: 0 } |  |  | 0.666 |
| walker |  | 6748 | 5 | Code::CodeKey { rung: Decl, file: krep.c, decl: 28, sub: 0, line: 128 } |  |  | 0.666 |
| walker |  | 6753 | 5 | Code::CodeKey { rung: Decl, file: krep.c, decl: 29, sub: 0, line: 139 } |  |  | 0.666 |
| walker |  | 6758 | 5 | Code::CodeKey { rung: Decl, file: krep.c, decl: 30, sub: 0, line: 175 } |  |  | 0.666 |
| walker |  | 6763 | 5 | Code::CodeKey { rung: Decl, file: krep.c, decl: 31, sub: 0, line: 244 } |  |  | 0.666 |
| walker |  | 6768 | 5 | Code::CodeKey { rung: Decl, file: krep.c, decl: 32, sub: 0, line: 256 } |  |  | 0.666 |
| walker |  | 6778 | 10 | Code::CodeKey { rung: Doc, file: krep.c, decl: 29, sub: 0, line: 139 } |  |  | 0.666 |
| walker |  | 6789 | 11 | Code::CodeKey { rung: Doc, file: krep.c, decl: 28, sub: 0, line: 128 } |  |  | 0.666 |
| walker |  | 6801 | 12 | Code::CodeKey { rung: Doc, file: krep.c, decl: 27, sub: 0, line: 125 } |  |  | 0.666 |
| walker |  | 6814 | 13 | Code::CodeKey { rung: Doc, file: krep.c, decl: 31, sub: 0, line: 244 } |  |  | 0.666 |
| walker |  | 6832 | 18 | Code::CodeKey { rung: Doc, file: krep.c, decl: 30, sub: 0, line: 175 } |  |  | 0.666 |
| ns | 6841 |  | 233 | test/test_compat.h: the TESTING wrapper roster and its guards | 4.13 |  | 0.654 |
| walker |  | 6881 | 49 | Code::CodeKey { rung: Doc, file: krep.c, decl: 32, sub: 0, line: 256 } |  |  | 0.654 |
| ns | 6972 |  | 131 | test/test_krep.h: shared test helper declarations | 4.14 |  | 0.646 |
| ns | 7080 |  | 108 | Makefile compiler configuration: CC, CFLAGS, LDFLAGS, PREFIX | 5.1 |  | 0.640 |
| walker |  | 7093 | 212 | Code::CodeKey { rung: Names, file: krep.c, decl: 0, sub: 3, line: 0 } |  |  | 0.653 |
| walker |  | 7098 | 5 | Code::CodeKey { rung: Decl, file: krep.c, decl: 34, sub: 0, line: 363 } |  |  | 0.653 |
| walker |  | 7103 | 5 | Code::CodeKey { rung: Decl, file: krep.c, decl: 35, sub: 0, line: 401 } |  |  | 0.653 |
| walker |  | 7108 | 5 | Code::CodeKey { rung: Decl, file: krep.c, decl: 36, sub: 0, line: 420 } |  |  | 0.653 |
| walker |  | 7113 | 5 | Code::CodeKey { rung: Decl, file: krep.c, decl: 37, sub: 0, line: 438 } |  |  | 0.653 |
| walker |  | 7118 | 5 | Code::CodeKey { rung: Decl, file: krep.c, decl: 38, sub: 0, line: 461 } |  |  | 0.653 |
| walker |  | 7124 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 39, sub: 0, line: 1084 } |  |  | 0.653 |
| walker |  | 7161 | 37 | Code::CodeKey { rung: Decl, file: krep.c, decl: 33, sub: 0, line: 329 } |  |  | 0.653 |
| walker |  | 7177 | 16 | Code::CodeKey { rung: Doc, file: krep.c, decl: 35, sub: 0, line: 401 } |  |  | 0.653 |
| walker |  | 7195 | 18 | Code::CodeKey { rung: Doc, file: krep.c, decl: 36, sub: 0, line: 420 } |  |  | 0.653 |
| walker |  | 7215 | 20 | Code::CodeKey { rung: Doc, file: krep.c, decl: 33, sub: 0, line: 329 } |  |  | 0.653 |
| walker |  | 7247 | 32 | Code::CodeKey { rung: Doc, file: krep.c, decl: 37, sub: 0, line: 438 } |  |  | 0.653 |
| walker |  | 7284 | 37 | Code::CodeKey { rung: Doc, file: krep.c, decl: 39, sub: 0, line: 1084 } |  |  | 0.653 |
| ns | 7316 |  | 236 | .github/workflows/ci.yml in full | 5.2 |  | 0.636 |
| walker |  | 7324 | 40 | Code::CodeKey { rung: Doc, file: krep.c, decl: 34, sub: 0, line: 363 } |  |  | 0.636 |
| walker |  | 7564 | 240 | Code::CodeKey { rung: Names, file: krep.c, decl: 0, sub: 4, line: 0 } |  |  | 0.650 |
| walker |  | 7570 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 40, sub: 0, line: 1125 } |  |  | 0.650 |
| walker |  | 7576 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 41, sub: 0, line: 1137 } |  |  | 0.650 |
| ns | 7580 |  | 264 | Makefile architecture detection: which SIMD flags each arch gets | 5.3 |  | 0.637 |
| walker |  | 7582 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 42, sub: 0, line: 1198 } |  |  | 0.637 |
| walker |  | 7588 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 43, sub: 0, line: 1213 } |  |  | 0.637 |
| walker |  | 7594 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 46, sub: 0, line: 1585 } |  |  | 0.637 |
| walker |  | 7600 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 48, sub: 0, line: 1771 } |  |  | 0.637 |
| walker |  | 7606 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 49, sub: 0, line: 1873 } |  |  | 0.637 |
| walker |  | 7647 | 41 | Code::CodeKey { rung: Decl, file: krep.c, decl: 45, sub: 0, line: 1389 } |  |  | 0.637 |
| walker |  | 7697 | 50 | Code::CodeKey { rung: Decl, file: krep.c, decl: 44, sub: 0, line: 1259 } |  |  | 0.637 |
| walker |  | 7747 | 50 | Code::CodeKey { rung: Decl, file: krep.c, decl: 47, sub: 0, line: 1628 } |  |  | 0.637 |
| walker |  | 7757 | 10 | Code::CodeKey { rung: Doc, file: krep.c, decl: 41, sub: 0, line: 1137 } |  |  | 0.637 |
| walker |  | 7768 | 11 | Code::CodeKey { rung: Doc, file: krep.c, decl: 40, sub: 0, line: 1125 } |  |  | 0.637 |
| walker |  | 7783 | 15 | Code::CodeKey { rung: Doc, file: krep.c, decl: 43, sub: 0, line: 1213 } |  |  | 0.637 |
| walker |  | 7800 | 17 | Code::CodeKey { rung: Doc, file: krep.c, decl: 42, sub: 0, line: 1198 } |  |  | 0.637 |
| walker |  | 7819 | 19 | Code::CodeKey { rung: Doc, file: krep.c, decl: 49, sub: 0, line: 1873 } |  |  | 0.637 |
| walker |  | 7855 | 36 | Code::CodeKey { rung: Doc, file: krep.c, decl: 44, sub: 0, line: 1259 } |  |  | 0.637 |
| ns | 7883 |  | 303 | Makefile compile/link rules and the parallel -DTESTING build | 5.4 |  | 0.627 |
| walker |  | 7911 | 56 | Code::CodeKey { rung: Doc, file: krep.c, decl: 47, sub: 0, line: 1628 } |  |  | 0.627 |
| walker |  | 7970 | 59 | Code::CodeKey { rung: Doc, file: krep.c, decl: 46, sub: 0, line: 1585 } |  |  | 0.627 |
| ns | 8052 |  | 169 | Makefile run targets: test, test-directory, ci, bench-rg, all-tests | 5.5 |  | 0.617 |
| ns | 8209 |  | 157 | .github/workflows/release.yml: tag trigger and release artifacts | 5.6 |  | 0.606 |
| walker |  | 8251 | 281 | Code::CodeKey { rung: Names, file: krep.c, decl: 0, sub: 5, line: 0 } |  |  | 0.619 |
| walker |  | 8257 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 50, sub: 0, line: 1919 } |  |  | 0.619 |
| walker |  | 8263 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 51, sub: 0, line: 1964 } |  |  | 0.619 |
| walker |  | 8269 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 52, sub: 0, line: 1999 } |  |  | 0.619 |
| walker |  | 8275 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 54, sub: 0, line: 2252 } |  |  | 0.619 |
| walker |  | 8281 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 55, sub: 0, line: 2265 } |  |  | 0.619 |
| walker |  | 8287 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 56, sub: 0, line: 2274 } |  |  | 0.619 |
| walker |  | 8293 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 57, sub: 0, line: 3071 } |  |  | 0.619 |
| walker |  | 8299 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 58, sub: 0, line: 3090 } |  |  | 0.619 |
| walker |  | 8305 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 59, sub: 0, line: 3122 } |  |  | 0.619 |
| walker |  | 8311 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 62, sub: 0, line: 3163 } |  |  | 0.619 |
| walker |  | 8317 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 63, sub: 0, line: 3180 } |  |  | 0.619 |
| walker |  | 8351 | 34 | Code::CodeKey { rung: Decl, file: krep.c, decl: 60, sub: 0, line: 3146 } |  |  | 0.619 |
| walker |  | 8408 | 57 | Code::CodeKey { rung: Decl, file: krep.c, decl: 61, sub: 0, line: 3154 } |  |  | 0.619 |
| walker |  | 8418 | 10 | Code::CodeKey { rung: Doc, file: krep.c, decl: 53, sub: 0, line: 2249 } |  |  | 0.619 |
| walker |  | 8431 | 13 | Code::CodeKey { rung: Doc, file: krep.c, decl: 55, sub: 0, line: 2265 } |  |  | 0.619 |
| walker |  | 8445 | 14 | Code::CodeKey { rung: Doc, file: krep.c, decl: 62, sub: 0, line: 3163 } |  |  | 0.619 |
| walker |  | 8460 | 15 | Code::CodeKey { rung: Doc, file: krep.c, decl: 57, sub: 0, line: 3071 } |  |  | 0.619 |
| ns | 8462 |  | 253 | krep.c constants: VERSION and every performance tunable | 6.1 |  | 0.623 |
| walker |  | 8475 | 15 | Code::CodeKey { rung: Doc, file: krep.c, decl: 58, sub: 0, line: 3090 } |  |  | 0.623 |
| walker |  | 8490 | 15 | Code::CodeKey { rung: Doc, file: krep.c, decl: 59, sub: 0, line: 3122 } |  |  | 0.623 |
| walker |  | 8505 | 15 | Code::CodeKey { rung: Doc, file: krep.c, decl: 60, sub: 0, line: 3146 } |  |  | 0.623 |
| walker |  | 8521 | 16 | Code::CodeKey { rung: Doc, file: krep.c, decl: 52, sub: 0, line: 1999 } |  |  | 0.623 |
| walker |  | 8537 | 16 | Code::CodeKey { rung: Doc, file: krep.c, decl: 54, sub: 0, line: 2252 } |  |  | 0.623 |
| walker |  | 8554 | 17 | Code::CodeKey { rung: Doc, file: krep.c, decl: 51, sub: 0, line: 1964 } |  |  | 0.623 |
| walker |  | 8572 | 18 | Code::CodeKey { rung: Doc, file: krep.c, decl: 63, sub: 0, line: 3180 } |  |  | 0.623 |
| walker |  | 8591 | 19 | Code::CodeKey { rung: Doc, file: krep.c, decl: 50, sub: 0, line: 1919 } |  |  | 0.623 |
| walker |  | 8611 | 20 | Code::CodeKey { rung: Doc, file: krep.c, decl: 61, sub: 0, line: 3154 } |  |  | 0.624 |
| ns | 8696 |  | 234 | krep.c global option state and the lower_table constructor | 6.2 |  | 0.624 |
| walker |  | 8914 | 303 | Code::CodeKey { rung: Names, file: krep.c, decl: 0, sub: 6, line: 0 } |  |  | 0.640 |
| walker |  | 8920 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 64, sub: 0, line: 3240 } |  |  | 0.640 |
| walker |  | 8926 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 65, sub: 0, line: 3272 } |  |  | 0.640 |
| walker |  | 8932 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 66, sub: 0, line: 3297 } |  |  | 0.640 |
| walker |  | 8938 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 68, sub: 0, line: 3442 } |  |  | 0.640 |
| walker |  | 8944 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 70, sub: 0, line: 4046 } |  |  | 0.640 |
| walker |  | 8950 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 71, sub: 0, line: 4104 } |  |  | 0.640 |
| walker |  | 8956 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 72, sub: 0, line: 4209 } |  |  | 0.640 |
| walker |  | 8962 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 73, sub: 0, line: 4251 } |  |  | 0.640 |
| ns | 8966 |  | 270 | main: the getopt_long table and short-option string | 6.3 |  | 0.632 |
| walker |  | 8968 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 74, sub: 0, line: 4313 } |  |  | 0.632 |
| walker |  | 8974 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 75, sub: 0, line: 4332 } |  |  | 0.632 |
| walker |  | 8998 | 24 | Code::CodeKey { rung: Decl, file: krep.c, decl: 67, sub: 0, line: 3310 } |  |  | 0.632 |
| walker |  | 9039 | 41 | Code::CodeKey { rung: Decl, file: krep.c, decl: 69, sub: 0, line: 3891 } |  |  | 0.632 |
| walker |  | 9080 | 41 | Code::CodeKey { rung: Decl, file: krep.c, decl: 76, sub: 0, line: 4371 } |  |  | 0.632 |
| walker |  | 9091 | 11 | Code::CodeKey { rung: Doc, file: krep.c, decl: 75, sub: 0, line: 4332 } |  |  | 0.632 |
| walker |  | 9104 | 13 | Code::CodeKey { rung: Doc, file: krep.c, decl: 69, sub: 0, line: 3891 } |  |  | 0.632 |
| walker |  | 9117 | 13 | Code::CodeKey { rung: Doc, file: krep.c, decl: 74, sub: 0, line: 4313 } |  |  | 0.632 |
| walker |  | 9131 | 14 | Code::CodeKey { rung: Doc, file: krep.c, decl: 72, sub: 0, line: 4209 } |  |  | 0.632 |
| walker |  | 9146 | 15 | Code::CodeKey { rung: Doc, file: krep.c, decl: 67, sub: 0, line: 3310 } |  |  | 0.632 |
| walker |  | 9161 | 15 | Code::CodeKey { rung: Doc, file: krep.c, decl: 73, sub: 0, line: 4251 } |  |  | 0.632 |
| walker |  | 9177 | 16 | Code::CodeKey { rung: Doc, file: krep.c, decl: 70, sub: 0, line: 4046 } |  |  | 0.632 |
| walker |  | 9195 | 18 | Code::CodeKey { rung: Doc, file: krep.c, decl: 71, sub: 0, line: 4104 } |  |  | 0.632 |
| walker |  | 9215 | 20 | Code::CodeKey { rung: Doc, file: krep.c, decl: 65, sub: 0, line: 3272 } |  |  | 0.632 |
| walker |  | 9236 | 21 | Code::CodeKey { rung: Doc, file: krep.c, decl: 66, sub: 0, line: 3297 } |  |  | 0.632 |
| walker |  | 9257 | 21 | Code::CodeKey { rung: Doc, file: krep.c, decl: 76, sub: 0, line: 4371 } |  |  | 0.633 |
| walker |  | 9281 | 24 | Code::CodeKey { rung: Doc, file: krep.c, decl: 64, sub: 0, line: 3240 } |  |  | 0.633 |
| ns | 9299 |  | 333 | select_search_algorithm: dispatch head through the short-pattern branch | 6.4 |  | 0.619 |
| walker |  | 9316 | 35 | Code::CodeKey { rung: Doc, file: krep.c, decl: 68, sub: 0, line: 3442 } |  |  | 0.619 |
| walker |  | 9501 | 185 | Markdown::Section { file: README.md, section_index: 9, keeps_default_concavity: false } |  |  | 0.631 |
| ns | 9685 |  | 386 | select_search_algorithm: SIMD length limits and the KMP/Boyer-Moore fallback | 6.5 | 6.4 | 0.616 |
| walker |  | 9699 | 198 | Markdown::Section { file: README.md, section_index: 10, keeps_default_concavity: true } |  |  | 0.633 |
| walker |  | 9828 | 129 | Markdown::Section { file: README.md, section_index: 11, keeps_default_concavity: false } |  |  | 0.643 |
| ns | 9868 |  | 183 | gitignore data model: pattern record and parent-chained context | 6.6 |  | 0.648 |
| ns | 9993 |  | 125 | Licence header, dependabot config and .gitignore | 7.1 |  | 0.642 |
