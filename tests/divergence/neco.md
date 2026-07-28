Score(3000)=0.720 I=0.871 C=0.596 ns_rows≤3K=17/53 (reached=8 partial=1 missing=8)

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 33 | 33 | listing of '.' |  |  | 0.000 |
| ns | 47 |  | 47 | What Neco is | 1.1 |  | 0.000 |
| walker |  | 59 | 26 | listing of 'docs' |  |  | 0.000 |
| walker |  | 70 | 11 | README headline in docs/README.md |  |  | 0.000 |
| walker |  | 90 | 20 | listing of 'docs/assets' |  |  | 0.000 |
| walker |  | 105 | 15 | listing of '.github' |  |  | 0.000 |
| walker |  | 108 | 3 | listing of '.github/workflows' |  |  | 0.000 |
| ns | 121 |  | 74 | How the library is consumed and built | 1.2 |  | 0.000 |
| ns | 154 |  | 33 | Complete root listing | 1.3 |  | 0.322 |
| ns | 245 |  | 91 | examples/ and deps/ listings | 1.4 |  | 0.215 |
| walker |  | 251 | 143 | YAML config at .github/workflows/main.yml |  |  | 0.217 |
| walker |  | 265 | 14 | listing of 'docs/tools' |  |  | 0.228 |
| walker |  | 306 | 41 | listing of 'deps' |  |  | 0.290 |
| ns | 342 |  | 97 | docs/ and .github/ trees, complete to the leaves | 1.5 |  | 0.313 |
| walker |  | 395 | 89 | c header banner in neco.h |  |  | 0.313 |
| walker |  | 445 | 50 | listing of 'examples' |  |  | 0.478 |
| ns | 467 |  | 125 | Complete tests/ listing | 1.6 |  | 0.390 |
| walker |  | 570 | 125 | listing of 'tests' |  |  | 0.539 |
| ns | 599 |  | 132 | Running the tests, and where the examples live | 1.7 |  | 0.501 |
| ns | 868 |  | 269 | Complete roster of neco.h's API groups | 1.8 |  | 0.455 |
| walker |  | 997 | 427 | README headline in README.md |  |  | 0.679 |
| ns | 1084 |  | 216 | Basic operations: every coroutine-lifecycle signature | 1.9 |  | 0.637 |
| walker |  | 1230 | 233 | headings outline in README.md |  |  | 0.640 |
| walker |  | 1274 | 44 | README.md section #0 |  |  | 0.640 |
| walker |  | 1293 | 19 | README.md section #21 |  |  | 0.640 |
| ns | 1346 |  | 262 | Neco error codes, part 1 | 1.10 |  | 0.606 |
| ns | 1610 |  | 264 | Neco error codes, part 2, and the error functions | 1.11 | 1.10 | 0.579 |
| walker |  | 1712 | 419 | c decl names surface in neco.h |  |  | 0.641 |
| walker |  | 1712 | 0 | c decl at neco.h:35 |  |  | 0.641 |
| walker |  | 1712 | 0 | c decl at neco.h:61 |  |  | 0.641 |
| walker |  | 1775 | 63 | README.md section #2 |  |  | 0.734 |
| walker |  | 1846 | 71 | headings outline in docs/assets/API_head.md |  |  | 0.734 |
| walker |  | 1861 | 15 | README.md section #16 |  |  | 0.734 |
| walker |  | 1975 | 114 | c includes in neco.h |  |  | 0.734 |
| ns | 2034 |  | 424 | Channels: doc and complete signature set | 2.1 |  | 0.693 |
| walker |  | 2040 | 65 | README.md section #14 |  |  | 0.693 |
| walker |  | 2170 | 130 | README.md section #1 |  |  | 0.693 |
| ns | 2252 |  | 218 | Generators: complete signature set | 2.2 |  | 0.672 |
| ns | 2381 |  | 129 | Time: the duration constants and neco_now() | 2.3 |  | 0.658 |
| ns | 2560 |  | 179 | Mutexes: type, static initializer, and all eight operations | 2.4 |  | 0.640 |
| walker |  | 2668 | 498 | c decl names surface #1 in neco.h |  |  | 0.720 |
| walker |  | 2668 | 0 | c decl at neco.h:90 |  |  | 0.720 |
| walker |  | 2668 | 0 | c decl at neco.h:113 |  |  | 0.720 |
| ns | 2708 |  | 148 | WaitGroups: type, initializer and five operations | 2.5 |  | 0.704 |
| walker |  | 2719 | 51 | c decl doc at neco.h:90 |  |  | 0.704 |
| walker |  | 2797 | 78 | c decl doc at neco.h:61 |  |  | 0.733 |
| ns | 2850 |  | 142 | Condition variables: type, initializer and five operations | 2.6 |  | 0.718 |
| walker |  | 2883 | 86 | c decl doc at neco.h:113 |  |  | 0.719 |
| walker |  | 2957 | 74 | c decl doc at neco.h:35 |  |  | 0.720 |
| walker |  | 3068 | 111 | docs/README.md section #1 |  |  | 0.721 |
| walker |  | 3101 | 33 | README.md section #19 |  |  | 0.722 |
| ns | 3158 |  | 308 | Posix wrappers: the non-blocking fd operations | 2.7 |  | 0.698 |
| ns | 3308 |  | 150 | File descriptor helpers: neco_setnonblock and neco_wait | 2.8 |  | 0.679 |
| ns | 3398 |  | 90 | Networking utilities: neco_serve and neco_dial | 2.9 |  | 0.673 |
| walker |  | 3418 | 317 | c decl names surface in neco.c |  |  | 0.673 |
| walker |  | 3418 | 0 | c decl at neco.c:1312 |  |  | 0.673 |
| walker |  | 3418 | 0 | c decl at neco.c:1313 |  |  | 0.673 |
| walker |  | 3429 | 11 | c decl at neco.c:2029 |  |  | 0.673 |
| walker |  | 3440 | 11 | c decl at neco.c:2065 |  |  | 0.673 |
| walker |  | 3452 | 12 | c decl at neco.c:2056 |  |  | 0.673 |
| walker |  | 3465 | 13 | c decl at neco.c:2051 |  |  | 0.673 |
| walker |  | 3479 | 14 | c decl at neco.c:2074 |  |  | 0.673 |
| walker |  | 3493 | 14 | c decl at neco.c:2106 |  |  | 0.673 |
| walker |  | 3507 | 14 | c decl at neco.c:2120 |  |  | 0.673 |
| walker |  | 3524 | 17 | c decl at neco.c:1899 |  |  | 0.673 |
| walker |  | 3539 | 15 | c decl at neco.c:2037 |  |  | 0.673 |
| walker |  | 3554 | 15 | c decl at neco.c:2094 |  |  | 0.673 |
| walker |  | 3570 | 16 | c decl at neco.c:1320 |  |  | 0.673 |
| walker |  | 3589 | 19 | c decl at neco.c:1293 |  |  | 0.673 |
| ns | 3606 |  | 208 | Cancelation: neco_cancel, the type/state constants, cleanup macros | 2.10 |  | 0.652 |
| walker |  | 3609 | 20 | c decl at neco.c:1282 |  |  | 0.652 |
| walker |  | 3635 | 26 | c decl at neco.c:1814 |  |  | 0.652 |
| walker |  | 3664 | 29 | c decl at neco.c:1875 |  |  | 0.652 |
| walker |  | 3694 | 30 | c decl at neco.c:1880 |  |  | 0.652 |
| walker |  | 3754 | 60 | c decl at neco.c:1155 |  |  | 0.652 |
| walker |  | 3847 | 93 | c decl at neco.c:1337 |  |  | 0.652 |
| ns | 3865 |  | 259 | Streams and buffered I/O: the name of every entry point | 2.11 |  | 0.626 |
| walker |  | 3948 | 101 | c decl at neco.c:1364 |  |  | 0.626 |
| ns | 3990 |  | 125 | Random number generator: CSPRNG/PRNG attributes and three functions | 2.12 |  | 0.618 |
| ns | 4066 |  | 76 | Signals: watch, wait, unwatch | 2.13 |  | 0.612 |
| walker |  | 4073 | 125 | docs/API.md section #0 |  |  | 0.612 |
| walker |  | 4086 | 13 | docs/API.md section #3 |  |  | 0.612 |
| walker |  | 4098 | 12 | docs/API.md section #4 |  |  | 0.612 |
| ns | 4109 |  | 43 | Background worker: neco_work | 2.14 |  | 0.610 |
| walker |  | 4111 | 13 | docs/API.md section #5 |  |  | 0.610 |
| walker |  | 4124 | 13 | docs/API.md section #6 |  |  | 0.610 |
| walker |  | 4137 | 13 | docs/API.md section #15 |  |  | 0.610 |
| walker |  | 4148 | 11 | docs/API.md section #16 |  |  | 0.610 |
| walker |  | 4161 | 13 | docs/API.md section #14 |  |  | 0.610 |
| walker |  | 4174 | 13 | docs/API.md section #17 |  |  | 0.610 |
| ns | 4340 |  | 231 | neco_stats: every runtime counter, plus introspection | 2.15 |  | 0.591 |
| ns | 4423 |  | 83 | Global environment: allocator and process-wide defaults | 2.16 |  | 0.587 |
| ns | 4671 |  | 248 | The neco_main macro, expanded | 2.17 |  | 0.571 |
| walker |  | 4692 | 518 | c decl names surface #2 in neco.h |  |  | 0.629 |
| walker |  | 4692 | 0 | c decl at neco.h:135 |  |  | 0.629 |
| walker |  | 4692 | 0 | c decl at neco.h:149 |  |  | 0.629 |
| walker |  | 4692 | 0 | c decl at neco.h:169 |  |  | 0.629 |
| walker |  | 4704 | 12 | c decl doc at neco.h:169 |  |  | 0.631 |
| walker |  | 4755 | 51 | c decl doc at neco.h:149 |  |  | 0.632 |
| ns | 4779 |  | 108 | neco.h tail: private entry points and the EAI_SYSTEM shim | 2.18 |  | 0.622 |
| walker |  | 4904 | 149 | c decl at neco.c:1819 |  |  | 0.622 |
| walker |  | 4918 | 14 | docs/API.md section #2 |  |  | 0.622 |
| walker |  | 4932 | 14 | docs/API.md section #7 |  |  | 0.622 |
| walker |  | 4946 | 14 | docs/API.md section #13 |  |  | 0.622 |
| walker |  | 4961 | 15 | docs/API.md section #12 |  |  | 0.622 |
| ns | 4972 |  | 193 | Deadlines and cancelation, the governing rules | 3.1 |  | 0.611 |
| walker |  | 4976 | 15 | docs/API.md section #18 |  |  | 0.611 |
| walker |  | 4990 | 14 | docs/API.md section #19 |  |  | 0.611 |
| walker |  | 5117 | 127 | c decl doc at neco.h:135 |  |  | 0.612 |
| ns | 5263 |  | 291 | Error semantics: which errors panic, which leak errno, and lasterr | 3.2 |  | 0.597 |
| walker |  | 5293 | 176 | README.md section #4 |  |  | 0.597 |
| walker |  | 5309 | 16 | docs/API.md section #1 |  |  | 0.597 |
| walker |  | 5325 | 16 | docs/API.md section #10 |  |  | 0.597 |
| ns | 5497 |  | 234 | Async cancelation, and turning cancelation off | 3.3 |  | 0.586 |
| walker |  | 5526 | 201 | README.md section #3 |  |  | 0.586 |
| walker |  | 5543 | 17 | docs/API.md section #8 |  |  | 0.586 |
| walker |  | 5558 | 15 | docs/API.md section #9 |  |  | 0.586 |
| walker |  | 5612 | 54 | docs/API.md section #43 |  |  | 0.586 |
| walker |  | 5664 | 52 | docs/API.md section #44 |  |  | 0.586 |
| walker |  | 5712 | 48 | README.md section #17 |  |  | 0.587 |
| ns | 5729 |  | 232 | Platform notes: what does not work on Windows and WebAssembly | 3.4 |  | 0.574 |
| walker |  | 5730 | 18 | docs/API.md section #11 |  |  | 0.574 |
| walker |  | 5923 | 193 | c decl at neco.c:1124 |  |  | 0.574 |
| walker |  | 6103 | 180 | docs/TECHNICAL.md section #0 |  |  | 0.574 |
| ns | 6119 |  | 390 | The scheduler, context switching, and the thread-local runtime | 3.5 |  | 0.564 |
| ns | 6234 |  | 115 | How docs/API.md is produced | 3.6 |  | 0.570 |
| walker |  | 6600 | 497 | c decl names surface #3 in neco.h |  |  | 0.616 |
| walker |  | 6600 | 0 | c decl at neco.h:188 |  |  | 0.616 |
| walker |  | 6600 | 0 | c decl at neco.h:191 |  |  | 0.616 |
| walker |  | 6616 | 16 | c decl at neco.h:231 |  |  | 0.619 |
| ns | 6620 |  | 386 | neco.c compile-time options: the complete knob list | 4.1 |  | 0.602 |
| walker |  | 6633 | 17 | c decl at neco.h:177 |  |  | 0.605 |
| walker |  | 6657 | 24 | c decl at neco.h:179 |  |  | 0.608 |
| walker |  | 6673 | 16 | c decl doc at neco.h:188 |  |  | 0.612 |
| walker |  | 6688 | 15 | c decl doc at neco.h:191 |  |  | 0.615 |
| walker |  | 6740 | 52 | README.md section #18 |  |  | 0.622 |
| ns | 6879 |  | 259 | Amalgamation structure and the embedded-source boundaries | 4.2 |  | 0.604 |
| ns | 7011 |  | 132 | Section map of Neco's own implementation | 4.3 |  | 0.598 |
| walker |  | 7053 | 313 | c decl names surface #1 in neco.c |  |  | 0.598 |
| walker |  | 7053 | 0 | c decl at neco.c:3105 |  |  | 0.598 |
| walker |  | 7064 | 11 | c decl at neco.c:2154 |  |  | 0.598 |
| walker |  | 7077 | 13 | c decl at neco.c:2135 |  |  | 0.598 |
| walker |  | 7091 | 14 | c decl at neco.c:2125 |  |  | 0.598 |
| walker |  | 7105 | 14 | c decl at neco.c:2130 |  |  | 0.598 |
| walker |  | 7119 | 14 | c decl at neco.c:2145 |  |  | 0.598 |
| walker |  | 7134 | 15 | c decl at neco.c:2160 |  |  | 0.598 |
| walker |  | 7149 | 15 | c decl at neco.c:2607 |  |  | 0.598 |
| walker |  | 7164 | 15 | c decl at neco.c:2616 |  |  | 0.598 |
| walker |  | 7179 | 15 | c decl at neco.c:2926 |  |  | 0.598 |
| walker |  | 7195 | 16 | c decl at neco.c:2459 |  |  | 0.598 |
| walker |  | 7215 | 20 | c decl at neco.c:2551 |  |  | 0.598 |
| walker |  | 7235 | 20 | c decl at neco.c:2598 |  |  | 0.598 |
| walker |  | 7257 | 22 | c decl at neco.c:2441 |  |  | 0.598 |
| walker |  | 7287 | 30 | c decl at neco.c:2192 |  |  | 0.598 |
| ns | 7296 |  | 285 | struct coroutine: identity, stack, arguments, scheduling flags | 4.4 |  | 0.585 |
| walker |  | 7317 | 30 | c decl at neco.c:3121 |  |  | 0.585 |
| walker |  | 7355 | 38 | c decl at neco.c:2285 |  |  | 0.585 |
| walker |  | 7397 | 42 | c decl at neco.c:2261 |  |  | 0.585 |
| walker |  | 7462 | 65 | c decl at neco.c:2165 |  |  | 0.585 |
| ns | 7589 |  | 293 | What neco_chan and neco_gen actually are | 4.5 |  | 0.574 |
| walker |  | 7604 | 142 | c decl at neco.c:2247 |  |  | 0.574 |
| ns | 7773 |  | 184 | Where the event queue backend is chosen | 4.6 |  | 0.566 |
| walker |  | 7791 | 187 | c decl at neco.c:2267 |  |  | 0.566 |
| walker |  | 7900 | 109 | docs/assets/API_head.md section #0 |  |  | 0.566 |
| walker |  | 7913 | 13 | docs/assets/API_head.md section #3 |  |  | 0.566 |
| walker |  | 7925 | 12 | docs/assets/API_head.md section #4 |  |  | 0.566 |
| walker |  | 7938 | 13 | docs/assets/API_head.md section #5 |  |  | 0.566 |
| walker |  | 7951 | 13 | docs/assets/API_head.md section #6 |  |  | 0.566 |
| walker |  | 7964 | 13 | docs/assets/API_head.md section #15 |  |  | 0.566 |
| walker |  | 7975 | 11 | docs/assets/API_head.md section #16 |  |  | 0.566 |
| ns | 7981 |  | 208 | Test suite knobs: compilers, sanitizers, valgrind | 5.1 |  | 0.559 |
| walker |  | 7988 | 13 | docs/assets/API_head.md section #14 |  |  | 0.559 |
| walker |  | 8001 | 13 | docs/assets/API_head.md section #17 |  |  | 0.559 |
| walker |  | 8073 | 72 | docs/API.md section #45 |  |  | 0.559 |
| walker |  | 8087 | 14 | docs/assets/API_head.md section #2 |  |  | 0.559 |
| walker |  | 8101 | 14 | docs/assets/API_head.md section #1 |  |  | 0.559 |
| walker |  | 8115 | 14 | docs/assets/API_head.md section #7 |  |  | 0.559 |
| walker |  | 8129 | 14 | docs/assets/API_head.md section #13 |  |  | 0.559 |
| walker |  | 8143 | 14 | docs/assets/API_head.md section #19 |  |  | 0.559 |
| walker |  | 8156 | 13 | docs/assets/API_head.md section #18 |  |  | 0.559 |
| ns | 8262 |  | 281 | The private, undocumented functions the tests may call | 5.2 |  | 0.549 |
| ns | 8336 |  | 74 | Every function-like macro in tests/tests.h | 5.3 |  | 0.545 |
| ns | 8599 |  | 263 | How run.sh compiles and runs each test | 5.4 |  | 0.535 |
| walker |  | 8609 | 453 | c decl names surface #4 in neco.h |  |  | 0.564 |
| walker |  | 8781 | 172 | c decl at neco.h:287 |  |  | 0.584 |
| ns | 8796 |  | 197 | The four NECO_TESTING-only shim headers | 5.5 |  | 0.575 |
| walker |  | 8813 | 32 | docs/assets/API_foot.md section #0 |  |  | 0.575 |
| walker |  | 8828 | 15 | docs/assets/API_head.md section #12 |  |  | 0.575 |
| walker |  | 8909 | 81 | docs/API.md section #121 |  |  | 0.575 |
| walker |  | 8992 | 83 | docs/API.md section #135 |  |  | 0.575 |
| walker |  | 9008 | 16 | docs/assets/API_head.md section #10 |  |  | 0.575 |
| walker |  | 9023 | 15 | listing of 'docs/tools/doxygen-md' |  |  | 0.590 |
| ns | 9095 |  | 299 | deps/sco.h: the scheduler contract | 6.1 |  | 0.577 |
| walker |  | 9109 | 86 | docs/API.md section #86 |  |  | 0.577 |
| walker |  | 9193 | 84 | docs/API.md section #87 |  |  | 0.577 |
| walker |  | 9280 | 87 | docs/API.md section #90 |  |  | 0.577 |
| walker |  | 9367 | 87 | docs/API.md section #122 |  |  | 0.577 |
| ns | 9399 |  | 304 | deps/stack.h: the coroutine stack allocator | 6.2 |  | 0.567 |
| walker |  | 9454 | 87 | docs/API.md section #124 |  |  | 0.567 |
| walker |  | 9541 | 87 | docs/API.md section #123 |  |  | 0.567 |
| ns | 9567 |  | 168 | deps/worker.h: the background thread pool | 6.3 |  | 0.561 |
| walker |  | 9629 | 88 | docs/API.md section #42 |  |  | 0.561 |
| ns | 9650 |  | 83 | deps/embed.sh: how neco.c is regenerated | 6.4 |  | 0.559 |
| walker |  | 9717 | 88 | docs/API.md section #89 |  |  | 0.559 |
| walker |  | 9805 | 88 | docs/API.md section #88 |  |  | 0.559 |
| ns | 9831 |  | 181 | examples/select.c: the multi-channel select pattern | 6.5 |  | 0.552 |
| walker |  | 9893 | 88 | docs/API.md section #99 |  |  | 0.552 |
| ns | 9943 |  | 112 | CI | 6.6 |  | 0.559 |
| walker |  | 9980 | 87 | docs/API.md section #100 |  |  | 0.559 |
| ns | 9988 |  | 45 | License | 6.7 |  | 0.559 |
