Score(3000)=0.604 I=0.828 C=0.441 ns_rows≤3K=20/56 grid(1000/1442/2080/3000/4327/6240/9000)=0.638/0.725/0.723/0.604/0.742/0.720/0.657

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
| walker |  | 1159 | 64 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.648 |
| ns | 1173 |  | 177 | README Key Features, first half | 2.1 |  | 0.669 |
| walker |  | 1203 | 44 | Markdown::Section { file: README.md, section_index: 14, keeps_default_concavity: false } |  |  | 0.669 |
| ns | 1343 |  | 170 | README Key Features, second half | 2.2 |  | 0.685 |
| walker |  | 1387 | 184 | Code::CodeKey { rung: Names, file: aho_corasick.h, decl: 0, sub: 0, line: 0 } |  |  | 0.710 |
| walker |  | 1397 | 10 | Code::CodeKey { rung: Doc, file: aho_corasick.h, decl: 5, sub: 0, line: 23 } |  |  | 0.710 |
| walker |  | 1409 | 12 | Code::CodeKey { rung: Doc, file: aho_corasick.h, decl: 9, sub: 0, line: 33 } |  |  | 0.716 |
| walker |  | 1421 | 12 | Code::CodeKey { rung: Doc, file: aho_corasick.h, decl: 11, sub: 0, line: 39 } |  |  | 0.725 |
| walker |  | 1435 | 14 | Code::CodeKey { rung: Doc, file: aho_corasick.h, decl: 6, sub: 0, line: 26 } |  |  | 0.725 |
| walker |  | 1450 | 15 | Code::CodeKey { rung: Doc, file: aho_corasick.h, decl: 8, sub: 0, line: 30 } |  |  | 0.736 |
| walker |  | 1468 | 18 | Code::CodeKey { rung: Doc, file: aho_corasick.h, decl: 1, sub: 0, line: 14 } |  |  | 0.736 |
| walker |  | 1488 | 20 | Code::CodeKey { rung: Doc, file: aho_corasick.h, decl: 10, sub: 0, line: 36 } |  |  | 0.752 |
| walker |  | 1523 | 35 | Code::CodeKey { rung: Doc, file: aho_corasick.h, decl: 3, sub: 0, line: 19 } |  |  | 0.752 |
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
| walker |  | 4214 | 116 | Code::CodeKey { rung: Doc, file: krep.h, decl: 25, sub: 0, line: 161 } |  |  | 0.737 |
| ns | 4236 |  | 225 | ANSI colour macro block, output palette and help palette | 3.9 |  | 0.742 |
| ns | 4395 |  | 159 | Documented contracts of the three entry points and the printer | 3.10 | 1.7 | 0.738 |
| walker |  | 4411 | 197 | Code::CodeKey { rung: Names, file: krep.h, decl: 0, sub: 3, line: 0 } |  |  | 0.747 |
| walker |  | 4418 | 7 | Code::CodeKey { rung: Doc, file: krep.h, decl: 33, sub: 0, line: 203 } |  |  | 0.747 |
| walker |  | 4600 | 182 | Code::CodeKey { rung: Names, file: krep.h, decl: 0, sub: 4, line: 0 } |  |  | 0.757 |
| walker |  | 4619 | 19 | Code::CodeKey { rung: Decl, file: krep.h, decl: 40, sub: 0, line: 218 } |  |  | 0.763 |
| ns | 4634 |  | 239 | krep.c file-level section banner map, all sixteen top-level banners | 4.1 |  | 0.739 |
| walker |  | 4638 | 19 | Code::CodeKey { rung: Decl, file: krep.h, decl: 39, sub: 0, line: 214 } |  |  | 0.746 |
| walker |  | 4657 | 19 | Code::CodeKey { rung: Decl, file: krep.h, decl: 42, sub: 0, line: 226 } |  |  | 0.752 |
| walker |  | 4681 | 24 | Code::CodeKey { rung: Decl, file: krep.h, decl: 41, sub: 0, line: 222 } |  |  | 0.763 |
| walker |  | 4689 | 8 | Code::CodeKey { rung: Doc, file: krep.h, decl: 43, sub: 0, line: 231 } |  |  | 0.763 |
| ns | 4758 |  | 124 | krep.c definition roster, lines 139-461: match results, line finding, printing | 4.2 |  | 0.749 |
| walker |  | 4848 | 159 | Code::CodeKey { rung: Names, file: krep.h, decl: 0, sub: 5, line: 0 } |  |  | 0.765 |
| walker |  | 4853 | 5 | Code::CodeKey { rung: Decl, file: krep.h, decl: 49, sub: 0, line: 298 } |  |  | 0.765 |
| walker |  | 4858 | 5 | Code::CodeKey { rung: Decl, file: krep.h, decl: 50, sub: 0, line: 312 } |  |  | 0.765 |
| walker |  | 4926 | 68 | Code::CodeKey { rung: Doc, file: krep.h, decl: 49, sub: 0, line: 298 } |  |  | 0.765 |
| ns | 4932 |  | 174 | krep.c definition roster, lines 1084-1964: utilities, the four scalar algorithms, orchestration | 4.3 |  | 0.748 |
| walker |  | 5023 | 97 | Code::CodeKey { rung: Doc, file: krep.h, decl: 47, sub: 0, line: 278 } |  |  | 0.748 |
| ns | 5124 |  | 192 | krep.c definition roster, lines 1999-3442: public API bodies, skip filters, gitignore | 4.4 |  | 0.730 |
| walker |  | 5125 | 102 | Code::CodeKey { rung: Doc, file: krep.h, decl: 48, sub: 0, line: 288 } |  |  | 0.730 |
| walker |  | 5232 | 107 | Code::CodeKey { rung: Doc, file: krep.h, decl: 50, sub: 0, line: 312 } |  |  | 0.730 |
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
| walker |  | 6198 | 185 | Markdown::Section { file: README.md, section_index: 9, keeps_default_concavity: false } |  |  | 0.720 |
| ns | 6272 |  | 258 | test/test_krep.c: complete roster of test functions, helpers and main | 4.10 |  | 0.700 |
| walker |  | 6396 | 198 | Markdown::Section { file: README.md, section_index: 10, keeps_default_concavity: true } |  |  | 0.723 |
| ns | 6497 |  | 225 | test/test_regex.c and test/test_multiple_patterns.c rosters | 4.11 |  | 0.707 |
| walker |  | 6525 | 129 | Markdown::Section { file: README.md, section_index: 11, keeps_default_concavity: false } |  |  | 0.721 |
| ns | 6608 |  | 111 | test/test_directory.c roster: the separate recursive-search integration binary | 4.12 |  | 0.713 |
| walker |  | 6733 | 208 | Code::CodeKey { rung: Names, file: krep.c, decl: 0, sub: 0, line: 0 } |  |  | 0.713 |
| walker |  | 6740 | 7 | Code::CodeKey { rung: Doc, file: krep.c, decl: 4, sub: 0, line: 77 } |  |  | 0.713 |
| walker |  | 6751 | 11 | Code::CodeKey { rung: Doc, file: krep.c, decl: 2, sub: 0, line: 42 } |  |  | 0.713 |
| walker |  | 6765 | 14 | Code::CodeKey { rung: Doc, file: krep.c, decl: 3, sub: 0, line: 44 } |  |  | 0.713 |
| walker |  | 6781 | 16 | Code::CodeKey { rung: Doc, file: krep.c, decl: 1, sub: 0, line: 39 } |  |  | 0.713 |
| ns | 6841 |  | 233 | test/test_compat.h: the TESTING wrapper roster and its guards | 4.13 |  | 0.701 |
| ns | 6972 |  | 131 | test/test_krep.h: shared test helper declarations | 4.14 |  | 0.691 |
| walker |  | 7010 | 229 | Code::CodeKey { rung: Names, file: krep.c, decl: 0, sub: 1, line: 0 } |  |  | 0.692 |
| walker |  | 7019 | 9 | Code::CodeKey { rung: Doc, file: krep.c, decl: 15, sub: 0, line: 93 } |  |  | 0.692 |
| walker |  | 7035 | 16 | Code::CodeKey { rung: Doc, file: krep.c, decl: 20, sub: 0, line: 116 } |  |  | 0.692 |
| ns | 7080 |  | 108 | Makefile compiler configuration: CC, CFLAGS, LDFLAGS, PREFIX | 5.1 |  | 0.687 |
| walker |  | 7255 | 220 | Code::CodeKey { rung: Names, file: krep.c, decl: 0, sub: 2, line: 0 } |  |  | 0.689 |
| walker |  | 7260 | 5 | Code::CodeKey { rung: Decl, file: krep.c, decl: 28, sub: 0, line: 128 } |  |  | 0.689 |
| walker |  | 7265 | 5 | Code::CodeKey { rung: Decl, file: krep.c, decl: 29, sub: 0, line: 139 } |  |  | 0.689 |
| walker |  | 7270 | 5 | Code::CodeKey { rung: Decl, file: krep.c, decl: 30, sub: 0, line: 175 } |  |  | 0.689 |
| walker |  | 7275 | 5 | Code::CodeKey { rung: Decl, file: krep.c, decl: 31, sub: 0, line: 244 } |  |  | 0.689 |
| walker |  | 7280 | 5 | Code::CodeKey { rung: Decl, file: krep.c, decl: 32, sub: 0, line: 256 } |  |  | 0.689 |
| walker |  | 7290 | 10 | Code::CodeKey { rung: Doc, file: krep.c, decl: 29, sub: 0, line: 139 } |  |  | 0.689 |
| walker |  | 7301 | 11 | Code::CodeKey { rung: Doc, file: krep.c, decl: 28, sub: 0, line: 128 } |  |  | 0.689 |
| walker |  | 7313 | 12 | Code::CodeKey { rung: Doc, file: krep.c, decl: 27, sub: 0, line: 125 } |  |  | 0.690 |
| ns | 7316 |  | 236 | .github/workflows/ci.yml in full | 5.2 |  | 0.672 |
| walker |  | 7326 | 13 | Code::CodeKey { rung: Doc, file: krep.c, decl: 31, sub: 0, line: 244 } |  |  | 0.672 |
| walker |  | 7344 | 18 | Code::CodeKey { rung: Doc, file: krep.c, decl: 30, sub: 0, line: 175 } |  |  | 0.672 |
| walker |  | 7393 | 49 | Code::CodeKey { rung: Doc, file: krep.c, decl: 32, sub: 0, line: 256 } |  |  | 0.672 |
| ns | 7580 |  | 264 | Makefile architecture detection: which SIMD flags each arch gets | 5.3 |  | 0.658 |
| walker |  | 7605 | 212 | Code::CodeKey { rung: Names, file: krep.c, decl: 0, sub: 3, line: 0 } |  |  | 0.671 |
| walker |  | 7610 | 5 | Code::CodeKey { rung: Decl, file: krep.c, decl: 34, sub: 0, line: 363 } |  |  | 0.671 |
| walker |  | 7615 | 5 | Code::CodeKey { rung: Decl, file: krep.c, decl: 35, sub: 0, line: 401 } |  |  | 0.671 |
| walker |  | 7620 | 5 | Code::CodeKey { rung: Decl, file: krep.c, decl: 36, sub: 0, line: 420 } |  |  | 0.671 |
| walker |  | 7625 | 5 | Code::CodeKey { rung: Decl, file: krep.c, decl: 37, sub: 0, line: 438 } |  |  | 0.671 |
| walker |  | 7630 | 5 | Code::CodeKey { rung: Decl, file: krep.c, decl: 38, sub: 0, line: 461 } |  |  | 0.671 |
| walker |  | 7636 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 39, sub: 0, line: 1084 } |  |  | 0.671 |
| walker |  | 7673 | 37 | Code::CodeKey { rung: Decl, file: krep.c, decl: 33, sub: 0, line: 329 } |  |  | 0.671 |
| walker |  | 7689 | 16 | Code::CodeKey { rung: Doc, file: krep.c, decl: 35, sub: 0, line: 401 } |  |  | 0.671 |
| walker |  | 7707 | 18 | Code::CodeKey { rung: Doc, file: krep.c, decl: 36, sub: 0, line: 420 } |  |  | 0.671 |
| walker |  | 7727 | 20 | Code::CodeKey { rung: Doc, file: krep.c, decl: 33, sub: 0, line: 329 } |  |  | 0.671 |
| walker |  | 7759 | 32 | Code::CodeKey { rung: Doc, file: krep.c, decl: 37, sub: 0, line: 438 } |  |  | 0.671 |
| walker |  | 7796 | 37 | Code::CodeKey { rung: Doc, file: krep.c, decl: 39, sub: 0, line: 1084 } |  |  | 0.671 |
| walker |  | 7836 | 40 | Code::CodeKey { rung: Doc, file: krep.c, decl: 34, sub: 0, line: 363 } |  |  | 0.671 |
| ns | 7883 |  | 303 | Makefile compile/link rules and the parallel -DTESTING build | 5.4 |  | 0.660 |
| ns | 8052 |  | 169 | Makefile run targets: test, test-directory, ci, bench-rg, all-tests | 5.5 |  | 0.650 |
| walker |  | 8076 | 240 | Code::CodeKey { rung: Names, file: krep.c, decl: 0, sub: 4, line: 0 } |  |  | 0.662 |
| walker |  | 8082 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 40, sub: 0, line: 1125 } |  |  | 0.662 |
| walker |  | 8088 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 41, sub: 0, line: 1137 } |  |  | 0.662 |
| walker |  | 8094 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 42, sub: 0, line: 1198 } |  |  | 0.662 |
| walker |  | 8100 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 43, sub: 0, line: 1213 } |  |  | 0.662 |
| walker |  | 8106 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 46, sub: 0, line: 1585 } |  |  | 0.662 |
| walker |  | 8112 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 48, sub: 0, line: 1771 } |  |  | 0.662 |
| walker |  | 8118 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 49, sub: 0, line: 1873 } |  |  | 0.662 |
| walker |  | 8159 | 41 | Code::CodeKey { rung: Decl, file: krep.c, decl: 45, sub: 0, line: 1389 } |  |  | 0.662 |
| walker |  | 8209 | 50 | Code::CodeKey { rung: Decl, file: krep.c, decl: 44, sub: 0, line: 1259 } |  |  | 0.650 |
| ns | 8209 |  | 157 | .github/workflows/release.yml: tag trigger and release artifacts | 5.6 |  | 0.650 |
| walker |  | 8259 | 50 | Code::CodeKey { rung: Decl, file: krep.c, decl: 47, sub: 0, line: 1628 } |  |  | 0.650 |
| walker |  | 8269 | 10 | Code::CodeKey { rung: Doc, file: krep.c, decl: 41, sub: 0, line: 1137 } |  |  | 0.650 |
| walker |  | 8280 | 11 | Code::CodeKey { rung: Doc, file: krep.c, decl: 40, sub: 0, line: 1125 } |  |  | 0.650 |
| walker |  | 8295 | 15 | Code::CodeKey { rung: Doc, file: krep.c, decl: 43, sub: 0, line: 1213 } |  |  | 0.650 |
| walker |  | 8312 | 17 | Code::CodeKey { rung: Doc, file: krep.c, decl: 42, sub: 0, line: 1198 } |  |  | 0.650 |
| walker |  | 8331 | 19 | Code::CodeKey { rung: Doc, file: krep.c, decl: 49, sub: 0, line: 1873 } |  |  | 0.650 |
| walker |  | 8367 | 36 | Code::CodeKey { rung: Doc, file: krep.c, decl: 44, sub: 0, line: 1259 } |  |  | 0.650 |
| walker |  | 8423 | 56 | Code::CodeKey { rung: Doc, file: krep.c, decl: 47, sub: 0, line: 1628 } |  |  | 0.650 |
| ns | 8462 |  | 253 | krep.c constants: VERSION and every performance tunable | 6.1 |  | 0.653 |
| walker |  | 8482 | 59 | Code::CodeKey { rung: Doc, file: krep.c, decl: 46, sub: 0, line: 1585 } |  |  | 0.653 |
| ns | 8696 |  | 234 | krep.c global option state and the lower_table constructor | 6.2 |  | 0.653 |
| walker |  | 8763 | 281 | Code::CodeKey { rung: Names, file: krep.c, decl: 0, sub: 5, line: 0 } |  |  | 0.665 |
| walker |  | 8769 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 50, sub: 0, line: 1919 } |  |  | 0.665 |
| walker |  | 8775 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 51, sub: 0, line: 1964 } |  |  | 0.665 |
| walker |  | 8781 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 52, sub: 0, line: 1999 } |  |  | 0.665 |
| walker |  | 8787 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 54, sub: 0, line: 2252 } |  |  | 0.665 |
| walker |  | 8793 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 55, sub: 0, line: 2265 } |  |  | 0.665 |
| walker |  | 8799 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 56, sub: 0, line: 2274 } |  |  | 0.665 |
| walker |  | 8805 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 57, sub: 0, line: 3071 } |  |  | 0.665 |
| walker |  | 8811 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 58, sub: 0, line: 3090 } |  |  | 0.665 |
| walker |  | 8817 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 59, sub: 0, line: 3122 } |  |  | 0.665 |
| walker |  | 8823 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 62, sub: 0, line: 3163 } |  |  | 0.665 |
| walker |  | 8829 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 63, sub: 0, line: 3180 } |  |  | 0.665 |
| walker |  | 8863 | 34 | Code::CodeKey { rung: Decl, file: krep.c, decl: 60, sub: 0, line: 3146 } |  |  | 0.665 |
| walker |  | 8920 | 57 | Code::CodeKey { rung: Decl, file: krep.c, decl: 61, sub: 0, line: 3154 } |  |  | 0.665 |
| walker |  | 8930 | 10 | Code::CodeKey { rung: Doc, file: krep.c, decl: 53, sub: 0, line: 2249 } |  |  | 0.665 |
| walker |  | 8943 | 13 | Code::CodeKey { rung: Doc, file: krep.c, decl: 55, sub: 0, line: 2265 } |  |  | 0.665 |
| walker |  | 8957 | 14 | Code::CodeKey { rung: Doc, file: krep.c, decl: 62, sub: 0, line: 3163 } |  |  | 0.665 |
| ns | 8966 |  | 270 | main: the getopt_long table and short-option string | 6.3 |  | 0.657 |
| walker |  | 8972 | 15 | Code::CodeKey { rung: Doc, file: krep.c, decl: 57, sub: 0, line: 3071 } |  |  | 0.657 |
| walker |  | 8987 | 15 | Code::CodeKey { rung: Doc, file: krep.c, decl: 58, sub: 0, line: 3090 } |  |  | 0.657 |
| walker |  | 9002 | 15 | Code::CodeKey { rung: Doc, file: krep.c, decl: 59, sub: 0, line: 3122 } |  |  | 0.657 |
| walker |  | 9017 | 15 | Code::CodeKey { rung: Doc, file: krep.c, decl: 60, sub: 0, line: 3146 } |  |  | 0.658 |
| walker |  | 9033 | 16 | Code::CodeKey { rung: Doc, file: krep.c, decl: 52, sub: 0, line: 1999 } |  |  | 0.658 |
| walker |  | 9049 | 16 | Code::CodeKey { rung: Doc, file: krep.c, decl: 54, sub: 0, line: 2252 } |  |  | 0.658 |
| walker |  | 9066 | 17 | Code::CodeKey { rung: Doc, file: krep.c, decl: 51, sub: 0, line: 1964 } |  |  | 0.658 |
| walker |  | 9084 | 18 | Code::CodeKey { rung: Doc, file: krep.c, decl: 63, sub: 0, line: 3180 } |  |  | 0.658 |
| walker |  | 9103 | 19 | Code::CodeKey { rung: Doc, file: krep.c, decl: 50, sub: 0, line: 1919 } |  |  | 0.658 |
| walker |  | 9123 | 20 | Code::CodeKey { rung: Doc, file: krep.c, decl: 61, sub: 0, line: 3154 } |  |  | 0.658 |
| ns | 9299 |  | 333 | select_search_algorithm: dispatch head through the short-pattern branch | 6.4 |  | 0.644 |
| walker |  | 9426 | 303 | Code::CodeKey { rung: Names, file: krep.c, decl: 0, sub: 6, line: 0 } |  |  | 0.659 |
| walker |  | 9432 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 64, sub: 0, line: 3240 } |  |  | 0.659 |
| walker |  | 9438 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 65, sub: 0, line: 3272 } |  |  | 0.659 |
| walker |  | 9444 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 66, sub: 0, line: 3297 } |  |  | 0.659 |
| walker |  | 9450 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 68, sub: 0, line: 3442 } |  |  | 0.659 |
| walker |  | 9456 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 70, sub: 0, line: 4046 } |  |  | 0.659 |
| walker |  | 9462 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 71, sub: 0, line: 4104 } |  |  | 0.659 |
| walker |  | 9468 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 72, sub: 0, line: 4209 } |  |  | 0.659 |
| walker |  | 9474 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 73, sub: 0, line: 4251 } |  |  | 0.659 |
| walker |  | 9480 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 74, sub: 0, line: 4313 } |  |  | 0.659 |
| walker |  | 9486 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 75, sub: 0, line: 4332 } |  |  | 0.659 |
| walker |  | 9510 | 24 | Code::CodeKey { rung: Decl, file: krep.c, decl: 67, sub: 0, line: 3310 } |  |  | 0.659 |
| walker |  | 9551 | 41 | Code::CodeKey { rung: Decl, file: krep.c, decl: 69, sub: 0, line: 3891 } |  |  | 0.659 |
| walker |  | 9592 | 41 | Code::CodeKey { rung: Decl, file: krep.c, decl: 76, sub: 0, line: 4371 } |  |  | 0.659 |
| walker |  | 9603 | 11 | Code::CodeKey { rung: Doc, file: krep.c, decl: 75, sub: 0, line: 4332 } |  |  | 0.659 |
| walker |  | 9616 | 13 | Code::CodeKey { rung: Doc, file: krep.c, decl: 69, sub: 0, line: 3891 } |  |  | 0.659 |
| walker |  | 9629 | 13 | Code::CodeKey { rung: Doc, file: krep.c, decl: 74, sub: 0, line: 4313 } |  |  | 0.659 |
| walker |  | 9643 | 14 | Code::CodeKey { rung: Doc, file: krep.c, decl: 72, sub: 0, line: 4209 } |  |  | 0.659 |
| walker |  | 9658 | 15 | Code::CodeKey { rung: Doc, file: krep.c, decl: 67, sub: 0, line: 3310 } |  |  | 0.659 |
| walker |  | 9673 | 15 | Code::CodeKey { rung: Doc, file: krep.c, decl: 73, sub: 0, line: 4251 } |  |  | 0.659 |
| ns | 9685 |  | 386 | select_search_algorithm: SIMD length limits and the KMP/Boyer-Moore fallback | 6.5 | 6.4 | 0.643 |
| walker |  | 9689 | 16 | Code::CodeKey { rung: Doc, file: krep.c, decl: 70, sub: 0, line: 4046 } |  |  | 0.643 |
| walker |  | 9707 | 18 | Code::CodeKey { rung: Doc, file: krep.c, decl: 71, sub: 0, line: 4104 } |  |  | 0.643 |
| walker |  | 9727 | 20 | Code::CodeKey { rung: Doc, file: krep.c, decl: 65, sub: 0, line: 3272 } |  |  | 0.643 |
| walker |  | 9748 | 21 | Code::CodeKey { rung: Doc, file: krep.c, decl: 66, sub: 0, line: 3297 } |  |  | 0.643 |
| walker |  | 9769 | 21 | Code::CodeKey { rung: Doc, file: krep.c, decl: 76, sub: 0, line: 4371 } |  |  | 0.643 |
| walker |  | 9793 | 24 | Code::CodeKey { rung: Doc, file: krep.c, decl: 64, sub: 0, line: 3240 } |  |  | 0.643 |
| walker |  | 9828 | 35 | Code::CodeKey { rung: Doc, file: krep.c, decl: 68, sub: 0, line: 3442 } |  |  | 0.643 |
| ns | 9868 |  | 183 | gitignore data model: pattern record and parent-chained context | 6.6 |  | 0.648 |
| ns | 9993 |  | 125 | Licence header, dependabot config and .gitignore | 7.1 |  | 0.642 |
