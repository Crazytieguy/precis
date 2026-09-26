Score(3000)=0.544 I=0.766 C=0.387 ns_rows≤3K=18/50 grid(1000/1442/2080/3000/4327/6240/9000)=0.591/0.607/0.544/0.544/0.476/0.463/0.545

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| ns | 95 |  | 95 | Package identity: `tea` package doc lede + module path | 1.1 |  | 0.000 |
| ns | 163 |  | 68 | `Model` interface skeleton: `Init` / `Update` / `View` | 1.2 |  | 0.000 |
| walker |  | 202 | 202 | Fs::DirListing { dir: . } |  |  | 0.000 |
| ns | 266 |  | 103 | `Msg` alias and the `Cmd` type | 1.3 |  | 0.000 |
| walker |  | 278 | 76 | Markdown::ReadmeHeadline { file: README.md } |  |  | 0.000 |
| walker |  | 292 | 14 | Fs::DirListing { dir: tutorials } |  |  | 0.000 |
| walker |  | 321 | 29 | GoMod::Identity { file: go.mod } |  |  | 0.043 |
| walker |  | 329 | 8 | Fs::DirListing { dir: tutorials/basics } |  |  | 0.043 |
| walker |  | 337 | 8 | Fs::DirListing { dir: tutorials/commands } |  |  | 0.043 |
| ns | 338 |  | 72 | Program entry points: `NewProgram`, `Run`, `ProgramOption` | 1.4 |  | 0.038 |
| ns | 450 |  | 112 | `Model` method doc comments (refines 1.2) | 1.5 | 1.2 | 0.032 |
| walker |  | 466 | 129 | Plaintext::Whole { file: Taskfile.yaml } |  |  | 0.033 |
| walker |  | 510 | 44 | GoMod::Identity { file: tutorials/go.mod } |  |  | 0.033 |
| walker |  | 520 | 10 | Fs::DirListing { dir: testdata } |  |  | 0.033 |
| walker |  | 532 | 12 | Fs::DirListing { dir: .github } |  |  | 0.033 |
| walker |  | 563 | 31 | Fs::DirListing { dir: .github/workflows } |  |  | 0.033 |
| ns | 652 |  | 202 | Complete root directory listing | 1.6 |  | 0.413 |
| walker |  | 713 | 150 | Code::CodeKey { rung: ModuleDoc, file: tea.go, decl: 0, sub: 0, line: 0 } |  |  | 0.712 |
| ns | 851 |  | 199 | `View` struct: header plus every field name | 1.7 |  | 0.620 |
| ns | 975 |  | 124 | `NewView` / `View.SetContent` constructors | 1.8 | 1.7 | 0.584 |
| walker |  | 978 | 265 | Code::CodeKey { rung: Names, file: tea.go, decl: 0, sub: 0, line: 0 } |  |  | 0.591 |
| walker |  | 983 | 5 | Code::CodeKey { rung: Decl, file: tea.go, decl: 13, sub: 0, line: 312 } |  |  | 0.591 |
| ns | 1132 |  | 157 | `Cmd` semantics and the `Quit` command | 1.9 | 1.3 | 0.554 |
| walker |  | 1145 | 162 | Code::CodeKey { rung: Decl, file: tea.go, decl: 5, sub: 0, line: 53 } |  |  | 0.633 |
| walker |  | 1152 | 7 | Code::CodeKey { rung: Doc, file: tea.go, decl: 13, sub: 0, line: 312 } |  |  | 0.633 |
| ns | 1338 |  | 206 | Direct dependency set from `go.mod` | 1.10 |  | 0.605 |
| walker |  | 1400 | 248 | Code::CodeKey { rung: Decl, file: tea.go, decl: 11, sub: 0, line: 286 } |  |  | 0.607 |
| ns | 1522 |  | 184 | README lede: positioning and shipped feature set | 1.11 |  | 0.589 |
| ns | 1665 |  | 143 | Sentinel errors returned by `Program.Run` | 2.1 |  | 0.574 |
| walker |  | 1761 | 361 | Code::CodeKey { rung: Decl, file: tea.go, decl: 8, sub: 0, line: 241 } |  |  | 0.574 |
| walker |  | 1775 | 14 | Code::CodeKey { rung: Doc, file: tea.go, decl: 12, sub: 0, line: 309 } |  |  | 0.574 |
| ns | 1810 |  | 145 | Every exported `*Program` method (complete roster) | 2.2 | 1.4 | 0.555 |
| walker |  | 1995 | 220 | Code::CodeKey { rung: Names, file: tea.go, decl: 0, sub: 1, line: 0 } |  |  | 0.568 |
| ns | 2001 |  | 191 | Every `ProgramOption` constructor (complete roster) | 2.3 |  | 0.544 |
| walker |  | 2110 | 115 | Code::CodeKey { rung: Decl, file: tea.go, decl: 15, sub: 0, line: 336 } |  |  | 0.544 |
| ns | 2130 |  | 129 | Build/test/lint entry points (`Taskfile.yaml`) | 2.4 |  | 0.569 |
| walker |  | 2258 | 148 | Code::CodeKey { rung: Decl, file: tea.go, decl: 17, sub: 0, line: 357 } |  |  | 0.569 |
| walker |  | 2271 | 13 | Code::CodeKey { rung: Doc, file: tea.go, decl: 28, sub: 0, line: 595 } |  |  | 0.577 |
| walker |  | 2416 | 145 | Code::CodeKey { rung: Names, file: tea.go, decl: 0, sub: 2, line: 0 } |  |  | 0.610 |
| walker |  | 2430 | 14 | Code::CodeKey { rung: Doc, file: tea.go, decl: 17, sub: 0, line: 357 } |  |  | 0.610 |
| ns | 2436 |  | 306 | Event-loop dispatch table: every internally-handled message type | 2.5 |  | 0.564 |
| walker |  | 2445 | 15 | Code::CodeKey { rung: Doc, file: tea.go, decl: 10, sub: 0, line: 284 } |  |  | 0.564 |
| walker |  | 2633 | 188 | Code::CodeKey { rung: Decl, file: tea.go, decl: 20, sub: 0, line: 426 } |  |  | 0.564 |
| walker |  | 2645 | 12 | Code::CodeKey { rung: Doc, file: tea.go, decl: 20, sub: 0, line: 426 } |  |  | 0.564 |
| walker |  | 2661 | 16 | Code::CodeKey { rung: Doc, file: tea.go, decl: 5, sub: 0, line: 53 } |  |  | 0.604 |
| ns | 2682 |  | 246 | Event-loop skeleton: translate → filter → dispatch → update → render | 2.6 | 2.5 | 0.569 |
| ns | 2901 |  | 219 | Program control messages: suspend, resume, interrupt | 2.7 | 1.9 | 0.544 |
| walker |  | 3053 | 392 | Code::CodeKey { rung: Decl, file: tea.go, decl: 20, sub: 1, line: 426 } |  |  | 0.544 |
| ns | 3100 |  | 199 | `Run()`: how input and the TTY are acquired | 2.8 |  | 0.526 |
| walker |  | 3241 | 188 | Code::CodeKey { rung: Decl, file: tea.go, decl: 20, sub: 2, line: 426 } |  |  | 0.526 |
| walker |  | 3258 | 17 | Code::CodeKey { rung: Doc, file: tea.go, decl: 14, sub: 0, line: 321 } |  |  | 0.526 |
| walker |  | 3275 | 17 | Code::CodeKey { rung: Doc, file: tea.go, decl: 23, sub: 0, line: 564 } |  |  | 0.528 |
| ns | 3288 |  | 188 | `Run()`: renderer choice, colour profile, first three messages | 2.9 | 2.8 | 0.513 |
| walker |  | 3473 | 198 | Code::CodeKey { rung: Decl, file: tea.go, decl: 20, sub: 3, line: 426 } |  |  | 0.513 |
| ns | 3474 |  | 186 | `Run()`: model init, event loop, graceful shutdown | 2.10 | 2.9 | 0.496 |
| walker |  | 3491 | 18 | Code::CodeKey { rung: Doc, file: tea.go, decl: 33, sub: 0, line: 1209 } |  |  | 0.496 |
| ns | 3693 |  | 219 | Debug hooks: `TEA_TRACE`, `TEA_DEBUG`, panic recovery | 2.11 |  | 0.486 |
| walker |  | 3719 | 228 | Code::CodeKey { rung: Decl, file: tea.go, decl: 20, sub: 4, line: 426 } |  |  | 0.486 |
| ns | 3916 |  | 223 | Frame pacing: the render ticker and the FPS bounds | 2.12 |  | 0.470 |
| walker |  | 4030 | 311 | Code::CodeKey { rung: Decl, file: tea.go, decl: 20, sub: 5, line: 426 } |  |  | 0.470 |
| walker |  | 4047 | 17 | Code::CodeKey { rung: Doc, file: tea.go, decl: 21, sub: 0, line: 555 } |  |  | 0.471 |
| walker |  | 4066 | 19 | Code::CodeKey { rung: Doc, file: tea.go, decl: 2, sub: 0, line: 42 } |  |  | 0.474 |
| walker |  | 4084 | 18 | Fs::DirListing { dir: .github/ISSUE_TEMPLATE } |  |  | 0.474 |
| walker |  | 4107 | 23 | Code::CodeKey { rung: Doc, file: tea.go, decl: 27, sub: 0, line: 590 } |  |  | 0.478 |
| ns | 4118 |  | 202 | Input event message types: keys, mouse, paste, focus (complete) | 3.1 |  | 0.463 |
| walker |  | 4157 | 50 | GoMod::File { file: tutorials/go.mod } |  |  | 0.463 |
| walker |  | 4181 | 24 | Code::CodeKey { rung: Doc, file: tea.go, decl: 18, sub: 0, line: 374 } |  |  | 0.463 |
| walker |  | 4206 | 25 | Code::CodeKey { rung: Doc, file: tea.go, decl: 1, sub: 0, line: 39 } |  |  | 0.467 |
| walker |  | 4234 | 28 | Code::CodeKey { rung: Doc, file: tea.go, decl: 25, sub: 0, line: 578 } |  |  | 0.472 |
| walker |  | 4266 | 32 | Code::CodeKey { rung: Doc, file: tea.go, decl: 22, sub: 0, line: 561 } |  |  | 0.476 |
| ns | 4380 |  | 262 | Terminal report message types: colour, clipboard, size, capability (complete) | 3.2 |  | 0.460 |
| walker |  | 4516 | 250 | Fs::DirListing { dir: examples } |  |  | 0.465 |
| ns | 4550 |  | 170 | Command constructors, part 1: lifecycle, batching, timing, output | 3.3 | 2.7 | 0.456 |
| ns | 4696 |  | 146 | Command constructors, part 2: terminal queries and clipboard | 3.4 | 3.3 | 0.449 |
| walker |  | 4762 | 246 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.450 |
| walker |  | 4796 | 34 | Code::CodeKey { rung: Doc, file: tea.go, decl: 4, sub: 0, line: 50 } |  |  | 0.460 |
| walker |  | 4832 | 36 | Code::CodeKey { rung: Doc, file: tea.go, decl: 3, sub: 0, line: 46 } |  |  | 0.467 |
| walker |  | 4870 | 38 | Code::CodeKey { rung: Doc, file: tea.go, decl: 34, sub: 0, line: 1319 } |  |  | 0.467 |
| ns | 5037 |  | 341 | `input.go`: the ultraviolet-event → `tea.Msg` translation table | 3.5 |  | 0.448 |
| ns | 5113 |  | 76 | `Key` struct fields | 3.6 |  | 0.443 |
| ns | 5293 |  | 180 | Key message methods: `String`, `Keystroke`, `Key` | 3.7 | 3.6 | 0.437 |
| walker |  | 5379 | 509 | Code::CodeKey { rung: Decl, file: tea.go, decl: 7, sub: 0, line: 84 } |  |  | 0.439 |
| walker |  | 5418 | 39 | Code::CodeKey { rung: Doc, file: tea.go, decl: 7, sub: 0, line: 84 } |  |  | 0.445 |
| ns | 5614 |  | 321 | `key.go` key-code catalog: group headings plus the C0/G0 names | 3.8 |  | 0.428 |
| walker |  | 5621 | 203 | Code::CodeKey { rung: Decl, file: tea.go, decl: 7, sub: 1, line: 84 } |  |  | 0.437 |
| ns | 5797 |  | 183 | Modifier keys: the complete `KeyMod` constant set | 3.9 |  | 0.429 |
| walker |  | 5831 | 210 | Code::CodeKey { rung: Decl, file: tea.go, decl: 7, sub: 2, line: 84 } |  |  | 0.436 |
| ns | 6023 |  | 226 | `Mouse` struct and the complete mouse-button constant set | 3.10 | 3.1 | 0.427 |
| walker |  | 6046 | 215 | Code::CodeKey { rung: Decl, file: tea.go, decl: 7, sub: 3, line: 84 } |  |  | 0.437 |
| walker |  | 6199 | 153 | Code::CodeKey { rung: Decl, file: tea.go, decl: 7, sub: 4, line: 84 } |  |  | 0.445 |
| ns | 6200 |  | 177 | `MouseMode` enum: none / cell-motion / all-motion | 3.11 | 1.7 | 0.463 |
| walker |  | 6248 | 49 | Code::CodeKey { rung: Doc, file: tea.go, decl: 16, sub: 0, line: 349 } |  |  | 0.463 |
| ns | 6378 |  | 178 | Cursor value types: `Cursor`, `NewCursor`, `CursorShape` | 3.12 | 1.7 | 0.460 |
| walker |  | 6494 | 246 | GoMod::File { file: go.mod } |  |  | 0.478 |
| walker |  | 6547 | 53 | Code::CodeKey { rung: Doc, file: tea.go, decl: 29, sub: 0, line: 991 } |  |  | 0.478 |
| ns | 6566 |  | 188 | Keyboard enhancements: request fields and response predicates | 3.13 | 3.2 | 0.474 |
| walker |  | 6723 | 176 | Code::CodeKey { rung: Names, file: cursed_renderer.go, decl: 0, sub: 0, line: 0 } |  |  | 0.474 |
| ns | 6759 |  | 193 | `Batch` vs `Sequence` semantics and where they execute | 3.14 | 3.3 | 0.469 |
| walker |  | 6946 | 223 | Code::CodeKey { rung: Decl, file: cursed_renderer.go, decl: 1, sub: 0, line: 18 } |  |  | 0.469 |
| walker |  | 6955 | 9 | Code::CodeKey { rung: Doc, file: cursed_renderer.go, decl: 6, sub: 0, line: 75 } |  |  | 0.469 |
| ns | 6959 |  | 200 | `Exec` / `ExecProcess` and the `ExecCommand` interface | 3.15 | 3.3 | 0.463 |
| walker |  | 6964 | 9 | Code::CodeKey { rung: Doc, file: cursed_renderer.go, decl: 7, sub: 0, line: 143 } |  |  | 0.463 |
| walker |  | 6974 | 10 | Code::CodeKey { rung: Doc, file: cursed_renderer.go, decl: 8, sub: 0, line: 249 } |  |  | 0.463 |
| walker |  | 6988 | 14 | Code::CodeKey { rung: Doc, file: cursed_renderer.go, decl: 4, sub: 0, line: 52 } |  |  | 0.463 |
| walker |  | 7003 | 15 | Code::CodeKey { rung: Doc, file: cursed_renderer.go, decl: 5, sub: 0, line: 59 } |  |  | 0.463 |
| ns | 7111 |  | 152 | Logging to a file: `LogToFile` / `LogToFileWith` | 3.16 |  | 0.459 |
| walker |  | 7273 | 270 | Code::CodeKey { rung: Names, file: nil_renderer.go, decl: 0, sub: 0, line: 0 } |  |  | 0.459 |
| walker |  | 7280 | 7 | Code::CodeKey { rung: Doc, file: nil_renderer.go, decl: 3, sub: 0, line: 15 } |  |  | 0.459 |
| walker |  | 7287 | 7 | Code::CodeKey { rung: Doc, file: nil_renderer.go, decl: 6, sub: 0, line: 24 } |  |  | 0.459 |
| walker |  | 7295 | 8 | Code::CodeKey { rung: Doc, file: nil_renderer.go, decl: 4, sub: 0, line: 18 } |  |  | 0.459 |
| walker |  | 7303 | 8 | Code::CodeKey { rung: Doc, file: nil_renderer.go, decl: 5, sub: 0, line: 21 } |  |  | 0.459 |
| walker |  | 7312 | 9 | Code::CodeKey { rung: Doc, file: nil_renderer.go, decl: 7, sub: 0, line: 27 } |  |  | 0.459 |
| walker |  | 7321 | 9 | Code::CodeKey { rung: Doc, file: nil_renderer.go, decl: 8, sub: 0, line: 30 } |  |  | 0.459 |
| ns | 7328 |  | 217 | The `renderer` interface: complete method set, and its two implementations | 4.1 |  | 0.452 |
| walker |  | 7330 | 9 | Code::CodeKey { rung: Doc, file: nil_renderer.go, decl: 9, sub: 0, line: 33 } |  |  | 0.452 |
| walker |  | 7339 | 9 | Code::CodeKey { rung: Doc, file: nil_renderer.go, decl: 10, sub: 0, line: 36 } |  |  | 0.452 |
| walker |  | 7348 | 9 | Code::CodeKey { rung: Doc, file: nil_renderer.go, decl: 11, sub: 0, line: 39 } |  |  | 0.452 |
| walker |  | 7358 | 10 | Code::CodeKey { rung: Doc, file: nil_renderer.go, decl: 12, sub: 0, line: 42 } |  |  | 0.452 |
| walker |  | 7368 | 10 | Code::CodeKey { rung: Doc, file: nil_renderer.go, decl: 15, sub: 0, line: 51 } |  |  | 0.452 |
| walker |  | 7379 | 11 | Code::CodeKey { rung: Doc, file: nil_renderer.go, decl: 14, sub: 0, line: 48 } |  |  | 0.452 |
| walker |  | 7391 | 12 | Code::CodeKey { rung: Doc, file: nil_renderer.go, decl: 13, sub: 0, line: 45 } |  |  | 0.452 |
| walker |  | 7398 | 7 | Code::CodeKey { rung: Body, file: nil_renderer.go, decl: 15, sub: 0, line: 51 } |  |  | 0.452 |
| walker |  | 7433 | 35 | Code::CodeKey { rung: Names, file: renderer.go, decl: 0, sub: 0, line: 0 } |  |  | 0.453 |
| walker |  | 7532 | 99 | Code::CodeKey { rung: Names, file: commands.go, decl: 0, sub: 0, line: 0 } |  |  | 0.462 |
| walker |  | 7541 | 9 | Code::CodeKey { rung: Body, file: commands.go, decl: 6, sub: 0, line: 173 } |  |  | 0.462 |
| walker |  | 7579 | 38 | Code::CodeKey { rung: Names, file: screen.go, decl: 0, sub: 0, line: 0 } |  |  | 0.464 |
| walker |  | 7601 | 22 | Code::CodeKey { rung: Decl, file: screen.go, decl: 1, sub: 0, line: 9 } |  |  | 0.465 |
| walker |  | 7651 | 50 | Code::CodeKey { rung: Decl, file: screen.go, decl: 3, sub: 0, line: 62 } |  |  | 0.465 |
| walker |  | 7660 | 9 | Code::CodeKey { rung: Body, file: screen.go, decl: 2, sub: 0, line: 20 } |  |  | 0.465 |
| walker |  | 7675 | 15 | Code::CodeKey { rung: Body, file: commands.go, decl: 1, sub: 0, line: 15 } |  |  | 0.465 |
| ns | 7681 |  | 353 | `cursedRenderer`: every function in the 860-line renderer | 4.2 | 4.1 | 0.457 |
| walker |  | 7690 | 15 | Code::CodeKey { rung: Body, file: commands.go, decl: 3, sub: 0, line: 25 } |  |  | 0.457 |
| walker |  | 7867 | 177 | Code::CodeKey { rung: Names, file: cursed_renderer.go, decl: 0, sub: 1, line: 0 } |  |  | 0.468 |
| walker |  | 7876 | 9 | Code::CodeKey { rung: Doc, file: cursed_renderer.go, decl: 9, sub: 0, line: 257 } |  |  | 0.468 |
| walker |  | 7885 | 9 | Code::CodeKey { rung: Doc, file: cursed_renderer.go, decl: 10, sub: 0, line: 579 } |  |  | 0.468 |
| walker |  | 7894 | 9 | Code::CodeKey { rung: Doc, file: cursed_renderer.go, decl: 11, sub: 0, line: 587 } |  |  | 0.468 |
| walker |  | 7903 | 9 | Code::CodeKey { rung: Doc, file: cursed_renderer.go, decl: 14, sub: 0, line: 619 } |  |  | 0.468 |
| walker |  | 7913 | 10 | Code::CodeKey { rung: Doc, file: cursed_renderer.go, decl: 15, sub: 0, line: 634 } |  |  | 0.468 |
| walker |  | 7924 | 11 | Code::CodeKey { rung: Doc, file: cursed_renderer.go, decl: 13, sub: 0, line: 611 } |  |  | 0.468 |
| ns | 8039 |  | 358 | `View` fields → ANSI sequences, in `cursedRenderer.start` | 4.3 | 4.2 | 0.458 |
| walker |  | 8110 | 186 | Code::CodeKey { rung: Names, file: cursed_renderer.go, decl: 0, sub: 2, line: 0 } |  |  | 0.476 |
| walker |  | 8120 | 10 | Code::CodeKey { rung: Doc, file: cursed_renderer.go, decl: 22, sub: 0, line: 707 } |  |  | 0.476 |
| walker |  | 8130 | 10 | Code::CodeKey { rung: Doc, file: cursed_renderer.go, decl: 23, sub: 0, line: 766 } |  |  | 0.476 |
| walker |  | 8141 | 11 | Code::CodeKey { rung: Doc, file: cursed_renderer.go, decl: 21, sub: 0, line: 690 } |  |  | 0.476 |
| walker |  | 8153 | 12 | Code::CodeKey { rung: Doc, file: cursed_renderer.go, decl: 20, sub: 0, line: 683 } |  |  | 0.476 |
| walker |  | 8167 | 14 | Code::CodeKey { rung: Doc, file: cursed_renderer.go, decl: 19, sub: 0, line: 674 } |  |  | 0.476 |
| walker |  | 8224 | 57 | Code::CodeKey { rung: Doc, file: tea.go, decl: 32, sub: 0, line: 1204 } |  |  | 0.476 |
| walker |  | 8282 | 58 | Code::CodeKey { rung: Doc, file: tea.go, decl: 15, sub: 0, line: 336 } |  |  | 0.476 |
| walker |  | 8364 | 82 | Markdown::Section { file: README.md, section_index: 16, keeps_default_concavity: false } |  |  | 0.476 |
| walker |  | 8387 | 23 | Code::CodeKey { rung: Names, file: input.go, decl: 0, sub: 0, line: 0 } |  |  | 0.476 |
| ns | 8420 |  | 381 | Platform matrix: build tags and per-OS terminal functions | 4.4 |  | 0.467 |
| walker |  | 8593 | 206 | Code::CodeKey { rung: Names, file: options.go, decl: 0, sub: 0, line: 0 } |  |  | 0.489 |
| ns | 8618 |  | 198 | `tty.go`: terminal acquisition, input loop, resize detection | 4.5 |  | 0.485 |
| ns | 8882 |  | 264 | `examples/` and `tutorials/` directory listings | 5.1 |  | 0.529 |
| walker |  | 9001 | 408 | Code::CodeKey { rung: Names, file: mouse.go, decl: 0, sub: 0, line: 0 } |  |  | 0.545 |
| walker |  | 9006 | 5 | Code::CodeKey { rung: Decl, file: mouse.go, decl: 2, sub: 0, line: 29 } |  |  | 0.546 |
| walker |  | 9039 | 33 | Code::CodeKey { rung: Decl, file: mouse.go, decl: 4, sub: 0, line: 71 } |  |  | 0.550 |
| ns | 9062 |  | 180 | `UPGRADE_GUIDE_V2.md`: complete section map | 5.2 |  | 0.544 |
| walker |  | 9079 | 40 | Code::CodeKey { rung: Decl, file: mouse.go, decl: 3, sub: 0, line: 46 } |  |  | 0.545 |
| walker |  | 9092 | 13 | Code::CodeKey { rung: Doc, file: mouse.go, decl: 5, sub: 0, line: 78 } |  |  | 0.545 |
| walker |  | 9106 | 14 | Code::CodeKey { rung: Doc, file: mouse.go, decl: 7, sub: 0, line: 86 } |  |  | 0.545 |
| walker |  | 9120 | 14 | Code::CodeKey { rung: Doc, file: mouse.go, decl: 10, sub: 0, line: 101 } |  |  | 0.545 |
| walker |  | 9134 | 14 | Code::CodeKey { rung: Doc, file: mouse.go, decl: 13, sub: 0, line: 116 } |  |  | 0.545 |
| walker |  | 9148 | 14 | Code::CodeKey { rung: Doc, file: mouse.go, decl: 15, sub: 0, line: 128 } |  |  | 0.545 |
| walker |  | 9162 | 14 | Code::CodeKey { rung: Doc, file: mouse.go, decl: 16, sub: 0, line: 131 } |  |  | 0.545 |
| walker |  | 9177 | 15 | Code::CodeKey { rung: Doc, file: mouse.go, decl: 6, sub: 0, line: 83 } |  |  | 0.545 |
| walker |  | 9192 | 15 | Code::CodeKey { rung: Doc, file: mouse.go, decl: 9, sub: 0, line: 98 } |  |  | 0.545 |
| walker |  | 9207 | 15 | Code::CodeKey { rung: Doc, file: mouse.go, decl: 12, sub: 0, line: 113 } |  |  | 0.545 |
| ns | 9347 |  | 285 | The v1→v2 migration checklist | 5.3 | 5.2 | 0.541 |
| walker |  | 9401 | 194 | Code::CodeKey { rung: Names, file: color.go, decl: 0, sub: 0, line: 0 } |  |  | 0.544 |
| walker |  | 9413 | 12 | Code::CodeKey { rung: Doc, file: color.go, decl: 5, sub: 0, line: 39 } |  |  | 0.544 |
| walker |  | 9425 | 12 | Code::CodeKey { rung: Doc, file: color.go, decl: 8, sub: 0, line: 70 } |  |  | 0.544 |
| walker |  | 9437 | 12 | Code::CodeKey { rung: Doc, file: color.go, decl: 11, sub: 0, line: 84 } |  |  | 0.544 |
| walker |  | 9451 | 14 | Code::CodeKey { rung: Doc, file: color.go, decl: 6, sub: 0, line: 44 } |  |  | 0.544 |
| walker |  | 9465 | 14 | Code::CodeKey { rung: Doc, file: color.go, decl: 9, sub: 0, line: 75 } |  |  | 0.544 |
| walker |  | 9479 | 14 | Code::CodeKey { rung: Doc, file: color.go, decl: 12, sub: 0, line: 89 } |  |  | 0.544 |
| walker |  | 9488 | 9 | Code::CodeKey { rung: Body, file: color.go, decl: 1, sub: 0, line: 13 } |  |  | 0.544 |
| walker |  | 9506 | 18 | Code::CodeKey { rung: Doc, file: color.go, decl: 1, sub: 0, line: 13 } |  |  | 0.544 |
| ns | 9516 |  | 169 | README section map | 5.4 |  | 0.552 |
| walker |  | 9524 | 18 | Code::CodeKey { rung: Doc, file: color.go, decl: 2, sub: 0, line: 21 } |  |  | 0.552 |
| walker |  | 9542 | 18 | Code::CodeKey { rung: Doc, file: color.go, decl: 3, sub: 0, line: 29 } |  |  | 0.552 |
| walker |  | 9560 | 18 | Code::CodeKey { rung: Doc, file: input.go, decl: 1, sub: 0, line: 8 } |  |  | 0.552 |
| walker |  | 9578 | 18 | Code::CodeKey { rung: Doc, file: mouse.go, decl: 1, sub: 0, line: 10 } |  |  | 0.552 |
| walker |  | 9587 | 9 | Code::CodeKey { rung: Body, file: color.go, decl: 2, sub: 0, line: 21 } |  |  | 0.552 |
| walker |  | 9596 | 9 | Code::CodeKey { rung: Body, file: color.go, decl: 3, sub: 0, line: 29 } |  |  | 0.552 |
| walker |  | 9620 | 24 | Code::CodeKey { rung: Doc, file: options.go, decl: 8, sub: 0, line: 84 } |  |  | 0.552 |
| walker |  | 9643 | 23 | Code::CodeKey { rung: Names, file: tty.go, decl: 0, sub: 0, line: 0 } |  |  | 0.552 |
| walker |  | 9662 | 19 | Code::CodeKey { rung: Doc, file: tty.go, decl: 1, sub: 0, line: 130 } |  |  | 0.552 |
| walker |  | 9739 | 77 | Code::CodeKey { rung: Names, file: tty_unix.go, decl: 0, sub: 0, line: 0 } |  |  | 0.553 |
| walker |  | 9753 | 14 | Code::CodeKey { rung: Doc, file: tty_unix.go, decl: 3, sub: 0, line: 40 } |  |  | 0.553 |
| walker |  | 9779 | 26 | Code::CodeKey { rung: Names, file: raw.go, decl: 0, sub: 0, line: 0 } |  |  | 0.554 |
| walker |  | 9792 | 13 | Code::CodeKey { rung: Decl, file: raw.go, decl: 1, sub: 0, line: 5 } |  |  | 0.554 |
| walker |  | 9835 | 43 | Code::CodeKey { rung: Names, file: termcap.go, decl: 0, sub: 0, line: 0 } |  |  | 0.556 |
| walker |  | 9847 | 12 | Code::CodeKey { rung: Decl, file: termcap.go, decl: 2, sub: 0, line: 41 } |  |  | 0.557 |
| walker |  | 9855 | 8 | Code::CodeKey { rung: Body, file: termcap.go, decl: 3, sub: 0, line: 46 } |  |  | 0.557 |
| walker |  | 9867 | 12 | Code::CodeKey { rung: Doc, file: termcap.go, decl: 3, sub: 0, line: 46 } |  |  | 0.557 |
| ns | 9876 |  | 360 | Test suite: every test function, and the golden-file fixtures | 5.5 |  | 0.546 |
| walker |  | 9890 | 23 | Code::CodeKey { rung: Names, file: key.go, decl: 0, sub: 0, line: 0 } |  |  | 0.546 |
| ns | 9917 |  | 41 | CI workflow inventory and golden-test fixture directories | 5.6 |  | 0.550 |
