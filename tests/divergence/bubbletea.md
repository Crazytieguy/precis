Score(3000)=0.528 I=0.761 C=0.366 ns_rows≤3K=18/50 grid(1000/1442/2080/3000/4327/6240/9000)=0.591/0.607/0.581/0.528/0.461/0.439/0.503

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
| ns | 1338 |  | 206 | Direct dependency set from `go.mod` | 1.10 |  | 0.605 |
| walker |  | 1489 | 254 | Code::CodeKey { rung: Decl, file: tea.go, decl: 11, sub: 0, line: 286 } |  |  | 0.607 |
| walker |  | 1503 | 14 | Code::CodeKey { rung: Doc, file: tea.go, decl: 12, sub: 0, line: 309 } |  |  | 0.607 |
| walker |  | 1518 | 15 | Code::CodeKey { rung: Doc, file: tea.go, decl: 10, sub: 0, line: 284 } |  |  | 0.607 |
| ns | 1522 |  | 184 | README lede: positioning and shipped feature set | 1.11 |  | 0.590 |
| walker |  | 1534 | 16 | Code::CodeKey { rung: Doc, file: tea.go, decl: 5, sub: 0, line: 53 } |  |  | 0.636 |
| ns | 1665 |  | 143 | Sentinel errors returned by `Program.Run` | 2.1 |  | 0.619 |
| ns | 1810 |  | 145 | Every exported `*Program` method (complete roster) | 2.2 | 1.4 | 0.599 |
| walker |  | 1895 | 361 | Code::CodeKey { rung: Decl, file: tea.go, decl: 8, sub: 0, line: 241 } |  |  | 0.599 |
| walker |  | 1914 | 19 | Code::CodeKey { rung: Doc, file: tea.go, decl: 2, sub: 0, line: 42 } |  |  | 0.604 |
| ns | 2001 |  | 191 | Every `ProgramOption` constructor (complete roster) | 2.3 |  | 0.579 |
| ns | 2130 |  | 129 | Build/test/lint entry points (`Taskfile.yaml`) | 2.4 |  | 0.603 |
| walker |  | 2156 | 242 | Code::CodeKey { rung: Names, file: tea.go, decl: 0, sub: 1, line: 0 } |  |  | 0.608 |
| walker |  | 2183 | 27 | Code::CodeKey { rung: Decl, file: tea.go, decl: 20, sub: 0, line: 395 } |  |  | 0.608 |
| walker |  | 2298 | 115 | Code::CodeKey { rung: Decl, file: tea.go, decl: 15, sub: 0, line: 336 } |  |  | 0.608 |
| ns | 2436 |  | 306 | Event-loop dispatch table: every internally-handled message type | 2.5 |  | 0.562 |
| walker |  | 2446 | 148 | Code::CodeKey { rung: Decl, file: tea.go, decl: 17, sub: 0, line: 357 } |  |  | 0.562 |
| walker |  | 2459 | 13 | Code::CodeKey { rung: Doc, file: tea.go, decl: 22, sub: 0, line: 409 } |  |  | 0.562 |
| walker |  | 2472 | 13 | Code::CodeKey { rung: Doc, file: tea.go, decl: 31, sub: 0, line: 595 } |  |  | 0.567 |
| walker |  | 2486 | 14 | Code::CodeKey { rung: Doc, file: tea.go, decl: 17, sub: 0, line: 357 } |  |  | 0.567 |
| walker |  | 2675 | 189 | Code::CodeKey { rung: Names, file: tea.go, decl: 0, sub: 2, line: 0 } |  |  | 0.577 |
| ns | 2682 |  | 246 | Event-loop skeleton: translate → filter → dispatch → update → render | 2.6 | 2.5 | 0.544 |
| walker |  | 2687 | 12 | Code::CodeKey { rung: Doc, file: tea.go, decl: 33, sub: 0, line: 685 } |  |  | 0.544 |
| walker |  | 2701 | 14 | Code::CodeKey { rung: Doc, file: tea.go, decl: 36, sub: 0, line: 886 } |  |  | 0.544 |
| walker |  | 2718 | 17 | Code::CodeKey { rung: Doc, file: tea.go, decl: 14, sub: 0, line: 321 } |  |  | 0.544 |
| walker |  | 2735 | 17 | Code::CodeKey { rung: Doc, file: tea.go, decl: 26, sub: 0, line: 564 } |  |  | 0.544 |
| ns | 2901 |  | 219 | Program control messages: suspend, resume, interrupt | 2.7 | 1.9 | 0.523 |
| walker |  | 2930 | 195 | Code::CodeKey { rung: Decl, file: tea.go, decl: 23, sub: 0, line: 426 } |  |  | 0.523 |
| walker |  | 2942 | 12 | Code::CodeKey { rung: Doc, file: tea.go, decl: 23, sub: 0, line: 426 } |  |  | 0.523 |
| walker |  | 2959 | 17 | Code::CodeKey { rung: Doc, file: tea.go, decl: 24, sub: 0, line: 555 } |  |  | 0.525 |
| ns | 3100 |  | 199 | `Run()`: how input and the TTY are acquired | 2.8 |  | 0.507 |
| walker |  | 3258 | 299 | Code::CodeKey { rung: Names, file: tea.go, decl: 0, sub: 3, line: 0 } |  |  | 0.533 |
| walker |  | 3270 | 12 | Code::CodeKey { rung: Doc, file: tea.go, decl: 55, sub: 0, line: 1393 } |  |  | 0.533 |
| walker |  | 3286 | 16 | Code::CodeKey { rung: Doc, file: tea.go, decl: 45, sub: 0, line: 1214 } |  |  | 0.533 |
| ns | 3288 |  | 188 | `Run()`: renderer choice, colour profile, first three messages | 2.9 | 2.8 | 0.518 |
| walker |  | 3303 | 17 | Code::CodeKey { rung: Doc, file: tea.go, decl: 46, sub: 0, line: 1221 } |  |  | 0.518 |
| walker |  | 3321 | 18 | Code::CodeKey { rung: Doc, file: tea.go, decl: 44, sub: 0, line: 1209 } |  |  | 0.518 |
| ns | 3474 |  | 186 | `Run()`: model init, event loop, graceful shutdown | 2.10 | 2.9 | 0.501 |
| ns | 3693 |  | 219 | Debug hooks: `TEA_TRACE`, `TEA_DEBUG`, panic recovery | 2.11 |  | 0.491 |
| walker |  | 3713 | 392 | Code::CodeKey { rung: Decl, file: tea.go, decl: 23, sub: 1, line: 426 } |  |  | 0.491 |
| walker |  | 3901 | 188 | Code::CodeKey { rung: Decl, file: tea.go, decl: 23, sub: 2, line: 426 } |  |  | 0.491 |
| ns | 3916 |  | 223 | Frame pacing: the render ticker and the FPS bounds | 2.12 |  | 0.476 |
| walker |  | 4099 | 198 | Code::CodeKey { rung: Decl, file: tea.go, decl: 23, sub: 3, line: 426 } |  |  | 0.476 |
| ns | 4118 |  | 202 | Input event message types: keys, mouse, paste, focus (complete) | 3.1 |  | 0.461 |
| walker |  | 4327 | 228 | Code::CodeKey { rung: Decl, file: tea.go, decl: 23, sub: 4, line: 426 } |  |  | 0.461 |
| ns | 4380 |  | 262 | Terminal report message types: colour, clipboard, size, capability (complete) | 3.2 |  | 0.445 |
| ns | 4550 |  | 170 | Command constructors, part 1: lifecycle, batching, timing, output | 3.3 | 2.7 | 0.437 |
| walker |  | 4631 | 304 | Code::CodeKey { rung: Decl, file: tea.go, decl: 23, sub: 5, line: 426 } |  |  | 0.437 |
| walker |  | 4654 | 23 | Code::CodeKey { rung: Doc, file: tea.go, decl: 30, sub: 0, line: 590 } |  |  | 0.441 |
| walker |  | 4678 | 24 | Code::CodeKey { rung: Doc, file: tea.go, decl: 18, sub: 0, line: 374 } |  |  | 0.441 |
| ns | 4696 |  | 146 | Command constructors, part 2: terminal queries and clipboard | 3.4 | 3.3 | 0.434 |
| walker |  | 4703 | 25 | Code::CodeKey { rung: Doc, file: tea.go, decl: 1, sub: 0, line: 39 } |  |  | 0.437 |
| walker |  | 4721 | 18 | Fs::DirListing { dir: .github/ISSUE_TEMPLATE } |  |  | 0.437 |
| walker |  | 4749 | 28 | Code::CodeKey { rung: Doc, file: tea.go, decl: 28, sub: 0, line: 578 } |  |  | 0.442 |
| walker |  | 4778 | 29 | Code::CodeKey { rung: Doc, file: tea.go, decl: 21, sub: 0, line: 402 } |  |  | 0.442 |
| walker |  | 4807 | 29 | Code::CodeKey { rung: Doc, file: tea.go, decl: 47, sub: 0, line: 1241 } |  |  | 0.442 |
| walker |  | 4837 | 30 | Code::CodeKey { rung: Doc, file: tea.go, decl: 34, sub: 0, line: 700 } |  |  | 0.442 |
| walker |  | 4869 | 32 | Code::CodeKey { rung: Doc, file: tea.go, decl: 25, sub: 0, line: 561 } |  |  | 0.446 |
| walker |  | 4891 | 22 | Code::CodeKey { rung: Names, file: focus.go, decl: 0, sub: 0, line: 0 } |  |  | 0.447 |
| walker |  | 4925 | 34 | Code::CodeKey { rung: Doc, file: tea.go, decl: 4, sub: 0, line: 50 } |  |  | 0.456 |
| walker |  | 4961 | 36 | Code::CodeKey { rung: Doc, file: tea.go, decl: 3, sub: 0, line: 46 } |  |  | 0.464 |
| ns | 5037 |  | 341 | `input.go`: the ultraviolet-event → `tea.Msg` translation table | 3.5 |  | 0.445 |
| walker |  | 5058 | 97 | Code::CodeKey { rung: Names, file: cursor.go, decl: 0, sub: 0, line: 0 } |  |  | 0.446 |
| walker |  | 5069 | 11 | Code::CodeKey { rung: Decl, file: cursor.go, decl: 4, sub: 0, line: 15 } |  |  | 0.446 |
| walker |  | 5083 | 14 | Code::CodeKey { rung: Decl, file: cursor.go, decl: 2, sub: 0, line: 7 } |  |  | 0.446 |
| walker |  | 5089 | 6 | Code::CodeKey { rung: Doc, file: cursor.go, decl: 4, sub: 0, line: 15 } |  |  | 0.446 |
| walker |  | 5100 | 11 | Code::CodeKey { rung: Doc, file: cursor.go, decl: 3, sub: 0, line: 12 } |  |  | 0.447 |
| walker |  | 5113 | 13 | Code::CodeKey { rung: Doc, file: cursor.go, decl: 1, sub: 0, line: 4 } |  |  | 0.441 |
| ns | 5113 |  | 76 | `Key` struct fields | 3.6 |  | 0.441 |
| walker |  | 5123 | 10 | Code::CodeKey { rung: Body, file: cursor.go, decl: 6, sub: 0, line: 26 } |  |  | 0.441 |
| walker |  | 5160 | 37 | Code::CodeKey { rung: Doc, file: tea.go, decl: 35, sub: 0, line: 743 } |  |  | 0.441 |
| walker |  | 5197 | 37 | Code::CodeKey { rung: Doc, file: tea.go, decl: 48, sub: 0, line: 1269 } |  |  | 0.441 |
| walker |  | 5247 | 50 | Code::CodeKey { rung: Names, file: screen.go, decl: 0, sub: 0, line: 0 } |  |  | 0.443 |
| walker |  | 5269 | 22 | Code::CodeKey { rung: Decl, file: screen.go, decl: 1, sub: 0, line: 9 } |  |  | 0.444 |
| ns | 5293 |  | 180 | Key message methods: `String`, `Keystroke`, `Key` | 3.7 | 3.6 | 0.438 |
| walker |  | 5319 | 50 | Code::CodeKey { rung: Decl, file: screen.go, decl: 4, sub: 0, line: 62 } |  |  | 0.438 |
| walker |  | 5328 | 9 | Code::CodeKey { rung: Body, file: screen.go, decl: 2, sub: 0, line: 20 } |  |  | 0.438 |
| walker |  | 5344 | 16 | Code::CodeKey { rung: Doc, file: cursor.go, decl: 2, sub: 0, line: 7 } |  |  | 0.438 |
| walker |  | 5360 | 16 | Code::CodeKey { rung: Doc, file: cursor.go, decl: 5, sub: 0, line: 22 } |  |  | 0.438 |
| walker |  | 5398 | 38 | Code::CodeKey { rung: Doc, file: tea.go, decl: 50, sub: 0, line: 1319 } |  |  | 0.438 |
| walker |  | 5449 | 51 | Code::CodeKey { rung: Names, file: paste.go, decl: 0, sub: 0, line: 0 } |  |  | 0.441 |
| walker |  | 5461 | 12 | Code::CodeKey { rung: Decl, file: paste.go, decl: 1, sub: 0, line: 5 } |  |  | 0.441 |
| walker |  | 5469 | 8 | Code::CodeKey { rung: Body, file: paste.go, decl: 2, sub: 0, line: 10 } |  |  | 0.441 |
| walker |  | 5481 | 12 | Code::CodeKey { rung: Doc, file: paste.go, decl: 2, sub: 0, line: 10 } |  |  | 0.441 |
| walker |  | 5507 | 26 | Code::CodeKey { rung: Names, file: raw.go, decl: 0, sub: 0, line: 0 } |  |  | 0.442 |
| walker |  | 5520 | 13 | Code::CodeKey { rung: Decl, file: raw.go, decl: 1, sub: 0, line: 5 } |  |  | 0.443 |
| ns | 5614 |  | 321 | `key.go` key-code catalog: group headings plus the C0/G0 names | 3.8 |  | 0.425 |
| walker |  | 5770 | 250 | Fs::DirListing { dir: examples } |  |  | 0.429 |
| ns | 5797 |  | 183 | Modifier keys: the complete `KeyMod` constant set | 3.9 |  | 0.422 |
| walker |  | 6016 | 246 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.423 |
| ns | 6023 |  | 226 | `Mouse` struct and the complete mouse-button constant set | 3.10 | 3.1 | 0.414 |
| walker |  | 6165 | 149 | Code::CodeKey { rung: Names, file: clipboard.go, decl: 0, sub: 0, line: 0 } |  |  | 0.420 |
| walker |  | 6186 | 21 | Code::CodeKey { rung: Decl, file: clipboard.go, decl: 1, sub: 0, line: 5 } |  |  | 0.422 |
| walker |  | 6194 | 8 | Code::CodeKey { rung: Body, file: clipboard.go, decl: 2, sub: 0, line: 15 } |  |  | 0.422 |
| ns | 6200 |  | 177 | `MouseMode` enum: none / cell-motion / all-motion | 3.11 | 1.7 | 0.439 |
| walker |  | 6202 | 8 | Code::CodeKey { rung: Body, file: clipboard.go, decl: 3, sub: 0, line: 20 } |  |  | 0.439 |
| walker |  | 6211 | 9 | Code::CodeKey { rung: Body, file: clipboard.go, decl: 7, sub: 0, line: 42 } |  |  | 0.439 |
| walker |  | 6226 | 15 | Code::CodeKey { rung: Doc, file: clipboard.go, decl: 3, sub: 0, line: 20 } |  |  | 0.439 |
| walker |  | 6236 | 10 | Code::CodeKey { rung: Body, file: clipboard.go, decl: 11, sub: 0, line: 68 } |  |  | 0.439 |
| walker |  | 6291 | 55 | Code::CodeKey { rung: Names, file: termcap.go, decl: 0, sub: 0, line: 0 } |  |  | 0.443 |
| walker |  | 6303 | 12 | Code::CodeKey { rung: Decl, file: termcap.go, decl: 3, sub: 0, line: 41 } |  |  | 0.444 |
| walker |  | 6311 | 8 | Code::CodeKey { rung: Body, file: termcap.go, decl: 4, sub: 0, line: 46 } |  |  | 0.444 |
| walker |  | 6323 | 12 | Code::CodeKey { rung: Doc, file: termcap.go, decl: 4, sub: 0, line: 46 } |  |  | 0.444 |
| walker |  | 6378 | 55 | Code::CodeKey { rung: Names, file: xterm.go, decl: 0, sub: 0, line: 0 } |  |  | 0.466 |
| ns | 6378 |  | 178 | Cursor value types: `Cursor`, `NewCursor`, `CursorShape` | 3.12 | 1.7 | 0.466 |
| walker |  | 6390 | 12 | Code::CodeKey { rung: Decl, file: xterm.go, decl: 1, sub: 0, line: 4 } |  |  | 0.467 |
| walker |  | 6398 | 8 | Code::CodeKey { rung: Body, file: xterm.go, decl: 2, sub: 0, line: 9 } |  |  | 0.467 |
| walker |  | 6406 | 8 | Code::CodeKey { rung: Body, file: xterm.go, decl: 4, sub: 0, line: 20 } |  |  | 0.467 |
| walker |  | 6418 | 12 | Code::CodeKey { rung: Doc, file: xterm.go, decl: 2, sub: 0, line: 9 } |  |  | 0.467 |
| walker |  | 6435 | 17 | Code::CodeKey { rung: Doc, file: xterm.go, decl: 1, sub: 0, line: 4 } |  |  | 0.467 |
| walker |  | 6476 | 41 | Code::CodeKey { rung: Doc, file: tea.go, decl: 20, sub: 0, line: 395 } |  |  | 0.467 |
| ns | 6566 |  | 188 | Keyboard enhancements: request fields and response predicates | 3.13 | 3.2 | 0.463 |
| ns | 6759 |  | 193 | `Batch` vs `Sequence` semantics and where they execute | 3.14 | 3.3 | 0.459 |
| ns | 6959 |  | 200 | `Exec` / `ExecProcess` and the `ExecCommand` interface | 3.15 | 3.3 | 0.453 |
| ns | 7111 |  | 152 | Logging to a file: `LogToFile` / `LogToFileWith` | 3.16 |  | 0.449 |
| ns | 7328 |  | 217 | The `renderer` interface: complete method set, and its two implementations | 4.1 |  | 0.441 |
| ns | 7681 |  | 353 | `cursedRenderer`: every function in the 860-line renderer | 4.2 | 4.1 | 0.431 |
| ns | 8039 |  | 358 | `View` fields → ANSI sequences, in `cursedRenderer.start` | 4.3 | 4.2 | 0.422 |
| ns | 8420 |  | 381 | Platform matrix: build tags and per-OS terminal functions | 4.4 |  | 0.414 |
| walker |  | 8526 | 2050 | Code::CodeKey { rung: Names, file: key.go, decl: 0, sub: 0, line: 0 } |  |  | 0.425 |
| walker |  | 8574 | 48 | Code::CodeKey { rung: Decl, file: key.go, decl: 2, sub: 0, line: 16 } |  |  | 0.429 |
| walker |  | 8610 | 36 | Code::CodeKey { rung: Decl, file: key.go, decl: 2, sub: 2, line: 16 } |  |  | 0.433 |
| ns | 8618 |  | 198 | `tty.go`: terminal acquisition, input loop, resize detection | 4.5 |  | 0.429 |
| walker |  | 8627 | 17 | Code::CodeKey { rung: Decl, file: key.go, decl: 2, sub: 3, line: 16 } |  |  | 0.432 |
| walker |  | 8663 | 36 | Code::CodeKey { rung: Decl, file: key.go, decl: 2, sub: 8, line: 16 } |  |  | 0.436 |
| walker |  | 8703 | 40 | Code::CodeKey { rung: Decl, file: key.go, decl: 2, sub: 10, line: 16 } |  |  | 0.441 |
| walker |  | 8746 | 43 | Code::CodeKey { rung: Decl, file: key.go, decl: 1, sub: 0, line: 9 } |  |  | 0.452 |
| walker |  | 8753 | 7 | Code::CodeKey { rung: Doc, file: key.go, decl: 2, sub: 0, line: 16 } |  |  | 0.452 |
| walker |  | 8767 | 14 | Code::CodeKey { rung: Names, file: profile.go, decl: 0, sub: 0, line: 0 } |  |  | 0.454 |
| walker |  | 8778 | 11 | Code::CodeKey { rung: Decl, file: profile.go, decl: 1, sub: 0, line: 13 } |  |  | 0.455 |
| walker |  | 8834 | 56 | Code::CodeKey { rung: Names, file: tty_windows.go, decl: 0, sub: 0, line: 0 } |  |  | 0.456 |
| walker |  | 8876 | 42 | Code::CodeKey { rung: Doc, file: tea.go, decl: 56, sub: 0, line: 1427 } |  |  | 0.456 |
| ns | 8882 |  | 264 | `examples/` and `tutorials/` directory listings | 5.1 |  | 0.501 |
| walker |  | 8975 | 99 | Code::CodeKey { rung: Names, file: renderer.go, decl: 0, sub: 0, line: 0 } |  |  | 0.503 |
| walker |  | 8988 | 13 | Code::CodeKey { rung: Decl, file: renderer.go, decl: 3, sub: 0, line: 59 } |  |  | 0.503 |
| walker |  | 9027 | 39 | Code::CodeKey { rung: Decl, file: renderer.go, decl: 1, sub: 0, line: 10 } |  |  | 0.506 |
| ns | 9062 |  | 180 | `UPGRADE_GUIDE_V2.md`: complete section map | 5.2 |  | 0.500 |
| ns | 9347 |  | 285 | The v1→v2 migration checklist | 5.3 | 5.2 | 0.496 |
| walker |  | 9411 | 384 | Code::CodeKey { rung: Decl, file: renderer.go, decl: 2, sub: 0, line: 18 } |  |  | 0.505 |
| walker |  | 9424 | 13 | Code::CodeKey { rung: Doc, file: renderer.go, decl: 2, sub: 0, line: 18 } |  |  | 0.507 |
| ns | 9516 |  | 169 | README section map | 5.4 |  | 0.515 |
| walker |  | 9826 | 402 | Code::CodeKey { rung: Names, file: mouse.go, decl: 0, sub: 0, line: 0 } |  |  | 0.532 |
| walker |  | 9837 | 11 | Code::CodeKey { rung: Decl, file: mouse.go, decl: 2, sub: 0, line: 29 } |  |  | 0.534 |
| walker |  | 9870 | 33 | Code::CodeKey { rung: Decl, file: mouse.go, decl: 4, sub: 0, line: 71 } |  |  | 0.538 |
| ns | 9876 |  | 360 | Test suite: every test function, and the golden-file fixtures | 5.5 |  | 0.527 |
| walker |  | 9910 | 40 | Code::CodeKey { rung: Decl, file: mouse.go, decl: 3, sub: 0, line: 46 } |  |  | 0.530 |
| ns | 9917 |  | 41 | CI workflow inventory and golden-test fixture directories | 5.6 |  | 0.534 |
| walker |  | 9923 | 13 | Code::CodeKey { rung: Doc, file: mouse.go, decl: 5, sub: 0, line: 78 } |  |  | 0.534 |
| walker |  | 9937 | 14 | Code::CodeKey { rung: Doc, file: mouse.go, decl: 7, sub: 0, line: 86 } |  |  | 0.534 |
| walker |  | 9951 | 14 | Code::CodeKey { rung: Doc, file: mouse.go, decl: 10, sub: 0, line: 101 } |  |  | 0.534 |
| walker |  | 9965 | 14 | Code::CodeKey { rung: Doc, file: mouse.go, decl: 13, sub: 0, line: 116 } |  |  | 0.534 |
| walker |  | 9979 | 14 | Code::CodeKey { rung: Doc, file: mouse.go, decl: 15, sub: 0, line: 128 } |  |  | 0.534 |
| walker |  | 9993 | 14 | Code::CodeKey { rung: Doc, file: mouse.go, decl: 16, sub: 0, line: 131 } |  |  | 0.534 |
