Score(3000)=0.563 I=0.781 C=0.405 ns_rows≤3K=18/50 grid(1000/1442/2080/3000/4327/6240/9000)=0.584/0.531/0.612/0.563/0.530/0.500/0.529

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
| ns | 1338 |  | 206 | Direct dependency set from `go.mod` | 1.10 |  | 0.531 |
| walker |  | 1432 | 115 | Code::CodeKey { rung: Decl, file: tea.go, decl: 15, sub: 0, line: 336 } |  |  | 0.531 |
| walker |  | 1451 | 19 | Code::CodeKey { rung: Doc, file: tea.go, decl: 2, sub: 0, line: 42 } |  |  | 0.532 |
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
| ns | 2682 |  | 246 | Event-loop skeleton: translate → filter → dispatch → update → render | 2.6 | 2.5 | 0.567 |
| walker |  | 2822 | 280 | Code::CodeKey { rung: Names, file: tea.go, decl: 0, sub: 1, line: 0 } |  |  | 0.575 |
| walker |  | 2849 | 27 | Code::CodeKey { rung: Decl, file: tea.go, decl: 20, sub: 0, line: 395 } |  |  | 0.575 |
| walker |  | 2861 | 12 | Code::CodeKey { rung: Doc, file: tea.go, decl: 33, sub: 0, line: 685 } |  |  | 0.575 |
| walker |  | 2874 | 13 | Code::CodeKey { rung: Doc, file: tea.go, decl: 22, sub: 0, line: 409 } |  |  | 0.575 |
| walker |  | 2887 | 13 | Code::CodeKey { rung: Doc, file: tea.go, decl: 31, sub: 0, line: 595 } |  |  | 0.579 |
| ns | 2901 |  | 219 | Program control messages: suspend, resume, interrupt | 2.7 | 1.9 | 0.554 |
| walker |  | 2904 | 17 | Code::CodeKey { rung: Doc, file: tea.go, decl: 26, sub: 0, line: 564 } |  |  | 0.556 |
| walker |  | 2923 | 19 | Code::CodeKey { rung: Doc, file: tea.go, decl: 24, sub: 0, line: 555 } |  |  | 0.558 |
| walker |  | 2946 | 23 | Code::CodeKey { rung: Doc, file: tea.go, decl: 30, sub: 0, line: 590 } |  |  | 0.563 |
| walker |  | 3094 | 148 | Code::CodeKey { rung: Decl, file: tea.go, decl: 17, sub: 0, line: 357 } |  |  | 0.563 |
| ns | 3100 |  | 199 | `Run()`: how input and the TTY are acquired | 2.8 |  | 0.544 |
| walker |  | 3108 | 14 | Code::CodeKey { rung: Doc, file: tea.go, decl: 17, sub: 0, line: 357 } |  |  | 0.544 |
| walker |  | 3132 | 24 | Code::CodeKey { rung: Doc, file: tea.go, decl: 18, sub: 0, line: 374 } |  |  | 0.544 |
| walker |  | 3160 | 28 | Code::CodeKey { rung: Doc, file: tea.go, decl: 28, sub: 0, line: 578 } |  |  | 0.550 |
| walker |  | 3189 | 29 | Code::CodeKey { rung: Doc, file: tea.go, decl: 21, sub: 0, line: 402 } |  |  | 0.550 |
| walker |  | 3219 | 30 | Code::CodeKey { rung: Doc, file: tea.go, decl: 34, sub: 0, line: 700 } |  |  | 0.550 |
| walker |  | 3251 | 32 | Code::CodeKey { rung: Doc, file: tea.go, decl: 25, sub: 0, line: 561 } |  |  | 0.555 |
| walker |  | 3288 | 37 | Code::CodeKey { rung: Doc, file: tea.go, decl: 35, sub: 0, line: 743 } |  |  | 0.539 |
| ns | 3288 |  | 188 | `Run()`: renderer choice, colour profile, first three messages | 2.9 | 2.8 | 0.539 |
| walker |  | 3329 | 41 | Code::CodeKey { rung: Doc, file: tea.go, decl: 20, sub: 0, line: 395 } |  |  | 0.539 |
| walker |  | 3411 | 82 | Code::CodeKey { rung: Doc, file: tea.go, decl: 29, sub: 0, line: 586 } |  |  | 0.550 |
| ns | 3474 |  | 186 | `Run()`: model init, event loop, graceful shutdown | 2.10 | 2.9 | 0.531 |
| walker |  | 3496 | 85 | Code::CodeKey { rung: Doc, file: tea.go, decl: 27, sub: 0, line: 574 } |  |  | 0.545 |
| ns | 3693 |  | 219 | Debug hooks: `TEA_TRACE`, `TEA_DEBUG`, panic recovery | 2.11 |  | 0.534 |
| walker |  | 3738 | 242 | Code::CodeKey { rung: Names, file: tea.go, decl: 0, sub: 2, line: 0 } |  |  | 0.548 |
| walker |  | 3752 | 14 | Code::CodeKey { rung: Doc, file: tea.go, decl: 36, sub: 0, line: 886 } |  |  | 0.548 |
| walker |  | 3768 | 16 | Code::CodeKey { rung: Doc, file: tea.go, decl: 45, sub: 0, line: 1214 } |  |  | 0.548 |
| walker |  | 3785 | 17 | Code::CodeKey { rung: Doc, file: tea.go, decl: 46, sub: 0, line: 1221 } |  |  | 0.548 |
| walker |  | 3803 | 18 | Code::CodeKey { rung: Doc, file: tea.go, decl: 44, sub: 0, line: 1209 } |  |  | 0.548 |
| walker |  | 3832 | 29 | Code::CodeKey { rung: Doc, file: tea.go, decl: 47, sub: 0, line: 1241 } |  |  | 0.548 |
| walker |  | 3869 | 37 | Code::CodeKey { rung: Doc, file: tea.go, decl: 48, sub: 0, line: 1269 } |  |  | 0.548 |
| ns | 3916 |  | 223 | Frame pacing: the render ticker and the FPS bounds | 2.12 |  | 0.529 |
| walker |  | 3922 | 53 | Code::CodeKey { rung: Doc, file: tea.go, decl: 40, sub: 0, line: 991 } |  |  | 0.529 |
| walker |  | 3979 | 57 | Code::CodeKey { rung: Doc, file: tea.go, decl: 43, sub: 0, line: 1204 } |  |  | 0.529 |
| ns | 4118 |  | 202 | Input event message types: keys, mouse, paste, focus (complete) | 3.1 |  | 0.513 |
| walker |  | 4137 | 158 | Code::CodeKey { rung: Names, file: tea.go, decl: 0, sub: 3, line: 0 } |  |  | 0.530 |
| walker |  | 4149 | 12 | Code::CodeKey { rung: Doc, file: tea.go, decl: 55, sub: 0, line: 1393 } |  |  | 0.530 |
| walker |  | 4187 | 38 | Code::CodeKey { rung: Doc, file: tea.go, decl: 50, sub: 0, line: 1319 } |  |  | 0.530 |
| walker |  | 4229 | 42 | Code::CodeKey { rung: Doc, file: tea.go, decl: 56, sub: 0, line: 1427 } |  |  | 0.530 |
| walker |  | 4275 | 46 | Code::CodeKey { rung: Doc, file: tea.go, decl: 49, sub: 0, line: 1294 } |  |  | 0.530 |
| walker |  | 4334 | 59 | Code::CodeKey { rung: Doc, file: tea.go, decl: 52, sub: 0, line: 1344 } |  |  | 0.530 |
| ns | 4380 |  | 262 | Terminal report message types: colour, clipboard, size, capability (complete) | 3.2 |  | 0.512 |
| walker |  | 4395 | 61 | Code::CodeKey { rung: Doc, file: tea.go, decl: 53, sub: 0, line: 1372 } |  |  | 0.512 |
| walker |  | 4502 | 107 | Code::CodeKey { rung: Doc, file: tea.go, decl: 9, sub: 0, line: 279 } |  |  | 0.512 |
| ns | 4550 |  | 170 | Command constructors, part 1: lifecycle, batching, timing, output | 3.3 | 2.7 | 0.503 |
| walker |  | 4610 | 108 | Code::CodeKey { rung: Doc, file: tea.go, decl: 41, sub: 0, line: 1183 } |  |  | 0.503 |
| ns | 4696 |  | 146 | Command constructors, part 2: terminal queries and clipboard | 3.4 | 3.3 | 0.495 |
| walker |  | 4723 | 113 | Code::CodeKey { rung: Doc, file: tea.go, decl: 19, sub: 0, line: 390 } |  |  | 0.533 |
| walker |  | 4839 | 116 | Code::CodeKey { rung: Doc, file: tea.go, decl: 42, sub: 0, line: 1197 } |  |  | 0.533 |
| walker |  | 4959 | 120 | Code::CodeKey { rung: Doc, file: tea.go, decl: 54, sub: 0, line: 1386 } |  |  | 0.533 |
| ns | 5037 |  | 341 | `input.go`: the ultraviolet-event → `tea.Msg` translation table | 3.5 |  | 0.511 |
| ns | 5113 |  | 76 | `Key` struct fields | 3.6 |  | 0.505 |
| walker |  | 5209 | 250 | Fs::DirListing { dir: examples } |  |  | 0.510 |
| ns | 5293 |  | 180 | Key message methods: `String`, `Keystroke`, `Key` | 3.7 | 3.6 | 0.503 |
| walker |  | 5455 | 246 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.504 |
| ns | 5614 |  | 321 | `key.go` key-code catalog: group headings plus the C0/G0 names | 3.8 |  | 0.484 |
| walker |  | 5706 | 251 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: true } |  |  | 0.500 |
| walker |  | 5728 | 22 | Code::CodeKey { rung: Names, file: focus.go, decl: 0, sub: 0, line: 0 } |  |  | 0.500 |
| walker |  | 5752 | 24 | Code::CodeKey { rung: Doc, file: focus.go, decl: 2, sub: 0, line: 9 } |  |  | 0.500 |
| walker |  | 5778 | 26 | Code::CodeKey { rung: Doc, file: focus.go, decl: 1, sub: 0, line: 5 } |  |  | 0.500 |
| ns | 5797 |  | 183 | Modifier keys: the complete `KeyMod` constant set | 3.9 |  | 0.492 |
| walker |  | 5828 | 50 | Code::CodeKey { rung: Names, file: screen.go, decl: 0, sub: 0, line: 0 } |  |  | 0.493 |
| walker |  | 5850 | 22 | Code::CodeKey { rung: Decl, file: screen.go, decl: 1, sub: 0, line: 9 } |  |  | 0.493 |
| walker |  | 5900 | 50 | Code::CodeKey { rung: Decl, file: screen.go, decl: 4, sub: 0, line: 62 } |  |  | 0.493 |
| walker |  | 5909 | 9 | Code::CodeKey { rung: Body, file: screen.go, decl: 2, sub: 0, line: 20 } |  |  | 0.493 |
| walker |  | 5944 | 35 | Code::CodeKey { rung: Doc, file: screen.go, decl: 3, sub: 0, line: 26 } |  |  | 0.493 |
| walker |  | 6015 | 71 | Code::CodeKey { rung: Doc, file: screen.go, decl: 1, sub: 0, line: 9 } |  |  | 0.493 |
| ns | 6023 |  | 226 | `Mouse` struct and the complete mouse-button constant set | 3.10 | 3.1 | 0.483 |
| walker |  | 6057 | 42 | Markdown::Section { file: README.md, section_index: 16, keeps_default_concavity: false } |  |  | 0.483 |
| walker |  | 6108 | 51 | Code::CodeKey { rung: Names, file: paste.go, decl: 0, sub: 0, line: 0 } |  |  | 0.485 |
| walker |  | 6120 | 12 | Code::CodeKey { rung: Decl, file: paste.go, decl: 1, sub: 0, line: 5 } |  |  | 0.485 |
| walker |  | 6128 | 8 | Code::CodeKey { rung: Body, file: paste.go, decl: 2, sub: 0, line: 10 } |  |  | 0.485 |
| walker |  | 6140 | 12 | Code::CodeKey { rung: Doc, file: paste.go, decl: 2, sub: 0, line: 10 } |  |  | 0.485 |
| walker |  | 6169 | 29 | Code::CodeKey { rung: Doc, file: paste.go, decl: 4, sub: 0, line: 20 } |  |  | 0.485 |
| walker |  | 6200 | 31 | Code::CodeKey { rung: Doc, file: paste.go, decl: 1, sub: 0, line: 5 } |  |  | 0.500 |
| ns | 6200 |  | 177 | `MouseMode` enum: none / cell-motion / all-motion | 3.11 | 1.7 | 0.500 |
| walker |  | 6231 | 31 | Code::CodeKey { rung: Doc, file: paste.go, decl: 3, sub: 0, line: 16 } |  |  | 0.500 |
| walker |  | 6257 | 26 | Code::CodeKey { rung: Names, file: raw.go, decl: 0, sub: 0, line: 0 } |  |  | 0.500 |
| walker |  | 6270 | 13 | Code::CodeKey { rung: Decl, file: raw.go, decl: 1, sub: 0, line: 5 } |  |  | 0.501 |
| walker |  | 6296 | 26 | Code::CodeKey { rung: Body, file: raw.go, decl: 2, sub: 0, line: 33 } |  |  | 0.501 |
| walker |  | 6327 | 31 | Code::CodeKey { rung: Doc, file: raw.go, decl: 1, sub: 0, line: 5 } |  |  | 0.501 |
| ns | 6378 |  | 178 | Cursor value types: `Cursor`, `NewCursor`, `CursorShape` | 3.12 | 1.7 | 0.497 |
| walker |  | 6508 | 181 | Code::CodeKey { rung: Doc, file: tea.go, decl: 39, sub: 0, line: 972 } |  |  | 0.497 |
| ns | 6566 |  | 188 | Keyboard enhancements: request fields and response predicates | 3.13 | 3.2 | 0.493 |
| walker |  | 6657 | 149 | Code::CodeKey { rung: Names, file: clipboard.go, decl: 0, sub: 0, line: 0 } |  |  | 0.497 |
| walker |  | 6678 | 21 | Code::CodeKey { rung: Decl, file: clipboard.go, decl: 1, sub: 0, line: 5 } |  |  | 0.499 |
| walker |  | 6686 | 8 | Code::CodeKey { rung: Body, file: clipboard.go, decl: 2, sub: 0, line: 15 } |  |  | 0.499 |
| walker |  | 6694 | 8 | Code::CodeKey { rung: Body, file: clipboard.go, decl: 3, sub: 0, line: 20 } |  |  | 0.499 |
| walker |  | 6703 | 9 | Code::CodeKey { rung: Body, file: clipboard.go, decl: 7, sub: 0, line: 42 } |  |  | 0.499 |
| walker |  | 6713 | 10 | Code::CodeKey { rung: Body, file: clipboard.go, decl: 11, sub: 0, line: 68 } |  |  | 0.499 |
| walker |  | 6728 | 15 | Code::CodeKey { rung: Doc, file: clipboard.go, decl: 3, sub: 0, line: 20 } |  |  | 0.499 |
| walker |  | 6756 | 28 | Code::CodeKey { rung: Doc, file: clipboard.go, decl: 4, sub: 0, line: 26 } |  |  | 0.499 |
| ns | 6759 |  | 193 | `Batch` vs `Sequence` semantics and where they execute | 3.14 | 3.3 | 0.494 |
| walker |  | 6784 | 28 | Code::CodeKey { rung: Doc, file: clipboard.go, decl: 6, sub: 0, line: 38 } |  |  | 0.494 |
| walker |  | 6813 | 29 | Code::CodeKey { rung: Doc, file: clipboard.go, decl: 8, sub: 0, line: 48 } |  |  | 0.494 |
| walker |  | 6842 | 29 | Code::CodeKey { rung: Doc, file: clipboard.go, decl: 10, sub: 0, line: 62 } |  |  | 0.494 |
| walker |  | 6875 | 33 | Code::CodeKey { rung: Doc, file: clipboard.go, decl: 5, sub: 0, line: 30 } |  |  | 0.494 |
| walker |  | 6908 | 33 | Code::CodeKey { rung: Doc, file: clipboard.go, decl: 7, sub: 0, line: 42 } |  |  | 0.494 |
| walker |  | 6944 | 36 | Code::CodeKey { rung: Doc, file: clipboard.go, decl: 1, sub: 0, line: 5 } |  |  | 0.494 |
| ns | 6959 |  | 200 | `Exec` / `ExecProcess` and the `ExecCommand` interface | 3.15 | 3.3 | 0.487 |
| walker |  | 7005 | 61 | Code::CodeKey { rung: Doc, file: clipboard.go, decl: 2, sub: 0, line: 15 } |  |  | 0.487 |
| walker |  | 7066 | 61 | Code::CodeKey { rung: Doc, file: clipboard.go, decl: 9, sub: 0, line: 54 } |  |  | 0.487 |
| ns | 7111 |  | 152 | Logging to a file: `LogToFile` / `LogToFileWith` | 3.16 |  | 0.483 |
| walker |  | 7127 | 61 | Code::CodeKey { rung: Doc, file: clipboard.go, decl: 11, sub: 0, line: 68 } |  |  | 0.483 |
| walker |  | 7182 | 55 | Code::CodeKey { rung: Names, file: termcap.go, decl: 0, sub: 0, line: 0 } |  |  | 0.485 |
| walker |  | 7194 | 12 | Code::CodeKey { rung: Decl, file: termcap.go, decl: 3, sub: 0, line: 41 } |  |  | 0.486 |
| walker |  | 7202 | 8 | Code::CodeKey { rung: Body, file: termcap.go, decl: 4, sub: 0, line: 46 } |  |  | 0.486 |
| walker |  | 7214 | 12 | Code::CodeKey { rung: Doc, file: termcap.go, decl: 4, sub: 0, line: 46 } |  |  | 0.486 |
| walker |  | 7240 | 26 | Code::CodeKey { rung: Body, file: termcap.go, decl: 2, sub: 0, line: 30 } |  |  | 0.486 |
| walker |  | 7272 | 32 | Code::CodeKey { rung: Doc, file: termcap.go, decl: 1, sub: 0, line: 5 } |  |  | 0.486 |
| walker |  | 7327 | 55 | Code::CodeKey { rung: Names, file: xterm.go, decl: 0, sub: 0, line: 0 } |  |  | 0.490 |
| ns | 7328 |  | 217 | The `renderer` interface: complete method set, and its two implementations | 4.1 |  | 0.481 |
| walker |  | 7339 | 12 | Code::CodeKey { rung: Decl, file: xterm.go, decl: 1, sub: 0, line: 4 } |  |  | 0.482 |
| walker |  | 7347 | 8 | Code::CodeKey { rung: Body, file: xterm.go, decl: 2, sub: 0, line: 9 } |  |  | 0.482 |
| walker |  | 7355 | 8 | Code::CodeKey { rung: Body, file: xterm.go, decl: 4, sub: 0, line: 20 } |  |  | 0.482 |
| walker |  | 7367 | 12 | Code::CodeKey { rung: Doc, file: xterm.go, decl: 2, sub: 0, line: 9 } |  |  | 0.482 |
| walker |  | 7384 | 17 | Code::CodeKey { rung: Doc, file: xterm.go, decl: 1, sub: 0, line: 4 } |  |  | 0.482 |
| walker |  | 7413 | 29 | Code::CodeKey { rung: Doc, file: xterm.go, decl: 3, sub: 0, line: 15 } |  |  | 0.482 |
| walker |  | 7456 | 43 | Code::CodeKey { rung: Doc, file: xterm.go, decl: 4, sub: 0, line: 20 } |  |  | 0.482 |
| ns | 7681 |  | 353 | `cursedRenderer`: every function in the 860-line renderer | 4.2 | 4.1 | 0.472 |
| walker |  | 7932 | 476 | GoMod::File { file: go.mod } |  |  | 0.488 |
| walker |  | 7946 | 14 | Code::CodeKey { rung: Names, file: profile.go, decl: 0, sub: 0, line: 0 } |  |  | 0.490 |
| walker |  | 7957 | 11 | Code::CodeKey { rung: Decl, file: profile.go, decl: 1, sub: 0, line: 13 } |  |  | 0.491 |
| walker |  | 8002 | 45 | Code::CodeKey { rung: Names, file: tty_unix.go, decl: 0, sub: 0, line: 0 } |  |  | 0.491 |
| walker |  | 8016 | 14 | Code::CodeKey { rung: Doc, file: tty_unix.go, decl: 3, sub: 0, line: 40 } |  |  | 0.491 |
| ns | 8039 |  | 358 | `View` fields → ANSI sequences, in `cursedRenderer.start` | 4.3 | 4.2 | 0.481 |
| walker |  | 8093 | 77 | Code::CodeKey { rung: Body, file: tty_unix.go, decl: 3, sub: 0, line: 40 } |  |  | 0.481 |
| walker |  | 8323 | 230 | Code::CodeKey { rung: Names, file: color.go, decl: 0, sub: 0, line: 0 } |  |  | 0.495 |
| walker |  | 8332 | 9 | Code::CodeKey { rung: Body, file: color.go, decl: 2, sub: 0, line: 13 } |  |  | 0.495 |
| walker |  | 8341 | 9 | Code::CodeKey { rung: Body, file: color.go, decl: 4, sub: 0, line: 21 } |  |  | 0.495 |
| walker |  | 8350 | 9 | Code::CodeKey { rung: Body, file: color.go, decl: 6, sub: 0, line: 29 } |  |  | 0.495 |
| walker |  | 8362 | 12 | Code::CodeKey { rung: Doc, file: color.go, decl: 8, sub: 0, line: 39 } |  |  | 0.495 |
| walker |  | 8374 | 12 | Code::CodeKey { rung: Doc, file: color.go, decl: 11, sub: 0, line: 70 } |  |  | 0.495 |
| walker |  | 8386 | 12 | Code::CodeKey { rung: Doc, file: color.go, decl: 14, sub: 0, line: 84 } |  |  | 0.495 |
| walker |  | 8400 | 14 | Code::CodeKey { rung: Doc, file: color.go, decl: 9, sub: 0, line: 44 } |  |  | 0.495 |
| walker |  | 8414 | 14 | Code::CodeKey { rung: Doc, file: color.go, decl: 12, sub: 0, line: 75 } |  |  | 0.495 |
| ns | 8420 |  | 381 | Platform matrix: build tags and per-OS terminal functions | 4.4 |  | 0.485 |
| walker |  | 8428 | 14 | Code::CodeKey { rung: Doc, file: color.go, decl: 15, sub: 0, line: 89 } |  |  | 0.485 |
| walker |  | 8444 | 16 | Code::CodeKey { rung: Doc, file: color.go, decl: 2, sub: 0, line: 13 } |  |  | 0.485 |
| walker |  | 8460 | 16 | Code::CodeKey { rung: Doc, file: color.go, decl: 4, sub: 0, line: 21 } |  |  | 0.485 |
| walker |  | 8476 | 16 | Code::CodeKey { rung: Doc, file: color.go, decl: 6, sub: 0, line: 29 } |  |  | 0.485 |
| walker |  | 8494 | 18 | Code::CodeKey { rung: Doc, file: color.go, decl: 1, sub: 0, line: 10 } |  |  | 0.485 |
| walker |  | 8512 | 18 | Code::CodeKey { rung: Doc, file: color.go, decl: 3, sub: 0, line: 18 } |  |  | 0.485 |
| walker |  | 8530 | 18 | Code::CodeKey { rung: Doc, file: color.go, decl: 5, sub: 0, line: 26 } |  |  | 0.485 |
| walker |  | 8564 | 34 | Code::CodeKey { rung: Doc, file: color.go, decl: 13, sub: 0, line: 81 } |  |  | 0.485 |
| walker |  | 8612 | 48 | Code::CodeKey { rung: Doc, file: color.go, decl: 7, sub: 0, line: 36 } |  |  | 0.485 |
| ns | 8618 |  | 198 | `tty.go`: terminal acquisition, input loop, resize detection | 4.5 |  | 0.481 |
| walker |  | 8658 | 46 | Code::CodeKey { rung: Names, file: tty_windows.go, decl: 0, sub: 0, line: 0 } |  |  | 0.483 |
| walker |  | 8748 | 90 | Code::CodeKey { rung: Doc, file: termcap.go, decl: 3, sub: 0, line: 41 } |  |  | 0.483 |
| ns | 8882 |  | 264 | `examples/` and `tutorials/` directory listings | 5.1 |  | 0.529 |
| ns | 9062 |  | 180 | `UPGRADE_GUIDE_V2.md`: complete section map | 5.2 |  | 0.523 |
| walker |  | 9264 | 516 | Code::CodeKey { rung: Decl, file: tea.go, decl: 7, sub: 0, line: 84 } |  |  | 0.524 |
| walker |  | 9303 | 39 | Code::CodeKey { rung: Doc, file: tea.go, decl: 7, sub: 0, line: 84 } |  |  | 0.528 |
| ns | 9347 |  | 285 | The v1→v2 migration checklist | 5.3 | 5.2 | 0.524 |
| walker |  | 9509 | 206 | Code::CodeKey { rung: Names, file: options.go, decl: 0, sub: 0, line: 0 } |  |  | 0.543 |
| ns | 9516 |  | 169 | README section map | 5.4 |  | 0.552 |
| walker |  | 9533 | 24 | Code::CodeKey { rung: Doc, file: options.go, decl: 8, sub: 0, line: 84 } |  |  | 0.552 |
| walker |  | 9560 | 27 | Code::CodeKey { rung: Body, file: options.go, decl: 3, sub: 0, line: 30 } |  |  | 0.552 |
| walker |  | 9587 | 27 | Code::CodeKey { rung: Body, file: options.go, decl: 5, sub: 0, line: 58 } |  |  | 0.552 |
| walker |  | 9614 | 27 | Code::CodeKey { rung: Body, file: options.go, decl: 10, sub: 0, line: 133 } |  |  | 0.552 |
| walker |  | 9648 | 34 | Code::CodeKey { rung: Doc, file: options.go, decl: 3, sub: 0, line: 30 } |  |  | 0.552 |
| walker |  | 9685 | 37 | Code::CodeKey { rung: Doc, file: options.go, decl: 6, sub: 0, line: 66 } |  |  | 0.552 |
| walker |  | 9742 | 57 | Code::CodeKey { rung: Doc, file: options.go, decl: 2, sub: 0, line: 22 } |  |  | 0.552 |
| walker |  | 9803 | 61 | Code::CodeKey { rung: Doc, file: options.go, decl: 4, sub: 0, line: 40 } |  |  | 0.552 |
| walker |  | 9864 | 61 | Code::CodeKey { rung: Doc, file: options.go, decl: 11, sub: 0, line: 142 } |  |  | 0.552 |
| ns | 9876 |  | 360 | Test suite: every test function, and the golden-file fixtures | 5.5 |  | 0.541 |
| ns | 9917 |  | 41 | CI workflow inventory and golden-test fixture directories | 5.6 |  | 0.545 |
| walker |  | 9932 | 68 | Code::CodeKey { rung: Doc, file: options.go, decl: 7, sub: 0, line: 76 } |  |  | 0.545 |
