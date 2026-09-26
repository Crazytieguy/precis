Score(3000)=0.528 I=0.761 C=0.366 ns_rows≤3K=18/50 grid(1000/1442/2080/3000/4327/6240/9000)=0.591/0.606/0.581/0.528/0.461/0.424/0.499

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
| walker |  | 4903 | 34 | Code::CodeKey { rung: Doc, file: tea.go, decl: 4, sub: 0, line: 50 } |  |  | 0.456 |
| walker |  | 4939 | 36 | Code::CodeKey { rung: Doc, file: tea.go, decl: 3, sub: 0, line: 46 } |  |  | 0.463 |
| walker |  | 4976 | 37 | Code::CodeKey { rung: Doc, file: tea.go, decl: 35, sub: 0, line: 743 } |  |  | 0.463 |
| walker |  | 5013 | 37 | Code::CodeKey { rung: Doc, file: tea.go, decl: 48, sub: 0, line: 1269 } |  |  | 0.463 |
| ns | 5037 |  | 341 | `input.go`: the ultraviolet-event → `tea.Msg` translation table | 3.5 |  | 0.445 |
| walker |  | 5051 | 38 | Code::CodeKey { rung: Doc, file: tea.go, decl: 50, sub: 0, line: 1319 } |  |  | 0.445 |
| ns | 5113 |  | 76 | `Key` struct fields | 3.6 |  | 0.439 |
| ns | 5293 |  | 180 | Key message methods: `String`, `Keystroke`, `Key` | 3.7 | 3.6 | 0.434 |
| walker |  | 5301 | 250 | Fs::DirListing { dir: examples } |  |  | 0.438 |
| walker |  | 5547 | 246 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.439 |
| walker |  | 5588 | 41 | Code::CodeKey { rung: Doc, file: tea.go, decl: 20, sub: 0, line: 395 } |  |  | 0.439 |
| ns | 5614 |  | 321 | `key.go` key-code catalog: group headings plus the C0/G0 names | 3.8 |  | 0.421 |
| walker |  | 5630 | 42 | Code::CodeKey { rung: Doc, file: tea.go, decl: 56, sub: 0, line: 1427 } |  |  | 0.421 |
| walker |  | 5676 | 46 | Code::CodeKey { rung: Doc, file: tea.go, decl: 49, sub: 0, line: 1294 } |  |  | 0.421 |
| walker |  | 5725 | 49 | Code::CodeKey { rung: Doc, file: tea.go, decl: 16, sub: 0, line: 349 } |  |  | 0.421 |
| ns | 5797 |  | 183 | Modifier keys: the complete `KeyMod` constant set | 3.9 |  | 0.414 |
| ns | 6023 |  | 226 | `Mouse` struct and the complete mouse-button constant set | 3.10 | 3.1 | 0.405 |
| ns | 6200 |  | 177 | `MouseMode` enum: none / cell-motion / all-motion | 3.11 | 1.7 | 0.423 |
| walker |  | 6241 | 516 | Code::CodeKey { rung: Decl, file: tea.go, decl: 7, sub: 0, line: 84 } |  |  | 0.425 |
| walker |  | 6280 | 39 | Code::CodeKey { rung: Doc, file: tea.go, decl: 7, sub: 0, line: 84 } |  |  | 0.431 |
| walker |  | 6333 | 53 | Code::CodeKey { rung: Doc, file: tea.go, decl: 40, sub: 0, line: 991 } |  |  | 0.431 |
| ns | 6378 |  | 178 | Cursor value types: `Cursor`, `NewCursor`, `CursorShape` | 3.12 | 1.7 | 0.429 |
| walker |  | 6536 | 203 | Code::CodeKey { rung: Decl, file: tea.go, decl: 7, sub: 1, line: 84 } |  |  | 0.438 |
| ns | 6566 |  | 188 | Keyboard enhancements: request fields and response predicates | 3.13 | 3.2 | 0.435 |
| walker |  | 6746 | 210 | Code::CodeKey { rung: Decl, file: tea.go, decl: 7, sub: 2, line: 84 } |  |  | 0.441 |
| ns | 6759 |  | 193 | `Batch` vs `Sequence` semantics and where they execute | 3.14 | 3.3 | 0.437 |
| ns | 6959 |  | 200 | `Exec` / `ExecProcess` and the `ExecCommand` interface | 3.15 | 3.3 | 0.431 |
| walker |  | 6961 | 215 | Code::CodeKey { rung: Decl, file: tea.go, decl: 7, sub: 3, line: 84 } |  |  | 0.440 |
| walker |  | 7107 | 146 | Code::CodeKey { rung: Decl, file: tea.go, decl: 7, sub: 4, line: 84 } |  |  | 0.445 |
| ns | 7111 |  | 152 | Logging to a file: `LogToFile` / `LogToFileWith` | 3.16 |  | 0.441 |
| walker |  | 7164 | 57 | Code::CodeKey { rung: Doc, file: tea.go, decl: 43, sub: 0, line: 1204 } |  |  | 0.441 |
| ns | 7328 |  | 217 | The `renderer` interface: complete method set, and its two implementations | 4.1 |  | 0.434 |
| walker |  | 7410 | 246 | GoMod::File { file: go.mod } |  |  | 0.450 |
| walker |  | 7468 | 58 | Code::CodeKey { rung: Doc, file: tea.go, decl: 15, sub: 0, line: 336 } |  |  | 0.450 |
| walker |  | 7527 | 59 | Code::CodeKey { rung: Doc, file: tea.go, decl: 52, sub: 0, line: 1344 } |  |  | 0.450 |
| walker |  | 7588 | 61 | Code::CodeKey { rung: Doc, file: tea.go, decl: 53, sub: 0, line: 1372 } |  |  | 0.450 |
| ns | 7681 |  | 353 | `cursedRenderer`: every function in the 860-line renderer | 4.2 | 4.1 | 0.441 |
| walker |  | 7764 | 176 | Code::CodeKey { rung: Names, file: cursed_renderer.go, decl: 0, sub: 0, line: 0 } |  |  | 0.443 |
| walker |  | 7987 | 223 | Code::CodeKey { rung: Decl, file: cursed_renderer.go, decl: 1, sub: 0, line: 18 } |  |  | 0.443 |
| walker |  | 7996 | 9 | Code::CodeKey { rung: Doc, file: cursed_renderer.go, decl: 6, sub: 0, line: 75 } |  |  | 0.443 |
| walker |  | 8005 | 9 | Code::CodeKey { rung: Doc, file: cursed_renderer.go, decl: 7, sub: 0, line: 143 } |  |  | 0.443 |
| walker |  | 8015 | 10 | Code::CodeKey { rung: Doc, file: cursed_renderer.go, decl: 8, sub: 0, line: 249 } |  |  | 0.443 |
| walker |  | 8029 | 14 | Code::CodeKey { rung: Doc, file: cursed_renderer.go, decl: 4, sub: 0, line: 52 } |  |  | 0.443 |
| ns | 8039 |  | 358 | `View` fields → ANSI sequences, in `cursedRenderer.start` | 4.3 | 4.2 | 0.434 |
| walker |  | 8044 | 15 | Code::CodeKey { rung: Doc, file: cursed_renderer.go, decl: 5, sub: 0, line: 59 } |  |  | 0.434 |
| walker |  | 8314 | 270 | Code::CodeKey { rung: Names, file: nil_renderer.go, decl: 0, sub: 0, line: 0 } |  |  | 0.435 |
| walker |  | 8321 | 7 | Code::CodeKey { rung: Doc, file: nil_renderer.go, decl: 3, sub: 0, line: 15 } |  |  | 0.435 |
| walker |  | 8328 | 7 | Code::CodeKey { rung: Doc, file: nil_renderer.go, decl: 6, sub: 0, line: 24 } |  |  | 0.435 |
| walker |  | 8336 | 8 | Code::CodeKey { rung: Doc, file: nil_renderer.go, decl: 4, sub: 0, line: 18 } |  |  | 0.435 |
| walker |  | 8344 | 8 | Code::CodeKey { rung: Doc, file: nil_renderer.go, decl: 5, sub: 0, line: 21 } |  |  | 0.435 |
| walker |  | 8353 | 9 | Code::CodeKey { rung: Doc, file: nil_renderer.go, decl: 7, sub: 0, line: 27 } |  |  | 0.435 |
| walker |  | 8362 | 9 | Code::CodeKey { rung: Doc, file: nil_renderer.go, decl: 8, sub: 0, line: 30 } |  |  | 0.435 |
| walker |  | 8371 | 9 | Code::CodeKey { rung: Doc, file: nil_renderer.go, decl: 9, sub: 0, line: 33 } |  |  | 0.435 |
| walker |  | 8380 | 9 | Code::CodeKey { rung: Doc, file: nil_renderer.go, decl: 10, sub: 0, line: 36 } |  |  | 0.435 |
| walker |  | 8389 | 9 | Code::CodeKey { rung: Doc, file: nil_renderer.go, decl: 11, sub: 0, line: 39 } |  |  | 0.435 |
| walker |  | 8399 | 10 | Code::CodeKey { rung: Doc, file: nil_renderer.go, decl: 12, sub: 0, line: 42 } |  |  | 0.435 |
| walker |  | 8409 | 10 | Code::CodeKey { rung: Doc, file: nil_renderer.go, decl: 15, sub: 0, line: 51 } |  |  | 0.435 |
| walker |  | 8420 | 11 | Code::CodeKey { rung: Doc, file: nil_renderer.go, decl: 14, sub: 0, line: 48 } |  |  | 0.426 |
| ns | 8420 |  | 381 | Platform matrix: build tags and per-OS terminal functions | 4.4 |  | 0.426 |
| walker |  | 8432 | 12 | Code::CodeKey { rung: Doc, file: nil_renderer.go, decl: 13, sub: 0, line: 45 } |  |  | 0.426 |
| walker |  | 8439 | 7 | Code::CodeKey { rung: Body, file: nil_renderer.go, decl: 15, sub: 0, line: 51 } |  |  | 0.426 |
| walker |  | 8538 | 99 | Code::CodeKey { rung: Names, file: renderer.go, decl: 0, sub: 0, line: 0 } |  |  | 0.429 |
| walker |  | 8551 | 13 | Code::CodeKey { rung: Decl, file: renderer.go, decl: 3, sub: 0, line: 59 } |  |  | 0.429 |
| walker |  | 8590 | 39 | Code::CodeKey { rung: Decl, file: renderer.go, decl: 1, sub: 0, line: 10 } |  |  | 0.432 |
| ns | 8618 |  | 198 | `tty.go`: terminal acquisition, input loop, resize detection | 4.5 |  | 0.428 |
| ns | 8882 |  | 264 | `examples/` and `tutorials/` directory listings | 5.1 |  | 0.481 |
| walker |  | 8974 | 384 | Code::CodeKey { rung: Decl, file: renderer.go, decl: 2, sub: 0, line: 18 } |  |  | 0.496 |
| walker |  | 8987 | 13 | Code::CodeKey { rung: Doc, file: renderer.go, decl: 2, sub: 0, line: 18 } |  |  | 0.499 |
| ns | 9062 |  | 180 | `UPGRADE_GUIDE_V2.md`: complete section map | 5.2 |  | 0.494 |
| walker |  | 9132 | 145 | Code::CodeKey { rung: Names, file: commands.go, decl: 0, sub: 0, line: 0 } |  |  | 0.501 |
| walker |  | 9141 | 9 | Code::CodeKey { rung: Body, file: commands.go, decl: 9, sub: 0, line: 173 } |  |  | 0.501 |
| walker |  | 9191 | 50 | Code::CodeKey { rung: Names, file: screen.go, decl: 0, sub: 0, line: 0 } |  |  | 0.502 |
| walker |  | 9213 | 22 | Code::CodeKey { rung: Decl, file: screen.go, decl: 1, sub: 0, line: 9 } |  |  | 0.503 |
| walker |  | 9263 | 50 | Code::CodeKey { rung: Decl, file: screen.go, decl: 4, sub: 0, line: 62 } |  |  | 0.503 |
| walker |  | 9272 | 9 | Code::CodeKey { rung: Body, file: screen.go, decl: 2, sub: 0, line: 20 } |  |  | 0.503 |
| walker |  | 9290 | 18 | Code::CodeKey { rung: Doc, file: commands.go, decl: 4, sub: 0, line: 30 } |  |  | 0.504 |
| walker |  | 9305 | 15 | Code::CodeKey { rung: Body, file: commands.go, decl: 1, sub: 0, line: 15 } |  |  | 0.504 |
| walker |  | 9320 | 15 | Code::CodeKey { rung: Body, file: commands.go, decl: 3, sub: 0, line: 25 } |  |  | 0.504 |
| ns | 9347 |  | 285 | The v1→v2 migration checklist | 5.3 | 5.2 | 0.500 |
| walker |  | 9497 | 177 | Code::CodeKey { rung: Names, file: cursed_renderer.go, decl: 0, sub: 1, line: 0 } |  |  | 0.507 |
| walker |  | 9506 | 9 | Code::CodeKey { rung: Doc, file: cursed_renderer.go, decl: 9, sub: 0, line: 257 } |  |  | 0.507 |
| walker |  | 9515 | 9 | Code::CodeKey { rung: Doc, file: cursed_renderer.go, decl: 10, sub: 0, line: 579 } |  |  | 0.507 |
| ns | 9516 |  | 169 | README section map | 5.4 |  | 0.516 |
| walker |  | 9524 | 9 | Code::CodeKey { rung: Doc, file: cursed_renderer.go, decl: 11, sub: 0, line: 587 } |  |  | 0.516 |
| walker |  | 9533 | 9 | Code::CodeKey { rung: Doc, file: cursed_renderer.go, decl: 14, sub: 0, line: 619 } |  |  | 0.516 |
| walker |  | 9543 | 10 | Code::CodeKey { rung: Doc, file: cursed_renderer.go, decl: 15, sub: 0, line: 634 } |  |  | 0.516 |
| walker |  | 9554 | 11 | Code::CodeKey { rung: Doc, file: cursed_renderer.go, decl: 13, sub: 0, line: 611 } |  |  | 0.516 |
| walker |  | 9740 | 186 | Code::CodeKey { rung: Names, file: cursed_renderer.go, decl: 0, sub: 2, line: 0 } |  |  | 0.529 |
| walker |  | 9750 | 10 | Code::CodeKey { rung: Doc, file: cursed_renderer.go, decl: 22, sub: 0, line: 707 } |  |  | 0.529 |
| walker |  | 9760 | 10 | Code::CodeKey { rung: Doc, file: cursed_renderer.go, decl: 23, sub: 0, line: 766 } |  |  | 0.529 |
| walker |  | 9771 | 11 | Code::CodeKey { rung: Doc, file: cursed_renderer.go, decl: 21, sub: 0, line: 690 } |  |  | 0.529 |
| walker |  | 9783 | 12 | Code::CodeKey { rung: Doc, file: cursed_renderer.go, decl: 20, sub: 0, line: 683 } |  |  | 0.529 |
| walker |  | 9797 | 14 | Code::CodeKey { rung: Doc, file: cursed_renderer.go, decl: 19, sub: 0, line: 674 } |  |  | 0.529 |
| walker |  | 9824 | 27 | Code::CodeKey { rung: Doc, file: renderer.go, decl: 6, sub: 0, line: 96 } |  |  | 0.529 |
| ns | 9876 |  | 360 | Test suite: every test function, and the golden-file fixtures | 5.5 |  | 0.519 |
| walker |  | 9906 | 82 | Markdown::Section { file: README.md, section_index: 16, keeps_default_concavity: false } |  |  | 0.519 |
| ns | 9917 |  | 41 | CI workflow inventory and golden-test fixture directories | 5.6 |  | 0.523 |
| walker |  | 9929 | 23 | Code::CodeKey { rung: Names, file: input.go, decl: 0, sub: 0, line: 0 } |  |  | 0.523 |
| walker |  | 9947 | 18 | Code::CodeKey { rung: Doc, file: input.go, decl: 1, sub: 0, line: 8 } |  |  | 0.524 |
| walker |  | 9994 | 47 | Code::CodeKey { rung: Names, file: options.go, decl: 0, sub: 0, line: 0 } |  |  | 0.529 |
