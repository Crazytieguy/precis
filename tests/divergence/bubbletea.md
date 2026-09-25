Score(3000)=0.549 I=0.775 C=0.389 ns_rows≤3K=18/50 grid(1000/1442/2080/3000/4327/6240/9000)=0.584/0.531/0.612/0.549/0.519/0.492/0.555

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
| ns | 2682 |  | 246 | Event-loop skeleton: translate → filter → dispatch → update → render | 2.6 | 2.5 | 0.552 |
| walker |  | 2724 | 280 | Code::CodeKey { rung: Names, file: tea.go, decl: 0, sub: 1, line: 0 } |  |  | 0.560 |
| walker |  | 2751 | 27 | Code::CodeKey { rung: Decl, file: tea.go, decl: 20, sub: 0, line: 395 } |  |  | 0.560 |
| walker |  | 2763 | 12 | Code::CodeKey { rung: Doc, file: tea.go, decl: 33, sub: 0, line: 685 } |  |  | 0.560 |
| walker |  | 2776 | 13 | Code::CodeKey { rung: Doc, file: tea.go, decl: 22, sub: 0, line: 409 } |  |  | 0.560 |
| walker |  | 2789 | 13 | Code::CodeKey { rung: Doc, file: tea.go, decl: 31, sub: 0, line: 595 } |  |  | 0.564 |
| walker |  | 2806 | 17 | Code::CodeKey { rung: Doc, file: tea.go, decl: 26, sub: 0, line: 564 } |  |  | 0.565 |
| walker |  | 2825 | 19 | Code::CodeKey { rung: Doc, file: tea.go, decl: 24, sub: 0, line: 555 } |  |  | 0.567 |
| walker |  | 2848 | 23 | Code::CodeKey { rung: Doc, file: tea.go, decl: 30, sub: 0, line: 590 } |  |  | 0.567 |
| ns | 2901 |  | 219 | Program control messages: suspend, resume, interrupt | 2.7 | 1.9 | 0.549 |
| walker |  | 2996 | 148 | Code::CodeKey { rung: Decl, file: tea.go, decl: 17, sub: 0, line: 357 } |  |  | 0.549 |
| walker |  | 3010 | 14 | Code::CodeKey { rung: Doc, file: tea.go, decl: 17, sub: 0, line: 357 } |  |  | 0.549 |
| walker |  | 3034 | 24 | Code::CodeKey { rung: Doc, file: tea.go, decl: 18, sub: 0, line: 374 } |  |  | 0.549 |
| walker |  | 3062 | 28 | Code::CodeKey { rung: Doc, file: tea.go, decl: 28, sub: 0, line: 578 } |  |  | 0.556 |
| walker |  | 3091 | 29 | Code::CodeKey { rung: Doc, file: tea.go, decl: 21, sub: 0, line: 402 } |  |  | 0.556 |
| ns | 3100 |  | 199 | `Run()`: how input and the TTY are acquired | 2.8 |  | 0.537 |
| walker |  | 3121 | 30 | Code::CodeKey { rung: Doc, file: tea.go, decl: 34, sub: 0, line: 700 } |  |  | 0.537 |
| walker |  | 3153 | 32 | Code::CodeKey { rung: Doc, file: tea.go, decl: 25, sub: 0, line: 561 } |  |  | 0.542 |
| walker |  | 3190 | 37 | Code::CodeKey { rung: Doc, file: tea.go, decl: 35, sub: 0, line: 743 } |  |  | 0.542 |
| walker |  | 3231 | 41 | Code::CodeKey { rung: Doc, file: tea.go, decl: 20, sub: 0, line: 395 } |  |  | 0.542 |
| ns | 3288 |  | 188 | `Run()`: renderer choice, colour profile, first three messages | 2.9 | 2.8 | 0.526 |
| walker |  | 3473 | 242 | Code::CodeKey { rung: Names, file: tea.go, decl: 0, sub: 2, line: 0 } |  |  | 0.541 |
| ns | 3474 |  | 186 | `Run()`: model init, event loop, graceful shutdown | 2.10 | 2.9 | 0.522 |
| walker |  | 3487 | 14 | Code::CodeKey { rung: Doc, file: tea.go, decl: 36, sub: 0, line: 886 } |  |  | 0.522 |
| walker |  | 3503 | 16 | Code::CodeKey { rung: Doc, file: tea.go, decl: 45, sub: 0, line: 1214 } |  |  | 0.522 |
| walker |  | 3520 | 17 | Code::CodeKey { rung: Doc, file: tea.go, decl: 46, sub: 0, line: 1221 } |  |  | 0.522 |
| walker |  | 3538 | 18 | Code::CodeKey { rung: Doc, file: tea.go, decl: 44, sub: 0, line: 1209 } |  |  | 0.522 |
| walker |  | 3567 | 29 | Code::CodeKey { rung: Doc, file: tea.go, decl: 47, sub: 0, line: 1241 } |  |  | 0.522 |
| walker |  | 3604 | 37 | Code::CodeKey { rung: Doc, file: tea.go, decl: 48, sub: 0, line: 1269 } |  |  | 0.522 |
| walker |  | 3657 | 53 | Code::CodeKey { rung: Doc, file: tea.go, decl: 40, sub: 0, line: 991 } |  |  | 0.522 |
| ns | 3693 |  | 219 | Debug hooks: `TEA_TRACE`, `TEA_DEBUG`, panic recovery | 2.11 |  | 0.512 |
| walker |  | 3714 | 57 | Code::CodeKey { rung: Doc, file: tea.go, decl: 43, sub: 0, line: 1204 } |  |  | 0.512 |
| walker |  | 3872 | 158 | Code::CodeKey { rung: Names, file: tea.go, decl: 0, sub: 3, line: 0 } |  |  | 0.530 |
| walker |  | 3884 | 12 | Code::CodeKey { rung: Doc, file: tea.go, decl: 55, sub: 0, line: 1393 } |  |  | 0.530 |
| ns | 3916 |  | 223 | Frame pacing: the render ticker and the FPS bounds | 2.12 |  | 0.513 |
| walker |  | 3922 | 38 | Code::CodeKey { rung: Doc, file: tea.go, decl: 50, sub: 0, line: 1319 } |  |  | 0.513 |
| walker |  | 3964 | 42 | Code::CodeKey { rung: Doc, file: tea.go, decl: 56, sub: 0, line: 1427 } |  |  | 0.513 |
| walker |  | 4010 | 46 | Code::CodeKey { rung: Doc, file: tea.go, decl: 49, sub: 0, line: 1294 } |  |  | 0.513 |
| walker |  | 4069 | 59 | Code::CodeKey { rung: Doc, file: tea.go, decl: 52, sub: 0, line: 1344 } |  |  | 0.513 |
| ns | 4118 |  | 202 | Input event message types: keys, mouse, paste, focus (complete) | 3.1 |  | 0.497 |
| walker |  | 4130 | 61 | Code::CodeKey { rung: Doc, file: tea.go, decl: 53, sub: 0, line: 1372 } |  |  | 0.497 |
| walker |  | 4212 | 82 | Code::CodeKey { rung: Doc, file: tea.go, decl: 29, sub: 0, line: 586 } |  |  | 0.507 |
| walker |  | 4297 | 85 | Code::CodeKey { rung: Doc, file: tea.go, decl: 27, sub: 0, line: 574 } |  |  | 0.519 |
| ns | 4380 |  | 262 | Terminal report message types: colour, clipboard, size, capability (complete) | 3.2 |  | 0.501 |
| walker |  | 4395 | 98 | Code::CodeKey { rung: Doc, file: tea.go, decl: 6, sub: 0, line: 76 } |  |  | 0.512 |
| walker |  | 4502 | 107 | Code::CodeKey { rung: Doc, file: tea.go, decl: 9, sub: 0, line: 279 } |  |  | 0.512 |
| ns | 4550 |  | 170 | Command constructors, part 1: lifecycle, batching, timing, output | 3.3 | 2.7 | 0.503 |
| walker |  | 4610 | 108 | Code::CodeKey { rung: Doc, file: tea.go, decl: 41, sub: 0, line: 1183 } |  |  | 0.503 |
| ns | 4696 |  | 146 | Command constructors, part 2: terminal queries and clipboard | 3.4 | 3.3 | 0.495 |
| walker |  | 4723 | 113 | Code::CodeKey { rung: Doc, file: tea.go, decl: 19, sub: 0, line: 390 } |  |  | 0.533 |
| walker |  | 4839 | 116 | Code::CodeKey { rung: Doc, file: tea.go, decl: 42, sub: 0, line: 1197 } |  |  | 0.533 |
| walker |  | 4959 | 120 | Code::CodeKey { rung: Doc, file: tea.go, decl: 54, sub: 0, line: 1386 } |  |  | 0.533 |
| walker |  | 4981 | 22 | Code::CodeKey { rung: Names, file: focus.go, decl: 0, sub: 0, line: 0 } |  |  | 0.533 |
| walker |  | 5005 | 24 | Code::CodeKey { rung: Doc, file: focus.go, decl: 2, sub: 0, line: 9 } |  |  | 0.533 |
| walker |  | 5031 | 26 | Code::CodeKey { rung: Doc, file: focus.go, decl: 1, sub: 0, line: 5 } |  |  | 0.533 |
| ns | 5037 |  | 341 | `input.go`: the ultraviolet-event → `tea.Msg` translation table | 3.5 |  | 0.512 |
| ns | 5113 |  | 76 | `Key` struct fields | 3.6 |  | 0.506 |
| walker |  | 5281 | 250 | Fs::DirListing { dir: examples } |  |  | 0.510 |
| ns | 5293 |  | 180 | Key message methods: `String`, `Keystroke`, `Key` | 3.7 | 3.6 | 0.504 |
| walker |  | 5331 | 50 | Code::CodeKey { rung: Names, file: screen.go, decl: 0, sub: 0, line: 0 } |  |  | 0.505 |
| walker |  | 5353 | 22 | Code::CodeKey { rung: Decl, file: screen.go, decl: 1, sub: 0, line: 9 } |  |  | 0.505 |
| walker |  | 5403 | 50 | Code::CodeKey { rung: Decl, file: screen.go, decl: 4, sub: 0, line: 62 } |  |  | 0.505 |
| walker |  | 5412 | 9 | Code::CodeKey { rung: Body, file: screen.go, decl: 2, sub: 0, line: 20 } |  |  | 0.505 |
| walker |  | 5447 | 35 | Code::CodeKey { rung: Doc, file: screen.go, decl: 3, sub: 0, line: 26 } |  |  | 0.505 |
| walker |  | 5498 | 51 | Code::CodeKey { rung: Names, file: paste.go, decl: 0, sub: 0, line: 0 } |  |  | 0.508 |
| walker |  | 5510 | 12 | Code::CodeKey { rung: Decl, file: paste.go, decl: 1, sub: 0, line: 5 } |  |  | 0.508 |
| walker |  | 5518 | 8 | Code::CodeKey { rung: Body, file: paste.go, decl: 2, sub: 0, line: 10 } |  |  | 0.508 |
| walker |  | 5530 | 12 | Code::CodeKey { rung: Doc, file: paste.go, decl: 2, sub: 0, line: 10 } |  |  | 0.508 |
| walker |  | 5559 | 29 | Code::CodeKey { rung: Doc, file: paste.go, decl: 4, sub: 0, line: 20 } |  |  | 0.508 |
| walker |  | 5590 | 31 | Code::CodeKey { rung: Doc, file: paste.go, decl: 1, sub: 0, line: 5 } |  |  | 0.508 |
| ns | 5614 |  | 321 | `key.go` key-code catalog: group headings plus the C0/G0 names | 3.8 |  | 0.488 |
| walker |  | 5621 | 31 | Code::CodeKey { rung: Doc, file: paste.go, decl: 3, sub: 0, line: 16 } |  |  | 0.488 |
| walker |  | 5647 | 26 | Code::CodeKey { rung: Names, file: raw.go, decl: 0, sub: 0, line: 0 } |  |  | 0.488 |
| walker |  | 5660 | 13 | Code::CodeKey { rung: Decl, file: raw.go, decl: 1, sub: 0, line: 5 } |  |  | 0.489 |
| walker |  | 5686 | 26 | Code::CodeKey { rung: Body, file: raw.go, decl: 2, sub: 0, line: 33 } |  |  | 0.489 |
| walker |  | 5717 | 31 | Code::CodeKey { rung: Doc, file: raw.go, decl: 1, sub: 0, line: 5 } |  |  | 0.489 |
| ns | 5797 |  | 183 | Modifier keys: the complete `KeyMod` constant set | 3.9 |  | 0.480 |
| walker |  | 5963 | 246 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.481 |
| ns | 6023 |  | 226 | `Mouse` struct and the complete mouse-button constant set | 3.10 | 3.1 | 0.471 |
| walker |  | 6112 | 149 | Code::CodeKey { rung: Names, file: clipboard.go, decl: 0, sub: 0, line: 0 } |  |  | 0.476 |
| walker |  | 6133 | 21 | Code::CodeKey { rung: Decl, file: clipboard.go, decl: 1, sub: 0, line: 5 } |  |  | 0.477 |
| walker |  | 6141 | 8 | Code::CodeKey { rung: Body, file: clipboard.go, decl: 2, sub: 0, line: 15 } |  |  | 0.477 |
| walker |  | 6149 | 8 | Code::CodeKey { rung: Body, file: clipboard.go, decl: 3, sub: 0, line: 20 } |  |  | 0.477 |
| walker |  | 6158 | 9 | Code::CodeKey { rung: Body, file: clipboard.go, decl: 7, sub: 0, line: 42 } |  |  | 0.477 |
| walker |  | 6168 | 10 | Code::CodeKey { rung: Body, file: clipboard.go, decl: 11, sub: 0, line: 68 } |  |  | 0.477 |
| walker |  | 6183 | 15 | Code::CodeKey { rung: Doc, file: clipboard.go, decl: 3, sub: 0, line: 20 } |  |  | 0.477 |
| ns | 6200 |  | 177 | `MouseMode` enum: none / cell-motion / all-motion | 3.11 | 1.7 | 0.492 |
| walker |  | 6211 | 28 | Code::CodeKey { rung: Doc, file: clipboard.go, decl: 4, sub: 0, line: 26 } |  |  | 0.492 |
| walker |  | 6239 | 28 | Code::CodeKey { rung: Doc, file: clipboard.go, decl: 6, sub: 0, line: 38 } |  |  | 0.492 |
| walker |  | 6268 | 29 | Code::CodeKey { rung: Doc, file: clipboard.go, decl: 8, sub: 0, line: 48 } |  |  | 0.492 |
| walker |  | 6297 | 29 | Code::CodeKey { rung: Doc, file: clipboard.go, decl: 10, sub: 0, line: 62 } |  |  | 0.492 |
| walker |  | 6330 | 33 | Code::CodeKey { rung: Doc, file: clipboard.go, decl: 5, sub: 0, line: 30 } |  |  | 0.492 |
| walker |  | 6363 | 33 | Code::CodeKey { rung: Doc, file: clipboard.go, decl: 7, sub: 0, line: 42 } |  |  | 0.492 |
| ns | 6378 |  | 178 | Cursor value types: `Cursor`, `NewCursor`, `CursorShape` | 3.12 | 1.7 | 0.489 |
| walker |  | 6399 | 36 | Code::CodeKey { rung: Doc, file: clipboard.go, decl: 1, sub: 0, line: 5 } |  |  | 0.489 |
| walker |  | 6460 | 61 | Code::CodeKey { rung: Doc, file: clipboard.go, decl: 2, sub: 0, line: 15 } |  |  | 0.489 |
| walker |  | 6521 | 61 | Code::CodeKey { rung: Doc, file: clipboard.go, decl: 9, sub: 0, line: 54 } |  |  | 0.489 |
| ns | 6566 |  | 188 | Keyboard enhancements: request fields and response predicates | 3.13 | 3.2 | 0.485 |
| walker |  | 6582 | 61 | Code::CodeKey { rung: Doc, file: clipboard.go, decl: 11, sub: 0, line: 68 } |  |  | 0.485 |
| walker |  | 6637 | 55 | Code::CodeKey { rung: Names, file: termcap.go, decl: 0, sub: 0, line: 0 } |  |  | 0.488 |
| walker |  | 6649 | 12 | Code::CodeKey { rung: Decl, file: termcap.go, decl: 3, sub: 0, line: 41 } |  |  | 0.489 |
| walker |  | 6657 | 8 | Code::CodeKey { rung: Body, file: termcap.go, decl: 4, sub: 0, line: 46 } |  |  | 0.489 |
| walker |  | 6669 | 12 | Code::CodeKey { rung: Doc, file: termcap.go, decl: 4, sub: 0, line: 46 } |  |  | 0.489 |
| walker |  | 6695 | 26 | Code::CodeKey { rung: Body, file: termcap.go, decl: 2, sub: 0, line: 30 } |  |  | 0.489 |
| walker |  | 6727 | 32 | Code::CodeKey { rung: Doc, file: termcap.go, decl: 1, sub: 0, line: 5 } |  |  | 0.489 |
| ns | 6759 |  | 193 | `Batch` vs `Sequence` semantics and where they execute | 3.14 | 3.3 | 0.484 |
| walker |  | 6782 | 55 | Code::CodeKey { rung: Names, file: xterm.go, decl: 0, sub: 0, line: 0 } |  |  | 0.487 |
| walker |  | 6794 | 12 | Code::CodeKey { rung: Decl, file: xterm.go, decl: 1, sub: 0, line: 4 } |  |  | 0.488 |
| walker |  | 6802 | 8 | Code::CodeKey { rung: Body, file: xterm.go, decl: 2, sub: 0, line: 9 } |  |  | 0.488 |
| walker |  | 6810 | 8 | Code::CodeKey { rung: Body, file: xterm.go, decl: 4, sub: 0, line: 20 } |  |  | 0.488 |
| walker |  | 6822 | 12 | Code::CodeKey { rung: Doc, file: xterm.go, decl: 2, sub: 0, line: 9 } |  |  | 0.488 |
| walker |  | 6839 | 17 | Code::CodeKey { rung: Doc, file: xterm.go, decl: 1, sub: 0, line: 4 } |  |  | 0.488 |
| walker |  | 6868 | 29 | Code::CodeKey { rung: Doc, file: xterm.go, decl: 3, sub: 0, line: 15 } |  |  | 0.488 |
| walker |  | 6911 | 43 | Code::CodeKey { rung: Doc, file: xterm.go, decl: 4, sub: 0, line: 20 } |  |  | 0.488 |
| walker |  | 6925 | 14 | Code::CodeKey { rung: Names, file: profile.go, decl: 0, sub: 0, line: 0 } |  |  | 0.490 |
| walker |  | 6936 | 11 | Code::CodeKey { rung: Decl, file: profile.go, decl: 1, sub: 0, line: 13 } |  |  | 0.492 |
| ns | 6959 |  | 200 | `Exec` / `ExecProcess` and the `ExecCommand` interface | 3.15 | 3.3 | 0.485 |
| walker |  | 6981 | 45 | Code::CodeKey { rung: Names, file: tty_unix.go, decl: 0, sub: 0, line: 0 } |  |  | 0.485 |
| walker |  | 6995 | 14 | Code::CodeKey { rung: Doc, file: tty_unix.go, decl: 3, sub: 0, line: 40 } |  |  | 0.485 |
| walker |  | 7066 | 71 | Code::CodeKey { rung: Doc, file: screen.go, decl: 1, sub: 0, line: 9 } |  |  | 0.485 |
| ns | 7111 |  | 152 | Logging to a file: `LogToFile` / `LogToFileWith` | 3.16 |  | 0.481 |
| walker |  | 7296 | 230 | Code::CodeKey { rung: Names, file: color.go, decl: 0, sub: 0, line: 0 } |  |  | 0.497 |
| walker |  | 7305 | 9 | Code::CodeKey { rung: Body, file: color.go, decl: 2, sub: 0, line: 13 } |  |  | 0.497 |
| walker |  | 7314 | 9 | Code::CodeKey { rung: Body, file: color.go, decl: 4, sub: 0, line: 21 } |  |  | 0.497 |
| walker |  | 7323 | 9 | Code::CodeKey { rung: Body, file: color.go, decl: 6, sub: 0, line: 29 } |  |  | 0.497 |
| ns | 7328 |  | 217 | The `renderer` interface: complete method set, and its two implementations | 4.1 |  | 0.488 |
| walker |  | 7335 | 12 | Code::CodeKey { rung: Doc, file: color.go, decl: 8, sub: 0, line: 39 } |  |  | 0.488 |
| walker |  | 7347 | 12 | Code::CodeKey { rung: Doc, file: color.go, decl: 11, sub: 0, line: 70 } |  |  | 0.488 |
| walker |  | 7359 | 12 | Code::CodeKey { rung: Doc, file: color.go, decl: 14, sub: 0, line: 84 } |  |  | 0.488 |
| walker |  | 7373 | 14 | Code::CodeKey { rung: Doc, file: color.go, decl: 9, sub: 0, line: 44 } |  |  | 0.488 |
| walker |  | 7387 | 14 | Code::CodeKey { rung: Doc, file: color.go, decl: 12, sub: 0, line: 75 } |  |  | 0.488 |
| walker |  | 7401 | 14 | Code::CodeKey { rung: Doc, file: color.go, decl: 15, sub: 0, line: 89 } |  |  | 0.488 |
| walker |  | 7417 | 16 | Code::CodeKey { rung: Doc, file: color.go, decl: 2, sub: 0, line: 13 } |  |  | 0.488 |
| walker |  | 7433 | 16 | Code::CodeKey { rung: Doc, file: color.go, decl: 4, sub: 0, line: 21 } |  |  | 0.488 |
| walker |  | 7449 | 16 | Code::CodeKey { rung: Doc, file: color.go, decl: 6, sub: 0, line: 29 } |  |  | 0.488 |
| walker |  | 7467 | 18 | Code::CodeKey { rung: Doc, file: color.go, decl: 1, sub: 0, line: 10 } |  |  | 0.488 |
| walker |  | 7485 | 18 | Code::CodeKey { rung: Doc, file: color.go, decl: 3, sub: 0, line: 18 } |  |  | 0.488 |
| walker |  | 7503 | 18 | Code::CodeKey { rung: Doc, file: color.go, decl: 5, sub: 0, line: 26 } |  |  | 0.488 |
| walker |  | 7537 | 34 | Code::CodeKey { rung: Doc, file: color.go, decl: 13, sub: 0, line: 81 } |  |  | 0.488 |
| walker |  | 7585 | 48 | Code::CodeKey { rung: Doc, file: color.go, decl: 7, sub: 0, line: 36 } |  |  | 0.488 |
| walker |  | 7631 | 46 | Code::CodeKey { rung: Names, file: tty_windows.go, decl: 0, sub: 0, line: 0 } |  |  | 0.488 |
| walker |  | 7673 | 42 | Markdown::Section { file: README.md, section_index: 16, keeps_default_concavity: false } |  |  | 0.488 |
| ns | 7681 |  | 353 | `cursedRenderer`: every function in the 860-line renderer | 4.2 | 4.1 | 0.478 |
| walker |  | 7879 | 206 | Code::CodeKey { rung: Names, file: options.go, decl: 0, sub: 0, line: 0 } |  |  | 0.501 |
| walker |  | 7903 | 24 | Code::CodeKey { rung: Doc, file: options.go, decl: 8, sub: 0, line: 84 } |  |  | 0.501 |
| walker |  | 7930 | 27 | Code::CodeKey { rung: Body, file: options.go, decl: 3, sub: 0, line: 30 } |  |  | 0.501 |
| walker |  | 7957 | 27 | Code::CodeKey { rung: Body, file: options.go, decl: 5, sub: 0, line: 58 } |  |  | 0.501 |
| walker |  | 7984 | 27 | Code::CodeKey { rung: Body, file: options.go, decl: 10, sub: 0, line: 133 } |  |  | 0.501 |
| walker |  | 8018 | 34 | Code::CodeKey { rung: Doc, file: options.go, decl: 3, sub: 0, line: 30 } |  |  | 0.501 |
| ns | 8039 |  | 358 | `View` fields → ANSI sequences, in `cursedRenderer.start` | 4.3 | 4.2 | 0.490 |
| walker |  | 8055 | 37 | Code::CodeKey { rung: Doc, file: options.go, decl: 6, sub: 0, line: 66 } |  |  | 0.490 |
| walker |  | 8112 | 57 | Code::CodeKey { rung: Doc, file: options.go, decl: 2, sub: 0, line: 22 } |  |  | 0.490 |
| walker |  | 8173 | 61 | Code::CodeKey { rung: Doc, file: options.go, decl: 4, sub: 0, line: 40 } |  |  | 0.490 |
| walker |  | 8234 | 61 | Code::CodeKey { rung: Doc, file: options.go, decl: 11, sub: 0, line: 142 } |  |  | 0.490 |
| walker |  | 8302 | 68 | Code::CodeKey { rung: Doc, file: options.go, decl: 7, sub: 0, line: 76 } |  |  | 0.490 |
| walker |  | 8372 | 70 | Code::CodeKey { rung: Doc, file: options.go, decl: 13, sub: 0, line: 163 } |  |  | 0.490 |
| ns | 8420 |  | 381 | Platform matrix: build tags and per-OS terminal functions | 4.4 |  | 0.483 |
| walker |  | 8445 | 73 | Code::CodeKey { rung: Doc, file: options.go, decl: 1, sub: 0, line: 17 } |  |  | 0.483 |
| walker |  | 8590 | 145 | Code::CodeKey { rung: Names, file: commands.go, decl: 0, sub: 0, line: 0 } |  |  | 0.492 |
| walker |  | 8599 | 9 | Code::CodeKey { rung: Body, file: commands.go, decl: 9, sub: 0, line: 173 } |  |  | 0.492 |
| walker |  | 8614 | 15 | Code::CodeKey { rung: Body, file: commands.go, decl: 1, sub: 0, line: 15 } |  |  | 0.492 |
| ns | 8618 |  | 198 | `tty.go`: terminal acquisition, input loop, resize detection | 4.5 |  | 0.488 |
| walker |  | 8629 | 15 | Code::CodeKey { rung: Body, file: commands.go, decl: 3, sub: 0, line: 25 } |  |  | 0.488 |
| walker |  | 8647 | 18 | Code::CodeKey { rung: Doc, file: commands.go, decl: 4, sub: 0, line: 30 } |  |  | 0.489 |
| walker |  | 8679 | 32 | Code::CodeKey { rung: Doc, file: commands.go, decl: 3, sub: 0, line: 25 } |  |  | 0.491 |
| walker |  | 8717 | 38 | Code::CodeKey { rung: Doc, file: commands.go, decl: 2, sub: 0, line: 21 } |  |  | 0.491 |
| walker |  | 8814 | 97 | Code::CodeKey { rung: Names, file: cursor.go, decl: 0, sub: 0, line: 0 } |  |  | 0.504 |
| walker |  | 8825 | 11 | Code::CodeKey { rung: Decl, file: cursor.go, decl: 4, sub: 0, line: 15 } |  |  | 0.507 |
| walker |  | 8839 | 14 | Code::CodeKey { rung: Decl, file: cursor.go, decl: 2, sub: 0, line: 7 } |  |  | 0.507 |
| walker |  | 8845 | 6 | Code::CodeKey { rung: Doc, file: cursor.go, decl: 4, sub: 0, line: 15 } |  |  | 0.509 |
| walker |  | 8855 | 10 | Code::CodeKey { rung: Body, file: cursor.go, decl: 6, sub: 0, line: 26 } |  |  | 0.509 |
| walker |  | 8866 | 11 | Code::CodeKey { rung: Doc, file: cursor.go, decl: 3, sub: 0, line: 12 } |  |  | 0.513 |
| walker |  | 8879 | 13 | Code::CodeKey { rung: Doc, file: cursor.go, decl: 1, sub: 0, line: 4 } |  |  | 0.513 |
| ns | 8882 |  | 264 | `examples/` and `tutorials/` directory listings | 5.1 |  | 0.555 |
| walker |  | 8895 | 16 | Code::CodeKey { rung: Doc, file: cursor.go, decl: 2, sub: 0, line: 7 } |  |  | 0.555 |
| walker |  | 8911 | 16 | Code::CodeKey { rung: Doc, file: cursor.go, decl: 5, sub: 0, line: 22 } |  |  | 0.555 |
| walker |  | 8946 | 35 | Code::CodeKey { rung: Doc, file: cursor.go, decl: 6, sub: 0, line: 26 } |  |  | 0.555 |
| ns | 9062 |  | 180 | `UPGRADE_GUIDE_V2.md`: complete section map | 5.2 |  | 0.548 |
| walker |  | 9127 | 181 | Code::CodeKey { rung: Doc, file: tea.go, decl: 39, sub: 0, line: 972 } |  |  | 0.548 |
| walker |  | 9226 | 99 | Code::CodeKey { rung: Names, file: renderer.go, decl: 0, sub: 0, line: 0 } |  |  | 0.552 |
| walker |  | 9239 | 13 | Code::CodeKey { rung: Decl, file: renderer.go, decl: 3, sub: 0, line: 59 } |  |  | 0.552 |
| walker |  | 9278 | 39 | Code::CodeKey { rung: Decl, file: renderer.go, decl: 1, sub: 0, line: 10 } |  |  | 0.554 |
| walker |  | 9305 | 27 | Code::CodeKey { rung: Doc, file: renderer.go, decl: 6, sub: 0, line: 96 } |  |  | 0.554 |
| ns | 9347 |  | 285 | The v1→v2 migration checklist | 5.3 | 5.2 | 0.550 |
| walker |  | 9351 | 46 | Code::CodeKey { rung: Body, file: renderer.go, decl: 4, sub: 0, line: 70 } |  |  | 0.550 |
| ns | 9516 |  | 169 | README section map | 5.4 |  | 0.558 |
| walker |  | 9735 | 384 | Code::CodeKey { rung: Decl, file: renderer.go, decl: 2, sub: 0, line: 18 } |  |  | 0.566 |
| walker |  | 9748 | 13 | Code::CodeKey { rung: Doc, file: renderer.go, decl: 2, sub: 0, line: 18 } |  |  | 0.569 |
| walker |  | 9825 | 77 | Code::CodeKey { rung: Body, file: tty_unix.go, decl: 3, sub: 0, line: 40 } |  |  | 0.569 |
| ns | 9876 |  | 360 | Test suite: every test function, and the golden-file fixtures | 5.5 |  | 0.558 |
| walker |  | 9905 | 80 | Code::CodeKey { rung: Doc, file: commands.go, decl: 5, sub: 0, line: 36 } |  |  | 0.558 |
| ns | 9917 |  | 41 | CI workflow inventory and golden-test fixture directories | 5.6 |  | 0.562 |
