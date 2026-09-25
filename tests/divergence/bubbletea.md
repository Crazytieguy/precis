Score(3000)=0.556 I=0.777 C=0.398 ns_rows≤3K=18/50 grid(1000/1442/2080/3000/4327/6240/9000)=0.591/0.559/0.612/0.556/0.530/0.511/0.562

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
| walker |  | 542 | 44 | GoMod::Identity { file: tutorials/go.mod } |  |  | 0.033 |
| walker |  | 554 | 12 | Fs::DirListing { dir: .github } |  |  | 0.033 |
| walker |  | 585 | 31 | Fs::DirListing { dir: .github/workflows } |  |  | 0.033 |
| ns | 652 |  | 202 | Complete root directory listing | 1.6 |  | 0.413 |
| walker |  | 657 | 72 | Fs::DirListing { dir: testdata/TestViewModel } |  |  | 0.413 |
| walker |  | 807 | 150 | Code::CodeKey { rung: ModuleDoc, file: tea.go, decl: 0, sub: 0, line: 0 } |  |  | 0.712 |
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
| walker |  | 1295 | 115 | Code::CodeKey { rung: Decl, file: tea.go, decl: 15, sub: 0, line: 336 } |  |  | 0.556 |
| walker |  | 1314 | 19 | Code::CodeKey { rung: Doc, file: tea.go, decl: 2, sub: 0, line: 42 } |  |  | 0.556 |
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
| ns | 2436 |  | 306 | Event-loop dispatch table: every internally-handled message type | 2.5 |  | 0.586 |
| walker |  | 2587 | 280 | Code::CodeKey { rung: Names, file: tea.go, decl: 0, sub: 1, line: 0 } |  |  | 0.594 |
| walker |  | 2614 | 27 | Code::CodeKey { rung: Decl, file: tea.go, decl: 20, sub: 0, line: 395 } |  |  | 0.594 |
| walker |  | 2626 | 12 | Code::CodeKey { rung: Doc, file: tea.go, decl: 33, sub: 0, line: 685 } |  |  | 0.594 |
| walker |  | 2639 | 13 | Code::CodeKey { rung: Doc, file: tea.go, decl: 22, sub: 0, line: 409 } |  |  | 0.594 |
| walker |  | 2652 | 13 | Code::CodeKey { rung: Doc, file: tea.go, decl: 31, sub: 0, line: 595 } |  |  | 0.598 |
| walker |  | 2669 | 17 | Code::CodeKey { rung: Doc, file: tea.go, decl: 26, sub: 0, line: 564 } |  |  | 0.598 |
| ns | 2682 |  | 246 | Event-loop skeleton: translate → filter → dispatch → update → render | 2.6 | 2.5 | 0.565 |
| walker |  | 2688 | 19 | Code::CodeKey { rung: Doc, file: tea.go, decl: 24, sub: 0, line: 555 } |  |  | 0.567 |
| walker |  | 2711 | 23 | Code::CodeKey { rung: Doc, file: tea.go, decl: 30, sub: 0, line: 590 } |  |  | 0.567 |
| walker |  | 2859 | 148 | Code::CodeKey { rung: Decl, file: tea.go, decl: 17, sub: 0, line: 357 } |  |  | 0.567 |
| walker |  | 2873 | 14 | Code::CodeKey { rung: Doc, file: tea.go, decl: 17, sub: 0, line: 357 } |  |  | 0.567 |
| walker |  | 2897 | 24 | Code::CodeKey { rung: Doc, file: tea.go, decl: 18, sub: 0, line: 374 } |  |  | 0.567 |
| ns | 2901 |  | 219 | Program control messages: suspend, resume, interrupt | 2.7 | 1.9 | 0.549 |
| walker |  | 2925 | 28 | Code::CodeKey { rung: Doc, file: tea.go, decl: 28, sub: 0, line: 578 } |  |  | 0.556 |
| walker |  | 2954 | 29 | Code::CodeKey { rung: Doc, file: tea.go, decl: 21, sub: 0, line: 402 } |  |  | 0.556 |
| walker |  | 2984 | 30 | Code::CodeKey { rung: Doc, file: tea.go, decl: 34, sub: 0, line: 700 } |  |  | 0.556 |
| walker |  | 3016 | 32 | Code::CodeKey { rung: Doc, file: tea.go, decl: 25, sub: 0, line: 561 } |  |  | 0.561 |
| walker |  | 3053 | 37 | Code::CodeKey { rung: Doc, file: tea.go, decl: 35, sub: 0, line: 743 } |  |  | 0.561 |
| walker |  | 3094 | 41 | Code::CodeKey { rung: Doc, file: tea.go, decl: 20, sub: 0, line: 395 } |  |  | 0.561 |
| ns | 3100 |  | 199 | `Run()`: how input and the TTY are acquired | 2.8 |  | 0.542 |
| ns | 3288 |  | 188 | `Run()`: renderer choice, colour profile, first three messages | 2.9 | 2.8 | 0.526 |
| walker |  | 3336 | 242 | Code::CodeKey { rung: Names, file: tea.go, decl: 0, sub: 2, line: 0 } |  |  | 0.541 |
| walker |  | 3350 | 14 | Code::CodeKey { rung: Doc, file: tea.go, decl: 36, sub: 0, line: 886 } |  |  | 0.541 |
| walker |  | 3366 | 16 | Code::CodeKey { rung: Doc, file: tea.go, decl: 45, sub: 0, line: 1214 } |  |  | 0.541 |
| walker |  | 3383 | 17 | Code::CodeKey { rung: Doc, file: tea.go, decl: 46, sub: 0, line: 1221 } |  |  | 0.541 |
| walker |  | 3401 | 18 | Code::CodeKey { rung: Doc, file: tea.go, decl: 44, sub: 0, line: 1209 } |  |  | 0.541 |
| walker |  | 3430 | 29 | Code::CodeKey { rung: Doc, file: tea.go, decl: 47, sub: 0, line: 1241 } |  |  | 0.541 |
| walker |  | 3467 | 37 | Code::CodeKey { rung: Doc, file: tea.go, decl: 48, sub: 0, line: 1269 } |  |  | 0.541 |
| ns | 3474 |  | 186 | `Run()`: model init, event loop, graceful shutdown | 2.10 | 2.9 | 0.522 |
| walker |  | 3520 | 53 | Code::CodeKey { rung: Doc, file: tea.go, decl: 40, sub: 0, line: 991 } |  |  | 0.522 |
| walker |  | 3577 | 57 | Code::CodeKey { rung: Doc, file: tea.go, decl: 43, sub: 0, line: 1204 } |  |  | 0.522 |
| ns | 3693 |  | 219 | Debug hooks: `TEA_TRACE`, `TEA_DEBUG`, panic recovery | 2.11 |  | 0.512 |
| walker |  | 3735 | 158 | Code::CodeKey { rung: Names, file: tea.go, decl: 0, sub: 3, line: 0 } |  |  | 0.530 |
| walker |  | 3747 | 12 | Code::CodeKey { rung: Doc, file: tea.go, decl: 55, sub: 0, line: 1393 } |  |  | 0.530 |
| walker |  | 3785 | 38 | Code::CodeKey { rung: Doc, file: tea.go, decl: 50, sub: 0, line: 1319 } |  |  | 0.530 |
| walker |  | 3827 | 42 | Code::CodeKey { rung: Doc, file: tea.go, decl: 56, sub: 0, line: 1427 } |  |  | 0.530 |
| walker |  | 3873 | 46 | Code::CodeKey { rung: Doc, file: tea.go, decl: 49, sub: 0, line: 1294 } |  |  | 0.530 |
| ns | 3916 |  | 223 | Frame pacing: the render ticker and the FPS bounds | 2.12 |  | 0.513 |
| walker |  | 3932 | 59 | Code::CodeKey { rung: Doc, file: tea.go, decl: 52, sub: 0, line: 1344 } |  |  | 0.513 |
| walker |  | 3993 | 61 | Code::CodeKey { rung: Doc, file: tea.go, decl: 53, sub: 0, line: 1372 } |  |  | 0.513 |
| walker |  | 4075 | 82 | Code::CodeKey { rung: Doc, file: tea.go, decl: 29, sub: 0, line: 586 } |  |  | 0.523 |
| ns | 4118 |  | 202 | Input event message types: keys, mouse, paste, focus (complete) | 3.1 |  | 0.507 |
| walker |  | 4160 | 85 | Code::CodeKey { rung: Doc, file: tea.go, decl: 27, sub: 0, line: 574 } |  |  | 0.519 |
| walker |  | 4258 | 98 | Code::CodeKey { rung: Doc, file: tea.go, decl: 6, sub: 0, line: 76 } |  |  | 0.530 |
| walker |  | 4365 | 107 | Code::CodeKey { rung: Doc, file: tea.go, decl: 9, sub: 0, line: 279 } |  |  | 0.530 |
| ns | 4380 |  | 262 | Terminal report message types: colour, clipboard, size, capability (complete) | 3.2 |  | 0.512 |
| walker |  | 4473 | 108 | Code::CodeKey { rung: Doc, file: tea.go, decl: 41, sub: 0, line: 1183 } |  |  | 0.512 |
| ns | 4550 |  | 170 | Command constructors, part 1: lifecycle, batching, timing, output | 3.3 | 2.7 | 0.503 |
| ns | 4696 |  | 146 | Command constructors, part 2: terminal queries and clipboard | 3.4 | 3.3 | 0.495 |
| walker |  | 4719 | 246 | GoMod::File { file: go.mod } |  |  | 0.517 |
| walker |  | 4832 | 113 | Code::CodeKey { rung: Doc, file: tea.go, decl: 19, sub: 0, line: 390 } |  |  | 0.555 |
| walker |  | 4948 | 116 | Code::CodeKey { rung: Doc, file: tea.go, decl: 42, sub: 0, line: 1197 } |  |  | 0.555 |
| ns | 5037 |  | 341 | `input.go`: the ultraviolet-event → `tea.Msg` translation table | 3.5 |  | 0.533 |
| walker |  | 5068 | 120 | Code::CodeKey { rung: Doc, file: tea.go, decl: 54, sub: 0, line: 1386 } |  |  | 0.533 |
| walker |  | 5090 | 22 | Code::CodeKey { rung: Names, file: focus.go, decl: 0, sub: 0, line: 0 } |  |  | 0.533 |
| ns | 5113 |  | 76 | `Key` struct fields | 3.6 |  | 0.527 |
| walker |  | 5114 | 24 | Code::CodeKey { rung: Doc, file: focus.go, decl: 2, sub: 0, line: 9 } |  |  | 0.527 |
| walker |  | 5140 | 26 | Code::CodeKey { rung: Doc, file: focus.go, decl: 1, sub: 0, line: 5 } |  |  | 0.527 |
| ns | 5293 |  | 180 | Key message methods: `String`, `Keystroke`, `Key` | 3.7 | 3.6 | 0.520 |
| walker |  | 5390 | 250 | Fs::DirListing { dir: examples } |  |  | 0.525 |
| walker |  | 5440 | 50 | Code::CodeKey { rung: Names, file: screen.go, decl: 0, sub: 0, line: 0 } |  |  | 0.526 |
| walker |  | 5462 | 22 | Code::CodeKey { rung: Decl, file: screen.go, decl: 1, sub: 0, line: 9 } |  |  | 0.526 |
| walker |  | 5512 | 50 | Code::CodeKey { rung: Decl, file: screen.go, decl: 4, sub: 0, line: 62 } |  |  | 0.526 |
| walker |  | 5521 | 9 | Code::CodeKey { rung: Body, file: screen.go, decl: 2, sub: 0, line: 20 } |  |  | 0.526 |
| walker |  | 5556 | 35 | Code::CodeKey { rung: Doc, file: screen.go, decl: 3, sub: 0, line: 26 } |  |  | 0.526 |
| walker |  | 5607 | 51 | Code::CodeKey { rung: Names, file: paste.go, decl: 0, sub: 0, line: 0 } |  |  | 0.529 |
| ns | 5614 |  | 321 | `key.go` key-code catalog: group headings plus the C0/G0 names | 3.8 |  | 0.508 |
| walker |  | 5619 | 12 | Code::CodeKey { rung: Decl, file: paste.go, decl: 1, sub: 0, line: 5 } |  |  | 0.508 |
| walker |  | 5627 | 8 | Code::CodeKey { rung: Body, file: paste.go, decl: 2, sub: 0, line: 10 } |  |  | 0.508 |
| walker |  | 5639 | 12 | Code::CodeKey { rung: Doc, file: paste.go, decl: 2, sub: 0, line: 10 } |  |  | 0.508 |
| walker |  | 5668 | 29 | Code::CodeKey { rung: Doc, file: paste.go, decl: 4, sub: 0, line: 20 } |  |  | 0.508 |
| walker |  | 5699 | 31 | Code::CodeKey { rung: Doc, file: paste.go, decl: 1, sub: 0, line: 5 } |  |  | 0.508 |
| walker |  | 5730 | 31 | Code::CodeKey { rung: Doc, file: paste.go, decl: 3, sub: 0, line: 16 } |  |  | 0.508 |
| walker |  | 5756 | 26 | Code::CodeKey { rung: Names, file: raw.go, decl: 0, sub: 0, line: 0 } |  |  | 0.509 |
| walker |  | 5769 | 13 | Code::CodeKey { rung: Decl, file: raw.go, decl: 1, sub: 0, line: 5 } |  |  | 0.509 |
| walker |  | 5795 | 26 | Code::CodeKey { rung: Body, file: raw.go, decl: 2, sub: 0, line: 33 } |  |  | 0.509 |
| ns | 5797 |  | 183 | Modifier keys: the complete `KeyMod` constant set | 3.9 |  | 0.500 |
| walker |  | 5826 | 31 | Code::CodeKey { rung: Doc, file: raw.go, decl: 1, sub: 0, line: 5 } |  |  | 0.500 |
| ns | 6023 |  | 226 | `Mouse` struct and the complete mouse-button constant set | 3.10 | 3.1 | 0.490 |
| walker |  | 6072 | 246 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.491 |
| ns | 6200 |  | 177 | `MouseMode` enum: none / cell-motion / all-motion | 3.11 | 1.7 | 0.505 |
| walker |  | 6221 | 149 | Code::CodeKey { rung: Names, file: clipboard.go, decl: 0, sub: 0, line: 0 } |  |  | 0.509 |
| walker |  | 6242 | 21 | Code::CodeKey { rung: Decl, file: clipboard.go, decl: 1, sub: 0, line: 5 } |  |  | 0.511 |
| walker |  | 6250 | 8 | Code::CodeKey { rung: Body, file: clipboard.go, decl: 2, sub: 0, line: 15 } |  |  | 0.511 |
| walker |  | 6258 | 8 | Code::CodeKey { rung: Body, file: clipboard.go, decl: 3, sub: 0, line: 20 } |  |  | 0.511 |
| walker |  | 6267 | 9 | Code::CodeKey { rung: Body, file: clipboard.go, decl: 7, sub: 0, line: 42 } |  |  | 0.511 |
| walker |  | 6277 | 10 | Code::CodeKey { rung: Body, file: clipboard.go, decl: 11, sub: 0, line: 68 } |  |  | 0.511 |
| walker |  | 6292 | 15 | Code::CodeKey { rung: Doc, file: clipboard.go, decl: 3, sub: 0, line: 20 } |  |  | 0.511 |
| walker |  | 6320 | 28 | Code::CodeKey { rung: Doc, file: clipboard.go, decl: 4, sub: 0, line: 26 } |  |  | 0.511 |
| walker |  | 6348 | 28 | Code::CodeKey { rung: Doc, file: clipboard.go, decl: 6, sub: 0, line: 38 } |  |  | 0.511 |
| walker |  | 6377 | 29 | Code::CodeKey { rung: Doc, file: clipboard.go, decl: 8, sub: 0, line: 48 } |  |  | 0.511 |
| ns | 6378 |  | 178 | Cursor value types: `Cursor`, `NewCursor`, `CursorShape` | 3.12 | 1.7 | 0.507 |
| walker |  | 6406 | 29 | Code::CodeKey { rung: Doc, file: clipboard.go, decl: 10, sub: 0, line: 62 } |  |  | 0.507 |
| walker |  | 6439 | 33 | Code::CodeKey { rung: Doc, file: clipboard.go, decl: 5, sub: 0, line: 30 } |  |  | 0.507 |
| walker |  | 6472 | 33 | Code::CodeKey { rung: Doc, file: clipboard.go, decl: 7, sub: 0, line: 42 } |  |  | 0.507 |
| walker |  | 6508 | 36 | Code::CodeKey { rung: Doc, file: clipboard.go, decl: 1, sub: 0, line: 5 } |  |  | 0.507 |
| ns | 6566 |  | 188 | Keyboard enhancements: request fields and response predicates | 3.13 | 3.2 | 0.502 |
| walker |  | 6569 | 61 | Code::CodeKey { rung: Doc, file: clipboard.go, decl: 2, sub: 0, line: 15 } |  |  | 0.502 |
| walker |  | 6630 | 61 | Code::CodeKey { rung: Doc, file: clipboard.go, decl: 9, sub: 0, line: 54 } |  |  | 0.502 |
| walker |  | 6691 | 61 | Code::CodeKey { rung: Doc, file: clipboard.go, decl: 11, sub: 0, line: 68 } |  |  | 0.502 |
| walker |  | 6746 | 55 | Code::CodeKey { rung: Names, file: termcap.go, decl: 0, sub: 0, line: 0 } |  |  | 0.505 |
| walker |  | 6758 | 12 | Code::CodeKey { rung: Decl, file: termcap.go, decl: 3, sub: 0, line: 41 } |  |  | 0.506 |
| ns | 6759 |  | 193 | `Batch` vs `Sequence` semantics and where they execute | 3.14 | 3.3 | 0.501 |
| walker |  | 6766 | 8 | Code::CodeKey { rung: Body, file: termcap.go, decl: 4, sub: 0, line: 46 } |  |  | 0.501 |
| walker |  | 6778 | 12 | Code::CodeKey { rung: Doc, file: termcap.go, decl: 4, sub: 0, line: 46 } |  |  | 0.501 |
| walker |  | 6804 | 26 | Code::CodeKey { rung: Body, file: termcap.go, decl: 2, sub: 0, line: 30 } |  |  | 0.501 |
| walker |  | 6836 | 32 | Code::CodeKey { rung: Doc, file: termcap.go, decl: 1, sub: 0, line: 5 } |  |  | 0.501 |
| walker |  | 6891 | 55 | Code::CodeKey { rung: Names, file: xterm.go, decl: 0, sub: 0, line: 0 } |  |  | 0.505 |
| walker |  | 6903 | 12 | Code::CodeKey { rung: Decl, file: xterm.go, decl: 1, sub: 0, line: 4 } |  |  | 0.506 |
| walker |  | 6911 | 8 | Code::CodeKey { rung: Body, file: xterm.go, decl: 2, sub: 0, line: 9 } |  |  | 0.506 |
| walker |  | 6919 | 8 | Code::CodeKey { rung: Body, file: xterm.go, decl: 4, sub: 0, line: 20 } |  |  | 0.506 |
| walker |  | 6931 | 12 | Code::CodeKey { rung: Doc, file: xterm.go, decl: 2, sub: 0, line: 9 } |  |  | 0.506 |
| walker |  | 6948 | 17 | Code::CodeKey { rung: Doc, file: xterm.go, decl: 1, sub: 0, line: 4 } |  |  | 0.506 |
| ns | 6959 |  | 200 | `Exec` / `ExecProcess` and the `ExecCommand` interface | 3.15 | 3.3 | 0.499 |
| walker |  | 6977 | 29 | Code::CodeKey { rung: Doc, file: xterm.go, decl: 3, sub: 0, line: 15 } |  |  | 0.499 |
| walker |  | 7020 | 43 | Code::CodeKey { rung: Doc, file: xterm.go, decl: 4, sub: 0, line: 20 } |  |  | 0.499 |
| walker |  | 7034 | 14 | Code::CodeKey { rung: Names, file: profile.go, decl: 0, sub: 0, line: 0 } |  |  | 0.501 |
| walker |  | 7045 | 11 | Code::CodeKey { rung: Decl, file: profile.go, decl: 1, sub: 0, line: 13 } |  |  | 0.502 |
| walker |  | 7090 | 45 | Code::CodeKey { rung: Names, file: tty_unix.go, decl: 0, sub: 0, line: 0 } |  |  | 0.502 |
| walker |  | 7104 | 14 | Code::CodeKey { rung: Doc, file: tty_unix.go, decl: 3, sub: 0, line: 40 } |  |  | 0.502 |
| ns | 7111 |  | 152 | Logging to a file: `LogToFile` / `LogToFileWith` | 3.16 |  | 0.498 |
| walker |  | 7175 | 71 | Code::CodeKey { rung: Doc, file: screen.go, decl: 1, sub: 0, line: 9 } |  |  | 0.498 |
| walker |  | 7225 | 50 | GoMod::File { file: tutorials/go.mod } |  |  | 0.498 |
| ns | 7328 |  | 217 | The `renderer` interface: complete method set, and its two implementations | 4.1 |  | 0.489 |
| walker |  | 7455 | 230 | Code::CodeKey { rung: Names, file: color.go, decl: 0, sub: 0, line: 0 } |  |  | 0.504 |
| walker |  | 7464 | 9 | Code::CodeKey { rung: Body, file: color.go, decl: 2, sub: 0, line: 13 } |  |  | 0.504 |
| walker |  | 7473 | 9 | Code::CodeKey { rung: Body, file: color.go, decl: 4, sub: 0, line: 21 } |  |  | 0.504 |
| walker |  | 7482 | 9 | Code::CodeKey { rung: Body, file: color.go, decl: 6, sub: 0, line: 29 } |  |  | 0.504 |
| walker |  | 7494 | 12 | Code::CodeKey { rung: Doc, file: color.go, decl: 8, sub: 0, line: 39 } |  |  | 0.504 |
| walker |  | 7506 | 12 | Code::CodeKey { rung: Doc, file: color.go, decl: 11, sub: 0, line: 70 } |  |  | 0.504 |
| walker |  | 7518 | 12 | Code::CodeKey { rung: Doc, file: color.go, decl: 14, sub: 0, line: 84 } |  |  | 0.504 |
| walker |  | 7532 | 14 | Code::CodeKey { rung: Doc, file: color.go, decl: 9, sub: 0, line: 44 } |  |  | 0.504 |
| walker |  | 7546 | 14 | Code::CodeKey { rung: Doc, file: color.go, decl: 12, sub: 0, line: 75 } |  |  | 0.504 |
| walker |  | 7560 | 14 | Code::CodeKey { rung: Doc, file: color.go, decl: 15, sub: 0, line: 89 } |  |  | 0.504 |
| walker |  | 7576 | 16 | Code::CodeKey { rung: Doc, file: color.go, decl: 2, sub: 0, line: 13 } |  |  | 0.504 |
| walker |  | 7592 | 16 | Code::CodeKey { rung: Doc, file: color.go, decl: 4, sub: 0, line: 21 } |  |  | 0.504 |
| walker |  | 7608 | 16 | Code::CodeKey { rung: Doc, file: color.go, decl: 6, sub: 0, line: 29 } |  |  | 0.504 |
| walker |  | 7626 | 18 | Code::CodeKey { rung: Doc, file: color.go, decl: 1, sub: 0, line: 10 } |  |  | 0.504 |
| walker |  | 7644 | 18 | Code::CodeKey { rung: Doc, file: color.go, decl: 3, sub: 0, line: 18 } |  |  | 0.504 |
| walker |  | 7662 | 18 | Code::CodeKey { rung: Doc, file: color.go, decl: 5, sub: 0, line: 26 } |  |  | 0.504 |
| ns | 7681 |  | 353 | `cursedRenderer`: every function in the 860-line renderer | 4.2 | 4.1 | 0.493 |
| walker |  | 7696 | 34 | Code::CodeKey { rung: Doc, file: color.go, decl: 13, sub: 0, line: 81 } |  |  | 0.493 |
| walker |  | 7744 | 48 | Code::CodeKey { rung: Doc, file: color.go, decl: 7, sub: 0, line: 36 } |  |  | 0.493 |
| walker |  | 7790 | 46 | Code::CodeKey { rung: Names, file: tty_windows.go, decl: 0, sub: 0, line: 0 } |  |  | 0.494 |
| walker |  | 7832 | 42 | Markdown::Section { file: README.md, section_index: 16, keeps_default_concavity: false } |  |  | 0.494 |
| walker |  | 8038 | 206 | Code::CodeKey { rung: Names, file: options.go, decl: 0, sub: 0, line: 0 } |  |  | 0.517 |
| ns | 8039 |  | 358 | `View` fields → ANSI sequences, in `cursedRenderer.start` | 4.3 | 4.2 | 0.506 |
| walker |  | 8062 | 24 | Code::CodeKey { rung: Doc, file: options.go, decl: 8, sub: 0, line: 84 } |  |  | 0.506 |
| walker |  | 8089 | 27 | Code::CodeKey { rung: Body, file: options.go, decl: 3, sub: 0, line: 30 } |  |  | 0.506 |
| walker |  | 8116 | 27 | Code::CodeKey { rung: Body, file: options.go, decl: 5, sub: 0, line: 58 } |  |  | 0.506 |
| walker |  | 8143 | 27 | Code::CodeKey { rung: Body, file: options.go, decl: 10, sub: 0, line: 133 } |  |  | 0.506 |
| walker |  | 8177 | 34 | Code::CodeKey { rung: Doc, file: options.go, decl: 3, sub: 0, line: 30 } |  |  | 0.506 |
| walker |  | 8214 | 37 | Code::CodeKey { rung: Doc, file: options.go, decl: 6, sub: 0, line: 66 } |  |  | 0.506 |
| walker |  | 8271 | 57 | Code::CodeKey { rung: Doc, file: options.go, decl: 2, sub: 0, line: 22 } |  |  | 0.506 |
| walker |  | 8332 | 61 | Code::CodeKey { rung: Doc, file: options.go, decl: 4, sub: 0, line: 40 } |  |  | 0.506 |
| walker |  | 8393 | 61 | Code::CodeKey { rung: Doc, file: options.go, decl: 11, sub: 0, line: 142 } |  |  | 0.506 |
| ns | 8420 |  | 381 | Platform matrix: build tags and per-OS terminal functions | 4.4 |  | 0.498 |
| walker |  | 8461 | 68 | Code::CodeKey { rung: Doc, file: options.go, decl: 7, sub: 0, line: 76 } |  |  | 0.498 |
| walker |  | 8531 | 70 | Code::CodeKey { rung: Doc, file: options.go, decl: 13, sub: 0, line: 163 } |  |  | 0.498 |
| walker |  | 8604 | 73 | Code::CodeKey { rung: Doc, file: options.go, decl: 1, sub: 0, line: 17 } |  |  | 0.498 |
| ns | 8618 |  | 198 | `tty.go`: terminal acquisition, input loop, resize detection | 4.5 |  | 0.493 |
| walker |  | 8749 | 145 | Code::CodeKey { rung: Names, file: commands.go, decl: 0, sub: 0, line: 0 } |  |  | 0.502 |
| walker |  | 8758 | 9 | Code::CodeKey { rung: Body, file: commands.go, decl: 9, sub: 0, line: 173 } |  |  | 0.502 |
| walker |  | 8773 | 15 | Code::CodeKey { rung: Body, file: commands.go, decl: 1, sub: 0, line: 15 } |  |  | 0.502 |
| walker |  | 8788 | 15 | Code::CodeKey { rung: Body, file: commands.go, decl: 3, sub: 0, line: 25 } |  |  | 0.502 |
| walker |  | 8806 | 18 | Code::CodeKey { rung: Doc, file: commands.go, decl: 4, sub: 0, line: 30 } |  |  | 0.503 |
| walker |  | 8838 | 32 | Code::CodeKey { rung: Doc, file: commands.go, decl: 3, sub: 0, line: 25 } |  |  | 0.506 |
| walker |  | 8876 | 38 | Code::CodeKey { rung: Doc, file: commands.go, decl: 2, sub: 0, line: 21 } |  |  | 0.506 |
| ns | 8882 |  | 264 | `examples/` and `tutorials/` directory listings | 5.1 |  | 0.549 |
| walker |  | 8973 | 97 | Code::CodeKey { rung: Names, file: cursor.go, decl: 0, sub: 0, line: 0 } |  |  | 0.560 |
| walker |  | 8984 | 11 | Code::CodeKey { rung: Decl, file: cursor.go, decl: 4, sub: 0, line: 15 } |  |  | 0.562 |
| walker |  | 8998 | 14 | Code::CodeKey { rung: Decl, file: cursor.go, decl: 2, sub: 0, line: 7 } |  |  | 0.562 |
| walker |  | 9004 | 6 | Code::CodeKey { rung: Doc, file: cursor.go, decl: 4, sub: 0, line: 15 } |  |  | 0.564 |
| walker |  | 9014 | 10 | Code::CodeKey { rung: Body, file: cursor.go, decl: 6, sub: 0, line: 26 } |  |  | 0.564 |
| walker |  | 9025 | 11 | Code::CodeKey { rung: Doc, file: cursor.go, decl: 3, sub: 0, line: 12 } |  |  | 0.567 |
| walker |  | 9038 | 13 | Code::CodeKey { rung: Doc, file: cursor.go, decl: 1, sub: 0, line: 4 } |  |  | 0.567 |
| walker |  | 9054 | 16 | Code::CodeKey { rung: Doc, file: cursor.go, decl: 2, sub: 0, line: 7 } |  |  | 0.567 |
| ns | 9062 |  | 180 | `UPGRADE_GUIDE_V2.md`: complete section map | 5.2 |  | 0.561 |
| walker |  | 9070 | 16 | Code::CodeKey { rung: Doc, file: cursor.go, decl: 5, sub: 0, line: 22 } |  |  | 0.561 |
| walker |  | 9105 | 35 | Code::CodeKey { rung: Doc, file: cursor.go, decl: 6, sub: 0, line: 26 } |  |  | 0.561 |
| walker |  | 9286 | 181 | Code::CodeKey { rung: Doc, file: tea.go, decl: 39, sub: 0, line: 972 } |  |  | 0.561 |
| ns | 9347 |  | 285 | The v1→v2 migration checklist | 5.3 | 5.2 | 0.556 |
| walker |  | 9385 | 99 | Code::CodeKey { rung: Names, file: renderer.go, decl: 0, sub: 0, line: 0 } |  |  | 0.560 |
| walker |  | 9398 | 13 | Code::CodeKey { rung: Decl, file: renderer.go, decl: 3, sub: 0, line: 59 } |  |  | 0.560 |
| walker |  | 9437 | 39 | Code::CodeKey { rung: Decl, file: renderer.go, decl: 1, sub: 0, line: 10 } |  |  | 0.562 |
| walker |  | 9464 | 27 | Code::CodeKey { rung: Doc, file: renderer.go, decl: 6, sub: 0, line: 96 } |  |  | 0.562 |
| walker |  | 9510 | 46 | Code::CodeKey { rung: Body, file: renderer.go, decl: 4, sub: 0, line: 70 } |  |  | 0.562 |
| ns | 9516 |  | 169 | README section map | 5.4 |  | 0.570 |
| ns | 9876 |  | 360 | Test suite: every test function, and the golden-file fixtures | 5.5 |  | 0.559 |
| walker |  | 9894 | 384 | Code::CodeKey { rung: Decl, file: renderer.go, decl: 2, sub: 0, line: 18 } |  |  | 0.567 |
| walker |  | 9907 | 13 | Code::CodeKey { rung: Doc, file: renderer.go, decl: 2, sub: 0, line: 18 } |  |  | 0.570 |
| ns | 9917 |  | 41 | CI workflow inventory and golden-test fixture directories | 5.6 |  | 0.573 |
| walker |  | 9984 | 77 | Code::CodeKey { rung: Body, file: tty_unix.go, decl: 3, sub: 0, line: 40 } |  |  | 0.573 |
