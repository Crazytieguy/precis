Score(3000)=0.466 I=0.666 C=0.326 ns_rows≤3K=18/50 grid(1000/1442/2080/3000/4327/6240/9000)=0.339/0.308/0.350/0.466/0.400/0.464/0.540

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| ns | 95 |  | 95 | Package identity: `tea` package doc lede + module path | 1.1 |  | 0.000 |
| ns | 163 |  | 68 | `Model` interface skeleton: `Init` / `Update` / `View` | 1.2 |  | 0.000 |
| walker |  | 202 | 202 | listing of '.' |  |  | 0.000 |
| walker |  | 212 | 10 | listing of 'testdata' |  |  | 0.000 |
| walker |  | 226 | 14 | listing of 'tutorials' |  |  | 0.000 |
| walker |  | 234 | 8 | listing of 'tutorials/basics' |  |  | 0.000 |
| walker |  | 242 | 8 | listing of 'tutorials/commands' |  |  | 0.000 |
| walker |  | 256 | 14 | go names profile.go |  |  | 0.000 |
| ns | 266 |  | 103 | `Msg` alias and the `Cmd` type | 1.3 |  | 0.000 |
| walker |  | 267 | 11 | go decl profile.go:13 |  |  | 0.000 |
| walker |  | 287 | 20 | go names termios_other.go |  |  | 0.000 |
| walker |  | 309 | 22 | go names focus.go |  |  | 0.000 |
| walker |  | 331 | 22 | go names signals_unix.go |  |  | 0.000 |
| ns | 338 |  | 72 | Program entry points: `NewProgram`, `Run`, `ProgramOption` | 1.4 |  | 0.000 |
| walker |  | 353 | 22 | go names signals_windows.go |  |  | 0.000 |
| walker |  | 376 | 23 | go names input.go |  |  | 0.000 |
| walker |  | 400 | 24 | go names termios_windows.go |  |  | 0.000 |
| ns | 450 |  | 112 | `Model` method doc comments (refines 1.2) | 1.5 | 1.2 | 0.000 |
| walker |  | 529 | 129 | plaintext config Taskfile.yaml |  |  | 0.000 |
| walker |  | 558 | 29 | go module identity in go.mod |  |  | 0.033 |
| walker |  | 583 | 25 | go names termios_bsd.go |  |  | 0.033 |
| walker |  | 608 | 25 | go names termios_unix.go |  |  | 0.033 |
| walker |  | 634 | 26 | go names raw.go |  |  | 0.033 |
| walker |  | 647 | 13 | go decl raw.go:5 |  |  | 0.033 |
| ns | 652 |  | 202 | Complete root directory listing | 1.6 |  | 0.411 |
| walker |  | 669 | 22 | listing of 'testdata/TestClearMsg' |  |  | 0.411 |
| walker |  | 714 | 45 | go names tty_unix.go |  |  | 0.412 |
| walker |  | 760 | 46 | go names tty_windows.go |  |  | 0.412 |
| walker |  | 810 | 50 | go names screen.go |  |  | 0.413 |
| walker |  | 832 | 22 | go decl screen.go:9 |  |  | 0.413 |
| ns | 851 |  | 199 | `View` struct: header plus every field name | 1.7 |  | 0.360 |
| walker |  | 883 | 51 | go names paste.go |  |  | 0.361 |
| walker |  | 895 | 12 | go decl paste.go:5 |  |  | 0.361 |
| walker |  | 903 | 8 | go body paste.go:10 |  |  | 0.361 |
| walker |  | 953 | 50 | go decl screen.go:62 |  |  | 0.361 |
| ns | 975 |  | 124 | `NewView` / `View.SetContent` constructors | 1.8 | 1.7 | 0.339 |
| walker |  | 1029 | 76 | README headline in README.md |  |  | 0.340 |
| walker |  | 1084 | 55 | go names termcap.go |  |  | 0.341 |
| walker |  | 1096 | 12 | go decl termcap.go:41 |  |  | 0.341 |
| walker |  | 1104 | 8 | go body termcap.go:46 |  |  | 0.341 |
| ns | 1132 |  | 157 | `Cmd` semantics and the `Quit` command | 1.9 | 1.3 | 0.320 |
| walker |  | 1159 | 55 | go names xterm.go |  |  | 0.320 |
| walker |  | 1171 | 12 | go decl xterm.go:4 |  |  | 0.320 |
| walker |  | 1179 | 8 | go body xterm.go:9 |  |  | 0.320 |
| walker |  | 1187 | 8 | go body xterm.go:20 |  |  | 0.320 |
| walker |  | 1196 | 9 | go body screen.go:20 |  |  | 0.320 |
| walker |  | 1208 | 12 | listing of '.github' |  |  | 0.320 |
| walker |  | 1239 | 31 | listing of '.github/workflows' |  |  | 0.322 |
| walker |  | 1301 | 62 | go names environ.go |  |  | 0.322 |
| ns | 1338 |  | 206 | Direct dependency set from `go.mod` | 1.10 |  | 0.308 |
| walker |  | 1367 | 66 | go names logging.go |  |  | 0.308 |
| walker |  | 1389 | 22 | go decl logging.go:29 |  |  | 0.308 |
| ns | 1522 |  | 184 | README lede: positioning and shipped feature set | 1.11 |  | 0.300 |
| ns | 1665 |  | 143 | Sentinel errors returned by `Program.Run` | 2.1 |  | 0.289 |
| walker |  | 1689 | 300 | go names tea.go |  |  | 0.299 |
| walker |  | 1700 | 11 | go decl tea.go:312 |  |  | 0.299 |
| walker |  | 1707 | 7 | go doc tea.go:312 |  |  | 0.299 |
| ns | 1810 |  | 145 | Every exported `*Program` method (complete roster) | 2.2 | 1.4 | 0.289 |
| walker |  | 1822 | 115 | go decl tea.go:336 |  |  | 0.289 |
| walker |  | 1831 | 9 | go body tea.go:279 |  |  | 0.290 |
| walker |  | 1993 | 162 | go decl tea.go:53 |  |  | 0.365 |
| ns | 2001 |  | 191 | Every `ProgramOption` constructor (complete roster) | 2.3 |  | 0.350 |
| ns | 2130 |  | 129 | Build/test/lint entry points (`Taskfile.yaml`) | 2.4 |  | 0.368 |
| walker |  | 2247 | 254 | go decl tea.go:286 |  |  | 0.371 |
| walker |  | 2395 | 148 | go module doc tea.go |  |  | 0.561 |
| walker |  | 2407 | 12 | go doc paste.go:10 |  |  | 0.561 |
| walker |  | 2419 | 12 | go doc termcap.go:46 |  |  | 0.561 |
| walker |  | 2431 | 12 | go doc xterm.go:9 |  |  | 0.561 |
| ns | 2436 |  | 306 | Event-loop dispatch table: every internally-handled message type | 2.5 |  | 0.519 |
| ns | 2682 |  | 246 | Event-loop skeleton: translate → filter → dispatch → update → render | 2.6 | 2.5 | 0.489 |
| walker |  | 2792 | 361 | go decl tea.go:241 |  |  | 0.489 |
| walker |  | 2836 | 44 | go module identity in tutorials/go.mod |  |  | 0.489 |
| walker |  | 2850 | 14 | go body environ.go:24 |  |  | 0.489 |
| walker |  | 2864 | 14 | go body environ.go:32 |  |  | 0.489 |
| ns | 2901 |  | 219 | Program control messages: suspend, resume, interrupt | 2.7 | 1.9 | 0.466 |
| ns | 3100 |  | 199 | `Run()`: how input and the TTY are acquired | 2.8 |  | 0.450 |
| walker |  | 3144 | 280 | go names tea.go #1 |  |  | 0.456 |
| walker |  | 3171 | 27 | go decl tea.go:395 |  |  | 0.456 |
| ns | 3288 |  | 188 | `Run()`: renderer choice, colour profile, first three messages | 2.9 | 2.8 | 0.443 |
| walker |  | 3319 | 148 | go decl tea.go:357 |  |  | 0.444 |
| walker |  | 3416 | 97 | go names cursor.go |  |  | 0.444 |
| walker |  | 3427 | 11 | go decl cursor.go:15 |  |  | 0.445 |
| walker |  | 3441 | 14 | go decl cursor.go:7 |  |  | 0.445 |
| walker |  | 3447 | 6 | go doc cursor.go:15 |  |  | 0.445 |
| walker |  | 3457 | 10 | go body cursor.go:26 |  |  | 0.445 |
| ns | 3474 |  | 186 | `Run()`: model init, event loop, graceful shutdown | 2.10 | 2.9 | 0.430 |
| walker |  | 3556 | 99 | go names renderer.go |  |  | 0.430 |
| walker |  | 3569 | 13 | go decl renderer.go:59 |  |  | 0.430 |
| walker |  | 3580 | 11 | go doc cursor.go:12 |  |  | 0.430 |
| walker |  | 3596 | 16 | go body logging.go:23 |  |  | 0.430 |
| ns | 3693 |  | 219 | Debug hooks: `TEA_TRACE`, `TEA_DEBUG`, panic recovery | 2.11 |  | 0.421 |
| walker |  | 3812 | 216 | go names cursed_renderer.go |  |  | 0.422 |
| walker |  | 3884 | 72 | listing of 'testdata/TestViewModel' |  |  | 0.422 |
| ns | 3916 |  | 223 | Frame pacing: the render ticker and the FPS bounds | 2.12 |  | 0.408 |
| walker |  | 4004 | 120 | go names keyboard.go |  |  | 0.409 |
| walker |  | 4015 | 11 | go body keyboard.go:33 |  |  | 0.409 |
| walker |  | 4033 | 18 | go body keyboard.go:39 |  |  | 0.409 |
| walker |  | 4051 | 18 | go body keyboard.go:45 |  |  | 0.409 |
| walker |  | 4069 | 18 | go body keyboard.go:57 |  |  | 0.409 |
| walker |  | 4090 | 21 | go doc logging.go:35 |  |  | 0.409 |
| ns | 4118 |  | 202 | Input event message types: keys, mouse, paste, focus (complete) | 3.1 |  | 0.400 |
| walker |  | 4332 | 242 | go names tea.go #2 |  |  | 0.411 |
| walker |  | 4371 | 39 | go decl renderer.go:10 |  |  | 0.416 |
| ns | 4380 |  | 262 | Terminal report message types: colour, clipboard, size, capability (complete) | 3.2 |  | 0.421 |
| walker |  | 4516 | 145 | go names commands.go |  |  | 0.424 |
| walker |  | 4525 | 9 | go body commands.go:173 |  |  | 0.424 |
| walker |  | 4540 | 15 | go body commands.go:15 |  |  | 0.424 |
| ns | 4550 |  | 170 | Command constructors, part 1: lifecycle, batching, timing, output | 3.3 | 2.7 | 0.433 |
| walker |  | 4555 | 15 | go body commands.go:25 |  |  | 0.433 |
| walker |  | 4579 | 24 | go doc focus.go:9 |  |  | 0.433 |
| ns | 4696 |  | 146 | Command constructors, part 2: terminal queries and clipboard | 3.4 | 3.3 | 0.429 |
| walker |  | 4728 | 149 | go names clipboard.go |  |  | 0.441 |
| walker |  | 4749 | 21 | go decl clipboard.go:5 |  |  | 0.445 |
| walker |  | 4757 | 8 | go body clipboard.go:15 |  |  | 0.445 |
| walker |  | 4765 | 8 | go body clipboard.go:20 |  |  | 0.445 |
| walker |  | 4774 | 9 | go body clipboard.go:42 |  |  | 0.445 |
| walker |  | 4784 | 10 | go body clipboard.go:68 |  |  | 0.445 |
| walker |  | 4936 | 152 | go names mod.go |  |  | 0.446 |
| walker |  | 4947 | 11 | go doc mod.go:6 |  |  | 0.447 |
| walker |  | 5033 | 86 | go decl mod.go:9 |  |  | 0.447 |
| ns | 5037 |  | 341 | `input.go`: the ultraviolet-event → `tea.Msg` translation table | 3.5 |  | 0.429 |
| walker |  | 5039 | 6 | go doc mod.go:9 |  |  | 0.429 |
| walker |  | 5046 | 7 | go body signals_windows.go:8 |  |  | 0.429 |
| walker |  | 5063 | 17 | go doc xterm.go:4 |  |  | 0.429 |
| walker |  | 5089 | 26 | go doc focus.go:5 |  |  | 0.429 |
| ns | 5113 |  | 76 | `Key` struct fields | 3.6 |  | 0.424 |
| walker |  | 5249 | 160 | go names tty.go |  |  | 0.424 |
| walker |  | 5268 | 19 | go doc tty.go:130 |  |  | 0.424 |
| ns | 5293 |  | 180 | Key message methods: `String`, `Keystroke`, `Key` | 3.7 | 3.6 | 0.419 |
| walker |  | 5294 | 26 | go body raw.go:33 |  |  | 0.419 |
| walker |  | 5320 | 26 | go body termcap.go:30 |  |  | 0.419 |
| walker |  | 5333 | 13 | go doc cursor.go:4 |  |  | 0.419 |
| walker |  | 5563 | 230 | go names color.go |  |  | 0.442 |
| walker |  | 5572 | 9 | go body color.go:13 |  |  | 0.442 |
| walker |  | 5581 | 9 | go body color.go:21 |  |  | 0.442 |
| walker |  | 5590 | 9 | go body color.go:29 |  |  | 0.442 |
| walker |  | 5602 | 12 | go doc color.go:39 |  |  | 0.442 |
| walker |  | 5614 | 12 | go doc color.go:70 |  |  | 0.425 |
| ns | 5614 |  | 321 | `key.go` key-code catalog: group headings plus the C0/G0 names | 3.8 |  | 0.425 |
| walker |  | 5626 | 12 | go doc color.go:84 |  |  | 0.425 |
| ns | 5797 |  | 183 | Modifier keys: the complete `KeyMod` constant set | 3.9 |  | 0.438 |
| walker |  | 5832 | 206 | go names options.go |  |  | 0.464 |
| walker |  | 5856 | 24 | go doc options.go:84 |  |  | 0.464 |
| walker |  | 5883 | 27 | go body options.go:30 |  |  | 0.464 |
| walker |  | 5910 | 27 | go body options.go:58 |  |  | 0.464 |
| walker |  | 5937 | 27 | go body options.go:133 |  |  | 0.464 |
| walker |  | 5966 | 29 | go doc logging.go:29 |  |  | 0.464 |
| walker |  | 5995 | 29 | go doc paste.go:20 |  |  | 0.464 |
| walker |  | 6004 | 9 | go doc cursed_renderer.go:75 |  |  | 0.464 |
| walker |  | 6013 | 9 | go doc cursed_renderer.go:143 |  |  | 0.464 |
| walker |  | 6022 | 9 | go doc cursed_renderer.go:257 |  |  | 0.464 |
| ns | 6023 |  | 226 | `Mouse` struct and the complete mouse-button constant set | 3.10 | 3.1 | 0.454 |
| walker |  | 6031 | 9 | go doc cursed_renderer.go:579 |  |  | 0.454 |
| walker |  | 6062 | 31 | go doc raw.go:5 |  |  | 0.454 |
| walker |  | 6077 | 15 | go doc clipboard.go:20 |  |  | 0.454 |
| walker |  | 6109 | 32 | go doc commands.go:25 |  |  | 0.454 |
| ns | 6200 |  | 177 | `MouseMode` enum: none / cell-motion / all-motion | 3.11 | 1.7 | 0.464 |
| walker |  | 6307 | 198 | go names exec.go |  |  | 0.470 |
| walker |  | 6350 | 43 | go decl exec.go:60 |  |  | 0.470 |
| walker |  | 6372 | 22 | go decl exec.go:10 |  |  | 0.470 |
| ns | 6378 |  | 178 | Cursor value types: `Cursor`, `NewCursor`, `CursorShape` | 3.12 | 1.7 | 0.479 |
| walker |  | 6386 | 14 | go body exec.go:50 |  |  | 0.479 |
| walker |  | 6412 | 26 | go doc exec.go:60 |  |  | 0.479 |
| ns | 6566 |  | 188 | Keyboard enhancements: request fields and response predicates | 3.13 | 3.2 | 0.486 |
| walker |  | 6682 | 270 | go names nil_renderer.go |  |  | 0.486 |
| walker |  | 6689 | 7 | go doc nil_renderer.go:15 |  |  | 0.486 |
| walker |  | 6696 | 7 | go doc nil_renderer.go:24 |  |  | 0.486 |
| walker |  | 6703 | 7 | go body nil_renderer.go:51 |  |  | 0.486 |
| walker |  | 6711 | 8 | go doc nil_renderer.go:18 |  |  | 0.486 |
| walker |  | 6732 | 21 | go body keyboard.go:51 |  |  | 0.486 |
| ns | 6759 |  | 193 | `Batch` vs `Sequence` semantics and where they execute | 3.14 | 3.3 | 0.485 |
| walker |  | 6797 | 65 | README headline in tutorials/basics/README.md |  |  | 0.485 |
| walker |  | 6955 | 158 | go names tea.go #3 |  |  | 0.496 |
| ns | 6959 |  | 200 | `Exec` / `ExecProcess` and the `ExecCommand` interface | 3.15 | 3.3 | 0.495 |
| walker |  | 7027 | 72 | README headline in tutorials/commands/README.md |  |  | 0.495 |
| walker |  | 7040 | 13 | go body color.go:39 |  |  | 0.495 |
| ns | 7111 |  | 152 | Logging to a file: `LogToFile` / `LogToFileWith` | 3.16 |  | 0.496 |
| walker |  | 7272 | 232 | go decl keyboard.go:9 |  |  | 0.498 |
| walker |  | 7280 | 8 | go doc nil_renderer.go:21 |  |  | 0.498 |
| ns | 7328 |  | 217 | The `renderer` interface: complete method set, and its two implementations | 4.1 |  | 0.491 |
| ns | 7681 |  | 353 | `cursedRenderer`: every function in the 860-line renderer | 4.2 | 4.1 | 0.484 |
| walker |  | 7682 | 402 | go names mouse.go |  |  | 0.504 |
| walker |  | 7693 | 11 | go decl mouse.go:29 |  |  | 0.506 |
| walker |  | 7726 | 33 | go decl mouse.go:71 |  |  | 0.511 |
| walker |  | 7766 | 40 | go decl mouse.go:46 |  |  | 0.514 |
| walker |  | 7774 | 8 | go body mouse.go:93 |  |  | 0.514 |
| walker |  | 7782 | 8 | go body mouse.go:108 |  |  | 0.514 |
| walker |  | 7790 | 8 | go body mouse.go:123 |  |  | 0.514 |
| walker |  | 7798 | 8 | go body mouse.go:142 |  |  | 0.514 |
| walker |  | 7808 | 10 | go body mouse.go:86 |  |  | 0.514 |
| walker |  | 7836 | 28 | go body options.go:22 |  |  | 0.514 |
| walker |  | 7867 | 31 | go doc paste.go:5 |  |  | 0.514 |
| walker |  | 7883 | 16 | go doc cursor.go:7 |  |  | 0.514 |
| walker |  | 7897 | 14 | go doc tty_unix.go:40 |  |  | 0.514 |
| walker |  | 7945 | 48 | go doc environ.go:24 |  |  | 0.514 |
| walker |  | 7991 | 46 | go body renderer.go:70 |  |  | 0.514 |
| walker |  | 8038 | 47 | go body renderer.go:86 |  |  | 0.514 |
| ns | 8039 |  | 358 | `View` fields → ANSI sequences, in `cursedRenderer.start` | 4.3 | 4.2 | 0.503 |
| ns | 8420 |  | 381 | Platform matrix: build tags and per-OS terminal functions | 4.4 |  | 0.500 |
| walker |  | 8554 | 516 | go decl tea.go:84 |  |  | 0.501 |
| walker |  | 8586 | 32 | go body exec.go:22 |  |  | 0.501 |
| ns | 8618 |  | 198 | `tty.go`: terminal acquisition, input loop, resize detection | 4.5 |  | 0.503 |
| walker |  | 8836 | 250 | listing of 'examples' |  |  | 0.508 |
| walker |  | 8846 | 10 | go doc cursed_renderer.go:249 |  |  | 0.508 |
| walker |  | 8856 | 10 | go body mouse.go:101 |  |  | 0.508 |
| walker |  | 8869 | 13 | go body color.go:70 |  |  | 0.508 |
| ns | 8882 |  | 264 | `examples/` and `tutorials/` directory listings | 5.1 |  | 0.540 |
| walker |  | 8885 | 16 | go doc tty.go:41 |  |  | 0.540 |
| walker |  | 8901 | 16 | go doc tty.go:56 |  |  | 0.540 |
| ns | 9062 |  | 180 | `UPGRADE_GUIDE_V2.md`: complete section map | 5.2 |  | 0.533 |
| ns | 9347 |  | 285 | The v1→v2 migration checklist | 5.3 | 5.2 | 0.529 |
| walker |  | 9493 | 592 | go decl tea.go:426 |  |  | 0.529 |
| ns | 9516 |  | 169 | README section map | 5.4 |  | 0.523 |
| walker |  | 9531 | 38 | go doc commands.go:21 |  |  | 0.523 |
| walker |  | 9854 | 323 | go names cursed_renderer.go #1 |  |  | 0.540 |
| walker |  | 9872 | 18 | go doc input.go:8 |  |  | 0.541 |
| ns | 9876 |  | 360 | Test suite: every test function, and the golden-file fixtures | 5.5 |  | 0.530 |
| walker |  | 9900 | 28 | go body options.go:98 |  |  | 0.530 |
| ns | 9917 |  | 41 | CI workflow inventory and golden-test fixture directories | 5.6 |  | 0.533 |
