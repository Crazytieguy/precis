Score(3000)=0.787 I=0.888 C=0.697 ns_rows≤3K=17/53 grid(1000/1442/2080/3000/4327/6240/9000)=0.681/0.765/0.764/0.787/0.780/0.764/0.630

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 28 | 28 | Fs::DirListing { dir: . } |  |  | 0.000 |
| ns | 47 |  | 47 | What Neco is | 1.1 |  | 0.000 |
| walker |  | 53 | 25 | Fs::DirListing { dir: docs } |  |  | 0.000 |
| walker |  | 64 | 11 | Markdown::ReadmeHeadline { file: docs/README.md } |  |  | 0.000 |
| walker |  | 85 | 21 | Fs::DirListing { dir: docs/assets } |  |  | 0.000 |
| walker |  | 100 | 15 | Fs::DirListing { dir: .github } |  |  | 0.000 |
| walker |  | 104 | 4 | Fs::DirListing { dir: .github/workflows } |  |  | 0.000 |
| walker |  | 118 | 14 | Fs::DirListing { dir: docs/tools } |  |  | 0.000 |
| ns | 121 |  | 74 | How the library is consumed and built | 1.2 |  | 0.000 |
| ns | 149 |  | 28 | Complete root listing | 1.3 |  | 0.338 |
| walker |  | 160 | 42 | Fs::DirListing { dir: deps } |  |  | 0.360 |
| walker |  | 211 | 51 | Fs::DirListing { dir: examples } |  |  | 0.442 |
| ns | 242 |  | 93 | examples/ and deps/ listings | 1.4 |  | 0.504 |
| walker |  | 337 | 126 | Fs::DirListing { dir: tests } |  |  | 0.525 |
| ns | 337 |  | 95 | docs/ and .github/ trees, complete to the leaves | 1.5 |  | 0.525 |
| ns | 463 |  | 126 | Complete tests/ listing | 1.6 |  | 0.537 |
| ns | 595 |  | 132 | Running the tests, and where the examples live | 1.7 |  | 0.499 |
| walker |  | 764 | 427 | Markdown::ReadmeHeadline { file: README.md } |  |  | 0.746 |
| ns | 864 |  | 269 | Complete roster of neco.h's API groups | 1.8 |  | 0.678 |
| walker |  | 997 | 233 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.681 |
| walker |  | 1041 | 44 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: true } |  |  | 0.681 |
| walker |  | 1060 | 19 | Markdown::Section { file: README.md, section_index: 17, keeps_default_concavity: false } |  |  | 0.682 |
| ns | 1080 |  | 216 | Basic operations: every coroutine-lifecycle signature | 1.9 |  | 0.639 |
| walker |  | 1123 | 63 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.739 |
| walker |  | 1194 | 71 | Markdown::HeadingsOutline { file: docs/assets/API_head.md } |  |  | 0.739 |
| ns | 1342 |  | 262 | Neco error codes, part 1 | 1.10 |  | 0.700 |
| walker |  | 1412 | 218 | Code::CodeKey { rung: Names, file: neco.h, decl: 0, sub: 0, line: 0 } |  |  | 0.765 |
| walker |  | 1486 | 74 | Code::CodeKey { rung: Doc, file: neco.h, decl: 1, sub: 0, line: 35 } |  |  | 0.765 |
| ns | 1606 |  | 264 | Neco error codes, part 2, and the error functions | 1.11 | 1.10 | 0.731 |
| walker |  | 1689 | 203 | Code::CodeKey { rung: Names, file: neco.h, decl: 0, sub: 1, line: 0 } |  |  | 0.733 |
| walker |  | 1767 | 78 | Code::CodeKey { rung: Doc, file: neco.h, decl: 15, sub: 0, line: 61 } |  |  | 0.736 |
| walker |  | 1981 | 214 | Code::CodeKey { rung: Names, file: neco.h, decl: 0, sub: 2, line: 0 } |  |  | 0.742 |
| ns | 2030 |  | 424 | Channels: doc and complete signature set | 2.1 |  | 0.763 |
| walker |  | 2032 | 51 | Code::CodeKey { rung: Doc, file: neco.h, decl: 33, sub: 0, line: 90 } |  |  | 0.764 |
| walker |  | 2224 | 192 | Code::CodeKey { rung: Names, file: neco.h, decl: 0, sub: 3, line: 0 } |  |  | 0.768 |
| ns | 2248 |  | 218 | Generators: complete signature set | 2.2 |  | 0.775 |
| walker |  | 2310 | 86 | Code::CodeKey { rung: Doc, file: neco.h, decl: 43, sub: 0, line: 113 } |  |  | 0.776 |
| ns | 2377 |  | 129 | Time: the duration constants and neco_now() | 2.3 |  | 0.760 |
| walker |  | 2534 | 224 | Code::CodeKey { rung: Names, file: neco.h, decl: 0, sub: 4, line: 0 } |  |  | 0.764 |
| ns | 2556 |  | 179 | Mutexes: type, static initializer, and all eight operations | 2.4 |  | 0.770 |
| ns | 2704 |  | 148 | WaitGroups: type, initializer and five operations | 2.5 |  | 0.758 |
| walker |  | 2757 | 223 | Code::CodeKey { rung: Names, file: neco.h, decl: 0, sub: 5, line: 0 } |  |  | 0.780 |
| walker |  | 2808 | 51 | Code::CodeKey { rung: Doc, file: neco.h, decl: 60, sub: 0, line: 149 } |  |  | 0.781 |
| ns | 2846 |  | 142 | Condition variables: type, initializer and five operations | 2.6 |  | 0.785 |
| walker |  | 2990 | 182 | Code::CodeKey { rung: Names, file: neco.h, decl: 0, sub: 6, line: 0 } |  |  | 0.787 |
| walker |  | 3002 | 12 | Code::CodeKey { rung: Doc, file: neco.h, decl: 67, sub: 0, line: 169 } |  |  | 0.787 |
| ns | 3154 |  | 308 | Posix wrappers: the non-blocking fd operations | 2.7 |  | 0.773 |
| walker |  | 3214 | 212 | Code::CodeKey { rung: Names, file: neco.h, decl: 0, sub: 7, line: 0 } |  |  | 0.787 |
| walker |  | 3231 | 17 | Code::CodeKey { rung: Decl, file: neco.h, decl: 75, sub: 0, line: 177 } |  |  | 0.791 |
| walker |  | 3255 | 24 | Code::CodeKey { rung: Decl, file: neco.h, decl: 76, sub: 0, line: 179 } |  |  | 0.796 |
| walker |  | 3270 | 15 | Code::CodeKey { rung: Doc, file: neco.h, decl: 78, sub: 0, line: 191 } |  |  | 0.797 |
| walker |  | 3286 | 16 | Code::CodeKey { rung: Doc, file: neco.h, decl: 77, sub: 0, line: 188 } |  |  | 0.797 |
| ns | 3304 |  | 150 | File descriptor helpers: neco_setnonblock and neco_wait | 2.8 |  | 0.790 |
| ns | 3394 |  | 90 | Networking utilities: neco_serve and neco_dial | 2.9 |  | 0.785 |
| walker |  | 3413 | 127 | Code::CodeKey { rung: Doc, file: neco.h, decl: 53, sub: 0, line: 135 } |  |  | 0.787 |
| ns | 3602 |  | 208 | Cancelation: neco_cancel, the type/state constants, cleanup macros | 2.10 |  | 0.763 |
| walker |  | 3649 | 236 | Code::CodeKey { rung: Names, file: neco.h, decl: 0, sub: 8, line: 0 } |  |  | 0.798 |
| walker |  | 3665 | 16 | Code::CodeKey { rung: Decl, file: neco.h, decl: 94, sub: 0, line: 231 } |  |  | 0.803 |
| ns | 3861 |  | 259 | Streams and buffered I/O: the name of every entry point | 2.11 |  | 0.770 |
| walker |  | 3872 | 207 | Code::CodeKey { rung: Names, file: neco.h, decl: 0, sub: 9, line: 0 } |  |  | 0.773 |
| ns | 3986 |  | 125 | Random number generator: CSPRNG/PRNG attributes and three functions | 2.12 |  | 0.775 |
| ns | 4062 |  | 76 | Signals: watch, wait, unwatch | 2.13 |  | 0.772 |
| walker |  | 4099 | 227 | Code::CodeKey { rung: Names, file: neco.h, decl: 0, sub: 10, line: 0 } |  |  | 0.779 |
| ns | 4105 |  | 43 | Background worker: neco_work | 2.14 |  | 0.777 |
| walker |  | 4262 | 163 | Code::CodeKey { rung: Decl, file: neco.h, decl: 106, sub: 0, line: 287 } |  |  | 0.780 |
| ns | 4336 |  | 231 | neco_stats: every runtime counter, plus introspection | 2.15 |  | 0.786 |
| ns | 4419 |  | 83 | Global environment: allocator and process-wide defaults | 2.16 |  | 0.788 |
| walker |  | 4490 | 228 | Code::CodeKey { rung: Names, file: neco.h, decl: 0, sub: 11, line: 0 } |  |  | 0.809 |
| ns | 4667 |  | 248 | The neco_main macro, expanded | 2.17 |  | 0.787 |
| walker |  | 4683 | 193 | Code::CodeKey { rung: Names, file: neco.h, decl: 0, sub: 12, line: 0 } |  |  | 0.804 |
| ns | 4775 |  | 108 | neco.h tail: private entry points and the EAI_SYSTEM shim | 2.18 |  | 0.791 |
| walker |  | 4901 | 218 | Code::CodeKey { rung: Names, file: neco.h, decl: 0, sub: 13, line: 0 } |  |  | 0.811 |
| ns | 4968 |  | 193 | Deadlines and cancelation, the governing rules | 3.1 |  | 0.797 |
| walker |  | 5100 | 199 | Code::CodeKey { rung: Names, file: neco.h, decl: 0, sub: 14, line: 0 } |  |  | 0.805 |
| ns | 5259 |  | 291 | Error semantics: which errors panic, which leak errno, and lasterr | 3.2 |  | 0.785 |
| walker |  | 5291 | 191 | Code::CodeKey { rung: Names, file: neco.h, decl: 0, sub: 15, line: 0 } |  |  | 0.802 |
| ns | 5493 |  | 234 | Async cancelation, and turning cancelation off | 3.3 |  | 0.788 |
| walker |  | 5507 | 216 | Code::CodeKey { rung: Decl, file: neco.h, decl: 162, sub: 0, line: 427 } |  |  | 0.809 |
| walker |  | 5539 | 32 | Markdown::Section { file: docs/assets/API_foot.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.809 |
| walker |  | 5669 | 130 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.809 |
| ns | 5725 |  | 232 | Platform notes: what does not work on Windows and WebAssembly | 3.4 |  | 0.792 |
| walker |  | 5780 | 111 | Markdown::Section { file: docs/README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.793 |
| walker |  | 5905 | 125 | Markdown::Section { file: docs/API.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.793 |
| walker |  | 6081 | 176 | Markdown::Section { file: README.md, section_index: 4, keeps_default_concavity: false } |  |  | 0.793 |
| ns | 6115 |  | 390 | The scheduler, context switching, and the thread-local runtime | 3.5 |  | 0.762 |
| ns | 6230 |  | 115 | How docs/API.md is produced | 3.6 |  | 0.764 |
| walker |  | 6282 | 201 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.764 |
| walker |  | 6336 | 54 | Markdown::Section { file: docs/API.md, section_index: 29, keeps_default_concavity: false } |  |  | 0.764 |
| walker |  | 6388 | 52 | Markdown::Section { file: docs/API.md, section_index: 30, keeps_default_concavity: false } |  |  | 0.764 |
| walker |  | 6568 | 180 | Markdown::Section { file: docs/TECHNICAL.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.764 |
| ns | 6616 |  | 386 | neco.c compile-time options: the complete knob list | 4.1 |  | 0.743 |
| walker |  | 6677 | 109 | Markdown::Section { file: docs/assets/API_head.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.743 |
| ns | 6875 |  | 259 | Amalgamation structure and the embedded-source boundaries | 4.2 |  | 0.722 |
| walker |  | 6937 | 260 | Code::CodeKey { rung: Names, file: neco.c, decl: 0, sub: 0, line: 0 } |  |  | 0.722 |
| walker |  | 6984 | 47 | Code::CodeKey { rung: Decl, file: neco.c, decl: 1, sub: 0, line: 133 } |  |  | 0.722 |
| ns | 7007 |  | 132 | Section map of Neco's own implementation | 4.3 |  | 0.715 |
| walker |  | 7042 | 58 | Code::CodeKey { rung: Decl, file: neco.c, decl: 10, sub: 0, line: 1155 } |  |  | 0.715 |
| walker |  | 7052 | 10 | Code::CodeKey { rung: Body, file: neco.c, decl: 2, sub: 0, line: 283 } |  |  | 0.715 |
| walker |  | 7074 | 22 | Code::CodeKey { rung: Body, file: neco.c, decl: 14, sub: 0, line: 1199 } |  |  | 0.715 |
| walker |  | 7279 | 205 | Code::CodeKey { rung: Decl, file: neco.c, decl: 3, sub: 0, line: 1124 } |  |  | 0.715 |
| ns | 7292 |  | 285 | struct coroutine: identity, stack, arguments, scheduling flags | 4.4 |  | 0.699 |
| walker |  | 7539 | 260 | Code::CodeKey { rung: Names, file: neco.c, decl: 0, sub: 1, line: 0 } |  |  | 0.699 |
| walker |  | 7554 | 15 | Code::CodeKey { rung: Decl, file: neco.c, decl: 16, sub: 0, line: 1251 } |  |  | 0.699 |
| walker |  | 7573 | 19 | Code::CodeKey { rung: Decl, file: neco.c, decl: 23, sub: 0, line: 1348 } |  |  | 0.699 |
| ns | 7585 |  | 293 | What neco_chan and neco_gen actually are | 4.5 |  | 0.686 |
| walker |  | 7594 | 21 | Code::CodeKey { rung: Decl, file: neco.c, decl: 15, sub: 0, line: 1204 } |  |  | 0.686 |
| walker |  | 7605 | 11 | Code::CodeKey { rung: Doc, file: neco.c, decl: 17, sub: 0, line: 1282 } |  |  | 0.686 |
| walker |  | 7616 | 11 | Code::CodeKey { rung: Doc, file: neco.c, decl: 18, sub: 0, line: 1293 } |  |  | 0.686 |
| walker |  | 7715 | 99 | Code::CodeKey { rung: Decl, file: neco.c, decl: 24, sub: 0, line: 1364 } |  |  | 0.686 |
| ns | 7769 |  | 184 | Where the event queue backend is chosen | 4.6 |  | 0.676 |
| walker |  | 7838 | 123 | Code::CodeKey { rung: Decl, file: neco.c, decl: 22, sub: 0, line: 1330 } |  |  | 0.676 |
| walker |  | 7865 | 27 | Code::CodeKey { rung: Doc, file: neco.c, decl: 19, sub: 0, line: 1312 } |  |  | 0.676 |
| walker |  | 7905 | 40 | Code::CodeKey { rung: Doc, file: neco.c, decl: 21, sub: 0, line: 1320 } |  |  | 0.676 |
| ns | 7977 |  | 208 | Test suite knobs: compilers, sanitizers, valgrind | 5.1 |  | 0.668 |
| walker |  | 8152 | 247 | Code::CodeKey { rung: Names, file: neco.c, decl: 0, sub: 2, line: 0 } |  |  | 0.668 |
| walker |  | 8178 | 26 | Code::CodeKey { rung: Decl, file: neco.c, decl: 26, sub: 0, line: 1814 } |  |  | 0.668 |
| walker |  | 8206 | 28 | Code::CodeKey { rung: Decl, file: neco.c, decl: 31, sub: 0, line: 1880 } |  |  | 0.668 |
| walker |  | 8234 | 28 | Code::CodeKey { rung: Decl, file: neco.c, decl: 34, sub: 0, line: 1899 } |  |  | 0.668 |
| ns | 8258 |  | 281 | The private, undocumented functions the tests may call | 5.2 |  | 0.656 |
| walker |  | 8263 | 29 | Code::CodeKey { rung: Decl, file: neco.c, decl: 30, sub: 0, line: 1875 } |  |  | 0.656 |
| ns | 8332 |  | 74 | Every function-like macro in tests/tests.h | 5.3 |  | 0.651 |
| walker |  | 8410 | 147 | Code::CodeKey { rung: Decl, file: neco.c, decl: 27, sub: 0, line: 1819 } |  |  | 0.651 |
| walker |  | 8452 | 42 | Code::CodeKey { rung: Doc, file: neco.c, decl: 29, sub: 0, line: 1854 } |  |  | 0.651 |
| ns | 8595 |  | 263 | How run.sh compiles and runs each test | 5.4 |  | 0.639 |
| walker |  | 8698 | 246 | Code::CodeKey { rung: Names, file: neco.c, decl: 0, sub: 3, line: 0 } |  |  | 0.639 |
| walker |  | 8715 | 17 | Code::CodeKey { rung: Doc, file: neco.c, decl: 52, sub: 0, line: 1944 } |  |  | 0.639 |
| ns | 8792 |  | 197 | The four NECO_TESTING-only shim headers | 5.5 |  | 0.630 |
| walker |  | 9010 | 295 | Code::CodeKey { rung: Names, file: neco.c, decl: 0, sub: 4, line: 0 } |  |  | 0.630 |
| ns | 9091 |  | 299 | deps/sco.h: the scheduler contract | 6.1 |  | 0.616 |
| walker |  | 9299 | 289 | Code::CodeKey { rung: Names, file: neco.c, decl: 0, sub: 5, line: 0 } |  |  | 0.616 |
| walker |  | 9341 | 42 | Code::CodeKey { rung: Decl, file: neco.c, decl: 78, sub: 0, line: 2261 } |  |  | 0.616 |
| ns | 9395 |  | 304 | deps/stack.h: the coroutine stack allocator | 6.2 |  | 0.606 |
| walker |  | 9404 | 63 | Code::CodeKey { rung: Decl, file: neco.c, decl: 74, sub: 0, line: 2165 } |  |  | 0.606 |
| walker |  | 9421 | 17 | Code::CodeKey { rung: Doc, file: neco.c, decl: 76, sub: 0, line: 2192 } |  |  | 0.606 |
| walker |  | 9442 | 21 | Code::CodeKey { rung: Doc, file: neco.c, decl: 72, sub: 0, line: 2154 } |  |  | 0.606 |
| ns | 9563 |  | 168 | deps/worker.h: the background thread pool | 6.3 |  | 0.599 |
| walker |  | 9584 | 142 | Code::CodeKey { rung: Decl, file: neco.c, decl: 77, sub: 0, line: 2247 } |  |  | 0.599 |
| ns | 9646 |  | 83 | deps/embed.sh: how neco.c is regenerated | 6.4 |  | 0.596 |
| walker |  | 9773 | 189 | Code::CodeKey { rung: Decl, file: neco.c, decl: 79, sub: 0, line: 2267 } |  |  | 0.596 |
| ns | 9827 |  | 181 | examples/select.c: the multi-channel select pattern | 6.5 |  | 0.589 |
| ns | 9939 |  | 112 | CI | 6.6 |  | 0.584 |
| ns | 9984 |  | 45 | License | 6.7 |  | 0.584 |
