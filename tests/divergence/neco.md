Score(3000)=0.632 I=0.844 C=0.474 ns_rows≤3K=17/53 grid(1000/1442/2080/3000/4327/6240/9000)=0.681/0.700/0.674/0.632/0.678/0.729/0.663

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
| walker |  | 1226 | 32 | Markdown::Section { file: docs/assets/API_foot.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.739 |
| ns | 1342 |  | 262 | Neco error codes, part 1 | 1.10 |  | 0.700 |
| walker |  | 1356 | 130 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.700 |
| walker |  | 1574 | 218 | Code::CodeKey { rung: Names, file: neco.h, decl: 0, sub: 0, line: 0 } |  |  | 0.765 |
| ns | 1606 |  | 264 | Neco error codes, part 2, and the error functions | 1.11 | 1.10 | 0.731 |
| walker |  | 1648 | 74 | Code::CodeKey { rung: Doc, file: neco.h, decl: 1, sub: 0, line: 35 } |  |  | 0.731 |
| walker |  | 1759 | 111 | Markdown::Section { file: docs/README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.732 |
| walker |  | 1884 | 125 | Markdown::Section { file: docs/API.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.732 |
| ns | 2030 |  | 424 | Channels: doc and complete signature set | 2.1 |  | 0.674 |
| walker |  | 2060 | 176 | Markdown::Section { file: README.md, section_index: 4, keeps_default_concavity: false } |  |  | 0.674 |
| ns | 2248 |  | 218 | Generators: complete signature set | 2.2 |  | 0.654 |
| walker |  | 2261 | 201 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.654 |
| walker |  | 2315 | 54 | Markdown::Section { file: docs/API.md, section_index: 29, keeps_default_concavity: false } |  |  | 0.654 |
| walker |  | 2367 | 52 | Markdown::Section { file: docs/API.md, section_index: 30, keeps_default_concavity: false } |  |  | 0.654 |
| ns | 2377 |  | 129 | Time: the duration constants and neco_now() | 2.3 |  | 0.640 |
| walker |  | 2547 | 180 | Markdown::Section { file: docs/TECHNICAL.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.640 |
| ns | 2556 |  | 179 | Mutexes: type, static initializer, and all eight operations | 2.4 |  | 0.622 |
| ns | 2704 |  | 148 | WaitGroups: type, initializer and five operations | 2.5 |  | 0.609 |
| walker |  | 2750 | 203 | Code::CodeKey { rung: Names, file: neco.h, decl: 0, sub: 1, line: 0 } |  |  | 0.626 |
| walker |  | 2828 | 78 | Code::CodeKey { rung: Doc, file: neco.h, decl: 15, sub: 0, line: 61 } |  |  | 0.646 |
| ns | 2846 |  | 142 | Condition variables: type, initializer and five operations | 2.6 |  | 0.632 |
| walker |  | 3042 | 214 | Code::CodeKey { rung: Names, file: neco.h, decl: 0, sub: 2, line: 0 } |  |  | 0.678 |
| walker |  | 3093 | 51 | Code::CodeKey { rung: Doc, file: neco.h, decl: 33, sub: 0, line: 90 } |  |  | 0.679 |
| ns | 3154 |  | 308 | Posix wrappers: the non-blocking fd operations | 2.7 |  | 0.657 |
| walker |  | 3285 | 192 | Code::CodeKey { rung: Names, file: neco.h, decl: 0, sub: 3, line: 0 } |  |  | 0.686 |
| ns | 3304 |  | 150 | File descriptor helpers: neco_setnonblock and neco_wait | 2.8 |  | 0.666 |
| walker |  | 3371 | 86 | Code::CodeKey { rung: Doc, file: neco.h, decl: 43, sub: 0, line: 113 } |  |  | 0.668 |
| ns | 3394 |  | 90 | Networking utilities: neco_serve and neco_dial | 2.9 |  | 0.661 |
| walker |  | 3595 | 224 | Code::CodeKey { rung: Names, file: neco.h, decl: 0, sub: 4, line: 0 } |  |  | 0.693 |
| ns | 3602 |  | 208 | Cancelation: neco_cancel, the type/state constants, cleanup macros | 2.10 |  | 0.672 |
| walker |  | 3722 | 127 | Code::CodeKey { rung: Doc, file: neco.h, decl: 53, sub: 0, line: 135 } |  |  | 0.673 |
| ns | 3861 |  | 259 | Streams and buffered I/O: the name of every entry point | 2.11 |  | 0.646 |
| walker |  | 3945 | 223 | Code::CodeKey { rung: Names, file: neco.h, decl: 0, sub: 5, line: 0 } |  |  | 0.682 |
| ns | 3986 |  | 125 | Random number generator: CSPRNG/PRNG attributes and three functions | 2.12 |  | 0.674 |
| walker |  | 3996 | 51 | Code::CodeKey { rung: Doc, file: neco.h, decl: 60, sub: 0, line: 149 } |  |  | 0.676 |
| ns | 4062 |  | 76 | Signals: watch, wait, unwatch | 2.13 |  | 0.669 |
| ns | 4105 |  | 43 | Background worker: neco_work | 2.14 |  | 0.667 |
| walker |  | 4178 | 182 | Code::CodeKey { rung: Names, file: neco.h, decl: 0, sub: 6, line: 0 } |  |  | 0.676 |
| walker |  | 4190 | 12 | Code::CodeKey { rung: Doc, file: neco.h, decl: 67, sub: 0, line: 169 } |  |  | 0.678 |
| ns | 4336 |  | 231 | neco_stats: every runtime counter, plus introspection | 2.15 |  | 0.657 |
| walker |  | 4402 | 212 | Code::CodeKey { rung: Names, file: neco.h, decl: 0, sub: 7, line: 0 } |  |  | 0.676 |
| walker |  | 4419 | 17 | Code::CodeKey { rung: Decl, file: neco.h, decl: 75, sub: 0, line: 177 } |  |  | 0.675 |
| ns | 4419 |  | 83 | Global environment: allocator and process-wide defaults | 2.16 |  | 0.675 |
| walker |  | 4443 | 24 | Code::CodeKey { rung: Decl, file: neco.h, decl: 76, sub: 0, line: 179 } |  |  | 0.679 |
| walker |  | 4458 | 15 | Code::CodeKey { rung: Doc, file: neco.h, decl: 78, sub: 0, line: 191 } |  |  | 0.683 |
| walker |  | 4474 | 16 | Code::CodeKey { rung: Doc, file: neco.h, decl: 77, sub: 0, line: 188 } |  |  | 0.687 |
| ns | 4667 |  | 248 | The neco_main macro, expanded | 2.17 |  | 0.669 |
| walker |  | 4710 | 236 | Code::CodeKey { rung: Names, file: neco.h, decl: 0, sub: 8, line: 0 } |  |  | 0.699 |
| walker |  | 4726 | 16 | Code::CodeKey { rung: Decl, file: neco.h, decl: 94, sub: 0, line: 231 } |  |  | 0.704 |
| ns | 4775 |  | 108 | neco.h tail: private entry points and the EAI_SYSTEM shim | 2.18 |  | 0.692 |
| walker |  | 4933 | 207 | Code::CodeKey { rung: Names, file: neco.h, decl: 0, sub: 9, line: 0 } |  |  | 0.710 |
| ns | 4968 |  | 193 | Deadlines and cancelation, the governing rules | 3.1 |  | 0.697 |
| walker |  | 5160 | 227 | Code::CodeKey { rung: Names, file: neco.h, decl: 0, sub: 10, line: 0 } |  |  | 0.712 |
| ns | 5259 |  | 291 | Error semantics: which errors panic, which leak errno, and lasterr | 3.2 |  | 0.695 |
| walker |  | 5323 | 163 | Code::CodeKey { rung: Decl, file: neco.h, decl: 106, sub: 0, line: 287 } |  |  | 0.723 |
| ns | 5493 |  | 234 | Async cancelation, and turning cancelation off | 3.3 |  | 0.710 |
| walker |  | 5551 | 228 | Code::CodeKey { rung: Names, file: neco.h, decl: 0, sub: 11, line: 0 } |  |  | 0.729 |
| ns | 5725 |  | 232 | Platform notes: what does not work on Windows and WebAssembly | 3.4 |  | 0.713 |
| walker |  | 5744 | 193 | Code::CodeKey { rung: Names, file: neco.h, decl: 0, sub: 12, line: 0 } |  |  | 0.729 |
| walker |  | 5962 | 218 | Code::CodeKey { rung: Names, file: neco.h, decl: 0, sub: 13, line: 0 } |  |  | 0.748 |
| ns | 6115 |  | 390 | The scheduler, context switching, and the thread-local runtime | 3.5 |  | 0.719 |
| walker |  | 6161 | 199 | Code::CodeKey { rung: Names, file: neco.h, decl: 0, sub: 14, line: 0 } |  |  | 0.726 |
| ns | 6230 |  | 115 | How docs/API.md is produced | 3.6 |  | 0.729 |
| walker |  | 6352 | 191 | Code::CodeKey { rung: Names, file: neco.h, decl: 0, sub: 15, line: 0 } |  |  | 0.745 |
| walker |  | 6568 | 216 | Code::CodeKey { rung: Decl, file: neco.h, decl: 162, sub: 0, line: 427 } |  |  | 0.764 |
| ns | 6616 |  | 386 | neco.c compile-time options: the complete knob list | 4.1 |  | 0.743 |
| walker |  | 6677 | 109 | Markdown::Section { file: docs/assets/API_head.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.743 |
| walker |  | 6749 | 72 | Markdown::Section { file: docs/API.md, section_index: 31, keeps_default_concavity: false } |  |  | 0.743 |
| walker |  | 6830 | 81 | Markdown::Section { file: docs/API.md, section_index: 112, keeps_default_concavity: false } |  |  | 0.743 |
| ns | 6875 |  | 259 | Amalgamation structure and the embedded-source boundaries | 4.2 |  | 0.722 |
| ns | 7007 |  | 132 | Section map of Neco's own implementation | 4.3 |  | 0.715 |
| walker |  | 7054 | 224 | Markdown::Section { file: README.md, section_index: 15, keeps_default_concavity: false } |  |  | 0.732 |
| ns | 7292 |  | 285 | struct coroutine: identity, stack, arguments, scheduling flags | 4.4 |  | 0.715 |
| ns | 7585 |  | 293 | What neco_chan and neco_gen actually are | 4.5 |  | 0.701 |
| ns | 7769 |  | 184 | Where the event queue backend is chosen | 4.6 |  | 0.692 |
| walker |  | 7791 | 737 | Markdown::Section { file: README.md, section_index: 16, keeps_default_concavity: true } |  |  | 0.711 |
| walker |  | 7874 | 83 | Markdown::Section { file: docs/API.md, section_index: 127, keeps_default_concavity: false } |  |  | 0.711 |
| walker |  | 7960 | 86 | Markdown::Section { file: docs/API.md, section_index: 75, keeps_default_concavity: false } |  |  | 0.711 |
| ns | 7977 |  | 208 | Test suite knobs: compilers, sanitizers, valgrind | 5.1 |  | 0.702 |
| walker |  | 8044 | 84 | Markdown::Section { file: docs/API.md, section_index: 76, keeps_default_concavity: false } |  |  | 0.702 |
| walker |  | 8131 | 87 | Markdown::Section { file: docs/API.md, section_index: 79, keeps_default_concavity: false } |  |  | 0.702 |
| walker |  | 8218 | 87 | Markdown::Section { file: docs/API.md, section_index: 113, keeps_default_concavity: false } |  |  | 0.702 |
| ns | 8258 |  | 281 | The private, undocumented functions the tests may call | 5.2 |  | 0.690 |
| walker |  | 8305 | 87 | Markdown::Section { file: docs/API.md, section_index: 115, keeps_default_concavity: false } |  |  | 0.690 |
| ns | 8332 |  | 74 | Every function-like macro in tests/tests.h | 5.3 |  | 0.685 |
| walker |  | 8392 | 87 | Markdown::Section { file: docs/API.md, section_index: 114, keeps_default_concavity: false } |  |  | 0.685 |
| walker |  | 8480 | 88 | Markdown::Section { file: docs/API.md, section_index: 28, keeps_default_concavity: false } |  |  | 0.685 |
| walker |  | 8568 | 88 | Markdown::Section { file: docs/API.md, section_index: 78, keeps_default_concavity: false } |  |  | 0.685 |
| ns | 8595 |  | 263 | How run.sh compiles and runs each test | 5.4 |  | 0.673 |
| walker |  | 8656 | 88 | Markdown::Section { file: docs/API.md, section_index: 77, keeps_default_concavity: false } |  |  | 0.673 |
| walker |  | 8744 | 88 | Markdown::Section { file: docs/API.md, section_index: 88, keeps_default_concavity: false } |  |  | 0.673 |
| ns | 8792 |  | 197 | The four NECO_TESTING-only shim headers | 5.5 |  | 0.663 |
| walker |  | 8831 | 87 | Markdown::Section { file: docs/API.md, section_index: 89, keeps_default_concavity: false } |  |  | 0.663 |
| walker |  | 8918 | 87 | Markdown::Section { file: docs/API.md, section_index: 90, keeps_default_concavity: false } |  |  | 0.663 |
| ns | 9091 |  | 299 | deps/sco.h: the scheduler contract | 6.1 |  | 0.648 |
| walker |  | 9213 | 295 | Markdown::Section { file: README.md, section_index: 5, keeps_default_concavity: false } |  |  | 0.648 |
| walker |  | 9229 | 16 | Fs::DirListing { dir: docs/tools/doxygen-md } |  |  | 0.662 |
| walker |  | 9318 | 89 | Markdown::Section { file: docs/API.md, section_index: 80, keeps_default_concavity: false } |  |  | 0.662 |
| ns | 9395 |  | 304 | deps/stack.h: the coroutine stack allocator | 6.2 |  | 0.651 |
| walker |  | 9407 | 89 | Markdown::Section { file: docs/API.md, section_index: 86, keeps_default_concavity: false } |  |  | 0.651 |
| walker |  | 9495 | 88 | Markdown::Section { file: docs/API.md, section_index: 85, keeps_default_concavity: false } |  |  | 0.651 |
| ns | 9563 |  | 168 | deps/worker.h: the background thread pool | 6.3 |  | 0.644 |
| walker |  | 9585 | 90 | Markdown::Section { file: docs/API.md, section_index: 137, keeps_default_concavity: false } |  |  | 0.644 |
| ns | 9646 |  | 83 | deps/embed.sh: how neco.c is regenerated | 6.4 |  | 0.641 |
| walker |  | 9674 | 89 | Markdown::Section { file: docs/API.md, section_index: 138, keeps_default_concavity: false } |  |  | 0.641 |
| walker |  | 9765 | 91 | Markdown::Section { file: docs/API.md, section_index: 152, keeps_default_concavity: false } |  |  | 0.641 |
| ns | 9827 |  | 181 | examples/select.c: the multi-channel select pattern | 6.5 |  | 0.634 |
| walker |  | 9854 | 89 | Markdown::Section { file: docs/API.md, section_index: 153, keeps_default_concavity: false } |  |  | 0.634 |
| ns | 9939 |  | 112 | CI | 6.6 |  | 0.628 |
| walker |  | 9946 | 92 | Markdown::Section { file: docs/API.md, section_index: 91, keeps_default_concavity: false } |  |  | 0.628 |
| ns | 9984 |  | 45 | License | 6.7 |  | 0.628 |
