Score(3000)=0.541 I=0.764 C=0.383 ns_rows≤3K=18/50 grid(1000/1442/2080/3000/4327/6240/9000)=0.584/0.532/0.612/0.541/0.505/0.492/0.546

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
| ns | 3693 |  | 219 | Debug hooks: `TEA_TRACE`, `TEA_DEBUG`, panic recovery | 2.11 |  | 0.481 |
| walker |  | 3774 | 280 | Code::CodeKey { rung: Names, file: tea.go, decl: 0, sub: 1, line: 0 } |  |  | 0.490 |
| walker |  | 3787 | 13 | Code::CodeKey { rung: Doc, file: tea.go, decl: 31, sub: 0, line: 595 } |  |  | 0.493 |
| walker |  | 3814 | 27 | Code::CodeKey { rung: Decl, file: tea.go, decl: 20, sub: 0, line: 395 } |  |  | 0.493 |
| walker |  | 3831 | 17 | Code::CodeKey { rung: Doc, file: tea.go, decl: 26, sub: 0, line: 564 } |  |  | 0.496 |
| walker |  | 3850 | 19 | Code::CodeKey { rung: Doc, file: tea.go, decl: 24, sub: 0, line: 555 } |  |  | 0.497 |
| walker |  | 3873 | 23 | Code::CodeKey { rung: Doc, file: tea.go, decl: 30, sub: 0, line: 590 } |  |  | 0.501 |
| ns | 3916 |  | 223 | Frame pacing: the render ticker and the FPS bounds | 2.12 |  | 0.485 |
| walker |  | 4021 | 148 | Code::CodeKey { rung: Decl, file: tea.go, decl: 17, sub: 0, line: 357 } |  |  | 0.485 |
| walker |  | 4035 | 14 | Code::CodeKey { rung: Doc, file: tea.go, decl: 17, sub: 0, line: 357 } |  |  | 0.485 |
| walker |  | 4059 | 24 | Code::CodeKey { rung: Doc, file: tea.go, decl: 18, sub: 0, line: 374 } |  |  | 0.485 |
| walker |  | 4087 | 28 | Code::CodeKey { rung: Doc, file: tea.go, decl: 28, sub: 0, line: 578 } |  |  | 0.491 |
| ns | 4118 |  | 202 | Input event message types: keys, mouse, paste, focus (complete) | 3.1 |  | 0.479 |
| walker |  | 4119 | 32 | Code::CodeKey { rung: Doc, file: tea.go, decl: 25, sub: 0, line: 561 } |  |  | 0.483 |
| walker |  | 4131 | 12 | Code::CodeKey { rung: Doc, file: tea.go, decl: 33, sub: 0, line: 685 } |  |  | 0.483 |
| walker |  | 4144 | 13 | Code::CodeKey { rung: Doc, file: tea.go, decl: 22, sub: 0, line: 409 } |  |  | 0.483 |
| walker |  | 4226 | 82 | Code::CodeKey { rung: Doc, file: tea.go, decl: 29, sub: 0, line: 586 } |  |  | 0.493 |
| walker |  | 4311 | 85 | Code::CodeKey { rung: Doc, file: tea.go, decl: 27, sub: 0, line: 574 } |  |  | 0.505 |
| walker |  | 4340 | 29 | Code::CodeKey { rung: Doc, file: tea.go, decl: 21, sub: 0, line: 402 } |  |  | 0.505 |
| walker |  | 4370 | 30 | Code::CodeKey { rung: Doc, file: tea.go, decl: 34, sub: 0, line: 700 } |  |  | 0.505 |
| ns | 4380 |  | 262 | Terminal report message types: colour, clipboard, size, capability (complete) | 3.2 |  | 0.499 |
| walker |  | 4483 | 113 | Code::CodeKey { rung: Doc, file: tea.go, decl: 19, sub: 0, line: 390 } |  |  | 0.538 |
| walker |  | 4520 | 37 | Code::CodeKey { rung: Doc, file: tea.go, decl: 35, sub: 0, line: 743 } |  |  | 0.538 |
| ns | 4550 |  | 170 | Command constructors, part 1: lifecycle, batching, timing, output | 3.3 | 2.7 | 0.529 |
| ns | 4696 |  | 146 | Command constructors, part 2: terminal queries and clipboard | 3.4 | 3.3 | 0.522 |
| walker |  | 4762 | 242 | Code::CodeKey { rung: Names, file: tea.go, decl: 0, sub: 2, line: 0 } |  |  | 0.534 |
| walker |  | 4780 | 18 | Code::CodeKey { rung: Doc, file: tea.go, decl: 44, sub: 0, line: 1209 } |  |  | 0.534 |
| walker |  | 4794 | 14 | Code::CodeKey { rung: Doc, file: tea.go, decl: 36, sub: 0, line: 886 } |  |  | 0.534 |
| walker |  | 4847 | 53 | Code::CodeKey { rung: Doc, file: tea.go, decl: 40, sub: 0, line: 991 } |  |  | 0.534 |
| walker |  | 4863 | 16 | Code::CodeKey { rung: Doc, file: tea.go, decl: 45, sub: 0, line: 1214 } |  |  | 0.534 |
| walker |  | 4920 | 57 | Code::CodeKey { rung: Doc, file: tea.go, decl: 43, sub: 0, line: 1204 } |  |  | 0.534 |
| walker |  | 4937 | 17 | Code::CodeKey { rung: Doc, file: tea.go, decl: 46, sub: 0, line: 1221 } |  |  | 0.534 |
| walker |  | 4966 | 29 | Code::CodeKey { rung: Doc, file: tea.go, decl: 47, sub: 0, line: 1241 } |  |  | 0.534 |
| ns | 5037 |  | 341 | `input.go`: the ultraviolet-event → `tea.Msg` translation table | 3.5 |  | 0.513 |
| walker |  | 5074 | 108 | Code::CodeKey { rung: Doc, file: tea.go, decl: 41, sub: 0, line: 1183 } |  |  | 0.513 |
| ns | 5113 |  | 76 | `Key` struct fields | 3.6 |  | 0.507 |
| walker |  | 5190 | 116 | Code::CodeKey { rung: Doc, file: tea.go, decl: 42, sub: 0, line: 1197 } |  |  | 0.507 |
| walker |  | 5227 | 37 | Code::CodeKey { rung: Doc, file: tea.go, decl: 48, sub: 0, line: 1269 } |  |  | 0.507 |
| ns | 5293 |  | 180 | Key message methods: `String`, `Keystroke`, `Key` | 3.7 | 3.6 | 0.500 |
| walker |  | 5385 | 158 | Code::CodeKey { rung: Names, file: tea.go, decl: 0, sub: 3, line: 0 } |  |  | 0.514 |
| walker |  | 5423 | 38 | Code::CodeKey { rung: Doc, file: tea.go, decl: 50, sub: 0, line: 1319 } |  |  | 0.514 |
| walker |  | 5435 | 12 | Code::CodeKey { rung: Doc, file: tea.go, decl: 55, sub: 0, line: 1393 } |  |  | 0.514 |
| walker |  | 5494 | 59 | Code::CodeKey { rung: Doc, file: tea.go, decl: 52, sub: 0, line: 1344 } |  |  | 0.514 |
| walker |  | 5555 | 61 | Code::CodeKey { rung: Doc, file: tea.go, decl: 53, sub: 0, line: 1372 } |  |  | 0.514 |
| ns | 5614 |  | 321 | `key.go` key-code catalog: group headings plus the C0/G0 names | 3.8 |  | 0.494 |
| walker |  | 5675 | 120 | Code::CodeKey { rung: Doc, file: tea.go, decl: 54, sub: 0, line: 1386 } |  |  | 0.494 |
| walker |  | 5716 | 41 | Code::CodeKey { rung: Doc, file: tea.go, decl: 20, sub: 0, line: 395 } |  |  | 0.494 |
| walker |  | 5758 | 42 | Code::CodeKey { rung: Doc, file: tea.go, decl: 56, sub: 0, line: 1427 } |  |  | 0.494 |
| ns | 5797 |  | 183 | Modifier keys: the complete `KeyMod` constant set | 3.9 |  | 0.485 |
| walker |  | 5823 | 65 | Markdown::ReadmeHeadline { file: tutorials/basics/README.md } |  |  | 0.485 |
| walker |  | 5885 | 62 | Code::CodeKey { rung: Names, file: environ.go, decl: 0, sub: 0, line: 0 } |  |  | 0.487 |
| walker |  | 5899 | 14 | Code::CodeKey { rung: Body, file: environ.go, decl: 2, sub: 0, line: 24 } |  |  | 0.487 |
| walker |  | 5913 | 14 | Code::CodeKey { rung: Body, file: environ.go, decl: 3, sub: 0, line: 32 } |  |  | 0.487 |
| walker |  | 5959 | 46 | Code::CodeKey { rung: Doc, file: tea.go, decl: 49, sub: 0, line: 1294 } |  |  | 0.487 |
| ns | 6023 |  | 226 | `Mouse` struct and the complete mouse-button constant set | 3.10 | 3.1 | 0.477 |
| walker |  | 6025 | 66 | Code::CodeKey { rung: Names, file: logging.go, decl: 0, sub: 0, line: 0 } |  |  | 0.477 |
| walker |  | 6047 | 22 | Code::CodeKey { rung: Decl, file: logging.go, decl: 2, sub: 0, line: 29 } |  |  | 0.477 |
| walker |  | 6063 | 16 | Code::CodeKey { rung: Body, file: logging.go, decl: 1, sub: 0, line: 23 } |  |  | 0.477 |
| walker |  | 6084 | 21 | Code::CodeKey { rung: Doc, file: logging.go, decl: 3, sub: 0, line: 35 } |  |  | 0.477 |
| walker |  | 6113 | 29 | Code::CodeKey { rung: Doc, file: logging.go, decl: 2, sub: 0, line: 29 } |  |  | 0.477 |
| walker |  | 6185 | 72 | Markdown::ReadmeHeadline { file: tutorials/commands/README.md } |  |  | 0.477 |
| ns | 6200 |  | 177 | `MouseMode` enum: none / cell-motion / all-motion | 3.11 | 1.7 | 0.492 |
| walker |  | 6228 | 43 | Code::CodeKey { rung: Doc, file: xterm.go, decl: 4, sub: 0, line: 20 } |  |  | 0.492 |
| walker |  | 6242 | 14 | Code::CodeKey { rung: Doc, file: tty_unix.go, decl: 3, sub: 0, line: 40 } |  |  | 0.492 |
| walker |  | 6290 | 48 | Code::CodeKey { rung: Doc, file: environ.go, decl: 2, sub: 0, line: 24 } |  |  | 0.492 |
| ns | 6378 |  | 178 | Cursor value types: `Cursor`, `NewCursor`, `CursorShape` | 3.12 | 1.7 | 0.488 |
| ns | 6566 |  | 188 | Keyboard enhancements: request fields and response predicates | 3.13 | 3.2 | 0.484 |
| ns | 6759 |  | 193 | `Batch` vs `Sequence` semantics and where they execute | 3.14 | 3.3 | 0.479 |
| walker |  | 6806 | 516 | Code::CodeKey { rung: Decl, file: tea.go, decl: 7, sub: 0, line: 84 } |  |  | 0.481 |
| walker |  | 6845 | 39 | Code::CodeKey { rung: Doc, file: tea.go, decl: 7, sub: 0, line: 84 } |  |  | 0.486 |
| ns | 6959 |  | 200 | `Exec` / `ExecProcess` and the `ExecCommand` interface | 3.15 | 3.3 | 0.480 |
| walker |  | 7101 | 256 | Code::CodeKey { rung: Decl, file: tea.go, decl: 7, sub: 1, line: 84 } |  |  | 0.490 |
| ns | 7111 |  | 152 | Logging to a file: `LogToFile` / `LogToFileWith` | 3.16 |  | 0.492 |
| ns | 7328 |  | 217 | The `renderer` interface: complete method set, and its two implementations | 4.1 |  | 0.483 |
| walker |  | 7380 | 279 | Code::CodeKey { rung: Decl, file: tea.go, decl: 7, sub: 2, line: 84 } |  |  | 0.488 |
| walker |  | 7624 | 244 | Code::CodeKey { rung: Decl, file: tea.go, decl: 7, sub: 3, line: 84 } |  |  | 0.500 |
| ns | 7681 |  | 353 | `cursedRenderer`: every function in the 860-line renderer | 4.2 | 4.1 | 0.490 |
| walker |  | 7721 | 97 | Code::CodeKey { rung: Names, file: cursor.go, decl: 0, sub: 0, line: 0 } |  |  | 0.501 |
| walker |  | 7732 | 11 | Code::CodeKey { rung: Decl, file: cursor.go, decl: 4, sub: 0, line: 15 } |  |  | 0.504 |
| walker |  | 7746 | 14 | Code::CodeKey { rung: Decl, file: cursor.go, decl: 2, sub: 0, line: 7 } |  |  | 0.504 |
| walker |  | 7752 | 6 | Code::CodeKey { rung: Doc, file: cursor.go, decl: 4, sub: 0, line: 15 } |  |  | 0.506 |
| walker |  | 7762 | 10 | Code::CodeKey { rung: Body, file: cursor.go, decl: 6, sub: 0, line: 26 } |  |  | 0.506 |
| walker |  | 7773 | 11 | Code::CodeKey { rung: Doc, file: cursor.go, decl: 3, sub: 0, line: 12 } |  |  | 0.510 |
| walker |  | 7786 | 13 | Code::CodeKey { rung: Doc, file: cursor.go, decl: 1, sub: 0, line: 4 } |  |  | 0.510 |
| walker |  | 7802 | 16 | Code::CodeKey { rung: Doc, file: cursor.go, decl: 2, sub: 0, line: 7 } |  |  | 0.510 |
| walker |  | 7837 | 35 | Code::CodeKey { rung: Doc, file: cursor.go, decl: 6, sub: 0, line: 26 } |  |  | 0.510 |
| ns | 8039 |  | 358 | `View` fields → ANSI sequences, in `cursedRenderer.start` | 4.3 | 4.2 | 0.499 |
| walker |  | 8087 | 250 | Fs::DirListing { dir: examples } |  |  | 0.503 |
| walker |  | 8103 | 16 | Code::CodeKey { rung: Doc, file: cursor.go, decl: 5, sub: 0, line: 22 } |  |  | 0.503 |
| walker |  | 8202 | 99 | Code::CodeKey { rung: Names, file: renderer.go, decl: 0, sub: 0, line: 0 } |  |  | 0.506 |
| walker |  | 8215 | 13 | Code::CodeKey { rung: Decl, file: renderer.go, decl: 3, sub: 0, line: 59 } |  |  | 0.506 |
| walker |  | 8254 | 39 | Code::CodeKey { rung: Decl, file: renderer.go, decl: 1, sub: 0, line: 10 } |  |  | 0.509 |
| walker |  | 8300 | 46 | Code::CodeKey { rung: Body, file: renderer.go, decl: 4, sub: 0, line: 70 } |  |  | 0.509 |
| walker |  | 8347 | 47 | Code::CodeKey { rung: Body, file: renderer.go, decl: 5, sub: 0, line: 86 } |  |  | 0.509 |
| ns | 8420 |  | 381 | Platform matrix: build tags and per-OS terminal functions | 4.4 |  | 0.507 |
| ns | 8618 |  | 198 | `tty.go`: terminal acquisition, input loop, resize detection | 4.5 |  | 0.502 |
| ns | 8882 |  | 264 | `examples/` and `tutorials/` directory listings | 5.1 |  | 0.546 |
| walker |  | 8937 | 590 | Code::CodeKey { rung: Decl, file: tea.go, decl: 23, sub: 0, line: 426 } |  |  | 0.546 |
| walker |  | 8949 | 12 | Code::CodeKey { rung: Doc, file: tea.go, decl: 23, sub: 0, line: 426 } |  |  | 0.546 |
| ns | 9062 |  | 180 | `UPGRADE_GUIDE_V2.md`: complete section map | 5.2 |  | 0.539 |
| walker |  | 9182 | 233 | Code::CodeKey { rung: Decl, file: tea.go, decl: 23, sub: 1, line: 426 } |  |  | 0.539 |
| ns | 9347 |  | 285 | The v1→v2 migration checklist | 5.3 | 5.2 | 0.535 |
| walker |  | 9454 | 272 | Code::CodeKey { rung: Decl, file: tea.go, decl: 23, sub: 2, line: 426 } |  |  | 0.535 |
| ns | 9516 |  | 169 | README section map | 5.4 |  | 0.529 |
| walker |  | 9670 | 216 | Code::CodeKey { rung: Names, file: cursed_renderer.go, decl: 0, sub: 0, line: 0 } |  |  | 0.532 |
| walker |  | 9679 | 9 | Code::CodeKey { rung: Doc, file: cursed_renderer.go, decl: 6, sub: 0, line: 75 } |  |  | 0.532 |
| walker |  | 9688 | 9 | Code::CodeKey { rung: Doc, file: cursed_renderer.go, decl: 7, sub: 0, line: 143 } |  |  | 0.532 |
| walker |  | 9697 | 9 | Code::CodeKey { rung: Doc, file: cursed_renderer.go, decl: 9, sub: 0, line: 257 } |  |  | 0.532 |
| walker |  | 9706 | 9 | Code::CodeKey { rung: Doc, file: cursed_renderer.go, decl: 10, sub: 0, line: 579 } |  |  | 0.532 |
| walker |  | 9716 | 10 | Code::CodeKey { rung: Doc, file: cursed_renderer.go, decl: 8, sub: 0, line: 249 } |  |  | 0.532 |
| walker |  | 9730 | 14 | Code::CodeKey { rung: Doc, file: cursed_renderer.go, decl: 4, sub: 0, line: 52 } |  |  | 0.532 |
| walker |  | 9745 | 15 | Code::CodeKey { rung: Doc, file: cursed_renderer.go, decl: 5, sub: 0, line: 59 } |  |  | 0.532 |
| ns | 9876 |  | 360 | Test suite: every test function, and the golden-file fixtures | 5.5 |  | 0.521 |
| ns | 9917 |  | 41 | CI workflow inventory and golden-test fixture directories | 5.6 |  | 0.526 |
