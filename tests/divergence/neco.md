Score(3000)=0.597 I=0.834 C=0.427 ns_rows≤3K=17/53 grid(1000/1442/2080/3000/4327/6240/9000)=0.681/0.700/0.674/0.597/0.538/0.518/0.473

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
| walker |  | 2656 | 109 | Markdown::Section { file: docs/assets/API_head.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.622 |
| ns | 2704 |  | 148 | WaitGroups: type, initializer and five operations | 2.5 |  | 0.609 |
| walker |  | 2728 | 72 | Markdown::Section { file: docs/API.md, section_index: 31, keeps_default_concavity: false } |  |  | 0.609 |
| walker |  | 2809 | 81 | Markdown::Section { file: docs/API.md, section_index: 112, keeps_default_concavity: false } |  |  | 0.609 |
| ns | 2846 |  | 142 | Condition variables: type, initializer and five operations | 2.6 |  | 0.597 |
| walker |  | 3033 | 224 | Markdown::Section { file: README.md, section_index: 15, keeps_default_concavity: false } |  |  | 0.598 |
| ns | 3154 |  | 308 | Posix wrappers: the non-blocking fd operations | 2.7 |  | 0.579 |
| ns | 3304 |  | 150 | File descriptor helpers: neco_setnonblock and neco_wait | 2.8 |  | 0.563 |
| ns | 3394 |  | 90 | Networking utilities: neco_serve and neco_dial | 2.9 |  | 0.558 |
| ns | 3602 |  | 208 | Cancelation: neco_cancel, the type/state constants, cleanup macros | 2.10 |  | 0.541 |
| walker |  | 3770 | 737 | Markdown::Section { file: README.md, section_index: 16, keeps_default_concavity: true } |  |  | 0.543 |
| ns | 3861 |  | 259 | Streams and buffered I/O: the name of every entry point | 2.11 |  | 0.521 |
| walker |  | 3973 | 203 | Code::CodeKey { rung: Names, file: neco.h, decl: 0, sub: 1, line: 0 } |  |  | 0.535 |
| ns | 3986 |  | 125 | Random number generator: CSPRNG/PRNG attributes and three functions | 2.12 |  | 0.529 |
| walker |  | 4051 | 78 | Code::CodeKey { rung: Doc, file: neco.h, decl: 15, sub: 0, line: 61 } |  |  | 0.545 |
| ns | 4062 |  | 76 | Signals: watch, wait, unwatch | 2.13 |  | 0.540 |
| ns | 4105 |  | 43 | Background worker: neco_work | 2.14 |  | 0.538 |
| walker |  | 4134 | 83 | Markdown::Section { file: docs/API.md, section_index: 127, keeps_default_concavity: false } |  |  | 0.538 |
| walker |  | 4220 | 86 | Markdown::Section { file: docs/API.md, section_index: 75, keeps_default_concavity: false } |  |  | 0.538 |
| walker |  | 4304 | 84 | Markdown::Section { file: docs/API.md, section_index: 76, keeps_default_concavity: false } |  |  | 0.538 |
| ns | 4336 |  | 231 | neco_stats: every runtime counter, plus introspection | 2.15 |  | 0.521 |
| walker |  | 4391 | 87 | Markdown::Section { file: docs/API.md, section_index: 79, keeps_default_concavity: false } |  |  | 0.521 |
| ns | 4419 |  | 83 | Global environment: allocator and process-wide defaults | 2.16 |  | 0.518 |
| walker |  | 4478 | 87 | Markdown::Section { file: docs/API.md, section_index: 113, keeps_default_concavity: false } |  |  | 0.518 |
| walker |  | 4565 | 87 | Markdown::Section { file: docs/API.md, section_index: 115, keeps_default_concavity: false } |  |  | 0.518 |
| walker |  | 4652 | 87 | Markdown::Section { file: docs/API.md, section_index: 114, keeps_default_concavity: false } |  |  | 0.518 |
| ns | 4667 |  | 248 | The neco_main macro, expanded | 2.17 |  | 0.503 |
| walker |  | 4740 | 88 | Markdown::Section { file: docs/API.md, section_index: 28, keeps_default_concavity: false } |  |  | 0.503 |
| ns | 4775 |  | 108 | neco.h tail: private entry points and the EAI_SYSTEM shim | 2.18 |  | 0.495 |
| walker |  | 4828 | 88 | Markdown::Section { file: docs/API.md, section_index: 78, keeps_default_concavity: false } |  |  | 0.495 |
| walker |  | 4916 | 88 | Markdown::Section { file: docs/API.md, section_index: 77, keeps_default_concavity: false } |  |  | 0.495 |
| ns | 4968 |  | 193 | Deadlines and cancelation, the governing rules | 3.1 |  | 0.486 |
| walker |  | 5004 | 88 | Markdown::Section { file: docs/API.md, section_index: 88, keeps_default_concavity: false } |  |  | 0.486 |
| walker |  | 5091 | 87 | Markdown::Section { file: docs/API.md, section_index: 89, keeps_default_concavity: false } |  |  | 0.486 |
| walker |  | 5178 | 87 | Markdown::Section { file: docs/API.md, section_index: 90, keeps_default_concavity: false } |  |  | 0.486 |
| ns | 5259 |  | 291 | Error semantics: which errors panic, which leak errno, and lasterr | 3.2 |  | 0.474 |
| walker |  | 5473 | 295 | Markdown::Section { file: README.md, section_index: 5, keeps_default_concavity: false } |  |  | 0.474 |
| walker |  | 5489 | 16 | Fs::DirListing { dir: docs/tools/doxygen-md } |  |  | 0.496 |
| ns | 5493 |  | 234 | Async cancelation, and turning cancelation off | 3.3 |  | 0.487 |
| walker |  | 5578 | 89 | Markdown::Section { file: docs/API.md, section_index: 80, keeps_default_concavity: false } |  |  | 0.487 |
| walker |  | 5667 | 89 | Markdown::Section { file: docs/API.md, section_index: 86, keeps_default_concavity: false } |  |  | 0.487 |
| ns | 5725 |  | 232 | Platform notes: what does not work on Windows and WebAssembly | 3.4 |  | 0.477 |
| walker |  | 5755 | 88 | Markdown::Section { file: docs/API.md, section_index: 85, keeps_default_concavity: false } |  |  | 0.477 |
| walker |  | 5845 | 90 | Markdown::Section { file: docs/API.md, section_index: 137, keeps_default_concavity: false } |  |  | 0.477 |
| walker |  | 5934 | 89 | Markdown::Section { file: docs/API.md, section_index: 138, keeps_default_concavity: false } |  |  | 0.477 |
| walker |  | 6025 | 91 | Markdown::Section { file: docs/API.md, section_index: 152, keeps_default_concavity: false } |  |  | 0.477 |
| walker |  | 6114 | 89 | Markdown::Section { file: docs/API.md, section_index: 153, keeps_default_concavity: false } |  |  | 0.477 |
| ns | 6115 |  | 390 | The scheduler, context switching, and the thread-local runtime | 3.5 |  | 0.511 |
| walker |  | 6206 | 92 | Markdown::Section { file: docs/API.md, section_index: 91, keeps_default_concavity: false } |  |  | 0.511 |
| ns | 6230 |  | 115 | How docs/API.md is produced | 3.6 |  | 0.518 |
| walker |  | 6299 | 93 | Markdown::Section { file: docs/API.md, section_index: 82, keeps_default_concavity: false } |  |  | 0.518 |
| walker |  | 6391 | 92 | Markdown::Section { file: docs/API.md, section_index: 83, keeps_default_concavity: false } |  |  | 0.518 |
| walker |  | 6482 | 91 | Markdown::Section { file: docs/API.md, section_index: 84, keeps_default_concavity: false } |  |  | 0.518 |
| walker |  | 6577 | 95 | Markdown::Section { file: docs/API.md, section_index: 81, keeps_default_concavity: false } |  |  | 0.518 |
| ns | 6616 |  | 386 | neco.c compile-time options: the complete knob list | 4.1 |  | 0.504 |
| walker |  | 6672 | 95 | Markdown::Section { file: docs/API.md, section_index: 87, keeps_default_concavity: false } |  |  | 0.504 |
| walker |  | 6770 | 98 | Markdown::Section { file: docs/API.md, section_index: 92, keeps_default_concavity: false } |  |  | 0.504 |
| walker |  | 6868 | 98 | Markdown::Section { file: docs/API.md, section_index: 143, keeps_default_concavity: false } |  |  | 0.504 |
| ns | 6875 |  | 259 | Amalgamation structure and the embedded-source boundaries | 4.2 |  | 0.490 |
| walker |  | 6967 | 99 | Markdown::Section { file: docs/API.md, section_index: 27, keeps_default_concavity: false } |  |  | 0.490 |
| ns | 7007 |  | 132 | Section map of Neco's own implementation | 4.3 |  | 0.485 |
| walker |  | 7068 | 101 | Markdown::Section { file: docs/API.md, section_index: 141, keeps_default_concavity: false } |  |  | 0.485 |
| walker |  | 7167 | 99 | Markdown::Section { file: docs/API.md, section_index: 142, keeps_default_concavity: false } |  |  | 0.485 |
| walker |  | 7268 | 101 | Markdown::Section { file: docs/API.md, section_index: 145, keeps_default_concavity: false } |  |  | 0.485 |
| ns | 7292 |  | 285 | struct coroutine: identity, stack, arguments, scheduling flags | 4.4 |  | 0.474 |
| walker |  | 7372 | 104 | Markdown::Section { file: docs/API.md, section_index: 139, keeps_default_concavity: false } |  |  | 0.474 |
| walker |  | 7477 | 105 | Markdown::Section { file: docs/API.md, section_index: 144, keeps_default_concavity: false } |  |  | 0.474 |
| walker |  | 7585 | 108 | Markdown::Section { file: docs/API.md, section_index: 19, keeps_default_concavity: false } |  |  | 0.465 |
| ns | 7585 |  | 293 | What neco_chan and neco_gen actually are | 4.5 |  | 0.465 |
| walker |  | 7693 | 108 | Markdown::Section { file: docs/API.md, section_index: 26, keeps_default_concavity: false } |  |  | 0.465 |
| ns | 7769 |  | 184 | Where the event queue backend is chosen | 4.6 |  | 0.458 |
| walker |  | 7801 | 108 | Markdown::Section { file: docs/API.md, section_index: 146, keeps_default_concavity: false } |  |  | 0.458 |
| walker |  | 7909 | 108 | Markdown::Section { file: docs/API.md, section_index: 150, keeps_default_concavity: false } |  |  | 0.458 |
| ns | 7977 |  | 208 | Test suite knobs: compilers, sanitizers, valgrind | 5.1 |  | 0.453 |
| walker |  | 8211 | 302 | Markdown::Section { file: README.md, section_index: 14, keeps_default_concavity: false } |  |  | 0.477 |
| ns | 8258 |  | 281 | The private, undocumented functions the tests may call | 5.2 |  | 0.468 |
| walker |  | 8324 | 113 | Markdown::Section { file: docs/API.md, section_index: 149, keeps_default_concavity: false } |  |  | 0.468 |
| ns | 8332 |  | 74 | Every function-like macro in tests/tests.h | 5.3 |  | 0.465 |
| walker |  | 8440 | 116 | Markdown::Section { file: docs/API.md, section_index: 136, keeps_default_concavity: false } |  |  | 0.465 |
| ns | 8595 |  | 263 | How run.sh compiles and runs each test | 5.4 |  | 0.456 |
| walker |  | 8654 | 214 | Code::CodeKey { rung: Names, file: neco.h, decl: 0, sub: 2, line: 0 } |  |  | 0.479 |
| walker |  | 8705 | 51 | Code::CodeKey { rung: Doc, file: neco.h, decl: 33, sub: 0, line: 90 } |  |  | 0.479 |
| ns | 8792 |  | 197 | The four NECO_TESTING-only shim headers | 5.5 |  | 0.472 |
| walker |  | 8822 | 117 | Markdown::Section { file: docs/API.md, section_index: 147, keeps_default_concavity: false } |  |  | 0.472 |
| walker |  | 8965 | 143 | Plaintext::Whole { file: .github/workflows/main.yml } |  |  | 0.473 |
| ns | 9091 |  | 299 | deps/sco.h: the scheduler contract | 6.1 |  | 0.462 |
| walker |  | 9326 | 361 | Markdown::Section { file: README.md, section_index: 8, keeps_default_concavity: false } |  |  | 0.462 |
| ns | 9395 |  | 304 | deps/stack.h: the coroutine stack allocator | 6.2 |  | 0.455 |
| walker |  | 9452 | 126 | Markdown::Section { file: docs/API.md, section_index: 140, keeps_default_concavity: false } |  |  | 0.455 |
| ns | 9563 |  | 168 | deps/worker.h: the background thread pool | 6.3 |  | 0.450 |
| walker |  | 9583 | 131 | Markdown::Section { file: docs/API.md, section_index: 36, keeps_default_concavity: false } |  |  | 0.450 |
| ns | 9646 |  | 83 | deps/embed.sh: how neco.c is regenerated | 6.4 |  | 0.448 |
| walker |  | 9716 | 133 | Markdown::Section { file: docs/API.md, section_index: 134, keeps_default_concavity: false } |  |  | 0.448 |
| ns | 9827 |  | 181 | examples/select.c: the multi-channel select pattern | 6.5 |  | 0.443 |
| walker |  | 9850 | 134 | Markdown::Section { file: docs/API.md, section_index: 40, keeps_default_concavity: false } |  |  | 0.443 |
| walker |  | 9904 | 54 | Markdown::ReadmeHeadline { file: tests/README.md } |  |  | 0.446 |
| ns | 9939 |  | 112 | CI | 6.6 |  | 0.455 |
| ns | 9984 |  | 45 | License | 6.7 |  | 0.456 |
