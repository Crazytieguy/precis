Score(3000)=0.544 I=0.766 C=0.387 ns_rows≤3K=18/50 grid(1000/1442/2080/3000/4327/6240/9000)=0.591/0.606/0.537/0.544/0.467/0.455/0.544

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| ns | 95 |  | 95 | Package identity: `tea` package doc lede + module path | 1.1 |  | 0.000 |
| ns | 163 |  | 68 | `Model` interface skeleton: `Init` / `Update` / `View` | 1.2 |  | 0.000 |
| walker |  | 202 | 202 | Fs::DirListing { dir: . } |  |  | 0.000 |
| ns | 266 |  | 103 | `Msg` alias and the `Cmd` type | 1.3 |  | 0.000 |
| walker |  | 278 | 76 | Markdown::ReadmeHeadline { file: README.md } |  |  | 0.000 |
| walker |  | 288 | 10 | Fs::DirListing { dir: testdata } |  |  | 0.000 |
| walker |  | 302 | 14 | Fs::DirListing { dir: tutorials } |  |  | 0.000 |
| walker |  | 331 | 29 | GoMod::Identity { file: go.mod } |  |  | 0.043 |
| ns | 338 |  | 72 | Program entry points: `NewProgram`, `Run`, `ProgramOption` | 1.4 |  | 0.038 |
| walker |  | 339 | 8 | Fs::DirListing { dir: tutorials/basics } |  |  | 0.038 |
| walker |  | 347 | 8 | Fs::DirListing { dir: tutorials/commands } |  |  | 0.038 |
| ns | 450 |  | 112 | `Model` method doc comments (refines 1.2) | 1.5 | 1.2 | 0.032 |
| walker |  | 476 | 129 | Plaintext::Whole { file: Taskfile.yaml } |  |  | 0.033 |
| walker |  | 498 | 22 | Fs::DirListing { dir: testdata/TestClearMsg } |  |  | 0.033 |
| walker |  | 542 | 44 | GoMod::Identity { file: tutorials/go.mod } |  |  | 0.033 |
| walker |  | 554 | 12 | Fs::DirListing { dir: .github } |  |  | 0.033 |
| walker |  | 585 | 31 | Fs::DirListing { dir: .github/workflows } |  |  | 0.033 |
| ns | 652 |  | 202 | Complete root directory listing | 1.6 |  | 0.413 |
| walker |  | 657 | 72 | Fs::DirListing { dir: testdata/TestViewModel } |  |  | 0.413 |
| walker |  | 807 | 150 | Code::CodeKey { rung: ModuleDoc, file: tea.go, decl: 0, sub: 0, line: 0 } |  |  | 0.712 |
| ns | 851 |  | 199 | `View` struct: header plus every field name | 1.7 |  | 0.620 |
| ns | 975 |  | 124 | `NewView` / `View.SetContent` constructors | 1.8 | 1.7 | 0.584 |
| walker |  | 1072 | 265 | Code::CodeKey { rung: Names, file: tea.go, decl: 0, sub: 0, line: 0 } |  |  | 0.591 |
| walker |  | 1077 | 5 | Code::CodeKey { rung: Decl, file: tea.go, decl: 13, sub: 0, line: 312 } |  |  | 0.591 |
| ns | 1132 |  | 157 | `Cmd` semantics and the `Quit` command | 1.9 | 1.3 | 0.554 |
| walker |  | 1239 | 162 | Code::CodeKey { rung: Decl, file: tea.go, decl: 5, sub: 0, line: 53 } |  |  | 0.633 |
| walker |  | 1246 | 7 | Code::CodeKey { rung: Doc, file: tea.go, decl: 13, sub: 0, line: 312 } |  |  | 0.633 |
| ns | 1338 |  | 206 | Direct dependency set from `go.mod` | 1.10 |  | 0.605 |
| walker |  | 1494 | 248 | Code::CodeKey { rung: Decl, file: tea.go, decl: 11, sub: 0, line: 286 } |  |  | 0.607 |
| ns | 1522 |  | 184 | README lede: positioning and shipped feature set | 1.11 |  | 0.589 |
| ns | 1665 |  | 143 | Sentinel errors returned by `Program.Run` | 2.1 |  | 0.574 |
| ns | 1810 |  | 145 | Every exported `*Program` method (complete roster) | 2.2 | 1.4 | 0.555 |
| walker |  | 1855 | 361 | Code::CodeKey { rung: Decl, file: tea.go, decl: 8, sub: 0, line: 241 } |  |  | 0.555 |
| walker |  | 1869 | 14 | Code::CodeKey { rung: Doc, file: tea.go, decl: 12, sub: 0, line: 309 } |  |  | 0.555 |
| ns | 2001 |  | 191 | Every `ProgramOption` constructor (complete roster) | 2.3 |  | 0.532 |
| walker |  | 2089 | 220 | Code::CodeKey { rung: Names, file: tea.go, decl: 0, sub: 1, line: 0 } |  |  | 0.544 |
| ns | 2130 |  | 129 | Build/test/lint entry points (`Taskfile.yaml`) | 2.4 |  | 0.569 |
| walker |  | 2204 | 115 | Code::CodeKey { rung: Decl, file: tea.go, decl: 15, sub: 0, line: 336 } |  |  | 0.569 |
| walker |  | 2352 | 148 | Code::CodeKey { rung: Decl, file: tea.go, decl: 17, sub: 0, line: 357 } |  |  | 0.569 |
| walker |  | 2365 | 13 | Code::CodeKey { rung: Doc, file: tea.go, decl: 28, sub: 0, line: 595 } |  |  | 0.577 |
| ns | 2436 |  | 306 | Event-loop dispatch table: every internally-handled message type | 2.5 |  | 0.534 |
| walker |  | 2510 | 145 | Code::CodeKey { rung: Names, file: tea.go, decl: 0, sub: 2, line: 0 } |  |  | 0.564 |
| walker |  | 2524 | 14 | Code::CodeKey { rung: Doc, file: tea.go, decl: 17, sub: 0, line: 357 } |  |  | 0.564 |
| walker |  | 2539 | 15 | Code::CodeKey { rung: Doc, file: tea.go, decl: 10, sub: 0, line: 284 } |  |  | 0.564 |
| ns | 2682 |  | 246 | Event-loop skeleton: translate → filter → dispatch → update → render | 2.6 | 2.5 | 0.532 |
| walker |  | 2727 | 188 | Code::CodeKey { rung: Decl, file: tea.go, decl: 20, sub: 0, line: 426 } |  |  | 0.532 |
| walker |  | 2739 | 12 | Code::CodeKey { rung: Doc, file: tea.go, decl: 20, sub: 0, line: 426 } |  |  | 0.532 |
| walker |  | 2755 | 16 | Code::CodeKey { rung: Doc, file: tea.go, decl: 5, sub: 0, line: 53 } |  |  | 0.569 |
| ns | 2901 |  | 219 | Program control messages: suspend, resume, interrupt | 2.7 | 1.9 | 0.544 |
| ns | 3100 |  | 199 | `Run()`: how input and the TTY are acquired | 2.8 |  | 0.526 |
| walker |  | 3147 | 392 | Code::CodeKey { rung: Decl, file: tea.go, decl: 20, sub: 1, line: 426 } |  |  | 0.526 |
| ns | 3288 |  | 188 | `Run()`: renderer choice, colour profile, first three messages | 2.9 | 2.8 | 0.511 |
| walker |  | 3335 | 188 | Code::CodeKey { rung: Decl, file: tea.go, decl: 20, sub: 2, line: 426 } |  |  | 0.511 |
| walker |  | 3352 | 17 | Code::CodeKey { rung: Doc, file: tea.go, decl: 14, sub: 0, line: 321 } |  |  | 0.511 |
| walker |  | 3369 | 17 | Code::CodeKey { rung: Doc, file: tea.go, decl: 23, sub: 0, line: 564 } |  |  | 0.513 |
| ns | 3474 |  | 186 | `Run()`: model init, event loop, graceful shutdown | 2.10 | 2.9 | 0.496 |
| walker |  | 3567 | 198 | Code::CodeKey { rung: Decl, file: tea.go, decl: 20, sub: 3, line: 426 } |  |  | 0.496 |
| walker |  | 3585 | 18 | Code::CodeKey { rung: Doc, file: tea.go, decl: 33, sub: 0, line: 1209 } |  |  | 0.496 |
| ns | 3693 |  | 219 | Debug hooks: `TEA_TRACE`, `TEA_DEBUG`, panic recovery | 2.11 |  | 0.486 |
| walker |  | 3813 | 228 | Code::CodeKey { rung: Decl, file: tea.go, decl: 20, sub: 4, line: 426 } |  |  | 0.486 |
| ns | 3916 |  | 223 | Frame pacing: the render ticker and the FPS bounds | 2.12 |  | 0.470 |
| ns | 4118 |  | 202 | Input event message types: keys, mouse, paste, focus (complete) | 3.1 |  | 0.455 |
| walker |  | 4124 | 311 | Code::CodeKey { rung: Decl, file: tea.go, decl: 20, sub: 5, line: 426 } |  |  | 0.455 |
| walker |  | 4141 | 17 | Code::CodeKey { rung: Doc, file: tea.go, decl: 21, sub: 0, line: 555 } |  |  | 0.457 |
| walker |  | 4160 | 19 | Code::CodeKey { rung: Doc, file: tea.go, decl: 2, sub: 0, line: 42 } |  |  | 0.459 |
| walker |  | 4178 | 18 | Fs::DirListing { dir: .github/ISSUE_TEMPLATE } |  |  | 0.459 |
| walker |  | 4201 | 23 | Code::CodeKey { rung: Doc, file: tea.go, decl: 27, sub: 0, line: 590 } |  |  | 0.463 |
| walker |  | 4251 | 50 | GoMod::File { file: tutorials/go.mod } |  |  | 0.463 |
| walker |  | 4275 | 24 | Code::CodeKey { rung: Doc, file: tea.go, decl: 18, sub: 0, line: 374 } |  |  | 0.463 |
| walker |  | 4300 | 25 | Code::CodeKey { rung: Doc, file: tea.go, decl: 1, sub: 0, line: 39 } |  |  | 0.467 |
| walker |  | 4328 | 28 | Code::CodeKey { rung: Doc, file: tea.go, decl: 25, sub: 0, line: 578 } |  |  | 0.472 |
| walker |  | 4360 | 32 | Code::CodeKey { rung: Doc, file: tea.go, decl: 22, sub: 0, line: 561 } |  |  | 0.476 |
| ns | 4380 |  | 262 | Terminal report message types: colour, clipboard, size, capability (complete) | 3.2 |  | 0.460 |
| ns | 4550 |  | 170 | Command constructors, part 1: lifecycle, batching, timing, output | 3.3 | 2.7 | 0.452 |
| walker |  | 4610 | 250 | Fs::DirListing { dir: examples } |  |  | 0.456 |
| ns | 4696 |  | 146 | Command constructors, part 2: terminal queries and clipboard | 3.4 | 3.3 | 0.449 |
| walker |  | 4856 | 246 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.450 |
| walker |  | 4890 | 34 | Code::CodeKey { rung: Doc, file: tea.go, decl: 4, sub: 0, line: 50 } |  |  | 0.460 |
| walker |  | 4926 | 36 | Code::CodeKey { rung: Doc, file: tea.go, decl: 3, sub: 0, line: 46 } |  |  | 0.467 |
| walker |  | 4964 | 38 | Code::CodeKey { rung: Doc, file: tea.go, decl: 34, sub: 0, line: 1319 } |  |  | 0.467 |
| ns | 5037 |  | 341 | `input.go`: the ultraviolet-event → `tea.Msg` translation table | 3.5 |  | 0.448 |
| ns | 5113 |  | 76 | `Key` struct fields | 3.6 |  | 0.443 |
| ns | 5293 |  | 180 | Key message methods: `String`, `Keystroke`, `Key` | 3.7 | 3.6 | 0.437 |
| walker |  | 5473 | 509 | Code::CodeKey { rung: Decl, file: tea.go, decl: 7, sub: 0, line: 84 } |  |  | 0.439 |
| walker |  | 5512 | 39 | Code::CodeKey { rung: Doc, file: tea.go, decl: 7, sub: 0, line: 84 } |  |  | 0.445 |
| ns | 5614 |  | 321 | `key.go` key-code catalog: group headings plus the C0/G0 names | 3.8 |  | 0.428 |
| walker |  | 5715 | 203 | Code::CodeKey { rung: Decl, file: tea.go, decl: 7, sub: 1, line: 84 } |  |  | 0.437 |
| ns | 5797 |  | 183 | Modifier keys: the complete `KeyMod` constant set | 3.9 |  | 0.429 |
| walker |  | 5925 | 210 | Code::CodeKey { rung: Decl, file: tea.go, decl: 7, sub: 2, line: 84 } |  |  | 0.436 |
| ns | 6023 |  | 226 | `Mouse` struct and the complete mouse-button constant set | 3.10 | 3.1 | 0.427 |
| walker |  | 6140 | 215 | Code::CodeKey { rung: Decl, file: tea.go, decl: 7, sub: 3, line: 84 } |  |  | 0.437 |
| ns | 6200 |  | 177 | `MouseMode` enum: none / cell-motion / all-motion | 3.11 | 1.7 | 0.455 |
| walker |  | 6293 | 153 | Code::CodeKey { rung: Decl, file: tea.go, decl: 7, sub: 4, line: 84 } |  |  | 0.463 |
| walker |  | 6342 | 49 | Code::CodeKey { rung: Doc, file: tea.go, decl: 16, sub: 0, line: 349 } |  |  | 0.463 |
| ns | 6378 |  | 178 | Cursor value types: `Cursor`, `NewCursor`, `CursorShape` | 3.12 | 1.7 | 0.460 |
| ns | 6566 |  | 188 | Keyboard enhancements: request fields and response predicates | 3.13 | 3.2 | 0.456 |
| walker |  | 6588 | 246 | GoMod::File { file: go.mod } |  |  | 0.474 |
| walker |  | 6641 | 53 | Code::CodeKey { rung: Doc, file: tea.go, decl: 29, sub: 0, line: 991 } |  |  | 0.474 |
| ns | 6759 |  | 193 | `Batch` vs `Sequence` semantics and where they execute | 3.14 | 3.3 | 0.469 |
| walker |  | 6817 | 176 | Code::CodeKey { rung: Names, file: cursed_renderer.go, decl: 0, sub: 0, line: 0 } |  |  | 0.469 |
| ns | 6959 |  | 200 | `Exec` / `ExecProcess` and the `ExecCommand` interface | 3.15 | 3.3 | 0.463 |
| walker |  | 7040 | 223 | Code::CodeKey { rung: Decl, file: cursed_renderer.go, decl: 1, sub: 0, line: 18 } |  |  | 0.463 |
| walker |  | 7049 | 9 | Code::CodeKey { rung: Doc, file: cursed_renderer.go, decl: 6, sub: 0, line: 75 } |  |  | 0.463 |
| walker |  | 7058 | 9 | Code::CodeKey { rung: Doc, file: cursed_renderer.go, decl: 7, sub: 0, line: 143 } |  |  | 0.463 |
| walker |  | 7068 | 10 | Code::CodeKey { rung: Doc, file: cursed_renderer.go, decl: 8, sub: 0, line: 249 } |  |  | 0.463 |
| walker |  | 7082 | 14 | Code::CodeKey { rung: Doc, file: cursed_renderer.go, decl: 4, sub: 0, line: 52 } |  |  | 0.463 |
| walker |  | 7097 | 15 | Code::CodeKey { rung: Doc, file: cursed_renderer.go, decl: 5, sub: 0, line: 59 } |  |  | 0.463 |
| ns | 7111 |  | 152 | Logging to a file: `LogToFile` / `LogToFileWith` | 3.16 |  | 0.459 |
| ns | 7328 |  | 217 | The `renderer` interface: complete method set, and its two implementations | 4.1 |  | 0.451 |
| walker |  | 7367 | 270 | Code::CodeKey { rung: Names, file: nil_renderer.go, decl: 0, sub: 0, line: 0 } |  |  | 0.452 |
| walker |  | 7374 | 7 | Code::CodeKey { rung: Doc, file: nil_renderer.go, decl: 3, sub: 0, line: 15 } |  |  | 0.452 |
| walker |  | 7381 | 7 | Code::CodeKey { rung: Doc, file: nil_renderer.go, decl: 6, sub: 0, line: 24 } |  |  | 0.452 |
| walker |  | 7389 | 8 | Code::CodeKey { rung: Doc, file: nil_renderer.go, decl: 4, sub: 0, line: 18 } |  |  | 0.452 |
| walker |  | 7397 | 8 | Code::CodeKey { rung: Doc, file: nil_renderer.go, decl: 5, sub: 0, line: 21 } |  |  | 0.452 |
| walker |  | 7406 | 9 | Code::CodeKey { rung: Doc, file: nil_renderer.go, decl: 7, sub: 0, line: 27 } |  |  | 0.452 |
| walker |  | 7415 | 9 | Code::CodeKey { rung: Doc, file: nil_renderer.go, decl: 8, sub: 0, line: 30 } |  |  | 0.452 |
| walker |  | 7424 | 9 | Code::CodeKey { rung: Doc, file: nil_renderer.go, decl: 9, sub: 0, line: 33 } |  |  | 0.452 |
| walker |  | 7433 | 9 | Code::CodeKey { rung: Doc, file: nil_renderer.go, decl: 10, sub: 0, line: 36 } |  |  | 0.452 |
| walker |  | 7442 | 9 | Code::CodeKey { rung: Doc, file: nil_renderer.go, decl: 11, sub: 0, line: 39 } |  |  | 0.452 |
| walker |  | 7452 | 10 | Code::CodeKey { rung: Doc, file: nil_renderer.go, decl: 12, sub: 0, line: 42 } |  |  | 0.452 |
| walker |  | 7462 | 10 | Code::CodeKey { rung: Doc, file: nil_renderer.go, decl: 15, sub: 0, line: 51 } |  |  | 0.452 |
| walker |  | 7473 | 11 | Code::CodeKey { rung: Doc, file: nil_renderer.go, decl: 14, sub: 0, line: 48 } |  |  | 0.452 |
| walker |  | 7485 | 12 | Code::CodeKey { rung: Doc, file: nil_renderer.go, decl: 13, sub: 0, line: 45 } |  |  | 0.452 |
| walker |  | 7492 | 7 | Code::CodeKey { rung: Body, file: nil_renderer.go, decl: 15, sub: 0, line: 51 } |  |  | 0.452 |
| walker |  | 7527 | 35 | Code::CodeKey { rung: Names, file: renderer.go, decl: 0, sub: 0, line: 0 } |  |  | 0.453 |
| walker |  | 7626 | 99 | Code::CodeKey { rung: Names, file: commands.go, decl: 0, sub: 0, line: 0 } |  |  | 0.462 |
| walker |  | 7635 | 9 | Code::CodeKey { rung: Body, file: commands.go, decl: 6, sub: 0, line: 173 } |  |  | 0.462 |
| walker |  | 7673 | 38 | Code::CodeKey { rung: Names, file: screen.go, decl: 0, sub: 0, line: 0 } |  |  | 0.464 |
| ns | 7681 |  | 353 | `cursedRenderer`: every function in the 860-line renderer | 4.2 | 4.1 | 0.457 |
| walker |  | 7695 | 22 | Code::CodeKey { rung: Decl, file: screen.go, decl: 1, sub: 0, line: 9 } |  |  | 0.457 |
| walker |  | 7745 | 50 | Code::CodeKey { rung: Decl, file: screen.go, decl: 3, sub: 0, line: 62 } |  |  | 0.457 |
| walker |  | 7754 | 9 | Code::CodeKey { rung: Body, file: screen.go, decl: 2, sub: 0, line: 20 } |  |  | 0.457 |
| walker |  | 7769 | 15 | Code::CodeKey { rung: Body, file: commands.go, decl: 1, sub: 0, line: 15 } |  |  | 0.457 |
| walker |  | 7784 | 15 | Code::CodeKey { rung: Body, file: commands.go, decl: 3, sub: 0, line: 25 } |  |  | 0.457 |
| walker |  | 7961 | 177 | Code::CodeKey { rung: Names, file: cursed_renderer.go, decl: 0, sub: 1, line: 0 } |  |  | 0.468 |
| walker |  | 7970 | 9 | Code::CodeKey { rung: Doc, file: cursed_renderer.go, decl: 9, sub: 0, line: 257 } |  |  | 0.468 |
| walker |  | 7979 | 9 | Code::CodeKey { rung: Doc, file: cursed_renderer.go, decl: 10, sub: 0, line: 579 } |  |  | 0.468 |
| walker |  | 7988 | 9 | Code::CodeKey { rung: Doc, file: cursed_renderer.go, decl: 11, sub: 0, line: 587 } |  |  | 0.468 |
| walker |  | 7997 | 9 | Code::CodeKey { rung: Doc, file: cursed_renderer.go, decl: 14, sub: 0, line: 619 } |  |  | 0.468 |
| walker |  | 8007 | 10 | Code::CodeKey { rung: Doc, file: cursed_renderer.go, decl: 15, sub: 0, line: 634 } |  |  | 0.468 |
| walker |  | 8018 | 11 | Code::CodeKey { rung: Doc, file: cursed_renderer.go, decl: 13, sub: 0, line: 611 } |  |  | 0.468 |
| ns | 8039 |  | 358 | `View` fields → ANSI sequences, in `cursedRenderer.start` | 4.3 | 4.2 | 0.458 |
| walker |  | 8204 | 186 | Code::CodeKey { rung: Names, file: cursed_renderer.go, decl: 0, sub: 2, line: 0 } |  |  | 0.476 |
| walker |  | 8214 | 10 | Code::CodeKey { rung: Doc, file: cursed_renderer.go, decl: 22, sub: 0, line: 707 } |  |  | 0.476 |
| walker |  | 8224 | 10 | Code::CodeKey { rung: Doc, file: cursed_renderer.go, decl: 23, sub: 0, line: 766 } |  |  | 0.476 |
| walker |  | 8235 | 11 | Code::CodeKey { rung: Doc, file: cursed_renderer.go, decl: 21, sub: 0, line: 690 } |  |  | 0.476 |
| walker |  | 8247 | 12 | Code::CodeKey { rung: Doc, file: cursed_renderer.go, decl: 20, sub: 0, line: 683 } |  |  | 0.476 |
| walker |  | 8261 | 14 | Code::CodeKey { rung: Doc, file: cursed_renderer.go, decl: 19, sub: 0, line: 674 } |  |  | 0.476 |
| walker |  | 8318 | 57 | Code::CodeKey { rung: Doc, file: tea.go, decl: 32, sub: 0, line: 1204 } |  |  | 0.476 |
| walker |  | 8376 | 58 | Code::CodeKey { rung: Doc, file: tea.go, decl: 15, sub: 0, line: 336 } |  |  | 0.476 |
| ns | 8420 |  | 381 | Platform matrix: build tags and per-OS terminal functions | 4.4 |  | 0.467 |
| walker |  | 8458 | 82 | Markdown::Section { file: README.md, section_index: 16, keeps_default_concavity: false } |  |  | 0.467 |
| walker |  | 8481 | 23 | Code::CodeKey { rung: Names, file: input.go, decl: 0, sub: 0, line: 0 } |  |  | 0.467 |
| ns | 8618 |  | 198 | `tty.go`: terminal acquisition, input loop, resize detection | 4.5 |  | 0.463 |
| walker |  | 8687 | 206 | Code::CodeKey { rung: Names, file: options.go, decl: 0, sub: 0, line: 0 } |  |  | 0.485 |
| ns | 8882 |  | 264 | `examples/` and `tutorials/` directory listings | 5.1 |  | 0.529 |
| ns | 9062 |  | 180 | `UPGRADE_GUIDE_V2.md`: complete section map | 5.2 |  | 0.523 |
| walker |  | 9095 | 408 | Code::CodeKey { rung: Names, file: mouse.go, decl: 0, sub: 0, line: 0 } |  |  | 0.538 |
| walker |  | 9100 | 5 | Code::CodeKey { rung: Decl, file: mouse.go, decl: 2, sub: 0, line: 29 } |  |  | 0.539 |
| walker |  | 9133 | 33 | Code::CodeKey { rung: Decl, file: mouse.go, decl: 4, sub: 0, line: 71 } |  |  | 0.544 |
| walker |  | 9173 | 40 | Code::CodeKey { rung: Decl, file: mouse.go, decl: 3, sub: 0, line: 46 } |  |  | 0.545 |
| walker |  | 9186 | 13 | Code::CodeKey { rung: Doc, file: mouse.go, decl: 5, sub: 0, line: 78 } |  |  | 0.545 |
| walker |  | 9200 | 14 | Code::CodeKey { rung: Doc, file: mouse.go, decl: 7, sub: 0, line: 86 } |  |  | 0.545 |
| walker |  | 9214 | 14 | Code::CodeKey { rung: Doc, file: mouse.go, decl: 10, sub: 0, line: 101 } |  |  | 0.545 |
| walker |  | 9228 | 14 | Code::CodeKey { rung: Doc, file: mouse.go, decl: 13, sub: 0, line: 116 } |  |  | 0.545 |
| walker |  | 9242 | 14 | Code::CodeKey { rung: Doc, file: mouse.go, decl: 15, sub: 0, line: 128 } |  |  | 0.545 |
| walker |  | 9256 | 14 | Code::CodeKey { rung: Doc, file: mouse.go, decl: 16, sub: 0, line: 131 } |  |  | 0.545 |
| walker |  | 9271 | 15 | Code::CodeKey { rung: Doc, file: mouse.go, decl: 6, sub: 0, line: 83 } |  |  | 0.545 |
| walker |  | 9286 | 15 | Code::CodeKey { rung: Doc, file: mouse.go, decl: 9, sub: 0, line: 98 } |  |  | 0.545 |
| walker |  | 9301 | 15 | Code::CodeKey { rung: Doc, file: mouse.go, decl: 12, sub: 0, line: 113 } |  |  | 0.545 |
| ns | 9347 |  | 285 | The v1→v2 migration checklist | 5.3 | 5.2 | 0.541 |
| walker |  | 9495 | 194 | Code::CodeKey { rung: Names, file: color.go, decl: 0, sub: 0, line: 0 } |  |  | 0.544 |
| walker |  | 9507 | 12 | Code::CodeKey { rung: Doc, file: color.go, decl: 5, sub: 0, line: 39 } |  |  | 0.544 |
| ns | 9516 |  | 169 | README section map | 5.4 |  | 0.552 |
| walker |  | 9519 | 12 | Code::CodeKey { rung: Doc, file: color.go, decl: 8, sub: 0, line: 70 } |  |  | 0.552 |
| walker |  | 9531 | 12 | Code::CodeKey { rung: Doc, file: color.go, decl: 11, sub: 0, line: 84 } |  |  | 0.552 |
| walker |  | 9545 | 14 | Code::CodeKey { rung: Doc, file: color.go, decl: 6, sub: 0, line: 44 } |  |  | 0.552 |
| walker |  | 9559 | 14 | Code::CodeKey { rung: Doc, file: color.go, decl: 9, sub: 0, line: 75 } |  |  | 0.552 |
| walker |  | 9573 | 14 | Code::CodeKey { rung: Doc, file: color.go, decl: 12, sub: 0, line: 89 } |  |  | 0.552 |
| walker |  | 9582 | 9 | Code::CodeKey { rung: Body, file: color.go, decl: 1, sub: 0, line: 13 } |  |  | 0.552 |
| walker |  | 9600 | 18 | Code::CodeKey { rung: Doc, file: color.go, decl: 1, sub: 0, line: 13 } |  |  | 0.552 |
| walker |  | 9618 | 18 | Code::CodeKey { rung: Doc, file: color.go, decl: 2, sub: 0, line: 21 } |  |  | 0.552 |
| walker |  | 9636 | 18 | Code::CodeKey { rung: Doc, file: color.go, decl: 3, sub: 0, line: 29 } |  |  | 0.552 |
| walker |  | 9654 | 18 | Code::CodeKey { rung: Doc, file: input.go, decl: 1, sub: 0, line: 8 } |  |  | 0.552 |
| walker |  | 9672 | 18 | Code::CodeKey { rung: Doc, file: mouse.go, decl: 1, sub: 0, line: 10 } |  |  | 0.552 |
| walker |  | 9681 | 9 | Code::CodeKey { rung: Body, file: color.go, decl: 2, sub: 0, line: 21 } |  |  | 0.552 |
| walker |  | 9690 | 9 | Code::CodeKey { rung: Body, file: color.go, decl: 3, sub: 0, line: 29 } |  |  | 0.552 |
| walker |  | 9714 | 24 | Code::CodeKey { rung: Doc, file: options.go, decl: 8, sub: 0, line: 84 } |  |  | 0.552 |
| walker |  | 9737 | 23 | Code::CodeKey { rung: Names, file: tty.go, decl: 0, sub: 0, line: 0 } |  |  | 0.552 |
| walker |  | 9756 | 19 | Code::CodeKey { rung: Doc, file: tty.go, decl: 1, sub: 0, line: 130 } |  |  | 0.552 |
| walker |  | 9833 | 77 | Code::CodeKey { rung: Names, file: tty_unix.go, decl: 0, sub: 0, line: 0 } |  |  | 0.553 |
| walker |  | 9847 | 14 | Code::CodeKey { rung: Doc, file: tty_unix.go, decl: 3, sub: 0, line: 40 } |  |  | 0.553 |
| walker |  | 9873 | 26 | Code::CodeKey { rung: Names, file: raw.go, decl: 0, sub: 0, line: 0 } |  |  | 0.554 |
| ns | 9876 |  | 360 | Test suite: every test function, and the golden-file fixtures | 5.5 |  | 0.543 |
| walker |  | 9886 | 13 | Code::CodeKey { rung: Decl, file: raw.go, decl: 1, sub: 0, line: 5 } |  |  | 0.544 |
| ns | 9917 |  | 41 | CI workflow inventory and golden-test fixture directories | 5.6 |  | 0.548 |
| walker |  | 9929 | 43 | Code::CodeKey { rung: Names, file: termcap.go, decl: 0, sub: 0, line: 0 } |  |  | 0.549 |
| walker |  | 9941 | 12 | Code::CodeKey { rung: Decl, file: termcap.go, decl: 2, sub: 0, line: 41 } |  |  | 0.550 |
| walker |  | 9949 | 8 | Code::CodeKey { rung: Body, file: termcap.go, decl: 3, sub: 0, line: 46 } |  |  | 0.550 |
| walker |  | 9961 | 12 | Code::CodeKey { rung: Doc, file: termcap.go, decl: 3, sub: 0, line: 46 } |  |  | 0.550 |
| walker |  | 9984 | 23 | Code::CodeKey { rung: Names, file: key.go, decl: 0, sub: 0, line: 0 } |  |  | 0.550 |
