Score(3000)=0.788 I=0.891 C=0.696 ns_rows≤3K=17/53 grid(1000/1442/2080/3000/4327/6240/9000)=0.680/0.765/0.766/0.788/0.793/0.772/0.636

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 28 | 28 | Fs::DirListing { dir: . } |  |  | 0.000 |
| ns | 47 |  | 47 | What Neco is | 1.1 |  | 0.000 |
| walker |  | 53 | 25 | Fs::DirListing { dir: docs } |  |  | 0.000 |
| walker |  | 74 | 21 | Fs::DirListing { dir: docs/assets } |  |  | 0.000 |
| walker |  | 89 | 15 | Fs::DirListing { dir: .github } |  |  | 0.000 |
| walker |  | 93 | 4 | Fs::DirListing { dir: .github/workflows } |  |  | 0.000 |
| walker |  | 107 | 14 | Fs::DirListing { dir: docs/tools } |  |  | 0.000 |
| ns | 121 |  | 74 | How the library is consumed and built | 1.2 |  | 0.000 |
| walker |  | 149 | 42 | Fs::DirListing { dir: deps } |  |  | 0.360 |
| ns | 149 |  | 28 | Complete root listing | 1.3 |  | 0.360 |
| walker |  | 200 | 51 | Fs::DirListing { dir: examples } |  |  | 0.442 |
| ns | 242 |  | 93 | examples/ and deps/ listings | 1.4 |  | 0.504 |
| walker |  | 326 | 126 | Fs::DirListing { dir: tests } |  |  | 0.557 |
| ns | 337 |  | 95 | docs/ and .github/ trees, complete to the leaves | 1.5 |  | 0.525 |
| ns | 463 |  | 126 | Complete tests/ listing | 1.6 |  | 0.537 |
| ns | 595 |  | 132 | Running the tests, and where the examples live | 1.7 |  | 0.499 |
| walker |  | 753 | 427 | Markdown::ReadmeHeadline { file: README.md } |  |  | 0.746 |
| ns | 864 |  | 269 | Complete roster of neco.h's API groups | 1.8 |  | 0.678 |
| walker |  | 986 | 233 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.680 |
| walker |  | 1030 | 44 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: true } |  |  | 0.680 |
| ns | 1080 |  | 216 | Basic operations: every coroutine-lifecycle signature | 1.9 |  | 0.637 |
| walker |  | 1093 | 63 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.738 |
| walker |  | 1311 | 218 | Code::CodeKey { rung: Names, file: neco.h, decl: 0, sub: 0, line: 0 } |  |  | 0.807 |
| ns | 1342 |  | 262 | Neco error codes, part 1 | 1.10 |  | 0.764 |
| walker |  | 1385 | 74 | Code::CodeKey { rung: Doc, file: neco.h, decl: 1, sub: 0, line: 35 } |  |  | 0.765 |
| walker |  | 1588 | 203 | Code::CodeKey { rung: Names, file: neco.h, decl: 0, sub: 1, line: 0 } |  |  | 0.767 |
| ns | 1606 |  | 264 | Neco error codes, part 2, and the error functions | 1.11 | 1.10 | 0.733 |
| walker |  | 1666 | 78 | Code::CodeKey { rung: Doc, file: neco.h, decl: 15, sub: 0, line: 61 } |  |  | 0.736 |
| walker |  | 1880 | 214 | Code::CodeKey { rung: Names, file: neco.h, decl: 0, sub: 2, line: 0 } |  |  | 0.741 |
| walker |  | 1931 | 51 | Code::CodeKey { rung: Doc, file: neco.h, decl: 33, sub: 0, line: 90 } |  |  | 0.743 |
| ns | 2030 |  | 424 | Channels: doc and complete signature set | 2.1 |  | 0.764 |
| walker |  | 2123 | 192 | Code::CodeKey { rung: Names, file: neco.h, decl: 0, sub: 3, line: 0 } |  |  | 0.768 |
| walker |  | 2209 | 86 | Code::CodeKey { rung: Doc, file: neco.h, decl: 43, sub: 0, line: 113 } |  |  | 0.769 |
| ns | 2248 |  | 218 | Generators: complete signature set | 2.2 |  | 0.776 |
| ns | 2377 |  | 129 | Time: the duration constants and neco_now() | 2.3 |  | 0.760 |
| walker |  | 2433 | 224 | Code::CodeKey { rung: Names, file: neco.h, decl: 0, sub: 4, line: 0 } |  |  | 0.764 |
| ns | 2556 |  | 179 | Mutexes: type, static initializer, and all eight operations | 2.4 |  | 0.770 |
| walker |  | 2656 | 223 | Code::CodeKey { rung: Names, file: neco.h, decl: 0, sub: 5, line: 0 } |  |  | 0.775 |
| ns | 2704 |  | 148 | WaitGroups: type, initializer and five operations | 2.5 |  | 0.779 |
| walker |  | 2707 | 51 | Code::CodeKey { rung: Doc, file: neco.h, decl: 60, sub: 0, line: 149 } |  |  | 0.781 |
| ns | 2846 |  | 142 | Condition variables: type, initializer and five operations | 2.6 |  | 0.785 |
| walker |  | 2889 | 182 | Code::CodeKey { rung: Names, file: neco.h, decl: 0, sub: 6, line: 0 } |  |  | 0.786 |
| walker |  | 2901 | 12 | Code::CodeKey { rung: Doc, file: neco.h, decl: 67, sub: 0, line: 169 } |  |  | 0.786 |
| walker |  | 3113 | 212 | Code::CodeKey { rung: Names, file: neco.h, decl: 0, sub: 7, line: 0 } |  |  | 0.789 |
| walker |  | 3130 | 17 | Code::CodeKey { rung: Decl, file: neco.h, decl: 75, sub: 0, line: 177 } |  |  | 0.790 |
| walker |  | 3154 | 24 | Code::CodeKey { rung: Decl, file: neco.h, decl: 76, sub: 0, line: 179 } |  |  | 0.796 |
| ns | 3154 |  | 308 | Posix wrappers: the non-blocking fd operations | 2.7 |  | 0.796 |
| walker |  | 3169 | 15 | Code::CodeKey { rung: Doc, file: neco.h, decl: 78, sub: 0, line: 191 } |  |  | 0.796 |
| walker |  | 3185 | 16 | Code::CodeKey { rung: Doc, file: neco.h, decl: 77, sub: 0, line: 188 } |  |  | 0.797 |
| ns | 3304 |  | 150 | File descriptor helpers: neco_setnonblock and neco_wait | 2.8 |  | 0.790 |
| walker |  | 3315 | 130 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.790 |
| ns | 3394 |  | 90 | Networking utilities: neco_serve and neco_dial | 2.9 |  | 0.785 |
| walker |  | 3551 | 236 | Code::CodeKey { rung: Names, file: neco.h, decl: 0, sub: 8, line: 0 } |  |  | 0.796 |
| walker |  | 3567 | 16 | Code::CodeKey { rung: Decl, file: neco.h, decl: 94, sub: 0, line: 231 } |  |  | 0.796 |
| ns | 3602 |  | 208 | Cancelation: neco_cancel, the type/state constants, cleanup macros | 2.10 |  | 0.801 |
| walker |  | 3774 | 207 | Code::CodeKey { rung: Names, file: neco.h, decl: 0, sub: 9, line: 0 } |  |  | 0.803 |
| ns | 3861 |  | 259 | Streams and buffered I/O: the name of every entry point | 2.11 |  | 0.771 |
| ns | 3986 |  | 125 | Random number generator: CSPRNG/PRNG attributes and three functions | 2.12 |  | 0.773 |
| walker |  | 4001 | 227 | Code::CodeKey { rung: Names, file: neco.h, decl: 0, sub: 10, line: 0 } |  |  | 0.780 |
| ns | 4062 |  | 76 | Signals: watch, wait, unwatch | 2.13 |  | 0.777 |
| ns | 4105 |  | 43 | Background worker: neco_work | 2.14 |  | 0.775 |
| walker |  | 4164 | 163 | Code::CodeKey { rung: Decl, file: neco.h, decl: 106, sub: 0, line: 287 } |  |  | 0.778 |
| ns | 4336 |  | 231 | neco_stats: every runtime counter, plus introspection | 2.15 |  | 0.785 |
| walker |  | 4392 | 228 | Code::CodeKey { rung: Names, file: neco.h, decl: 0, sub: 11, line: 0 } |  |  | 0.806 |
| ns | 4419 |  | 83 | Global environment: allocator and process-wide defaults | 2.16 |  | 0.807 |
| walker |  | 4585 | 193 | Code::CodeKey { rung: Names, file: neco.h, decl: 0, sub: 12, line: 0 } |  |  | 0.825 |
| ns | 4667 |  | 248 | The neco_main macro, expanded | 2.17 |  | 0.802 |
| ns | 4775 |  | 108 | neco.h tail: private entry points and the EAI_SYSTEM shim | 2.18 |  | 0.789 |
| walker |  | 4803 | 218 | Code::CodeKey { rung: Names, file: neco.h, decl: 0, sub: 13, line: 0 } |  |  | 0.810 |
| ns | 4968 |  | 193 | Deadlines and cancelation, the governing rules | 3.1 |  | 0.795 |
| walker |  | 5002 | 199 | Code::CodeKey { rung: Names, file: neco.h, decl: 0, sub: 14, line: 0 } |  |  | 0.803 |
| walker |  | 5193 | 191 | Code::CodeKey { rung: Names, file: neco.h, decl: 0, sub: 15, line: 0 } |  |  | 0.821 |
| ns | 5259 |  | 291 | Error semantics: which errors panic, which leak errno, and lasterr | 3.2 |  | 0.801 |
| walker |  | 5409 | 216 | Code::CodeKey { rung: Decl, file: neco.h, decl: 162, sub: 0, line: 427 } |  |  | 0.822 |
| ns | 5493 |  | 234 | Async cancelation, and turning cancelation off | 3.3 |  | 0.807 |
| walker |  | 5536 | 127 | Code::CodeKey { rung: Doc, file: neco.h, decl: 53, sub: 0, line: 135 } |  |  | 0.809 |
| walker |  | 5712 | 176 | Markdown::Section { file: README.md, section_index: 4, keeps_default_concavity: false } |  |  | 0.809 |
| ns | 5725 |  | 232 | Platform notes: what does not work on Windows and WebAssembly | 3.4 |  | 0.791 |
| walker |  | 5913 | 201 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.791 |
| ns | 6115 |  | 390 | The scheduler, context switching, and the thread-local runtime | 3.5 |  | 0.761 |
| walker |  | 6137 | 224 | Markdown::Section { file: README.md, section_index: 15, keeps_default_concavity: false } |  |  | 0.779 |
| ns | 6230 |  | 115 | How docs/API.md is produced | 3.6 |  | 0.772 |
| walker |  | 6416 | 279 | Code::CodeKey { rung: Names, file: neco.c, decl: 0, sub: 0, line: 0 } |  |  | 0.772 |
| walker |  | 6422 | 6 | Code::CodeKey { rung: Decl, file: neco.c, decl: 15, sub: 0, line: 1145 } |  |  | 0.772 |
| walker |  | 6429 | 7 | Code::CodeKey { rung: Decl, file: neco.c, decl: 4, sub: 0, line: 187 } |  |  | 0.772 |
| walker |  | 6442 | 13 | Code::CodeKey { rung: Decl, file: neco.c, decl: 14, sub: 0, line: 1143 } |  |  | 0.772 |
| walker |  | 6456 | 14 | Code::CodeKey { rung: Decl, file: neco.c, decl: 12, sub: 0, line: 1035 } |  |  | 0.772 |
| walker |  | 6479 | 23 | Code::CodeKey { rung: Decl, file: neco.c, decl: 11, sub: 0, line: 1023 } |  |  | 0.772 |
| walker |  | 6521 | 42 | Code::CodeKey { rung: Decl, file: neco.c, decl: 1, sub: 0, line: 133 } |  |  | 0.772 |
| walker |  | 6565 | 44 | Code::CodeKey { rung: Decl, file: neco.c, decl: 5, sub: 0, line: 197 } |  |  | 0.772 |
| ns | 6616 |  | 386 | neco.c compile-time options: the complete knob list | 4.1 |  | 0.750 |
| walker |  | 6634 | 69 | Code::CodeKey { rung: Decl, file: neco.c, decl: 2, sub: 0, line: 172 } |  |  | 0.750 |
| walker |  | 6703 | 69 | Code::CodeKey { rung: Decl, file: neco.c, decl: 7, sub: 0, line: 234 } |  |  | 0.750 |
| walker |  | 6775 | 72 | Code::CodeKey { rung: Decl, file: neco.c, decl: 8, sub: 0, line: 241 } |  |  | 0.750 |
| ns | 6875 |  | 259 | Amalgamation structure and the embedded-source boundaries | 4.2 |  | 0.729 |
| walker |  | 6880 | 105 | Code::CodeKey { rung: Decl, file: neco.c, decl: 3, sub: 0, line: 179 } |  |  | 0.729 |
| ns | 7007 |  | 132 | Section map of Neco's own implementation | 4.3 |  | 0.722 |
| walker |  | 7083 | 203 | Code::CodeKey { rung: Decl, file: neco.c, decl: 13, sub: 0, line: 1124 } |  |  | 0.722 |
| ns | 7292 |  | 285 | struct coroutine: identity, stack, arguments, scheduling flags | 4.4 |  | 0.706 |
| walker |  | 7344 | 261 | Code::CodeKey { rung: Names, file: neco.c, decl: 0, sub: 1, line: 0 } |  |  | 0.706 |
| walker |  | 7359 | 15 | Code::CodeKey { rung: Decl, file: neco.c, decl: 28, sub: 0, line: 1251 } |  |  | 0.706 |
| walker |  | 7380 | 21 | Code::CodeKey { rung: Decl, file: neco.c, decl: 27, sub: 0, line: 1204 } |  |  | 0.706 |
| walker |  | 7438 | 58 | Code::CodeKey { rung: Decl, file: neco.c, decl: 22, sub: 0, line: 1155 } |  |  | 0.706 |
| walker |  | 7449 | 11 | Code::CodeKey { rung: Doc, file: neco.c, decl: 29, sub: 0, line: 1282 } |  |  | 0.706 |
| ns | 7585 |  | 293 | What neco_chan and neco_gen actually are | 4.5 |  | 0.692 |
| walker |  | 7721 | 272 | Code::CodeKey { rung: Names, file: neco.c, decl: 0, sub: 2, line: 0 } |  |  | 0.692 |
| walker |  | 7740 | 19 | Code::CodeKey { rung: Decl, file: neco.c, decl: 36, sub: 0, line: 1348 } |  |  | 0.692 |
| walker |  | 7764 | 24 | Code::CodeKey { rung: Decl, file: neco.c, decl: 35, sub: 0, line: 1344 } |  |  | 0.692 |
| ns | 7769 |  | 184 | Where the event queue backend is chosen | 4.6 |  | 0.683 |
| walker |  | 7790 | 26 | Code::CodeKey { rung: Decl, file: neco.c, decl: 39, sub: 0, line: 1814 } |  |  | 0.683 |
| walker |  | 7819 | 29 | Code::CodeKey { rung: Decl, file: neco.c, decl: 43, sub: 0, line: 1875 } |  |  | 0.683 |
| walker |  | 7849 | 30 | Code::CodeKey { rung: Decl, file: neco.c, decl: 44, sub: 0, line: 1880 } |  |  | 0.683 |
| walker |  | 7860 | 11 | Code::CodeKey { rung: Doc, file: neco.c, decl: 30, sub: 0, line: 1293 } |  |  | 0.683 |
| walker |  | 7959 | 99 | Code::CodeKey { rung: Decl, file: neco.c, decl: 37, sub: 0, line: 1364 } |  |  | 0.683 |
| ns | 7977 |  | 208 | Test suite knobs: compilers, sanitizers, valgrind | 5.1 |  | 0.674 |
| walker |  | 8080 | 121 | Code::CodeKey { rung: Decl, file: neco.c, decl: 34, sub: 0, line: 1330 } |  |  | 0.674 |
| walker |  | 8227 | 147 | Code::CodeKey { rung: Decl, file: neco.c, decl: 40, sub: 0, line: 1819 } |  |  | 0.674 |
| walker |  | 8254 | 27 | Code::CodeKey { rung: Doc, file: neco.c, decl: 31, sub: 0, line: 1312 } |  |  | 0.674 |
| ns | 8258 |  | 281 | The private, undocumented functions the tests may call | 5.2 |  | 0.662 |
| walker |  | 8294 | 40 | Code::CodeKey { rung: Doc, file: neco.c, decl: 33, sub: 0, line: 1320 } |  |  | 0.662 |
| ns | 8332 |  | 74 | Every function-like macro in tests/tests.h | 5.3 |  | 0.658 |
| walker |  | 8336 | 42 | Code::CodeKey { rung: Doc, file: neco.c, decl: 42, sub: 0, line: 1854 } |  |  | 0.658 |
| walker |  | 8559 | 223 | Code::CodeKey { rung: Names, file: neco.c, decl: 0, sub: 3, line: 0 } |  |  | 0.658 |
| walker |  | 8587 | 28 | Code::CodeKey { rung: Decl, file: neco.c, decl: 47, sub: 0, line: 1899 } |  |  | 0.658 |
| ns | 8595 |  | 263 | How run.sh compiles and runs each test | 5.4 |  | 0.646 |
| ns | 8792 |  | 197 | The four NECO_TESTING-only shim headers | 5.5 |  | 0.636 |
| walker |  | 8828 | 241 | Code::CodeKey { rung: Names, file: neco.c, decl: 0, sub: 4, line: 0 } |  |  | 0.636 |
| walker |  | 8845 | 17 | Code::CodeKey { rung: Doc, file: neco.c, decl: 65, sub: 0, line: 1944 } |  |  | 0.636 |
| ns | 9091 |  | 299 | deps/sco.h: the scheduler contract | 6.1 |  | 0.622 |
| walker |  | 9174 | 329 | Code::CodeKey { rung: Names, file: neco.c, decl: 0, sub: 5, line: 0 } |  |  | 0.622 |
| ns | 9395 |  | 304 | deps/stack.h: the coroutine stack allocator | 6.2 |  | 0.611 |
| walker |  | 9477 | 303 | Code::CodeKey { rung: Names, file: neco.c, decl: 0, sub: 6, line: 0 } |  |  | 0.611 |
| walker |  | 9485 | 8 | Code::CodeKey { rung: Decl, file: neco.c, decl: 93, sub: 0, line: 2240 } |  |  | 0.611 |
| walker |  | 9505 | 20 | Code::CodeKey { rung: Decl, file: neco.c, decl: 90, sub: 0, line: 2225 } |  |  | 0.611 |
| walker |  | 9543 | 38 | Code::CodeKey { rung: Decl, file: neco.c, decl: 97, sub: 0, line: 2285 } |  |  | 0.611 |
| ns | 9563 |  | 168 | deps/worker.h: the background thread pool | 6.3 |  | 0.605 |
| walker |  | 9585 | 42 | Code::CodeKey { rung: Decl, file: neco.c, decl: 95, sub: 0, line: 2261 } |  |  | 0.605 |
| ns | 9646 |  | 83 | deps/embed.sh: how neco.c is regenerated | 6.4 |  | 0.602 |
| walker |  | 9648 | 63 | Code::CodeKey { rung: Decl, file: neco.c, decl: 87, sub: 0, line: 2165 } |  |  | 0.602 |
| walker |  | 9740 | 92 | Code::CodeKey { rung: Decl, file: neco.c, decl: 91, sub: 0, line: 2229 } |  |  | 0.602 |
| walker |  | 9757 | 17 | Code::CodeKey { rung: Doc, file: neco.c, decl: 89, sub: 0, line: 2192 } |  |  | 0.602 |
| walker |  | 9778 | 21 | Code::CodeKey { rung: Doc, file: neco.c, decl: 85, sub: 0, line: 2154 } |  |  | 0.602 |
| ns | 9827 |  | 181 | examples/select.c: the multi-channel select pattern | 6.5 |  | 0.595 |
| walker |  | 9920 | 142 | Code::CodeKey { rung: Decl, file: neco.c, decl: 94, sub: 0, line: 2247 } |  |  | 0.595 |
| ns | 9939 |  | 112 | CI | 6.6 |  | 0.590 |
| ns | 9984 |  | 45 | License | 6.7 |  | 0.588 |
| walker |  | 9997 | 77 | Code::CodeKey { rung: Decl, file: neco.c, decl: 96, sub: 0, line: 2267 } |  |  | 0.588 |
