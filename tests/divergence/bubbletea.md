Score(3000)=0.577 I=0.788 C=0.422 ns_rows≤3K=18/50 grid(1000/1442/2080/3000/4327/6240/9000)=0.593/0.607/0.544/0.577/0.526/0.496/0.573

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
| walker |  | 2002 | 7 | Code::CodeKey { rung: Decl, file: tea.go, decl: 20, sub: 0, line: 426 } |  |  | 0.544 |
| walker |  | 2117 | 115 | Code::CodeKey { rung: Decl, file: tea.go, decl: 15, sub: 0, line: 336 } |  |  | 0.544 |
| ns | 2130 |  | 129 | Build/test/lint entry points (`Taskfile.yaml`) | 2.4 |  | 0.569 |
| walker |  | 2265 | 148 | Code::CodeKey { rung: Decl, file: tea.go, decl: 17, sub: 0, line: 357 } |  |  | 0.569 |
| walker |  | 2277 | 12 | Code::CodeKey { rung: Doc, file: tea.go, decl: 20, sub: 0, line: 426 } |  |  | 0.569 |
| walker |  | 2290 | 13 | Code::CodeKey { rung: Doc, file: tea.go, decl: 28, sub: 0, line: 595 } |  |  | 0.577 |
| walker |  | 2435 | 145 | Code::CodeKey { rung: Names, file: tea.go, decl: 0, sub: 2, line: 0 } |  |  | 0.610 |
| ns | 2436 |  | 306 | Event-loop dispatch table: every internally-handled message type | 2.5 |  | 0.564 |
| walker |  | 2449 | 14 | Code::CodeKey { rung: Doc, file: tea.go, decl: 17, sub: 0, line: 357 } |  |  | 0.564 |
| walker |  | 2464 | 15 | Code::CodeKey { rung: Doc, file: tea.go, decl: 10, sub: 0, line: 284 } |  |  | 0.564 |
| walker |  | 2480 | 16 | Code::CodeKey { rung: Doc, file: tea.go, decl: 5, sub: 0, line: 53 } |  |  | 0.604 |
| walker |  | 2497 | 17 | Code::CodeKey { rung: Doc, file: tea.go, decl: 14, sub: 0, line: 321 } |  |  | 0.604 |
| walker |  | 2514 | 17 | Code::CodeKey { rung: Doc, file: tea.go, decl: 21, sub: 0, line: 555 } |  |  | 0.606 |
| walker |  | 2531 | 17 | Code::CodeKey { rung: Doc, file: tea.go, decl: 23, sub: 0, line: 564 } |  |  | 0.606 |
| walker |  | 2549 | 18 | Code::CodeKey { rung: Doc, file: tea.go, decl: 33, sub: 0, line: 1209 } |  |  | 0.606 |
| walker |  | 2568 | 19 | Code::CodeKey { rung: Doc, file: tea.go, decl: 2, sub: 0, line: 42 } |  |  | 0.610 |
| walker |  | 2586 | 18 | Fs::DirListing { dir: .github/ISSUE_TEMPLATE } |  |  | 0.610 |
| walker |  | 2609 | 23 | Code::CodeKey { rung: Doc, file: tea.go, decl: 27, sub: 0, line: 590 } |  |  | 0.610 |
| walker |  | 2659 | 50 | GoMod::File { file: tutorials/go.mod } |  |  | 0.610 |
| ns | 2682 |  | 246 | Event-loop skeleton: translate → filter → dispatch → update → render | 2.6 | 2.5 | 0.575 |
| walker |  | 2683 | 24 | Code::CodeKey { rung: Doc, file: tea.go, decl: 18, sub: 0, line: 374 } |  |  | 0.575 |
| walker |  | 2708 | 25 | Code::CodeKey { rung: Doc, file: tea.go, decl: 1, sub: 0, line: 39 } |  |  | 0.580 |
| walker |  | 2736 | 28 | Code::CodeKey { rung: Doc, file: tea.go, decl: 25, sub: 0, line: 578 } |  |  | 0.580 |
| walker |  | 2768 | 32 | Code::CodeKey { rung: Doc, file: tea.go, decl: 22, sub: 0, line: 561 } |  |  | 0.586 |
| ns | 2901 |  | 219 | Program control messages: suspend, resume, interrupt | 2.7 | 1.9 | 0.572 |
| walker |  | 3018 | 250 | Fs::DirListing { dir: examples } |  |  | 0.578 |
| ns | 3100 |  | 199 | `Run()`: how input and the TTY are acquired | 2.8 |  | 0.558 |
| walker |  | 3264 | 246 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.559 |
| ns | 3288 |  | 188 | `Run()`: renderer choice, colour profile, first three messages | 2.9 | 2.8 | 0.543 |
| walker |  | 3295 | 31 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.543 |
| walker |  | 3329 | 34 | Code::CodeKey { rung: Doc, file: tea.go, decl: 4, sub: 0, line: 50 } |  |  | 0.555 |
| walker |  | 3365 | 36 | Code::CodeKey { rung: Doc, file: tea.go, decl: 3, sub: 0, line: 46 } |  |  | 0.564 |
| walker |  | 3403 | 38 | Code::CodeKey { rung: Doc, file: tea.go, decl: 34, sub: 0, line: 1319 } |  |  | 0.564 |
| ns | 3474 |  | 186 | `Run()`: model init, event loop, graceful shutdown | 2.10 | 2.9 | 0.545 |
| ns | 3693 |  | 219 | Debug hooks: `TEA_TRACE`, `TEA_DEBUG`, panic recovery | 2.11 |  | 0.534 |
| ns | 3916 |  | 223 | Frame pacing: the render ticker and the FPS bounds | 2.12 |  | 0.516 |
| walker |  | 3919 | 516 | Code::CodeKey { rung: Decl, file: tea.go, decl: 7, sub: 0, line: 84 } |  |  | 0.519 |
| walker |  | 3958 | 39 | Code::CodeKey { rung: Doc, file: tea.go, decl: 7, sub: 0, line: 84 } |  |  | 0.527 |
| ns | 4118 |  | 202 | Input event message types: keys, mouse, paste, focus (complete) | 3.1 |  | 0.511 |
| walker |  | 4161 | 203 | Code::CodeKey { rung: Decl, file: tea.go, decl: 7, sub: 1, line: 84 } |  |  | 0.522 |
| walker |  | 4371 | 210 | Code::CodeKey { rung: Decl, file: tea.go, decl: 7, sub: 2, line: 84 } |  |  | 0.531 |
| ns | 4380 |  | 262 | Terminal report message types: colour, clipboard, size, capability (complete) | 3.2 |  | 0.513 |
| ns | 4550 |  | 170 | Command constructors, part 1: lifecycle, batching, timing, output | 3.3 | 2.7 | 0.504 |
| walker |  | 4586 | 215 | Code::CodeKey { rung: Decl, file: tea.go, decl: 7, sub: 3, line: 84 } |  |  | 0.516 |
| ns | 4696 |  | 146 | Command constructors, part 2: terminal queries and clipboard | 3.4 | 3.3 | 0.508 |
| walker |  | 4732 | 146 | Code::CodeKey { rung: Decl, file: tea.go, decl: 7, sub: 4, line: 84 } |  |  | 0.514 |
| walker |  | 4781 | 49 | Code::CodeKey { rung: Doc, file: tea.go, decl: 16, sub: 0, line: 349 } |  |  | 0.514 |
| walker |  | 5027 | 246 | GoMod::File { file: go.mod } |  |  | 0.537 |
| ns | 5037 |  | 341 | `input.go`: the ultraviolet-event → `tea.Msg` translation table | 3.5 |  | 0.515 |
| walker |  | 5080 | 53 | Code::CodeKey { rung: Doc, file: tea.go, decl: 29, sub: 0, line: 991 } |  |  | 0.515 |
| ns | 5113 |  | 76 | `Key` struct fields | 3.6 |  | 0.509 |
| walker |  | 5256 | 176 | Code::CodeKey { rung: Names, file: cursed_renderer.go, decl: 0, sub: 0, line: 0 } |  |  | 0.509 |
| ns | 5293 |  | 180 | Key message methods: `String`, `Keystroke`, `Key` | 3.7 | 3.6 | 0.503 |
| walker |  | 5479 | 223 | Code::CodeKey { rung: Decl, file: cursed_renderer.go, decl: 1, sub: 0, line: 18 } |  |  | 0.503 |
| walker |  | 5488 | 9 | Code::CodeKey { rung: Doc, file: cursed_renderer.go, decl: 6, sub: 0, line: 75 } |  |  | 0.503 |
| walker |  | 5497 | 9 | Code::CodeKey { rung: Doc, file: cursed_renderer.go, decl: 7, sub: 0, line: 143 } |  |  | 0.503 |
| walker |  | 5507 | 10 | Code::CodeKey { rung: Doc, file: cursed_renderer.go, decl: 8, sub: 0, line: 249 } |  |  | 0.503 |
| walker |  | 5521 | 14 | Code::CodeKey { rung: Doc, file: cursed_renderer.go, decl: 4, sub: 0, line: 52 } |  |  | 0.503 |
| walker |  | 5536 | 15 | Code::CodeKey { rung: Doc, file: cursed_renderer.go, decl: 5, sub: 0, line: 59 } |  |  | 0.503 |
| ns | 5614 |  | 321 | `key.go` key-code catalog: group headings plus the C0/G0 names | 3.8 |  | 0.483 |
| ns | 5797 |  | 183 | Modifier keys: the complete `KeyMod` constant set | 3.9 |  | 0.475 |
| walker |  | 5806 | 270 | Code::CodeKey { rung: Names, file: nil_renderer.go, decl: 0, sub: 0, line: 0 } |  |  | 0.475 |
| walker |  | 5813 | 7 | Code::CodeKey { rung: Doc, file: nil_renderer.go, decl: 3, sub: 0, line: 15 } |  |  | 0.475 |
| walker |  | 5820 | 7 | Code::CodeKey { rung: Doc, file: nil_renderer.go, decl: 6, sub: 0, line: 24 } |  |  | 0.475 |
| walker |  | 5828 | 8 | Code::CodeKey { rung: Doc, file: nil_renderer.go, decl: 4, sub: 0, line: 18 } |  |  | 0.475 |
| walker |  | 5836 | 8 | Code::CodeKey { rung: Doc, file: nil_renderer.go, decl: 5, sub: 0, line: 21 } |  |  | 0.475 |
| walker |  | 5845 | 9 | Code::CodeKey { rung: Doc, file: nil_renderer.go, decl: 7, sub: 0, line: 27 } |  |  | 0.475 |
| walker |  | 5854 | 9 | Code::CodeKey { rung: Doc, file: nil_renderer.go, decl: 8, sub: 0, line: 30 } |  |  | 0.475 |
| walker |  | 5863 | 9 | Code::CodeKey { rung: Doc, file: nil_renderer.go, decl: 9, sub: 0, line: 33 } |  |  | 0.475 |
| walker |  | 5872 | 9 | Code::CodeKey { rung: Doc, file: nil_renderer.go, decl: 10, sub: 0, line: 36 } |  |  | 0.475 |
| walker |  | 5881 | 9 | Code::CodeKey { rung: Doc, file: nil_renderer.go, decl: 11, sub: 0, line: 39 } |  |  | 0.475 |
| walker |  | 5891 | 10 | Code::CodeKey { rung: Doc, file: nil_renderer.go, decl: 12, sub: 0, line: 42 } |  |  | 0.475 |
| walker |  | 5901 | 10 | Code::CodeKey { rung: Doc, file: nil_renderer.go, decl: 15, sub: 0, line: 51 } |  |  | 0.475 |
| walker |  | 5912 | 11 | Code::CodeKey { rung: Doc, file: nil_renderer.go, decl: 14, sub: 0, line: 48 } |  |  | 0.475 |
| walker |  | 5924 | 12 | Code::CodeKey { rung: Doc, file: nil_renderer.go, decl: 13, sub: 0, line: 45 } |  |  | 0.475 |
| walker |  | 5931 | 7 | Code::CodeKey { rung: Body, file: nil_renderer.go, decl: 15, sub: 0, line: 51 } |  |  | 0.475 |
| walker |  | 5966 | 35 | Code::CodeKey { rung: Names, file: renderer.go, decl: 0, sub: 0, line: 0 } |  |  | 0.477 |
| ns | 6023 |  | 226 | `Mouse` struct and the complete mouse-button constant set | 3.10 | 3.1 | 0.467 |
| walker |  | 6065 | 99 | Code::CodeKey { rung: Names, file: commands.go, decl: 0, sub: 0, line: 0 } |  |  | 0.477 |
| walker |  | 6074 | 9 | Code::CodeKey { rung: Body, file: commands.go, decl: 6, sub: 0, line: 173 } |  |  | 0.477 |
| walker |  | 6112 | 38 | Code::CodeKey { rung: Names, file: screen.go, decl: 0, sub: 0, line: 0 } |  |  | 0.480 |
| walker |  | 6134 | 22 | Code::CodeKey { rung: Decl, file: screen.go, decl: 1, sub: 0, line: 9 } |  |  | 0.480 |
| walker |  | 6184 | 50 | Code::CodeKey { rung: Decl, file: screen.go, decl: 3, sub: 0, line: 62 } |  |  | 0.480 |
| walker |  | 6193 | 9 | Code::CodeKey { rung: Body, file: screen.go, decl: 2, sub: 0, line: 20 } |  |  | 0.480 |
| ns | 6200 |  | 177 | `MouseMode` enum: none / cell-motion / all-motion | 3.11 | 1.7 | 0.496 |
| walker |  | 6208 | 15 | Code::CodeKey { rung: Body, file: commands.go, decl: 1, sub: 0, line: 15 } |  |  | 0.496 |
| walker |  | 6223 | 15 | Code::CodeKey { rung: Body, file: commands.go, decl: 3, sub: 0, line: 25 } |  |  | 0.496 |
| ns | 6378 |  | 178 | Cursor value types: `Cursor`, `NewCursor`, `CursorShape` | 3.12 | 1.7 | 0.492 |
| walker |  | 6400 | 177 | Code::CodeKey { rung: Names, file: cursed_renderer.go, decl: 0, sub: 1, line: 0 } |  |  | 0.493 |
| walker |  | 6409 | 9 | Code::CodeKey { rung: Doc, file: cursed_renderer.go, decl: 9, sub: 0, line: 257 } |  |  | 0.493 |
| walker |  | 6418 | 9 | Code::CodeKey { rung: Doc, file: cursed_renderer.go, decl: 10, sub: 0, line: 579 } |  |  | 0.493 |
| walker |  | 6427 | 9 | Code::CodeKey { rung: Doc, file: cursed_renderer.go, decl: 11, sub: 0, line: 587 } |  |  | 0.493 |
| walker |  | 6436 | 9 | Code::CodeKey { rung: Doc, file: cursed_renderer.go, decl: 14, sub: 0, line: 619 } |  |  | 0.493 |
| walker |  | 6446 | 10 | Code::CodeKey { rung: Doc, file: cursed_renderer.go, decl: 15, sub: 0, line: 634 } |  |  | 0.493 |
| walker |  | 6457 | 11 | Code::CodeKey { rung: Doc, file: cursed_renderer.go, decl: 13, sub: 0, line: 611 } |  |  | 0.493 |
| ns | 6566 |  | 188 | Keyboard enhancements: request fields and response predicates | 3.13 | 3.2 | 0.489 |
| walker |  | 6643 | 186 | Code::CodeKey { rung: Names, file: cursed_renderer.go, decl: 0, sub: 2, line: 0 } |  |  | 0.490 |
| walker |  | 6653 | 10 | Code::CodeKey { rung: Doc, file: cursed_renderer.go, decl: 22, sub: 0, line: 707 } |  |  | 0.490 |
| walker |  | 6663 | 10 | Code::CodeKey { rung: Doc, file: cursed_renderer.go, decl: 23, sub: 0, line: 766 } |  |  | 0.490 |
| walker |  | 6674 | 11 | Code::CodeKey { rung: Doc, file: cursed_renderer.go, decl: 21, sub: 0, line: 690 } |  |  | 0.490 |
| walker |  | 6686 | 12 | Code::CodeKey { rung: Doc, file: cursed_renderer.go, decl: 20, sub: 0, line: 683 } |  |  | 0.490 |
| walker |  | 6700 | 14 | Code::CodeKey { rung: Doc, file: cursed_renderer.go, decl: 19, sub: 0, line: 674 } |  |  | 0.490 |
| ns | 6759 |  | 193 | `Batch` vs `Sequence` semantics and where they execute | 3.14 | 3.3 | 0.484 |
| walker |  | 6801 | 101 | Markdown::Section { file: README.md, section_index: 9, keeps_default_concavity: false } |  |  | 0.484 |
| walker |  | 6858 | 57 | Code::CodeKey { rung: Doc, file: tea.go, decl: 32, sub: 0, line: 1204 } |  |  | 0.484 |
| walker |  | 6916 | 58 | Code::CodeKey { rung: Doc, file: tea.go, decl: 15, sub: 0, line: 336 } |  |  | 0.484 |
| ns | 6959 |  | 200 | `Exec` / `ExecProcess` and the `ExecCommand` interface | 3.15 | 3.3 | 0.478 |
| walker |  | 6998 | 82 | Markdown::Section { file: README.md, section_index: 16, keeps_default_concavity: false } |  |  | 0.478 |
| walker |  | 7021 | 23 | Code::CodeKey { rung: Names, file: input.go, decl: 0, sub: 0, line: 0 } |  |  | 0.478 |
| ns | 7111 |  | 152 | Logging to a file: `LogToFile` / `LogToFileWith` | 3.16 |  | 0.474 |
| walker |  | 7227 | 206 | Code::CodeKey { rung: Names, file: options.go, decl: 0, sub: 0, line: 0 } |  |  | 0.499 |
| ns | 7328 |  | 217 | The `renderer` interface: complete method set, and its two implementations | 4.1 |  | 0.491 |
| walker |  | 7635 | 408 | Code::CodeKey { rung: Names, file: mouse.go, decl: 0, sub: 0, line: 0 } |  |  | 0.512 |
| walker |  | 7640 | 5 | Code::CodeKey { rung: Decl, file: mouse.go, decl: 2, sub: 0, line: 29 } |  |  | 0.513 |
| walker |  | 7673 | 33 | Code::CodeKey { rung: Decl, file: mouse.go, decl: 4, sub: 0, line: 71 } |  |  | 0.519 |
| ns | 7681 |  | 353 | `cursedRenderer`: every function in the 860-line renderer | 4.2 | 4.1 | 0.536 |
| walker |  | 7713 | 40 | Code::CodeKey { rung: Decl, file: mouse.go, decl: 3, sub: 0, line: 46 } |  |  | 0.538 |
| walker |  | 7726 | 13 | Code::CodeKey { rung: Doc, file: mouse.go, decl: 5, sub: 0, line: 78 } |  |  | 0.538 |
| walker |  | 7740 | 14 | Code::CodeKey { rung: Doc, file: mouse.go, decl: 7, sub: 0, line: 86 } |  |  | 0.538 |
| walker |  | 7754 | 14 | Code::CodeKey { rung: Doc, file: mouse.go, decl: 10, sub: 0, line: 101 } |  |  | 0.538 |
| walker |  | 7768 | 14 | Code::CodeKey { rung: Doc, file: mouse.go, decl: 13, sub: 0, line: 116 } |  |  | 0.538 |
| walker |  | 7782 | 14 | Code::CodeKey { rung: Doc, file: mouse.go, decl: 15, sub: 0, line: 128 } |  |  | 0.538 |
| walker |  | 7796 | 14 | Code::CodeKey { rung: Doc, file: mouse.go, decl: 16, sub: 0, line: 131 } |  |  | 0.538 |
| walker |  | 7811 | 15 | Code::CodeKey { rung: Doc, file: mouse.go, decl: 6, sub: 0, line: 83 } |  |  | 0.538 |
| walker |  | 7826 | 15 | Code::CodeKey { rung: Doc, file: mouse.go, decl: 9, sub: 0, line: 98 } |  |  | 0.538 |
| walker |  | 7841 | 15 | Code::CodeKey { rung: Doc, file: mouse.go, decl: 12, sub: 0, line: 113 } |  |  | 0.538 |
| walker |  | 8035 | 194 | Code::CodeKey { rung: Names, file: color.go, decl: 0, sub: 0, line: 0 } |  |  | 0.542 |
| ns | 8039 |  | 358 | `View` fields → ANSI sequences, in `cursedRenderer.start` | 4.3 | 4.2 | 0.531 |
| walker |  | 8047 | 12 | Code::CodeKey { rung: Doc, file: color.go, decl: 5, sub: 0, line: 39 } |  |  | 0.531 |
| walker |  | 8059 | 12 | Code::CodeKey { rung: Doc, file: color.go, decl: 8, sub: 0, line: 70 } |  |  | 0.531 |
| walker |  | 8071 | 12 | Code::CodeKey { rung: Doc, file: color.go, decl: 11, sub: 0, line: 84 } |  |  | 0.531 |
| walker |  | 8085 | 14 | Code::CodeKey { rung: Doc, file: color.go, decl: 6, sub: 0, line: 44 } |  |  | 0.531 |
| walker |  | 8099 | 14 | Code::CodeKey { rung: Doc, file: color.go, decl: 9, sub: 0, line: 75 } |  |  | 0.531 |
| walker |  | 8113 | 14 | Code::CodeKey { rung: Doc, file: color.go, decl: 12, sub: 0, line: 89 } |  |  | 0.531 |
| walker |  | 8122 | 9 | Code::CodeKey { rung: Body, file: color.go, decl: 1, sub: 0, line: 13 } |  |  | 0.531 |
| walker |  | 8140 | 18 | Code::CodeKey { rung: Doc, file: color.go, decl: 1, sub: 0, line: 13 } |  |  | 0.531 |
| walker |  | 8158 | 18 | Code::CodeKey { rung: Doc, file: color.go, decl: 2, sub: 0, line: 21 } |  |  | 0.531 |
| walker |  | 8176 | 18 | Code::CodeKey { rung: Doc, file: color.go, decl: 3, sub: 0, line: 29 } |  |  | 0.531 |
| walker |  | 8194 | 18 | Code::CodeKey { rung: Doc, file: input.go, decl: 1, sub: 0, line: 8 } |  |  | 0.531 |
| walker |  | 8212 | 18 | Code::CodeKey { rung: Doc, file: mouse.go, decl: 1, sub: 0, line: 10 } |  |  | 0.531 |
| walker |  | 8221 | 9 | Code::CodeKey { rung: Body, file: color.go, decl: 2, sub: 0, line: 21 } |  |  | 0.531 |
| walker |  | 8230 | 9 | Code::CodeKey { rung: Body, file: color.go, decl: 3, sub: 0, line: 29 } |  |  | 0.531 |
| walker |  | 8254 | 24 | Code::CodeKey { rung: Doc, file: options.go, decl: 8, sub: 0, line: 84 } |  |  | 0.531 |
| walker |  | 8277 | 23 | Code::CodeKey { rung: Names, file: tty.go, decl: 0, sub: 0, line: 0 } |  |  | 0.531 |
| walker |  | 8296 | 19 | Code::CodeKey { rung: Doc, file: tty.go, decl: 1, sub: 0, line: 130 } |  |  | 0.531 |
| walker |  | 8373 | 77 | Code::CodeKey { rung: Names, file: tty_unix.go, decl: 0, sub: 0, line: 0 } |  |  | 0.531 |
| walker |  | 8387 | 14 | Code::CodeKey { rung: Doc, file: tty_unix.go, decl: 3, sub: 0, line: 40 } |  |  | 0.531 |
| walker |  | 8413 | 26 | Code::CodeKey { rung: Names, file: raw.go, decl: 0, sub: 0, line: 0 } |  |  | 0.533 |
| ns | 8420 |  | 381 | Platform matrix: build tags and per-OS terminal functions | 4.4 |  | 0.523 |
| walker |  | 8426 | 13 | Code::CodeKey { rung: Decl, file: raw.go, decl: 1, sub: 0, line: 5 } |  |  | 0.523 |
| walker |  | 8469 | 43 | Code::CodeKey { rung: Names, file: termcap.go, decl: 0, sub: 0, line: 0 } |  |  | 0.526 |
| walker |  | 8481 | 12 | Code::CodeKey { rung: Decl, file: termcap.go, decl: 2, sub: 0, line: 41 } |  |  | 0.527 |
| walker |  | 8489 | 8 | Code::CodeKey { rung: Body, file: termcap.go, decl: 3, sub: 0, line: 46 } |  |  | 0.527 |
| walker |  | 8501 | 12 | Code::CodeKey { rung: Doc, file: termcap.go, decl: 3, sub: 0, line: 46 } |  |  | 0.527 |
| ns | 8618 |  | 198 | `tty.go`: terminal acquisition, input loop, resize detection | 4.5 |  | 0.522 |
| walker |  | 8724 | 223 | Code::CodeKey { rung: Names, file: key.go, decl: 0, sub: 0, line: 0 } |  |  | 0.523 |
| walker |  | 8766 | 42 | Code::CodeKey { rung: Decl, file: key.go, decl: 2, sub: 0, line: 16 } |  |  | 0.524 |
| walker |  | 8801 | 35 | Code::CodeKey { rung: Decl, file: key.go, decl: 2, sub: 2, line: 16 } |  |  | 0.526 |
| walker |  | 8812 | 11 | Code::CodeKey { rung: Decl, file: key.go, decl: 2, sub: 3, line: 16 } |  |  | 0.527 |
| walker |  | 8847 | 35 | Code::CodeKey { rung: Decl, file: key.go, decl: 2, sub: 8, line: 16 } |  |  | 0.529 |
| walker |  | 8875 | 28 | Code::CodeKey { rung: Decl, file: key.go, decl: 2, sub: 10, line: 16 } |  |  | 0.531 |
| ns | 8882 |  | 264 | `examples/` and `tutorials/` directory listings | 5.1 |  | 0.569 |
| walker |  | 8910 | 35 | Code::CodeKey { rung: Decl, file: key.go, decl: 1, sub: 0, line: 9 } |  |  | 0.573 |
| walker |  | 8917 | 7 | Code::CodeKey { rung: Doc, file: key.go, decl: 2, sub: 0, line: 16 } |  |  | 0.573 |
| ns | 9062 |  | 180 | `UPGRADE_GUIDE_V2.md`: complete section map | 5.2 |  | 0.566 |
| walker |  | 9092 | 175 | Code::CodeKey { rung: Names, file: key.go, decl: 0, sub: 1, line: 0 } |  |  | 0.566 |
| walker |  | 9268 | 176 | Code::CodeKey { rung: Names, file: key.go, decl: 0, sub: 2, line: 0 } |  |  | 0.567 |
| ns | 9347 |  | 285 | The v1→v2 migration checklist | 5.3 | 5.2 | 0.562 |
| walker |  | 9459 | 191 | Code::CodeKey { rung: Names, file: key.go, decl: 0, sub: 3, line: 0 } |  |  | 0.564 |
| ns | 9516 |  | 169 | README section map | 5.4 |  | 0.571 |
| walker |  | 9656 | 197 | Code::CodeKey { rung: Names, file: key.go, decl: 0, sub: 4, line: 0 } |  |  | 0.571 |
| walker |  | 9712 | 56 | Code::CodeKey { rung: Names, file: tty_windows.go, decl: 0, sub: 0, line: 0 } |  |  | 0.573 |
| ns | 9876 |  | 360 | Test suite: every test function, and the golden-file fixtures | 5.5 |  | 0.561 |
| walker |  | 9907 | 195 | Code::CodeKey { rung: Names, file: key.go, decl: 0, sub: 5, line: 0 } |  |  | 0.561 |
| ns | 9917 |  | 41 | CI workflow inventory and golden-test fixture directories | 5.6 |  | 0.565 |
| walker |  | 9998 | 91 | Code::CodeKey { rung: Names, file: key.go, decl: 0, sub: 6, line: 0 } |  |  | 0.565 |
