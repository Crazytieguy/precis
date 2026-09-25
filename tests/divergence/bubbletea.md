Score(3000)=0.541 I=0.764 C=0.383 ns_rows≤3K=18/50 grid(1000/1442/2080/3000/4327/6240/9000)=0.584/0.532/0.612/0.541/0.455/0.491/0.542

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
| walker |  | 436 | 14 | Code::CodeKey { rung: Names, file: profile.go, decl: 0, sub: 0, line: 0 } |  |  | 0.039 |
| walker |  | 447 | 11 | Code::CodeKey { rung: Decl, file: profile.go, decl: 1, sub: 0, line: 13 } |  |  | 0.039 |
| ns | 450 |  | 112 | `Model` method doc comments (refines 1.2) | 1.5 | 1.2 | 0.033 |
| walker |  | 523 | 76 | Markdown::ReadmeHeadline { file: README.md } |  |  | 0.033 |
| walker |  | 535 | 12 | Fs::DirListing { dir: .github } |  |  | 0.033 |
| walker |  | 566 | 31 | Fs::DirListing { dir: .github/workflows } |  |  | 0.033 |
| walker |  | 586 | 20 | Code::CodeKey { rung: Names, file: termios_other.go, decl: 0, sub: 0, line: 0 } |  |  | 0.033 |
| ns | 652 |  | 202 | Complete root directory listing | 1.6 |  | 0.413 |
| walker |  | 736 | 150 | Code::CodeKey { rung: ModuleDoc, file: tea.go, decl: 0, sub: 0, line: 0 } |  |  | 0.712 |
| walker |  | 758 | 22 | Code::CodeKey { rung: Names, file: focus.go, decl: 0, sub: 0, line: 0 } |  |  | 0.712 |
| walker |  | 780 | 22 | Code::CodeKey { rung: Names, file: signals_unix.go, decl: 0, sub: 0, line: 0 } |  |  | 0.712 |
| walker |  | 802 | 22 | Code::CodeKey { rung: Names, file: signals_windows.go, decl: 0, sub: 0, line: 0 } |  |  | 0.712 |
| walker |  | 825 | 23 | Code::CodeKey { rung: Names, file: input.go, decl: 0, sub: 0, line: 0 } |  |  | 0.712 |
| walker |  | 849 | 24 | Code::CodeKey { rung: Names, file: termios_windows.go, decl: 0, sub: 0, line: 0 } |  |  | 0.712 |
| ns | 851 |  | 199 | `View` struct: header plus every field name | 1.7 |  | 0.621 |
| walker |  | 874 | 25 | Code::CodeKey { rung: Names, file: termios_bsd.go, decl: 0, sub: 0, line: 0 } |  |  | 0.621 |
| walker |  | 899 | 25 | Code::CodeKey { rung: Names, file: termios_unix.go, decl: 0, sub: 0, line: 0 } |  |  | 0.621 |
| walker |  | 925 | 26 | Code::CodeKey { rung: Names, file: raw.go, decl: 0, sub: 0, line: 0 } |  |  | 0.621 |
| walker |  | 938 | 13 | Code::CodeKey { rung: Decl, file: raw.go, decl: 1, sub: 0, line: 5 } |  |  | 0.621 |
| ns | 975 |  | 124 | `NewView` / `View.SetContent` constructors | 1.8 | 1.7 | 0.584 |
| walker |  | 982 | 44 | GoMod::Identity { file: tutorials/go.mod } |  |  | 0.584 |
| ns | 1132 |  | 157 | `Cmd` semantics and the `Quit` command | 1.9 | 1.3 | 0.548 |
| walker |  | 1280 | 298 | Code::CodeKey { rung: Names, file: tea.go, decl: 0, sub: 0, line: 0 } |  |  | 0.554 |
| walker |  | 1291 | 11 | Code::CodeKey { rung: Decl, file: tea.go, decl: 13, sub: 0, line: 312 } |  |  | 0.554 |
| walker |  | 1298 | 7 | Code::CodeKey { rung: Doc, file: tea.go, decl: 13, sub: 0, line: 312 } |  |  | 0.554 |
| walker |  | 1307 | 9 | Code::CodeKey { rung: Body, file: tea.go, decl: 9, sub: 0, line: 279 } |  |  | 0.556 |
| walker |  | 1322 | 15 | Code::CodeKey { rung: Doc, file: tea.go, decl: 10, sub: 0, line: 284 } |  |  | 0.556 |
| walker |  | 1338 | 16 | Code::CodeKey { rung: Doc, file: tea.go, decl: 12, sub: 0, line: 309 } |  |  | 0.531 |
| ns | 1338 |  | 206 | Direct dependency set from `go.mod` | 1.10 |  | 0.531 |
| walker |  | 1355 | 17 | Code::CodeKey { rung: Doc, file: tea.go, decl: 14, sub: 0, line: 321 } |  |  | 0.531 |
| walker |  | 1374 | 19 | Code::CodeKey { rung: Doc, file: tea.go, decl: 2, sub: 0, line: 42 } |  |  | 0.532 |
| walker |  | 1489 | 115 | Code::CodeKey { rung: Decl, file: tea.go, decl: 15, sub: 0, line: 336 } |  |  | 0.532 |
| walker |  | 1514 | 25 | Code::CodeKey { rung: Doc, file: tea.go, decl: 1, sub: 0, line: 39 } |  |  | 0.533 |
| ns | 1522 |  | 184 | README lede: positioning and shipped feature set | 1.11 |  | 0.518 |
| ns | 1665 |  | 143 | Sentinel errors returned by `Program.Run` | 2.1 |  | 0.514 |
| walker |  | 1676 | 162 | Code::CodeKey { rung: Decl, file: tea.go, decl: 5, sub: 0, line: 53 } |  |  | 0.585 |
| walker |  | 1692 | 16 | Code::CodeKey { rung: Doc, file: tea.go, decl: 5, sub: 0, line: 53 } |  |  | 0.631 |
| walker |  | 1726 | 34 | Code::CodeKey { rung: Doc, file: tea.go, decl: 4, sub: 0, line: 50 } |  |  | 0.643 |
| walker |  | 1762 | 36 | Code::CodeKey { rung: Doc, file: tea.go, decl: 3, sub: 0, line: 46 } |  |  | 0.658 |
| ns | 1810 |  | 145 | Every exported `*Program` method (complete roster) | 2.2 | 1.4 | 0.636 |
| ns | 2001 |  | 191 | Every `ProgramOption` constructor (complete roster) | 2.3 |  | 0.610 |
| walker |  | 2014 | 252 | Code::CodeKey { rung: Decl, file: tea.go, decl: 11, sub: 0, line: 286 } |  |  | 0.612 |
| walker |  | 2063 | 49 | Code::CodeKey { rung: Doc, file: tea.go, decl: 16, sub: 0, line: 349 } |  |  | 0.612 |
| walker |  | 2121 | 58 | Code::CodeKey { rung: Doc, file: tea.go, decl: 15, sub: 0, line: 336 } |  |  | 0.612 |
| ns | 2130 |  | 129 | Build/test/lint entry points (`Taskfile.yaml`) | 2.4 |  | 0.633 |
| ns | 2436 |  | 306 | Event-loop dispatch table: every internally-handled message type | 2.5 |  | 0.586 |
| walker |  | 2482 | 361 | Code::CodeKey { rung: Decl, file: tea.go, decl: 8, sub: 0, line: 241 } |  |  | 0.586 |
| walker |  | 2554 | 72 | Fs::DirListing { dir: testdata/TestViewModel } |  |  | 0.586 |
| walker |  | 2652 | 98 | Code::CodeKey { rung: Doc, file: tea.go, decl: 6, sub: 0, line: 76 } |  |  | 0.602 |
| ns | 2682 |  | 246 | Event-loop skeleton: translate → filter → dispatch → update → render | 2.6 | 2.5 | 0.567 |
| walker |  | 2759 | 107 | Code::CodeKey { rung: Doc, file: tea.go, decl: 9, sub: 0, line: 279 } |  |  | 0.567 |
| walker |  | 2783 | 24 | Code::CodeKey { rung: Doc, file: focus.go, decl: 2, sub: 0, line: 9 } |  |  | 0.567 |
| walker |  | 2828 | 45 | Code::CodeKey { rung: Names, file: tty_unix.go, decl: 0, sub: 0, line: 0 } |  |  | 0.567 |
| walker |  | 2835 | 7 | Code::CodeKey { rung: Body, file: signals_windows.go, decl: 1, sub: 0, line: 8 } |  |  | 0.567 |
| walker |  | 2881 | 46 | Code::CodeKey { rung: Names, file: tty_windows.go, decl: 0, sub: 0, line: 0 } |  |  | 0.568 |
| ns | 2901 |  | 219 | Program control messages: suspend, resume, interrupt | 2.7 | 1.9 | 0.541 |
| walker |  | 2907 | 26 | Code::CodeKey { rung: Doc, file: focus.go, decl: 1, sub: 0, line: 5 } |  |  | 0.541 |
| walker |  | 2933 | 26 | Code::CodeKey { rung: Body, file: raw.go, decl: 2, sub: 0, line: 33 } |  |  | 0.541 |
| walker |  | 2983 | 50 | Code::CodeKey { rung: Names, file: screen.go, decl: 0, sub: 0, line: 0 } |  |  | 0.541 |
| walker |  | 3005 | 22 | Code::CodeKey { rung: Decl, file: screen.go, decl: 1, sub: 0, line: 9 } |  |  | 0.541 |
| walker |  | 3055 | 50 | Code::CodeKey { rung: Decl, file: screen.go, decl: 4, sub: 0, line: 62 } |  |  | 0.541 |
| walker |  | 3064 | 9 | Code::CodeKey { rung: Body, file: screen.go, decl: 2, sub: 0, line: 20 } |  |  | 0.541 |
| ns | 3100 |  | 199 | `Run()`: how input and the TTY are acquired | 2.8 |  | 0.522 |
| walker |  | 3115 | 51 | Code::CodeKey { rung: Names, file: paste.go, decl: 0, sub: 0, line: 0 } |  |  | 0.523 |
| walker |  | 3127 | 12 | Code::CodeKey { rung: Decl, file: paste.go, decl: 1, sub: 0, line: 5 } |  |  | 0.523 |
| walker |  | 3135 | 8 | Code::CodeKey { rung: Body, file: paste.go, decl: 2, sub: 0, line: 10 } |  |  | 0.523 |
| walker |  | 3147 | 12 | Code::CodeKey { rung: Doc, file: paste.go, decl: 2, sub: 0, line: 10 } |  |  | 0.523 |
| walker |  | 3176 | 29 | Code::CodeKey { rung: Doc, file: paste.go, decl: 4, sub: 0, line: 20 } |  |  | 0.523 |
| walker |  | 3231 | 55 | Code::CodeKey { rung: Names, file: termcap.go, decl: 0, sub: 0, line: 0 } |  |  | 0.523 |
| walker |  | 3243 | 12 | Code::CodeKey { rung: Decl, file: termcap.go, decl: 3, sub: 0, line: 41 } |  |  | 0.523 |
| walker |  | 3251 | 8 | Code::CodeKey { rung: Body, file: termcap.go, decl: 4, sub: 0, line: 46 } |  |  | 0.523 |
| walker |  | 3263 | 12 | Code::CodeKey { rung: Doc, file: termcap.go, decl: 4, sub: 0, line: 46 } |  |  | 0.523 |
| ns | 3288 |  | 188 | `Run()`: renderer choice, colour profile, first three messages | 2.9 | 2.8 | 0.508 |
| walker |  | 3289 | 26 | Code::CodeKey { rung: Body, file: termcap.go, decl: 2, sub: 0, line: 30 } |  |  | 0.508 |
| walker |  | 3344 | 55 | Code::CodeKey { rung: Names, file: xterm.go, decl: 0, sub: 0, line: 0 } |  |  | 0.508 |
| walker |  | 3356 | 12 | Code::CodeKey { rung: Decl, file: xterm.go, decl: 1, sub: 0, line: 4 } |  |  | 0.508 |
| walker |  | 3364 | 8 | Code::CodeKey { rung: Body, file: xterm.go, decl: 2, sub: 0, line: 9 } |  |  | 0.508 |
| walker |  | 3372 | 8 | Code::CodeKey { rung: Body, file: xterm.go, decl: 4, sub: 0, line: 20 } |  |  | 0.508 |
| walker |  | 3384 | 12 | Code::CodeKey { rung: Doc, file: xterm.go, decl: 2, sub: 0, line: 9 } |  |  | 0.508 |
| walker |  | 3401 | 17 | Code::CodeKey { rung: Doc, file: xterm.go, decl: 1, sub: 0, line: 4 } |  |  | 0.508 |
| walker |  | 3432 | 31 | Code::CodeKey { rung: Doc, file: paste.go, decl: 1, sub: 0, line: 5 } |  |  | 0.508 |
| walker |  | 3463 | 31 | Code::CodeKey { rung: Doc, file: paste.go, decl: 3, sub: 0, line: 16 } |  |  | 0.508 |
| ns | 3474 |  | 186 | `Run()`: model init, event loop, graceful shutdown | 2.10 | 2.9 | 0.491 |
| walker |  | 3494 | 31 | Code::CodeKey { rung: Doc, file: raw.go, decl: 1, sub: 0, line: 5 } |  |  | 0.491 |
| walker |  | 3559 | 65 | Markdown::ReadmeHeadline { file: tutorials/basics/README.md } |  |  | 0.491 |
| walker |  | 3621 | 62 | Code::CodeKey { rung: Names, file: environ.go, decl: 0, sub: 0, line: 0 } |  |  | 0.491 |
| walker |  | 3635 | 14 | Code::CodeKey { rung: Body, file: environ.go, decl: 2, sub: 0, line: 24 } |  |  | 0.491 |
| walker |  | 3649 | 14 | Code::CodeKey { rung: Body, file: environ.go, decl: 3, sub: 0, line: 32 } |  |  | 0.491 |
| ns | 3693 |  | 219 | Debug hooks: `TEA_TRACE`, `TEA_DEBUG`, panic recovery | 2.11 |  | 0.481 |
| walker |  | 3715 | 66 | Code::CodeKey { rung: Names, file: logging.go, decl: 0, sub: 0, line: 0 } |  |  | 0.481 |
| walker |  | 3737 | 22 | Code::CodeKey { rung: Decl, file: logging.go, decl: 2, sub: 0, line: 29 } |  |  | 0.481 |
| walker |  | 3753 | 16 | Code::CodeKey { rung: Body, file: logging.go, decl: 1, sub: 0, line: 23 } |  |  | 0.482 |
| walker |  | 3774 | 21 | Code::CodeKey { rung: Doc, file: logging.go, decl: 3, sub: 0, line: 35 } |  |  | 0.482 |
| walker |  | 3803 | 29 | Code::CodeKey { rung: Doc, file: logging.go, decl: 2, sub: 0, line: 29 } |  |  | 0.482 |
| walker |  | 3875 | 72 | Markdown::ReadmeHeadline { file: tutorials/commands/README.md } |  |  | 0.482 |
| ns | 3916 |  | 223 | Frame pacing: the render ticker and the FPS bounds | 2.12 |  | 0.466 |
| walker |  | 3918 | 43 | Code::CodeKey { rung: Doc, file: xterm.go, decl: 4, sub: 0, line: 20 } |  |  | 0.466 |
| walker |  | 3932 | 14 | Code::CodeKey { rung: Doc, file: tty_unix.go, decl: 3, sub: 0, line: 40 } |  |  | 0.466 |
| walker |  | 3980 | 48 | Code::CodeKey { rung: Doc, file: environ.go, decl: 2, sub: 0, line: 24 } |  |  | 0.466 |
| ns | 4118 |  | 202 | Input event message types: keys, mouse, paste, focus (complete) | 3.1 |  | 0.455 |
| ns | 4380 |  | 262 | Terminal report message types: colour, clipboard, size, capability (complete) | 3.2 |  | 0.453 |
| walker |  | 4496 | 516 | Code::CodeKey { rung: Decl, file: tea.go, decl: 7, sub: 0, line: 84 } |  |  | 0.455 |
| walker |  | 4535 | 39 | Code::CodeKey { rung: Doc, file: tea.go, decl: 7, sub: 0, line: 84 } |  |  | 0.463 |
| ns | 4550 |  | 170 | Command constructors, part 1: lifecycle, batching, timing, output | 3.3 | 2.7 | 0.454 |
| ns | 4696 |  | 146 | Command constructors, part 2: terminal queries and clipboard | 3.4 | 3.3 | 0.449 |
| walker |  | 4815 | 280 | Code::CodeKey { rung: Names, file: tea.go, decl: 0, sub: 1, line: 0 } |  |  | 0.457 |
| walker |  | 4828 | 13 | Code::CodeKey { rung: Doc, file: tea.go, decl: 31, sub: 0, line: 595 } |  |  | 0.461 |
| walker |  | 4855 | 27 | Code::CodeKey { rung: Decl, file: tea.go, decl: 20, sub: 0, line: 395 } |  |  | 0.461 |
| walker |  | 4872 | 17 | Code::CodeKey { rung: Doc, file: tea.go, decl: 26, sub: 0, line: 564 } |  |  | 0.462 |
| walker |  | 4891 | 19 | Code::CodeKey { rung: Doc, file: tea.go, decl: 24, sub: 0, line: 555 } |  |  | 0.464 |
| walker |  | 4914 | 23 | Code::CodeKey { rung: Doc, file: tea.go, decl: 30, sub: 0, line: 590 } |  |  | 0.467 |
| ns | 5037 |  | 341 | `input.go`: the ultraviolet-event → `tea.Msg` translation table | 3.5 |  | 0.448 |
| walker |  | 5062 | 148 | Code::CodeKey { rung: Decl, file: tea.go, decl: 17, sub: 0, line: 357 } |  |  | 0.449 |
| walker |  | 5076 | 14 | Code::CodeKey { rung: Doc, file: tea.go, decl: 17, sub: 0, line: 357 } |  |  | 0.449 |
| walker |  | 5100 | 24 | Code::CodeKey { rung: Doc, file: tea.go, decl: 18, sub: 0, line: 374 } |  |  | 0.449 |
| ns | 5113 |  | 76 | `Key` struct fields | 3.6 |  | 0.443 |
| walker |  | 5128 | 28 | Code::CodeKey { rung: Doc, file: tea.go, decl: 28, sub: 0, line: 578 } |  |  | 0.448 |
| walker |  | 5160 | 32 | Code::CodeKey { rung: Doc, file: tea.go, decl: 25, sub: 0, line: 561 } |  |  | 0.452 |
| walker |  | 5172 | 12 | Code::CodeKey { rung: Doc, file: tea.go, decl: 33, sub: 0, line: 685 } |  |  | 0.452 |
| walker |  | 5185 | 13 | Code::CodeKey { rung: Doc, file: tea.go, decl: 22, sub: 0, line: 409 } |  |  | 0.452 |
| walker |  | 5267 | 82 | Code::CodeKey { rung: Doc, file: tea.go, decl: 29, sub: 0, line: 586 } |  |  | 0.460 |
| ns | 5293 |  | 180 | Key message methods: `String`, `Keystroke`, `Key` | 3.7 | 3.6 | 0.454 |
| walker |  | 5352 | 85 | Code::CodeKey { rung: Doc, file: tea.go, decl: 27, sub: 0, line: 574 } |  |  | 0.464 |
| walker |  | 5381 | 29 | Code::CodeKey { rung: Doc, file: tea.go, decl: 21, sub: 0, line: 402 } |  |  | 0.464 |
| walker |  | 5411 | 30 | Code::CodeKey { rung: Doc, file: tea.go, decl: 34, sub: 0, line: 700 } |  |  | 0.464 |
| walker |  | 5524 | 113 | Code::CodeKey { rung: Doc, file: tea.go, decl: 19, sub: 0, line: 390 } |  |  | 0.500 |
| walker |  | 5561 | 37 | Code::CodeKey { rung: Doc, file: tea.go, decl: 35, sub: 0, line: 743 } |  |  | 0.500 |
| walker |  | 5602 | 41 | Code::CodeKey { rung: Doc, file: tea.go, decl: 20, sub: 0, line: 395 } |  |  | 0.500 |
| ns | 5614 |  | 321 | `key.go` key-code catalog: group headings plus the C0/G0 names | 3.8 |  | 0.480 |
| walker |  | 5699 | 97 | Code::CodeKey { rung: Names, file: cursor.go, decl: 0, sub: 0, line: 0 } |  |  | 0.484 |
| walker |  | 5710 | 11 | Code::CodeKey { rung: Decl, file: cursor.go, decl: 4, sub: 0, line: 15 } |  |  | 0.484 |
| walker |  | 5724 | 14 | Code::CodeKey { rung: Decl, file: cursor.go, decl: 2, sub: 0, line: 7 } |  |  | 0.484 |
| walker |  | 5730 | 6 | Code::CodeKey { rung: Doc, file: cursor.go, decl: 4, sub: 0, line: 15 } |  |  | 0.484 |
| walker |  | 5740 | 10 | Code::CodeKey { rung: Body, file: cursor.go, decl: 6, sub: 0, line: 26 } |  |  | 0.484 |
| walker |  | 5751 | 11 | Code::CodeKey { rung: Doc, file: cursor.go, decl: 3, sub: 0, line: 12 } |  |  | 0.485 |
| walker |  | 5764 | 13 | Code::CodeKey { rung: Doc, file: cursor.go, decl: 1, sub: 0, line: 4 } |  |  | 0.485 |
| walker |  | 5780 | 16 | Code::CodeKey { rung: Doc, file: cursor.go, decl: 2, sub: 0, line: 7 } |  |  | 0.485 |
| ns | 5797 |  | 183 | Modifier keys: the complete `KeyMod` constant set | 3.9 |  | 0.476 |
| walker |  | 5815 | 35 | Code::CodeKey { rung: Doc, file: cursor.go, decl: 6, sub: 0, line: 26 } |  |  | 0.476 |
| ns | 6023 |  | 226 | `Mouse` struct and the complete mouse-button constant set | 3.10 | 3.1 | 0.466 |
| walker |  | 6065 | 250 | Fs::DirListing { dir: examples } |  |  | 0.470 |
| walker |  | 6081 | 16 | Code::CodeKey { rung: Doc, file: cursor.go, decl: 5, sub: 0, line: 22 } |  |  | 0.470 |
| walker |  | 6180 | 99 | Code::CodeKey { rung: Names, file: renderer.go, decl: 0, sub: 0, line: 0 } |  |  | 0.473 |
| walker |  | 6193 | 13 | Code::CodeKey { rung: Decl, file: renderer.go, decl: 3, sub: 0, line: 59 } |  |  | 0.473 |
| ns | 6200 |  | 177 | `MouseMode` enum: none / cell-motion / all-motion | 3.11 | 1.7 | 0.488 |
| walker |  | 6232 | 39 | Code::CodeKey { rung: Decl, file: renderer.go, decl: 1, sub: 0, line: 10 } |  |  | 0.491 |
| walker |  | 6278 | 46 | Code::CodeKey { rung: Body, file: renderer.go, decl: 4, sub: 0, line: 70 } |  |  | 0.491 |
| walker |  | 6325 | 47 | Code::CodeKey { rung: Body, file: renderer.go, decl: 5, sub: 0, line: 86 } |  |  | 0.491 |
| ns | 6378 |  | 178 | Cursor value types: `Cursor`, `NewCursor`, `CursorShape` | 3.12 | 1.7 | 0.507 |
| ns | 6566 |  | 188 | Keyboard enhancements: request fields and response predicates | 3.13 | 3.2 | 0.503 |
| ns | 6759 |  | 193 | `Batch` vs `Sequence` semantics and where they execute | 3.14 | 3.3 | 0.497 |
| walker |  | 6915 | 590 | Code::CodeKey { rung: Decl, file: tea.go, decl: 23, sub: 0, line: 426 } |  |  | 0.497 |
| walker |  | 6927 | 12 | Code::CodeKey { rung: Doc, file: tea.go, decl: 23, sub: 0, line: 426 } |  |  | 0.497 |
| ns | 6959 |  | 200 | `Exec` / `ExecProcess` and the `ExecCommand` interface | 3.15 | 3.3 | 0.490 |
| ns | 7111 |  | 152 | Logging to a file: `LogToFile` / `LogToFileWith` | 3.16 |  | 0.492 |
| walker |  | 7143 | 216 | Code::CodeKey { rung: Names, file: cursed_renderer.go, decl: 0, sub: 0, line: 0 } |  |  | 0.492 |
| walker |  | 7152 | 9 | Code::CodeKey { rung: Doc, file: cursed_renderer.go, decl: 6, sub: 0, line: 75 } |  |  | 0.492 |
| walker |  | 7161 | 9 | Code::CodeKey { rung: Doc, file: cursed_renderer.go, decl: 7, sub: 0, line: 143 } |  |  | 0.492 |
| walker |  | 7170 | 9 | Code::CodeKey { rung: Doc, file: cursed_renderer.go, decl: 9, sub: 0, line: 257 } |  |  | 0.492 |
| walker |  | 7179 | 9 | Code::CodeKey { rung: Doc, file: cursed_renderer.go, decl: 10, sub: 0, line: 579 } |  |  | 0.492 |
| walker |  | 7189 | 10 | Code::CodeKey { rung: Doc, file: cursed_renderer.go, decl: 8, sub: 0, line: 249 } |  |  | 0.492 |
| walker |  | 7203 | 14 | Code::CodeKey { rung: Doc, file: cursed_renderer.go, decl: 4, sub: 0, line: 52 } |  |  | 0.492 |
| walker |  | 7218 | 15 | Code::CodeKey { rung: Doc, file: cursed_renderer.go, decl: 5, sub: 0, line: 59 } |  |  | 0.492 |
| walker |  | 7236 | 18 | Code::CodeKey { rung: Doc, file: input.go, decl: 1, sub: 0, line: 8 } |  |  | 0.493 |
| ns | 7328 |  | 217 | The `renderer` interface: complete method set, and its two implementations | 4.1 |  | 0.484 |
| walker |  | 7482 | 246 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.485 |
| ns | 7681 |  | 353 | `cursedRenderer`: every function in the 860-line renderer | 4.2 | 4.1 | 0.479 |
| walker |  | 7733 | 251 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: true } |  |  | 0.491 |
| walker |  | 7853 | 120 | Code::CodeKey { rung: Names, file: keyboard.go, decl: 0, sub: 0, line: 0 } |  |  | 0.506 |
| walker |  | 7864 | 11 | Code::CodeKey { rung: Body, file: keyboard.go, decl: 2, sub: 0, line: 33 } |  |  | 0.506 |
| walker |  | 7882 | 18 | Code::CodeKey { rung: Body, file: keyboard.go, decl: 3, sub: 0, line: 39 } |  |  | 0.506 |
| walker |  | 7900 | 18 | Code::CodeKey { rung: Body, file: keyboard.go, decl: 4, sub: 0, line: 45 } |  |  | 0.506 |
| walker |  | 7918 | 18 | Code::CodeKey { rung: Body, file: keyboard.go, decl: 6, sub: 0, line: 57 } |  |  | 0.506 |
| walker |  | 7942 | 24 | Code::CodeKey { rung: Doc, file: keyboard.go, decl: 4, sub: 0, line: 45 } |  |  | 0.506 |
| walker |  | 7968 | 26 | Code::CodeKey { rung: Doc, file: keyboard.go, decl: 6, sub: 0, line: 57 } |  |  | 0.506 |
| walker |  | 7997 | 29 | Code::CodeKey { rung: Doc, file: keyboard.go, decl: 5, sub: 0, line: 51 } |  |  | 0.506 |
| walker |  | 8030 | 33 | Code::CodeKey { rung: Doc, file: keyboard.go, decl: 3, sub: 0, line: 39 } |  |  | 0.506 |
| ns | 8039 |  | 358 | `View` fields → ANSI sequences, in `cursedRenderer.start` | 4.3 | 4.2 | 0.495 |
| walker |  | 8065 | 35 | Code::CodeKey { rung: Doc, file: keyboard.go, decl: 2, sub: 0, line: 33 } |  |  | 0.495 |
| walker |  | 8295 | 230 | Code::CodeKey { rung: Decl, file: keyboard.go, decl: 1, sub: 0, line: 9 } |  |  | 0.498 |
| walker |  | 8323 | 28 | Code::CodeKey { rung: Doc, file: keyboard.go, decl: 1, sub: 0, line: 9 } |  |  | 0.498 |
| walker |  | 8394 | 71 | Code::CodeKey { rung: Doc, file: screen.go, decl: 1, sub: 0, line: 9 } |  |  | 0.498 |
| ns | 8420 |  | 381 | Platform matrix: build tags and per-OS terminal functions | 4.4 |  | 0.495 |
| walker |  | 8467 | 73 | Code::CodeKey { rung: Doc, file: environ.go, decl: 3, sub: 0, line: 32 } |  |  | 0.495 |
| ns | 8618 |  | 198 | `tty.go`: terminal acquisition, input loop, resize detection | 4.5 |  | 0.491 |
| walker |  | 8709 | 242 | Code::CodeKey { rung: Names, file: tea.go, decl: 0, sub: 2, line: 0 } |  |  | 0.500 |
| walker |  | 8727 | 18 | Code::CodeKey { rung: Doc, file: tea.go, decl: 44, sub: 0, line: 1209 } |  |  | 0.500 |
| walker |  | 8741 | 14 | Code::CodeKey { rung: Doc, file: tea.go, decl: 36, sub: 0, line: 886 } |  |  | 0.500 |
| walker |  | 8794 | 53 | Code::CodeKey { rung: Doc, file: tea.go, decl: 40, sub: 0, line: 991 } |  |  | 0.500 |
| walker |  | 8810 | 16 | Code::CodeKey { rung: Doc, file: tea.go, decl: 45, sub: 0, line: 1214 } |  |  | 0.500 |
| walker |  | 8867 | 57 | Code::CodeKey { rung: Doc, file: tea.go, decl: 43, sub: 0, line: 1204 } |  |  | 0.500 |
| ns | 8882 |  | 264 | `examples/` and `tutorials/` directory listings | 5.1 |  | 0.542 |
| walker |  | 8884 | 17 | Code::CodeKey { rung: Doc, file: tea.go, decl: 46, sub: 0, line: 1221 } |  |  | 0.542 |
| walker |  | 8913 | 29 | Code::CodeKey { rung: Doc, file: tea.go, decl: 47, sub: 0, line: 1241 } |  |  | 0.542 |
| walker |  | 9021 | 108 | Code::CodeKey { rung: Doc, file: tea.go, decl: 41, sub: 0, line: 1183 } |  |  | 0.542 |
| ns | 9062 |  | 180 | `UPGRADE_GUIDE_V2.md`: complete section map | 5.2 |  | 0.536 |
| walker |  | 9137 | 116 | Code::CodeKey { rung: Doc, file: tea.go, decl: 42, sub: 0, line: 1197 } |  |  | 0.536 |
| walker |  | 9174 | 37 | Code::CodeKey { rung: Doc, file: tea.go, decl: 48, sub: 0, line: 1269 } |  |  | 0.536 |
| walker |  | 9216 | 42 | Markdown::Section { file: README.md, section_index: 16, keeps_default_concavity: false } |  |  | 0.536 |
| walker |  | 9238 | 22 | Code::CodeKey { rung: Body, file: termios_windows.go, decl: 1, sub: 0, line: 8 } |  |  | 0.538 |
| ns | 9347 |  | 285 | The v1→v2 migration checklist | 5.3 | 5.2 | 0.533 |
| walker |  | 9383 | 145 | Code::CodeKey { rung: Names, file: commands.go, decl: 0, sub: 0, line: 0 } |  |  | 0.542 |
| walker |  | 9392 | 9 | Code::CodeKey { rung: Body, file: commands.go, decl: 9, sub: 0, line: 173 } |  |  | 0.542 |
| walker |  | 9407 | 15 | Code::CodeKey { rung: Body, file: commands.go, decl: 1, sub: 0, line: 15 } |  |  | 0.542 |
| walker |  | 9422 | 15 | Code::CodeKey { rung: Body, file: commands.go, decl: 3, sub: 0, line: 25 } |  |  | 0.542 |
| walker |  | 9454 | 32 | Code::CodeKey { rung: Doc, file: commands.go, decl: 3, sub: 0, line: 25 } |  |  | 0.544 |
| walker |  | 9492 | 38 | Code::CodeKey { rung: Doc, file: commands.go, decl: 2, sub: 0, line: 21 } |  |  | 0.544 |
| walker |  | 9510 | 18 | Code::CodeKey { rung: Doc, file: commands.go, decl: 4, sub: 0, line: 30 } |  |  | 0.545 |
| ns | 9516 |  | 169 | README section map | 5.4 |  | 0.553 |
| walker |  | 9766 | 256 | Code::CodeKey { rung: Decl, file: tea.go, decl: 7, sub: 1, line: 84 } |  |  | 0.561 |
| walker |  | 9789 | 23 | Code::CodeKey { rung: Body, file: termios_bsd.go, decl: 1, sub: 0, line: 11 } |  |  | 0.562 |
| ns | 9876 |  | 360 | Test suite: every test function, and the golden-file fixtures | 5.5 |  | 0.551 |
| ns | 9917 |  | 41 | CI workflow inventory and golden-test fixture directories | 5.6 |  | 0.555 |
| walker |  | 9938 | 149 | Code::CodeKey { rung: Names, file: clipboard.go, decl: 0, sub: 0, line: 0 } |  |  | 0.561 |
| walker |  | 9959 | 21 | Code::CodeKey { rung: Decl, file: clipboard.go, decl: 1, sub: 0, line: 5 } |  |  | 0.563 |
| walker |  | 9967 | 8 | Code::CodeKey { rung: Body, file: clipboard.go, decl: 2, sub: 0, line: 15 } |  |  | 0.563 |
| walker |  | 9975 | 8 | Code::CodeKey { rung: Body, file: clipboard.go, decl: 3, sub: 0, line: 20 } |  |  | 0.563 |
| walker |  | 9984 | 9 | Code::CodeKey { rung: Body, file: clipboard.go, decl: 7, sub: 0, line: 42 } |  |  | 0.563 |
| walker |  | 9999 | 15 | Code::CodeKey { rung: Doc, file: clipboard.go, decl: 3, sub: 0, line: 20 } |  |  | 0.563 |
