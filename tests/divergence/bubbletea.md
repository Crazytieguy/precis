Score(3000)=0.552 I=0.769 C=0.396 ns_rows≤3K=18/50 grid(1000/1442/2080/3000/4327/6240/9000)=0.591/0.606/0.577/0.552/0.476/0.457/0.543

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
| walker |  | 1084 | 7 | Code::CodeKey { rung: Doc, file: tea.go, decl: 13, sub: 0, line: 312 } |  |  | 0.591 |
| ns | 1132 |  | 157 | `Cmd` semantics and the `Quit` command | 1.9 | 1.3 | 0.554 |
| walker |  | 1246 | 162 | Code::CodeKey { rung: Decl, file: tea.go, decl: 5, sub: 0, line: 53 } |  |  | 0.633 |
| ns | 1338 |  | 206 | Direct dependency set from `go.mod` | 1.10 |  | 0.605 |
| walker |  | 1494 | 248 | Code::CodeKey { rung: Decl, file: tea.go, decl: 11, sub: 0, line: 286 } |  |  | 0.607 |
| walker |  | 1508 | 14 | Code::CodeKey { rung: Doc, file: tea.go, decl: 12, sub: 0, line: 309 } |  |  | 0.607 |
| ns | 1522 |  | 184 | README lede: positioning and shipped feature set | 1.11 |  | 0.589 |
| walker |  | 1523 | 15 | Code::CodeKey { rung: Doc, file: tea.go, decl: 10, sub: 0, line: 284 } |  |  | 0.590 |
| walker |  | 1539 | 16 | Code::CodeKey { rung: Doc, file: tea.go, decl: 5, sub: 0, line: 53 } |  |  | 0.637 |
| ns | 1665 |  | 143 | Sentinel errors returned by `Program.Run` | 2.1 |  | 0.619 |
| ns | 1810 |  | 145 | Every exported `*Program` method (complete roster) | 2.2 | 1.4 | 0.599 |
| walker |  | 1900 | 361 | Code::CodeKey { rung: Decl, file: tea.go, decl: 8, sub: 0, line: 241 } |  |  | 0.599 |
| ns | 2001 |  | 191 | Every `ProgramOption` constructor (complete roster) | 2.3 |  | 0.575 |
| walker |  | 2120 | 220 | Code::CodeKey { rung: Names, file: tea.go, decl: 0, sub: 1, line: 0 } |  |  | 0.587 |
| ns | 2130 |  | 129 | Build/test/lint entry points (`Taskfile.yaml`) | 2.4 |  | 0.611 |
| walker |  | 2235 | 115 | Code::CodeKey { rung: Decl, file: tea.go, decl: 15, sub: 0, line: 336 } |  |  | 0.611 |
| walker |  | 2383 | 148 | Code::CodeKey { rung: Decl, file: tea.go, decl: 17, sub: 0, line: 357 } |  |  | 0.611 |
| walker |  | 2396 | 13 | Code::CodeKey { rung: Doc, file: tea.go, decl: 28, sub: 0, line: 595 } |  |  | 0.619 |
| walker |  | 2410 | 14 | Code::CodeKey { rung: Doc, file: tea.go, decl: 17, sub: 0, line: 357 } |  |  | 0.620 |
| ns | 2436 |  | 306 | Event-loop dispatch table: every internally-handled message type | 2.5 |  | 0.573 |
| walker |  | 2555 | 145 | Code::CodeKey { rung: Names, file: tea.go, decl: 0, sub: 2, line: 0 } |  |  | 0.604 |
| walker |  | 2572 | 17 | Code::CodeKey { rung: Doc, file: tea.go, decl: 14, sub: 0, line: 321 } |  |  | 0.604 |
| walker |  | 2589 | 17 | Code::CodeKey { rung: Doc, file: tea.go, decl: 23, sub: 0, line: 564 } |  |  | 0.604 |
| walker |  | 2607 | 18 | Code::CodeKey { rung: Doc, file: tea.go, decl: 33, sub: 0, line: 1209 } |  |  | 0.604 |
| ns | 2682 |  | 246 | Event-loop skeleton: translate → filter → dispatch → update → render | 2.6 | 2.5 | 0.569 |
| walker |  | 2802 | 195 | Code::CodeKey { rung: Decl, file: tea.go, decl: 20, sub: 0, line: 426 } |  |  | 0.569 |
| walker |  | 2814 | 12 | Code::CodeKey { rung: Doc, file: tea.go, decl: 20, sub: 0, line: 426 } |  |  | 0.569 |
| walker |  | 2831 | 17 | Code::CodeKey { rung: Doc, file: tea.go, decl: 21, sub: 0, line: 555 } |  |  | 0.571 |
| walker |  | 2850 | 19 | Code::CodeKey { rung: Doc, file: tea.go, decl: 2, sub: 0, line: 42 } |  |  | 0.575 |
| ns | 2901 |  | 219 | Program control messages: suspend, resume, interrupt | 2.7 | 1.9 | 0.552 |
| ns | 3100 |  | 199 | `Run()`: how input and the TTY are acquired | 2.8 |  | 0.533 |
| walker |  | 3242 | 392 | Code::CodeKey { rung: Decl, file: tea.go, decl: 20, sub: 1, line: 426 } |  |  | 0.533 |
| ns | 3288 |  | 188 | `Run()`: renderer choice, colour profile, first three messages | 2.9 | 2.8 | 0.518 |
| walker |  | 3430 | 188 | Code::CodeKey { rung: Decl, file: tea.go, decl: 20, sub: 2, line: 426 } |  |  | 0.518 |
| ns | 3474 |  | 186 | `Run()`: model init, event loop, graceful shutdown | 2.10 | 2.9 | 0.500 |
| walker |  | 3628 | 198 | Code::CodeKey { rung: Decl, file: tea.go, decl: 20, sub: 3, line: 426 } |  |  | 0.500 |
| ns | 3693 |  | 219 | Debug hooks: `TEA_TRACE`, `TEA_DEBUG`, panic recovery | 2.11 |  | 0.490 |
| walker |  | 3856 | 228 | Code::CodeKey { rung: Decl, file: tea.go, decl: 20, sub: 4, line: 426 } |  |  | 0.490 |
| ns | 3916 |  | 223 | Frame pacing: the render ticker and the FPS bounds | 2.12 |  | 0.474 |
| ns | 4118 |  | 202 | Input event message types: keys, mouse, paste, focus (complete) | 3.1 |  | 0.459 |
| walker |  | 4160 | 304 | Code::CodeKey { rung: Decl, file: tea.go, decl: 20, sub: 5, line: 426 } |  |  | 0.459 |
| walker |  | 4183 | 23 | Code::CodeKey { rung: Doc, file: tea.go, decl: 27, sub: 0, line: 590 } |  |  | 0.463 |
| walker |  | 4207 | 24 | Code::CodeKey { rung: Doc, file: tea.go, decl: 18, sub: 0, line: 374 } |  |  | 0.463 |
| walker |  | 4232 | 25 | Code::CodeKey { rung: Doc, file: tea.go, decl: 1, sub: 0, line: 39 } |  |  | 0.467 |
| walker |  | 4250 | 18 | Fs::DirListing { dir: .github/ISSUE_TEMPLATE } |  |  | 0.467 |
| walker |  | 4278 | 28 | Code::CodeKey { rung: Doc, file: tea.go, decl: 25, sub: 0, line: 578 } |  |  | 0.472 |
| walker |  | 4310 | 32 | Code::CodeKey { rung: Doc, file: tea.go, decl: 22, sub: 0, line: 561 } |  |  | 0.476 |
| walker |  | 4344 | 34 | Code::CodeKey { rung: Doc, file: tea.go, decl: 4, sub: 0, line: 50 } |  |  | 0.486 |
| walker |  | 4380 | 36 | Code::CodeKey { rung: Doc, file: tea.go, decl: 3, sub: 0, line: 46 } |  |  | 0.478 |
| ns | 4380 |  | 262 | Terminal report message types: colour, clipboard, size, capability (complete) | 3.2 |  | 0.478 |
| walker |  | 4418 | 38 | Code::CodeKey { rung: Doc, file: tea.go, decl: 34, sub: 0, line: 1319 } |  |  | 0.478 |
| ns | 4550 |  | 170 | Command constructors, part 1: lifecycle, batching, timing, output | 3.3 | 2.7 | 0.469 |
| walker |  | 4668 | 250 | Fs::DirListing { dir: examples } |  |  | 0.474 |
| ns | 4696 |  | 146 | Command constructors, part 2: terminal queries and clipboard | 3.4 | 3.3 | 0.466 |
| walker |  | 4914 | 246 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.467 |
| walker |  | 4963 | 49 | Code::CodeKey { rung: Doc, file: tea.go, decl: 16, sub: 0, line: 349 } |  |  | 0.467 |
| ns | 5037 |  | 341 | `input.go`: the ultraviolet-event → `tea.Msg` translation table | 3.5 |  | 0.448 |
| ns | 5113 |  | 76 | `Key` struct fields | 3.6 |  | 0.443 |
| ns | 5293 |  | 180 | Key message methods: `String`, `Keystroke`, `Key` | 3.7 | 3.6 | 0.437 |
| walker |  | 5479 | 516 | Code::CodeKey { rung: Decl, file: tea.go, decl: 7, sub: 0, line: 84 } |  |  | 0.439 |
| walker |  | 5518 | 39 | Code::CodeKey { rung: Doc, file: tea.go, decl: 7, sub: 0, line: 84 } |  |  | 0.446 |
| walker |  | 5571 | 53 | Code::CodeKey { rung: Doc, file: tea.go, decl: 29, sub: 0, line: 991 } |  |  | 0.446 |
| ns | 5614 |  | 321 | `key.go` key-code catalog: group headings plus the C0/G0 names | 3.8 |  | 0.429 |
| walker |  | 5774 | 203 | Code::CodeKey { rung: Decl, file: tea.go, decl: 7, sub: 1, line: 84 } |  |  | 0.438 |
| ns | 5797 |  | 183 | Modifier keys: the complete `KeyMod` constant set | 3.9 |  | 0.431 |
| walker |  | 5984 | 210 | Code::CodeKey { rung: Decl, file: tea.go, decl: 7, sub: 2, line: 84 } |  |  | 0.438 |
| ns | 6023 |  | 226 | `Mouse` struct and the complete mouse-button constant set | 3.10 | 3.1 | 0.429 |
| walker |  | 6199 | 215 | Code::CodeKey { rung: Decl, file: tea.go, decl: 7, sub: 3, line: 84 } |  |  | 0.439 |
| ns | 6200 |  | 177 | `MouseMode` enum: none / cell-motion / all-motion | 3.11 | 1.7 | 0.457 |
| walker |  | 6345 | 146 | Code::CodeKey { rung: Decl, file: tea.go, decl: 7, sub: 4, line: 84 } |  |  | 0.463 |
| ns | 6378 |  | 178 | Cursor value types: `Cursor`, `NewCursor`, `CursorShape` | 3.12 | 1.7 | 0.460 |
| walker |  | 6402 | 57 | Code::CodeKey { rung: Doc, file: tea.go, decl: 32, sub: 0, line: 1204 } |  |  | 0.460 |
| ns | 6566 |  | 188 | Keyboard enhancements: request fields and response predicates | 3.13 | 3.2 | 0.456 |
| walker |  | 6648 | 246 | GoMod::File { file: go.mod } |  |  | 0.474 |
| walker |  | 6706 | 58 | Code::CodeKey { rung: Doc, file: tea.go, decl: 15, sub: 0, line: 336 } |  |  | 0.474 |
| ns | 6759 |  | 193 | `Batch` vs `Sequence` semantics and where they execute | 3.14 | 3.3 | 0.469 |
| walker |  | 6765 | 59 | Code::CodeKey { rung: Doc, file: tea.go, decl: 35, sub: 0, line: 1344 } |  |  | 0.469 |
| walker |  | 6826 | 61 | Code::CodeKey { rung: Doc, file: tea.go, decl: 36, sub: 0, line: 1372 } |  |  | 0.469 |
| ns | 6959 |  | 200 | `Exec` / `ExecProcess` and the `ExecCommand` interface | 3.15 | 3.3 | 0.462 |
| walker |  | 7002 | 176 | Code::CodeKey { rung: Names, file: cursed_renderer.go, decl: 0, sub: 0, line: 0 } |  |  | 0.463 |
| ns | 7111 |  | 152 | Logging to a file: `LogToFile` / `LogToFileWith` | 3.16 |  | 0.459 |
| walker |  | 7225 | 223 | Code::CodeKey { rung: Decl, file: cursed_renderer.go, decl: 1, sub: 0, line: 18 } |  |  | 0.459 |
| walker |  | 7234 | 9 | Code::CodeKey { rung: Doc, file: cursed_renderer.go, decl: 6, sub: 0, line: 75 } |  |  | 0.459 |
| walker |  | 7243 | 9 | Code::CodeKey { rung: Doc, file: cursed_renderer.go, decl: 7, sub: 0, line: 143 } |  |  | 0.459 |
| walker |  | 7253 | 10 | Code::CodeKey { rung: Doc, file: cursed_renderer.go, decl: 8, sub: 0, line: 249 } |  |  | 0.459 |
| walker |  | 7267 | 14 | Code::CodeKey { rung: Doc, file: cursed_renderer.go, decl: 4, sub: 0, line: 52 } |  |  | 0.459 |
| walker |  | 7282 | 15 | Code::CodeKey { rung: Doc, file: cursed_renderer.go, decl: 5, sub: 0, line: 59 } |  |  | 0.459 |
| ns | 7328 |  | 217 | The `renderer` interface: complete method set, and its two implementations | 4.1 |  | 0.451 |
| walker |  | 7552 | 270 | Code::CodeKey { rung: Names, file: nil_renderer.go, decl: 0, sub: 0, line: 0 } |  |  | 0.452 |
| walker |  | 7559 | 7 | Code::CodeKey { rung: Doc, file: nil_renderer.go, decl: 3, sub: 0, line: 15 } |  |  | 0.452 |
| walker |  | 7566 | 7 | Code::CodeKey { rung: Doc, file: nil_renderer.go, decl: 6, sub: 0, line: 24 } |  |  | 0.452 |
| walker |  | 7574 | 8 | Code::CodeKey { rung: Doc, file: nil_renderer.go, decl: 4, sub: 0, line: 18 } |  |  | 0.452 |
| walker |  | 7582 | 8 | Code::CodeKey { rung: Doc, file: nil_renderer.go, decl: 5, sub: 0, line: 21 } |  |  | 0.452 |
| walker |  | 7591 | 9 | Code::CodeKey { rung: Doc, file: nil_renderer.go, decl: 7, sub: 0, line: 27 } |  |  | 0.452 |
| walker |  | 7600 | 9 | Code::CodeKey { rung: Doc, file: nil_renderer.go, decl: 8, sub: 0, line: 30 } |  |  | 0.452 |
| walker |  | 7609 | 9 | Code::CodeKey { rung: Doc, file: nil_renderer.go, decl: 9, sub: 0, line: 33 } |  |  | 0.452 |
| walker |  | 7618 | 9 | Code::CodeKey { rung: Doc, file: nil_renderer.go, decl: 10, sub: 0, line: 36 } |  |  | 0.452 |
| walker |  | 7627 | 9 | Code::CodeKey { rung: Doc, file: nil_renderer.go, decl: 11, sub: 0, line: 39 } |  |  | 0.452 |
| walker |  | 7637 | 10 | Code::CodeKey { rung: Doc, file: nil_renderer.go, decl: 12, sub: 0, line: 42 } |  |  | 0.452 |
| walker |  | 7647 | 10 | Code::CodeKey { rung: Doc, file: nil_renderer.go, decl: 15, sub: 0, line: 51 } |  |  | 0.452 |
| walker |  | 7658 | 11 | Code::CodeKey { rung: Doc, file: nil_renderer.go, decl: 14, sub: 0, line: 48 } |  |  | 0.452 |
| walker |  | 7670 | 12 | Code::CodeKey { rung: Doc, file: nil_renderer.go, decl: 13, sub: 0, line: 45 } |  |  | 0.452 |
| walker |  | 7677 | 7 | Code::CodeKey { rung: Body, file: nil_renderer.go, decl: 15, sub: 0, line: 51 } |  |  | 0.452 |
| ns | 7681 |  | 353 | `cursedRenderer`: every function in the 860-line renderer | 4.2 | 4.1 | 0.444 |
| walker |  | 7712 | 35 | Code::CodeKey { rung: Names, file: renderer.go, decl: 0, sub: 0, line: 0 } |  |  | 0.446 |
| walker |  | 7811 | 99 | Code::CodeKey { rung: Names, file: commands.go, decl: 0, sub: 0, line: 0 } |  |  | 0.454 |
| walker |  | 7820 | 9 | Code::CodeKey { rung: Body, file: commands.go, decl: 6, sub: 0, line: 173 } |  |  | 0.454 |
| walker |  | 7858 | 38 | Code::CodeKey { rung: Names, file: screen.go, decl: 0, sub: 0, line: 0 } |  |  | 0.457 |
| walker |  | 7880 | 22 | Code::CodeKey { rung: Decl, file: screen.go, decl: 1, sub: 0, line: 9 } |  |  | 0.457 |
| walker |  | 7930 | 50 | Code::CodeKey { rung: Decl, file: screen.go, decl: 3, sub: 0, line: 62 } |  |  | 0.457 |
| walker |  | 7939 | 9 | Code::CodeKey { rung: Body, file: screen.go, decl: 2, sub: 0, line: 20 } |  |  | 0.457 |
| walker |  | 7954 | 15 | Code::CodeKey { rung: Body, file: commands.go, decl: 1, sub: 0, line: 15 } |  |  | 0.457 |
| walker |  | 7969 | 15 | Code::CodeKey { rung: Body, file: commands.go, decl: 3, sub: 0, line: 25 } |  |  | 0.457 |
| ns | 8039 |  | 358 | `View` fields → ANSI sequences, in `cursedRenderer.start` | 4.3 | 4.2 | 0.447 |
| walker |  | 8146 | 177 | Code::CodeKey { rung: Names, file: cursed_renderer.go, decl: 0, sub: 1, line: 0 } |  |  | 0.458 |
| walker |  | 8155 | 9 | Code::CodeKey { rung: Doc, file: cursed_renderer.go, decl: 9, sub: 0, line: 257 } |  |  | 0.458 |
| walker |  | 8164 | 9 | Code::CodeKey { rung: Doc, file: cursed_renderer.go, decl: 10, sub: 0, line: 579 } |  |  | 0.458 |
| walker |  | 8173 | 9 | Code::CodeKey { rung: Doc, file: cursed_renderer.go, decl: 11, sub: 0, line: 587 } |  |  | 0.458 |
| walker |  | 8182 | 9 | Code::CodeKey { rung: Doc, file: cursed_renderer.go, decl: 14, sub: 0, line: 619 } |  |  | 0.458 |
| walker |  | 8192 | 10 | Code::CodeKey { rung: Doc, file: cursed_renderer.go, decl: 15, sub: 0, line: 634 } |  |  | 0.458 |
| walker |  | 8203 | 11 | Code::CodeKey { rung: Doc, file: cursed_renderer.go, decl: 13, sub: 0, line: 611 } |  |  | 0.458 |
| walker |  | 8389 | 186 | Code::CodeKey { rung: Names, file: cursed_renderer.go, decl: 0, sub: 2, line: 0 } |  |  | 0.476 |
| walker |  | 8399 | 10 | Code::CodeKey { rung: Doc, file: cursed_renderer.go, decl: 22, sub: 0, line: 707 } |  |  | 0.476 |
| walker |  | 8409 | 10 | Code::CodeKey { rung: Doc, file: cursed_renderer.go, decl: 23, sub: 0, line: 766 } |  |  | 0.476 |
| walker |  | 8420 | 11 | Code::CodeKey { rung: Doc, file: cursed_renderer.go, decl: 21, sub: 0, line: 690 } |  |  | 0.467 |
| ns | 8420 |  | 381 | Platform matrix: build tags and per-OS terminal functions | 4.4 |  | 0.467 |
| walker |  | 8432 | 12 | Code::CodeKey { rung: Doc, file: cursed_renderer.go, decl: 20, sub: 0, line: 683 } |  |  | 0.467 |
| walker |  | 8446 | 14 | Code::CodeKey { rung: Doc, file: cursed_renderer.go, decl: 19, sub: 0, line: 674 } |  |  | 0.467 |
| walker |  | 8528 | 82 | Markdown::Section { file: README.md, section_index: 16, keeps_default_concavity: false } |  |  | 0.467 |
| walker |  | 8551 | 23 | Code::CodeKey { rung: Names, file: input.go, decl: 0, sub: 0, line: 0 } |  |  | 0.467 |
| walker |  | 8569 | 18 | Code::CodeKey { rung: Doc, file: input.go, decl: 1, sub: 0, line: 8 } |  |  | 0.467 |
| ns | 8618 |  | 198 | `tty.go`: terminal acquisition, input loop, resize detection | 4.5 |  | 0.463 |
| walker |  | 8775 | 206 | Code::CodeKey { rung: Names, file: options.go, decl: 0, sub: 0, line: 0 } |  |  | 0.485 |
| ns | 8882 |  | 264 | `examples/` and `tutorials/` directory listings | 5.1 |  | 0.529 |
| ns | 9062 |  | 180 | `UPGRADE_GUIDE_V2.md`: complete section map | 5.2 |  | 0.523 |
| walker |  | 9183 | 408 | Code::CodeKey { rung: Names, file: mouse.go, decl: 0, sub: 0, line: 0 } |  |  | 0.539 |
| walker |  | 9188 | 5 | Code::CodeKey { rung: Decl, file: mouse.go, decl: 2, sub: 0, line: 29 } |  |  | 0.540 |
| walker |  | 9221 | 33 | Code::CodeKey { rung: Decl, file: mouse.go, decl: 4, sub: 0, line: 71 } |  |  | 0.544 |
| walker |  | 9261 | 40 | Code::CodeKey { rung: Decl, file: mouse.go, decl: 3, sub: 0, line: 46 } |  |  | 0.546 |
| walker |  | 9274 | 13 | Code::CodeKey { rung: Doc, file: mouse.go, decl: 5, sub: 0, line: 78 } |  |  | 0.546 |
| walker |  | 9288 | 14 | Code::CodeKey { rung: Doc, file: mouse.go, decl: 7, sub: 0, line: 86 } |  |  | 0.546 |
| walker |  | 9302 | 14 | Code::CodeKey { rung: Doc, file: mouse.go, decl: 10, sub: 0, line: 101 } |  |  | 0.546 |
| walker |  | 9316 | 14 | Code::CodeKey { rung: Doc, file: mouse.go, decl: 13, sub: 0, line: 116 } |  |  | 0.546 |
| walker |  | 9330 | 14 | Code::CodeKey { rung: Doc, file: mouse.go, decl: 15, sub: 0, line: 128 } |  |  | 0.546 |
| walker |  | 9344 | 14 | Code::CodeKey { rung: Doc, file: mouse.go, decl: 16, sub: 0, line: 131 } |  |  | 0.546 |
| ns | 9347 |  | 285 | The v1→v2 migration checklist | 5.3 | 5.2 | 0.541 |
| walker |  | 9359 | 15 | Code::CodeKey { rung: Doc, file: mouse.go, decl: 6, sub: 0, line: 83 } |  |  | 0.541 |
| walker |  | 9374 | 15 | Code::CodeKey { rung: Doc, file: mouse.go, decl: 9, sub: 0, line: 98 } |  |  | 0.541 |
| walker |  | 9389 | 15 | Code::CodeKey { rung: Doc, file: mouse.go, decl: 12, sub: 0, line: 113 } |  |  | 0.541 |
| walker |  | 9407 | 18 | Code::CodeKey { rung: Doc, file: mouse.go, decl: 1, sub: 0, line: 10 } |  |  | 0.541 |
| ns | 9516 |  | 169 | README section map | 5.4 |  | 0.549 |
| walker |  | 9601 | 194 | Code::CodeKey { rung: Names, file: color.go, decl: 0, sub: 0, line: 0 } |  |  | 0.552 |
| walker |  | 9613 | 12 | Code::CodeKey { rung: Doc, file: color.go, decl: 5, sub: 0, line: 39 } |  |  | 0.552 |
| walker |  | 9625 | 12 | Code::CodeKey { rung: Doc, file: color.go, decl: 8, sub: 0, line: 70 } |  |  | 0.552 |
| walker |  | 9637 | 12 | Code::CodeKey { rung: Doc, file: color.go, decl: 11, sub: 0, line: 84 } |  |  | 0.552 |
| walker |  | 9651 | 14 | Code::CodeKey { rung: Doc, file: color.go, decl: 6, sub: 0, line: 44 } |  |  | 0.552 |
| walker |  | 9665 | 14 | Code::CodeKey { rung: Doc, file: color.go, decl: 9, sub: 0, line: 75 } |  |  | 0.552 |
| walker |  | 9679 | 14 | Code::CodeKey { rung: Doc, file: color.go, decl: 12, sub: 0, line: 89 } |  |  | 0.552 |
| walker |  | 9688 | 9 | Code::CodeKey { rung: Body, file: color.go, decl: 1, sub: 0, line: 13 } |  |  | 0.552 |
| walker |  | 9706 | 18 | Code::CodeKey { rung: Doc, file: color.go, decl: 1, sub: 0, line: 13 } |  |  | 0.552 |
| walker |  | 9724 | 18 | Code::CodeKey { rung: Doc, file: color.go, decl: 2, sub: 0, line: 21 } |  |  | 0.552 |
| walker |  | 9742 | 18 | Code::CodeKey { rung: Doc, file: color.go, decl: 3, sub: 0, line: 29 } |  |  | 0.552 |
| walker |  | 9751 | 9 | Code::CodeKey { rung: Body, file: color.go, decl: 2, sub: 0, line: 21 } |  |  | 0.552 |
| walker |  | 9760 | 9 | Code::CodeKey { rung: Body, file: color.go, decl: 3, sub: 0, line: 29 } |  |  | 0.552 |
| walker |  | 9784 | 24 | Code::CodeKey { rung: Doc, file: options.go, decl: 8, sub: 0, line: 84 } |  |  | 0.552 |
| walker |  | 9807 | 23 | Code::CodeKey { rung: Names, file: tty.go, decl: 0, sub: 0, line: 0 } |  |  | 0.552 |
| walker |  | 9826 | 19 | Code::CodeKey { rung: Doc, file: tty.go, decl: 1, sub: 0, line: 130 } |  |  | 0.552 |
| ns | 9876 |  | 360 | Test suite: every test function, and the golden-file fixtures | 5.5 |  | 0.541 |
| walker |  | 9903 | 77 | Code::CodeKey { rung: Names, file: tty_unix.go, decl: 0, sub: 0, line: 0 } |  |  | 0.542 |
| walker |  | 9917 | 14 | Code::CodeKey { rung: Doc, file: tty_unix.go, decl: 3, sub: 0, line: 40 } |  |  | 0.546 |
| ns | 9917 |  | 41 | CI workflow inventory and golden-test fixture directories | 5.6 |  | 0.546 |
| walker |  | 9943 | 26 | Code::CodeKey { rung: Names, file: raw.go, decl: 0, sub: 0, line: 0 } |  |  | 0.547 |
| walker |  | 9956 | 13 | Code::CodeKey { rung: Decl, file: raw.go, decl: 1, sub: 0, line: 5 } |  |  | 0.548 |
| walker |  | 9999 | 43 | Code::CodeKey { rung: Names, file: termcap.go, decl: 0, sub: 0, line: 0 } |  |  | 0.549 |
