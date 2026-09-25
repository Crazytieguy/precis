Score(3000)=0.713 I=0.933 C=0.545 ns_rows≤3K=21/56 grid(1000/1442/2080/3000/4327/6240/9000)=0.789/0.958/0.801/0.713/0.762/0.681/0.648

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 45 | 45 | Fs::DirListing { dir: . } |  |  | 0.000 |
| walker |  | 70 | 25 | Fs::DirListing { dir: doc } |  |  | 0.000 |
| ns | 96 |  | 96 | Crate identity: package name, description, homepage, licence | 1.1 |  | 0.000 |
| walker |  | 114 | 44 | Fs::DirListing { dir: src } |  |  | 0.000 |
| walker |  | 127 | 13 | Fs::DirListing { dir: src/parameter } |  |  | 0.000 |
| ns | 141 |  | 45 | Repository root listing (complete) | 1.2 |  | 0.512 |
| walker |  | 144 | 17 | Fs::DirListing { dir: src/output } |  |  | 0.518 |
| walker |  | 164 | 20 | Fs::DirListing { dir: src/timer } |  |  | 0.528 |
| walker |  | 192 | 28 | Fs::DirListing { dir: src/util } |  |  | 0.551 |
| ns | 217 |  | 76 | Cargo.toml: version 1.20.0, edition, MSRV, build script | 1.3 | 1.1 | 0.470 |
| walker |  | 227 | 35 | Fs::DirListing { dir: src/export } |  |  | 0.481 |
| walker |  | 235 | 8 | Fs::DirListing { dir: .github } |  |  | 0.482 |
| walker |  | 240 | 5 | Fs::DirListing { dir: .github/workflows } |  |  | 0.482 |
| ns | 261 |  | 44 | src/ listing: the flat modules and six subdirectories | 1.4 |  | 0.523 |
| walker |  | 309 | 69 | Markdown::ReadmeHeadline { file: README.md } |  |  | 0.523 |
| walker |  | 329 | 20 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.523 |
| walker |  | 342 | 13 | Code::CodeKey { rung: ModuleDoc, file: src/util/units.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.523 |
| ns | 343 |  | 82 | README feature list, first five bullets (rest elided) | 1.5 |  | 0.473 |
| ns | 409 |  | 66 | README feature list, remaining bullets | 1.6 | 1.5 | 0.449 |
| ns | 471 |  | 62 | src/benchmark/ and src/export/ listings (complete) | 1.7 |  | 0.419 |
| walker |  | 491 | 149 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: true } |  |  | 0.522 |
| ns | 549 |  | 78 | src/output/, src/parameter/, src/timer/, src/util/ listings (complete) | 1.8 |  | 0.538 |
| ns | 643 |  | 94 | src/main.rs: module declarations and the types the entry point imports | 1.9 |  | 0.498 |
| walker |  | 663 | 172 | Toml::Identity { file: Cargo.toml } |  |  | 0.858 |
| walker |  | 698 | 35 | Toml::Operational { file: Cargo.toml } |  |  | 0.858 |
| walker |  | 719 | 21 | Fs::DirListing { dir: tests } |  |  | 0.861 |
| walker |  | 758 | 39 | Markdown::Section { file: README.md, section_index: 11, keeps_default_concavity: false } |  |  | 0.861 |
| walker |  | 855 | 97 | Code::CodeKey { rung: ModuleDoc, file: src/outlier_detection.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.861 |
| ns | 899 |  | 256 | src/main.rs: the run() pipeline | 1.10 | 1.9 | 0.754 |
| walker |  | 975 | 120 | Code::CodeKey { rung: Names, file: src/main.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.820 |
| ns | 983 |  | 84 | src/main.rs: main() error reporting and exit code | 1.11 | 1.10 | 0.785 |
| walker |  | 1051 | 76 | Code::CodeKey { rung: Decl, file: src/main.rs, decl: 2, sub: 0, line: 53 } |  |  | 0.832 |
| ns | 1098 |  | 115 | tests/, doc/, scripts/ and .github/ listings (complete) | 1.12 |  | 0.787 |
| ns | 1235 |  | 137 | src/cli.rs: get_cli_arguments() and the build_command() clap header | 2.1 |  | 0.753 |
| walker |  | 1294 | 243 | Code::CodeKey { rung: Decl, file: src/main.rs, decl: 1, sub: 0, line: 29 } |  |  | 0.855 |
| walker |  | 1350 | 56 | Fs::DirListing { dir: scripts } |  |  | 0.912 |
| walker |  | 1377 | 27 | Fs::DirListing { dir: src/benchmark } |  |  | 0.958 |
| walker |  | 1389 | 12 | Code::CodeKey { rung: Names, file: build.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.958 |
| walker |  | 1443 | 54 | Markdown::Section { file: README.md, section_index: 14, keeps_default_concavity: false } |  |  | 0.958 |
| walker |  | 1525 | 82 | Markdown::Section { file: README.md, section_index: 13, keeps_default_concavity: false } |  |  | 0.958 |
| ns | 1672 |  | 437 | Complete option roster: every Arg::new(...) in src/cli.rs | 2.2 | 2.1 | 0.861 |
| walker |  | 1764 | 239 | Code::CodeKey { rung: Names, file: src/options.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.862 |
| walker |  | 1771 | 7 | Code::CodeKey { rung: Decl, file: src/options.rs, decl: 2, sub: 0, line: 19 } |  |  | 0.862 |
| walker |  | 1781 | 10 | Code::CodeKey { rung: Decl, file: src/options.rs, decl: 1, sub: 0, line: 16 } |  |  | 0.862 |
| walker |  | 1797 | 16 | Code::CodeKey { rung: Decl, file: src/options.rs, decl: 4, sub: 0, line: 32 } |  |  | 0.862 |
| walker |  | 1813 | 16 | Code::CodeKey { rung: Decl, file: src/options.rs, decl: 27, sub: 0, line: 251 } |  |  | 0.862 |
| walker |  | 1831 | 18 | Code::CodeKey { rung: Decl, file: src/options.rs, decl: 15, sub: 0, line: 116 } |  |  | 0.862 |
| walker |  | 1849 | 18 | Code::CodeKey { rung: Decl, file: src/options.rs, decl: 24, sub: 0, line: 191 } |  |  | 0.862 |
| ns | 1862 |  | 190 | Complete short-alias roster for the CLI options | 2.3 | 2.2 | 0.822 |
| walker |  | 1878 | 29 | Code::CodeKey { rung: Decl, file: src/options.rs, decl: 18, sub: 0, line: 132 } |  |  | 0.822 |
| walker |  | 1908 | 30 | Code::CodeKey { rung: Decl, file: src/options.rs, decl: 6, sub: 0, line: 38 } |  |  | 0.822 |
| walker |  | 1941 | 33 | Code::CodeKey { rung: Decl, file: src/options.rs, decl: 21, sub: 0, line: 164 } |  |  | 0.822 |
| walker |  | 1978 | 37 | Code::CodeKey { rung: Decl, file: src/options.rs, decl: 13, sub: 0, line: 101 } |  |  | 0.822 |
| ns | 2016 |  | 154 | Enumerated value sets, defaults, and the two hidden options | 2.4 | 2.2 | 0.799 |
| walker |  | 2017 | 39 | Code::CodeKey { rung: Decl, file: src/options.rs, decl: 23, sub: 0, line: 184 } |  |  | 0.801 |
| walker |  | 2070 | 53 | Code::CodeKey { rung: Decl, file: src/options.rs, decl: 8, sub: 0, line: 47 } |  |  | 0.801 |
| walker |  | 2125 | 55 | Code::CodeKey { rung: Decl, file: src/options.rs, decl: 14, sub: 0, line: 108 } |  |  | 0.801 |
| ns | 2146 |  | 130 | Complete set of inter-option conflicts and requirements | 2.5 | 2.2 | 0.784 |
| walker |  | 2188 | 63 | Code::CodeKey { rung: Decl, file: src/options.rs, decl: 29, sub: 0, line: 275 } |  |  | 0.784 |
| walker |  | 2252 | 64 | Code::CodeKey { rung: Decl, file: src/options.rs, decl: 3, sub: 0, line: 23 } |  |  | 0.786 |
| walker |  | 2263 | 11 | Code::CodeKey { rung: Doc, file: src/options.rs, decl: 14, sub: 0, line: 108 } |  |  | 0.787 |
| walker |  | 2332 | 69 | Code::CodeKey { rung: Decl, file: src/options.rs, decl: 17, sub: 0, line: 122 } |  |  | 0.787 |
| walker |  | 2344 | 12 | Code::CodeKey { rung: Doc, file: src/options.rs, decl: 3, sub: 0, line: 23 } |  |  | 0.788 |
| walker |  | 2356 | 12 | Code::CodeKey { rung: Doc, file: src/options.rs, decl: 9, sub: 0, line: 49 } |  |  | 0.788 |
| ns | 2366 |  | 220 | src/options.rs: the Options struct, run bounds through setup_command | 3.1 |  | 0.757 |
| walker |  | 2452 | 96 | Code::CodeKey { rung: Decl, file: src/options.rs, decl: 11, sub: 0, line: 70 } |  |  | 0.759 |
| walker |  | 2464 | 12 | Code::CodeKey { rung: Doc, file: src/options.rs, decl: 11, sub: 0, line: 70 } |  |  | 0.759 |
| ns | 2577 |  | 211 | src/options.rs: remaining Options fields (output, sort, executor, I/O, time unit) | 3.2 | 3.1 | 0.734 |
| walker |  | 2592 | 128 | Code::CodeKey { rung: Decl, file: src/options.rs, decl: 20, sub: 0, line: 148 } |  |  | 0.738 |
| walker |  | 2605 | 13 | Code::CodeKey { rung: Doc, file: src/options.rs, decl: 20, sub: 0, line: 148 } |  |  | 0.738 |
| walker |  | 2754 | 149 | Code::CodeKey { rung: Decl, file: src/options.rs, decl: 12, sub: 0, line: 83 } |  |  | 0.742 |
| walker |  | 2762 | 8 | Code::CodeKey { rung: Doc, file: src/options.rs, decl: 12, sub: 0, line: 83 } |  |  | 0.743 |
| ns | 2839 |  | 262 | src/options.rs: impl Default for Options -- the concrete default values | 3.3 | 3.1 | 0.706 |
| ns | 2991 |  | 152 | src/options.rs: DEFAULT_SHELL per platform, the Shell enum and its parser | 3.4 |  | 0.713 |
| ns | 3094 |  | 103 | src/options.rs: ExecutorKind (Raw / Shell / Mock) and its default | 3.5 | 3.4 | 0.711 |
| walker |  | 3235 | 473 | Toml::Dependencies { file: Cargo.toml } |  |  | 0.711 |
| walker |  | 3299 | 64 | Code::CodeKey { rung: Names, file: src/error.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.711 |
| ns | 3318 |  | 224 | src/options.rs: OutputStyleOption and SortOrder vocabularies | 3.6 |  | 0.726 |
| walker |  | 3323 | 24 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 4, sub: 0, line: 30 } |  |  | 0.726 |
| walker |  | 3348 | 25 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 2, sub: 0, line: 24 } |  |  | 0.726 |
| walker |  | 3362 | 14 | Code::CodeKey { rung: Body, file: src/error.rs, decl: 3, sub: 0, line: 25 } |  |  | 0.726 |
| walker |  | 3376 | 14 | Code::CodeKey { rung: Body, file: src/error.rs, decl: 5, sub: 0, line: 31 } |  |  | 0.726 |
| ns | 3560 |  | 242 | src/options.rs: CommandInputPolicy and CommandOutputPolicy | 3.7 |  | 0.741 |
| walker |  | 3621 | 245 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 1, sub: 0, line: 6 } |  |  | 0.744 |
| ns | 3800 |  | 240 | src/options.rs: CmdFailureAction and RunBounds (default min = 10) | 3.8 |  | 0.749 |
| walker |  | 3992 | 371 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 6, sub: 0, line: 36 } |  |  | 0.753 |
| walker |  | 4017 | 25 | Code::CodeKey { rung: Names, file: src/output/mod.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.753 |
| walker |  | 4100 | 83 | Code::CodeKey { rung: Names, file: src/command.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.753 |
| walker |  | 4132 | 32 | Code::CodeKey { rung: Decl, file: src/command.rs, decl: 16, sub: 0, line: 387 } |  |  | 0.753 |
| ns | 4189 |  | 389 | src/error.rs: the complete OptionsError variant set with messages | 3.10 |  | 0.762 |
| walker |  | 4211 | 79 | Code::CodeKey { rung: Decl, file: src/command.rs, decl: 12, sub: 0, line: 136 } |  |  | 0.762 |
| walker |  | 4221 | 10 | Code::CodeKey { rung: Body, file: src/command.rs, decl: 14, sub: 0, line: 250 } |  |  | 0.762 |
| walker |  | 4236 | 15 | Code::CodeKey { rung: Doc, file: src/command.rs, decl: 11, sub: 0, line: 134 } |  |  | 0.762 |
| walker |  | 4347 | 111 | Code::CodeKey { rung: Decl, file: src/command.rs, decl: 1, sub: 0, line: 21 } |  |  | 0.763 |
| walker |  | 4360 | 13 | Code::CodeKey { rung: Doc, file: src/command.rs, decl: 1, sub: 0, line: 21 } |  |  | 0.763 |
| ns | 4407 |  | 218 | src/error.rs: the complete ParameterScanError variant set | 3.11 | 3.10 | 0.769 |
| walker |  | 4540 | 180 | Code::CodeKey { rung: Decl, file: src/command.rs, decl: 2, sub: 0, line: 33 } |  |  | 0.770 |
| walker |  | 4595 | 55 | Code::CodeKey { rung: Decl, file: src/command.rs, decl: 4, sub: 0, line: 42 } |  |  | 0.770 |
| walker |  | 4604 | 9 | Code::CodeKey { rung: Body, file: src/command.rs, decl: 9, sub: 0, line: 96 } |  |  | 0.770 |
| walker |  | 4616 | 12 | Code::CodeKey { rung: Body, file: src/command.rs, decl: 7, sub: 0, line: 77 } |  |  | 0.770 |
| ns | 4627 |  | 220 | src/benchmark/executor.rs: the Executor trait | 4.1 |  | 0.751 |
| walker |  | 4660 | 44 | Code::CodeKey { rung: Names, file: src/util/mod.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.751 |
| ns | 4752 |  | 125 | The three Executor implementations and their state | 4.2 | 4.1 | 0.738 |
| walker |  | 4863 | 203 | Code::CodeKey { rung: Body, file: build.rs, decl: 1, sub: 0, line: 7 } |  |  | 0.739 |
| ns | 4904 |  | 152 | BenchmarkIteration and the $HYPERFINE_ITERATION values | 4.3 |  | 0.725 |
| walker |  | 4905 | 42 | Code::CodeKey { rung: Names, file: src/util/units.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.725 |
| walker |  | 4947 | 42 | Code::CodeKey { rung: Decl, file: src/util/units.rs, decl: 4, sub: 0, line: 16 } |  |  | 0.725 |
| walker |  | 4990 | 43 | Code::CodeKey { rung: Decl, file: src/util/units.rs, decl: 3, sub: 0, line: 9 } |  |  | 0.725 |
| walker |  | 4997 | 7 | Code::CodeKey { rung: Doc, file: src/util/units.rs, decl: 3, sub: 0, line: 9 } |  |  | 0.725 |
| walker |  | 5007 | 10 | Code::CodeKey { rung: Doc, file: src/util/units.rs, decl: 2, sub: 0, line: 6 } |  |  | 0.725 |
| walker |  | 5017 | 10 | Code::CodeKey { rung: Doc, file: src/util/units.rs, decl: 5, sub: 0, line: 18 } |  |  | 0.725 |
| walker |  | 5032 | 15 | Code::CodeKey { rung: Doc, file: src/util/units.rs, decl: 6, sub: 0, line: 27 } |  |  | 0.725 |
| ns | 5072 |  | 168 | The shared command runner: stdio wiring and injected environment variables | 4.4 | 4.3 | 0.713 |
| walker |  | 5073 | 41 | Code::CodeKey { rung: Names, file: src/cli.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.714 |
| walker |  | 5115 | 42 | Code::CodeKey { rung: Decl, file: src/cli.rs, decl: 1, sub: 0, line: 8 } |  |  | 0.714 |
| walker |  | 5130 | 15 | Code::CodeKey { rung: Doc, file: src/cli.rs, decl: 2, sub: 0, line: 18 } |  |  | 0.716 |
| walker |  | 5152 | 22 | Code::CodeKey { rung: Body, file: src/cli.rs, decl: 1, sub: 0, line: 8 } |  |  | 0.716 |
| ns | 5284 |  | 212 | Non-zero exit handling and the failure message users actually see | 4.5 | 4.4 | 0.703 |
| walker |  | 5291 | 139 | Code::CodeKey { rung: Names, file: src/export/mod.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.703 |
| walker |  | 5306 | 15 | Code::CodeKey { rung: Decl, file: src/export/mod.rs, decl: 2, sub: 0, line: 46 } |  |  | 0.703 |
| walker |  | 5325 | 19 | Code::CodeKey { rung: Decl, file: src/export/mod.rs, decl: 4, sub: 0, line: 56 } |  |  | 0.703 |
| walker |  | 5353 | 28 | Code::CodeKey { rung: Decl, file: src/export/mod.rs, decl: 5, sub: 0, line: 61 } |  |  | 0.703 |
| walker |  | 5391 | 38 | Code::CodeKey { rung: Decl, file: src/export/mod.rs, decl: 6, sub: 0, line: 67 } |  |  | 0.703 |
| walker |  | 5441 | 50 | Code::CodeKey { rung: Decl, file: src/export/mod.rs, decl: 3, sub: 0, line: 48 } |  |  | 0.704 |
| walker |  | 5451 | 10 | Code::CodeKey { rung: Doc, file: src/export/mod.rs, decl: 2, sub: 0, line: 46 } |  |  | 0.704 |
| walker |  | 5462 | 11 | Code::CodeKey { rung: Doc, file: src/export/mod.rs, decl: 6, sub: 0, line: 67 } |  |  | 0.704 |
| ns | 5481 |  | 197 | Shell-spawning calibration: 50 probe runs, and where the overhead is subtracted | 4.6 | 4.2 | 0.694 |
| walker |  | 5541 | 79 | Code::CodeKey { rung: Decl, file: src/export/mod.rs, decl: 7, sub: 0, line: 73 } |  |  | 0.694 |
| walker |  | 5585 | 44 | Code::CodeKey { rung: Decl, file: src/export/mod.rs, decl: 8, sub: 0, line: 76 } |  |  | 0.694 |
| walker |  | 5598 | 13 | Code::CodeKey { rung: Doc, file: src/export/mod.rs, decl: 3, sub: 0, line: 48 } |  |  | 0.694 |
| walker |  | 5613 | 15 | Code::CodeKey { rung: Doc, file: src/export/mod.rs, decl: 9, sub: 0, line: 103 } |  |  | 0.694 |
| walker |  | 5628 | 15 | Code::CodeKey { rung: Doc, file: src/export/mod.rs, decl: 11, sub: 0, line: 158 } |  |  | 0.694 |
| ns | 5679 |  | 198 | src/benchmark/scheduler.rs: Scheduler state and executor selection | 4.7 |  | 0.684 |
| walker |  | 5754 | 126 | Code::CodeKey { rung: Decl, file: src/export/mod.rs, decl: 1, sub: 0, line: 27 } |  |  | 0.685 |
| walker |  | 5771 | 17 | Code::CodeKey { rung: Doc, file: src/export/mod.rs, decl: 1, sub: 0, line: 27 } |  |  | 0.685 |
| walker |  | 5798 | 27 | Code::CodeKey { rung: Doc, file: src/export/mod.rs, decl: 8, sub: 0, line: 76 } |  |  | 0.685 |
| ns | 5933 |  | 254 | run_benchmarks(): reference command first, calibrate once, export after each benchmark | 4.8 | 4.7 | 0.669 |
| walker |  | 6057 | 259 | Code::CodeKey { rung: Decl, file: src/options.rs, decl: 26, sub: 0, line: 198 } |  |  | 0.681 |
| walker |  | 6070 | 13 | Code::CodeKey { rung: Doc, file: src/options.rs, decl: 26, sub: 0, line: 198 } |  |  | 0.684 |
| ns | 6208 |  | 275 | src/benchmark/mod.rs: Benchmark struct, MIN_EXECUTION_TIME, and the complete method roster | 4.10 |  | 0.669 |
| walker |  | 6389 | 319 | Code::CodeKey { rung: Decl, file: src/options.rs, decl: 26, sub: 1, line: 198 } |  |  | 0.697 |
| ns | 6418 |  | 210 | How the number of runs is decided | 4.11 | 4.10 | 0.683 |
| ns | 6626 |  | 208 | The three warning triggers inside Benchmark::run | 4.12 | 4.10 | 0.673 |
| walker |  | 6640 | 251 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.673 |
| walker |  | 6665 | 25 | Code::CodeKey { rung: Names, file: src/timer/wall_clock_timer.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.673 |
| walker |  | 6677 | 12 | Code::CodeKey { rung: Decl, file: src/timer/wall_clock_timer.rs, decl: 1, sub: 0, line: 5 } |  |  | 0.673 |
| walker |  | 6712 | 35 | Code::CodeKey { rung: Decl, file: src/timer/wall_clock_timer.rs, decl: 2, sub: 0, line: 9 } |  |  | 0.673 |
| walker |  | 6738 | 26 | Code::CodeKey { rung: Body, file: src/timer/wall_clock_timer.rs, decl: 3, sub: 0, line: 10 } |  |  | 0.673 |
| ns | 6779 |  | 153 | src/outlier_detection.rs: the modified Z-score method and its threshold | 4.13 | 4.12 | 0.667 |
| walker |  | 6780 | 42 | Code::CodeKey { rung: Body, file: src/timer/wall_clock_timer.rs, decl: 4, sub: 0, line: 16 } |  |  | 0.667 |
| walker |  | 6896 | 116 | Code::CodeKey { rung: Names, file: src/timer/mod.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.667 |
| walker |  | 6910 | 14 | Code::CodeKey { rung: Doc, file: src/timer/mod.rs, decl: 3, sub: 0, line: 52 } |  |  | 0.667 |
| walker |  | 6995 | 85 | Code::CodeKey { rung: Decl, file: src/timer/mod.rs, decl: 2, sub: 0, line: 41 } |  |  | 0.667 |
| walker |  | 7010 | 15 | Code::CodeKey { rung: Doc, file: src/timer/mod.rs, decl: 2, sub: 0, line: 41 } |  |  | 0.667 |
| walker |  | 7025 | 15 | Code::CodeKey { rung: Doc, file: src/timer/mod.rs, decl: 4, sub: 0, line: 83 } |  |  | 0.667 |
| ns | 7075 |  | 296 | src/benchmark/benchmark_result.rs: the complete BenchmarkResult field set (= the JSON export schema) | 5.1 |  | 0.654 |
| walker |  | 7147 | 122 | Code::CodeKey { rung: Decl, file: src/timer/mod.rs, decl: 1, sub: 0, line: 27 } |  |  | 0.654 |
| walker |  | 7197 | 50 | Code::CodeKey { rung: Names, file: src/outlier_detection.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.656 |
| ns | 7215 |  | 140 | src/benchmark/timing_result.rs: TimingResult in full | 5.2 | 5.1 | 0.648 |
| walker |  | 7260 | 63 | Code::CodeKey { rung: Doc, file: src/outlier_detection.rs, decl: 1, sub: 0, line: 13 } |  |  | 0.654 |
| walker |  | 7364 | 104 | Code::CodeKey { rung: Doc, file: src/outlier_detection.rs, decl: 2, sub: 0, line: 21 } |  |  | 0.656 |
| ns | 7378 |  | 163 | src/benchmark/relative_speed.rs: annotated result type and the complete public function set | 5.3 |  | 0.648 |
| walker |  | 7430 | 66 | Code::CodeKey { rung: Names, file: src/parameter/mod.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.648 |
| walker |  | 7462 | 32 | Code::CodeKey { rung: Decl, file: src/parameter/mod.rs, decl: 1, sub: 0, line: 7 } |  |  | 0.648 |
| walker |  | 7496 | 34 | Code::CodeKey { rung: Decl, file: src/parameter/mod.rs, decl: 2, sub: 0, line: 13 } |  |  | 0.648 |
| ns | 7536 |  | 158 | src/export/mod.rs: the ExportType enum -- the five output formats | 5.4 |  | 0.656 |
| walker |  | 7557 | 61 | Code::CodeKey { rung: Body, file: src/parameter/mod.rs, decl: 3, sub: 0, line: 14 } |  |  | 0.656 |
| ns | 7698 |  | 162 | src/export/mod.rs: the Exporter trait and ExportManager | 5.5 | 5.4 | 0.663 |
| walker |  | 7769 | 212 | Markdown::Section { file: README.md, section_index: 12, keeps_default_concavity: false } |  |  | 0.663 |
| walker |  | 7809 | 40 | Code::CodeKey { rung: Names, file: src/output/warnings.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.663 |
| ns | 7835 |  | 137 | Flag-to-exporter wiring and the '-' means stdout convention | 5.6 | 5.5 | 0.658 |
| walker |  | 7839 | 30 | Code::CodeKey { rung: Decl, file: src/output/warnings.rs, decl: 1, sub: 0, line: 7 } |  |  | 0.658 |
| walker |  | 7869 | 30 | Code::CodeKey { rung: Decl, file: src/output/warnings.rs, decl: 3, sub: 0, line: 20 } |  |  | 0.658 |
| walker |  | 7921 | 52 | Code::CodeKey { rung: Decl, file: src/output/warnings.rs, decl: 2, sub: 0, line: 13 } |  |  | 0.659 |
| walker |  | 7931 | 10 | Code::CodeKey { rung: Doc, file: src/output/warnings.rs, decl: 2, sub: 0, line: 13 } |  |  | 0.659 |
| walker |  | 7999 | 68 | Code::CodeKey { rung: Names, file: src/util/number.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.659 |
| walker |  | 8018 | 19 | Code::CodeKey { rung: Decl, file: src/util/number.rs, decl: 6, sub: 0, line: 30 } |  |  | 0.659 |
| walker |  | 8038 | 20 | Code::CodeKey { rung: Decl, file: src/util/number.rs, decl: 4, sub: 0, line: 24 } |  |  | 0.659 |
| ns | 8051 |  | 216 | src/export/markup.rs: the shared table shape and the blanket Exporter impl | 5.7 | 5.5 | 0.651 |
| walker |  | 8068 | 30 | Code::CodeKey { rung: Decl, file: src/util/number.rs, decl: 2, sub: 0, line: 15 } |  |  | 0.651 |
| walker |  | 8109 | 41 | Code::CodeKey { rung: Decl, file: src/util/number.rs, decl: 8, sub: 0, line: 36 } |  |  | 0.651 |
| walker |  | 8160 | 51 | Code::CodeKey { rung: Decl, file: src/util/number.rs, decl: 1, sub: 0, line: 8 } |  |  | 0.652 |
| walker |  | 8170 | 10 | Code::CodeKey { rung: Body, file: src/util/number.rs, decl: 5, sub: 0, line: 25 } |  |  | 0.652 |
| walker |  | 8180 | 10 | Code::CodeKey { rung: Body, file: src/util/number.rs, decl: 7, sub: 0, line: 31 } |  |  | 0.652 |
| ns | 8267 |  | 216 | The five exporter types, and the CSV column set | 5.8 | 5.4 | 0.644 |
| walker |  | 8439 | 259 | Markdown::Section { file: README.md, section_index: 7, keeps_default_concavity: false } |  |  | 0.644 |
| ns | 8524 |  | 257 | src/output/format.rs: automatic time-unit selection, and the Unit type | 5.9 |  | 0.637 |
| walker |  | 8650 | 211 | Markdown::Section { file: README.md, section_index: 8, keeps_default_concavity: false } |  |  | 0.637 |
| ns | 8661 |  | 137 | src/output/warnings.rs: the complete Warnings set | 5.10 | 4.12 | 0.642 |
| walker |  | 8666 | 16 | Code::CodeKey { rung: Body, file: src/command.rs, decl: 17, sub: 0, line: 388 } |  |  | 0.642 |
| walker |  | 8709 | 43 | Code::CodeKey { rung: Names, file: src/export/json.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.642 |
| walker |  | 8717 | 8 | Code::CodeKey { rung: Decl, file: src/export/json.rs, decl: 2, sub: 0, line: 16 } |  |  | 0.642 |
| walker |  | 8730 | 13 | Code::CodeKey { rung: Decl, file: src/export/json.rs, decl: 3, sub: 0, line: 19 } |  |  | 0.642 |
| walker |  | 8757 | 27 | Code::CodeKey { rung: Decl, file: src/export/json.rs, decl: 1, sub: 0, line: 11 } |  |  | 0.642 |
| walker |  | 8812 | 55 | Code::CodeKey { rung: Decl, file: src/export/json.rs, decl: 4, sub: 0, line: 20 } |  |  | 0.642 |
| walker |  | 8841 | 29 | Code::CodeKey { rung: Names, file: src/export/csv.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.643 |
| walker |  | 8849 | 8 | Code::CodeKey { rung: Decl, file: src/export/csv.rs, decl: 1, sub: 0, line: 12 } |  |  | 0.643 |
| walker |  | 8864 | 15 | Code::CodeKey { rung: Decl, file: src/export/csv.rs, decl: 2, sub: 0, line: 15 } |  |  | 0.643 |
| ns | 8869 |  | 208 | src/command.rs: the Command type and its complete method roster | 6.1 |  | 0.647 |
| walker |  | 8919 | 55 | Code::CodeKey { rung: Decl, file: src/export/csv.rs, decl: 3, sub: 0, line: 16 } |  |  | 0.647 |
| walker |  | 8949 | 30 | Code::CodeKey { rung: Names, file: src/export/markdown.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.648 |
| walker |  | 8957 | 8 | Code::CodeKey { rung: Decl, file: src/export/markdown.rs, decl: 1, sub: 0, line: 5 } |  |  | 0.648 |
| ns | 9015 |  | 146 | How {param} placeholders are substituted, and why naively | 6.2 | 6.1 | 0.644 |
| walker |  | 9030 | 73 | Code::CodeKey { rung: Decl, file: src/export/markdown.rs, decl: 2, sub: 0, line: 8 } |  |  | 0.644 |
| walker |  | 9042 | 12 | Code::CodeKey { rung: Body, file: src/export/markdown.rs, decl: 5, sub: 0, line: 26 } |  |  | 0.644 |
| walker |  | 9059 | 17 | Code::CodeKey { rung: Body, file: src/export/markdown.rs, decl: 3, sub: 0, line: 9 } |  |  | 0.644 |
| walker |  | 9074 | 15 | Code::CodeKey { rung: Names, file: src/util/randomized_environment_offset.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.644 |
| walker |  | 9095 | 21 | Code::CodeKey { rung: Body, file: src/util/randomized_environment_offset.rs, decl: 1, sub: 0, line: 6 } |  |  | 0.644 |
| walker |  | 9157 | 62 | Code::CodeKey { rung: Names, file: src/export/markup.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.644 |
| walker |  | 9170 | 13 | Code::CodeKey { rung: Decl, file: src/export/markup.rs, decl: 10, sub: 0, line: 108 } |  |  | 0.644 |
| walker |  | 9187 | 17 | Code::CodeKey { rung: Decl, file: src/export/markup.rs, decl: 1, sub: 0, line: 10 } |  |  | 0.644 |
| ns | 9202 |  | 187 | src/command.rs: Commands and its complete method roster, with the three construction modes | 6.3 |  | 0.640 |
| walker |  | 9240 | 53 | Code::CodeKey { rung: Decl, file: src/export/markup.rs, decl: 11, sub: 0, line: 109 } |  |  | 0.640 |
| walker |  | 9396 | 156 | Code::CodeKey { rung: Decl, file: src/export/markup.rs, decl: 2, sub: 0, line: 15 } |  |  | 0.645 |
| walker |  | 9405 | 9 | Code::CodeKey { rung: Body, file: src/export/markup.rs, decl: 6, sub: 0, line: 87 } |  |  | 0.645 |
| walker |  | 9414 | 9 | Code::CodeKey { rung: Body, file: src/export/markup.rs, decl: 7, sub: 0, line: 91 } |  |  | 0.645 |
| ns | 9431 |  | 229 | src/parameter/: ParameterValue, RangeStep and its 100_000 cap, tokenize() | 6.4 |  | 0.640 |
| ns | 9581 |  | 150 | build.rs: shell completions generated from the same clap command | 7.1 | 2.1 | 0.641 |
| walker |  | 9741 | 327 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.641 |
| ns | 9790 |  | 209 | .github/workflows/CICD.yml: the complete job set and the commands each runs | 7.2 |  | 0.634 |
| walker |  | 9821 | 80 | Code::CodeKey { rung: Names, file: src/timer/unix_timer.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.634 |
| walker |  | 9836 | 15 | Code::CodeKey { rung: Decl, file: src/timer/unix_timer.rs, decl: 2, sub: 0, line: 18 } |  |  | 0.634 |
| walker |  | 9878 | 42 | Code::CodeKey { rung: Decl, file: src/timer/unix_timer.rs, decl: 3, sub: 0, line: 22 } |  |  | 0.634 |
| walker |  | 9950 | 72 | Code::CodeKey { rung: Decl, file: src/timer/unix_timer.rs, decl: 1, sub: 0, line: 9 } |  |  | 0.634 |
| walker |  | 9964 | 14 | Code::CodeKey { rung: Doc, file: src/timer/unix_timer.rs, decl: 6, sub: 0, line: 41 } |  |  | 0.634 |
| ns | 9982 |  | 192 | tests/: the shared harness helpers and the debug-mode test idiom | 7.3 |  | 0.627 |
| walker |  | 9983 | 19 | Code::CodeKey { rung: Doc, file: src/timer/unix_timer.rs, decl: 7, sub: 0, line: 71 } |  |  | 0.627 |
| walker |  | 9992 | 9 | Code::CodeKey { rung: Body, file: src/timer/unix_timer.rs, decl: 4, sub: 0, line: 23 } |  |  | 0.627 |
