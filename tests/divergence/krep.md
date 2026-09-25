Score(3000)=0.603 I=0.824 C=0.441 ns_rows≤3K=20/56 grid(1000/1442/2080/3000/4327/6240/9000)=0.638/0.686/0.632/0.603/0.745/0.724/0.704

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
| ns | 1343 |  | 170 | README Key Features, second half | 2.2 |  | 0.685 |
| walker |  | 1375 | 216 | Code::CodeKey { rung: Names, file: krep.h, decl: 0, sub: 0, line: 0 } |  |  | 0.686 |
| walker |  | 1384 | 9 | Code::CodeKey { rung: Doc, file: krep.h, decl: 10, sub: 0, line: 42 } |  |  | 0.686 |
| walker |  | 1398 | 14 | Code::CodeKey { rung: Doc, file: krep.h, decl: 1, sub: 0, line: 19 } |  |  | 0.686 |
| walker |  | 1412 | 14 | Code::CodeKey { rung: Doc, file: krep.h, decl: 2, sub: 0, line: 22 } |  |  | 0.686 |
| walker |  | 1432 | 20 | Code::CodeKey { rung: Doc, file: krep.h, decl: 4, sub: 0, line: 34 } |  |  | 0.686 |
| walker |  | 1476 | 44 | Markdown::Section { file: README.md, section_index: 14, keeps_default_concavity: false } |  |  | 0.686 |
| ns | 1534 |  | 191 | README command-line options table, first half (-i through -F) | 2.3 |  | 0.657 |
| walker |  | 1660 | 184 | Code::CodeKey { rung: Names, file: aho_corasick.h, decl: 0, sub: 0, line: 0 } |  |  | 0.680 |
| walker |  | 1670 | 10 | Code::CodeKey { rung: Doc, file: aho_corasick.h, decl: 5, sub: 0, line: 23 } |  |  | 0.680 |
| walker |  | 1682 | 12 | Code::CodeKey { rung: Doc, file: aho_corasick.h, decl: 9, sub: 0, line: 33 } |  |  | 0.687 |
| walker |  | 1694 | 12 | Code::CodeKey { rung: Doc, file: aho_corasick.h, decl: 11, sub: 0, line: 39 } |  |  | 0.695 |
| walker |  | 1708 | 14 | Code::CodeKey { rung: Doc, file: aho_corasick.h, decl: 6, sub: 0, line: 26 } |  |  | 0.695 |
| walker |  | 1723 | 15 | Code::CodeKey { rung: Doc, file: aho_corasick.h, decl: 8, sub: 0, line: 30 } |  |  | 0.706 |
| walker |  | 1741 | 18 | Code::CodeKey { rung: Doc, file: aho_corasick.h, decl: 1, sub: 0, line: 14 } |  |  | 0.706 |
| ns | 1753 |  | 219 | README command-line options table, second half (-r through -h) | 2.4 |  | 0.671 |
| walker |  | 1761 | 20 | Code::CodeKey { rung: Doc, file: aho_corasick.h, decl: 10, sub: 0, line: 36 } |  |  | 0.685 |
| ns | 1963 |  | 210 | README: the documented smart algorithm-selection policy | 2.5 |  | 0.649 |
| ns | 2069 |  | 106 | README usage examples: the six concrete invocations, command lines only | 2.6 |  | 0.632 |
| walker |  | 2176 | 415 | Markdown::Section { file: README.md, section_index: 7, keeps_default_concavity: true } |  |  | 0.724 |
| ns | 2181 |  | 112 | README: the multi-threading architecture section | 2.7 |  | 0.696 |
| ns | 2287 |  | 106 | README: the recursive-search skipping rules | 2.8 |  | 0.676 |
| walker |  | 2298 | 122 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.676 |
| walker |  | 2333 | 35 | Code::CodeKey { rung: Doc, file: aho_corasick.h, decl: 3, sub: 0, line: 19 } |  |  | 0.676 |
| ns | 2402 |  | 115 | README scope note: explicitly not a grep/ripgrep replacement | 2.9 |  | 0.682 |
| ns | 2512 |  | 110 | README install and build-from-source commands | 2.10 |  | 0.656 |
| walker |  | 2611 | 278 | Code::CodeKey { rung: Names, file: krep.h, decl: 0, sub: 1, line: 0 } |  |  | 0.658 |
| walker |  | 2643 | 32 | Code::CodeKey { rung: Decl, file: krep.h, decl: 17, sub: 0, line: 98 } |  |  | 0.659 |
| walker |  | 2683 | 40 | Code::CodeKey { rung: Decl, file: krep.h, decl: 14, sub: 0, line: 49 } |  |  | 0.660 |
| walker |  | 2734 | 51 | Code::CodeKey { rung: Decl, file: krep.h, decl: 19, sub: 0, line: 125 } |  |  | 0.660 |
| walker |  | 2796 | 62 | Code::CodeKey { rung: Decl, file: krep.h, decl: 15, sub: 0, line: 55 } |  |  | 0.662 |
| ns | 2902 |  | 390 | search_params_t: the struct every search function takes | 3.1 |  | 0.603 |
| walker |  | 2957 | 161 | Code::CodeKey { rung: Decl, file: krep.h, decl: 20, sub: 0, line: 132 } |  |  | 0.603 |
| ns | 3089 |  | 187 | match_position_t and match_result_t | 3.2 |  | 0.612 |
| ns | 3180 |  | 91 | search_func_t: the algorithm function-pointer type | 3.3 |  | 0.610 |
| walker |  | 3207 | 250 | Code::CodeKey { rung: Decl, file: krep.h, decl: 18, sub: 0, line: 104 } |  |  | 0.614 |
| ns | 3282 |  | 102 | krep.h thread-pool API: all four functions in full | 3.4 |  | 0.610 |
| ns | 3562 |  | 280 | thread_data_t: per-thread search state, including the false-sharing padding | 3.5 |  | 0.630 |
| walker |  | 3575 | 368 | Code::CodeKey { rung: Decl, file: krep.h, decl: 16, sub: 0, line: 65 } |  |  | 0.714 |
| walker |  | 3584 | 9 | Code::CodeKey { rung: Doc, file: krep.h, decl: 19, sub: 0, line: 125 } |  |  | 0.714 |
| walker |  | 3593 | 9 | Code::CodeKey { rung: Doc, file: krep.h, decl: 21, sub: 0, line: 146 } |  |  | 0.716 |
| walker |  | 3603 | 10 | Code::CodeKey { rung: Doc, file: krep.h, decl: 18, sub: 0, line: 104 } |  |  | 0.721 |
| walker |  | 3616 | 13 | Code::CodeKey { rung: Doc, file: krep.h, decl: 16, sub: 0, line: 65 } |  |  | 0.721 |
| ns | 3659 |  | 97 | krep.h helper roster: line finding, word matching, table preparation | 3.6 |  | 0.708 |
| ns | 3826 |  | 167 | skip_directories: the complete recursive-search directory blocklist | 3.7 |  | 0.694 |
| walker |  | 3842 | 226 | Code::CodeKey { rung: Names, file: krep.h, decl: 0, sub: 2, line: 0 } |  |  | 0.730 |
| walker |  | 3852 | 10 | Code::CodeKey { rung: Doc, file: krep.h, decl: 28, sub: 0, line: 183 } |  |  | 0.730 |
| ns | 4011 |  | 185 | skip_extensions: head, ellipsis, and the count line | 3.8 |  | 0.717 |
| walker |  | 4060 | 208 | Code::CodeKey { rung: Names, file: krep.c, decl: 0, sub: 0, line: 0 } |  |  | 0.718 |
| walker |  | 4067 | 7 | Code::CodeKey { rung: Doc, file: krep.c, decl: 4, sub: 0, line: 77 } |  |  | 0.718 |
| walker |  | 4078 | 11 | Code::CodeKey { rung: Doc, file: krep.c, decl: 2, sub: 0, line: 42 } |  |  | 0.718 |
| walker |  | 4092 | 14 | Code::CodeKey { rung: Doc, file: krep.c, decl: 3, sub: 0, line: 44 } |  |  | 0.718 |
| walker |  | 4108 | 16 | Code::CodeKey { rung: Doc, file: krep.c, decl: 1, sub: 0, line: 39 } |  |  | 0.718 |
| ns | 4236 |  | 225 | ANSI colour macro block, output palette and help palette | 3.9 |  | 0.724 |
| walker |  | 4293 | 185 | Markdown::Section { file: README.md, section_index: 9, keeps_default_concavity: false } |  |  | 0.745 |
| ns | 4395 |  | 159 | Documented contracts of the three entry points and the printer | 3.10 | 1.7 | 0.732 |
| walker |  | 4491 | 198 | Markdown::Section { file: README.md, section_index: 10, keeps_default_concavity: true } |  |  | 0.762 |
| walker |  | 4620 | 129 | Markdown::Section { file: README.md, section_index: 11, keeps_default_concavity: false } |  |  | 0.781 |
| ns | 4634 |  | 239 | krep.c file-level section banner map, all sixteen top-level banners | 4.1 |  | 0.757 |
| ns | 4758 |  | 124 | krep.c definition roster, lines 139-461: match results, line finding, printing | 4.2 |  | 0.743 |
| walker |  | 4866 | 246 | Code::CodeKey { rung: Names, file: aho_corasick.c, decl: 0, sub: 0, line: 0 } |  |  | 0.745 |
| walker |  | 4871 | 5 | Code::CodeKey { rung: Decl, file: aho_corasick.c, decl: 6, sub: 0, line: 111 } |  |  | 0.745 |
| walker |  | 4876 | 5 | Code::CodeKey { rung: Decl, file: aho_corasick.c, decl: 7, sub: 0, line: 274 } |  |  | 0.745 |
| walker |  | 4881 | 5 | Code::CodeKey { rung: Decl, file: aho_corasick.c, decl: 8, sub: 0, line: 287 } |  |  | 0.745 |
| walker |  | 4886 | 5 | Code::CodeKey { rung: Decl, file: aho_corasick.c, decl: 3, sub: 0, line: 34 } |  |  | 0.745 |
| walker |  | 4891 | 5 | Code::CodeKey { rung: Decl, file: aho_corasick.c, decl: 4, sub: 0, line: 55 } |  |  | 0.745 |
| walker |  | 4896 | 5 | Code::CodeKey { rung: Decl, file: aho_corasick.c, decl: 5, sub: 0, line: 86 } |  |  | 0.745 |
| ns | 4932 |  | 174 | krep.c definition roster, lines 1084-1964: utilities, the four scalar algorithms, orchestration | 4.3 |  | 0.728 |
| walker |  | 4933 | 37 | Code::CodeKey { rung: Decl, file: aho_corasick.c, decl: 12, sub: 0, line: 299 } |  |  | 0.728 |
| walker |  | 4996 | 63 | Code::CodeKey { rung: Decl, file: aho_corasick.c, decl: 2, sub: 0, line: 26 } |  |  | 0.728 |
| walker |  | 5098 | 102 | Code::CodeKey { rung: Decl, file: aho_corasick.c, decl: 1, sub: 0, line: 17 } |  |  | 0.729 |
| walker |  | 5111 | 13 | Code::CodeKey { rung: Doc, file: aho_corasick.c, decl: 12, sub: 0, line: 299 } |  |  | 0.729 |
| ns | 5124 |  | 192 | krep.c definition roster, lines 1999-3442: public API bodies, skip filters, gitignore | 4.4 |  | 0.712 |
| walker |  | 5125 | 14 | Code::CodeKey { rung: Doc, file: aho_corasick.c, decl: 6, sub: 0, line: 111 } |  |  | 0.712 |
| walker |  | 5139 | 14 | Code::CodeKey { rung: Doc, file: aho_corasick.c, decl: 7, sub: 0, line: 274 } |  |  | 0.712 |
| walker |  | 5336 | 197 | Code::CodeKey { rung: Names, file: krep.h, decl: 0, sub: 3, line: 0 } |  |  | 0.721 |
| walker |  | 5343 | 7 | Code::CodeKey { rung: Doc, file: krep.h, decl: 33, sub: 0, line: 203 } |  |  | 0.721 |
| ns | 5363 |  | 239 | krep.c definition roster, lines 3451-5108: main, thread pool, and the guarded SIMD kernels | 4.5 |  | 0.701 |
| ns | 5546 |  | 183 | search_file internal section map, all eleven banners inside lines 2274-3070 | 4.6 |  | 0.689 |
| walker |  | 5635 | 292 | Markdown::Section { file: README.md, section_index: 4, keeps_default_concavity: false } |  |  | 0.712 |
| walker |  | 5649 | 14 | Code::CodeKey { rung: Doc, file: aho_corasick.c, decl: 11, sub: 0, line: 296 } |  |  | 0.712 |
| ns | 5668 |  | 122 | print_matching_items internal section map: the -o and full-line output modes | 4.7 |  | 0.703 |
| ns | 5822 |  | 154 | aho_corasick.c: complete definition roster | 4.8 |  | 0.708 |
| walker |  | 5878 | 229 | Code::CodeKey { rung: Names, file: krep.c, decl: 0, sub: 1, line: 0 } |  |  | 0.709 |
| walker |  | 5887 | 9 | Code::CodeKey { rung: Doc, file: krep.c, decl: 15, sub: 0, line: 93 } |  |  | 0.709 |
| walker |  | 5903 | 16 | Code::CodeKey { rung: Doc, file: krep.c, decl: 20, sub: 0, line: 116 } |  |  | 0.709 |
| ns | 6014 |  | 192 | aho_corasick.c trie data model: ac_node_t and struct ac_trie | 4.9 |  | 0.706 |
| walker |  | 6200 | 297 | Markdown::Section { file: README.md, section_index: 6, keeps_default_concavity: false } |  |  | 0.718 |
| walker |  | 6223 | 23 | Code::CodeKey { rung: Doc, file: krep.h, decl: 14, sub: 0, line: 49 } |  |  | 0.724 |
| ns | 6272 |  | 258 | test/test_krep.c: complete roster of test functions, helpers and main | 4.10 |  | 0.705 |
| ns | 6497 |  | 225 | test/test_regex.c and test/test_multiple_patterns.c rosters | 4.11 |  | 0.689 |
| walker |  | 6513 | 290 | Markdown::Section { file: README.md, section_index: 8, keeps_default_concavity: false } |  |  | 0.689 |
| ns | 6608 |  | 111 | test/test_directory.c roster: the separate recursive-search integration binary | 4.12 |  | 0.681 |
| walker |  | 6695 | 182 | Code::CodeKey { rung: Names, file: krep.h, decl: 0, sub: 4, line: 0 } |  |  | 0.689 |
| walker |  | 6714 | 19 | Code::CodeKey { rung: Decl, file: krep.h, decl: 40, sub: 0, line: 218 } |  |  | 0.693 |
| walker |  | 6733 | 19 | Code::CodeKey { rung: Decl, file: krep.h, decl: 39, sub: 0, line: 214 } |  |  | 0.698 |
| walker |  | 6752 | 19 | Code::CodeKey { rung: Decl, file: krep.h, decl: 42, sub: 0, line: 226 } |  |  | 0.703 |
| walker |  | 6776 | 24 | Code::CodeKey { rung: Decl, file: krep.h, decl: 41, sub: 0, line: 222 } |  |  | 0.711 |
| walker |  | 6784 | 8 | Code::CodeKey { rung: Doc, file: krep.h, decl: 43, sub: 0, line: 231 } |  |  | 0.711 |
| ns | 6841 |  | 233 | test/test_compat.h: the TESTING wrapper roster and its guards | 4.13 |  | 0.699 |
| ns | 6972 |  | 131 | test/test_krep.h: shared test helper declarations | 4.14 |  | 0.689 |
| walker |  | 7004 | 220 | Code::CodeKey { rung: Names, file: krep.c, decl: 0, sub: 2, line: 0 } |  |  | 0.692 |
| walker |  | 7009 | 5 | Code::CodeKey { rung: Decl, file: krep.c, decl: 29, sub: 0, line: 139 } |  |  | 0.692 |
| walker |  | 7014 | 5 | Code::CodeKey { rung: Decl, file: krep.c, decl: 30, sub: 0, line: 175 } |  |  | 0.692 |
| walker |  | 7019 | 5 | Code::CodeKey { rung: Decl, file: krep.c, decl: 31, sub: 0, line: 244 } |  |  | 0.692 |
| walker |  | 7024 | 5 | Code::CodeKey { rung: Decl, file: krep.c, decl: 32, sub: 0, line: 256 } |  |  | 0.692 |
| walker |  | 7029 | 5 | Code::CodeKey { rung: Decl, file: krep.c, decl: 28, sub: 0, line: 128 } |  |  | 0.692 |
| walker |  | 7039 | 10 | Code::CodeKey { rung: Doc, file: krep.c, decl: 29, sub: 0, line: 139 } |  |  | 0.692 |
| walker |  | 7051 | 12 | Code::CodeKey { rung: Doc, file: krep.c, decl: 27, sub: 0, line: 125 } |  |  | 0.693 |
| walker |  | 7064 | 13 | Code::CodeKey { rung: Doc, file: krep.c, decl: 31, sub: 0, line: 244 } |  |  | 0.693 |
| ns | 7080 |  | 108 | Makefile compiler configuration: CC, CFLAGS, LDFLAGS, PREFIX | 5.1 |  | 0.687 |
| walker |  | 7082 | 18 | Code::CodeKey { rung: Doc, file: krep.c, decl: 30, sub: 0, line: 175 } |  |  | 0.687 |
| walker |  | 7103 | 21 | Code::CodeKey { rung: Doc, file: aho_corasick.c, decl: 9, sub: 0, line: 293 } |  |  | 0.687 |
| walker |  | 7125 | 22 | Code::CodeKey { rung: Doc, file: aho_corasick.c, decl: 8, sub: 0, line: 287 } |  |  | 0.687 |
| ns | 7316 |  | 236 | .github/workflows/ci.yml in full | 5.2 |  | 0.669 |
| walker |  | 7337 | 212 | Code::CodeKey { rung: Names, file: krep.c, decl: 0, sub: 3, line: 0 } |  |  | 0.681 |
| walker |  | 7342 | 5 | Code::CodeKey { rung: Decl, file: krep.c, decl: 34, sub: 0, line: 363 } |  |  | 0.681 |
| walker |  | 7347 | 5 | Code::CodeKey { rung: Decl, file: krep.c, decl: 35, sub: 0, line: 401 } |  |  | 0.681 |
| walker |  | 7352 | 5 | Code::CodeKey { rung: Decl, file: krep.c, decl: 38, sub: 0, line: 461 } |  |  | 0.681 |
| walker |  | 7357 | 5 | Code::CodeKey { rung: Decl, file: krep.c, decl: 36, sub: 0, line: 420 } |  |  | 0.681 |
| walker |  | 7362 | 5 | Code::CodeKey { rung: Decl, file: krep.c, decl: 37, sub: 0, line: 438 } |  |  | 0.681 |
| walker |  | 7368 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 39, sub: 0, line: 1084 } |  |  | 0.681 |
| walker |  | 7405 | 37 | Code::CodeKey { rung: Decl, file: krep.c, decl: 33, sub: 0, line: 329 } |  |  | 0.681 |
| walker |  | 7564 | 159 | Code::CodeKey { rung: Names, file: krep.h, decl: 0, sub: 5, line: 0 } |  |  | 0.693 |
| walker |  | 7569 | 5 | Code::CodeKey { rung: Decl, file: krep.h, decl: 49, sub: 0, line: 298 } |  |  | 0.693 |
| walker |  | 7574 | 5 | Code::CodeKey { rung: Decl, file: krep.h, decl: 50, sub: 0, line: 312 } |  |  | 0.693 |
| ns | 7580 |  | 264 | Makefile architecture detection: which SIMD flags each arch gets | 5.3 |  | 0.679 |
| walker |  | 7590 | 16 | Code::CodeKey { rung: Doc, file: krep.c, decl: 35, sub: 0, line: 401 } |  |  | 0.679 |
| walker |  | 7826 | 236 | Plaintext::Whole { file: .github/workflows/ci.yml } |  |  | 0.710 |
| walker |  | 7845 | 19 | Code::CodeKey { rung: Body, file: krep.h, decl: 49, sub: 0, line: 298 } |  |  | 0.710 |
| walker |  | 7873 | 28 | Code::CodeKey { rung: Body, file: aho_corasick.c, decl: 8, sub: 0, line: 287 } |  |  | 0.710 |
| walker |  | 7882 | 9 | Code::CodeKey { rung: Doc, file: aho_corasick.c, decl: 3, sub: 0, line: 34 } |  |  | 0.710 |
| ns | 7883 |  | 303 | Makefile compile/link rules and the parallel -DTESTING build | 5.4 |  | 0.698 |
| ns | 8052 |  | 169 | Makefile run targets: test, test-directory, ci, bench-rg, all-tests | 5.5 |  | 0.687 |
| walker |  | 8122 | 240 | Code::CodeKey { rung: Names, file: krep.c, decl: 0, sub: 4, line: 0 } |  |  | 0.699 |
| walker |  | 8128 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 40, sub: 0, line: 1125 } |  |  | 0.699 |
| walker |  | 8134 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 41, sub: 0, line: 1137 } |  |  | 0.699 |
| walker |  | 8140 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 42, sub: 0, line: 1198 } |  |  | 0.699 |
| walker |  | 8146 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 43, sub: 0, line: 1213 } |  |  | 0.699 |
| walker |  | 8152 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 48, sub: 0, line: 1771 } |  |  | 0.699 |
| walker |  | 8158 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 46, sub: 0, line: 1585 } |  |  | 0.699 |
| walker |  | 8164 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 49, sub: 0, line: 1873 } |  |  | 0.699 |
| walker |  | 8205 | 41 | Code::CodeKey { rung: Decl, file: krep.c, decl: 45, sub: 0, line: 1389 } |  |  | 0.699 |
| ns | 8209 |  | 157 | .github/workflows/release.yml: tag trigger and release artifacts | 5.6 |  | 0.687 |
| walker |  | 8255 | 50 | Code::CodeKey { rung: Decl, file: krep.c, decl: 44, sub: 0, line: 1259 } |  |  | 0.687 |
| walker |  | 8305 | 50 | Code::CodeKey { rung: Decl, file: krep.c, decl: 47, sub: 0, line: 1628 } |  |  | 0.687 |
| walker |  | 8315 | 10 | Code::CodeKey { rung: Doc, file: krep.c, decl: 41, sub: 0, line: 1137 } |  |  | 0.687 |
| walker |  | 8326 | 11 | Code::CodeKey { rung: Doc, file: krep.c, decl: 40, sub: 0, line: 1125 } |  |  | 0.687 |
| walker |  | 8363 | 37 | Code::CodeKey { rung: Doc, file: krep.h, decl: 17, sub: 0, line: 98 } |  |  | 0.692 |
| walker |  | 8375 | 12 | Code::CodeKey { rung: Doc, file: aho_corasick.c, decl: 4, sub: 0, line: 55 } |  |  | 0.692 |
| ns | 8462 |  | 253 | krep.c constants: VERSION and every performance tunable | 6.1 |  | 0.694 |
| ns | 8696 |  | 234 | krep.c global option state and the lower_table constructor | 6.2 |  | 0.693 |
| ns | 8966 |  | 270 | main: the getopt_long table and short-option string | 6.3 |  | 0.684 |
| walker |  | 8972 | 597 | Plaintext::Whole { file: .github/workflows/release.yml } |  |  | 0.704 |
| walker |  | 9253 | 281 | Code::CodeKey { rung: Names, file: krep.c, decl: 0, sub: 5, line: 0 } |  |  | 0.715 |
| walker |  | 9259 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 50, sub: 0, line: 1919 } |  |  | 0.715 |
| walker |  | 9265 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 51, sub: 0, line: 1964 } |  |  | 0.715 |
| walker |  | 9271 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 52, sub: 0, line: 1999 } |  |  | 0.715 |
| walker |  | 9277 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 56, sub: 0, line: 2274 } |  |  | 0.715 |
| walker |  | 9283 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 54, sub: 0, line: 2252 } |  |  | 0.715 |
| walker |  | 9289 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 55, sub: 0, line: 2265 } |  |  | 0.715 |
| walker |  | 9295 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 57, sub: 0, line: 3071 } |  |  | 0.715 |
| ns | 9299 |  | 333 | select_search_algorithm: dispatch head through the short-pattern branch | 6.4 |  | 0.700 |
| walker |  | 9301 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 58, sub: 0, line: 3090 } |  |  | 0.700 |
| walker |  | 9307 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 59, sub: 0, line: 3122 } |  |  | 0.700 |
| walker |  | 9313 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 62, sub: 0, line: 3163 } |  |  | 0.700 |
| walker |  | 9319 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 63, sub: 0, line: 3180 } |  |  | 0.700 |
| walker |  | 9353 | 34 | Code::CodeKey { rung: Decl, file: krep.c, decl: 60, sub: 0, line: 3146 } |  |  | 0.700 |
| walker |  | 9410 | 57 | Code::CodeKey { rung: Decl, file: krep.c, decl: 61, sub: 0, line: 3154 } |  |  | 0.701 |
| walker |  | 9425 | 15 | Code::CodeKey { rung: Doc, file: aho_corasick.c, decl: 5, sub: 0, line: 86 } |  |  | 0.701 |
| walker |  | 9460 | 35 | Plaintext::Whole { file: .gitignore } |  |  | 0.701 |
| ns | 9685 |  | 386 | select_search_algorithm: SIMD length limits and the KMP/Boyer-Moore fallback | 6.5 | 6.4 | 0.684 |
| walker |  | 9763 | 303 | Code::CodeKey { rung: Names, file: krep.c, decl: 0, sub: 6, line: 0 } |  |  | 0.698 |
| walker |  | 9769 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 68, sub: 0, line: 3442 } |  |  | 0.698 |
| walker |  | 9775 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 71, sub: 0, line: 4104 } |  |  | 0.698 |
| walker |  | 9781 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 72, sub: 0, line: 4209 } |  |  | 0.698 |
| walker |  | 9787 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 74, sub: 0, line: 4313 } |  |  | 0.698 |
| walker |  | 9793 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 75, sub: 0, line: 4332 } |  |  | 0.698 |
| walker |  | 9799 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 64, sub: 0, line: 3240 } |  |  | 0.698 |
| walker |  | 9805 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 65, sub: 0, line: 3272 } |  |  | 0.698 |
| walker |  | 9811 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 66, sub: 0, line: 3297 } |  |  | 0.698 |
| walker |  | 9817 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 70, sub: 0, line: 4046 } |  |  | 0.698 |
| walker |  | 9823 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 73, sub: 0, line: 4251 } |  |  | 0.698 |
| walker |  | 9864 | 41 | Code::CodeKey { rung: Decl, file: krep.c, decl: 69, sub: 0, line: 3891 } |  |  | 0.698 |
| ns | 9868 |  | 183 | gitignore data model: pattern record and parent-chained context | 6.6 |  | 0.696 |
| walker |  | 9905 | 41 | Code::CodeKey { rung: Decl, file: krep.c, decl: 76, sub: 0, line: 4371 } |  |  | 0.696 |
| walker |  | 9929 | 24 | Code::CodeKey { rung: Decl, file: krep.c, decl: 67, sub: 0, line: 3310 } |  |  | 0.696 |
| walker |  | 9992 | 63 | Code::CodeKey { rung: Body, file: krep.h, decl: 50, sub: 0, line: 312 } |  |  | 0.696 |
| ns | 9993 |  | 125 | Licence header, dependabot config and .gitignore | 7.1 |  | 0.690 |
