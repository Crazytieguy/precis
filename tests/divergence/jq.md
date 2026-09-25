Score(3000)=0.611 I=0.787 C=0.475 ns_rows≤3K=16/40 grid(1000/1442/2080/3000/4327/6240/9000)=0.750/0.680/0.659/0.611/0.498/0.568/0.498

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
| ns | 1385 |  | 225 | jq.h: jq_state lifecycle - init, compile, start, next, teardown | 2.1 |  | 0.680 |
| ns | 1503 |  | 118 | jq.h: debug-trace flags and halt / exit-code / error-message API | 2.2 |  | 0.662 |
| walker |  | 1574 | 220 | c names src/jq.h |  |  | 0.706 |
| walker |  | 1626 | 52 | c decl src/jq.h:11 |  |  | 0.714 |
| ns | 1749 |  | 246 | jq.h: input/debug/stderr callbacks and the attribute store | 2.3 |  | 0.685 |
| walker |  | 1954 | 328 | c names src/parser.h |  |  | 0.685 |
| walker |  | 1964 | 10 | c doc src/parser.h:126 |  |  | 0.685 |
| ns | 1979 |  | 230 | jq.h: jq_util_input_* file/stdin reader and jq_set_colors | 2.4 |  | 0.659 |
| walker |  | 2082 | 118 | c names src/locfile.h |  |  | 0.659 |
| walker |  | 2090 | 8 | c decl src/locfile.h:6 |  |  | 0.659 |
| walker |  | 2167 | 77 | c decl src/locfile.h:12 |  |  | 0.659 |
| ns | 2202 |  | 223 | main.c usage(): synopsis and one-paragraph description | 3.1 |  | 0.643 |
| walker |  | 2287 | 120 | c decl src/builtin.h:10 |  |  | 0.643 |
| walker |  | 2420 | 133 | c names src/jv_alloc.h |  |  | 0.643 |
| walker |  | 2564 | 144 | c decl src/parser.h:54 |  |  | 0.643 |
| ns | 2653 |  | 451 | main.c usage(): input and output-formatting options (-n through --seq) | 3.2 |  | 0.611 |
| walker |  | 2722 | 158 | c names src/jv_unicode.h |  |  | 0.611 |
| walker |  | 2905 | 183 | c names src/bytecode.h |  |  | 0.611 |
| walker |  | 2942 | 37 | c decl src/bytecode.h:59 |  |  | 0.611 |
| walker |  | 3019 | 77 | c decl src/bytecode.h:32 |  |  | 0.611 |
| ns | 3043 |  | 390 | main.c usage(): program, argument and mode options (-f through --) | 3.3 |  | 0.585 |
| walker |  | 3142 | 123 | c decl src/bytecode.h:48 |  |  | 0.585 |
| ns | 3210 |  | 167 | main.c: process exit-status codes | 3.4 |  | 0.573 |
| walker |  | 3279 | 137 | c decl src/bytecode.h:72 |  |  | 0.573 |
| walker |  | 3423 | 144 | c decl src/bytecode.h:20 |  |  | 0.573 |
| ns | 3433 |  | 223 | main.c: the undocumented flags (--debug-dump-disasm, --debug-trace, --run-tests) | 3.5 |  | 0.560 |
| walker |  | 3672 | 249 | c names src/lexer.h |  |  | 0.560 |
| ns | 3673 |  | 240 | Makefile.am: the TESTS list and test environment | 3.6 |  | 0.543 |
| ns | 3911 |  | 238 | tests/setup + tests/jqtest: how one test driver actually runs | 3.7 |  | 0.530 |
| walker |  | 3933 | 261 | c names src/compile.h |  |  | 0.530 |
| walker |  | 3949 | 16 | c decl src/compile.h:12 |  |  | 0.530 |
| ns | 4024 |  | 113 | tests/jq.test: the three-line test format, with the first cases | 3.8 |  | 0.518 |
| walker |  | 4174 | 225 | c names src/jv_dtoa.h |  |  | 0.518 |
| walker |  | 4206 | 32 | c decl src/jv_dtoa.h:6 |  |  | 0.518 |
| ns | 4257 |  | 233 | jv.h: jv_kind enum and the jv struct | 4.1 |  | 0.498 |
| ns | 4411 |  | 154 | jv.h: the consume/produce refcount contract | 4.2 |  | 0.489 |
| walker |  | 4423 | 217 | c names src/jq.h #1 |  |  | 0.516 |
| walker |  | 4759 | 336 | c names src/parser.h #1 |  |  | 0.516 |
| ns | 4813 |  | 402 | jv.h: comparison, invalid-with-message, constructors, numbers, arrays | 4.3 |  | 0.492 |
| walker |  | 4825 | 66 | c decl src/parser.h:179 |  |  | 0.492 |
| walker |  | 4943 | 118 | c names src/parser.h #2 |  |  | 0.492 |
| walker |  | 4987 | 44 | c decl src/parser.h:197 |  |  | 0.492 |
| walker |  | 5010 | 23 | c names src/jv_file.c |  |  | 0.492 |
| walker |  | 5062 | 52 | README.md section #1 |  |  | 0.492 |
| ns | 5142 |  | 329 | jv.h: the string API | 4.4 |  | 0.480 |
| walker |  | 5335 | 273 | c names src/jv.h |  |  | 0.493 |
| ns | 5353 |  | 211 | jv.h: the object API and its iterator protocol | 4.5 |  | 0.484 |
| walker |  | 5405 | 70 | c decl src/jv.h:19 |  |  | 0.494 |
| walker |  | 5480 | 75 | c decl src/jv.h:34 |  |  | 0.514 |
| walker |  | 5507 | 27 | c doc src/jv.h:34 |  |  | 0.525 |
| ns | 5549 |  | 196 | jv.h: path access, keys, ordering, sort/group/unique | 4.6 |  | 0.517 |
| walker |  | 5710 | 203 | c names src/jq.h #2 |  |  | 0.538 |
| ns | 5811 |  | 262 | jv.h: parsing - flags, one-shot parsers, streaming jv_parser, jv_load_file | 4.7 |  | 0.525 |
| walker |  | 5952 | 242 | c module doc src/parser.h |  |  | 0.525 |
| walker |  | 6117 | 165 | listing of 'tests' |  |  | 0.581 |
| ns | 6129 |  | 318 | jv.h: print flags and the dump/show functions | 4.8 |  | 0.568 |
| ns | 6267 |  | 138 | jv.h: the convenience macro walls (existence, not bodies) | 4.9 |  | 0.559 |
| walker |  | 6329 | 212 | c names src/compile.h #1 |  |  | 0.559 |
| walker |  | 6416 | 87 | c doc src/jq.h:60 |  |  | 0.559 |
| walker |  | 6666 | 250 | c names src/lexer.h #1 |  |  | 0.559 |
| walker |  | 6682 | 16 | c doc src/lexer.h:336 |  |  | 0.559 |
| walker |  | 6715 | 33 | c doc src/lexer.h:352 |  |  | 0.559 |
| ns | 6773 |  | 506 | opcode_list.h: every opcode and its immediate kind | 5.1 |  | 0.534 |
| walker |  | 7043 | 328 | c decl src/lexer.h:386 |  |  | 0.534 |
| walker |  | 7242 | 199 | c names src/lexer.h #2 |  |  | 0.534 |
| ns | 7248 |  | 475 | parser.y: the complete token list | 5.2 |  | 0.512 |
| walker |  | 7374 | 132 | c names src/jq.h #3 |  |  | 0.529 |
| ns | 7435 |  | 187 | parser.y: operator precedence and associativity | 5.3 |  | 0.521 |
| walker |  | 7682 | 308 | c module doc src/parser.h #1 |  |  | 0.521 |
| walker |  | 7844 | 162 | c decl src/lexer.h:386 #1 |  |  | 0.521 |
| walker |  | 8075 | 231 | c names src/compile.h #2 |  |  | 0.521 |
| ns | 8106 |  | 671 | builtin.c: function_list, part 1 - libm, binops, conversions, keys, strings, paths, sorting | 5.4 |  | 0.503 |
| walker |  | 8421 | 346 | plaintext config Dockerfile |  |  | 0.503 |
| ns | 8607 |  | 501 | builtin.c: function_list, part 2 - search, min/max, errors, env, regex, I/O, time | 5.5 |  | 0.491 |
| walker |  | 8631 | 210 | c decl src/parser.h:70 |  |  | 0.491 |
| walker |  | 8729 | 98 | listing of 'sig' |  |  | 0.491 |
| walker |  | 8958 | 229 | c names src/compile.h #3 |  |  | 0.498 |
| walker |  | 9041 | 83 | README.md section #7 |  |  | 0.498 |
| ns | 9119 |  | 512 | builtin.jq: every jq-defined builtin, part 1 (lines 1-115) | 5.6 |  | 0.473 |
| walker |  | 9272 | 231 | c names src/jv.h #1 |  |  | 0.501 |
| walker |  | 9303 | 31 | README.md section #3 |  |  | 0.501 |
| ns | 9468 |  | 349 | builtin.jq: every jq-defined builtin, part 2 (lines 116-244) | 5.7 |  | 0.486 |
| walker |  | 9538 | 235 | c names src/lexer.h #3 |  |  | 0.486 |
| walker |  | 9743 | 205 | c decl src/parser.h:70 #1 |  |  | 0.486 |
| ns | 9778 |  | 310 | execute.c: struct jq_state, the whole interpreter state | 5.8 |  | 0.475 |
| walker |  | 9828 | 85 | README.md section #8 |  |  | 0.475 |
| walker |  | 9858 | 30 | README.md section #2 |  |  | 0.475 |
| ns | 9977 |  | 199 | manual.yml: the section titles of the jq language reference | 6.1 |  | 0.471 |
