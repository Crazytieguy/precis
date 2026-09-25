Score(3000)=0.758 I=0.888 C=0.647 ns_rows≤3K=17/53 grid(1000/1442/2080/3000/4327/6240/9000)=0.680/0.765/0.736/0.758/0.804/0.769/0.634

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
| walker |  | 1261 | 168 | Code::CodeKey { rung: Names, file: neco.h, decl: 0, sub: 0, line: 0 } |  |  | 0.778 |
| ns | 1342 |  | 262 | Neco error codes, part 1 | 1.10 |  | 0.737 |
| walker |  | 1430 | 169 | Code::CodeKey { rung: Names, file: neco.h, decl: 0, sub: 1, line: 0 } |  |  | 0.765 |
| walker |  | 1586 | 156 | Code::CodeKey { rung: Names, file: neco.h, decl: 0, sub: 2, line: 0 } |  |  | 0.768 |
| ns | 1606 |  | 264 | Neco error codes, part 2, and the error functions | 1.11 | 1.10 | 0.734 |
| walker |  | 1769 | 183 | Code::CodeKey { rung: Names, file: neco.h, decl: 0, sub: 3, line: 0 } |  |  | 0.737 |
| walker |  | 1940 | 171 | Code::CodeKey { rung: Names, file: neco.h, decl: 0, sub: 4, line: 0 } |  |  | 0.740 |
| ns | 2030 |  | 424 | Channels: doc and complete signature set | 2.1 |  | 0.733 |
| walker |  | 2105 | 165 | Code::CodeKey { rung: Names, file: neco.h, decl: 0, sub: 5, line: 0 } |  |  | 0.737 |
| ns | 2248 |  | 218 | Generators: complete signature set | 2.2 |  | 0.745 |
| walker |  | 2268 | 163 | Code::CodeKey { rung: Names, file: neco.h, decl: 0, sub: 6, line: 0 } |  |  | 0.748 |
| ns | 2377 |  | 129 | Time: the duration constants and neco_now() | 2.3 |  | 0.733 |
| walker |  | 2437 | 169 | Code::CodeKey { rung: Names, file: neco.h, decl: 0, sub: 7, line: 0 } |  |  | 0.735 |
| walker |  | 2449 | 12 | Code::CodeKey { rung: Doc, file: neco.h, decl: 67, sub: 0, line: 169 } |  |  | 0.735 |
| ns | 2556 |  | 179 | Mutexes: type, static initializer, and all eight operations | 2.4 |  | 0.743 |
| walker |  | 2606 | 157 | Code::CodeKey { rung: Names, file: neco.h, decl: 0, sub: 8, line: 0 } |  |  | 0.745 |
| walker |  | 2625 | 19 | Code::CodeKey { rung: Decl, file: neco.h, decl: 75, sub: 0, line: 177 } |  |  | 0.746 |
| ns | 2704 |  | 148 | WaitGroups: type, initializer and five operations | 2.5 |  | 0.752 |
| walker |  | 2804 | 179 | Code::CodeKey { rung: Names, file: neco.h, decl: 0, sub: 9, line: 0 } |  |  | 0.754 |
| walker |  | 2828 | 24 | Code::CodeKey { rung: Decl, file: neco.h, decl: 76, sub: 0, line: 179 } |  |  | 0.754 |
| walker |  | 2843 | 15 | Code::CodeKey { rung: Doc, file: neco.h, decl: 78, sub: 0, line: 191 } |  |  | 0.755 |
| ns | 2846 |  | 142 | Condition variables: type, initializer and five operations | 2.6 |  | 0.757 |
| walker |  | 2859 | 16 | Code::CodeKey { rung: Doc, file: neco.h, decl: 77, sub: 0, line: 188 } |  |  | 0.758 |
| walker |  | 2989 | 130 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.758 |
| ns | 3154 |  | 308 | Posix wrappers: the non-blocking fd operations | 2.7 |  | 0.765 |
| walker |  | 3183 | 194 | Code::CodeKey { rung: Names, file: neco.h, decl: 0, sub: 10, line: 0 } |  |  | 0.769 |
| walker |  | 3201 | 18 | Code::CodeKey { rung: Decl, file: neco.h, decl: 94, sub: 0, line: 231 } |  |  | 0.769 |
| ns | 3304 |  | 150 | File descriptor helpers: neco_setnonblock and neco_wait | 2.8 |  | 0.763 |
| walker |  | 3372 | 171 | Code::CodeKey { rung: Names, file: neco.h, decl: 0, sub: 11, line: 0 } |  |  | 0.766 |
| ns | 3394 |  | 90 | Networking utilities: neco_serve and neco_dial | 2.9 |  | 0.768 |
| walker |  | 3552 | 180 | Code::CodeKey { rung: Names, file: neco.h, decl: 0, sub: 12, line: 0 } |  |  | 0.769 |
| ns | 3602 |  | 208 | Cancelation: neco_cancel, the type/state constants, cleanup macros | 2.10 |  | 0.775 |
| walker |  | 3715 | 163 | Code::CodeKey { rung: Decl, file: neco.h, decl: 106, sub: 0, line: 287 } |  |  | 0.779 |
| ns | 3861 |  | 259 | Streams and buffered I/O: the name of every entry point | 2.11 |  | 0.747 |
| walker |  | 3899 | 184 | Code::CodeKey { rung: Names, file: neco.h, decl: 0, sub: 13, line: 0 } |  |  | 0.765 |
| ns | 3986 |  | 125 | Random number generator: CSPRNG/PRNG attributes and three functions | 2.12 |  | 0.767 |
| ns | 4062 |  | 76 | Signals: watch, wait, unwatch | 2.13 |  | 0.765 |
| walker |  | 4066 | 167 | Code::CodeKey { rung: Names, file: neco.h, decl: 0, sub: 14, line: 0 } |  |  | 0.780 |
| ns | 4105 |  | 43 | Background worker: neco_work | 2.14 |  | 0.778 |
| walker |  | 4240 | 174 | Code::CodeKey { rung: Names, file: neco.h, decl: 0, sub: 15, line: 0 } |  |  | 0.794 |
| ns | 4336 |  | 231 | neco_stats: every runtime counter, plus introspection | 2.15 |  | 0.800 |
| ns | 4419 |  | 83 | Global environment: allocator and process-wide defaults | 2.16 |  | 0.801 |
| walker |  | 4423 | 183 | Code::CodeKey { rung: Names, file: neco.h, decl: 0, sub: 16, line: 0 } |  |  | 0.822 |
| walker |  | 4597 | 174 | Code::CodeKey { rung: Names, file: neco.h, decl: 0, sub: 17, line: 0 } |  |  | 0.828 |
| ns | 4667 |  | 248 | The neco_main macro, expanded | 2.17 |  | 0.805 |
| ns | 4775 |  | 108 | neco.h tail: private entry points and the EAI_SYSTEM shim | 2.18 |  | 0.792 |
| walker |  | 4848 | 251 | Code::CodeKey { rung: Names, file: neco.h, decl: 0, sub: 18, line: 0 } |  |  | 0.813 |
| ns | 4968 |  | 193 | Deadlines and cancelation, the governing rules | 3.1 |  | 0.799 |
| walker |  | 5064 | 216 | Code::CodeKey { rung: Decl, file: neco.h, decl: 162, sub: 0, line: 427 } |  |  | 0.821 |
| walker |  | 5115 | 51 | Code::CodeKey { rung: Doc, file: neco.h, decl: 33, sub: 0, line: 90 } |  |  | 0.821 |
| walker |  | 5166 | 51 | Code::CodeKey { rung: Doc, file: neco.h, decl: 60, sub: 0, line: 149 } |  |  | 0.822 |
| ns | 5259 |  | 291 | Error semantics: which errors panic, which leak errno, and lasterr | 3.2 |  | 0.801 |
| walker |  | 5342 | 176 | Markdown::Section { file: README.md, section_index: 4, keeps_default_concavity: false } |  |  | 0.801 |
| ns | 5493 |  | 234 | Async cancelation, and turning cancelation off | 3.3 |  | 0.787 |
| walker |  | 5543 | 201 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.787 |
| walker |  | 5617 | 74 | Code::CodeKey { rung: Doc, file: neco.h, decl: 1, sub: 0, line: 35 } |  |  | 0.788 |
| walker |  | 5695 | 78 | Code::CodeKey { rung: Doc, file: neco.h, decl: 15, sub: 0, line: 61 } |  |  | 0.805 |
| ns | 5725 |  | 232 | Platform notes: what does not work on Windows and WebAssembly | 3.4 |  | 0.787 |
| walker |  | 5781 | 86 | Code::CodeKey { rung: Doc, file: neco.h, decl: 43, sub: 0, line: 113 } |  |  | 0.788 |
| walker |  | 6005 | 224 | Markdown::Section { file: README.md, section_index: 15, keeps_default_concavity: false } |  |  | 0.790 |
| ns | 6115 |  | 390 | The scheduler, context switching, and the thread-local runtime | 3.5 |  | 0.776 |
| ns | 6230 |  | 115 | How docs/API.md is produced | 3.6 |  | 0.769 |
| walker |  | 6245 | 240 | Code::CodeKey { rung: Names, file: neco.c, decl: 0, sub: 0, line: 0 } |  |  | 0.769 |
| walker |  | 6252 | 7 | Code::CodeKey { rung: Decl, file: neco.c, decl: 4, sub: 0, line: 187 } |  |  | 0.769 |
| walker |  | 6260 | 8 | Code::CodeKey { rung: Decl, file: neco.c, decl: 15, sub: 0, line: 1145 } |  |  | 0.769 |
| walker |  | 6273 | 13 | Code::CodeKey { rung: Decl, file: neco.c, decl: 14, sub: 0, line: 1143 } |  |  | 0.769 |
| walker |  | 6287 | 14 | Code::CodeKey { rung: Decl, file: neco.c, decl: 12, sub: 0, line: 1035 } |  |  | 0.769 |
| walker |  | 6310 | 23 | Code::CodeKey { rung: Decl, file: neco.c, decl: 11, sub: 0, line: 1023 } |  |  | 0.769 |
| walker |  | 6352 | 42 | Code::CodeKey { rung: Decl, file: neco.c, decl: 1, sub: 0, line: 133 } |  |  | 0.769 |
| walker |  | 6396 | 44 | Code::CodeKey { rung: Decl, file: neco.c, decl: 5, sub: 0, line: 197 } |  |  | 0.769 |
| walker |  | 6465 | 69 | Code::CodeKey { rung: Decl, file: neco.c, decl: 2, sub: 0, line: 172 } |  |  | 0.769 |
| walker |  | 6534 | 69 | Code::CodeKey { rung: Decl, file: neco.c, decl: 7, sub: 0, line: 234 } |  |  | 0.769 |
| walker |  | 6606 | 72 | Code::CodeKey { rung: Decl, file: neco.c, decl: 8, sub: 0, line: 241 } |  |  | 0.769 |
| ns | 6616 |  | 386 | neco.c compile-time options: the complete knob list | 4.1 |  | 0.748 |
| walker |  | 6711 | 105 | Code::CodeKey { rung: Decl, file: neco.c, decl: 3, sub: 0, line: 179 } |  |  | 0.748 |
| ns | 6875 |  | 259 | Amalgamation structure and the embedded-source boundaries | 4.2 |  | 0.727 |
| walker |  | 6914 | 203 | Code::CodeKey { rung: Decl, file: neco.c, decl: 13, sub: 0, line: 1124 } |  |  | 0.727 |
| ns | 7007 |  | 132 | Section map of Neco's own implementation | 4.3 |  | 0.720 |
| walker |  | 7117 | 203 | Code::CodeKey { rung: Names, file: neco.c, decl: 0, sub: 1, line: 0 } |  |  | 0.720 |
| walker |  | 7175 | 58 | Code::CodeKey { rung: Decl, file: neco.c, decl: 22, sub: 0, line: 1155 } |  |  | 0.720 |
| ns | 7292 |  | 285 | struct coroutine: identity, stack, arguments, scheduling flags | 4.4 |  | 0.704 |
| walker |  | 7382 | 207 | Code::CodeKey { rung: Names, file: neco.c, decl: 0, sub: 2, line: 0 } |  |  | 0.704 |
| walker |  | 7397 | 15 | Code::CodeKey { rung: Decl, file: neco.c, decl: 28, sub: 0, line: 1251 } |  |  | 0.704 |
| walker |  | 7418 | 21 | Code::CodeKey { rung: Decl, file: neco.c, decl: 27, sub: 0, line: 1204 } |  |  | 0.704 |
| walker |  | 7444 | 26 | Code::CodeKey { rung: Decl, file: neco.c, decl: 35, sub: 0, line: 1344 } |  |  | 0.704 |
| walker |  | 7565 | 121 | Code::CodeKey { rung: Decl, file: neco.c, decl: 34, sub: 0, line: 1330 } |  |  | 0.704 |
| walker |  | 7576 | 11 | Code::CodeKey { rung: Doc, file: neco.c, decl: 29, sub: 0, line: 1282 } |  |  | 0.704 |
| ns | 7585 |  | 293 | What neco_chan and neco_gen actually are | 4.5 |  | 0.690 |
| walker |  | 7587 | 11 | Code::CodeKey { rung: Doc, file: neco.c, decl: 30, sub: 0, line: 1293 } |  |  | 0.690 |
| ns | 7769 |  | 184 | Where the event queue backend is chosen | 4.6 |  | 0.680 |
| walker |  | 7799 | 212 | Code::CodeKey { rung: Names, file: neco.c, decl: 0, sub: 3, line: 0 } |  |  | 0.680 |
| walker |  | 7818 | 19 | Code::CodeKey { rung: Decl, file: neco.c, decl: 36, sub: 0, line: 1348 } |  |  | 0.680 |
| walker |  | 7844 | 26 | Code::CodeKey { rung: Decl, file: neco.c, decl: 39, sub: 0, line: 1814 } |  |  | 0.680 |
| walker |  | 7872 | 28 | Code::CodeKey { rung: Decl, file: neco.c, decl: 44, sub: 0, line: 1880 } |  |  | 0.680 |
| walker |  | 7901 | 29 | Code::CodeKey { rung: Decl, file: neco.c, decl: 43, sub: 0, line: 1875 } |  |  | 0.680 |
| ns | 7977 |  | 208 | Test suite knobs: compilers, sanitizers, valgrind | 5.1 |  | 0.672 |
| walker |  | 8000 | 99 | Code::CodeKey { rung: Decl, file: neco.c, decl: 37, sub: 0, line: 1364 } |  |  | 0.672 |
| walker |  | 8147 | 147 | Code::CodeKey { rung: Decl, file: neco.c, decl: 40, sub: 0, line: 1819 } |  |  | 0.672 |
| ns | 8258 |  | 281 | The private, undocumented functions the tests may call | 5.2 |  | 0.660 |
| ns | 8332 |  | 74 | Every function-like macro in tests/tests.h | 5.3 |  | 0.655 |
| walker |  | 8338 | 191 | Code::CodeKey { rung: Names, file: neco.c, decl: 0, sub: 4, line: 0 } |  |  | 0.655 |
| walker |  | 8366 | 28 | Code::CodeKey { rung: Decl, file: neco.c, decl: 47, sub: 0, line: 1899 } |  |  | 0.655 |
| walker |  | 8564 | 198 | Code::CodeKey { rung: Names, file: neco.c, decl: 0, sub: 5, line: 0 } |  |  | 0.655 |
| walker |  | 8581 | 17 | Code::CodeKey { rung: Doc, file: neco.c, decl: 65, sub: 0, line: 1944 } |  |  | 0.655 |
| ns | 8595 |  | 263 | How run.sh compiles and runs each test | 5.4 |  | 0.643 |
| walker |  | 8608 | 27 | Code::CodeKey { rung: Doc, file: neco.c, decl: 31, sub: 0, line: 1312 } |  |  | 0.643 |
| ns | 8792 |  | 197 | The four NECO_TESTING-only shim headers | 5.5 |  | 0.634 |
| walker |  | 8855 | 247 | Code::CodeKey { rung: Names, file: neco.c, decl: 0, sub: 6, line: 0 } |  |  | 0.634 |
| ns | 9091 |  | 299 | deps/sco.h: the scheduler contract | 6.1 |  | 0.620 |
| walker |  | 9109 | 254 | Code::CodeKey { rung: Names, file: neco.c, decl: 0, sub: 7, line: 0 } |  |  | 0.620 |
| walker |  | 9172 | 63 | Code::CodeKey { rung: Decl, file: neco.c, decl: 87, sub: 0, line: 2165 } |  |  | 0.620 |
| walker |  | 9189 | 17 | Code::CodeKey { rung: Doc, file: neco.c, decl: 89, sub: 0, line: 2192 } |  |  | 0.620 |
| walker |  | 9210 | 21 | Code::CodeKey { rung: Doc, file: neco.c, decl: 85, sub: 0, line: 2154 } |  |  | 0.620 |
| ns | 9395 |  | 304 | deps/stack.h: the coroutine stack allocator | 6.2 |  | 0.609 |
| walker |  | 9437 | 227 | Code::CodeKey { rung: Names, file: neco.c, decl: 0, sub: 8, line: 0 } |  |  | 0.609 |
| walker |  | 9445 | 8 | Code::CodeKey { rung: Decl, file: neco.c, decl: 93, sub: 0, line: 2240 } |  |  | 0.609 |
| walker |  | 9465 | 20 | Code::CodeKey { rung: Decl, file: neco.c, decl: 90, sub: 0, line: 2225 } |  |  | 0.609 |
| walker |  | 9503 | 38 | Code::CodeKey { rung: Decl, file: neco.c, decl: 97, sub: 0, line: 2285 } |  |  | 0.609 |
| walker |  | 9545 | 42 | Code::CodeKey { rung: Decl, file: neco.c, decl: 95, sub: 0, line: 2261 } |  |  | 0.609 |
| ns | 9563 |  | 168 | deps/worker.h: the background thread pool | 6.3 |  | 0.603 |
| walker |  | 9637 | 92 | Code::CodeKey { rung: Decl, file: neco.c, decl: 91, sub: 0, line: 2229 } |  |  | 0.603 |
| ns | 9646 |  | 83 | deps/embed.sh: how neco.c is regenerated | 6.4 |  | 0.600 |
| walker |  | 9779 | 142 | Code::CodeKey { rung: Decl, file: neco.c, decl: 94, sub: 0, line: 2247 } |  |  | 0.600 |
| ns | 9827 |  | 181 | examples/select.c: the multi-channel select pattern | 6.5 |  | 0.593 |
| ns | 9939 |  | 112 | CI | 6.6 |  | 0.588 |
| walker |  | 9966 | 187 | Code::CodeKey { rung: Decl, file: neco.c, decl: 96, sub: 0, line: 2267 } |  |  | 0.588 |
| ns | 9984 |  | 45 | License | 6.7 |  | 0.586 |
| walker |  | 9992 | 26 | Code::CodeKey { rung: Names, file: neco.c, decl: 0, sub: 9, line: 0 } |  |  | 0.586 |
