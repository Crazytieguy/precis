Score(3000)=0.563 I=0.781 C=0.405 ns_rows≤3K=18/50 grid(1000/1442/2080/3000/4327/6240/9000)=0.584/0.532/0.612/0.563/0.530/0.500/0.517

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| ns | 95 |  | 95 | Package identity: `tea` package doc lede + module path | 1.1 |  | 0.000 |
| ns | 163 |  | 68 | `Model` interface skeleton: `Init` / `Update` / `View` | 1.2 |  | 0.000 |
| walker |  | 202 | 202 | Fs::DirListing { dir: . } |  |  | 0.000 |
| walker |  | 212 | 10 | Fs::DirListing { dir: testdata } |  |  | 0.000 |
| walker |  | 226 | 14 | Fs::DirListing { dir: tutorials } |  |  | 0.000 |
| walker |  | 234 | 8 | Fs::DirListing { dir: tutorials/basics } |  |  | 0.000 |
| walker |  | 242 | 8 | Fs::DirListing { dir: tutorials/commands } |  |  | 0.000 |
| ns | 266 |  | 103 | `Msg` alias and the `Cmd` type | 1.3 |  | 0.000 |
| walker |  | 271 | 29 | GoMod::Identity { file: go.mod } |  |  | 0.043 |
| ns | 338 |  | 72 | Program entry points: `NewProgram`, `Run`, `ProgramOption` | 1.4 |  | 0.038 |
| walker |  | 400 | 129 | Plaintext::Whole { file: Taskfile.yaml } |  |  | 0.039 |
| walker |  | 422 | 22 | Fs::DirListing { dir: testdata/TestClearMsg } |  |  | 0.039 |
| ns | 450 |  | 112 | `Model` method doc comments (refines 1.2) | 1.5 | 1.2 | 0.033 |
| walker |  | 498 | 76 | Markdown::ReadmeHeadline { file: README.md } |  |  | 0.033 |
| walker |  | 510 | 12 | Fs::DirListing { dir: .github } |  |  | 0.033 |
| walker |  | 541 | 31 | Fs::DirListing { dir: .github/workflows } |  |  | 0.033 |
| ns | 652 |  | 202 | Complete root directory listing | 1.6 |  | 0.413 |
| walker |  | 691 | 150 | Code::CodeKey { rung: ModuleDoc, file: tea.go, decl: 0, sub: 0, line: 0 } |  |  | 0.712 |
| walker |  | 735 | 44 | GoMod::Identity { file: tutorials/go.mod } |  |  | 0.712 |
| walker |  | 807 | 72 | Fs::DirListing { dir: testdata/TestViewModel } |  |  | 0.712 |
| ns | 851 |  | 199 | `View` struct: header plus every field name | 1.7 |  | 0.620 |
| ns | 975 |  | 124 | `NewView` / `View.SetContent` constructors | 1.8 | 1.7 | 0.584 |
| walker |  | 1105 | 298 | Code::CodeKey { rung: Names, file: tea.go, decl: 0, sub: 0, line: 0 } |  |  | 0.591 |
| walker |  | 1116 | 11 | Code::CodeKey { rung: Decl, file: tea.go, decl: 13, sub: 0, line: 312 } |  |  | 0.591 |
| walker |  | 1123 | 7 | Code::CodeKey { rung: Doc, file: tea.go, decl: 13, sub: 0, line: 312 } |  |  | 0.591 |
| walker |  | 1132 | 9 | Code::CodeKey { rung: Body, file: tea.go, decl: 9, sub: 0, line: 279 } |  |  | 0.556 |
| ns | 1132 |  | 157 | `Cmd` semantics and the `Quit` command | 1.9 | 1.3 | 0.556 |
| walker |  | 1147 | 15 | Code::CodeKey { rung: Doc, file: tea.go, decl: 10, sub: 0, line: 284 } |  |  | 0.556 |
| walker |  | 1163 | 16 | Code::CodeKey { rung: Doc, file: tea.go, decl: 12, sub: 0, line: 309 } |  |  | 0.556 |
| walker |  | 1180 | 17 | Code::CodeKey { rung: Doc, file: tea.go, decl: 14, sub: 0, line: 321 } |  |  | 0.556 |
| walker |  | 1199 | 19 | Code::CodeKey { rung: Doc, file: tea.go, decl: 2, sub: 0, line: 42 } |  |  | 0.556 |
| walker |  | 1314 | 115 | Code::CodeKey { rung: Decl, file: tea.go, decl: 15, sub: 0, line: 336 } |  |  | 0.556 |
| ns | 1338 |  | 206 | Direct dependency set from `go.mod` | 1.10 |  | 0.532 |
| walker |  | 1339 | 25 | Code::CodeKey { rung: Doc, file: tea.go, decl: 1, sub: 0, line: 39 } |  |  | 0.532 |
| walker |  | 1501 | 162 | Code::CodeKey { rung: Decl, file: tea.go, decl: 5, sub: 0, line: 53 } |  |  | 0.608 |
| walker |  | 1517 | 16 | Code::CodeKey { rung: Doc, file: tea.go, decl: 5, sub: 0, line: 53 } |  |  | 0.657 |
| ns | 1522 |  | 184 | README lede: positioning and shipped feature set | 1.11 |  | 0.638 |
| walker |  | 1551 | 34 | Code::CodeKey { rung: Doc, file: tea.go, decl: 4, sub: 0, line: 50 } |  |  | 0.650 |
| walker |  | 1587 | 36 | Code::CodeKey { rung: Doc, file: tea.go, decl: 3, sub: 0, line: 46 } |  |  | 0.651 |
| ns | 1665 |  | 143 | Sentinel errors returned by `Program.Run` | 2.1 |  | 0.658 |
| ns | 1810 |  | 145 | Every exported `*Program` method (complete roster) | 2.2 | 1.4 | 0.636 |
| walker |  | 1839 | 252 | Code::CodeKey { rung: Decl, file: tea.go, decl: 11, sub: 0, line: 286 } |  |  | 0.638 |
| walker |  | 1888 | 49 | Code::CodeKey { rung: Doc, file: tea.go, decl: 16, sub: 0, line: 349 } |  |  | 0.638 |
| walker |  | 1946 | 58 | Code::CodeKey { rung: Doc, file: tea.go, decl: 15, sub: 0, line: 336 } |  |  | 0.638 |
| ns | 2001 |  | 191 | Every `ProgramOption` constructor (complete roster) | 2.3 |  | 0.612 |
| ns | 2130 |  | 129 | Build/test/lint entry points (`Taskfile.yaml`) | 2.4 |  | 0.633 |
| walker |  | 2307 | 361 | Code::CodeKey { rung: Decl, file: tea.go, decl: 8, sub: 0, line: 241 } |  |  | 0.633 |
| walker |  | 2405 | 98 | Code::CodeKey { rung: Doc, file: tea.go, decl: 6, sub: 0, line: 76 } |  |  | 0.650 |
| ns | 2436 |  | 306 | Event-loop dispatch table: every internally-handled message type | 2.5 |  | 0.601 |
| walker |  | 2512 | 107 | Code::CodeKey { rung: Doc, file: tea.go, decl: 9, sub: 0, line: 279 } |  |  | 0.601 |
| ns | 2682 |  | 246 | Event-loop skeleton: translate → filter → dispatch → update → render | 2.6 | 2.5 | 0.567 |
| walker |  | 2792 | 280 | Code::CodeKey { rung: Names, file: tea.go, decl: 0, sub: 1, line: 0 } |  |  | 0.575 |
| walker |  | 2805 | 13 | Code::CodeKey { rung: Doc, file: tea.go, decl: 31, sub: 0, line: 595 } |  |  | 0.579 |
| walker |  | 2832 | 27 | Code::CodeKey { rung: Decl, file: tea.go, decl: 20, sub: 0, line: 395 } |  |  | 0.579 |
| walker |  | 2849 | 17 | Code::CodeKey { rung: Doc, file: tea.go, decl: 26, sub: 0, line: 564 } |  |  | 0.579 |
| walker |  | 2868 | 19 | Code::CodeKey { rung: Doc, file: tea.go, decl: 24, sub: 0, line: 555 } |  |  | 0.581 |
| walker |  | 2891 | 23 | Code::CodeKey { rung: Doc, file: tea.go, decl: 30, sub: 0, line: 590 } |  |  | 0.582 |
| ns | 2901 |  | 219 | Program control messages: suspend, resume, interrupt | 2.7 | 1.9 | 0.563 |
| walker |  | 3039 | 148 | Code::CodeKey { rung: Decl, file: tea.go, decl: 17, sub: 0, line: 357 } |  |  | 0.563 |
| walker |  | 3053 | 14 | Code::CodeKey { rung: Doc, file: tea.go, decl: 17, sub: 0, line: 357 } |  |  | 0.563 |
| walker |  | 3077 | 24 | Code::CodeKey { rung: Doc, file: tea.go, decl: 18, sub: 0, line: 374 } |  |  | 0.563 |
| ns | 3100 |  | 199 | `Run()`: how input and the TTY are acquired | 2.8 |  | 0.544 |
| walker |  | 3105 | 28 | Code::CodeKey { rung: Doc, file: tea.go, decl: 28, sub: 0, line: 578 } |  |  | 0.550 |
| walker |  | 3137 | 32 | Code::CodeKey { rung: Doc, file: tea.go, decl: 25, sub: 0, line: 561 } |  |  | 0.555 |
| walker |  | 3149 | 12 | Code::CodeKey { rung: Doc, file: tea.go, decl: 33, sub: 0, line: 685 } |  |  | 0.555 |
| walker |  | 3162 | 13 | Code::CodeKey { rung: Doc, file: tea.go, decl: 22, sub: 0, line: 409 } |  |  | 0.555 |
| walker |  | 3244 | 82 | Code::CodeKey { rung: Doc, file: tea.go, decl: 29, sub: 0, line: 586 } |  |  | 0.566 |
| ns | 3288 |  | 188 | `Run()`: renderer choice, colour profile, first three messages | 2.9 | 2.8 | 0.550 |
| walker |  | 3329 | 85 | Code::CodeKey { rung: Doc, file: tea.go, decl: 27, sub: 0, line: 574 } |  |  | 0.564 |
| ns | 3474 |  | 186 | `Run()`: model init, event loop, graceful shutdown | 2.10 | 2.9 | 0.545 |
| walker |  | 3571 | 242 | Code::CodeKey { rung: Names, file: tea.go, decl: 0, sub: 2, line: 0 } |  |  | 0.559 |
| walker |  | 3589 | 18 | Code::CodeKey { rung: Doc, file: tea.go, decl: 44, sub: 0, line: 1209 } |  |  | 0.559 |
| walker |  | 3603 | 14 | Code::CodeKey { rung: Doc, file: tea.go, decl: 36, sub: 0, line: 886 } |  |  | 0.559 |
| walker |  | 3656 | 53 | Code::CodeKey { rung: Doc, file: tea.go, decl: 40, sub: 0, line: 991 } |  |  | 0.559 |
| walker |  | 3672 | 16 | Code::CodeKey { rung: Doc, file: tea.go, decl: 45, sub: 0, line: 1214 } |  |  | 0.559 |
| ns | 3693 |  | 219 | Debug hooks: `TEA_TRACE`, `TEA_DEBUG`, panic recovery | 2.11 |  | 0.548 |
| walker |  | 3729 | 57 | Code::CodeKey { rung: Doc, file: tea.go, decl: 43, sub: 0, line: 1204 } |  |  | 0.548 |
| walker |  | 3746 | 17 | Code::CodeKey { rung: Doc, file: tea.go, decl: 46, sub: 0, line: 1221 } |  |  | 0.548 |
| walker |  | 3904 | 158 | Code::CodeKey { rung: Names, file: tea.go, decl: 0, sub: 3, line: 0 } |  |  | 0.565 |
| ns | 3916 |  | 223 | Frame pacing: the render ticker and the FPS bounds | 2.12 |  | 0.547 |
| walker |  | 3942 | 38 | Code::CodeKey { rung: Doc, file: tea.go, decl: 50, sub: 0, line: 1319 } |  |  | 0.547 |
| walker |  | 3954 | 12 | Code::CodeKey { rung: Doc, file: tea.go, decl: 55, sub: 0, line: 1393 } |  |  | 0.547 |
| walker |  | 4013 | 59 | Code::CodeKey { rung: Doc, file: tea.go, decl: 52, sub: 0, line: 1344 } |  |  | 0.547 |
| walker |  | 4074 | 61 | Code::CodeKey { rung: Doc, file: tea.go, decl: 53, sub: 0, line: 1372 } |  |  | 0.547 |
| walker |  | 4103 | 29 | Code::CodeKey { rung: Doc, file: tea.go, decl: 21, sub: 0, line: 402 } |  |  | 0.547 |
| ns | 4118 |  | 202 | Input event message types: keys, mouse, paste, focus (complete) | 3.1 |  | 0.530 |
| walker |  | 4132 | 29 | Code::CodeKey { rung: Doc, file: tea.go, decl: 47, sub: 0, line: 1241 } |  |  | 0.530 |
| walker |  | 4162 | 30 | Code::CodeKey { rung: Doc, file: tea.go, decl: 34, sub: 0, line: 700 } |  |  | 0.530 |
| walker |  | 4270 | 108 | Code::CodeKey { rung: Doc, file: tea.go, decl: 41, sub: 0, line: 1183 } |  |  | 0.530 |
| ns | 4380 |  | 262 | Terminal report message types: colour, clipboard, size, capability (complete) | 3.2 |  | 0.512 |
| walker |  | 4383 | 113 | Code::CodeKey { rung: Doc, file: tea.go, decl: 19, sub: 0, line: 390 } |  |  | 0.551 |
| walker |  | 4499 | 116 | Code::CodeKey { rung: Doc, file: tea.go, decl: 42, sub: 0, line: 1197 } |  |  | 0.551 |
| ns | 4550 |  | 170 | Command constructors, part 1: lifecycle, batching, timing, output | 3.3 | 2.7 | 0.541 |
| walker |  | 4619 | 120 | Code::CodeKey { rung: Doc, file: tea.go, decl: 54, sub: 0, line: 1386 } |  |  | 0.541 |
| walker |  | 4656 | 37 | Code::CodeKey { rung: Doc, file: tea.go, decl: 35, sub: 0, line: 743 } |  |  | 0.541 |
| walker |  | 4693 | 37 | Code::CodeKey { rung: Doc, file: tea.go, decl: 48, sub: 0, line: 1269 } |  |  | 0.541 |
| ns | 4696 |  | 146 | Command constructors, part 2: terminal queries and clipboard | 3.4 | 3.3 | 0.533 |
| walker |  | 4734 | 41 | Code::CodeKey { rung: Doc, file: tea.go, decl: 20, sub: 0, line: 395 } |  |  | 0.533 |
| walker |  | 4776 | 42 | Code::CodeKey { rung: Doc, file: tea.go, decl: 56, sub: 0, line: 1427 } |  |  | 0.533 |
| walker |  | 4841 | 65 | Markdown::ReadmeHeadline { file: tutorials/basics/README.md } |  |  | 0.533 |
| walker |  | 4887 | 46 | Code::CodeKey { rung: Doc, file: tea.go, decl: 49, sub: 0, line: 1294 } |  |  | 0.533 |
| walker |  | 4959 | 72 | Markdown::ReadmeHeadline { file: tutorials/commands/README.md } |  |  | 0.533 |
| ns | 5037 |  | 341 | `input.go`: the ultraviolet-event → `tea.Msg` translation table | 3.5 |  | 0.511 |
| ns | 5113 |  | 76 | `Key` struct fields | 3.6 |  | 0.505 |
| ns | 5293 |  | 180 | Key message methods: `String`, `Keystroke`, `Key` | 3.7 | 3.6 | 0.499 |
| walker |  | 5475 | 516 | Code::CodeKey { rung: Decl, file: tea.go, decl: 7, sub: 0, line: 84 } |  |  | 0.501 |
| walker |  | 5514 | 39 | Code::CodeKey { rung: Doc, file: tea.go, decl: 7, sub: 0, line: 84 } |  |  | 0.507 |
| ns | 5614 |  | 321 | `key.go` key-code catalog: group headings plus the C0/G0 names | 3.8 |  | 0.487 |
| walker |  | 5770 | 256 | Code::CodeKey { rung: Decl, file: tea.go, decl: 7, sub: 1, line: 84 } |  |  | 0.499 |
| ns | 5797 |  | 183 | Modifier keys: the complete `KeyMod` constant set | 3.9 |  | 0.491 |
| ns | 6023 |  | 226 | `Mouse` struct and the complete mouse-button constant set | 3.10 | 3.1 | 0.480 |
| walker |  | 6049 | 279 | Code::CodeKey { rung: Decl, file: tea.go, decl: 7, sub: 2, line: 84 } |  |  | 0.485 |
| ns | 6200 |  | 177 | `MouseMode` enum: none / cell-motion / all-motion | 3.11 | 1.7 | 0.500 |
| walker |  | 6293 | 244 | Code::CodeKey { rung: Decl, file: tea.go, decl: 7, sub: 3, line: 84 } |  |  | 0.514 |
| ns | 6378 |  | 178 | Cursor value types: `Cursor`, `NewCursor`, `CursorShape` | 3.12 | 1.7 | 0.510 |
| walker |  | 6543 | 250 | Fs::DirListing { dir: examples } |  |  | 0.514 |
| ns | 6566 |  | 188 | Keyboard enhancements: request fields and response predicates | 3.13 | 3.2 | 0.510 |
| ns | 6759 |  | 193 | `Batch` vs `Sequence` semantics and where they execute | 3.14 | 3.3 | 0.504 |
| ns | 6959 |  | 200 | `Exec` / `ExecProcess` and the `ExecCommand` interface | 3.15 | 3.3 | 0.498 |
| ns | 7111 |  | 152 | Logging to a file: `LogToFile` / `LogToFileWith` | 3.16 |  | 0.493 |
| walker |  | 7133 | 590 | Code::CodeKey { rung: Decl, file: tea.go, decl: 23, sub: 0, line: 426 } |  |  | 0.493 |
| walker |  | 7145 | 12 | Code::CodeKey { rung: Doc, file: tea.go, decl: 23, sub: 0, line: 426 } |  |  | 0.493 |
| ns | 7328 |  | 217 | The `renderer` interface: complete method set, and its two implementations | 4.1 |  | 0.485 |
| walker |  | 7378 | 233 | Code::CodeKey { rung: Decl, file: tea.go, decl: 23, sub: 1, line: 426 } |  |  | 0.485 |
| walker |  | 7650 | 272 | Code::CodeKey { rung: Decl, file: tea.go, decl: 23, sub: 2, line: 426 } |  |  | 0.485 |
| ns | 7681 |  | 353 | `cursedRenderer`: every function in the 860-line renderer | 4.2 | 4.1 | 0.474 |
| walker |  | 7908 | 258 | Code::CodeKey { rung: Decl, file: tea.go, decl: 23, sub: 3, line: 426 } |  |  | 0.474 |
| ns | 8039 |  | 358 | `View` fields → ANSI sequences, in `cursedRenderer.start` | 4.3 | 4.2 | 0.464 |
| walker |  | 8154 | 246 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.465 |
| walker |  | 8405 | 251 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: true } |  |  | 0.477 |
| ns | 8420 |  | 381 | Platform matrix: build tags and per-OS terminal functions | 4.4 |  | 0.468 |
| walker |  | 8427 | 22 | Code::CodeKey { rung: Names, file: focus.go, decl: 0, sub: 0, line: 0 } |  |  | 0.468 |
| walker |  | 8451 | 24 | Code::CodeKey { rung: Doc, file: focus.go, decl: 2, sub: 0, line: 9 } |  |  | 0.468 |
| walker |  | 8477 | 26 | Code::CodeKey { rung: Doc, file: focus.go, decl: 1, sub: 0, line: 5 } |  |  | 0.468 |
| ns | 8618 |  | 198 | `tty.go`: terminal acquisition, input loop, resize detection | 4.5 |  | 0.464 |
| walker |  | 8627 | 150 | Code::CodeKey { rung: Decl, file: tea.go, decl: 23, sub: 4, line: 426 } |  |  | 0.464 |
| walker |  | 8677 | 50 | Code::CodeKey { rung: Names, file: screen.go, decl: 0, sub: 0, line: 0 } |  |  | 0.465 |
| walker |  | 8699 | 22 | Code::CodeKey { rung: Decl, file: screen.go, decl: 1, sub: 0, line: 9 } |  |  | 0.465 |
| walker |  | 8749 | 50 | Code::CodeKey { rung: Decl, file: screen.go, decl: 4, sub: 0, line: 62 } |  |  | 0.465 |
| walker |  | 8758 | 9 | Code::CodeKey { rung: Body, file: screen.go, decl: 2, sub: 0, line: 20 } |  |  | 0.465 |
| walker |  | 8829 | 71 | Code::CodeKey { rung: Doc, file: screen.go, decl: 1, sub: 0, line: 9 } |  |  | 0.465 |
| walker |  | 8871 | 42 | Markdown::Section { file: README.md, section_index: 16, keeps_default_concavity: false } |  |  | 0.465 |
| ns | 8882 |  | 264 | `examples/` and `tutorials/` directory listings | 5.1 |  | 0.515 |
| walker |  | 8922 | 51 | Code::CodeKey { rung: Names, file: paste.go, decl: 0, sub: 0, line: 0 } |  |  | 0.517 |
| walker |  | 8934 | 12 | Code::CodeKey { rung: Decl, file: paste.go, decl: 1, sub: 0, line: 5 } |  |  | 0.517 |
| walker |  | 8942 | 8 | Code::CodeKey { rung: Body, file: paste.go, decl: 2, sub: 0, line: 10 } |  |  | 0.517 |
| walker |  | 8954 | 12 | Code::CodeKey { rung: Doc, file: paste.go, decl: 2, sub: 0, line: 10 } |  |  | 0.517 |
| walker |  | 8983 | 29 | Code::CodeKey { rung: Doc, file: paste.go, decl: 4, sub: 0, line: 20 } |  |  | 0.517 |
| walker |  | 9014 | 31 | Code::CodeKey { rung: Doc, file: paste.go, decl: 1, sub: 0, line: 5 } |  |  | 0.517 |
| walker |  | 9045 | 31 | Code::CodeKey { rung: Doc, file: paste.go, decl: 3, sub: 0, line: 16 } |  |  | 0.517 |
| ns | 9062 |  | 180 | `UPGRADE_GUIDE_V2.md`: complete section map | 5.2 |  | 0.511 |
| walker |  | 9071 | 26 | Code::CodeKey { rung: Names, file: raw.go, decl: 0, sub: 0, line: 0 } |  |  | 0.511 |
| walker |  | 9084 | 13 | Code::CodeKey { rung: Decl, file: raw.go, decl: 1, sub: 0, line: 5 } |  |  | 0.512 |
| walker |  | 9110 | 26 | Code::CodeKey { rung: Body, file: raw.go, decl: 2, sub: 0, line: 33 } |  |  | 0.512 |
| walker |  | 9141 | 31 | Code::CodeKey { rung: Doc, file: raw.go, decl: 1, sub: 0, line: 5 } |  |  | 0.512 |
| walker |  | 9290 | 149 | Code::CodeKey { rung: Names, file: clipboard.go, decl: 0, sub: 0, line: 0 } |  |  | 0.514 |
| walker |  | 9311 | 21 | Code::CodeKey { rung: Decl, file: clipboard.go, decl: 1, sub: 0, line: 5 } |  |  | 0.515 |
| walker |  | 9319 | 8 | Code::CodeKey { rung: Body, file: clipboard.go, decl: 2, sub: 0, line: 15 } |  |  | 0.515 |
| walker |  | 9327 | 8 | Code::CodeKey { rung: Body, file: clipboard.go, decl: 3, sub: 0, line: 20 } |  |  | 0.515 |
| walker |  | 9336 | 9 | Code::CodeKey { rung: Body, file: clipboard.go, decl: 7, sub: 0, line: 42 } |  |  | 0.515 |
| ns | 9347 |  | 285 | The v1→v2 migration checklist | 5.3 | 5.2 | 0.511 |
| walker |  | 9351 | 15 | Code::CodeKey { rung: Doc, file: clipboard.go, decl: 3, sub: 0, line: 20 } |  |  | 0.511 |
| walker |  | 9361 | 10 | Code::CodeKey { rung: Body, file: clipboard.go, decl: 11, sub: 0, line: 68 } |  |  | 0.511 |
| walker |  | 9394 | 33 | Code::CodeKey { rung: Doc, file: clipboard.go, decl: 5, sub: 0, line: 30 } |  |  | 0.511 |
| walker |  | 9427 | 33 | Code::CodeKey { rung: Doc, file: clipboard.go, decl: 7, sub: 0, line: 42 } |  |  | 0.511 |
| walker |  | 9463 | 36 | Code::CodeKey { rung: Doc, file: clipboard.go, decl: 1, sub: 0, line: 5 } |  |  | 0.511 |
| ns | 9516 |  | 169 | README section map | 5.4 |  | 0.520 |
| walker |  | 9524 | 61 | Code::CodeKey { rung: Doc, file: clipboard.go, decl: 2, sub: 0, line: 15 } |  |  | 0.520 |
| walker |  | 9585 | 61 | Code::CodeKey { rung: Doc, file: clipboard.go, decl: 9, sub: 0, line: 54 } |  |  | 0.520 |
| walker |  | 9646 | 61 | Code::CodeKey { rung: Doc, file: clipboard.go, decl: 11, sub: 0, line: 68 } |  |  | 0.520 |
| walker |  | 9701 | 55 | Code::CodeKey { rung: Names, file: termcap.go, decl: 0, sub: 0, line: 0 } |  |  | 0.522 |
| walker |  | 9713 | 12 | Code::CodeKey { rung: Decl, file: termcap.go, decl: 3, sub: 0, line: 41 } |  |  | 0.523 |
| walker |  | 9721 | 8 | Code::CodeKey { rung: Body, file: termcap.go, decl: 4, sub: 0, line: 46 } |  |  | 0.523 |
| walker |  | 9733 | 12 | Code::CodeKey { rung: Doc, file: termcap.go, decl: 4, sub: 0, line: 46 } |  |  | 0.523 |
| walker |  | 9759 | 26 | Code::CodeKey { rung: Body, file: termcap.go, decl: 2, sub: 0, line: 30 } |  |  | 0.523 |
| walker |  | 9814 | 55 | Code::CodeKey { rung: Names, file: xterm.go, decl: 0, sub: 0, line: 0 } |  |  | 0.525 |
| walker |  | 9826 | 12 | Code::CodeKey { rung: Decl, file: xterm.go, decl: 1, sub: 0, line: 4 } |  |  | 0.526 |
| walker |  | 9834 | 8 | Code::CodeKey { rung: Body, file: xterm.go, decl: 2, sub: 0, line: 9 } |  |  | 0.526 |
| walker |  | 9842 | 8 | Code::CodeKey { rung: Body, file: xterm.go, decl: 4, sub: 0, line: 20 } |  |  | 0.526 |
| walker |  | 9854 | 12 | Code::CodeKey { rung: Doc, file: xterm.go, decl: 2, sub: 0, line: 9 } |  |  | 0.526 |
| walker |  | 9871 | 17 | Code::CodeKey { rung: Doc, file: xterm.go, decl: 1, sub: 0, line: 4 } |  |  | 0.526 |
| ns | 9876 |  | 360 | Test suite: every test function, and the golden-file fixtures | 5.5 |  | 0.516 |
| walker |  | 9914 | 43 | Code::CodeKey { rung: Doc, file: xterm.go, decl: 4, sub: 0, line: 20 } |  |  | 0.516 |
| ns | 9917 |  | 41 | CI workflow inventory and golden-test fixture directories | 5.6 |  | 0.521 |
