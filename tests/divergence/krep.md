Score(3000)=0.694 I=0.845 C=0.570 ns_rows≤3K=20/56 grid(1000/1442/2080/3000/4327/6240/9000)=0.644/0.692/0.674/0.694/0.804/0.726/0.665

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 35 | 35 | Fs::DirListing { dir: . } |  |  | 0.000 |
| ns | 66 |  | 66 | Repository identity: title line and the one-paragraph what-it-is | 1.1 |  | 0.000 |
| walker |  | 101 | 66 | Markdown::ReadmeHeadline { file: README.md } |  |  | 1.000 |
| ns | 101 |  | 35 | Complete root directory listing | 1.2 |  | 1.000 |
| walker |  | 114 | 13 | Fs::DirListing { dir: .github } |  |  | 1.000 |
| walker |  | 122 | 8 | Fs::DirListing { dir: .github/workflows } |  |  | 1.000 |
| ns | 164 |  | 63 | Complete listings of test/, .github/ and .github/workflows/ | 1.3 |  | 0.705 |
| walker |  | 168 | 46 | Plaintext::Whole { file: Makefile } |  |  | 0.705 |
| walker |  | 210 | 42 | Fs::DirListing { dir: test } |  |  | 1.000 |
| ns | 215 |  | 51 | Makefile: binary name, source list, and the complete .PHONY target list | 1.4 |  | 0.929 |
| ns | 333 |  | 118 | README invocation synopsis: all six supported command forms | 1.5 |  | 0.805 |
| ns | 463 |  | 130 | Every H2 heading location in README.md | 1.6 |  | 0.668 |
| walker |  | 492 | 282 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.850 |
| ns | 559 |  | 96 | krep.h public API roster: the three search entry points, match-result management, printing | 1.7 |  | 0.762 |
| walker |  | 594 | 102 | Markdown::CommandBlock { file: README.md, row: 54 } |  |  | 0.765 |
| walker |  | 709 | 115 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.768 |
| ns | 789 |  | 230 | krep.h search-algorithm roster with the SIMD #if guards intact | 1.8 |  | 0.631 |
| walker |  | 841 | 132 | Markdown::Section { file: README.md, section_index: 5, keeps_default_concavity: true } |  |  | 0.709 |
| ns | 996 |  | 207 | aho_corasick.h: the complete second-module interface | 1.9 |  | 0.640 |
| ns | 1173 |  | 177 | README Key Features, first half | 2.1 |  | 0.612 |
| walker |  | 1193 | 352 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: true } |  |  | 0.671 |
| walker |  | 1257 | 64 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.671 |
| ns | 1343 |  | 170 | README Key Features, second half | 2.2 |  | 0.687 |
| ns | 1534 |  | 191 | README command-line options table, first half (-i through -F) | 2.3 |  | 0.658 |
| walker |  | 1672 | 415 | Markdown::Section { file: README.md, section_index: 7, keeps_default_concavity: true } |  |  | 0.715 |
| ns | 1753 |  | 219 | README command-line options table, second half (-r through -h) | 2.4 |  | 0.730 |
| walker |  | 1794 | 122 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.730 |
| ns | 1963 |  | 210 | README: the documented smart algorithm-selection policy | 2.5 |  | 0.691 |
| walker |  | 1985 | 191 | Code::CodeKey { rung: Names, file: krep.h, decl: 0, sub: 0, line: 0 } |  |  | 0.691 |
| walker |  | 1999 | 14 | Code::CodeKey { rung: Doc, file: krep.h, decl: 1, sub: 0, line: 19 } |  |  | 0.691 |
| walker |  | 2013 | 14 | Code::CodeKey { rung: Doc, file: krep.h, decl: 2, sub: 0, line: 22 } |  |  | 0.691 |
| walker |  | 2033 | 20 | Code::CodeKey { rung: Doc, file: krep.h, decl: 4, sub: 0, line: 34 } |  |  | 0.691 |
| ns | 2069 |  | 106 | README usage examples: the six concrete invocations, command lines only | 2.6 |  | 0.674 |
| ns | 2181 |  | 112 | README: the multi-threading architecture section | 2.7 |  | 0.647 |
| walker |  | 2230 | 197 | Code::CodeKey { rung: Names, file: krep.h, decl: 0, sub: 1, line: 0 } |  |  | 0.649 |
| walker |  | 2262 | 32 | Code::CodeKey { rung: Decl, file: krep.h, decl: 17, sub: 0, line: 98 } |  |  | 0.650 |
| ns | 2287 |  | 106 | README: the recursive-search skipping rules | 2.8 |  | 0.631 |
| walker |  | 2302 | 40 | Code::CodeKey { rung: Decl, file: krep.h, decl: 14, sub: 0, line: 49 } |  |  | 0.632 |
| walker |  | 2364 | 62 | Code::CodeKey { rung: Decl, file: krep.h, decl: 15, sub: 0, line: 55 } |  |  | 0.634 |
| walker |  | 2373 | 9 | Code::CodeKey { rung: Doc, file: krep.h, decl: 10, sub: 0, line: 42 } |  |  | 0.634 |
| ns | 2402 |  | 115 | README scope note: explicitly not a grep/ripgrep replacement | 2.9 |  | 0.641 |
| ns | 2512 |  | 110 | README install and build-from-source commands | 2.10 |  | 0.638 |
| walker |  | 2741 | 368 | Code::CodeKey { rung: Decl, file: krep.h, decl: 16, sub: 0, line: 65 } |  |  | 0.649 |
| walker |  | 2754 | 13 | Code::CodeKey { rung: Doc, file: krep.h, decl: 16, sub: 0, line: 65 } |  |  | 0.649 |
| walker |  | 2777 | 23 | Code::CodeKey { rung: Doc, file: krep.h, decl: 14, sub: 0, line: 49 } |  |  | 0.650 |
| ns | 2902 |  | 390 | search_params_t: the struct every search function takes | 3.1 |  | 0.688 |
| walker |  | 2986 | 209 | Code::CodeKey { rung: Names, file: krep.h, decl: 0, sub: 2, line: 0 } |  |  | 0.694 |
| walker |  | 3037 | 51 | Code::CodeKey { rung: Decl, file: krep.h, decl: 19, sub: 0, line: 125 } |  |  | 0.694 |
| ns | 3089 |  | 187 | match_position_t and match_result_t | 3.2 |  | 0.707 |
| ns | 3180 |  | 91 | search_func_t: the algorithm function-pointer type | 3.3 |  | 0.702 |
| walker |  | 3198 | 161 | Code::CodeKey { rung: Decl, file: krep.h, decl: 20, sub: 0, line: 132 } |  |  | 0.702 |
| walker |  | 3207 | 9 | Code::CodeKey { rung: Doc, file: krep.h, decl: 19, sub: 0, line: 125 } |  |  | 0.702 |
| walker |  | 3216 | 9 | Code::CodeKey { rung: Doc, file: krep.h, decl: 21, sub: 0, line: 146 } |  |  | 0.702 |
| ns | 3282 |  | 102 | krep.h thread-pool API: all four functions in full | 3.4 |  | 0.708 |
| walker |  | 3466 | 250 | Code::CodeKey { rung: Decl, file: krep.h, decl: 18, sub: 0, line: 104 } |  |  | 0.713 |
| walker |  | 3476 | 10 | Code::CodeKey { rung: Doc, file: krep.h, decl: 18, sub: 0, line: 104 } |  |  | 0.714 |
| ns | 3562 |  | 280 | thread_data_t: per-thread search state, including the false-sharing padding | 3.5 |  | 0.727 |
| walker |  | 3653 | 177 | Code::CodeKey { rung: Names, file: krep.h, decl: 0, sub: 3, line: 0 } |  |  | 0.757 |
| ns | 3659 |  | 97 | krep.h helper roster: line finding, word matching, table preparation | 3.6 |  | 0.743 |
| walker |  | 3660 | 7 | Code::CodeKey { rung: Doc, file: krep.h, decl: 33, sub: 0, line: 203 } |  |  | 0.743 |
| walker |  | 3670 | 10 | Code::CodeKey { rung: Doc, file: krep.h, decl: 28, sub: 0, line: 183 } |  |  | 0.743 |
| walker |  | 3813 | 143 | Code::CodeKey { rung: Names, file: krep.h, decl: 0, sub: 4, line: 0 } |  |  | 0.748 |
| ns | 3826 |  | 167 | skip_directories: the complete recursive-search directory blocklist | 3.7 |  | 0.733 |
| walker |  | 3965 | 152 | Code::CodeKey { rung: Names, file: krep.h, decl: 0, sub: 5, line: 0 } |  |  | 0.744 |
| walker |  | 3984 | 19 | Code::CodeKey { rung: Decl, file: krep.h, decl: 40, sub: 0, line: 218 } |  |  | 0.749 |
| walker |  | 4003 | 19 | Code::CodeKey { rung: Decl, file: krep.h, decl: 39, sub: 0, line: 214 } |  |  | 0.756 |
| ns | 4011 |  | 185 | skip_extensions: head, ellipsis, and the count line | 3.8 |  | 0.743 |
| walker |  | 4022 | 19 | Code::CodeKey { rung: Decl, file: krep.h, decl: 42, sub: 0, line: 226 } |  |  | 0.750 |
| walker |  | 4046 | 24 | Code::CodeKey { rung: Decl, file: krep.h, decl: 41, sub: 0, line: 222 } |  |  | 0.762 |
| walker |  | 4060 | 14 | Code::CodeKey { rung: Doc, file: krep.h, decl: 39, sub: 0, line: 214 } |  |  | 0.773 |
| ns | 4236 |  | 225 | ANSI colour macro block, output palette and help palette | 3.9 |  | 0.777 |
| walker |  | 4249 | 189 | Code::CodeKey { rung: Names, file: krep.h, decl: 0, sub: 6, line: 0 } |  |  | 0.795 |
| walker |  | 4254 | 5 | Code::CodeKey { rung: Decl, file: krep.h, decl: 49, sub: 0, line: 298 } |  |  | 0.795 |
| walker |  | 4259 | 5 | Code::CodeKey { rung: Decl, file: krep.h, decl: 50, sub: 0, line: 312 } |  |  | 0.795 |
| walker |  | 4267 | 8 | Code::CodeKey { rung: Doc, file: krep.h, decl: 43, sub: 0, line: 231 } |  |  | 0.795 |
| walker |  | 4304 | 37 | Code::CodeKey { rung: Doc, file: krep.h, decl: 17, sub: 0, line: 98 } |  |  | 0.804 |
| ns | 4395 |  | 159 | Documented contracts of the three entry points and the printer | 3.10 | 1.7 | 0.789 |
| walker |  | 4494 | 190 | Markdown::Section { file: README.md, section_index: 4, keeps_default_concavity: false } |  |  | 0.802 |
| walker |  | 4562 | 68 | Code::CodeKey { rung: Doc, file: krep.h, decl: 49, sub: 0, line: 298 } |  |  | 0.802 |
| ns | 4634 |  | 239 | krep.c file-level section banner map, all sixteen top-level banners | 4.1 |  | 0.778 |
| walker |  | 4747 | 185 | Markdown::Section { file: README.md, section_index: 9, keeps_default_concavity: false } |  |  | 0.798 |
| ns | 4758 |  | 124 | krep.c definition roster, lines 139-461: match results, line finding, printing | 4.2 |  | 0.783 |
| ns | 4932 |  | 174 | krep.c definition roster, lines 1084-1964: utilities, the four scalar algorithms, orchestration | 4.3 |  | 0.765 |
| walker |  | 4945 | 198 | Markdown::Section { file: README.md, section_index: 10, keeps_default_concavity: false } |  |  | 0.793 |
| walker |  | 5074 | 129 | Markdown::Section { file: README.md, section_index: 11, keeps_default_concavity: false } |  |  | 0.810 |
| ns | 5124 |  | 192 | krep.c definition roster, lines 1999-3442: public API bodies, skip filters, gitignore | 4.4 |  | 0.791 |
| walker |  | 5164 | 90 | Code::CodeKey { rung: Doc, file: krep.h, decl: 26, sub: 0, line: 170 } |  |  | 0.791 |
| walker |  | 5352 | 188 | Code::CodeKey { rung: Names, file: krep.c, decl: 0, sub: 0, line: 0 } |  |  | 0.791 |
| ns | 5363 |  | 239 | krep.c definition roster, lines 3451-5108: main, thread pool, and the guarded SIMD kernels | 4.5 |  | 0.770 |
| walker |  | 5369 | 17 | Code::CodeKey { rung: Decl, file: krep.c, decl: 1, sub: 0, line: 9 } |  |  | 0.770 |
| walker |  | 5376 | 7 | Code::CodeKey { rung: Doc, file: krep.c, decl: 5, sub: 0, line: 77 } |  |  | 0.770 |
| walker |  | 5387 | 11 | Code::CodeKey { rung: Doc, file: krep.c, decl: 3, sub: 0, line: 42 } |  |  | 0.770 |
| walker |  | 5401 | 14 | Code::CodeKey { rung: Doc, file: krep.c, decl: 4, sub: 0, line: 44 } |  |  | 0.770 |
| walker |  | 5417 | 16 | Code::CodeKey { rung: Doc, file: krep.c, decl: 2, sub: 0, line: 39 } |  |  | 0.770 |
| walker |  | 5438 | 21 | Code::CodeKey { rung: Doc, file: krep.c, decl: 1, sub: 0, line: 9 } |  |  | 0.770 |
| ns | 5546 |  | 183 | search_file internal section map, all eleven banners inside lines 2274-3070 | 4.6 |  | 0.757 |
| walker |  | 5633 | 195 | Code::CodeKey { rung: Names, file: krep.c, decl: 0, sub: 1, line: 0 } |  |  | 0.758 |
| walker |  | 5645 | 12 | Code::CodeKey { rung: Decl, file: krep.c, decl: 12, sub: 0, line: 84 } |  |  | 0.758 |
| walker |  | 5654 | 9 | Code::CodeKey { rung: Doc, file: krep.c, decl: 17, sub: 0, line: 93 } |  |  | 0.758 |
| ns | 5668 |  | 122 | print_matching_items internal section map: the -o and full-line output modes | 4.7 |  | 0.749 |
| ns | 5822 |  | 154 | aho_corasick.c: complete definition roster | 4.8 |  | 0.739 |
| walker |  | 5857 | 203 | Code::CodeKey { rung: Names, file: krep.c, decl: 0, sub: 2, line: 0 } |  |  | 0.739 |
| walker |  | 5864 | 7 | Code::CodeKey { rung: Decl, file: krep.c, decl: 26, sub: 0, line: 111 } |  |  | 0.739 |
| walker |  | 5877 | 13 | Code::CodeKey { rung: Doc, file: krep.c, decl: 23, sub: 0, line: 104 } |  |  | 0.739 |
| walker |  | 5891 | 14 | Code::CodeKey { rung: Doc, file: krep.c, decl: 22, sub: 0, line: 101 } |  |  | 0.739 |
| walker |  | 5905 | 14 | Code::CodeKey { rung: Doc, file: krep.c, decl: 27, sub: 0, line: 116 } |  |  | 0.739 |
| ns | 6014 |  | 192 | aho_corasick.c trie data model: ac_node_t and struct ac_trie | 4.9 |  | 0.721 |
| walker |  | 6097 | 192 | Code::CodeKey { rung: Names, file: krep.c, decl: 0, sub: 3, line: 0 } |  |  | 0.723 |
| walker |  | 6102 | 5 | Code::CodeKey { rung: Decl, file: krep.c, decl: 35, sub: 0, line: 128 } |  |  | 0.723 |
| walker |  | 6107 | 5 | Code::CodeKey { rung: Decl, file: krep.c, decl: 36, sub: 0, line: 139 } |  |  | 0.723 |
| walker |  | 6112 | 5 | Code::CodeKey { rung: Decl, file: krep.c, decl: 37, sub: 0, line: 175 } |  |  | 0.723 |
| walker |  | 6122 | 10 | Code::CodeKey { rung: Doc, file: krep.c, decl: 36, sub: 0, line: 139 } |  |  | 0.723 |
| walker |  | 6133 | 11 | Code::CodeKey { rung: Doc, file: krep.c, decl: 35, sub: 0, line: 128 } |  |  | 0.723 |
| walker |  | 6145 | 12 | Code::CodeKey { rung: Doc, file: krep.c, decl: 34, sub: 0, line: 125 } |  |  | 0.723 |
| walker |  | 6163 | 18 | Code::CodeKey { rung: Doc, file: krep.c, decl: 37, sub: 0, line: 175 } |  |  | 0.723 |
| ns | 6272 |  | 258 | test/test_krep.c: complete roster of test functions, helpers and main | 4.10 |  | 0.703 |
| walker |  | 6346 | 183 | Code::CodeKey { rung: Names, file: krep.c, decl: 0, sub: 4, line: 0 } |  |  | 0.716 |
| walker |  | 6351 | 5 | Code::CodeKey { rung: Decl, file: krep.c, decl: 38, sub: 0, line: 244 } |  |  | 0.716 |
| walker |  | 6356 | 5 | Code::CodeKey { rung: Decl, file: krep.c, decl: 39, sub: 0, line: 256 } |  |  | 0.716 |
| walker |  | 6361 | 5 | Code::CodeKey { rung: Decl, file: krep.c, decl: 41, sub: 0, line: 363 } |  |  | 0.716 |
| walker |  | 6366 | 5 | Code::CodeKey { rung: Decl, file: krep.c, decl: 42, sub: 0, line: 401 } |  |  | 0.716 |
| walker |  | 6371 | 5 | Code::CodeKey { rung: Decl, file: krep.c, decl: 43, sub: 0, line: 420 } |  |  | 0.716 |
| walker |  | 6376 | 5 | Code::CodeKey { rung: Decl, file: krep.c, decl: 44, sub: 0, line: 438 } |  |  | 0.716 |
| walker |  | 6413 | 37 | Code::CodeKey { rung: Decl, file: krep.c, decl: 40, sub: 0, line: 329 } |  |  | 0.716 |
| walker |  | 6426 | 13 | Code::CodeKey { rung: Doc, file: krep.c, decl: 38, sub: 0, line: 244 } |  |  | 0.716 |
| walker |  | 6442 | 16 | Code::CodeKey { rung: Doc, file: krep.c, decl: 42, sub: 0, line: 401 } |  |  | 0.716 |
| walker |  | 6460 | 18 | Code::CodeKey { rung: Doc, file: krep.c, decl: 43, sub: 0, line: 420 } |  |  | 0.716 |
| ns | 6497 |  | 225 | test/test_regex.c and test/test_multiple_patterns.c rosters | 4.11 |  | 0.700 |
| ns | 6608 |  | 111 | test/test_directory.c roster: the separate recursive-search integration binary | 4.12 |  | 0.692 |
| walker |  | 6626 | 166 | Code::CodeKey { rung: Names, file: krep.c, decl: 0, sub: 5, line: 0 } |  |  | 0.698 |
| walker |  | 6631 | 5 | Code::CodeKey { rung: Decl, file: krep.c, decl: 45, sub: 0, line: 461 } |  |  | 0.698 |
| walker |  | 6637 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 46, sub: 0, line: 1084 } |  |  | 0.698 |
| walker |  | 6643 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 47, sub: 0, line: 1125 } |  |  | 0.698 |
| walker |  | 6649 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 48, sub: 0, line: 1137 } |  |  | 0.698 |
| walker |  | 6655 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 49, sub: 0, line: 1198 } |  |  | 0.698 |
| walker |  | 6661 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 50, sub: 0, line: 1213 } |  |  | 0.698 |
| walker |  | 6671 | 10 | Code::CodeKey { rung: Doc, file: krep.c, decl: 48, sub: 0, line: 1137 } |  |  | 0.698 |
| walker |  | 6682 | 11 | Code::CodeKey { rung: Doc, file: krep.c, decl: 47, sub: 0, line: 1125 } |  |  | 0.698 |
| walker |  | 6697 | 15 | Code::CodeKey { rung: Doc, file: krep.c, decl: 50, sub: 0, line: 1213 } |  |  | 0.698 |
| walker |  | 6714 | 17 | Code::CodeKey { rung: Doc, file: krep.c, decl: 49, sub: 0, line: 1198 } |  |  | 0.698 |
| walker |  | 6734 | 20 | Code::CodeKey { rung: Doc, file: krep.c, decl: 40, sub: 0, line: 329 } |  |  | 0.698 |
| ns | 6841 |  | 233 | test/test_compat.h: the TESTING wrapper roster and its guards | 4.13 |  | 0.686 |
| walker |  | 6937 | 203 | Code::CodeKey { rung: Names, file: krep.c, decl: 0, sub: 6, line: 0 } |  |  | 0.703 |
| walker |  | 6943 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 53, sub: 0, line: 1585 } |  |  | 0.703 |
| walker |  | 6949 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 55, sub: 0, line: 1771 } |  |  | 0.703 |
| walker |  | 6955 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 56, sub: 0, line: 1873 } |  |  | 0.703 |
| walker |  | 6961 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 57, sub: 0, line: 1919 } |  |  | 0.703 |
| walker |  | 6967 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 58, sub: 0, line: 1964 } |  |  | 0.703 |
| ns | 6972 |  | 131 | test/test_krep.h: shared test helper declarations | 4.14 |  | 0.694 |
| walker |  | 6973 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 59, sub: 0, line: 1999 } |  |  | 0.694 |
| walker |  | 7014 | 41 | Code::CodeKey { rung: Decl, file: krep.c, decl: 52, sub: 0, line: 1389 } |  |  | 0.694 |
| walker |  | 7064 | 50 | Code::CodeKey { rung: Decl, file: krep.c, decl: 51, sub: 0, line: 1259 } |  |  | 0.694 |
| ns | 7080 |  | 108 | Makefile compiler configuration: CC, CFLAGS, LDFLAGS, PREFIX | 5.1 |  | 0.688 |
| walker |  | 7114 | 50 | Code::CodeKey { rung: Decl, file: krep.c, decl: 54, sub: 0, line: 1628 } |  |  | 0.688 |
| walker |  | 7130 | 16 | Code::CodeKey { rung: Doc, file: krep.c, decl: 59, sub: 0, line: 1999 } |  |  | 0.688 |
| walker |  | 7147 | 17 | Code::CodeKey { rung: Doc, file: krep.c, decl: 58, sub: 0, line: 1964 } |  |  | 0.688 |
| walker |  | 7166 | 19 | Code::CodeKey { rung: Doc, file: krep.c, decl: 56, sub: 0, line: 1873 } |  |  | 0.688 |
| walker |  | 7185 | 19 | Code::CodeKey { rung: Doc, file: krep.c, decl: 57, sub: 0, line: 1919 } |  |  | 0.688 |
| ns | 7316 |  | 236 | .github/workflows/ci.yml in full | 5.2 |  | 0.670 |
| walker |  | 7410 | 225 | Code::CodeKey { rung: Names, file: krep.c, decl: 0, sub: 7, line: 0 } |  |  | 0.678 |
| walker |  | 7416 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 61, sub: 0, line: 2252 } |  |  | 0.678 |
| walker |  | 7422 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 62, sub: 0, line: 2265 } |  |  | 0.678 |
| walker |  | 7428 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 63, sub: 0, line: 2274 } |  |  | 0.678 |
| walker |  | 7434 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 64, sub: 0, line: 3071 } |  |  | 0.678 |
| walker |  | 7440 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 65, sub: 0, line: 3090 } |  |  | 0.678 |
| walker |  | 7446 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 66, sub: 0, line: 3122 } |  |  | 0.678 |
| walker |  | 7452 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 69, sub: 0, line: 3163 } |  |  | 0.678 |
| walker |  | 7458 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 70, sub: 0, line: 3180 } |  |  | 0.678 |
| walker |  | 7492 | 34 | Code::CodeKey { rung: Decl, file: krep.c, decl: 67, sub: 0, line: 3146 } |  |  | 0.678 |
| walker |  | 7549 | 57 | Code::CodeKey { rung: Decl, file: krep.c, decl: 68, sub: 0, line: 3154 } |  |  | 0.679 |
| walker |  | 7559 | 10 | Code::CodeKey { rung: Doc, file: krep.c, decl: 60, sub: 0, line: 2249 } |  |  | 0.679 |
| walker |  | 7572 | 13 | Code::CodeKey { rung: Doc, file: krep.c, decl: 62, sub: 0, line: 2265 } |  |  | 0.679 |
| ns | 7580 |  | 264 | Makefile architecture detection: which SIMD flags each arch gets | 5.3 |  | 0.665 |
| walker |  | 7586 | 14 | Code::CodeKey { rung: Doc, file: krep.c, decl: 69, sub: 0, line: 3163 } |  |  | 0.665 |
| walker |  | 7601 | 15 | Code::CodeKey { rung: Doc, file: krep.c, decl: 64, sub: 0, line: 3071 } |  |  | 0.665 |
| walker |  | 7616 | 15 | Code::CodeKey { rung: Doc, file: krep.c, decl: 65, sub: 0, line: 3090 } |  |  | 0.665 |
| walker |  | 7631 | 15 | Code::CodeKey { rung: Doc, file: krep.c, decl: 66, sub: 0, line: 3122 } |  |  | 0.665 |
| walker |  | 7646 | 15 | Code::CodeKey { rung: Doc, file: krep.c, decl: 67, sub: 0, line: 3146 } |  |  | 0.666 |
| walker |  | 7662 | 16 | Code::CodeKey { rung: Doc, file: krep.c, decl: 61, sub: 0, line: 2252 } |  |  | 0.666 |
| walker |  | 7680 | 18 | Code::CodeKey { rung: Doc, file: krep.c, decl: 70, sub: 0, line: 3180 } |  |  | 0.666 |
| walker |  | 7700 | 20 | Code::CodeKey { rung: Doc, file: krep.c, decl: 68, sub: 0, line: 3154 } |  |  | 0.666 |
| walker |  | 7882 | 182 | Code::CodeKey { rung: Names, file: krep.c, decl: 0, sub: 8, line: 0 } |  |  | 0.678 |
| ns | 7883 |  | 303 | Makefile compile/link rules and the parallel -DTESTING build | 5.4 |  | 0.667 |
| walker |  | 7888 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 71, sub: 0, line: 3240 } |  |  | 0.667 |
| walker |  | 7894 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 72, sub: 0, line: 3272 } |  |  | 0.667 |
| walker |  | 7900 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 73, sub: 0, line: 3297 } |  |  | 0.667 |
| walker |  | 7906 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 75, sub: 0, line: 3442 } |  |  | 0.667 |
| walker |  | 7912 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 78, sub: 0, line: 4046 } |  |  | 0.667 |
| walker |  | 7936 | 24 | Code::CodeKey { rung: Decl, file: krep.c, decl: 74, sub: 0, line: 3310 } |  |  | 0.667 |
| walker |  | 7969 | 33 | Code::CodeKey { rung: Decl, file: krep.c, decl: 76, sub: 0, line: 3450 } |  |  | 0.667 |
| walker |  | 8010 | 41 | Code::CodeKey { rung: Decl, file: krep.c, decl: 77, sub: 0, line: 3891 } |  |  | 0.667 |
| walker |  | 8021 | 11 | Code::CodeKey { rung: Doc, file: krep.c, decl: 77, sub: 0, line: 3891 } |  |  | 0.667 |
| walker |  | 8036 | 15 | Code::CodeKey { rung: Doc, file: krep.c, decl: 74, sub: 0, line: 3310 } |  |  | 0.667 |
| walker |  | 8052 | 16 | Code::CodeKey { rung: Doc, file: krep.c, decl: 78, sub: 0, line: 4046 } |  |  | 0.657 |
| ns | 8052 |  | 169 | Makefile run targets: test, test-directory, ci, bench-rg, all-tests | 5.5 |  | 0.657 |
| walker |  | 8072 | 20 | Code::CodeKey { rung: Doc, file: krep.c, decl: 72, sub: 0, line: 3272 } |  |  | 0.657 |
| walker |  | 8093 | 21 | Code::CodeKey { rung: Doc, file: krep.c, decl: 73, sub: 0, line: 3297 } |  |  | 0.657 |
| walker |  | 8114 | 21 | Code::CodeKey { rung: Doc, file: krep.c, decl: 76, sub: 0, line: 3450 } |  |  | 0.657 |
| ns | 8209 |  | 157 | .github/workflows/release.yml: tag trigger and release artifacts | 5.6 |  | 0.645 |
| walker |  | 8347 | 233 | Code::CodeKey { rung: Names, file: krep.c, decl: 0, sub: 9, line: 0 } |  |  | 0.659 |
| walker |  | 8353 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 79, sub: 0, line: 4104 } |  |  | 0.659 |
| walker |  | 8359 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 80, sub: 0, line: 4209 } |  |  | 0.659 |
| walker |  | 8365 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 81, sub: 0, line: 4251 } |  |  | 0.659 |
| walker |  | 8371 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 82, sub: 0, line: 4313 } |  |  | 0.659 |
| walker |  | 8377 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 83, sub: 0, line: 4332 } |  |  | 0.659 |
| walker |  | 8418 | 41 | Code::CodeKey { rung: Decl, file: krep.c, decl: 84, sub: 0, line: 4371 } |  |  | 0.659 |
| ns | 8462 |  | 253 | krep.c constants: VERSION and every performance tunable | 6.1 |  | 0.666 |
| walker |  | 8481 | 63 | Code::CodeKey { rung: Decl, file: krep.c, decl: 85, sub: 0, line: 4505 } |  |  | 0.668 |
| walker |  | 8546 | 65 | Code::CodeKey { rung: Decl, file: krep.c, decl: 88, sub: 0, line: 5104 } |  |  | 0.669 |
| walker |  | 8611 | 65 | Code::CodeKey { rung: Decl, file: krep.c, decl: 87, sub: 0, line: 4872 } |  |  | 0.671 |
| walker |  | 8676 | 65 | Code::CodeKey { rung: Decl, file: krep.c, decl: 86, sub: 0, line: 4699 } |  |  | 0.674 |
| walker |  | 8687 | 11 | Code::CodeKey { rung: Doc, file: krep.c, decl: 83, sub: 0, line: 4332 } |  |  | 0.674 |
| ns | 8696 |  | 234 | krep.c global option state and the lower_table constructor | 6.2 |  | 0.673 |
| walker |  | 8700 | 13 | Code::CodeKey { rung: Doc, file: krep.c, decl: 82, sub: 0, line: 4313 } |  |  | 0.673 |
| walker |  | 8714 | 14 | Code::CodeKey { rung: Doc, file: krep.c, decl: 80, sub: 0, line: 4209 } |  |  | 0.673 |
| walker |  | 8729 | 15 | Code::CodeKey { rung: Doc, file: krep.c, decl: 81, sub: 0, line: 4251 } |  |  | 0.673 |
| walker |  | 8747 | 18 | Code::CodeKey { rung: Doc, file: krep.c, decl: 79, sub: 0, line: 4104 } |  |  | 0.673 |
| walker |  | 8768 | 21 | Code::CodeKey { rung: Doc, file: krep.c, decl: 84, sub: 0, line: 4371 } |  |  | 0.673 |
| walker |  | 8792 | 24 | Code::CodeKey { rung: Doc, file: krep.c, decl: 71, sub: 0, line: 3240 } |  |  | 0.673 |
| walker |  | 8824 | 32 | Code::CodeKey { rung: Doc, file: krep.c, decl: 44, sub: 0, line: 438 } |  |  | 0.673 |
| walker |  | 8857 | 33 | Code::CodeKey { rung: Doc, file: krep.c, decl: 86, sub: 0, line: 4699 } |  |  | 0.673 |
| walker |  | 8892 | 35 | Code::CodeKey { rung: Doc, file: krep.c, decl: 75, sub: 0, line: 3442 } |  |  | 0.673 |
| walker |  | 8928 | 36 | Code::CodeKey { rung: Doc, file: krep.c, decl: 51, sub: 0, line: 1259 } |  |  | 0.673 |
| walker |  | 8965 | 37 | Code::CodeKey { rung: Doc, file: krep.c, decl: 46, sub: 0, line: 1084 } |  |  | 0.673 |
| ns | 8966 |  | 270 | main: the getopt_long table and short-option string | 6.3 |  | 0.665 |
| walker |  | 9004 | 39 | Code::CodeKey { rung: Doc, file: krep.c, decl: 88, sub: 0, line: 5104 } |  |  | 0.665 |
| walker |  | 9044 | 40 | Code::CodeKey { rung: Doc, file: krep.c, decl: 41, sub: 0, line: 363 } |  |  | 0.665 |
| walker |  | 9290 | 246 | Code::CodeKey { rung: Names, file: aho_corasick.c, decl: 0, sub: 0, line: 0 } |  |  | 0.676 |
| walker |  | 9295 | 5 | Code::CodeKey { rung: Decl, file: aho_corasick.c, decl: 3, sub: 0, line: 34 } |  |  | 0.676 |
| ns | 9299 |  | 333 | select_search_algorithm: dispatch head through the short-pattern branch | 6.4 |  | 0.662 |
| walker |  | 9300 | 5 | Code::CodeKey { rung: Decl, file: aho_corasick.c, decl: 4, sub: 0, line: 55 } |  |  | 0.662 |
| walker |  | 9305 | 5 | Code::CodeKey { rung: Decl, file: aho_corasick.c, decl: 5, sub: 0, line: 86 } |  |  | 0.662 |
| walker |  | 9310 | 5 | Code::CodeKey { rung: Decl, file: aho_corasick.c, decl: 6, sub: 0, line: 111 } |  |  | 0.662 |
| walker |  | 9315 | 5 | Code::CodeKey { rung: Decl, file: aho_corasick.c, decl: 7, sub: 0, line: 274 } |  |  | 0.662 |
| walker |  | 9320 | 5 | Code::CodeKey { rung: Decl, file: aho_corasick.c, decl: 8, sub: 0, line: 287 } |  |  | 0.662 |
| walker |  | 9357 | 37 | Code::CodeKey { rung: Decl, file: aho_corasick.c, decl: 12, sub: 0, line: 299 } |  |  | 0.662 |
| walker |  | 9420 | 63 | Code::CodeKey { rung: Decl, file: aho_corasick.c, decl: 2, sub: 0, line: 26 } |  |  | 0.664 |
| walker |  | 9522 | 102 | Code::CodeKey { rung: Decl, file: aho_corasick.c, decl: 1, sub: 0, line: 17 } |  |  | 0.672 |
| ns | 9685 |  | 386 | select_search_algorithm: SIMD length limits and the KMP/Boyer-Moore fallback | 6.5 | 6.4 | 0.656 |
| walker |  | 9706 | 184 | Code::CodeKey { rung: Names, file: aho_corasick.h, decl: 0, sub: 0, line: 0 } |  |  | 0.662 |
| walker |  | 9716 | 10 | Code::CodeKey { rung: Doc, file: aho_corasick.h, decl: 5, sub: 0, line: 23 } |  |  | 0.662 |
| walker |  | 9728 | 12 | Code::CodeKey { rung: Doc, file: aho_corasick.h, decl: 9, sub: 0, line: 33 } |  |  | 0.664 |
| walker |  | 9740 | 12 | Code::CodeKey { rung: Doc, file: aho_corasick.h, decl: 11, sub: 0, line: 39 } |  |  | 0.666 |
| walker |  | 9754 | 14 | Code::CodeKey { rung: Doc, file: aho_corasick.h, decl: 6, sub: 0, line: 26 } |  |  | 0.666 |
| walker |  | 9769 | 15 | Code::CodeKey { rung: Doc, file: aho_corasick.h, decl: 8, sub: 0, line: 30 } |  |  | 0.668 |
| walker |  | 9787 | 18 | Code::CodeKey { rung: Doc, file: aho_corasick.h, decl: 1, sub: 0, line: 14 } |  |  | 0.668 |
| walker |  | 9807 | 20 | Code::CodeKey { rung: Doc, file: aho_corasick.h, decl: 10, sub: 0, line: 36 } |  |  | 0.672 |
| walker |  | 9842 | 35 | Code::CodeKey { rung: Doc, file: aho_corasick.h, decl: 3, sub: 0, line: 19 } |  |  | 0.672 |
| walker |  | 9851 | 9 | Code::CodeKey { rung: Doc, file: aho_corasick.c, decl: 3, sub: 0, line: 34 } |  |  | 0.672 |
| walker |  | 9863 | 12 | Code::CodeKey { rung: Doc, file: aho_corasick.c, decl: 4, sub: 0, line: 55 } |  |  | 0.672 |
| ns | 9868 |  | 183 | gitignore data model: pattern record and parent-chained context | 6.6 |  | 0.677 |
| walker |  | 9876 | 13 | Code::CodeKey { rung: Doc, file: aho_corasick.c, decl: 12, sub: 0, line: 299 } |  |  | 0.677 |
| walker |  | 9890 | 14 | Code::CodeKey { rung: Doc, file: aho_corasick.c, decl: 6, sub: 0, line: 111 } |  |  | 0.677 |
| walker |  | 9904 | 14 | Code::CodeKey { rung: Doc, file: aho_corasick.c, decl: 7, sub: 0, line: 274 } |  |  | 0.677 |
| walker |  | 9918 | 14 | Code::CodeKey { rung: Doc, file: aho_corasick.c, decl: 11, sub: 0, line: 296 } |  |  | 0.677 |
| walker |  | 9933 | 15 | Code::CodeKey { rung: Doc, file: aho_corasick.c, decl: 5, sub: 0, line: 86 } |  |  | 0.677 |
| walker |  | 9982 | 49 | Code::CodeKey { rung: Doc, file: krep.c, decl: 39, sub: 0, line: 256 } |  |  | 0.677 |
| ns | 9993 |  | 125 | Licence header, dependabot config and .gitignore | 7.1 |  | 0.670 |
