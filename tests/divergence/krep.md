Score(3000)=0.607 I=0.832 C=0.443 ns_rows≤3K=20/56 grid(1000/1442/2080/3000/4327/6240/9000)=0.650/0.757/0.680/0.607/0.757/0.705/0.626

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
| walker |  | 738 | 21 | Markdown::Section { file: README.md, section_index: 12, keeps_default_concavity: false } |  |  | 0.867 |
| walker |  | 767 | 29 | Markdown::Section { file: README.md, section_index: 13, keeps_default_concavity: false } |  |  | 0.867 |
| ns | 789 |  | 230 | krep.h search-algorithm roster with the SIMD #if guards intact | 1.8 |  | 0.713 |
| ns | 996 |  | 207 | aho_corasick.h: the complete second-module interface | 1.9 |  | 0.644 |
| walker |  | 1119 | 352 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: true } |  |  | 0.655 |
| ns | 1173 |  | 177 | README Key Features, first half | 2.1 |  | 0.674 |
| walker |  | 1303 | 184 | Code::CodeKey { rung: Names, file: aho_corasick.h, decl: 0, sub: 0, line: 0 } |  |  | 0.701 |
| walker |  | 1313 | 10 | Code::CodeKey { rung: Doc, file: aho_corasick.h, decl: 5, sub: 0, line: 23 } |  |  | 0.701 |
| walker |  | 1325 | 12 | Code::CodeKey { rung: Doc, file: aho_corasick.h, decl: 9, sub: 0, line: 33 } |  |  | 0.709 |
| walker |  | 1337 | 12 | Code::CodeKey { rung: Doc, file: aho_corasick.h, decl: 11, sub: 0, line: 39 } |  |  | 0.718 |
| ns | 1343 |  | 170 | README Key Features, second half | 2.2 |  | 0.730 |
| walker |  | 1351 | 14 | Code::CodeKey { rung: Doc, file: aho_corasick.h, decl: 6, sub: 0, line: 26 } |  |  | 0.730 |
| walker |  | 1366 | 15 | Code::CodeKey { rung: Doc, file: aho_corasick.h, decl: 8, sub: 0, line: 30 } |  |  | 0.741 |
| walker |  | 1384 | 18 | Code::CodeKey { rung: Doc, file: aho_corasick.h, decl: 1, sub: 0, line: 14 } |  |  | 0.741 |
| walker |  | 1404 | 20 | Code::CodeKey { rung: Doc, file: aho_corasick.h, decl: 10, sub: 0, line: 36 } |  |  | 0.757 |
| walker |  | 1439 | 35 | Code::CodeKey { rung: Doc, file: aho_corasick.h, decl: 3, sub: 0, line: 19 } |  |  | 0.757 |
| walker |  | 1503 | 64 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.757 |
| ns | 1534 |  | 191 | README command-line options table, first half (-i through -F) | 2.3 |  | 0.725 |
| walker |  | 1547 | 44 | Markdown::Section { file: README.md, section_index: 14, keeps_default_concavity: false } |  |  | 0.725 |
| ns | 1753 |  | 219 | README command-line options table, second half (-r through -h) | 2.4 |  | 0.689 |
| walker |  | 1763 | 216 | Code::CodeKey { rung: Names, file: krep.h, decl: 0, sub: 0, line: 0 } |  |  | 0.690 |
| walker |  | 1772 | 9 | Code::CodeKey { rung: Doc, file: krep.h, decl: 10, sub: 0, line: 42 } |  |  | 0.690 |
| walker |  | 1786 | 14 | Code::CodeKey { rung: Doc, file: krep.h, decl: 1, sub: 0, line: 19 } |  |  | 0.690 |
| walker |  | 1800 | 14 | Code::CodeKey { rung: Doc, file: krep.h, decl: 2, sub: 0, line: 22 } |  |  | 0.690 |
| walker |  | 1820 | 20 | Code::CodeKey { rung: Doc, file: krep.h, decl: 4, sub: 0, line: 34 } |  |  | 0.690 |
| ns | 1963 |  | 210 | README: the documented smart algorithm-selection policy | 2.5 |  | 0.653 |
| ns | 2069 |  | 106 | README usage examples: the six concrete invocations, command lines only | 2.6 |  | 0.636 |
| ns | 2181 |  | 112 | README: the multi-threading architecture section | 2.7 |  | 0.611 |
| walker |  | 2235 | 415 | Markdown::Section { file: README.md, section_index: 7, keeps_default_concavity: true } |  |  | 0.700 |
| ns | 2287 |  | 106 | README: the recursive-search skipping rules | 2.8 |  | 0.679 |
| walker |  | 2357 | 122 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.679 |
| ns | 2402 |  | 115 | README scope note: explicitly not a grep/ripgrep replacement | 2.9 |  | 0.684 |
| ns | 2512 |  | 110 | README install and build-from-source commands | 2.10 |  | 0.659 |
| walker |  | 2635 | 278 | Code::CodeKey { rung: Names, file: krep.h, decl: 0, sub: 1, line: 0 } |  |  | 0.661 |
| walker |  | 2667 | 32 | Code::CodeKey { rung: Decl, file: krep.h, decl: 17, sub: 0, line: 98 } |  |  | 0.662 |
| walker |  | 2707 | 40 | Code::CodeKey { rung: Decl, file: krep.h, decl: 14, sub: 0, line: 49 } |  |  | 0.662 |
| walker |  | 2758 | 51 | Code::CodeKey { rung: Decl, file: krep.h, decl: 19, sub: 0, line: 125 } |  |  | 0.662 |
| walker |  | 2767 | 9 | Code::CodeKey { rung: Doc, file: krep.h, decl: 19, sub: 0, line: 125 } |  |  | 0.662 |
| walker |  | 2776 | 9 | Code::CodeKey { rung: Doc, file: krep.h, decl: 21, sub: 0, line: 146 } |  |  | 0.663 |
| walker |  | 2838 | 62 | Code::CodeKey { rung: Decl, file: krep.h, decl: 15, sub: 0, line: 55 } |  |  | 0.665 |
| walker |  | 2861 | 23 | Code::CodeKey { rung: Doc, file: krep.h, decl: 14, sub: 0, line: 49 } |  |  | 0.666 |
| ns | 2902 |  | 390 | search_params_t: the struct every search function takes | 3.1 |  | 0.607 |
| walker |  | 3022 | 161 | Code::CodeKey { rung: Decl, file: krep.h, decl: 20, sub: 0, line: 132 } |  |  | 0.607 |
| walker |  | 3059 | 37 | Code::CodeKey { rung: Doc, file: krep.h, decl: 17, sub: 0, line: 98 } |  |  | 0.608 |
| ns | 3089 |  | 187 | match_position_t and match_result_t | 3.2 |  | 0.629 |
| ns | 3180 |  | 91 | search_func_t: the algorithm function-pointer type | 3.3 |  | 0.636 |
| ns | 3282 |  | 102 | krep.h thread-pool API: all four functions in full | 3.4 |  | 0.634 |
| walker |  | 3309 | 250 | Code::CodeKey { rung: Decl, file: krep.h, decl: 18, sub: 0, line: 104 } |  |  | 0.639 |
| walker |  | 3319 | 10 | Code::CodeKey { rung: Doc, file: krep.h, decl: 18, sub: 0, line: 104 } |  |  | 0.640 |
| ns | 3562 |  | 280 | thread_data_t: per-thread search state, including the false-sharing padding | 3.5 |  | 0.661 |
| ns | 3659 |  | 97 | krep.h helper roster: line finding, word matching, table preparation | 3.6 |  | 0.649 |
| walker |  | 3687 | 368 | Code::CodeKey { rung: Decl, file: krep.h, decl: 16, sub: 0, line: 65 } |  |  | 0.729 |
| walker |  | 3700 | 13 | Code::CodeKey { rung: Doc, file: krep.h, decl: 16, sub: 0, line: 65 } |  |  | 0.729 |
| ns | 3826 |  | 167 | skip_directories: the complete recursive-search directory blocklist | 3.7 |  | 0.715 |
| walker |  | 3926 | 226 | Code::CodeKey { rung: Names, file: krep.h, decl: 0, sub: 2, line: 0 } |  |  | 0.750 |
| walker |  | 3936 | 10 | Code::CodeKey { rung: Doc, file: krep.h, decl: 28, sub: 0, line: 183 } |  |  | 0.750 |
| ns | 4011 |  | 185 | skip_extensions: head, ellipsis, and the count line | 3.8 |  | 0.738 |
| walker |  | 4026 | 90 | Code::CodeKey { rung: Doc, file: krep.h, decl: 26, sub: 0, line: 170 } |  |  | 0.738 |
| walker |  | 4223 | 197 | Code::CodeKey { rung: Names, file: krep.h, decl: 0, sub: 3, line: 0 } |  |  | 0.748 |
| walker |  | 4230 | 7 | Code::CodeKey { rung: Doc, file: krep.h, decl: 33, sub: 0, line: 203 } |  |  | 0.748 |
| ns | 4236 |  | 225 | ANSI colour macro block, output palette and help palette | 3.9 |  | 0.753 |
| ns | 4395 |  | 159 | Documented contracts of the three entry points and the printer | 3.10 | 1.7 | 0.740 |
| walker |  | 4412 | 182 | Code::CodeKey { rung: Names, file: krep.h, decl: 0, sub: 4, line: 0 } |  |  | 0.750 |
| walker |  | 4431 | 19 | Code::CodeKey { rung: Decl, file: krep.h, decl: 40, sub: 0, line: 218 } |  |  | 0.755 |
| walker |  | 4450 | 19 | Code::CodeKey { rung: Decl, file: krep.h, decl: 39, sub: 0, line: 214 } |  |  | 0.762 |
| walker |  | 4469 | 19 | Code::CodeKey { rung: Decl, file: krep.h, decl: 42, sub: 0, line: 226 } |  |  | 0.768 |
| walker |  | 4493 | 24 | Code::CodeKey { rung: Decl, file: krep.h, decl: 41, sub: 0, line: 222 } |  |  | 0.779 |
| walker |  | 4501 | 8 | Code::CodeKey { rung: Doc, file: krep.h, decl: 43, sub: 0, line: 231 } |  |  | 0.779 |
| walker |  | 4597 | 96 | Code::CodeKey { rung: Doc, file: krep.h, decl: 27, sub: 0, line: 180 } |  |  | 0.780 |
| ns | 4634 |  | 239 | krep.c file-level section banner map, all sixteen top-level banners | 4.1 |  | 0.756 |
| walker |  | 4756 | 159 | Code::CodeKey { rung: Names, file: krep.h, decl: 0, sub: 5, line: 0 } |  |  | 0.773 |
| ns | 4758 |  | 124 | krep.c definition roster, lines 139-461: match results, line finding, printing | 4.2 |  | 0.759 |
| walker |  | 4761 | 5 | Code::CodeKey { rung: Decl, file: krep.h, decl: 49, sub: 0, line: 298 } |  |  | 0.759 |
| walker |  | 4766 | 5 | Code::CodeKey { rung: Decl, file: krep.h, decl: 50, sub: 0, line: 312 } |  |  | 0.759 |
| walker |  | 4834 | 68 | Code::CodeKey { rung: Doc, file: krep.h, decl: 49, sub: 0, line: 298 } |  |  | 0.759 |
| walker |  | 4931 | 97 | Code::CodeKey { rung: Doc, file: krep.h, decl: 47, sub: 0, line: 278 } |  |  | 0.759 |
| ns | 4932 |  | 174 | krep.c definition roster, lines 1084-1964: utilities, the four scalar algorithms, orchestration | 4.3 |  | 0.742 |
| walker |  | 5033 | 102 | Code::CodeKey { rung: Doc, file: krep.h, decl: 48, sub: 0, line: 288 } |  |  | 0.742 |
| ns | 5124 |  | 192 | krep.c definition roster, lines 1999-3442: public API bodies, skip filters, gitignore | 4.4 |  | 0.724 |
| walker |  | 5140 | 107 | Code::CodeKey { rung: Doc, file: krep.h, decl: 50, sub: 0, line: 312 } |  |  | 0.724 |
| walker |  | 5256 | 116 | Code::CodeKey { rung: Doc, file: krep.h, decl: 25, sub: 0, line: 161 } |  |  | 0.732 |
| ns | 5363 |  | 239 | krep.c definition roster, lines 3451-5108: main, thread pool, and the guarded SIMD kernels | 4.5 |  | 0.712 |
| walker |  | 5425 | 169 | Code::CodeKey { rung: Doc, file: krep.h, decl: 32, sub: 0, line: 200 } |  |  | 0.720 |
| ns | 5546 |  | 183 | search_file internal section map, all eleven banners inside lines 2274-3070 | 4.6 |  | 0.708 |
| ns | 5668 |  | 122 | print_matching_items internal section map: the -o and full-line output modes | 4.7 |  | 0.700 |
| walker |  | 5671 | 246 | Code::CodeKey { rung: Names, file: aho_corasick.c, decl: 0, sub: 0, line: 0 } |  |  | 0.701 |
| walker |  | 5676 | 5 | Code::CodeKey { rung: Decl, file: aho_corasick.c, decl: 3, sub: 0, line: 34 } |  |  | 0.701 |
| walker |  | 5681 | 5 | Code::CodeKey { rung: Decl, file: aho_corasick.c, decl: 4, sub: 0, line: 55 } |  |  | 0.701 |
| walker |  | 5686 | 5 | Code::CodeKey { rung: Decl, file: aho_corasick.c, decl: 5, sub: 0, line: 86 } |  |  | 0.701 |
| walker |  | 5691 | 5 | Code::CodeKey { rung: Decl, file: aho_corasick.c, decl: 6, sub: 0, line: 111 } |  |  | 0.701 |
| walker |  | 5696 | 5 | Code::CodeKey { rung: Decl, file: aho_corasick.c, decl: 7, sub: 0, line: 274 } |  |  | 0.701 |
| walker |  | 5701 | 5 | Code::CodeKey { rung: Decl, file: aho_corasick.c, decl: 8, sub: 0, line: 287 } |  |  | 0.701 |
| walker |  | 5738 | 37 | Code::CodeKey { rung: Decl, file: aho_corasick.c, decl: 12, sub: 0, line: 299 } |  |  | 0.701 |
| walker |  | 5801 | 63 | Code::CodeKey { rung: Decl, file: aho_corasick.c, decl: 2, sub: 0, line: 26 } |  |  | 0.701 |
| walker |  | 5810 | 9 | Code::CodeKey { rung: Doc, file: aho_corasick.c, decl: 3, sub: 0, line: 34 } |  |  | 0.701 |
| walker |  | 5822 | 12 | Code::CodeKey { rung: Doc, file: aho_corasick.c, decl: 4, sub: 0, line: 55 } |  |  | 0.706 |
| ns | 5822 |  | 154 | aho_corasick.c: complete definition roster | 4.8 |  | 0.706 |
| walker |  | 5835 | 13 | Code::CodeKey { rung: Doc, file: aho_corasick.c, decl: 12, sub: 0, line: 299 } |  |  | 0.706 |
| walker |  | 5849 | 14 | Code::CodeKey { rung: Doc, file: aho_corasick.c, decl: 6, sub: 0, line: 111 } |  |  | 0.706 |
| walker |  | 5863 | 14 | Code::CodeKey { rung: Doc, file: aho_corasick.c, decl: 7, sub: 0, line: 274 } |  |  | 0.706 |
| walker |  | 5877 | 14 | Code::CodeKey { rung: Doc, file: aho_corasick.c, decl: 11, sub: 0, line: 296 } |  |  | 0.706 |
| walker |  | 5892 | 15 | Code::CodeKey { rung: Doc, file: aho_corasick.c, decl: 5, sub: 0, line: 86 } |  |  | 0.706 |
| walker |  | 5994 | 102 | Code::CodeKey { rung: Decl, file: aho_corasick.c, decl: 1, sub: 0, line: 17 } |  |  | 0.707 |
| ns | 6014 |  | 192 | aho_corasick.c trie data model: ac_node_t and struct ac_trie | 4.9 |  | 0.705 |
| walker |  | 6015 | 21 | Code::CodeKey { rung: Doc, file: aho_corasick.c, decl: 9, sub: 0, line: 293 } |  |  | 0.705 |
| walker |  | 6037 | 22 | Code::CodeKey { rung: Doc, file: aho_corasick.c, decl: 8, sub: 0, line: 287 } |  |  | 0.705 |
| walker |  | 6245 | 208 | Code::CodeKey { rung: Names, file: krep.c, decl: 0, sub: 0, line: 0 } |  |  | 0.705 |
| walker |  | 6252 | 7 | Code::CodeKey { rung: Doc, file: krep.c, decl: 4, sub: 0, line: 77 } |  |  | 0.705 |
| walker |  | 6263 | 11 | Code::CodeKey { rung: Doc, file: krep.c, decl: 2, sub: 0, line: 42 } |  |  | 0.705 |
| ns | 6272 |  | 258 | test/test_krep.c: complete roster of test functions, helpers and main | 4.10 |  | 0.686 |
| walker |  | 6277 | 14 | Code::CodeKey { rung: Doc, file: krep.c, decl: 3, sub: 0, line: 44 } |  |  | 0.686 |
| walker |  | 6293 | 16 | Code::CodeKey { rung: Doc, file: krep.c, decl: 1, sub: 0, line: 39 } |  |  | 0.686 |
| ns | 6497 |  | 225 | test/test_regex.c and test/test_multiple_patterns.c rosters | 4.11 |  | 0.671 |
| walker |  | 6523 | 230 | Code::CodeKey { rung: Names, file: krep.c, decl: 0, sub: 1, line: 0 } |  |  | 0.672 |
| walker |  | 6536 | 13 | Code::CodeKey { rung: Decl, file: krep.c, decl: 21, sub: 0, line: 104 } |  |  | 0.672 |
| walker |  | 6551 | 15 | Code::CodeKey { rung: Decl, file: krep.c, decl: 20, sub: 0, line: 101 } |  |  | 0.672 |
| walker |  | 6560 | 9 | Code::CodeKey { rung: Doc, file: krep.c, decl: 15, sub: 0, line: 93 } |  |  | 0.672 |
| walker |  | 6573 | 13 | Code::CodeKey { rung: Doc, file: krep.c, decl: 21, sub: 0, line: 104 } |  |  | 0.672 |
| walker |  | 6587 | 14 | Code::CodeKey { rung: Doc, file: krep.c, decl: 20, sub: 0, line: 101 } |  |  | 0.672 |
| ns | 6608 |  | 111 | test/test_directory.c roster: the separate recursive-search integration binary | 4.12 |  | 0.664 |
| walker |  | 6820 | 233 | Code::CodeKey { rung: Names, file: krep.c, decl: 0, sub: 2, line: 0 } |  |  | 0.665 |
| walker |  | 6825 | 5 | Code::CodeKey { rung: Decl, file: krep.c, decl: 33, sub: 0, line: 128 } |  |  | 0.665 |
| walker |  | 6830 | 5 | Code::CodeKey { rung: Decl, file: krep.c, decl: 34, sub: 0, line: 139 } |  |  | 0.665 |
| walker |  | 6840 | 10 | Code::CodeKey { rung: Decl, file: krep.c, decl: 23, sub: 0, line: 109 } |  |  | 0.665 |
| ns | 6841 |  | 233 | test/test_compat.h: the TESTING wrapper roster and its guards | 4.13 |  | 0.654 |
| walker |  | 6851 | 11 | Code::CodeKey { rung: Decl, file: krep.c, decl: 22, sub: 0, line: 107 } |  |  | 0.654 |
| walker |  | 6863 | 12 | Code::CodeKey { rung: Decl, file: krep.c, decl: 24, sub: 0, line: 111 } |  |  | 0.654 |
| walker |  | 6873 | 10 | Code::CodeKey { rung: Doc, file: krep.c, decl: 34, sub: 0, line: 139 } |  |  | 0.654 |
| walker |  | 6884 | 11 | Code::CodeKey { rung: Doc, file: krep.c, decl: 33, sub: 0, line: 128 } |  |  | 0.654 |
| walker |  | 6896 | 12 | Code::CodeKey { rung: Doc, file: krep.c, decl: 32, sub: 0, line: 125 } |  |  | 0.654 |
| walker |  | 6910 | 14 | Code::CodeKey { rung: Doc, file: krep.c, decl: 25, sub: 0, line: 116 } |  |  | 0.654 |
| ns | 6972 |  | 131 | test/test_krep.h: shared test helper declarations | 4.14 |  | 0.645 |
| ns | 7080 |  | 108 | Makefile compiler configuration: CC, CFLAGS, LDFLAGS, PREFIX | 5.1 |  | 0.640 |
| walker |  | 7120 | 210 | Code::CodeKey { rung: Names, file: krep.c, decl: 0, sub: 3, line: 0 } |  |  | 0.652 |
| walker |  | 7125 | 5 | Code::CodeKey { rung: Decl, file: krep.c, decl: 35, sub: 0, line: 175 } |  |  | 0.652 |
| walker |  | 7130 | 5 | Code::CodeKey { rung: Decl, file: krep.c, decl: 36, sub: 0, line: 244 } |  |  | 0.652 |
| walker |  | 7135 | 5 | Code::CodeKey { rung: Decl, file: krep.c, decl: 37, sub: 0, line: 256 } |  |  | 0.652 |
| walker |  | 7140 | 5 | Code::CodeKey { rung: Decl, file: krep.c, decl: 39, sub: 0, line: 363 } |  |  | 0.652 |
| walker |  | 7145 | 5 | Code::CodeKey { rung: Decl, file: krep.c, decl: 40, sub: 0, line: 401 } |  |  | 0.652 |
| walker |  | 7150 | 5 | Code::CodeKey { rung: Decl, file: krep.c, decl: 41, sub: 0, line: 420 } |  |  | 0.652 |
| walker |  | 7155 | 5 | Code::CodeKey { rung: Decl, file: krep.c, decl: 42, sub: 0, line: 438 } |  |  | 0.652 |
| walker |  | 7192 | 37 | Code::CodeKey { rung: Decl, file: krep.c, decl: 38, sub: 0, line: 329 } |  |  | 0.652 |
| walker |  | 7205 | 13 | Code::CodeKey { rung: Doc, file: krep.c, decl: 36, sub: 0, line: 244 } |  |  | 0.652 |
| walker |  | 7221 | 16 | Code::CodeKey { rung: Doc, file: krep.c, decl: 40, sub: 0, line: 401 } |  |  | 0.652 |
| walker |  | 7239 | 18 | Code::CodeKey { rung: Doc, file: krep.c, decl: 35, sub: 0, line: 175 } |  |  | 0.652 |
| walker |  | 7257 | 18 | Code::CodeKey { rung: Doc, file: krep.c, decl: 41, sub: 0, line: 420 } |  |  | 0.652 |
| walker |  | 7277 | 20 | Code::CodeKey { rung: Doc, file: krep.c, decl: 38, sub: 0, line: 329 } |  |  | 0.652 |
| walker |  | 7309 | 32 | Code::CodeKey { rung: Doc, file: krep.c, decl: 42, sub: 0, line: 438 } |  |  | 0.652 |
| ns | 7316 |  | 236 | .github/workflows/ci.yml in full | 5.2 |  | 0.635 |
| walker |  | 7349 | 40 | Code::CodeKey { rung: Doc, file: krep.c, decl: 39, sub: 0, line: 363 } |  |  | 0.635 |
| walker |  | 7398 | 49 | Code::CodeKey { rung: Doc, file: krep.c, decl: 37, sub: 0, line: 256 } |  |  | 0.635 |
| ns | 7580 |  | 264 | Makefile architecture detection: which SIMD flags each arch gets | 5.3 |  | 0.623 |
| walker |  | 7614 | 216 | Code::CodeKey { rung: Names, file: krep.c, decl: 0, sub: 4, line: 0 } |  |  | 0.631 |
| walker |  | 7619 | 5 | Code::CodeKey { rung: Decl, file: krep.c, decl: 43, sub: 0, line: 461 } |  |  | 0.631 |
| walker |  | 7625 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 44, sub: 0, line: 1084 } |  |  | 0.631 |
| walker |  | 7631 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 45, sub: 0, line: 1125 } |  |  | 0.631 |
| walker |  | 7637 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 46, sub: 0, line: 1137 } |  |  | 0.631 |
| walker |  | 7643 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 47, sub: 0, line: 1198 } |  |  | 0.631 |
| walker |  | 7649 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 48, sub: 0, line: 1213 } |  |  | 0.631 |
| walker |  | 7690 | 41 | Code::CodeKey { rung: Decl, file: krep.c, decl: 50, sub: 0, line: 1389 } |  |  | 0.631 |
| walker |  | 7740 | 50 | Code::CodeKey { rung: Decl, file: krep.c, decl: 49, sub: 0, line: 1259 } |  |  | 0.631 |
| walker |  | 7750 | 10 | Code::CodeKey { rung: Doc, file: krep.c, decl: 46, sub: 0, line: 1137 } |  |  | 0.631 |
| walker |  | 7761 | 11 | Code::CodeKey { rung: Doc, file: krep.c, decl: 45, sub: 0, line: 1125 } |  |  | 0.631 |
| walker |  | 7776 | 15 | Code::CodeKey { rung: Doc, file: krep.c, decl: 48, sub: 0, line: 1213 } |  |  | 0.631 |
| walker |  | 7793 | 17 | Code::CodeKey { rung: Doc, file: krep.c, decl: 47, sub: 0, line: 1198 } |  |  | 0.631 |
| walker |  | 7829 | 36 | Code::CodeKey { rung: Doc, file: krep.c, decl: 49, sub: 0, line: 1259 } |  |  | 0.631 |
| walker |  | 7866 | 37 | Code::CodeKey { rung: Doc, file: krep.c, decl: 44, sub: 0, line: 1084 } |  |  | 0.631 |
| ns | 7883 |  | 303 | Makefile compile/link rules and the parallel -DTESTING build | 5.4 |  | 0.620 |
| ns | 8052 |  | 169 | Makefile run targets: test, test-directory, ci, bench-rg, all-tests | 5.5 |  | 0.611 |
| walker |  | 8099 | 233 | Code::CodeKey { rung: Names, file: krep.c, decl: 0, sub: 5, line: 0 } |  |  | 0.625 |
| walker |  | 8105 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 51, sub: 0, line: 1585 } |  |  | 0.625 |
| walker |  | 8111 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 53, sub: 0, line: 1771 } |  |  | 0.625 |
| walker |  | 8117 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 54, sub: 0, line: 1873 } |  |  | 0.625 |
| walker |  | 8123 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 55, sub: 0, line: 1919 } |  |  | 0.625 |
| walker |  | 8129 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 56, sub: 0, line: 1964 } |  |  | 0.625 |
| walker |  | 8135 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 57, sub: 0, line: 1999 } |  |  | 0.625 |
| walker |  | 8141 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 59, sub: 0, line: 2252 } |  |  | 0.625 |
| walker |  | 8147 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 60, sub: 0, line: 2265 } |  |  | 0.625 |
| walker |  | 8153 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 61, sub: 0, line: 2274 } |  |  | 0.625 |
| walker |  | 8203 | 50 | Code::CodeKey { rung: Decl, file: krep.c, decl: 52, sub: 0, line: 1628 } |  |  | 0.625 |
| ns | 8209 |  | 157 | .github/workflows/release.yml: tag trigger and release artifacts | 5.6 |  | 0.614 |
| walker |  | 8213 | 10 | Code::CodeKey { rung: Doc, file: krep.c, decl: 58, sub: 0, line: 2249 } |  |  | 0.614 |
| walker |  | 8226 | 13 | Code::CodeKey { rung: Doc, file: krep.c, decl: 60, sub: 0, line: 2265 } |  |  | 0.614 |
| walker |  | 8242 | 16 | Code::CodeKey { rung: Doc, file: krep.c, decl: 57, sub: 0, line: 1999 } |  |  | 0.614 |
| walker |  | 8258 | 16 | Code::CodeKey { rung: Doc, file: krep.c, decl: 59, sub: 0, line: 2252 } |  |  | 0.614 |
| walker |  | 8275 | 17 | Code::CodeKey { rung: Doc, file: krep.c, decl: 56, sub: 0, line: 1964 } |  |  | 0.614 |
| walker |  | 8294 | 19 | Code::CodeKey { rung: Doc, file: krep.c, decl: 54, sub: 0, line: 1873 } |  |  | 0.614 |
| walker |  | 8313 | 19 | Code::CodeKey { rung: Doc, file: krep.c, decl: 55, sub: 0, line: 1919 } |  |  | 0.614 |
| walker |  | 8369 | 56 | Code::CodeKey { rung: Doc, file: krep.c, decl: 52, sub: 0, line: 1628 } |  |  | 0.614 |
| walker |  | 8428 | 59 | Code::CodeKey { rung: Doc, file: krep.c, decl: 51, sub: 0, line: 1585 } |  |  | 0.614 |
| ns | 8462 |  | 253 | krep.c constants: VERSION and every performance tunable | 6.1 |  | 0.618 |
| walker |  | 8674 | 246 | Code::CodeKey { rung: Names, file: krep.c, decl: 0, sub: 6, line: 0 } |  |  | 0.632 |
| walker |  | 8680 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 62, sub: 0, line: 3071 } |  |  | 0.632 |
| walker |  | 8686 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 63, sub: 0, line: 3090 } |  |  | 0.632 |
| walker |  | 8692 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 64, sub: 0, line: 3122 } |  |  | 0.632 |
| ns | 8696 |  | 234 | krep.c global option state and the lower_table constructor | 6.2 |  | 0.633 |
| walker |  | 8698 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 67, sub: 0, line: 3163 } |  |  | 0.633 |
| walker |  | 8704 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 68, sub: 0, line: 3180 } |  |  | 0.633 |
| walker |  | 8710 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 69, sub: 0, line: 3240 } |  |  | 0.633 |
| walker |  | 8716 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 70, sub: 0, line: 3272 } |  |  | 0.633 |
| walker |  | 8722 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 71, sub: 0, line: 3297 } |  |  | 0.633 |
| walker |  | 8746 | 24 | Code::CodeKey { rung: Decl, file: krep.c, decl: 72, sub: 0, line: 3310 } |  |  | 0.633 |
| walker |  | 8780 | 34 | Code::CodeKey { rung: Decl, file: krep.c, decl: 65, sub: 0, line: 3146 } |  |  | 0.633 |
| walker |  | 8837 | 57 | Code::CodeKey { rung: Decl, file: krep.c, decl: 66, sub: 0, line: 3154 } |  |  | 0.633 |
| walker |  | 8851 | 14 | Code::CodeKey { rung: Doc, file: krep.c, decl: 67, sub: 0, line: 3163 } |  |  | 0.633 |
| walker |  | 8866 | 15 | Code::CodeKey { rung: Doc, file: krep.c, decl: 62, sub: 0, line: 3071 } |  |  | 0.633 |
| walker |  | 8881 | 15 | Code::CodeKey { rung: Doc, file: krep.c, decl: 63, sub: 0, line: 3090 } |  |  | 0.633 |
| walker |  | 8896 | 15 | Code::CodeKey { rung: Doc, file: krep.c, decl: 64, sub: 0, line: 3122 } |  |  | 0.633 |
| walker |  | 8911 | 15 | Code::CodeKey { rung: Doc, file: krep.c, decl: 65, sub: 0, line: 3146 } |  |  | 0.633 |
| walker |  | 8926 | 15 | Code::CodeKey { rung: Doc, file: krep.c, decl: 72, sub: 0, line: 3310 } |  |  | 0.633 |
| walker |  | 8944 | 18 | Code::CodeKey { rung: Doc, file: krep.c, decl: 68, sub: 0, line: 3180 } |  |  | 0.633 |
| walker |  | 8964 | 20 | Code::CodeKey { rung: Doc, file: krep.c, decl: 66, sub: 0, line: 3154 } |  |  | 0.634 |
| ns | 8966 |  | 270 | main: the getopt_long table and short-option string | 6.3 |  | 0.626 |
| walker |  | 8984 | 20 | Code::CodeKey { rung: Doc, file: krep.c, decl: 70, sub: 0, line: 3272 } |  |  | 0.626 |
| walker |  | 9005 | 21 | Code::CodeKey { rung: Doc, file: krep.c, decl: 71, sub: 0, line: 3297 } |  |  | 0.626 |
| walker |  | 9029 | 24 | Code::CodeKey { rung: Doc, file: krep.c, decl: 69, sub: 0, line: 3240 } |  |  | 0.626 |
| walker |  | 9231 | 202 | Code::CodeKey { rung: Names, file: krep.c, decl: 0, sub: 7, line: 0 } |  |  | 0.634 |
| walker |  | 9237 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 73, sub: 0, line: 3442 } |  |  | 0.634 |
| walker |  | 9243 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 75, sub: 0, line: 4046 } |  |  | 0.634 |
| walker |  | 9249 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 76, sub: 0, line: 4104 } |  |  | 0.634 |
| walker |  | 9255 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 77, sub: 0, line: 4209 } |  |  | 0.634 |
| walker |  | 9261 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 78, sub: 0, line: 4251 } |  |  | 0.634 |
| walker |  | 9267 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 79, sub: 0, line: 4313 } |  |  | 0.634 |
| walker |  | 9273 | 6 | Code::CodeKey { rung: Decl, file: krep.c, decl: 80, sub: 0, line: 4332 } |  |  | 0.634 |
| ns | 9299 |  | 333 | select_search_algorithm: dispatch head through the short-pattern branch | 6.4 |  | 0.620 |
| walker |  | 9314 | 41 | Code::CodeKey { rung: Decl, file: krep.c, decl: 74, sub: 0, line: 3891 } |  |  | 0.620 |
| walker |  | 9355 | 41 | Code::CodeKey { rung: Decl, file: krep.c, decl: 81, sub: 0, line: 4371 } |  |  | 0.620 |
| walker |  | 9366 | 11 | Code::CodeKey { rung: Doc, file: krep.c, decl: 80, sub: 0, line: 4332 } |  |  | 0.620 |
| walker |  | 9379 | 13 | Code::CodeKey { rung: Doc, file: krep.c, decl: 74, sub: 0, line: 3891 } |  |  | 0.620 |
| walker |  | 9392 | 13 | Code::CodeKey { rung: Doc, file: krep.c, decl: 79, sub: 0, line: 4313 } |  |  | 0.620 |
| walker |  | 9406 | 14 | Code::CodeKey { rung: Doc, file: krep.c, decl: 77, sub: 0, line: 4209 } |  |  | 0.620 |
| walker |  | 9421 | 15 | Code::CodeKey { rung: Doc, file: krep.c, decl: 78, sub: 0, line: 4251 } |  |  | 0.620 |
| walker |  | 9437 | 16 | Code::CodeKey { rung: Doc, file: krep.c, decl: 75, sub: 0, line: 4046 } |  |  | 0.620 |
| walker |  | 9455 | 18 | Code::CodeKey { rung: Doc, file: krep.c, decl: 76, sub: 0, line: 4104 } |  |  | 0.620 |
| walker |  | 9476 | 21 | Code::CodeKey { rung: Doc, file: krep.c, decl: 81, sub: 0, line: 4371 } |  |  | 0.620 |
| walker |  | 9511 | 35 | Code::CodeKey { rung: Doc, file: krep.c, decl: 73, sub: 0, line: 3442 } |  |  | 0.620 |
| ns | 9685 |  | 386 | select_search_algorithm: SIMD length limits and the KMP/Boyer-Moore fallback | 6.5 | 6.4 | 0.606 |
| walker |  | 9696 | 185 | Markdown::Section { file: README.md, section_index: 9, keeps_default_concavity: false } |  |  | 0.617 |
| ns | 9868 |  | 183 | gitignore data model: pattern record and parent-chained context | 6.6 |  | 0.623 |
| walker |  | 9894 | 198 | Markdown::Section { file: README.md, section_index: 10, keeps_default_concavity: false } |  |  | 0.640 |
| walker |  | 9991 | 97 | Markdown::Section { file: README.md, section_index: 11, keeps_default_concavity: false } |  |  | 0.645 |
| ns | 9993 |  | 125 | Licence header, dependabot config and .gitignore | 7.1 |  | 0.638 |
