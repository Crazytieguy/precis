Score(3000)=0.542 I=0.766 C=0.383 ns_rows≤3K=18/50 grid(1000/1442/2080/3000/4327/6240/9000)=0.591/0.608/0.582/0.542/0.507/0.452/0.561

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
| walker |  | 1055 | 248 | Code::CodeKey { rung: Names, file: tea.go, decl: 0, sub: 0, line: 0 } |  |  | 0.591 |
| walker |  | 1066 | 11 | Code::CodeKey { rung: Decl, file: tea.go, decl: 13, sub: 0, line: 312 } |  |  | 0.591 |
| walker |  | 1073 | 7 | Code::CodeKey { rung: Doc, file: tea.go, decl: 13, sub: 0, line: 312 } |  |  | 0.591 |
| ns | 1132 |  | 157 | `Cmd` semantics and the `Quit` command | 1.9 | 1.3 | 0.554 |
| walker |  | 1235 | 162 | Code::CodeKey { rung: Decl, file: tea.go, decl: 5, sub: 0, line: 53 } |  |  | 0.633 |
| walker |  | 1244 | 9 | Code::CodeKey { rung: Body, file: tea.go, decl: 9, sub: 0, line: 279 } |  |  | 0.635 |
| ns | 1338 |  | 206 | Direct dependency set from `go.mod` | 1.10 |  | 0.607 |
| walker |  | 1498 | 254 | Code::CodeKey { rung: Decl, file: tea.go, decl: 11, sub: 0, line: 286 } |  |  | 0.609 |
| walker |  | 1512 | 14 | Code::CodeKey { rung: Doc, file: tea.go, decl: 12, sub: 0, line: 309 } |  |  | 0.609 |
| ns | 1522 |  | 184 | README lede: positioning and shipped feature set | 1.11 |  | 0.591 |
| walker |  | 1527 | 15 | Code::CodeKey { rung: Doc, file: tea.go, decl: 10, sub: 0, line: 284 } |  |  | 0.591 |
| walker |  | 1543 | 16 | Code::CodeKey { rung: Doc, file: tea.go, decl: 5, sub: 0, line: 53 } |  |  | 0.638 |
| ns | 1665 |  | 143 | Sentinel errors returned by `Program.Run` | 2.1 |  | 0.621 |
| ns | 1810 |  | 145 | Every exported `*Program` method (complete roster) | 2.2 | 1.4 | 0.600 |
| walker |  | 1904 | 361 | Code::CodeKey { rung: Decl, file: tea.go, decl: 8, sub: 0, line: 241 } |  |  | 0.601 |
| walker |  | 1923 | 19 | Code::CodeKey { rung: Doc, file: tea.go, decl: 2, sub: 0, line: 42 } |  |  | 0.605 |
| ns | 2001 |  | 191 | Every `ProgramOption` constructor (complete roster) | 2.3 |  | 0.581 |
| ns | 2130 |  | 129 | Build/test/lint entry points (`Taskfile.yaml`) | 2.4 |  | 0.604 |
| walker |  | 2165 | 242 | Code::CodeKey { rung: Names, file: tea.go, decl: 0, sub: 1, line: 0 } |  |  | 0.609 |
| walker |  | 2192 | 27 | Code::CodeKey { rung: Decl, file: tea.go, decl: 20, sub: 0, line: 395 } |  |  | 0.609 |
| walker |  | 2307 | 115 | Code::CodeKey { rung: Decl, file: tea.go, decl: 15, sub: 0, line: 336 } |  |  | 0.609 |
| ns | 2436 |  | 306 | Event-loop dispatch table: every internally-handled message type | 2.5 |  | 0.563 |
| walker |  | 2455 | 148 | Code::CodeKey { rung: Decl, file: tea.go, decl: 17, sub: 0, line: 357 } |  |  | 0.564 |
| walker |  | 2468 | 13 | Code::CodeKey { rung: Doc, file: tea.go, decl: 22, sub: 0, line: 409 } |  |  | 0.564 |
| walker |  | 2481 | 13 | Code::CodeKey { rung: Doc, file: tea.go, decl: 31, sub: 0, line: 595 } |  |  | 0.568 |
| walker |  | 2495 | 14 | Code::CodeKey { rung: Doc, file: tea.go, decl: 17, sub: 0, line: 357 } |  |  | 0.568 |
| ns | 2682 |  | 246 | Event-loop skeleton: translate → filter → dispatch → update → render | 2.6 | 2.5 | 0.536 |
| walker |  | 2684 | 189 | Code::CodeKey { rung: Names, file: tea.go, decl: 0, sub: 2, line: 0 } |  |  | 0.545 |
| walker |  | 2696 | 12 | Code::CodeKey { rung: Doc, file: tea.go, decl: 33, sub: 0, line: 685 } |  |  | 0.545 |
| walker |  | 2710 | 14 | Code::CodeKey { rung: Doc, file: tea.go, decl: 36, sub: 0, line: 886 } |  |  | 0.545 |
| walker |  | 2727 | 17 | Code::CodeKey { rung: Doc, file: tea.go, decl: 14, sub: 0, line: 321 } |  |  | 0.545 |
| walker |  | 2744 | 17 | Code::CodeKey { rung: Doc, file: tea.go, decl: 26, sub: 0, line: 564 } |  |  | 0.546 |
| walker |  | 2763 | 19 | Code::CodeKey { rung: Doc, file: tea.go, decl: 24, sub: 0, line: 555 } |  |  | 0.548 |
| ns | 2901 |  | 219 | Program control messages: suspend, resume, interrupt | 2.7 | 1.9 | 0.526 |
| walker |  | 3062 | 299 | Code::CodeKey { rung: Names, file: tea.go, decl: 0, sub: 3, line: 0 } |  |  | 0.553 |
| walker |  | 3074 | 12 | Code::CodeKey { rung: Doc, file: tea.go, decl: 55, sub: 0, line: 1393 } |  |  | 0.553 |
| walker |  | 3090 | 16 | Code::CodeKey { rung: Doc, file: tea.go, decl: 45, sub: 0, line: 1214 } |  |  | 0.553 |
| ns | 3100 |  | 199 | `Run()`: how input and the TTY are acquired | 2.8 |  | 0.535 |
| walker |  | 3107 | 17 | Code::CodeKey { rung: Doc, file: tea.go, decl: 46, sub: 0, line: 1221 } |  |  | 0.535 |
| walker |  | 3125 | 18 | Code::CodeKey { rung: Doc, file: tea.go, decl: 44, sub: 0, line: 1209 } |  |  | 0.535 |
| walker |  | 3148 | 23 | Code::CodeKey { rung: Doc, file: tea.go, decl: 30, sub: 0, line: 590 } |  |  | 0.539 |
| walker |  | 3172 | 24 | Code::CodeKey { rung: Doc, file: tea.go, decl: 18, sub: 0, line: 374 } |  |  | 0.539 |
| walker |  | 3197 | 25 | Code::CodeKey { rung: Doc, file: tea.go, decl: 1, sub: 0, line: 39 } |  |  | 0.543 |
| walker |  | 3215 | 18 | Fs::DirListing { dir: .github/ISSUE_TEMPLATE } |  |  | 0.543 |
| walker |  | 3243 | 28 | Code::CodeKey { rung: Doc, file: tea.go, decl: 28, sub: 0, line: 578 } |  |  | 0.550 |
| walker |  | 3272 | 29 | Code::CodeKey { rung: Doc, file: tea.go, decl: 21, sub: 0, line: 402 } |  |  | 0.550 |
| ns | 3288 |  | 188 | `Run()`: renderer choice, colour profile, first three messages | 2.9 | 2.8 | 0.534 |
| walker |  | 3301 | 29 | Code::CodeKey { rung: Doc, file: tea.go, decl: 47, sub: 0, line: 1241 } |  |  | 0.534 |
| walker |  | 3331 | 30 | Code::CodeKey { rung: Doc, file: tea.go, decl: 34, sub: 0, line: 700 } |  |  | 0.534 |
| walker |  | 3363 | 32 | Code::CodeKey { rung: Doc, file: tea.go, decl: 25, sub: 0, line: 561 } |  |  | 0.538 |
| walker |  | 3385 | 22 | Code::CodeKey { rung: Names, file: focus.go, decl: 0, sub: 0, line: 0 } |  |  | 0.539 |
| walker |  | 3419 | 34 | Code::CodeKey { rung: Doc, file: tea.go, decl: 4, sub: 0, line: 50 } |  |  | 0.550 |
| walker |  | 3455 | 36 | Code::CodeKey { rung: Doc, file: tea.go, decl: 3, sub: 0, line: 46 } |  |  | 0.559 |
| ns | 3474 |  | 186 | `Run()`: model init, event loop, graceful shutdown | 2.10 | 2.9 | 0.540 |
| walker |  | 3552 | 97 | Code::CodeKey { rung: Names, file: cursor.go, decl: 0, sub: 0, line: 0 } |  |  | 0.541 |
| walker |  | 3563 | 11 | Code::CodeKey { rung: Decl, file: cursor.go, decl: 4, sub: 0, line: 15 } |  |  | 0.541 |
| walker |  | 3577 | 14 | Code::CodeKey { rung: Decl, file: cursor.go, decl: 2, sub: 0, line: 7 } |  |  | 0.541 |
| walker |  | 3583 | 6 | Code::CodeKey { rung: Doc, file: cursor.go, decl: 4, sub: 0, line: 15 } |  |  | 0.541 |
| walker |  | 3593 | 10 | Code::CodeKey { rung: Body, file: cursor.go, decl: 6, sub: 0, line: 26 } |  |  | 0.541 |
| walker |  | 3604 | 11 | Code::CodeKey { rung: Doc, file: cursor.go, decl: 3, sub: 0, line: 12 } |  |  | 0.542 |
| walker |  | 3617 | 13 | Code::CodeKey { rung: Doc, file: cursor.go, decl: 1, sub: 0, line: 4 } |  |  | 0.542 |
| walker |  | 3654 | 37 | Code::CodeKey { rung: Doc, file: tea.go, decl: 35, sub: 0, line: 743 } |  |  | 0.542 |
| walker |  | 3691 | 37 | Code::CodeKey { rung: Doc, file: tea.go, decl: 48, sub: 0, line: 1269 } |  |  | 0.542 |
| ns | 3693 |  | 219 | Debug hooks: `TEA_TRACE`, `TEA_DEBUG`, panic recovery | 2.11 |  | 0.532 |
| walker |  | 3741 | 50 | Code::CodeKey { rung: Names, file: screen.go, decl: 0, sub: 0, line: 0 } |  |  | 0.532 |
| walker |  | 3763 | 22 | Code::CodeKey { rung: Decl, file: screen.go, decl: 1, sub: 0, line: 9 } |  |  | 0.532 |
| walker |  | 3813 | 50 | Code::CodeKey { rung: Decl, file: screen.go, decl: 4, sub: 0, line: 62 } |  |  | 0.532 |
| walker |  | 3822 | 9 | Code::CodeKey { rung: Body, file: screen.go, decl: 2, sub: 0, line: 20 } |  |  | 0.532 |
| walker |  | 3838 | 16 | Code::CodeKey { rung: Doc, file: cursor.go, decl: 2, sub: 0, line: 7 } |  |  | 0.532 |
| walker |  | 3854 | 16 | Code::CodeKey { rung: Doc, file: cursor.go, decl: 5, sub: 0, line: 22 } |  |  | 0.532 |
| walker |  | 3892 | 38 | Code::CodeKey { rung: Doc, file: tea.go, decl: 50, sub: 0, line: 1319 } |  |  | 0.532 |
| ns | 3916 |  | 223 | Frame pacing: the render ticker and the FPS bounds | 2.12 |  | 0.514 |
| walker |  | 3943 | 51 | Code::CodeKey { rung: Names, file: paste.go, decl: 0, sub: 0, line: 0 } |  |  | 0.515 |
| walker |  | 3955 | 12 | Code::CodeKey { rung: Decl, file: paste.go, decl: 1, sub: 0, line: 5 } |  |  | 0.515 |
| walker |  | 3963 | 8 | Code::CodeKey { rung: Body, file: paste.go, decl: 2, sub: 0, line: 10 } |  |  | 0.515 |
| walker |  | 3975 | 12 | Code::CodeKey { rung: Doc, file: paste.go, decl: 2, sub: 0, line: 10 } |  |  | 0.515 |
| walker |  | 4001 | 26 | Code::CodeKey { rung: Names, file: raw.go, decl: 0, sub: 0, line: 0 } |  |  | 0.515 |
| walker |  | 4014 | 13 | Code::CodeKey { rung: Decl, file: raw.go, decl: 1, sub: 0, line: 5 } |  |  | 0.515 |
| ns | 4118 |  | 202 | Input event message types: keys, mouse, paste, focus (complete) | 3.1 |  | 0.502 |
| walker |  | 4264 | 250 | Fs::DirListing { dir: examples } |  |  | 0.507 |
| ns | 4380 |  | 262 | Terminal report message types: colour, clipboard, size, capability (complete) | 3.2 |  | 0.494 |
| walker |  | 4510 | 246 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.494 |
| ns | 4550 |  | 170 | Command constructors, part 1: lifecycle, batching, timing, output | 3.3 | 2.7 | 0.486 |
| walker |  | 4659 | 149 | Code::CodeKey { rung: Names, file: clipboard.go, decl: 0, sub: 0, line: 0 } |  |  | 0.488 |
| walker |  | 4680 | 21 | Code::CodeKey { rung: Decl, file: clipboard.go, decl: 1, sub: 0, line: 5 } |  |  | 0.491 |
| walker |  | 4688 | 8 | Code::CodeKey { rung: Body, file: clipboard.go, decl: 2, sub: 0, line: 15 } |  |  | 0.491 |
| walker |  | 4696 | 8 | Code::CodeKey { rung: Body, file: clipboard.go, decl: 3, sub: 0, line: 20 } |  |  | 0.489 |
| ns | 4696 |  | 146 | Command constructors, part 2: terminal queries and clipboard | 3.4 | 3.3 | 0.489 |
| walker |  | 4705 | 9 | Code::CodeKey { rung: Body, file: clipboard.go, decl: 7, sub: 0, line: 42 } |  |  | 0.489 |
| walker |  | 4715 | 10 | Code::CodeKey { rung: Body, file: clipboard.go, decl: 11, sub: 0, line: 68 } |  |  | 0.489 |
| walker |  | 4730 | 15 | Code::CodeKey { rung: Doc, file: clipboard.go, decl: 3, sub: 0, line: 20 } |  |  | 0.489 |
| walker |  | 4785 | 55 | Code::CodeKey { rung: Names, file: termcap.go, decl: 0, sub: 0, line: 0 } |  |  | 0.494 |
| walker |  | 4797 | 12 | Code::CodeKey { rung: Decl, file: termcap.go, decl: 3, sub: 0, line: 41 } |  |  | 0.495 |
| walker |  | 4805 | 8 | Code::CodeKey { rung: Body, file: termcap.go, decl: 4, sub: 0, line: 46 } |  |  | 0.495 |
| walker |  | 4817 | 12 | Code::CodeKey { rung: Doc, file: termcap.go, decl: 4, sub: 0, line: 46 } |  |  | 0.495 |
| walker |  | 4872 | 55 | Code::CodeKey { rung: Names, file: xterm.go, decl: 0, sub: 0, line: 0 } |  |  | 0.500 |
| walker |  | 4884 | 12 | Code::CodeKey { rung: Decl, file: xterm.go, decl: 1, sub: 0, line: 4 } |  |  | 0.502 |
| walker |  | 4892 | 8 | Code::CodeKey { rung: Body, file: xterm.go, decl: 2, sub: 0, line: 9 } |  |  | 0.502 |
| walker |  | 4900 | 8 | Code::CodeKey { rung: Body, file: xterm.go, decl: 4, sub: 0, line: 20 } |  |  | 0.502 |
| walker |  | 4912 | 12 | Code::CodeKey { rung: Doc, file: xterm.go, decl: 2, sub: 0, line: 9 } |  |  | 0.502 |
| walker |  | 4929 | 17 | Code::CodeKey { rung: Doc, file: xterm.go, decl: 1, sub: 0, line: 4 } |  |  | 0.502 |
| walker |  | 4970 | 41 | Code::CodeKey { rung: Doc, file: tea.go, decl: 20, sub: 0, line: 395 } |  |  | 0.502 |
| ns | 5037 |  | 341 | `input.go`: the ultraviolet-event → `tea.Msg` translation table | 3.5 |  | 0.482 |
| ns | 5113 |  | 76 | `Key` struct fields | 3.6 |  | 0.476 |
| ns | 5293 |  | 180 | Key message methods: `String`, `Keystroke`, `Key` | 3.7 | 3.6 | 0.470 |
| ns | 5614 |  | 321 | `key.go` key-code catalog: group headings plus the C0/G0 names | 3.8 |  | 0.451 |
| ns | 5797 |  | 183 | Modifier keys: the complete `KeyMod` constant set | 3.9 |  | 0.444 |
| ns | 6023 |  | 226 | `Mouse` struct and the complete mouse-button constant set | 3.10 | 3.1 | 0.434 |
| ns | 6200 |  | 177 | `MouseMode` enum: none / cell-motion / all-motion | 3.11 | 1.7 | 0.450 |
| ns | 6378 |  | 178 | Cursor value types: `Cursor`, `NewCursor`, `CursorShape` | 3.12 | 1.7 | 0.468 |
| ns | 6566 |  | 188 | Keyboard enhancements: request fields and response predicates | 3.13 | 3.2 | 0.464 |
| ns | 6759 |  | 193 | `Batch` vs `Sequence` semantics and where they execute | 3.14 | 3.3 | 0.459 |
| ns | 6959 |  | 200 | `Exec` / `ExecProcess` and the `ExecCommand` interface | 3.15 | 3.3 | 0.453 |
| walker |  | 7020 | 2050 | Code::CodeKey { rung: Names, file: key.go, decl: 0, sub: 0, line: 0 } |  |  | 0.465 |
| walker |  | 7063 | 43 | Code::CodeKey { rung: Decl, file: key.go, decl: 1, sub: 0, line: 9 } |  |  | 0.473 |
| walker |  | 7077 | 14 | Code::CodeKey { rung: Names, file: profile.go, decl: 0, sub: 0, line: 0 } |  |  | 0.474 |
| walker |  | 7088 | 11 | Code::CodeKey { rung: Decl, file: profile.go, decl: 1, sub: 0, line: 13 } |  |  | 0.476 |
| ns | 7111 |  | 152 | Logging to a file: `LogToFile` / `LogToFileWith` | 3.16 |  | 0.472 |
| walker |  | 7144 | 56 | Code::CodeKey { rung: Names, file: tty_windows.go, decl: 0, sub: 0, line: 0 } |  |  | 0.472 |
| walker |  | 7186 | 42 | Code::CodeKey { rung: Doc, file: tea.go, decl: 56, sub: 0, line: 1427 } |  |  | 0.472 |
| walker |  | 7285 | 99 | Code::CodeKey { rung: Names, file: renderer.go, decl: 0, sub: 0, line: 0 } |  |  | 0.475 |
| walker |  | 7298 | 13 | Code::CodeKey { rung: Decl, file: renderer.go, decl: 3, sub: 0, line: 59 } |  |  | 0.475 |
| ns | 7328 |  | 217 | The `renderer` interface: complete method set, and its two implementations | 4.1 |  | 0.466 |
| walker |  | 7337 | 39 | Code::CodeKey { rung: Decl, file: renderer.go, decl: 1, sub: 0, line: 10 } |  |  | 0.470 |
| ns | 7681 |  | 353 | `cursedRenderer`: every function in the 860-line renderer | 4.2 | 4.1 | 0.460 |
| walker |  | 7721 | 384 | Code::CodeKey { rung: Decl, file: renderer.go, decl: 2, sub: 0, line: 18 } |  |  | 0.472 |
| walker |  | 7734 | 13 | Code::CodeKey { rung: Doc, file: renderer.go, decl: 2, sub: 0, line: 18 } |  |  | 0.475 |
| ns | 8039 |  | 358 | `View` fields → ANSI sequences, in `cursedRenderer.start` | 4.3 | 4.2 | 0.465 |
| walker |  | 8136 | 402 | Code::CodeKey { rung: Names, file: mouse.go, decl: 0, sub: 0, line: 0 } |  |  | 0.487 |
| walker |  | 8147 | 11 | Code::CodeKey { rung: Decl, file: mouse.go, decl: 2, sub: 0, line: 29 } |  |  | 0.489 |
| walker |  | 8180 | 33 | Code::CodeKey { rung: Decl, file: mouse.go, decl: 4, sub: 0, line: 71 } |  |  | 0.495 |
| walker |  | 8220 | 40 | Code::CodeKey { rung: Decl, file: mouse.go, decl: 3, sub: 0, line: 46 } |  |  | 0.498 |
| walker |  | 8228 | 8 | Code::CodeKey { rung: Body, file: mouse.go, decl: 8, sub: 0, line: 93 } |  |  | 0.498 |
| walker |  | 8236 | 8 | Code::CodeKey { rung: Body, file: mouse.go, decl: 11, sub: 0, line: 108 } |  |  | 0.498 |
| walker |  | 8249 | 13 | Code::CodeKey { rung: Doc, file: mouse.go, decl: 5, sub: 0, line: 78 } |  |  | 0.498 |
| walker |  | 8263 | 14 | Code::CodeKey { rung: Doc, file: mouse.go, decl: 7, sub: 0, line: 86 } |  |  | 0.498 |
| walker |  | 8277 | 14 | Code::CodeKey { rung: Doc, file: mouse.go, decl: 10, sub: 0, line: 101 } |  |  | 0.498 |
| walker |  | 8291 | 14 | Code::CodeKey { rung: Doc, file: mouse.go, decl: 13, sub: 0, line: 116 } |  |  | 0.498 |
| walker |  | 8305 | 14 | Code::CodeKey { rung: Doc, file: mouse.go, decl: 15, sub: 0, line: 128 } |  |  | 0.498 |
| walker |  | 8319 | 14 | Code::CodeKey { rung: Doc, file: mouse.go, decl: 16, sub: 0, line: 131 } |  |  | 0.498 |
| walker |  | 8334 | 15 | Code::CodeKey { rung: Doc, file: mouse.go, decl: 6, sub: 0, line: 83 } |  |  | 0.498 |
| walker |  | 8349 | 15 | Code::CodeKey { rung: Doc, file: mouse.go, decl: 9, sub: 0, line: 98 } |  |  | 0.498 |
| walker |  | 8364 | 15 | Code::CodeKey { rung: Doc, file: mouse.go, decl: 12, sub: 0, line: 113 } |  |  | 0.498 |
| walker |  | 8382 | 18 | Code::CodeKey { rung: Doc, file: mouse.go, decl: 1, sub: 0, line: 10 } |  |  | 0.498 |
| ns | 8420 |  | 381 | Platform matrix: build tags and per-OS terminal functions | 4.4 |  | 0.489 |
| walker |  | 8565 | 183 | Code::CodeKey { rung: Names, file: key.go, decl: 0, sub: 1, line: 0 } |  |  | 0.502 |
| walker |  | 8604 | 39 | Code::CodeKey { rung: Decl, file: key.go, decl: 11, sub: 0, line: 259 } |  |  | 0.507 |
| walker |  | 8612 | 8 | Code::CodeKey { rung: Body, file: key.go, decl: 6, sub: 0, line: 219 } |  |  | 0.507 |
| ns | 8618 |  | 198 | `tty.go`: terminal acquisition, input loop, resize detection | 4.5 |  | 0.502 |
| walker |  | 8620 | 8 | Code::CodeKey { rung: Body, file: key.go, decl: 10, sub: 0, line: 253 } |  |  | 0.502 |
| walker |  | 8634 | 14 | Code::CodeKey { rung: Doc, file: key.go, decl: 3, sub: 0, line: 191 } |  |  | 0.502 |
| walker |  | 8648 | 14 | Code::CodeKey { rung: Doc, file: key.go, decl: 7, sub: 0, line: 224 } |  |  | 0.502 |
| walker |  | 8800 | 152 | Code::CodeKey { rung: Names, file: mod.go, decl: 0, sub: 0, line: 0 } |  |  | 0.512 |
| ns | 8882 |  | 264 | `examples/` and `tutorials/` directory listings | 5.1 |  | 0.549 |
| walker |  | 8886 | 86 | Code::CodeKey { rung: Decl, file: mod.go, decl: 2, sub: 0, line: 9 } |  |  | 0.550 |
| walker |  | 8892 | 6 | Code::CodeKey { rung: Doc, file: mod.go, decl: 2, sub: 0, line: 9 } |  |  | 0.552 |
| walker |  | 8903 | 11 | Code::CodeKey { rung: Doc, file: mod.go, decl: 1, sub: 0, line: 6 } |  |  | 0.554 |
| ns | 9062 |  | 180 | `UPGRADE_GUIDE_V2.md`: complete section map | 5.2 |  | 0.548 |
| walker |  | 9133 | 230 | Code::CodeKey { rung: Names, file: color.go, decl: 0, sub: 0, line: 0 } |  |  | 0.559 |
| walker |  | 9142 | 9 | Code::CodeKey { rung: Body, file: color.go, decl: 2, sub: 0, line: 13 } |  |  | 0.559 |
| walker |  | 9151 | 9 | Code::CodeKey { rung: Body, file: color.go, decl: 4, sub: 0, line: 21 } |  |  | 0.559 |
| walker |  | 9160 | 9 | Code::CodeKey { rung: Body, file: color.go, decl: 6, sub: 0, line: 29 } |  |  | 0.559 |
| walker |  | 9172 | 12 | Code::CodeKey { rung: Doc, file: color.go, decl: 8, sub: 0, line: 39 } |  |  | 0.559 |
| walker |  | 9184 | 12 | Code::CodeKey { rung: Doc, file: color.go, decl: 11, sub: 0, line: 70 } |  |  | 0.559 |
| walker |  | 9196 | 12 | Code::CodeKey { rung: Doc, file: color.go, decl: 14, sub: 0, line: 84 } |  |  | 0.559 |
| walker |  | 9210 | 14 | Code::CodeKey { rung: Doc, file: color.go, decl: 9, sub: 0, line: 44 } |  |  | 0.559 |
| walker |  | 9224 | 14 | Code::CodeKey { rung: Doc, file: color.go, decl: 12, sub: 0, line: 75 } |  |  | 0.559 |
| walker |  | 9238 | 14 | Code::CodeKey { rung: Doc, file: color.go, decl: 15, sub: 0, line: 89 } |  |  | 0.559 |
| walker |  | 9254 | 16 | Code::CodeKey { rung: Doc, file: color.go, decl: 2, sub: 0, line: 13 } |  |  | 0.559 |
| walker |  | 9270 | 16 | Code::CodeKey { rung: Doc, file: color.go, decl: 4, sub: 0, line: 21 } |  |  | 0.559 |
| walker |  | 9286 | 16 | Code::CodeKey { rung: Doc, file: color.go, decl: 6, sub: 0, line: 29 } |  |  | 0.559 |
| walker |  | 9304 | 18 | Code::CodeKey { rung: Doc, file: color.go, decl: 1, sub: 0, line: 10 } |  |  | 0.559 |
| walker |  | 9322 | 18 | Code::CodeKey { rung: Doc, file: color.go, decl: 3, sub: 0, line: 18 } |  |  | 0.559 |
| walker |  | 9340 | 18 | Code::CodeKey { rung: Doc, file: color.go, decl: 5, sub: 0, line: 26 } |  |  | 0.559 |
| ns | 9347 |  | 285 | The v1→v2 migration checklist | 5.3 | 5.2 | 0.554 |
| walker |  | 9386 | 46 | Code::CodeKey { rung: Doc, file: tea.go, decl: 49, sub: 0, line: 1294 } |  |  | 0.554 |
| ns | 9516 |  | 169 | README section map | 5.4 |  | 0.561 |
| walker |  | 9592 | 206 | Code::CodeKey { rung: Names, file: options.go, decl: 0, sub: 0, line: 0 } |  |  | 0.580 |
| walker |  | 9624 | 32 | Code::CodeKey { rung: Names, file: signals_windows.go, decl: 0, sub: 0, line: 0 } |  |  | 0.580 |
| walker |  | 9631 | 7 | Code::CodeKey { rung: Body, file: signals_windows.go, decl: 1, sub: 0, line: 8 } |  |  | 0.580 |
| walker |  | 9776 | 145 | Code::CodeKey { rung: Names, file: commands.go, decl: 0, sub: 0, line: 0 } |  |  | 0.589 |
| walker |  | 9785 | 9 | Code::CodeKey { rung: Body, file: commands.go, decl: 9, sub: 0, line: 173 } |  |  | 0.589 |
| walker |  | 9800 | 15 | Code::CodeKey { rung: Body, file: commands.go, decl: 1, sub: 0, line: 15 } |  |  | 0.589 |
| walker |  | 9815 | 15 | Code::CodeKey { rung: Body, file: commands.go, decl: 3, sub: 0, line: 25 } |  |  | 0.589 |
| walker |  | 9833 | 18 | Code::CodeKey { rung: Doc, file: commands.go, decl: 4, sub: 0, line: 30 } |  |  | 0.589 |
| ns | 9876 |  | 360 | Test suite: every test function, and the golden-file fixtures | 5.5 |  | 0.578 |
| walker |  | 9882 | 49 | Code::CodeKey { rung: Doc, file: tea.go, decl: 16, sub: 0, line: 349 } |  |  | 0.578 |
| walker |  | 9916 | 34 | Code::CodeKey { rung: Names, file: termios_windows.go, decl: 0, sub: 0, line: 0 } |  |  | 0.579 |
| ns | 9917 |  | 41 | CI workflow inventory and golden-test fixture directories | 5.6 |  | 0.582 |
| walker |  | 9962 | 46 | Code::CodeKey { rung: Decl, file: key.go, decl: 2, sub: 0, line: 16 } |  |  | 0.586 |
| walker |  | 9969 | 7 | Code::CodeKey { rung: Doc, file: key.go, decl: 2, sub: 0, line: 16 } |  |  | 0.586 |
| walker |  | 9991 | 22 | Code::CodeKey { rung: Decl, file: key.go, decl: 2, sub: 2, line: 16 } |  |  | 0.589 |
