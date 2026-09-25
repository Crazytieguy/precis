Score(3000)=0.699 I=0.825 C=0.591 ns_rows≤3K=16/40 grid(1000/1442/2080/3000/4327/6240/9000)=0.750/0.680/0.659/0.699/0.569/0.501/0.478

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| ns | 71 |  | 71 | README lede: what jq is | 1.1 |  | 0.000 |
| walker |  | 81 | 81 | listing of '.' |  |  | 0.000 |
| walker |  | 90 | 9 | listing of 'config' |  |  | 0.000 |
| walker |  | 142 | 52 | listing of 'docs' |  |  | 0.000 |
| ns | 152 |  | 81 | Repository root listing (complete) | 1.2 |  | 0.703 |
| walker |  | 155 | 13 | listing of 'docs/content' |  |  | 0.708 |
| walker |  | 159 | 4 | listing of 'docs/content/download' |  |  | 0.708 |
| walker |  | 163 | 4 | listing of 'docs/content/tutorial' |  |  | 0.708 |
| walker |  | 184 | 21 | listing of 'docs/templates' |  |  | 0.708 |
| ns | 215 |  | 63 | Front-end landmarks: jq_parse / jq_parse_library / block_compile | 1.3 |  | 0.643 |
| walker |  | 255 | 71 | README headline in README.md |  |  | 0.916 |
| walker |  | 286 | 31 | listing of 'docs/public' |  |  | 0.916 |
| walker |  | 290 | 4 | listing of 'docs/public/css' |  |  | 0.916 |
| walker |  | 295 | 5 | listing of 'docs/public/js' |  |  | 0.916 |
| walker |  | 307 | 12 | listing of 'vendor' |  |  | 0.916 |
| walker |  | 319 | 12 | listing of '.github' |  |  | 0.916 |
| ns | 334 |  | 119 | Back-end landmarks: load_program, builtins_bind, and who owns the bytecode format | 1.4 |  | 0.785 |
| walker |  | 354 | 35 | listing of '.github/workflows' |  |  | 0.789 |
| walker |  | 375 | 21 | listing of 'docs/templates/shared' |  |  | 0.789 |
| walker |  | 389 | 14 | listing of 'scripts' |  |  | 0.789 |
| walker |  | 436 | 47 | listing of 'config/m4' |  |  | 0.789 |
| walker |  | 492 | 56 | README headline in docs/README.md |  |  | 0.789 |
| ns | 517 |  | 183 | README: build-from-source dependencies and the exact command sequence | 1.5 |  | 0.631 |
| walker |  | 564 | 72 | headings outline in README.md |  |  | 0.633 |
| walker |  | 587 | 23 | listing of 'm4' |  |  | 0.633 |
| ns | 607 |  | 90 | README: static build and released-tarball builds | 1.6 |  | 0.595 |
| walker |  | 630 | 43 | listing of 'docs/content/manual' |  |  | 0.601 |
| walker |  | 634 | 4 | listing of 'docs/content/manual/dev' |  |  | 0.601 |
| walker |  | 638 | 4 | listing of 'docs/content/manual/v1.3' |  |  | 0.601 |
| walker |  | 642 | 4 | listing of 'docs/content/manual/v1.4' |  |  | 0.601 |
| walker |  | 646 | 4 | listing of 'docs/content/manual/v1.5' |  |  | 0.601 |
| walker |  | 650 | 4 | listing of 'docs/content/manual/v1.6' |  |  | 0.601 |
| walker |  | 654 | 4 | listing of 'docs/content/manual/v1.7' |  |  | 0.601 |
| walker |  | 658 | 4 | listing of 'docs/content/manual/v1.8' |  |  | 0.601 |
| ns | 840 |  | 233 | src/ listing (complete) - the flat core | 1.7 |  | 0.449 |
| walker |  | 891 | 233 | listing of 'src' |  |  | 0.750 |
| walker |  | 911 | 20 | c names src/jv_utf8_tables.h |  |  | 0.750 |
| walker |  | 932 | 21 | c names src/jv_dtoa_tsd.h |  |  | 0.750 |
| walker |  | 964 | 32 | c names src/jv_private.h |  |  | 0.750 |
| walker |  | 1003 | 39 | c names src/jq_parser.h |  |  | 0.759 |
| ns | 1005 |  | 165 | tests/ listing (complete) | 1.8 |  | 0.660 |
| walker |  | 1051 | 48 | c names src/linker.h |  |  | 0.665 |
| walker |  | 1104 | 53 | c names src/exec_stack.h |  |  | 0.665 |
| ns | 1113 |  | 108 | docs/ and docs/content/manual listings (complete) | 1.9 |  | 0.696 |
| walker |  | 1145 | 41 | c decl src/exec_stack.h:40 |  |  | 0.696 |
| ns | 1160 |  | 47 | CI workflows and vendored dependencies (complete listings) | 1.10 |  | 0.705 |
| walker |  | 1196 | 51 | c decl src/exec_stack.h:52 |  |  | 0.705 |
| walker |  | 1253 | 57 | c names src/builtin.h |  |  | 0.713 |
| walker |  | 1339 | 86 | c names src/util.h |  |  | 0.713 |
| walker |  | 1354 | 15 | c decl src/util.h:44 |  |  | 0.713 |
| walker |  | 1377 | 23 | c names src/jv_file.c |  |  | 0.713 |
| ns | 1385 |  | 225 | jq.h: jq_state lifecycle - init, compile, start, next, teardown | 2.1 |  | 0.680 |
| ns | 1503 |  | 118 | jq.h: debug-trace flags and halt / exit-code / error-message API | 2.2 |  | 0.662 |
| walker |  | 1597 | 220 | c names src/jq.h |  |  | 0.706 |
| walker |  | 1649 | 52 | c decl src/jq.h:11 |  |  | 0.714 |
| ns | 1749 |  | 246 | jq.h: input/debug/stderr callbacks and the attribute store | 2.3 |  | 0.685 |
| walker |  | 1977 | 328 | c names src/parser.h |  |  | 0.685 |
| ns | 1979 |  | 230 | jq.h: jq_util_input_* file/stdin reader and jq_set_colors | 2.4 |  | 0.659 |
| walker |  | 1987 | 10 | c doc src/parser.h:126 |  |  | 0.659 |
| walker |  | 2039 | 52 | README.md section #1 |  |  | 0.659 |
| ns | 2202 |  | 223 | main.c usage(): synopsis and one-paragraph description | 3.1 |  | 0.643 |
| walker |  | 2204 | 165 | listing of 'tests' |  |  | 0.735 |
| walker |  | 2322 | 118 | c names src/locfile.h |  |  | 0.735 |
| walker |  | 2330 | 8 | c decl src/locfile.h:6 |  |  | 0.735 |
| walker |  | 2407 | 77 | c decl src/locfile.h:12 |  |  | 0.735 |
| walker |  | 2527 | 120 | c decl src/builtin.h:10 |  |  | 0.735 |
| ns | 2653 |  | 451 | main.c usage(): input and output-formatting options (-n through --seq) | 3.2 |  | 0.699 |
| walker |  | 2660 | 133 | c names src/jv_alloc.h |  |  | 0.699 |
| walker |  | 2804 | 144 | c decl src/parser.h:54 |  |  | 0.699 |
| walker |  | 2962 | 158 | c names src/jv_unicode.h |  |  | 0.699 |
| ns | 3043 |  | 390 | main.c usage(): program, argument and mode options (-f through --) | 3.3 |  | 0.668 |
| walker |  | 3145 | 183 | c names src/bytecode.h |  |  | 0.668 |
| walker |  | 3182 | 37 | c decl src/bytecode.h:59 |  |  | 0.668 |
| ns | 3210 |  | 167 | main.c: process exit-status codes | 3.4 |  | 0.655 |
| walker |  | 3259 | 77 | c decl src/bytecode.h:32 |  |  | 0.655 |
| walker |  | 3382 | 123 | c decl src/bytecode.h:48 |  |  | 0.655 |
| ns | 3433 |  | 223 | main.c: the undocumented flags (--debug-dump-disasm, --debug-trace, --run-tests) | 3.5 |  | 0.640 |
| walker |  | 3519 | 137 | c decl src/bytecode.h:72 |  |  | 0.640 |
| walker |  | 3663 | 144 | c decl src/bytecode.h:20 |  |  | 0.640 |
| ns | 3673 |  | 240 | Makefile.am: the TESTS list and test environment | 3.6 |  | 0.621 |
| ns | 3911 |  | 238 | tests/setup + tests/jqtest: how one test driver actually runs | 3.7 |  | 0.606 |
| walker |  | 3912 | 249 | c names src/lexer.h |  |  | 0.606 |
| ns | 4024 |  | 113 | tests/jq.test: the three-line test format, with the first cases | 3.8 |  | 0.592 |
| walker |  | 4173 | 261 | c names src/compile.h |  |  | 0.592 |
| walker |  | 4189 | 16 | c decl src/compile.h:12 |  |  | 0.592 |
| ns | 4257 |  | 233 | jv.h: jv_kind enum and the jv struct | 4.1 |  | 0.569 |
| ns | 4411 |  | 154 | jv.h: the consume/produce refcount contract | 4.2 |  | 0.558 |
| walker |  | 4535 | 346 | plaintext config Dockerfile |  |  | 0.558 |
| walker |  | 4633 | 98 | listing of 'sig' |  |  | 0.558 |
| ns | 4813 |  | 402 | jv.h: comparison, invalid-with-message, constructors, numbers, arrays | 4.3 |  | 0.533 |
| walker |  | 4858 | 225 | c names src/jv_dtoa.h |  |  | 0.533 |
| walker |  | 4890 | 32 | c decl src/jv_dtoa.h:6 |  |  | 0.533 |
| walker |  | 4973 | 83 | README.md section #7 |  |  | 0.533 |
| ns | 5142 |  | 329 | jv.h: the string API | 4.4 |  | 0.520 |
| walker |  | 5190 | 217 | c names src/jq.h #1 |  |  | 0.543 |
| walker |  | 5221 | 31 | README.md section #3 |  |  | 0.543 |
| ns | 5353 |  | 211 | jv.h: the object API and its iterator protocol | 4.5 |  | 0.534 |
| ns | 5549 |  | 196 | jv.h: path access, keys, ordering, sort/group/unique | 4.6 |  | 0.526 |
| walker |  | 5557 | 336 | c names src/parser.h #1 |  |  | 0.526 |
| walker |  | 5623 | 66 | c decl src/parser.h:179 |  |  | 0.526 |
| walker |  | 5708 | 85 | README.md section #8 |  |  | 0.526 |
| ns | 5811 |  | 262 | jv.h: parsing - flags, one-shot parsers, streaming jv_parser, jv_load_file | 4.7 |  | 0.513 |
| walker |  | 5826 | 118 | c names src/parser.h #2 |  |  | 0.513 |
| walker |  | 5870 | 44 | c decl src/parser.h:197 |  |  | 0.513 |
| walker |  | 5900 | 30 | README.md section #2 |  |  | 0.513 |
| walker |  | 6067 | 167 | listing of 'vendor/decNumber' |  |  | 0.513 |
| ns | 6129 |  | 318 | jv.h: print flags and the dump/show functions | 4.8 |  | 0.501 |
| ns | 6267 |  | 138 | jv.h: the convenience macro walls (existence, not bodies) | 4.9 |  | 0.494 |
| walker |  | 6302 | 235 | c names src/jv_parse.c |  |  | 0.494 |
| walker |  | 6377 | 75 | c decl src/jv_parse.c:25 |  |  | 0.494 |
| walker |  | 6650 | 273 | c names src/jv.h |  |  | 0.505 |
| walker |  | 6720 | 70 | c decl src/jv.h:19 |  |  | 0.513 |
| ns | 6773 |  | 506 | opcode_list.h: every opcode and its immediate kind | 5.1 |  | 0.490 |
| walker |  | 6795 | 75 | c decl src/jv.h:34 |  |  | 0.507 |
| walker |  | 6822 | 27 | c doc src/jv.h:34 |  |  | 0.516 |
| walker |  | 6866 | 44 | README.md section #5 |  |  | 0.519 |
| walker |  | 6983 | 117 | c names src/jv_dtoa_tsd.c |  |  | 0.519 |
| ns | 7248 |  | 475 | parser.y: the complete token list | 5.2 |  | 0.497 |
| walker |  | 7306 | 323 | docs/README.md section #0 |  |  | 0.497 |
| ns | 7435 |  | 187 | parser.y: operator precedence and associativity | 5.3 |  | 0.490 |
| walker |  | 7509 | 203 | c names src/jq.h #2 |  |  | 0.507 |
| walker |  | 7721 | 212 | c names src/compile.h #1 |  |  | 0.507 |
| walker |  | 7808 | 87 | c doc src/jq.h:60 |  |  | 0.507 |
| walker |  | 7813 | 5 | listing of '.github/ISSUE_TEMPLATE' |  |  | 0.507 |
| walker |  | 7818 | 5 | listing of 'tests/torture' |  |  | 0.507 |
| ns | 8106 |  | 671 | builtin.c: function_list, part 1 - libm, binops, conversions, keys, strings, paths, sorting | 5.4 |  | 0.490 |
| walker |  | 8485 | 667 | plaintext config compile-ios.sh |  |  | 0.490 |
| ns | 8607 |  | 501 | builtin.c: function_list, part 2 - search, min/max, errors, env, regex, I/O, time | 5.5 |  | 0.478 |
| walker |  | 8735 | 250 | c names src/lexer.h #1 |  |  | 0.478 |
| walker |  | 8751 | 16 | c doc src/lexer.h:336 |  |  | 0.478 |
| walker |  | 8784 | 33 | c doc src/lexer.h:352 |  |  | 0.478 |
| walker |  | 9112 | 328 | c decl src/lexer.h:386 |  |  | 0.478 |
| ns | 9119 |  | 512 | builtin.jq: every jq-defined builtin, part 1 (lines 1-115) | 5.6 |  | 0.454 |
| walker |  | 9255 | 143 | c names src/locfile.c |  |  | 0.454 |
| walker |  | 9273 | 18 | c body src/locfile.c:37 |  |  | 0.454 |
| walker |  | 9288 | 15 | declaration surface of docs/public/robots.txt |  |  | 0.454 |
| walker |  | 9451 | 163 | c names src/jv_unicode.c |  |  | 0.454 |
| ns | 9468 |  | 349 | builtin.jq: every jq-defined builtin, part 2 (lines 116-244) | 5.7 |  | 0.441 |
| walker |  | 9470 | 19 | c doc src/jv_unicode.c:86 |  |  | 0.441 |
| walker |  | 9714 | 244 | c names src/execute.c |  |  | 0.441 |
| walker |  | 9738 | 24 | c decl src/execute.c:59 |  |  | 0.441 |
| walker |  | 9778 | 40 | c decl src/execute.c:53 |  |  | 0.431 |
| ns | 9778 |  | 310 | execute.c: struct jq_state, the whole interpreter state | 5.8 |  | 0.431 |
| walker |  | 9794 | 16 | c decl src/execute.c:130 |  |  | 0.431 |
| walker |  | 9811 | 17 | c doc src/execute.c:59 |  |  | 0.431 |
| walker |  | 9913 | 102 | c decl src/execute.c:65 |  |  | 0.431 |
| walker |  | 9921 | 8 | c doc src/execute.c:65 |  |  | 0.431 |
| ns | 9977 |  | 199 | manual.yml: the section titles of the jq language reference | 6.1 |  | 0.427 |
