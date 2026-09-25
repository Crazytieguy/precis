Score(3000)=0.603 I=0.828 C=0.440 ns_rows≤3K=20/56 grid(1000/1442/2080/3000/4327/6240/9000)=0.644/0.752/0.679/0.603/0.757/0.703/0.624

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
| walker |  | 561 | 115 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.764 |
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
| walker |  | 1313 | 12 | Code::CodeKey { rung: Doc, file: aho_corasick.h, decl: 11, sub: 0, line: 39 } |  |  | 0.712 |
| walker |  | 1327 | 14 | Code::CodeKey { rung: Doc, file: aho_corasick.h, decl: 6, sub: 0, line: 26 } |  |  | 0.712 |
| walker |  | 1342 | 15 | Code::CodeKey { rung: Doc, file: aho_corasick.h, decl: 8, sub: 0, line: 30 } |  |  | 0.724 |
| ns | 1343 |  | 170 | README Key Features, second half | 2.2 |  | 0.736 |
| walker |  | 1360 | 18 | Code::CodeKey { rung: Doc, file: aho_corasick.h, decl: 1, sub: 0, line: 14 } |  |  | 0.736 |
| walker |  | 1380 | 20 | Code::CodeKey { rung: Doc, file: aho_corasick.h, decl: 10, sub: 0, line: 36 } |  |  | 0.752 |
| walker |  | 1415 | 35 | Code::CodeKey { rung: Doc, file: aho_corasick.h, decl: 3, sub: 0, line: 19 } |  |  | 0.752 |
| walker |  | 1479 | 64 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.752 |
| walker |  | 1523 | 44 | Markdown::Section { file: README.md, section_index: 14, keeps_default_concavity: false } |  |  | 0.752 |
| ns | 1534 |  | 191 | README command-line options table, first half (-i through -F) | 2.3 |  | 0.719 |
| walker |  | 1739 | 216 | Code::CodeKey { rung: Names, file: krep.h, decl: 0, sub: 0, line: 0 } |  |  | 0.721 |
| walker |  | 1748 | 9 | Code::CodeKey { rung: Doc, file: krep.h, decl: 10, sub: 0, line: 42 } |  |  | 0.721 |
| ns | 1753 |  | 219 | README command-line options table, second half (-r through -h) | 2.4 |  | 0.685 |
| walker |  | 1762 | 14 | Code::CodeKey { rung: Doc, file: krep.h, decl: 1, sub: 0, line: 19 } |  |  | 0.685 |
| walker |  | 1776 | 14 | Code::CodeKey { rung: Doc, file: krep.h, decl: 2, sub: 0, line: 22 } |  |  | 0.685 |
| walker |  | 1796 | 20 | Code::CodeKey { rung: Doc, file: krep.h, decl: 4, sub: 0, line: 34 } |  |  | 0.685 |
| ns | 1963 |  | 210 | README: the documented smart algorithm-selection policy | 2.5 |  | 0.648 |
| ns | 2069 |  | 106 | README usage examples: the six concrete invocations, command lines only | 2.6 |  | 0.632 |
| ns | 2181 |  | 112 | README: the multi-threading architecture section | 2.7 |  | 0.607 |
| walker |  | 2211 | 415 | Markdown::Section { file: README.md, section_index: 7, keeps_default_concavity: true } |  |  | 0.695 |
| ns | 2287 |  | 106 | README: the recursive-search skipping rules | 2.8 |  | 0.675 |
| walker |  | 2333 | 122 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.675 |
| ns | 2402 |  | 115 | README scope note: explicitly not a grep/ripgrep replacement | 2.9 |  | 0.680 |
| ns | 2512 |  | 110 | README install and build-from-source commands | 2.10 |  | 0.655 |
| walker |  | 2611 | 278 | Code::CodeKey { rung: Names, file: krep.h, decl: 0, sub: 1, line: 0 } |  |  | 0.657 |
| walker |  | 2643 | 32 | Code::CodeKey { rung: Decl, file: krep.h, decl: 17, sub: 0, line: 98 } |  |  | 0.658 |
| walker |  | 2683 | 40 | Code::CodeKey { rung: Decl, file: krep.h, decl: 14, sub: 0, line: 49 } |  |  | 0.659 |
| walker |  | 2734 | 51 | Code::CodeKey { rung: Decl, file: krep.h, decl: 19, sub: 0, line: 125 } |  |  | 0.659 |
| walker |  | 2743 | 9 | Code::CodeKey { rung: Doc, file: krep.h, decl: 19, sub: 0, line: 125 } |  |  | 0.659 |
| walker |  | 2752 | 9 | Code::CodeKey { rung: Doc, file: krep.h, decl: 21, sub: 0, line: 146 } |  |  | 0.659 |
| walker |  | 2814 | 62 | Code::CodeKey { rung: Decl, file: krep.h, decl: 15, sub: 0, line: 55 } |  |  | 0.661 |
| walker |  | 2837 | 23 | Code::CodeKey { rung: Doc, file: krep.h, decl: 14, sub: 0, line: 49 } |  |  | 0.662 |
| ns | 2902 |  | 390 | search_params_t: the struct every search function takes | 3.1 |  | 0.603 |
| walker |  | 2998 | 161 | Code::CodeKey { rung: Decl, file: krep.h, decl: 20, sub: 0, line: 132 } |  |  | 0.603 |
| walker |  | 3035 | 37 | Code::CodeKey { rung: Doc, file: krep.h, decl: 17, sub: 0, line: 98 } |  |  | 0.604 |
| ns | 3089 |  | 187 | match_position_t and match_result_t | 3.2 |  | 0.625 |
| ns | 3180 |  | 91 | search_func_t: the algorithm function-pointer type | 3.3 |  | 0.633 |
| ns | 3282 |  | 102 | krep.h thread-pool API: all four functions in full | 3.4 |  | 0.631 |
| walker |  | 3285 | 250 | Code::CodeKey { rung: Decl, file: krep.h, decl: 18, sub: 0, line: 104 } |  |  | 0.636 |
| walker |  | 3295 | 10 | Code::CodeKey { rung: Doc, file: krep.h, decl: 18, sub: 0, line: 104 } |  |  | 0.636 |
| ns | 3562 |  | 280 | thread_data_t: per-thread search state, including the false-sharing padding | 3.5 |  | 0.658 |
| ns | 3659 |  | 97 | krep.h helper roster: line finding, word matching, table preparation | 3.6 |  | 0.646 |
| walker |  | 3663 | 368 | Code::CodeKey { rung: Decl, file: krep.h, decl: 16, sub: 0, line: 65 } |  |  | 0.726 |
| walker |  | 3676 | 13 | Code::CodeKey { rung: Doc, file: krep.h, decl: 16, sub: 0, line: 65 } |  |  | 0.726 |
| ns | 3826 |  | 167 | skip_directories: the complete recursive-search directory blocklist | 3.7 |  | 0.712 |
| walker |  | 3902 | 226 | Code::CodeKey { rung: Names, file: krep.h, decl: 0, sub: 2, line: 0 } |  |  | 0.747 |
| walker |  | 3912 | 10 | Code::CodeKey { rung: Doc, file: krep.h, decl: 28, sub: 0, line: 183 } |  |  | 0.747 |
| walker |  | 4002 | 90 | Code::CodeKey { rung: Doc, file: krep.h, decl: 26, sub: 0, line: 170 } |  |  | 0.747 |
| ns | 4011 |  | 185 | skip_extensions: head, ellipsis, and the count line | 3.8 |  | 0.735 |
| walker |  | 4199 | 197 | Code::CodeKey { rung: Names, file: krep.h, decl: 0, sub: 3, line: 0 } |  |  | 0.745 |
| walker |  | 4206 | 7 | Code::CodeKey { rung: Doc, file: krep.h, decl: 33, sub: 0, line: 203 } |  |  | 0.745 |
| ns | 4236 |  | 225 | ANSI colour macro block, output palette and help palette | 3.9 |  | 0.750 |
| walker |  | 4388 | 182 | Code::CodeKey { rung: Names, file: krep.h, decl: 0, sub: 4, line: 0 } |  |  | 0.760 |
| ns | 4395 |  | 159 | Documented contracts of the three entry points and the printer | 3.10 | 1.7 | 0.747 |
| walker |  | 4407 | 19 | Code::CodeKey { rung: Decl, file: krep.h, decl: 40, sub: 0, line: 218 } |  |  | 0.753 |
| walker |  | 4426 | 19 | Code::CodeKey { rung: Decl, file: krep.h, decl: 39, sub: 0, line: 214 } |  |  | 0.759 |
| walker |  | 4445 | 19 | Code::CodeKey { rung: Decl, file: krep.h, decl: 42, sub: 0, line: 226 } |  |  | 0.766 |
| walker |  | 4469 | 24 | Code::CodeKey { rung: Decl, file: krep.h, decl: 41, sub: 0, line: 222 } |  |  | 0.776 |
| walker |  | 4477 | 8 | Code::CodeKey { rung: Doc, file: krep.h, decl: 43, sub: 0, line: 231 } |  |  | 0.776 |
| walker |  | 4573 | 96 | Code::CodeKey { rung: Doc, file: krep.h, decl: 27, sub: 0, line: 180 } |  |  | 0.777 |
| ns | 4634 |  | 239 | krep.c file-level section banner map, all sixteen top-level banners | 4.1 |  | 0.754 |
| walker |  | 4732 | 159 | Code::CodeKey { rung: Names, file: krep.h, decl: 0, sub: 5, line: 0 } |  |  | 0.771 |
| walker |  | 4737 | 5 | Code::CodeKey { rung: Decl, file: krep.h, decl: 49, sub: 0, line: 298 } |  |  | 0.771 |
| walker |  | 4742 | 5 | Code::CodeKey { rung: Decl, file: krep.h, decl: 50, sub: 0, line: 312 } |  |  | 0.771 |
| ns | 4758 |  | 124 | krep.c definition roster, lines 139-461: match results, line finding, printing | 4.2 |  | 0.756 |
| walker |  | 4810 | 68 | Code::CodeKey { rung: Doc, file: krep.h, decl: 49, sub: 0, line: 298 } |  |  | 0.756 |
| walker |  | 4907 | 97 | Code::CodeKey { rung: Doc, file: krep.h, decl: 47, sub: 0, line: 278 } |  |  | 0.756 |
| ns | 4932 |  | 174 | krep.c definition roster, lines 1084-1964: utilities, the four scalar algorithms, orchestration | 4.3 |  | 0.739 |
| walker |  | 5009 | 102 | Code::CodeKey { rung: Doc, file: krep.h, decl: 48, sub: 0, line: 288 } |  |  | 0.739 |
| walker |  | 5116 | 107 | Code::CodeKey { rung: Doc, file: krep.h, decl: 50, sub: 0, line: 312 } |  |  | 0.739 |
| ns | 5124 |  | 192 | krep.c definition roster, lines 1999-3442: public API bodies, skip filters, gitignore | 4.4 |  | 0.722 |
| walker |  | 5232 | 116 | Code::CodeKey { rung: Doc, file: krep.h, decl: 25, sub: 0, line: 161 } |  |  | 0.729 |
| ns | 5363 |  | 239 | krep.c definition roster, lines 3451-5108: main, thread pool, and the guarded SIMD kernels | 4.5 |  | 0.710 |
| walker |  | 5401 | 169 | Code::CodeKey { rung: Doc, file: krep.h, decl: 32, sub: 0, line: 200 } |  |  | 0.718 |
| ns | 5546 |  | 183 | search_file internal section map, all eleven banners inside lines 2274-3070 | 4.6 |  | 0.706 |
| walker |  | 5647 | 246 | Code::CodeKey { rung: Names, file: aho_corasick.c, decl: 0, sub: 0, line: 0 } |  |  | 0.707 |
| walker |  | 5652 | 5 | Code::CodeKey { rung: Decl, file: aho_corasick.c, decl: 3, sub: 0, line: 34 } |  |  | 0.707 |
| walker |  | 5657 | 5 | Code::CodeKey { rung: Decl, file: aho_corasick.c, decl: 4, sub: 0, line: 55 } |  |  | 0.707 |
| walker |  | 5662 | 5 | Code::CodeKey { rung: Decl, file: aho_corasick.c, decl: 5, sub: 0, line: 86 } |  |  | 0.707 |
| walker |  | 5667 | 5 | Code::CodeKey { rung: Decl, file: aho_corasick.c, decl: 6, sub: 0, line: 111 } |  |  | 0.707 |
| ns | 5668 |  | 122 | print_matching_items internal section map: the -o and full-line output modes | 4.7 |  | 0.699 |
| walker |  | 5672 | 5 | Code::CodeKey { rung: Decl, file: aho_corasick.c, decl: 7, sub: 0, line: 274 } |  |  | 0.699 |
| walker |  | 5677 | 5 | Code::CodeKey { rung: Decl, file: aho_corasick.c, decl: 8, sub: 0, line: 287 } |  |  | 0.699 |
| walker |  | 5714 | 37 | Code::CodeKey { rung: Decl, file: aho_corasick.c, decl: 12, sub: 0, line: 299 } |  |  | 0.699 |
| walker |  | 5777 | 63 | Code::CodeKey { rung: Decl, file: aho_corasick.c, decl: 2, sub: 0, line: 26 } |  |  | 0.699 |
| walker |  | 5786 | 9 | Code::CodeKey { rung: Doc, file: aho_corasick.c, decl: 3, sub: 0, line: 34 } |  |  | 0.699 |
| walker |  | 5798 | 12 | Code::CodeKey { rung: Doc, file: aho_corasick.c, decl: 4, sub: 0, line: 55 } |  |  | 0.699 |
| walker |  | 5811 | 13 | Code::CodeKey { rung: Doc, file: aho_corasick.c, decl: 12, sub: 0, line: 299 } |  |  | 0.699 |
| ns | 5822 |  | 154 | aho_corasick.c: complete definition roster | 4.8 |  | 0.704 |
| walker |  | 5825 | 14 | Code::CodeKey { rung: Doc, file: aho_corasick.c, decl: 6, sub: 0, line: 111 } |  |  | 0.704 |
| walker |  | 5839 | 14 | Code::CodeKey { rung: Doc, file: aho_corasick.c, decl: 7, sub: 0, line: 274 } |  |  | 0.704 |
| walker |  | 5853 | 14 | Code::CodeKey { rung: Doc, file: aho_corasick.c, decl: 11, sub: 0, line: 296 } |  |  | 0.704 |
| walker |  | 5868 | 15 | Code::CodeKey { rung: Doc, file: aho_corasick.c, decl: 5, sub: 0, line: 86 } |  |  | 0.704 |
| walker |  | 5970 | 102 | Code::CodeKey { rung: Decl, file: aho_corasick.c, decl: 1, sub: 0, line: 17 } |  |  | 0.705 |
| walker |  | 5991 | 21 | Code::CodeKey { rung: Doc, file: aho_corasick.c, decl: 9, sub: 0, line: 293 } |  |  | 0.705 |
| walker |  | 6013 | 22 | Code::CodeKey { rung: Doc, file: aho_corasick.c, decl: 8, sub: 0, line: 287 } |  |  | 0.705 |
| ns | 6014 |  | 192 | aho_corasick.c trie data model: ac_node_t and struct ac_trie | 4.9 |  | 0.703 |
| walker |  | 6221 | 208 | Code::CodeKey { rung: Names, file: krep.c, decl: 0, sub: 0, line: 0 } |  |  | 0.703 |
| walker |  | 6228 | 7 | Code::CodeKey { rung: Doc, file: krep.c, decl: 4, sub: 0, line: 77 } |  |  | 0.703 |
| walker |  | 6239 | 11 | Code::CodeKey { rung: Doc, file: krep.c, decl: 2, sub: 0, line: 42 } |  |  | 0.703 |
| walker |  | 6253 | 14 | Code::CodeKey { rung: Doc, file: krep.c, decl: 3, sub: 0, line: 44 } |  |  | 0.703 |
| walker |  | 6269 | 16 | Code::CodeKey { rung: Doc, file: krep.c, decl: 1, sub: 0, line: 39 } |  |  | 0.703 |
| ns | 6272 |  | 258 | test/test_krep.c: complete roster of test functions, helpers and main | 4.10 |  | 0.684 |
| ns | 6497 |  | 225 | test/test_regex.c and test/test_multiple_patterns.c rosters | 4.11 |  | 0.669 |
| walker |  | 6499 | 230 | Code::CodeKey { rung: Names, file: krep.c, decl: 0, sub: 1, line: 0 } |  |  | 0.670 |
| walker |  | 6512 | 13 | Code::CodeKey { rung: Decl, file: krep.c, decl: 21, sub: 0, line: 104 } |  |  | 0.670 |
| walker |  | 6527 | 15 | Code::CodeKey { rung: Decl, file: krep.c, decl: 20, sub: 0, line: 101 } |  |  | 0.670 |
| walker |  | 6536 | 9 | Code::CodeKey { rung: Doc, file: krep.c, decl: 15, sub: 0, line: 93 } |  |  | 0.670 |
| walker |  | 6549 | 13 | Code::CodeKey { rung: Doc, file: krep.c, decl: 21, sub: 0, line: 104 } |  |  | 0.670 |
| walker |  | 6563 | 14 | Code::CodeKey { rung: Doc, file: krep.c, decl: 20, sub: 0, line: 101 } |  |  | 0.670 |
| ns | 6608 |  | 111 | test/test_directory.c roster: the separate recursive-search integration binary | 4.12 |  | 0.662 |
| walker |  | 6796 | 233 | Code::CodeKey { rung: Names, file: krep.c, decl: 0, sub: 2, line: 0 } |  |  | 0.663 |
| walker |  | 6801 | 5 | Code::CodeKey { rung: Decl, file: krep.c, decl: 33, sub: 0, line: 128 } |  |  | 0.663 |
| walker |  | 6806 | 5 | Code::CodeKey { rung: Decl, file: krep.c, decl: 34, sub: 0, line: 139 } |  |  | 0.663 |
| walker |  | 6816 | 10 | Code::CodeKey { rung: Decl, file: krep.c, decl: 23, sub: 0, line: 109 } |  |  | 0.663 |
| walker |  | 6827 | 11 | Code::CodeKey { rung: Decl, file: krep.c, decl: 22, sub: 0, line: 107 } |  |  | 0.663 |
| walker |  | 6839 | 12 | Code::CodeKey { rung: Decl, file: krep.c, decl: 24, sub: 0, line: 111 } |  |  | 0.663 |
| ns | 6841 |  | 233 | test/test_compat.h: the TESTING wrapper roster and its guards | 4.13 |  | 0.652 |
| walker |  | 6849 | 10 | Code::CodeKey { rung: Doc, file: krep.c, decl: 34, sub: 0, line: 139 } |  |  | 0.652 |
| walker |  | 6860 | 11 | Code::CodeKey { rung: Doc, file: krep.c, decl: 33, sub: 0, line: 128 } |  |  | 0.652 |
| walker |  | 6872 | 12 | Code::CodeKey { rung: Doc, file: krep.c, decl: 32, sub: 0, line: 125 } |  |  | 0.652 |
| walker |  | 6886 | 14 | Code::CodeKey { rung: Doc, file: krep.c, decl: 25, sub: 0, line: 116 } |  |  | 0.652 |
| ns | 6972 |  | 131 | test/test_krep.h: shared test helper declarations | 4.14 |  | 0.643 |
| ns | 7080 |  | 108 | Makefile compiler configuration: CC, CFLAGS, LDFLAGS, PREFIX | 5.1 |  | 0.638 |
| walker |  | 7096 | 210 | Code::CodeKey { rung: Names, file: krep.c, decl: 0, sub: 3, line: 0 } |  |  | 0.650 |
| walker |  | 7101 | 5 | Code::CodeKey { rung: Decl, file: krep.c, decl: 35, sub: 0, line: 175 } |  |  | 0.650 |
| walker |  | 7106 | 5 | Code::CodeKey { rung: Decl, file: krep.c, decl: 36, sub: 0, line: 244 } |  |  | 0.650 |
| walker |  | 7111 | 5 | Code::CodeKey { rung: Decl, file: krep.c, decl: 37, sub: 0, line: 256 } |  |  | 0.650 |
| walker |  | 7116 | 5 | Code::CodeKey { rung: Decl, file: krep.c, decl: 39, sub: 0, line: 363 } |  |  | 0.650 |
| walker |  | 7121 | 5 | Code::CodeKey { rung: Decl, file: krep.c, decl: 40, sub: 0, line: 401 } |  |  | 0.650 |
| walker |  | 7126 | 5 | Code::CodeKey { rung: Decl, file: krep.c, decl: 41, sub: 0, line: 420 } |  |  | 0.650 |
| walker |  | 7131 | 5 | Code::CodeKey { rung: Decl, file: krep.c, decl: 42, sub: 0, line: 438 } |  |  | 0.650 |
| walker |  | 7168 | 37 | Code::CodeKey { rung: Decl, file: krep.c, decl: 38, sub: 0, line: 329 } |  |  | 0.650 |
| walker |  | 7181 | 13 | Code::CodeKey { rung: Doc, file: krep.c, decl: 36, sub: 0, line: 244 } |  |  | 0.650 |
| walker |  | 7197 | 16 | Code::CodeKey { rung: Doc, file: krep.c, decl: 40, sub: 0, line: 401 } |  |  | 0.650 |
| walker |  | 7215 | 18 | Code::CodeKey { rung: Doc, file: krep.c, decl: 35, sub: 0, line: 175 } |  |  | 0.650 |
| walker |  | 7233 | 18 | Code::CodeKey { rung: Doc, file: krep.c, decl: 41, sub: 0, line: 420 } |  |  | 0.650 |
| walker |  | 7253 | 20 | Code::CodeKey { rung: Doc, file: krep.c, decl: 38, sub: 0, line: 329 } |  |  | 0.650 |
| walker |  | 7285 | 32 | Code::CodeKey { rung: Doc, file: krep.c, decl: 42, sub: 0, line: 438 } |  |  | 0.650 |
| ns | 7316 |  | 236 | .github/workflows/ci.yml in full | 5.2 |  | 0.633 |
| walker |  | 7325 | 40 | Code::CodeKey { rung: Doc, file: krep.c, decl: 39, sub: 0, line: 363 } |  |  | 0.633 |
| walker |  | 7374 | 49 | Code::CodeKey { rung: Doc, file: krep.c, decl: 37, sub: 0, line: 256 } |  |  | 0.633 |
| ns | 7580 |  | 264 | Makefile architecture detection: which SIMD flags each arch gets | 5.3 |  | 0.621 |
| walker |  | 7590 | 216 | Code::CodeKey { rung: Names, file: krep.c, decl: 0, sub: 4, line: 0 } |  |  | 0.628 |
| walker |  | 7595 | 5 | Code::CodeKey { rung: Decl, file: krep.c, decl: 43, sub: 0, line: 461 } |  |  | 0.628 |
| walker |  | 7601 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 44, sub: 0, line: 1084 } |  |  | 0.628 |
| walker |  | 7607 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 45, sub: 0, line: 1125 } |  |  | 0.628 |
| walker |  | 7613 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 46, sub: 0, line: 1137 } |  |  | 0.628 |
| walker |  | 7619 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 47, sub: 0, line: 1198 } |  |  | 0.628 |
| walker |  | 7625 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 48, sub: 0, line: 1213 } |  |  | 0.628 |
| walker |  | 7666 | 41 | Code::CodeKey { rung: Decl, file: krep.c, decl: 50, sub: 0, line: 1389 } |  |  | 0.628 |
| walker |  | 7716 | 50 | Code::CodeKey { rung: Decl, file: krep.c, decl: 49, sub: 0, line: 1259 } |  |  | 0.628 |
| walker |  | 7726 | 10 | Code::CodeKey { rung: Doc, file: krep.c, decl: 46, sub: 0, line: 1137 } |  |  | 0.628 |
| walker |  | 7737 | 11 | Code::CodeKey { rung: Doc, file: krep.c, decl: 45, sub: 0, line: 1125 } |  |  | 0.628 |
| walker |  | 7752 | 15 | Code::CodeKey { rung: Doc, file: krep.c, decl: 48, sub: 0, line: 1213 } |  |  | 0.628 |
| walker |  | 7769 | 17 | Code::CodeKey { rung: Doc, file: krep.c, decl: 47, sub: 0, line: 1198 } |  |  | 0.628 |
| walker |  | 7805 | 36 | Code::CodeKey { rung: Doc, file: krep.c, decl: 49, sub: 0, line: 1259 } |  |  | 0.628 |
| walker |  | 7842 | 37 | Code::CodeKey { rung: Doc, file: krep.c, decl: 44, sub: 0, line: 1084 } |  |  | 0.628 |
| ns | 7883 |  | 303 | Makefile compile/link rules and the parallel -DTESTING build | 5.4 |  | 0.618 |
| ns | 8052 |  | 169 | Makefile run targets: test, test-directory, ci, bench-rg, all-tests | 5.5 |  | 0.609 |
| walker |  | 8075 | 233 | Code::CodeKey { rung: Names, file: krep.c, decl: 0, sub: 5, line: 0 } |  |  | 0.623 |
| walker |  | 8081 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 51, sub: 0, line: 1585 } |  |  | 0.623 |
| walker |  | 8087 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 53, sub: 0, line: 1771 } |  |  | 0.623 |
| walker |  | 8093 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 54, sub: 0, line: 1873 } |  |  | 0.623 |
| walker |  | 8099 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 55, sub: 0, line: 1919 } |  |  | 0.623 |
| walker |  | 8105 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 56, sub: 0, line: 1964 } |  |  | 0.623 |
| walker |  | 8111 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 57, sub: 0, line: 1999 } |  |  | 0.623 |
| walker |  | 8117 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 59, sub: 0, line: 2252 } |  |  | 0.623 |
| walker |  | 8123 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 60, sub: 0, line: 2265 } |  |  | 0.623 |
| walker |  | 8129 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 61, sub: 0, line: 2274 } |  |  | 0.623 |
| walker |  | 8179 | 50 | Code::CodeKey { rung: Decl, file: krep.c, decl: 52, sub: 0, line: 1628 } |  |  | 0.623 |
| walker |  | 8189 | 10 | Code::CodeKey { rung: Doc, file: krep.c, decl: 58, sub: 0, line: 2249 } |  |  | 0.623 |
| walker |  | 8202 | 13 | Code::CodeKey { rung: Doc, file: krep.c, decl: 60, sub: 0, line: 2265 } |  |  | 0.623 |
| ns | 8209 |  | 157 | .github/workflows/release.yml: tag trigger and release artifacts | 5.6 |  | 0.612 |
| walker |  | 8218 | 16 | Code::CodeKey { rung: Doc, file: krep.c, decl: 57, sub: 0, line: 1999 } |  |  | 0.612 |
| walker |  | 8234 | 16 | Code::CodeKey { rung: Doc, file: krep.c, decl: 59, sub: 0, line: 2252 } |  |  | 0.612 |
| walker |  | 8251 | 17 | Code::CodeKey { rung: Doc, file: krep.c, decl: 56, sub: 0, line: 1964 } |  |  | 0.612 |
| walker |  | 8270 | 19 | Code::CodeKey { rung: Doc, file: krep.c, decl: 54, sub: 0, line: 1873 } |  |  | 0.612 |
| walker |  | 8289 | 19 | Code::CodeKey { rung: Doc, file: krep.c, decl: 55, sub: 0, line: 1919 } |  |  | 0.612 |
| walker |  | 8345 | 56 | Code::CodeKey { rung: Doc, file: krep.c, decl: 52, sub: 0, line: 1628 } |  |  | 0.612 |
| walker |  | 8404 | 59 | Code::CodeKey { rung: Doc, file: krep.c, decl: 51, sub: 0, line: 1585 } |  |  | 0.612 |
| ns | 8462 |  | 253 | krep.c constants: VERSION and every performance tunable | 6.1 |  | 0.616 |
| walker |  | 8650 | 246 | Code::CodeKey { rung: Names, file: krep.c, decl: 0, sub: 6, line: 0 } |  |  | 0.630 |
| walker |  | 8656 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 62, sub: 0, line: 3071 } |  |  | 0.630 |
| walker |  | 8662 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 63, sub: 0, line: 3090 } |  |  | 0.630 |
| walker |  | 8668 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 64, sub: 0, line: 3122 } |  |  | 0.630 |
| walker |  | 8674 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 67, sub: 0, line: 3163 } |  |  | 0.630 |
| walker |  | 8680 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 68, sub: 0, line: 3180 } |  |  | 0.630 |
| walker |  | 8686 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 69, sub: 0, line: 3240 } |  |  | 0.630 |
| walker |  | 8692 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 70, sub: 0, line: 3272 } |  |  | 0.630 |
| ns | 8696 |  | 234 | krep.c global option state and the lower_table constructor | 6.2 |  | 0.631 |
| walker |  | 8698 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 71, sub: 0, line: 3297 } |  |  | 0.631 |
| walker |  | 8722 | 24 | Code::CodeKey { rung: Decl, file: krep.c, decl: 72, sub: 0, line: 3310 } |  |  | 0.631 |
| walker |  | 8756 | 34 | Code::CodeKey { rung: Decl, file: krep.c, decl: 65, sub: 0, line: 3146 } |  |  | 0.631 |
| walker |  | 8813 | 57 | Code::CodeKey { rung: Decl, file: krep.c, decl: 66, sub: 0, line: 3154 } |  |  | 0.631 |
| walker |  | 8827 | 14 | Code::CodeKey { rung: Doc, file: krep.c, decl: 67, sub: 0, line: 3163 } |  |  | 0.631 |
| walker |  | 8842 | 15 | Code::CodeKey { rung: Doc, file: krep.c, decl: 62, sub: 0, line: 3071 } |  |  | 0.631 |
| walker |  | 8857 | 15 | Code::CodeKey { rung: Doc, file: krep.c, decl: 63, sub: 0, line: 3090 } |  |  | 0.631 |
| walker |  | 8872 | 15 | Code::CodeKey { rung: Doc, file: krep.c, decl: 64, sub: 0, line: 3122 } |  |  | 0.631 |
| walker |  | 8887 | 15 | Code::CodeKey { rung: Doc, file: krep.c, decl: 65, sub: 0, line: 3146 } |  |  | 0.632 |
| walker |  | 8902 | 15 | Code::CodeKey { rung: Doc, file: krep.c, decl: 72, sub: 0, line: 3310 } |  |  | 0.632 |
| walker |  | 8920 | 18 | Code::CodeKey { rung: Doc, file: krep.c, decl: 68, sub: 0, line: 3180 } |  |  | 0.632 |
| walker |  | 8940 | 20 | Code::CodeKey { rung: Doc, file: krep.c, decl: 66, sub: 0, line: 3154 } |  |  | 0.632 |
| walker |  | 8960 | 20 | Code::CodeKey { rung: Doc, file: krep.c, decl: 70, sub: 0, line: 3272 } |  |  | 0.632 |
| ns | 8966 |  | 270 | main: the getopt_long table and short-option string | 6.3 |  | 0.624 |
| walker |  | 8981 | 21 | Code::CodeKey { rung: Doc, file: krep.c, decl: 71, sub: 0, line: 3297 } |  |  | 0.624 |
| walker |  | 9005 | 24 | Code::CodeKey { rung: Doc, file: krep.c, decl: 69, sub: 0, line: 3240 } |  |  | 0.624 |
| walker |  | 9207 | 202 | Code::CodeKey { rung: Names, file: krep.c, decl: 0, sub: 7, line: 0 } |  |  | 0.632 |
| walker |  | 9213 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 73, sub: 0, line: 3442 } |  |  | 0.632 |
| walker |  | 9219 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 75, sub: 0, line: 4046 } |  |  | 0.632 |
| walker |  | 9225 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 76, sub: 0, line: 4104 } |  |  | 0.632 |
| walker |  | 9231 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 77, sub: 0, line: 4209 } |  |  | 0.632 |
| walker |  | 9237 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 78, sub: 0, line: 4251 } |  |  | 0.632 |
| walker |  | 9243 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 79, sub: 0, line: 4313 } |  |  | 0.632 |
| walker |  | 9249 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 80, sub: 0, line: 4332 } |  |  | 0.632 |
| walker |  | 9290 | 41 | Code::CodeKey { rung: Decl, file: krep.c, decl: 74, sub: 0, line: 3891 } |  |  | 0.632 |
| ns | 9299 |  | 333 | select_search_algorithm: dispatch head through the short-pattern branch | 6.4 |  | 0.618 |
| walker |  | 9331 | 41 | Code::CodeKey { rung: Decl, file: krep.c, decl: 81, sub: 0, line: 4371 } |  |  | 0.618 |
| walker |  | 9342 | 11 | Code::CodeKey { rung: Doc, file: krep.c, decl: 80, sub: 0, line: 4332 } |  |  | 0.618 |
| walker |  | 9355 | 13 | Code::CodeKey { rung: Doc, file: krep.c, decl: 74, sub: 0, line: 3891 } |  |  | 0.618 |
| walker |  | 9368 | 13 | Code::CodeKey { rung: Doc, file: krep.c, decl: 79, sub: 0, line: 4313 } |  |  | 0.618 |
| walker |  | 9382 | 14 | Code::CodeKey { rung: Doc, file: krep.c, decl: 77, sub: 0, line: 4209 } |  |  | 0.618 |
| walker |  | 9397 | 15 | Code::CodeKey { rung: Doc, file: krep.c, decl: 78, sub: 0, line: 4251 } |  |  | 0.618 |
| walker |  | 9413 | 16 | Code::CodeKey { rung: Doc, file: krep.c, decl: 75, sub: 0, line: 4046 } |  |  | 0.618 |
| walker |  | 9431 | 18 | Code::CodeKey { rung: Doc, file: krep.c, decl: 76, sub: 0, line: 4104 } |  |  | 0.618 |
| walker |  | 9452 | 21 | Code::CodeKey { rung: Doc, file: krep.c, decl: 81, sub: 0, line: 4371 } |  |  | 0.619 |
| walker |  | 9487 | 35 | Code::CodeKey { rung: Doc, file: krep.c, decl: 73, sub: 0, line: 3442 } |  |  | 0.619 |
| walker |  | 9672 | 185 | Markdown::Section { file: README.md, section_index: 9, keeps_default_concavity: false } |  |  | 0.631 |
| ns | 9685 |  | 386 | select_search_algorithm: SIMD length limits and the KMP/Boyer-Moore fallback | 6.5 | 6.4 | 0.616 |
| ns | 9868 |  | 183 | gitignore data model: pattern record and parent-chained context | 6.6 |  | 0.621 |
| walker |  | 9870 | 198 | Markdown::Section { file: README.md, section_index: 10, keeps_default_concavity: false } |  |  | 0.638 |
| ns | 9993 |  | 125 | Licence header, dependabot config and .gitignore | 7.1 |  | 0.632 |
| walker |  | 9999 | 129 | Markdown::Section { file: README.md, section_index: 11, keeps_default_concavity: false } |  |  | 0.642 |
| walker |  | 9999 | 0 | Markdown::Section { file: README.md, section_index: 4, keeps_default_concavity: false } |  |  | 0.642 |
