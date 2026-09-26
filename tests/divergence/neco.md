Score(3000)=0.786 I=0.909 C=0.679 ns_rows≤3K=17/53 grid(1000/1442/2080/3000/4327/6240/9000)=0.723/0.803/0.769/0.786/0.822/0.768/0.633

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 28 | 28 | Fs::DirListing { dir: . } |  |  | 0.000 |
| ns | 47 |  | 47 | What Neco is | 1.1 |  | 0.000 |
| walker |  | 53 | 25 | Fs::DirListing { dir: docs } |  |  | 0.000 |
| ns | 121 |  | 74 | How the library is consumed and built | 1.2 |  | 0.000 |
| ns | 149 |  | 28 | Complete root listing | 1.3 |  | 0.292 |
| ns | 242 |  | 93 | examples/ and deps/ listings | 1.4 |  | 0.195 |
| ns | 337 |  | 95 | docs/ and .github/ trees, complete to the leaves | 1.5 |  | 0.166 |
| ns | 463 |  | 126 | Complete tests/ listing | 1.6 |  | 0.135 |
| walker |  | 480 | 427 | Markdown::ReadmeHeadline { file: README.md } |  |  | 0.315 |
| walker |  | 501 | 21 | Fs::DirListing { dir: docs/assets } |  |  | 0.346 |
| walker |  | 516 | 15 | Fs::DirListing { dir: .github } |  |  | 0.378 |
| walker |  | 520 | 4 | Fs::DirListing { dir: .github/workflows } |  |  | 0.390 |
| walker |  | 534 | 14 | Fs::DirListing { dir: docs/tools } |  |  | 0.430 |
| walker |  | 550 | 16 | Fs::DirListing { dir: docs/tools/doxygen-md } |  |  | 0.491 |
| walker |  | 592 | 42 | Fs::DirListing { dir: deps } |  |  | 0.532 |
| ns | 595 |  | 132 | Running the tests, and where the examples live | 1.7 |  | 0.494 |
| walker |  | 643 | 51 | Fs::DirListing { dir: examples } |  |  | 0.630 |
| walker |  | 769 | 126 | Fs::DirListing { dir: tests } |  |  | 0.793 |
| ns | 864 |  | 269 | Complete roster of neco.h's API groups | 1.8 |  | 0.721 |
| walker |  | 1002 | 233 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.723 |
| walker |  | 1046 | 44 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: true } |  |  | 0.723 |
| ns | 1080 |  | 216 | Basic operations: every coroutine-lifecycle signature | 1.9 |  | 0.677 |
| walker |  | 1109 | 63 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.780 |
| walker |  | 1277 | 168 | Code::CodeKey { rung: Names, file: neco.h, decl: 0, sub: 0, line: 0 } |  |  | 0.819 |
| ns | 1342 |  | 262 | Neco error codes, part 1 | 1.10 |  | 0.776 |
| walker |  | 1446 | 169 | Code::CodeKey { rung: Names, file: neco.h, decl: 0, sub: 1, line: 0 } |  |  | 0.803 |
| walker |  | 1602 | 156 | Code::CodeKey { rung: Names, file: neco.h, decl: 0, sub: 2, line: 0 } |  |  | 0.807 |
| ns | 1606 |  | 264 | Neco error codes, part 2, and the error functions | 1.11 | 1.10 | 0.771 |
| walker |  | 1785 | 183 | Code::CodeKey { rung: Names, file: neco.h, decl: 0, sub: 3, line: 0 } |  |  | 0.774 |
| walker |  | 1956 | 171 | Code::CodeKey { rung: Names, file: neco.h, decl: 0, sub: 4, line: 0 } |  |  | 0.777 |
| ns | 2030 |  | 424 | Channels: doc and complete signature set | 2.1 |  | 0.766 |
| walker |  | 2121 | 165 | Code::CodeKey { rung: Names, file: neco.h, decl: 0, sub: 5, line: 0 } |  |  | 0.769 |
| ns | 2248 |  | 218 | Generators: complete signature set | 2.2 |  | 0.777 |
| walker |  | 2284 | 163 | Code::CodeKey { rung: Names, file: neco.h, decl: 0, sub: 6, line: 0 } |  |  | 0.779 |
| ns | 2377 |  | 129 | Time: the duration constants and neco_now() | 2.3 |  | 0.763 |
| walker |  | 2453 | 169 | Code::CodeKey { rung: Names, file: neco.h, decl: 0, sub: 7, line: 0 } |  |  | 0.766 |
| walker |  | 2465 | 12 | Code::CodeKey { rung: Doc, file: neco.h, decl: 67, sub: 0, line: 169 } |  |  | 0.766 |
| ns | 2556 |  | 179 | Mutexes: type, static initializer, and all eight operations | 2.4 |  | 0.773 |
| walker |  | 2622 | 157 | Code::CodeKey { rung: Names, file: neco.h, decl: 0, sub: 8, line: 0 } |  |  | 0.775 |
| walker |  | 2641 | 19 | Code::CodeKey { rung: Decl, file: neco.h, decl: 75, sub: 0, line: 177 } |  |  | 0.775 |
| ns | 2704 |  | 148 | WaitGroups: type, initializer and five operations | 2.5 |  | 0.780 |
| walker |  | 2820 | 179 | Code::CodeKey { rung: Names, file: neco.h, decl: 0, sub: 9, line: 0 } |  |  | 0.782 |
| walker |  | 2844 | 24 | Code::CodeKey { rung: Decl, file: neco.h, decl: 76, sub: 0, line: 179 } |  |  | 0.783 |
| ns | 2846 |  | 142 | Condition variables: type, initializer and five operations | 2.6 |  | 0.784 |
| walker |  | 2859 | 15 | Code::CodeKey { rung: Doc, file: neco.h, decl: 78, sub: 0, line: 191 } |  |  | 0.785 |
| walker |  | 2875 | 16 | Code::CodeKey { rung: Doc, file: neco.h, decl: 77, sub: 0, line: 188 } |  |  | 0.786 |
| walker |  | 3005 | 130 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.786 |
| ns | 3154 |  | 308 | Posix wrappers: the non-blocking fd operations | 2.7 |  | 0.792 |
| walker |  | 3199 | 194 | Code::CodeKey { rung: Names, file: neco.h, decl: 0, sub: 10, line: 0 } |  |  | 0.795 |
| ns | 3304 |  | 150 | File descriptor helpers: neco_setnonblock and neco_wait | 2.8 |  | 0.788 |
| walker |  | 3372 | 173 | Code::CodeKey { rung: Names, file: neco.h, decl: 0, sub: 11, line: 0 } |  |  | 0.791 |
| ns | 3394 |  | 90 | Networking utilities: neco_serve and neco_dial | 2.9 |  | 0.792 |
| walker |  | 3552 | 180 | Code::CodeKey { rung: Names, file: neco.h, decl: 0, sub: 12, line: 0 } |  |  | 0.794 |
| ns | 3602 |  | 208 | Cancelation: neco_cancel, the type/state constants, cleanup macros | 2.10 |  | 0.795 |
| walker |  | 3715 | 163 | Code::CodeKey { rung: Decl, file: neco.h, decl: 106, sub: 0, line: 287 } |  |  | 0.798 |
| ns | 3861 |  | 259 | Streams and buffered I/O: the name of every entry point | 2.11 |  | 0.766 |
| walker |  | 3899 | 184 | Code::CodeKey { rung: Names, file: neco.h, decl: 0, sub: 13, line: 0 } |  |  | 0.783 |
| ns | 3986 |  | 125 | Random number generator: CSPRNG/PRNG attributes and three functions | 2.12 |  | 0.785 |
| ns | 4062 |  | 76 | Signals: watch, wait, unwatch | 2.13 |  | 0.783 |
| walker |  | 4066 | 167 | Code::CodeKey { rung: Names, file: neco.h, decl: 0, sub: 14, line: 0 } |  |  | 0.798 |
| ns | 4105 |  | 43 | Background worker: neco_work | 2.14 |  | 0.796 |
| walker |  | 4240 | 174 | Code::CodeKey { rung: Names, file: neco.h, decl: 0, sub: 15, line: 0 } |  |  | 0.812 |
| ns | 4336 |  | 231 | neco_stats: every runtime counter, plus introspection | 2.15 |  | 0.817 |
| ns | 4419 |  | 83 | Global environment: allocator and process-wide defaults | 2.16 |  | 0.818 |
| walker |  | 4423 | 183 | Code::CodeKey { rung: Names, file: neco.h, decl: 0, sub: 16, line: 0 } |  |  | 0.839 |
| walker |  | 4597 | 174 | Code::CodeKey { rung: Names, file: neco.h, decl: 0, sub: 17, line: 0 } |  |  | 0.845 |
| ns | 4667 |  | 248 | The neco_main macro, expanded | 2.17 |  | 0.822 |
| walker |  | 4748 | 151 | Code::CodeKey { rung: Names, file: neco.h, decl: 0, sub: 18, line: 0 } |  |  | 0.835 |
| ns | 4775 |  | 108 | neco.h tail: private entry points and the EAI_SYSTEM shim | 2.18 |  | 0.822 |
| walker |  | 4872 | 124 | Code::CodeKey { rung: Names, file: neco.h, decl: 0, sub: 19, line: 0 } |  |  | 0.831 |
| walker |  | 4879 | 7 | Code::CodeKey { rung: Decl, file: neco.h, decl: 166, sub: 0, line: 452 } |  |  | 0.832 |
| ns | 4968 |  | 193 | Deadlines and cancelation, the governing rules | 3.1 |  | 0.818 |
| walker |  | 5095 | 216 | Code::CodeKey { rung: Decl, file: neco.h, decl: 162, sub: 0, line: 427 } |  |  | 0.840 |
| walker |  | 5146 | 51 | Code::CodeKey { rung: Doc, file: neco.h, decl: 33, sub: 0, line: 90 } |  |  | 0.840 |
| walker |  | 5197 | 51 | Code::CodeKey { rung: Doc, file: neco.h, decl: 60, sub: 0, line: 149 } |  |  | 0.840 |
| ns | 5259 |  | 291 | Error semantics: which errors panic, which leak errno, and lasterr | 3.2 |  | 0.820 |
| walker |  | 5373 | 176 | Markdown::Section { file: README.md, section_index: 4, keeps_default_concavity: false } |  |  | 0.820 |
| ns | 5493 |  | 234 | Async cancelation, and turning cancelation off | 3.3 |  | 0.805 |
| walker |  | 5574 | 201 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.805 |
| walker |  | 5648 | 74 | Code::CodeKey { rung: Doc, file: neco.h, decl: 1, sub: 0, line: 35 } |  |  | 0.806 |
| ns | 5725 |  | 232 | Platform notes: what does not work on Windows and WebAssembly | 3.4 |  | 0.788 |
| walker |  | 5726 | 78 | Code::CodeKey { rung: Doc, file: neco.h, decl: 15, sub: 0, line: 61 } |  |  | 0.805 |
| walker |  | 5812 | 86 | Code::CodeKey { rung: Doc, file: neco.h, decl: 43, sub: 0, line: 113 } |  |  | 0.806 |
| walker |  | 6058 | 246 | Code::CodeKey { rung: Names, file: neco.c, decl: 0, sub: 0, line: 0 } |  |  | 0.806 |
| walker |  | 6063 | 5 | Code::CodeKey { rung: Decl, file: neco.c, decl: 2, sub: 0, line: 67 } |  |  | 0.806 |
| walker |  | 6068 | 5 | Code::CodeKey { rung: Decl, file: neco.c, decl: 3, sub: 0, line: 70 } |  |  | 0.806 |
| walker |  | 6073 | 5 | Code::CodeKey { rung: Decl, file: neco.c, decl: 4, sub: 0, line: 73 } |  |  | 0.806 |
| walker |  | 6078 | 5 | Code::CodeKey { rung: Decl, file: neco.c, decl: 5, sub: 0, line: 76 } |  |  | 0.806 |
| walker |  | 6083 | 5 | Code::CodeKey { rung: Decl, file: neco.c, decl: 6, sub: 0, line: 79 } |  |  | 0.806 |
| walker |  | 6088 | 5 | Code::CodeKey { rung: Decl, file: neco.c, decl: 7, sub: 0, line: 82 } |  |  | 0.806 |
| walker |  | 6095 | 7 | Code::CodeKey { rung: Decl, file: neco.c, decl: 8, sub: 0, line: 85 } |  |  | 0.806 |
| walker |  | 6107 | 12 | Code::CodeKey { rung: Decl, file: neco.c, decl: 1, sub: 0, line: 61 } |  |  | 0.806 |
| ns | 6115 |  | 390 | The scheduler, context switching, and the thread-local runtime | 3.5 |  | 0.775 |
| ns | 6230 |  | 115 | How docs/API.md is produced | 3.6 |  | 0.768 |
| walker |  | 6405 | 298 | Code::CodeKey { rung: Names, file: neco.c, decl: 0, sub: 1, line: 0 } |  |  | 0.768 |
| walker |  | 6410 | 5 | Code::CodeKey { rung: Decl, file: neco.c, decl: 9, sub: 0, line: 88 } |  |  | 0.768 |
| walker |  | 6415 | 5 | Code::CodeKey { rung: Decl, file: neco.c, decl: 17, sub: 0, line: 187 } |  |  | 0.768 |
| walker |  | 6422 | 7 | Code::CodeKey { rung: Decl, file: neco.c, decl: 10, sub: 0, line: 91 } |  |  | 0.768 |
| walker |  | 6429 | 7 | Code::CodeKey { rung: Decl, file: neco.c, decl: 18, sub: 0, line: 190 } |  |  | 0.768 |
| walker |  | 6436 | 7 | Code::CodeKey { rung: Decl, file: neco.c, decl: 24, sub: 0, line: 257 } |  |  | 0.768 |
| walker |  | 6505 | 69 | Code::CodeKey { rung: Decl, file: neco.c, decl: 15, sub: 0, line: 172 } |  |  | 0.768 |
| walker |  | 6574 | 69 | Code::CodeKey { rung: Decl, file: neco.c, decl: 22, sub: 0, line: 234 } |  |  | 0.768 |
| ns | 6616 |  | 386 | neco.c compile-time options: the complete knob list | 4.1 |  | 0.746 |
| walker |  | 6646 | 72 | Code::CodeKey { rung: Decl, file: neco.c, decl: 23, sub: 0, line: 241 } |  |  | 0.746 |
| walker |  | 6751 | 105 | Code::CodeKey { rung: Decl, file: neco.c, decl: 16, sub: 0, line: 179 } |  |  | 0.746 |
| ns | 6875 |  | 259 | Amalgamation structure and the embedded-source boundaries | 4.2 |  | 0.726 |
| walker |  | 6956 | 205 | Code::CodeKey { rung: Names, file: neco.c, decl: 0, sub: 2, line: 0 } |  |  | 0.726 |
| walker |  | 6984 | 28 | Code::CodeKey { rung: Decl, file: neco.c, decl: 33, sub: 0, line: 402 } |  |  | 0.726 |
| ns | 7007 |  | 132 | Section map of Neco's own implementation | 4.3 |  | 0.719 |
| walker |  | 7068 | 84 | Code::CodeKey { rung: Decl, file: neco.c, decl: 30, sub: 0, line: 343 } |  |  | 0.719 |
| walker |  | 7261 | 193 | Code::CodeKey { rung: Names, file: neco.c, decl: 0, sub: 3, line: 0 } |  |  | 0.719 |
| walker |  | 7289 | 28 | Code::CodeKey { rung: Decl, file: neco.c, decl: 40, sub: 0, line: 494 } |  |  | 0.719 |
| ns | 7292 |  | 285 | struct coroutine: identity, stack, arguments, scheduling flags | 4.4 |  | 0.703 |
| walker |  | 7321 | 32 | Code::CodeKey { rung: Decl, file: neco.c, decl: 44, sub: 0, line: 726 } |  |  | 0.703 |
| walker |  | 7376 | 55 | Code::CodeKey { rung: Decl, file: neco.c, decl: 37, sub: 0, line: 422 } |  |  | 0.703 |
| walker |  | 7574 | 198 | Code::CodeKey { rung: Names, file: neco.c, decl: 0, sub: 4, line: 0 } |  |  | 0.703 |
| ns | 7585 |  | 293 | What neco_chan and neco_gen actually are | 4.5 |  | 0.689 |
| walker |  | 7611 | 37 | Code::CodeKey { rung: Decl, file: neco.c, decl: 46, sub: 0, line: 767 } |  |  | 0.689 |
| walker |  | 7732 | 121 | Code::CodeKey { rung: Decl, file: neco.c, decl: 51, sub: 0, line: 806 } |  |  | 0.689 |
| ns | 7769 |  | 184 | Where the event queue backend is chosen | 4.6 |  | 0.679 |
| walker |  | 7870 | 138 | Code::CodeKey { rung: Decl, file: neco.c, decl: 50, sub: 0, line: 787 } |  |  | 0.679 |
| ns | 7977 |  | 208 | Test suite knobs: compilers, sanitizers, valgrind | 5.1 |  | 0.671 |
| walker |  | 8069 | 199 | Code::CodeKey { rung: Names, file: neco.c, decl: 0, sub: 5, line: 0 } |  |  | 0.671 |
| walker |  | 8087 | 18 | Code::CodeKey { rung: Decl, file: neco.c, decl: 53, sub: 0, line: 877 } |  |  | 0.671 |
| walker |  | 8111 | 24 | Code::CodeKey { rung: Decl, file: neco.c, decl: 55, sub: 0, line: 883 } |  |  | 0.671 |
| walker |  | 8150 | 39 | Code::CodeKey { rung: Decl, file: neco.c, decl: 56, sub: 0, line: 900 } |  |  | 0.671 |
| walker |  | 8193 | 43 | Code::CodeKey { rung: Decl, file: neco.c, decl: 59, sub: 0, line: 961 } |  |  | 0.671 |
| walker |  | 8243 | 50 | Code::CodeKey { rung: Decl, file: neco.c, decl: 54, sub: 0, line: 879 } |  |  | 0.671 |
| ns | 8258 |  | 281 | The private, undocumented functions the tests may call | 5.2 |  | 0.659 |
| walker |  | 8261 | 18 | Code::CodeKey { rung: Doc, file: neco.c, decl: 47, sub: 0, line: 782 } |  |  | 0.659 |
| walker |  | 8280 | 19 | Code::CodeKey { rung: Doc, file: neco.c, decl: 27, sub: 0, line: 338 } |  |  | 0.659 |
| walker |  | 8300 | 20 | Code::CodeKey { rung: Doc, file: neco.c, decl: 34, sub: 0, line: 417 } |  |  | 0.659 |
| walker |  | 8320 | 20 | Code::CodeKey { rung: Doc, file: neco.c, decl: 41, sub: 0, line: 721 } |  |  | 0.659 |
| ns | 8332 |  | 74 | Every function-like macro in tests/tests.h | 5.3 |  | 0.654 |
| walker |  | 8568 | 248 | Code::CodeKey { rung: Names, file: neco.c, decl: 0, sub: 6, line: 0 } |  |  | 0.654 |
| walker |  | 8574 | 6 | Code::CodeKey { rung: Decl, file: neco.c, decl: 65, sub: 0, line: 1008 } |  |  | 0.654 |
| walker |  | 8582 | 8 | Code::CodeKey { rung: Decl, file: neco.c, decl: 70, sub: 0, line: 1031 } |  |  | 0.654 |
| ns | 8595 |  | 263 | How run.sh compiles and runs each test | 5.4 |  | 0.642 |
| walker |  | 8600 | 18 | Code::CodeKey { rung: Decl, file: neco.c, decl: 66, sub: 0, line: 1011 } |  |  | 0.642 |
| walker |  | 8627 | 27 | Code::CodeKey { rung: Decl, file: neco.c, decl: 61, sub: 0, line: 989 } |  |  | 0.642 |
| ns | 8792 |  | 197 | The four NECO_TESTING-only shim headers | 5.5 |  | 0.633 |
| walker |  | 8825 | 198 | Code::CodeKey { rung: Names, file: neco.c, decl: 0, sub: 7, line: 0 } |  |  | 0.633 |
| walker |  | 8833 | 8 | Code::CodeKey { rung: Decl, file: neco.c, decl: 75, sub: 0, line: 1046 } |  |  | 0.633 |
| walker |  | 8847 | 14 | Code::CodeKey { rung: Decl, file: neco.c, decl: 71, sub: 0, line: 1035 } |  |  | 0.633 |
| walker |  | 9052 | 205 | Code::CodeKey { rung: Names, file: neco.c, decl: 0, sub: 8, line: 0 } |  |  | 0.633 |
| walker |  | 9060 | 8 | Code::CodeKey { rung: Decl, file: neco.c, decl: 82, sub: 0, line: 1085 } |  |  | 0.633 |
| walker |  | 9068 | 8 | Code::CodeKey { rung: Decl, file: neco.c, decl: 87, sub: 0, line: 1145 } |  |  | 0.633 |
| ns | 9091 |  | 299 | deps/sco.h: the scheduler contract | 6.1 |  | 0.619 |
| walker |  | 9101 | 33 | Code::CodeKey { rung: Decl, file: neco.c, decl: 84, sub: 0, line: 1092 } |  |  | 0.619 |
| walker |  | 9304 | 203 | Code::CodeKey { rung: Decl, file: neco.c, decl: 85, sub: 0, line: 1124 } |  |  | 0.619 |
| ns | 9395 |  | 304 | deps/stack.h: the coroutine stack allocator | 6.2 |  | 0.609 |
| walker |  | 9507 | 203 | Code::CodeKey { rung: Names, file: neco.c, decl: 0, sub: 9, line: 0 } |  |  | 0.609 |
| ns | 9563 |  | 168 | deps/worker.h: the background thread pool | 6.3 |  | 0.602 |
| ns | 9646 |  | 83 | deps/embed.sh: how neco.c is regenerated | 6.4 |  | 0.599 |
| walker |  | 9732 | 225 | Code::CodeKey { rung: Names, file: neco.c, decl: 0, sub: 10, line: 0 } |  |  | 0.599 |
| walker |  | 9740 | 8 | Code::CodeKey { rung: Decl, file: neco.c, decl: 107, sub: 0, line: 1344 } |  |  | 0.599 |
| walker |  | 9755 | 15 | Code::CodeKey { rung: Decl, file: neco.c, decl: 100, sub: 0, line: 1251 } |  |  | 0.599 |
| walker |  | 9776 | 21 | Code::CodeKey { rung: Decl, file: neco.c, decl: 99, sub: 0, line: 1204 } |  |  | 0.599 |
| ns | 9827 |  | 181 | examples/select.c: the multi-channel select pattern | 6.5 |  | 0.592 |
| walker |  | 9867 | 91 | Code::CodeKey { rung: Decl, file: neco.c, decl: 106, sub: 0, line: 1337 } |  |  | 0.592 |
| walker |  | 9878 | 11 | Code::CodeKey { rung: Doc, file: neco.c, decl: 101, sub: 0, line: 1282 } |  |  | 0.592 |
| walker |  | 9889 | 11 | Code::CodeKey { rung: Doc, file: neco.c, decl: 102, sub: 0, line: 1293 } |  |  | 0.592 |
| ns | 9939 |  | 112 | CI | 6.6 |  | 0.587 |
| ns | 9984 |  | 45 | License | 6.7 |  | 0.585 |
| walker |  | 9995 | 106 | Code::CodeKey { rung: Names, file: neco.c, decl: 0, sub: 11, line: 0 } |  |  | 0.585 |
