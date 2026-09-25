Score(3000)=0.556 I=0.779 C=0.397 ns_rows≤3K=18/50 grid(1000/1442/2080/3000/4327/6240/9000)=0.584/0.532/0.612/0.556/0.530/0.501/0.534

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
| walker |  | 585 | 44 | GoMod::Identity { file: tutorials/go.mod } |  |  | 0.033 |
| ns | 652 |  | 202 | Complete root directory listing | 1.6 |  | 0.413 |
| walker |  | 657 | 72 | Fs::DirListing { dir: testdata/TestViewModel } |  |  | 0.413 |
| walker |  | 807 | 150 | Code::CodeKey { rung: ModuleDoc, file: tea.go, decl: 0, sub: 0, line: 0 } |  |  | 0.712 |
| ns | 851 |  | 199 | `View` struct: header plus every field name | 1.7 |  | 0.620 |
| walker |  | 872 | 65 | Markdown::ReadmeHeadline { file: tutorials/basics/README.md } |  |  | 0.620 |
| walker |  | 944 | 72 | Markdown::ReadmeHeadline { file: tutorials/commands/README.md } |  |  | 0.620 |
| ns | 975 |  | 124 | `NewView` / `View.SetContent` constructors | 1.8 | 1.7 | 0.584 |
| ns | 1132 |  | 157 | `Cmd` semantics and the `Quit` command | 1.9 | 1.3 | 0.547 |
| walker |  | 1242 | 298 | Code::CodeKey { rung: Names, file: tea.go, decl: 0, sub: 0, line: 0 } |  |  | 0.554 |
| walker |  | 1253 | 11 | Code::CodeKey { rung: Decl, file: tea.go, decl: 13, sub: 0, line: 312 } |  |  | 0.554 |
| walker |  | 1260 | 7 | Code::CodeKey { rung: Doc, file: tea.go, decl: 13, sub: 0, line: 312 } |  |  | 0.554 |
| walker |  | 1269 | 9 | Code::CodeKey { rung: Body, file: tea.go, decl: 9, sub: 0, line: 279 } |  |  | 0.556 |
| walker |  | 1284 | 15 | Code::CodeKey { rung: Doc, file: tea.go, decl: 10, sub: 0, line: 284 } |  |  | 0.556 |
| walker |  | 1300 | 16 | Code::CodeKey { rung: Doc, file: tea.go, decl: 12, sub: 0, line: 309 } |  |  | 0.556 |
| walker |  | 1317 | 17 | Code::CodeKey { rung: Doc, file: tea.go, decl: 14, sub: 0, line: 321 } |  |  | 0.556 |
| walker |  | 1336 | 19 | Code::CodeKey { rung: Doc, file: tea.go, decl: 2, sub: 0, line: 42 } |  |  | 0.556 |
| ns | 1338 |  | 206 | Direct dependency set from `go.mod` | 1.10 |  | 0.532 |
| walker |  | 1451 | 115 | Code::CodeKey { rung: Decl, file: tea.go, decl: 15, sub: 0, line: 336 } |  |  | 0.532 |
| walker |  | 1476 | 25 | Code::CodeKey { rung: Doc, file: tea.go, decl: 1, sub: 0, line: 39 } |  |  | 0.532 |
| ns | 1522 |  | 184 | README lede: positioning and shipped feature set | 1.11 |  | 0.517 |
| walker |  | 1638 | 162 | Code::CodeKey { rung: Decl, file: tea.go, decl: 5, sub: 0, line: 53 } |  |  | 0.591 |
| walker |  | 1654 | 16 | Code::CodeKey { rung: Doc, file: tea.go, decl: 5, sub: 0, line: 53 } |  |  | 0.638 |
| ns | 1665 |  | 143 | Sentinel errors returned by `Program.Run` | 2.1 |  | 0.631 |
| walker |  | 1688 | 34 | Code::CodeKey { rung: Doc, file: tea.go, decl: 4, sub: 0, line: 50 } |  |  | 0.642 |
| walker |  | 1724 | 36 | Code::CodeKey { rung: Doc, file: tea.go, decl: 3, sub: 0, line: 46 } |  |  | 0.658 |
| ns | 1810 |  | 145 | Every exported `*Program` method (complete roster) | 2.2 | 1.4 | 0.636 |
| walker |  | 1976 | 252 | Code::CodeKey { rung: Decl, file: tea.go, decl: 11, sub: 0, line: 286 } |  |  | 0.638 |
| ns | 2001 |  | 191 | Every `ProgramOption` constructor (complete roster) | 2.3 |  | 0.612 |
| walker |  | 2025 | 49 | Code::CodeKey { rung: Doc, file: tea.go, decl: 16, sub: 0, line: 349 } |  |  | 0.612 |
| walker |  | 2083 | 58 | Code::CodeKey { rung: Doc, file: tea.go, decl: 15, sub: 0, line: 336 } |  |  | 0.612 |
| ns | 2130 |  | 129 | Build/test/lint entry points (`Taskfile.yaml`) | 2.4 |  | 0.633 |
| ns | 2436 |  | 306 | Event-loop dispatch table: every internally-handled message type | 2.5 |  | 0.586 |
| walker |  | 2444 | 361 | Code::CodeKey { rung: Decl, file: tea.go, decl: 8, sub: 0, line: 241 } |  |  | 0.586 |
| walker |  | 2542 | 98 | Code::CodeKey { rung: Doc, file: tea.go, decl: 6, sub: 0, line: 76 } |  |  | 0.601 |
| walker |  | 2649 | 107 | Code::CodeKey { rung: Doc, file: tea.go, decl: 9, sub: 0, line: 279 } |  |  | 0.601 |
| ns | 2682 |  | 246 | Event-loop skeleton: translate → filter → dispatch → update → render | 2.6 | 2.5 | 0.567 |
| ns | 2901 |  | 219 | Program control messages: suspend, resume, interrupt | 2.7 | 1.9 | 0.540 |
| walker |  | 2929 | 280 | Code::CodeKey { rung: Names, file: tea.go, decl: 0, sub: 1, line: 0 } |  |  | 0.550 |
| walker |  | 2942 | 13 | Code::CodeKey { rung: Doc, file: tea.go, decl: 31, sub: 0, line: 595 } |  |  | 0.554 |
| walker |  | 2969 | 27 | Code::CodeKey { rung: Decl, file: tea.go, decl: 20, sub: 0, line: 395 } |  |  | 0.554 |
| walker |  | 2986 | 17 | Code::CodeKey { rung: Doc, file: tea.go, decl: 26, sub: 0, line: 564 } |  |  | 0.556 |
| walker |  | 3005 | 19 | Code::CodeKey { rung: Doc, file: tea.go, decl: 24, sub: 0, line: 555 } |  |  | 0.558 |
| walker |  | 3028 | 23 | Code::CodeKey { rung: Doc, file: tea.go, decl: 30, sub: 0, line: 590 } |  |  | 0.563 |
| ns | 3100 |  | 199 | `Run()`: how input and the TTY are acquired | 2.8 |  | 0.543 |
| walker |  | 3176 | 148 | Code::CodeKey { rung: Decl, file: tea.go, decl: 17, sub: 0, line: 357 } |  |  | 0.544 |
| walker |  | 3190 | 14 | Code::CodeKey { rung: Doc, file: tea.go, decl: 17, sub: 0, line: 357 } |  |  | 0.544 |
| walker |  | 3214 | 24 | Code::CodeKey { rung: Doc, file: tea.go, decl: 18, sub: 0, line: 374 } |  |  | 0.544 |
| walker |  | 3242 | 28 | Code::CodeKey { rung: Doc, file: tea.go, decl: 28, sub: 0, line: 578 } |  |  | 0.550 |
| walker |  | 3274 | 32 | Code::CodeKey { rung: Doc, file: tea.go, decl: 25, sub: 0, line: 561 } |  |  | 0.555 |
| walker |  | 3286 | 12 | Code::CodeKey { rung: Doc, file: tea.go, decl: 33, sub: 0, line: 685 } |  |  | 0.555 |
| ns | 3288 |  | 188 | `Run()`: renderer choice, colour profile, first three messages | 2.9 | 2.8 | 0.539 |
| walker |  | 3299 | 13 | Code::CodeKey { rung: Doc, file: tea.go, decl: 22, sub: 0, line: 409 } |  |  | 0.539 |
| walker |  | 3381 | 82 | Code::CodeKey { rung: Doc, file: tea.go, decl: 29, sub: 0, line: 586 } |  |  | 0.550 |
| walker |  | 3466 | 85 | Code::CodeKey { rung: Doc, file: tea.go, decl: 27, sub: 0, line: 574 } |  |  | 0.564 |
| ns | 3474 |  | 186 | `Run()`: model init, event loop, graceful shutdown | 2.10 | 2.9 | 0.545 |
| ns | 3693 |  | 219 | Debug hooks: `TEA_TRACE`, `TEA_DEBUG`, panic recovery | 2.11 |  | 0.534 |
| walker |  | 3708 | 242 | Code::CodeKey { rung: Names, file: tea.go, decl: 0, sub: 2, line: 0 } |  |  | 0.548 |
| walker |  | 3726 | 18 | Code::CodeKey { rung: Doc, file: tea.go, decl: 44, sub: 0, line: 1209 } |  |  | 0.548 |
| walker |  | 3740 | 14 | Code::CodeKey { rung: Doc, file: tea.go, decl: 36, sub: 0, line: 886 } |  |  | 0.548 |
| walker |  | 3793 | 53 | Code::CodeKey { rung: Doc, file: tea.go, decl: 40, sub: 0, line: 991 } |  |  | 0.548 |
| walker |  | 3809 | 16 | Code::CodeKey { rung: Doc, file: tea.go, decl: 45, sub: 0, line: 1214 } |  |  | 0.548 |
| walker |  | 3866 | 57 | Code::CodeKey { rung: Doc, file: tea.go, decl: 43, sub: 0, line: 1204 } |  |  | 0.548 |
| walker |  | 3883 | 17 | Code::CodeKey { rung: Doc, file: tea.go, decl: 46, sub: 0, line: 1221 } |  |  | 0.548 |
| ns | 3916 |  | 223 | Frame pacing: the render ticker and the FPS bounds | 2.12 |  | 0.529 |
| walker |  | 4041 | 158 | Code::CodeKey { rung: Names, file: tea.go, decl: 0, sub: 3, line: 0 } |  |  | 0.547 |
| walker |  | 4079 | 38 | Code::CodeKey { rung: Doc, file: tea.go, decl: 50, sub: 0, line: 1319 } |  |  | 0.547 |
| walker |  | 4091 | 12 | Code::CodeKey { rung: Doc, file: tea.go, decl: 55, sub: 0, line: 1393 } |  |  | 0.547 |
| ns | 4118 |  | 202 | Input event message types: keys, mouse, paste, focus (complete) | 3.1 |  | 0.530 |
| walker |  | 4150 | 59 | Code::CodeKey { rung: Doc, file: tea.go, decl: 52, sub: 0, line: 1344 } |  |  | 0.530 |
| walker |  | 4211 | 61 | Code::CodeKey { rung: Doc, file: tea.go, decl: 53, sub: 0, line: 1372 } |  |  | 0.530 |
| walker |  | 4240 | 29 | Code::CodeKey { rung: Doc, file: tea.go, decl: 21, sub: 0, line: 402 } |  |  | 0.530 |
| walker |  | 4269 | 29 | Code::CodeKey { rung: Doc, file: tea.go, decl: 47, sub: 0, line: 1241 } |  |  | 0.530 |
| walker |  | 4299 | 30 | Code::CodeKey { rung: Doc, file: tea.go, decl: 34, sub: 0, line: 700 } |  |  | 0.530 |
| ns | 4380 |  | 262 | Terminal report message types: colour, clipboard, size, capability (complete) | 3.2 |  | 0.512 |
| walker |  | 4407 | 108 | Code::CodeKey { rung: Doc, file: tea.go, decl: 41, sub: 0, line: 1183 } |  |  | 0.512 |
| walker |  | 4520 | 113 | Code::CodeKey { rung: Doc, file: tea.go, decl: 19, sub: 0, line: 390 } |  |  | 0.551 |
| ns | 4550 |  | 170 | Command constructors, part 1: lifecycle, batching, timing, output | 3.3 | 2.7 | 0.541 |
| walker |  | 4636 | 116 | Code::CodeKey { rung: Doc, file: tea.go, decl: 42, sub: 0, line: 1197 } |  |  | 0.541 |
| ns | 4696 |  | 146 | Command constructors, part 2: terminal queries and clipboard | 3.4 | 3.3 | 0.533 |
| walker |  | 4756 | 120 | Code::CodeKey { rung: Doc, file: tea.go, decl: 54, sub: 0, line: 1386 } |  |  | 0.533 |
| walker |  | 4793 | 37 | Code::CodeKey { rung: Doc, file: tea.go, decl: 35, sub: 0, line: 743 } |  |  | 0.533 |
| walker |  | 4830 | 37 | Code::CodeKey { rung: Doc, file: tea.go, decl: 48, sub: 0, line: 1269 } |  |  | 0.533 |
| ns | 5037 |  | 341 | `input.go`: the ultraviolet-event → `tea.Msg` translation table | 3.5 |  | 0.511 |
| walker |  | 5080 | 250 | Fs::DirListing { dir: examples } |  |  | 0.516 |
| ns | 5113 |  | 76 | `Key` struct fields | 3.6 |  | 0.510 |
| walker |  | 5121 | 41 | Code::CodeKey { rung: Doc, file: tea.go, decl: 20, sub: 0, line: 395 } |  |  | 0.510 |
| walker |  | 5163 | 42 | Code::CodeKey { rung: Doc, file: tea.go, decl: 56, sub: 0, line: 1427 } |  |  | 0.510 |
| ns | 5293 |  | 180 | Key message methods: `String`, `Keystroke`, `Key` | 3.7 | 3.6 | 0.503 |
| walker |  | 5409 | 246 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.504 |
| ns | 5614 |  | 321 | `key.go` key-code catalog: group headings plus the C0/G0 names | 3.8 |  | 0.484 |
| walker |  | 5660 | 251 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: true } |  |  | 0.500 |
| walker |  | 5682 | 22 | Code::CodeKey { rung: Names, file: focus.go, decl: 0, sub: 0, line: 0 } |  |  | 0.500 |
| walker |  | 5706 | 24 | Code::CodeKey { rung: Doc, file: focus.go, decl: 2, sub: 0, line: 9 } |  |  | 0.500 |
| walker |  | 5732 | 26 | Code::CodeKey { rung: Doc, file: focus.go, decl: 1, sub: 0, line: 5 } |  |  | 0.500 |
| walker |  | 5778 | 46 | Code::CodeKey { rung: Doc, file: tea.go, decl: 49, sub: 0, line: 1294 } |  |  | 0.500 |
| ns | 5797 |  | 183 | Modifier keys: the complete `KeyMod` constant set | 3.9 |  | 0.492 |
| walker |  | 5828 | 50 | Code::CodeKey { rung: Names, file: screen.go, decl: 0, sub: 0, line: 0 } |  |  | 0.493 |
| walker |  | 5850 | 22 | Code::CodeKey { rung: Decl, file: screen.go, decl: 1, sub: 0, line: 9 } |  |  | 0.493 |
| walker |  | 5900 | 50 | Code::CodeKey { rung: Decl, file: screen.go, decl: 4, sub: 0, line: 62 } |  |  | 0.493 |
| walker |  | 5909 | 9 | Code::CodeKey { rung: Body, file: screen.go, decl: 2, sub: 0, line: 20 } |  |  | 0.493 |
| walker |  | 5980 | 71 | Code::CodeKey { rung: Doc, file: screen.go, decl: 1, sub: 0, line: 9 } |  |  | 0.493 |
| walker |  | 6022 | 42 | Markdown::Section { file: README.md, section_index: 16, keeps_default_concavity: false } |  |  | 0.493 |
| ns | 6023 |  | 226 | `Mouse` struct and the complete mouse-button constant set | 3.10 | 3.1 | 0.483 |
| walker |  | 6073 | 51 | Code::CodeKey { rung: Names, file: paste.go, decl: 0, sub: 0, line: 0 } |  |  | 0.485 |
| walker |  | 6085 | 12 | Code::CodeKey { rung: Decl, file: paste.go, decl: 1, sub: 0, line: 5 } |  |  | 0.485 |
| walker |  | 6093 | 8 | Code::CodeKey { rung: Body, file: paste.go, decl: 2, sub: 0, line: 10 } |  |  | 0.485 |
| walker |  | 6105 | 12 | Code::CodeKey { rung: Doc, file: paste.go, decl: 2, sub: 0, line: 10 } |  |  | 0.485 |
| walker |  | 6134 | 29 | Code::CodeKey { rung: Doc, file: paste.go, decl: 4, sub: 0, line: 20 } |  |  | 0.485 |
| walker |  | 6165 | 31 | Code::CodeKey { rung: Doc, file: paste.go, decl: 1, sub: 0, line: 5 } |  |  | 0.485 |
| walker |  | 6196 | 31 | Code::CodeKey { rung: Doc, file: paste.go, decl: 3, sub: 0, line: 16 } |  |  | 0.485 |
| ns | 6200 |  | 177 | `MouseMode` enum: none / cell-motion / all-motion | 3.11 | 1.7 | 0.500 |
| walker |  | 6222 | 26 | Code::CodeKey { rung: Names, file: raw.go, decl: 0, sub: 0, line: 0 } |  |  | 0.500 |
| walker |  | 6235 | 13 | Code::CodeKey { rung: Decl, file: raw.go, decl: 1, sub: 0, line: 5 } |  |  | 0.501 |
| walker |  | 6261 | 26 | Code::CodeKey { rung: Body, file: raw.go, decl: 2, sub: 0, line: 33 } |  |  | 0.501 |
| walker |  | 6292 | 31 | Code::CodeKey { rung: Doc, file: raw.go, decl: 1, sub: 0, line: 5 } |  |  | 0.501 |
| ns | 6378 |  | 178 | Cursor value types: `Cursor`, `NewCursor`, `CursorShape` | 3.12 | 1.7 | 0.497 |
| walker |  | 6441 | 149 | Code::CodeKey { rung: Names, file: clipboard.go, decl: 0, sub: 0, line: 0 } |  |  | 0.501 |
| walker |  | 6462 | 21 | Code::CodeKey { rung: Decl, file: clipboard.go, decl: 1, sub: 0, line: 5 } |  |  | 0.503 |
| walker |  | 6470 | 8 | Code::CodeKey { rung: Body, file: clipboard.go, decl: 2, sub: 0, line: 15 } |  |  | 0.503 |
| walker |  | 6478 | 8 | Code::CodeKey { rung: Body, file: clipboard.go, decl: 3, sub: 0, line: 20 } |  |  | 0.503 |
| walker |  | 6487 | 9 | Code::CodeKey { rung: Body, file: clipboard.go, decl: 7, sub: 0, line: 42 } |  |  | 0.503 |
| walker |  | 6502 | 15 | Code::CodeKey { rung: Doc, file: clipboard.go, decl: 3, sub: 0, line: 20 } |  |  | 0.503 |
| walker |  | 6512 | 10 | Code::CodeKey { rung: Body, file: clipboard.go, decl: 11, sub: 0, line: 68 } |  |  | 0.503 |
| walker |  | 6545 | 33 | Code::CodeKey { rung: Doc, file: clipboard.go, decl: 5, sub: 0, line: 30 } |  |  | 0.503 |
| ns | 6566 |  | 188 | Keyboard enhancements: request fields and response predicates | 3.13 | 3.2 | 0.499 |
| walker |  | 6578 | 33 | Code::CodeKey { rung: Doc, file: clipboard.go, decl: 7, sub: 0, line: 42 } |  |  | 0.499 |
| walker |  | 6614 | 36 | Code::CodeKey { rung: Doc, file: clipboard.go, decl: 1, sub: 0, line: 5 } |  |  | 0.499 |
| walker |  | 6675 | 61 | Code::CodeKey { rung: Doc, file: clipboard.go, decl: 2, sub: 0, line: 15 } |  |  | 0.499 |
| walker |  | 6736 | 61 | Code::CodeKey { rung: Doc, file: clipboard.go, decl: 9, sub: 0, line: 54 } |  |  | 0.499 |
| ns | 6759 |  | 193 | `Batch` vs `Sequence` semantics and where they execute | 3.14 | 3.3 | 0.494 |
| walker |  | 6797 | 61 | Code::CodeKey { rung: Doc, file: clipboard.go, decl: 11, sub: 0, line: 68 } |  |  | 0.494 |
| walker |  | 6852 | 55 | Code::CodeKey { rung: Names, file: termcap.go, decl: 0, sub: 0, line: 0 } |  |  | 0.496 |
| walker |  | 6864 | 12 | Code::CodeKey { rung: Decl, file: termcap.go, decl: 3, sub: 0, line: 41 } |  |  | 0.497 |
| walker |  | 6872 | 8 | Code::CodeKey { rung: Body, file: termcap.go, decl: 4, sub: 0, line: 46 } |  |  | 0.497 |
| walker |  | 6884 | 12 | Code::CodeKey { rung: Doc, file: termcap.go, decl: 4, sub: 0, line: 46 } |  |  | 0.497 |
| walker |  | 6910 | 26 | Code::CodeKey { rung: Body, file: termcap.go, decl: 2, sub: 0, line: 30 } |  |  | 0.497 |
| ns | 6959 |  | 200 | `Exec` / `ExecProcess` and the `ExecCommand` interface | 3.15 | 3.3 | 0.491 |
| walker |  | 6965 | 55 | Code::CodeKey { rung: Names, file: xterm.go, decl: 0, sub: 0, line: 0 } |  |  | 0.494 |
| walker |  | 6977 | 12 | Code::CodeKey { rung: Decl, file: xterm.go, decl: 1, sub: 0, line: 4 } |  |  | 0.495 |
| walker |  | 6985 | 8 | Code::CodeKey { rung: Body, file: xterm.go, decl: 2, sub: 0, line: 9 } |  |  | 0.495 |
| walker |  | 6993 | 8 | Code::CodeKey { rung: Body, file: xterm.go, decl: 4, sub: 0, line: 20 } |  |  | 0.495 |
| walker |  | 7005 | 12 | Code::CodeKey { rung: Doc, file: xterm.go, decl: 2, sub: 0, line: 9 } |  |  | 0.495 |
| walker |  | 7022 | 17 | Code::CodeKey { rung: Doc, file: xterm.go, decl: 1, sub: 0, line: 4 } |  |  | 0.495 |
| walker |  | 7065 | 43 | Code::CodeKey { rung: Doc, file: xterm.go, decl: 4, sub: 0, line: 20 } |  |  | 0.495 |
| ns | 7111 |  | 152 | Logging to a file: `LogToFile` / `LogToFileWith` | 3.16 |  | 0.491 |
| ns | 7328 |  | 217 | The `renderer` interface: complete method set, and its two implementations | 4.1 |  | 0.482 |
| walker |  | 7541 | 476 | GoMod::File { file: go.mod } |  |  | 0.499 |
| walker |  | 7555 | 14 | Code::CodeKey { rung: Names, file: profile.go, decl: 0, sub: 0, line: 0 } |  |  | 0.500 |
| walker |  | 7566 | 11 | Code::CodeKey { rung: Decl, file: profile.go, decl: 1, sub: 0, line: 13 } |  |  | 0.502 |
| walker |  | 7656 | 90 | Code::CodeKey { rung: Doc, file: termcap.go, decl: 3, sub: 0, line: 41 } |  |  | 0.502 |
| ns | 7681 |  | 353 | `cursedRenderer`: every function in the 860-line renderer | 4.2 | 4.1 | 0.491 |
| walker |  | 7701 | 45 | Code::CodeKey { rung: Names, file: tty_unix.go, decl: 0, sub: 0, line: 0 } |  |  | 0.491 |
| walker |  | 7715 | 14 | Code::CodeKey { rung: Doc, file: tty_unix.go, decl: 3, sub: 0, line: 40 } |  |  | 0.491 |
| walker |  | 7945 | 230 | Code::CodeKey { rung: Names, file: color.go, decl: 0, sub: 0, line: 0 } |  |  | 0.506 |
| walker |  | 7954 | 9 | Code::CodeKey { rung: Body, file: color.go, decl: 2, sub: 0, line: 13 } |  |  | 0.506 |
| walker |  | 7963 | 9 | Code::CodeKey { rung: Body, file: color.go, decl: 4, sub: 0, line: 21 } |  |  | 0.506 |
| walker |  | 7972 | 9 | Code::CodeKey { rung: Body, file: color.go, decl: 6, sub: 0, line: 29 } |  |  | 0.506 |
| walker |  | 7984 | 12 | Code::CodeKey { rung: Doc, file: color.go, decl: 8, sub: 0, line: 39 } |  |  | 0.506 |
| walker |  | 7996 | 12 | Code::CodeKey { rung: Doc, file: color.go, decl: 11, sub: 0, line: 70 } |  |  | 0.506 |
| walker |  | 8008 | 12 | Code::CodeKey { rung: Doc, file: color.go, decl: 14, sub: 0, line: 84 } |  |  | 0.506 |
| walker |  | 8022 | 14 | Code::CodeKey { rung: Doc, file: color.go, decl: 9, sub: 0, line: 44 } |  |  | 0.506 |
| walker |  | 8036 | 14 | Code::CodeKey { rung: Doc, file: color.go, decl: 12, sub: 0, line: 75 } |  |  | 0.506 |
| ns | 8039 |  | 358 | `View` fields → ANSI sequences, in `cursedRenderer.start` | 4.3 | 4.2 | 0.495 |
| walker |  | 8050 | 14 | Code::CodeKey { rung: Doc, file: color.go, decl: 15, sub: 0, line: 89 } |  |  | 0.495 |
| walker |  | 8066 | 16 | Code::CodeKey { rung: Doc, file: color.go, decl: 2, sub: 0, line: 13 } |  |  | 0.495 |
| walker |  | 8082 | 16 | Code::CodeKey { rung: Doc, file: color.go, decl: 4, sub: 0, line: 21 } |  |  | 0.495 |
| walker |  | 8098 | 16 | Code::CodeKey { rung: Doc, file: color.go, decl: 6, sub: 0, line: 29 } |  |  | 0.495 |
| walker |  | 8132 | 34 | Code::CodeKey { rung: Doc, file: color.go, decl: 13, sub: 0, line: 81 } |  |  | 0.495 |
| walker |  | 8180 | 48 | Code::CodeKey { rung: Doc, file: color.go, decl: 7, sub: 0, line: 36 } |  |  | 0.495 |
| walker |  | 8198 | 18 | Code::CodeKey { rung: Doc, file: color.go, decl: 1, sub: 0, line: 10 } |  |  | 0.495 |
| walker |  | 8216 | 18 | Code::CodeKey { rung: Doc, file: color.go, decl: 3, sub: 0, line: 18 } |  |  | 0.495 |
| walker |  | 8234 | 18 | Code::CodeKey { rung: Doc, file: color.go, decl: 5, sub: 0, line: 26 } |  |  | 0.495 |
| walker |  | 8280 | 46 | Code::CodeKey { rung: Names, file: tty_windows.go, decl: 0, sub: 0, line: 0 } |  |  | 0.495 |
| ns | 8420 |  | 381 | Platform matrix: build tags and per-OS terminal functions | 4.4 |  | 0.487 |
| ns | 8618 |  | 198 | `tty.go`: terminal acquisition, input loop, resize detection | 4.5 |  | 0.483 |
| walker |  | 8796 | 516 | Code::CodeKey { rung: Decl, file: tea.go, decl: 7, sub: 0, line: 84 } |  |  | 0.484 |
| walker |  | 8835 | 39 | Code::CodeKey { rung: Doc, file: tea.go, decl: 7, sub: 0, line: 84 } |  |  | 0.489 |
| ns | 8882 |  | 264 | `examples/` and `tutorials/` directory listings | 5.1 |  | 0.534 |
| walker |  | 8930 | 95 | Code::CodeKey { rung: Doc, file: screen.go, decl: 2, sub: 0, line: 20 } |  |  | 0.534 |
| ns | 9062 |  | 180 | `UPGRADE_GUIDE_V2.md`: complete section map | 5.2 |  | 0.528 |
| walker |  | 9136 | 206 | Code::CodeKey { rung: Names, file: options.go, decl: 0, sub: 0, line: 0 } |  |  | 0.548 |
| walker |  | 9160 | 24 | Code::CodeKey { rung: Doc, file: options.go, decl: 8, sub: 0, line: 84 } |  |  | 0.548 |
| walker |  | 9187 | 27 | Code::CodeKey { rung: Body, file: options.go, decl: 3, sub: 0, line: 30 } |  |  | 0.548 |
| walker |  | 9214 | 27 | Code::CodeKey { rung: Body, file: options.go, decl: 5, sub: 0, line: 58 } |  |  | 0.548 |
| walker |  | 9241 | 27 | Code::CodeKey { rung: Body, file: options.go, decl: 10, sub: 0, line: 133 } |  |  | 0.548 |
| walker |  | 9275 | 34 | Code::CodeKey { rung: Doc, file: options.go, decl: 3, sub: 0, line: 30 } |  |  | 0.548 |
| walker |  | 9312 | 37 | Code::CodeKey { rung: Doc, file: options.go, decl: 6, sub: 0, line: 66 } |  |  | 0.548 |
| ns | 9347 |  | 285 | The v1→v2 migration checklist | 5.3 | 5.2 | 0.543 |
| walker |  | 9369 | 57 | Code::CodeKey { rung: Doc, file: options.go, decl: 2, sub: 0, line: 22 } |  |  | 0.543 |
| walker |  | 9430 | 61 | Code::CodeKey { rung: Doc, file: options.go, decl: 4, sub: 0, line: 40 } |  |  | 0.543 |
| walker |  | 9491 | 61 | Code::CodeKey { rung: Doc, file: options.go, decl: 11, sub: 0, line: 142 } |  |  | 0.543 |
| ns | 9516 |  | 169 | README section map | 5.4 |  | 0.552 |
| walker |  | 9559 | 68 | Code::CodeKey { rung: Doc, file: options.go, decl: 7, sub: 0, line: 76 } |  |  | 0.552 |
| walker |  | 9629 | 70 | Code::CodeKey { rung: Doc, file: options.go, decl: 13, sub: 0, line: 163 } |  |  | 0.552 |
| walker |  | 9702 | 73 | Code::CodeKey { rung: Doc, file: options.go, decl: 1, sub: 0, line: 17 } |  |  | 0.552 |
| walker |  | 9791 | 89 | Code::CodeKey { rung: Doc, file: options.go, decl: 12, sub: 0, line: 153 } |  |  | 0.552 |
| walker |  | 9819 | 28 | Code::CodeKey { rung: Doc, file: clipboard.go, decl: 4, sub: 0, line: 26 } |  |  | 0.552 |
| walker |  | 9847 | 28 | Code::CodeKey { rung: Doc, file: clipboard.go, decl: 6, sub: 0, line: 38 } |  |  | 0.552 |
| ns | 9876 |  | 360 | Test suite: every test function, and the golden-file fixtures | 5.5 |  | 0.541 |
| ns | 9917 |  | 41 | CI workflow inventory and golden-test fixture directories | 5.6 |  | 0.545 |
| walker |  | 9992 | 145 | Code::CodeKey { rung: Names, file: commands.go, decl: 0, sub: 0, line: 0 } |  |  | 0.552 |
