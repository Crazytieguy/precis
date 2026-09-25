Score(3000)=0.713 I=0.933 C=0.545 ns_rows≤3K=21/56 grid(1000/1442/2080/3000/4327/6240/9000)=0.733/0.897/0.801/0.713/0.762/0.686/0.646

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 45 | 45 | Fs::DirListing { dir: . } |  |  | 0.000 |
| ns | 96 |  | 96 | Crate identity: package name, description, homepage, licence | 1.1 |  | 0.000 |
| walker |  | 114 | 69 | Markdown::ReadmeHeadline { file: README.md } |  |  | 0.000 |
| walker |  | 139 | 25 | Fs::DirListing { dir: doc } |  |  | 0.000 |
| ns | 141 |  | 45 | Repository root listing (complete) | 1.2 |  | 0.441 |
| walker |  | 183 | 44 | Fs::DirListing { dir: src } |  |  | 0.510 |
| walker |  | 196 | 13 | Fs::DirListing { dir: src/parameter } |  |  | 0.512 |
| walker |  | 213 | 17 | Fs::DirListing { dir: src/output } |  |  | 0.518 |
| ns | 217 |  | 76 | Cargo.toml: version 1.20.0, edition, MSRV, build script | 1.3 | 1.1 | 0.442 |
| walker |  | 233 | 20 | Fs::DirListing { dir: src/timer } |  |  | 0.451 |
| walker |  | 261 | 28 | Fs::DirListing { dir: src/util } |  |  | 0.510 |
| ns | 261 |  | 44 | src/ listing: the flat modules and six subdirectories | 1.4 |  | 0.510 |
| walker |  | 296 | 35 | Fs::DirListing { dir: src/export } |  |  | 0.521 |
| walker |  | 304 | 8 | Fs::DirListing { dir: .github } |  |  | 0.522 |
| walker |  | 309 | 5 | Fs::DirListing { dir: .github/workflows } |  |  | 0.523 |
| walker |  | 322 | 13 | Code::CodeKey { rung: ModuleDoc, file: src/util/units.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.523 |
| ns | 343 |  | 82 | README feature list, first five bullets (rest elided) | 1.5 |  | 0.473 |
| ns | 409 |  | 66 | README feature list, remaining bullets | 1.6 | 1.5 | 0.449 |
| ns | 471 |  | 62 | src/benchmark/ and src/export/ listings (complete) | 1.7 |  | 0.419 |
| walker |  | 494 | 172 | Toml::Identity { file: Cargo.toml } |  |  | 0.811 |
| walker |  | 529 | 35 | Toml::Operational { file: Cargo.toml } |  |  | 0.811 |
| ns | 549 |  | 78 | src/output/, src/parameter/, src/timer/, src/util/ listings (complete) | 1.8 |  | 0.830 |
| walker |  | 550 | 21 | Fs::DirListing { dir: tests } |  |  | 0.833 |
| walker |  | 632 | 82 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.833 |
| ns | 643 |  | 94 | src/main.rs: module declarations and the types the entry point imports | 1.9 |  | 0.772 |
| walker |  | 650 | 18 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.772 |
| walker |  | 790 | 140 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: true } |  |  | 0.861 |
| walker |  | 817 | 27 | Markdown::Section { file: README.md, section_index: 11, keeps_default_concavity: false } |  |  | 0.861 |
| ns | 899 |  | 256 | src/main.rs: the run() pipeline | 1.10 | 1.9 | 0.754 |
| walker |  | 914 | 97 | Code::CodeKey { rung: ModuleDoc, file: src/outlier_detection.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.754 |
| walker |  | 954 | 40 | Markdown::Section { file: README.md, section_index: 14, keeps_default_concavity: false } |  |  | 0.754 |
| ns | 983 |  | 84 | src/main.rs: main() error reporting and exit code | 1.11 | 1.10 | 0.721 |
| walker |  | 1074 | 120 | Code::CodeKey { rung: Names, file: src/main.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.785 |
| ns | 1098 |  | 115 | tests/, doc/, scripts/ and .github/ listings (complete) | 1.12 |  | 0.744 |
| walker |  | 1150 | 76 | Code::CodeKey { rung: Decl, file: src/main.rs, decl: 2, sub: 0, line: 53 } |  |  | 0.787 |
| ns | 1235 |  | 137 | src/cli.rs: get_cli_arguments() and the build_command() clap header | 2.1 |  | 0.753 |
| walker |  | 1393 | 243 | Code::CodeKey { rung: Decl, file: src/main.rs, decl: 1, sub: 0, line: 29 } |  |  | 0.855 |
| walker |  | 1449 | 56 | Fs::DirListing { dir: scripts } |  |  | 0.912 |
| walker |  | 1461 | 12 | Code::CodeKey { rung: Names, file: build.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.912 |
| walker |  | 1488 | 27 | Fs::DirListing { dir: src/benchmark } |  |  | 0.958 |
| walker |  | 1558 | 70 | Markdown::Section { file: README.md, section_index: 13, keeps_default_concavity: false } |  |  | 0.958 |
| ns | 1672 |  | 437 | Complete option roster: every Arg::new(...) in src/cli.rs | 2.2 | 2.1 | 0.861 |
| walker |  | 1797 | 239 | Code::CodeKey { rung: Names, file: src/options.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.862 |
| walker |  | 1804 | 7 | Code::CodeKey { rung: Decl, file: src/options.rs, decl: 2, sub: 0, line: 19 } |  |  | 0.862 |
| walker |  | 1814 | 10 | Code::CodeKey { rung: Decl, file: src/options.rs, decl: 1, sub: 0, line: 16 } |  |  | 0.862 |
| walker |  | 1830 | 16 | Code::CodeKey { rung: Decl, file: src/options.rs, decl: 4, sub: 0, line: 32 } |  |  | 0.862 |
| walker |  | 1846 | 16 | Code::CodeKey { rung: Decl, file: src/options.rs, decl: 27, sub: 0, line: 251 } |  |  | 0.862 |
| ns | 1862 |  | 190 | Complete short-alias roster for the CLI options | 2.3 | 2.2 | 0.822 |
| walker |  | 1864 | 18 | Code::CodeKey { rung: Decl, file: src/options.rs, decl: 15, sub: 0, line: 116 } |  |  | 0.822 |
| walker |  | 1882 | 18 | Code::CodeKey { rung: Decl, file: src/options.rs, decl: 24, sub: 0, line: 191 } |  |  | 0.822 |
| walker |  | 1911 | 29 | Code::CodeKey { rung: Decl, file: src/options.rs, decl: 18, sub: 0, line: 132 } |  |  | 0.822 |
| walker |  | 1941 | 30 | Code::CodeKey { rung: Decl, file: src/options.rs, decl: 6, sub: 0, line: 38 } |  |  | 0.822 |
| walker |  | 1974 | 33 | Code::CodeKey { rung: Decl, file: src/options.rs, decl: 21, sub: 0, line: 164 } |  |  | 0.822 |
| walker |  | 2011 | 37 | Code::CodeKey { rung: Decl, file: src/options.rs, decl: 13, sub: 0, line: 101 } |  |  | 0.822 |
| ns | 2016 |  | 154 | Enumerated value sets, defaults, and the two hidden options | 2.4 | 2.2 | 0.799 |
| walker |  | 2050 | 39 | Code::CodeKey { rung: Decl, file: src/options.rs, decl: 23, sub: 0, line: 184 } |  |  | 0.801 |
| walker |  | 2103 | 53 | Code::CodeKey { rung: Decl, file: src/options.rs, decl: 8, sub: 0, line: 47 } |  |  | 0.801 |
| ns | 2146 |  | 130 | Complete set of inter-option conflicts and requirements | 2.5 | 2.2 | 0.784 |
| walker |  | 2158 | 55 | Code::CodeKey { rung: Decl, file: src/options.rs, decl: 14, sub: 0, line: 108 } |  |  | 0.784 |
| walker |  | 2221 | 63 | Code::CodeKey { rung: Decl, file: src/options.rs, decl: 29, sub: 0, line: 275 } |  |  | 0.784 |
| walker |  | 2285 | 64 | Code::CodeKey { rung: Decl, file: src/options.rs, decl: 3, sub: 0, line: 23 } |  |  | 0.786 |
| walker |  | 2354 | 69 | Code::CodeKey { rung: Decl, file: src/options.rs, decl: 17, sub: 0, line: 122 } |  |  | 0.787 |
| ns | 2366 |  | 220 | src/options.rs: the Options struct, run bounds through setup_command | 3.1 |  | 0.756 |
| walker |  | 2450 | 96 | Code::CodeKey { rung: Decl, file: src/options.rs, decl: 11, sub: 0, line: 70 } |  |  | 0.758 |
| ns | 2577 |  | 211 | src/options.rs: remaining Options fields (output, sort, executor, I/O, time unit) | 3.2 | 3.1 | 0.733 |
| walker |  | 2578 | 128 | Code::CodeKey { rung: Decl, file: src/options.rs, decl: 20, sub: 0, line: 148 } |  |  | 0.736 |
| walker |  | 2727 | 149 | Code::CodeKey { rung: Decl, file: src/options.rs, decl: 12, sub: 0, line: 83 } |  |  | 0.740 |
| walker |  | 2735 | 8 | Code::CodeKey { rung: Doc, file: src/options.rs, decl: 12, sub: 0, line: 83 } |  |  | 0.741 |
| walker |  | 2746 | 11 | Code::CodeKey { rung: Doc, file: src/options.rs, decl: 14, sub: 0, line: 108 } |  |  | 0.741 |
| walker |  | 2758 | 12 | Code::CodeKey { rung: Doc, file: src/options.rs, decl: 3, sub: 0, line: 23 } |  |  | 0.742 |
| walker |  | 2770 | 12 | Code::CodeKey { rung: Doc, file: src/options.rs, decl: 9, sub: 0, line: 49 } |  |  | 0.742 |
| walker |  | 2782 | 12 | Code::CodeKey { rung: Doc, file: src/options.rs, decl: 11, sub: 0, line: 70 } |  |  | 0.742 |
| walker |  | 2795 | 13 | Code::CodeKey { rung: Doc, file: src/options.rs, decl: 20, sub: 0, line: 148 } |  |  | 0.743 |
| ns | 2839 |  | 262 | src/options.rs: impl Default for Options -- the concrete default values | 3.3 | 3.1 | 0.706 |
| ns | 2991 |  | 152 | src/options.rs: DEFAULT_SHELL per platform, the Shell enum and its parser | 3.4 |  | 0.713 |
| ns | 3094 |  | 103 | src/options.rs: ExecutorKind (Raw / Shell / Mock) and its default | 3.5 | 3.4 | 0.711 |
| walker |  | 3268 | 473 | Toml::Dependencies { file: Cargo.toml } |  |  | 0.711 |
| ns | 3318 |  | 224 | src/options.rs: OutputStyleOption and SortOrder vocabularies | 3.6 |  | 0.726 |
| walker |  | 3332 | 64 | Code::CodeKey { rung: Names, file: src/error.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.726 |
| walker |  | 3356 | 24 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 4, sub: 0, line: 30 } |  |  | 0.726 |
| walker |  | 3381 | 25 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 2, sub: 0, line: 24 } |  |  | 0.726 |
| ns | 3560 |  | 242 | src/options.rs: CommandInputPolicy and CommandOutputPolicy | 3.7 |  | 0.741 |
| walker |  | 3626 | 245 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 1, sub: 0, line: 6 } |  |  | 0.744 |
| walker |  | 3640 | 14 | Code::CodeKey { rung: Body, file: src/error.rs, decl: 3, sub: 0, line: 25 } |  |  | 0.744 |
| ns | 3800 |  | 240 | src/options.rs: CmdFailureAction and RunBounds (default min = 10) | 3.8 |  | 0.749 |
| walker |  | 4011 | 371 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 6, sub: 0, line: 36 } |  |  | 0.753 |
| walker |  | 4036 | 25 | Code::CodeKey { rung: Names, file: src/output/mod.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.753 |
| walker |  | 4119 | 83 | Code::CodeKey { rung: Names, file: src/command.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.753 |
| walker |  | 4151 | 32 | Code::CodeKey { rung: Decl, file: src/command.rs, decl: 16, sub: 0, line: 387 } |  |  | 0.753 |
| ns | 4189 |  | 389 | src/error.rs: the complete OptionsError variant set with messages | 3.10 |  | 0.762 |
| walker |  | 4230 | 79 | Code::CodeKey { rung: Decl, file: src/command.rs, decl: 12, sub: 0, line: 136 } |  |  | 0.762 |
| walker |  | 4341 | 111 | Code::CodeKey { rung: Decl, file: src/command.rs, decl: 1, sub: 0, line: 21 } |  |  | 0.762 |
| ns | 4407 |  | 218 | src/error.rs: the complete ParameterScanError variant set | 3.11 | 3.10 | 0.769 |
| walker |  | 4523 | 182 | Code::CodeKey { rung: Decl, file: src/command.rs, decl: 2, sub: 0, line: 33 } |  |  | 0.769 |
| walker |  | 4578 | 55 | Code::CodeKey { rung: Decl, file: src/command.rs, decl: 4, sub: 0, line: 42 } |  |  | 0.769 |
| walker |  | 4591 | 13 | Code::CodeKey { rung: Doc, file: src/command.rs, decl: 1, sub: 0, line: 21 } |  |  | 0.770 |
| walker |  | 4604 | 13 | Code::CodeKey { rung: Doc, file: src/command.rs, decl: 11, sub: 0, line: 134 } |  |  | 0.770 |
| ns | 4627 |  | 220 | src/benchmark/executor.rs: the Executor trait | 4.1 |  | 0.751 |
| walker |  | 4648 | 44 | Code::CodeKey { rung: Names, file: src/util/mod.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.751 |
| walker |  | 4690 | 42 | Code::CodeKey { rung: Names, file: src/util/units.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.751 |
| walker |  | 4732 | 42 | Code::CodeKey { rung: Decl, file: src/util/units.rs, decl: 4, sub: 0, line: 16 } |  |  | 0.751 |
| ns | 4752 |  | 125 | The three Executor implementations and their state | 4.2 | 4.1 | 0.738 |
| walker |  | 4775 | 43 | Code::CodeKey { rung: Decl, file: src/util/units.rs, decl: 3, sub: 0, line: 9 } |  |  | 0.739 |
| walker |  | 4782 | 7 | Code::CodeKey { rung: Doc, file: src/util/units.rs, decl: 3, sub: 0, line: 9 } |  |  | 0.739 |
| walker |  | 4792 | 10 | Code::CodeKey { rung: Doc, file: src/util/units.rs, decl: 2, sub: 0, line: 6 } |  |  | 0.739 |
| walker |  | 4802 | 10 | Code::CodeKey { rung: Doc, file: src/util/units.rs, decl: 5, sub: 0, line: 18 } |  |  | 0.739 |
| walker |  | 4843 | 41 | Code::CodeKey { rung: Names, file: src/cli.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.739 |
| walker |  | 4885 | 42 | Code::CodeKey { rung: Decl, file: src/cli.rs, decl: 1, sub: 0, line: 8 } |  |  | 0.740 |
| walker |  | 4900 | 15 | Code::CodeKey { rung: Doc, file: src/cli.rs, decl: 2, sub: 0, line: 18 } |  |  | 0.741 |
| ns | 4904 |  | 152 | BenchmarkIteration and the $HYPERFINE_ITERATION values | 4.3 |  | 0.727 |
| walker |  | 4922 | 22 | Code::CodeKey { rung: Body, file: src/cli.rs, decl: 1, sub: 0, line: 8 } |  |  | 0.727 |
| walker |  | 5061 | 139 | Code::CodeKey { rung: Names, file: src/export/mod.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.727 |
| ns | 5072 |  | 168 | The shared command runner: stdio wiring and injected environment variables | 4.4 | 4.3 | 0.715 |
| walker |  | 5076 | 15 | Code::CodeKey { rung: Decl, file: src/export/mod.rs, decl: 2, sub: 0, line: 46 } |  |  | 0.715 |
| walker |  | 5095 | 19 | Code::CodeKey { rung: Decl, file: src/export/mod.rs, decl: 4, sub: 0, line: 56 } |  |  | 0.715 |
| walker |  | 5123 | 28 | Code::CodeKey { rung: Decl, file: src/export/mod.rs, decl: 5, sub: 0, line: 61 } |  |  | 0.715 |
| walker |  | 5161 | 38 | Code::CodeKey { rung: Decl, file: src/export/mod.rs, decl: 6, sub: 0, line: 67 } |  |  | 0.715 |
| walker |  | 5211 | 50 | Code::CodeKey { rung: Decl, file: src/export/mod.rs, decl: 3, sub: 0, line: 48 } |  |  | 0.716 |
| ns | 5284 |  | 212 | Non-zero exit handling and the failure message users actually see | 4.5 | 4.4 | 0.703 |
| walker |  | 5290 | 79 | Code::CodeKey { rung: Decl, file: src/export/mod.rs, decl: 7, sub: 0, line: 73 } |  |  | 0.703 |
| walker |  | 5334 | 44 | Code::CodeKey { rung: Decl, file: src/export/mod.rs, decl: 8, sub: 0, line: 76 } |  |  | 0.703 |
| walker |  | 5462 | 128 | Code::CodeKey { rung: Decl, file: src/export/mod.rs, decl: 1, sub: 0, line: 27 } |  |  | 0.704 |
| walker |  | 5470 | 8 | Code::CodeKey { rung: Doc, file: src/export/mod.rs, decl: 2, sub: 0, line: 46 } |  |  | 0.704 |
| walker |  | 5481 | 11 | Code::CodeKey { rung: Doc, file: src/export/mod.rs, decl: 6, sub: 0, line: 67 } |  |  | 0.694 |
| ns | 5481 |  | 197 | Shell-spawning calibration: 50 probe runs, and where the overhead is subtracted | 4.6 | 4.2 | 0.694 |
| walker |  | 5494 | 13 | Code::CodeKey { rung: Doc, file: src/export/mod.rs, decl: 3, sub: 0, line: 48 } |  |  | 0.694 |
| ns | 5679 |  | 198 | src/benchmark/scheduler.rs: Scheduler state and executor selection | 4.7 |  | 0.684 |
| walker |  | 5738 | 244 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.684 |
| ns | 5933 |  | 254 | run_benchmarks(): reference command first, calibrate once, export after each benchmark | 4.8 | 4.7 | 0.668 |
| walker |  | 5963 | 225 | Code::CodeKey { rung: Decl, file: src/options.rs, decl: 26, sub: 0, line: 198 } |  |  | 0.676 |
| walker |  | 5976 | 13 | Code::CodeKey { rung: Doc, file: src/options.rs, decl: 26, sub: 0, line: 198 } |  |  | 0.679 |
| walker |  | 6161 | 185 | Code::CodeKey { rung: Decl, file: src/options.rs, decl: 26, sub: 1, line: 198 } |  |  | 0.695 |
| ns | 6208 |  | 275 | src/benchmark/mod.rs: Benchmark struct, MIN_EXECUTION_TIME, and the complete method roster | 4.10 |  | 0.680 |
| walker |  | 6324 | 163 | Code::CodeKey { rung: Decl, file: src/options.rs, decl: 26, sub: 2, line: 198 } |  |  | 0.696 |
| walker |  | 6339 | 15 | Code::CodeKey { rung: Doc, file: src/export/mod.rs, decl: 9, sub: 0, line: 103 } |  |  | 0.696 |
| walker |  | 6354 | 15 | Code::CodeKey { rung: Doc, file: src/export/mod.rs, decl: 11, sub: 0, line: 158 } |  |  | 0.696 |
| walker |  | 6369 | 15 | Code::CodeKey { rung: Doc, file: src/util/units.rs, decl: 6, sub: 0, line: 27 } |  |  | 0.696 |
| walker |  | 6383 | 14 | Code::CodeKey { rung: Body, file: src/error.rs, decl: 5, sub: 0, line: 31 } |  |  | 0.696 |
| ns | 6418 |  | 210 | How the number of runs is decided | 4.11 | 4.10 | 0.682 |
| walker |  | 6585 | 202 | Markdown::Section { file: README.md, section_index: 12, keeps_default_concavity: false } |  |  | 0.682 |
| walker |  | 6610 | 25 | Code::CodeKey { rung: Names, file: src/timer/wall_clock_timer.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.682 |
| walker |  | 6622 | 12 | Code::CodeKey { rung: Decl, file: src/timer/wall_clock_timer.rs, decl: 1, sub: 0, line: 5 } |  |  | 0.682 |
| ns | 6626 |  | 208 | The three warning triggers inside Benchmark::run | 4.12 | 4.10 | 0.673 |
| walker |  | 6657 | 35 | Code::CodeKey { rung: Decl, file: src/timer/wall_clock_timer.rs, decl: 2, sub: 0, line: 9 } |  |  | 0.673 |
| walker |  | 6773 | 116 | Code::CodeKey { rung: Names, file: src/timer/mod.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.673 |
| ns | 6779 |  | 153 | src/outlier_detection.rs: the modified Z-score method and its threshold | 4.13 | 4.12 | 0.667 |
| walker |  | 6860 | 87 | Code::CodeKey { rung: Decl, file: src/timer/mod.rs, decl: 2, sub: 0, line: 41 } |  |  | 0.667 |
| walker |  | 6984 | 124 | Code::CodeKey { rung: Decl, file: src/timer/mod.rs, decl: 1, sub: 0, line: 27 } |  |  | 0.667 |
| walker |  | 6996 | 12 | Code::CodeKey { rung: Doc, file: src/timer/mod.rs, decl: 3, sub: 0, line: 52 } |  |  | 0.667 |
| walker |  | 7009 | 13 | Code::CodeKey { rung: Doc, file: src/timer/mod.rs, decl: 2, sub: 0, line: 41 } |  |  | 0.667 |
| walker |  | 7024 | 15 | Code::CodeKey { rung: Doc, file: src/timer/mod.rs, decl: 4, sub: 0, line: 83 } |  |  | 0.667 |
| walker |  | 7074 | 50 | Code::CodeKey { rung: Names, file: src/outlier_detection.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.668 |
| ns | 7075 |  | 296 | src/benchmark/benchmark_result.rs: the complete BenchmarkResult field set (= the JSON export schema) | 5.1 |  | 0.655 |
| ns | 7215 |  | 140 | src/benchmark/timing_result.rs: TimingResult in full | 5.2 | 5.1 | 0.647 |
| walker |  | 7324 | 250 | Markdown::Section { file: README.md, section_index: 7, keeps_default_concavity: false } |  |  | 0.647 |
| ns | 7378 |  | 163 | src/benchmark/relative_speed.rs: annotated result type and the complete public function set | 5.3 |  | 0.639 |
| walker |  | 7535 | 211 | Markdown::Section { file: README.md, section_index: 8, keeps_default_concavity: false } |  |  | 0.639 |
| ns | 7536 |  | 158 | src/export/mod.rs: the ExportType enum -- the five output formats | 5.4 |  | 0.643 |
| walker |  | 7601 | 66 | Code::CodeKey { rung: Names, file: src/parameter/mod.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.643 |
| walker |  | 7633 | 32 | Code::CodeKey { rung: Decl, file: src/parameter/mod.rs, decl: 1, sub: 0, line: 7 } |  |  | 0.643 |
| walker |  | 7667 | 34 | Code::CodeKey { rung: Decl, file: src/parameter/mod.rs, decl: 2, sub: 0, line: 13 } |  |  | 0.643 |
| ns | 7698 |  | 162 | src/export/mod.rs: the Exporter trait and ExportManager | 5.5 | 5.4 | 0.650 |
| walker |  | 7707 | 40 | Code::CodeKey { rung: Names, file: src/output/warnings.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.650 |
| walker |  | 7737 | 30 | Code::CodeKey { rung: Decl, file: src/output/warnings.rs, decl: 1, sub: 0, line: 7 } |  |  | 0.650 |
| walker |  | 7767 | 30 | Code::CodeKey { rung: Decl, file: src/output/warnings.rs, decl: 3, sub: 0, line: 20 } |  |  | 0.650 |
| walker |  | 7819 | 52 | Code::CodeKey { rung: Decl, file: src/output/warnings.rs, decl: 2, sub: 0, line: 13 } |  |  | 0.651 |
| walker |  | 7829 | 10 | Code::CodeKey { rung: Doc, file: src/output/warnings.rs, decl: 2, sub: 0, line: 13 } |  |  | 0.651 |
| ns | 7835 |  | 137 | Flag-to-exporter wiring and the '-' means stdout convention | 5.6 | 5.5 | 0.646 |
| walker |  | 7846 | 17 | Code::CodeKey { rung: Doc, file: src/export/mod.rs, decl: 1, sub: 0, line: 27 } |  |  | 0.651 |
| walker |  | 7914 | 68 | Code::CodeKey { rung: Names, file: src/util/number.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.651 |
| walker |  | 7933 | 19 | Code::CodeKey { rung: Decl, file: src/util/number.rs, decl: 6, sub: 0, line: 30 } |  |  | 0.651 |
| walker |  | 7953 | 20 | Code::CodeKey { rung: Decl, file: src/util/number.rs, decl: 4, sub: 0, line: 24 } |  |  | 0.651 |
| walker |  | 7983 | 30 | Code::CodeKey { rung: Decl, file: src/util/number.rs, decl: 2, sub: 0, line: 15 } |  |  | 0.651 |
| walker |  | 8024 | 41 | Code::CodeKey { rung: Decl, file: src/util/number.rs, decl: 8, sub: 0, line: 36 } |  |  | 0.651 |
| ns | 8051 |  | 216 | src/export/markup.rs: the shared table shape and the blanket Exporter impl | 5.7 | 5.5 | 0.643 |
| walker |  | 8075 | 51 | Code::CodeKey { rung: Decl, file: src/util/number.rs, decl: 1, sub: 0, line: 8 } |  |  | 0.644 |
| walker |  | 8084 | 9 | Code::CodeKey { rung: Body, file: src/command.rs, decl: 9, sub: 0, line: 96 } |  |  | 0.644 |
| walker |  | 8127 | 43 | Code::CodeKey { rung: Names, file: src/export/json.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.644 |
| walker |  | 8135 | 8 | Code::CodeKey { rung: Decl, file: src/export/json.rs, decl: 2, sub: 0, line: 16 } |  |  | 0.644 |
| walker |  | 8148 | 13 | Code::CodeKey { rung: Decl, file: src/export/json.rs, decl: 3, sub: 0, line: 19 } |  |  | 0.644 |
| walker |  | 8175 | 27 | Code::CodeKey { rung: Decl, file: src/export/json.rs, decl: 1, sub: 0, line: 11 } |  |  | 0.644 |
| walker |  | 8230 | 55 | Code::CodeKey { rung: Decl, file: src/export/json.rs, decl: 4, sub: 0, line: 20 } |  |  | 0.644 |
| walker |  | 8259 | 29 | Code::CodeKey { rung: Names, file: src/export/csv.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.644 |
| walker |  | 8267 | 8 | Code::CodeKey { rung: Decl, file: src/export/csv.rs, decl: 1, sub: 0, line: 12 } |  |  | 0.637 |
| ns | 8267 |  | 216 | The five exporter types, and the CSV column set | 5.8 | 5.4 | 0.637 |
| walker |  | 8282 | 15 | Code::CodeKey { rung: Decl, file: src/export/csv.rs, decl: 2, sub: 0, line: 15 } |  |  | 0.637 |
| walker |  | 8337 | 55 | Code::CodeKey { rung: Decl, file: src/export/csv.rs, decl: 3, sub: 0, line: 16 } |  |  | 0.637 |
| walker |  | 8367 | 30 | Code::CodeKey { rung: Names, file: src/export/markdown.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.638 |
| walker |  | 8375 | 8 | Code::CodeKey { rung: Decl, file: src/export/markdown.rs, decl: 1, sub: 0, line: 5 } |  |  | 0.638 |
| walker |  | 8448 | 73 | Code::CodeKey { rung: Decl, file: src/export/markdown.rs, decl: 2, sub: 0, line: 8 } |  |  | 0.638 |
| walker |  | 8460 | 12 | Code::CodeKey { rung: Body, file: src/export/markdown.rs, decl: 5, sub: 0, line: 26 } |  |  | 0.638 |
| walker |  | 8477 | 17 | Code::CodeKey { rung: Body, file: src/export/markdown.rs, decl: 3, sub: 0, line: 9 } |  |  | 0.638 |
| walker |  | 8492 | 15 | Code::CodeKey { rung: Names, file: src/util/randomized_environment_offset.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.638 |
| ns | 8524 |  | 257 | src/output/format.rs: automatic time-unit selection, and the Unit type | 5.9 |  | 0.631 |
| walker |  | 8554 | 62 | Code::CodeKey { rung: Names, file: src/export/markup.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.631 |
| walker |  | 8567 | 13 | Code::CodeKey { rung: Decl, file: src/export/markup.rs, decl: 10, sub: 0, line: 108 } |  |  | 0.631 |
| walker |  | 8584 | 17 | Code::CodeKey { rung: Decl, file: src/export/markup.rs, decl: 1, sub: 0, line: 10 } |  |  | 0.631 |
| walker |  | 8637 | 53 | Code::CodeKey { rung: Decl, file: src/export/markup.rs, decl: 11, sub: 0, line: 109 } |  |  | 0.631 |
| ns | 8661 |  | 137 | src/output/warnings.rs: the complete Warnings set | 5.10 | 4.12 | 0.636 |
| walker |  | 8793 | 156 | Code::CodeKey { rung: Decl, file: src/export/markup.rs, decl: 2, sub: 0, line: 15 } |  |  | 0.641 |
| walker |  | 8802 | 9 | Code::CodeKey { rung: Body, file: src/export/markup.rs, decl: 6, sub: 0, line: 87 } |  |  | 0.641 |
| walker |  | 8811 | 9 | Code::CodeKey { rung: Body, file: src/export/markup.rs, decl: 7, sub: 0, line: 91 } |  |  | 0.641 |
| ns | 8869 |  | 208 | src/command.rs: the Command type and its complete method roster | 6.1 |  | 0.646 |
| ns | 9015 |  | 146 | How {param} placeholders are substituted, and why naively | 6.2 | 6.1 | 0.641 |
| walker |  | 9138 | 327 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.641 |
| ns | 9202 |  | 187 | src/command.rs: Commands and its complete method roster, with the three construction modes | 6.3 |  | 0.638 |
| walker |  | 9218 | 80 | Code::CodeKey { rung: Names, file: src/timer/unix_timer.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.638 |
| walker |  | 9233 | 15 | Code::CodeKey { rung: Decl, file: src/timer/unix_timer.rs, decl: 2, sub: 0, line: 18 } |  |  | 0.638 |
| walker |  | 9275 | 42 | Code::CodeKey { rung: Decl, file: src/timer/unix_timer.rs, decl: 3, sub: 0, line: 22 } |  |  | 0.638 |
| walker |  | 9347 | 72 | Code::CodeKey { rung: Decl, file: src/timer/unix_timer.rs, decl: 1, sub: 0, line: 9 } |  |  | 0.638 |
| walker |  | 9361 | 14 | Code::CodeKey { rung: Doc, file: src/timer/unix_timer.rs, decl: 6, sub: 0, line: 41 } |  |  | 0.638 |
| walker |  | 9380 | 19 | Code::CodeKey { rung: Doc, file: src/timer/unix_timer.rs, decl: 7, sub: 0, line: 71 } |  |  | 0.638 |
| walker |  | 9412 | 32 | Code::CodeKey { rung: Names, file: src/export/orgmode.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.638 |
| walker |  | 9420 | 8 | Code::CodeKey { rung: Decl, file: src/export/orgmode.rs, decl: 1, sub: 0, line: 4 } |  |  | 0.638 |
| ns | 9431 |  | 229 | src/parameter/: ParameterValue, RangeStep and its 100_000 cap, tokenize() | 6.4 |  | 0.634 |
| walker |  | 9493 | 73 | Code::CodeKey { rung: Decl, file: src/export/orgmode.rs, decl: 2, sub: 0, line: 7 } |  |  | 0.634 |
| walker |  | 9505 | 12 | Code::CodeKey { rung: Body, file: src/export/orgmode.rs, decl: 5, sub: 0, line: 20 } |  |  | 0.634 |
| walker |  | 9526 | 21 | Code::CodeKey { rung: Body, file: src/util/randomized_environment_offset.rs, decl: 1, sub: 0, line: 6 } |  |  | 0.634 |
| walker |  | 9536 | 10 | Code::CodeKey { rung: Body, file: src/util/number.rs, decl: 5, sub: 0, line: 25 } |  |  | 0.634 |
| ns | 9581 |  | 150 | build.rs: shell completions generated from the same clap command | 7.1 | 2.1 | 0.628 |
| walker |  | 9623 | 87 | Code::CodeKey { rung: Names, file: src/timer/windows_timer.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.628 |
| walker |  | 9636 | 13 | Code::CodeKey { rung: Decl, file: src/timer/windows_timer.rs, decl: 3, sub: 0, line: 49 } |  |  | 0.628 |
| walker |  | 9653 | 17 | Code::CodeKey { rung: Decl, file: src/timer/windows_timer.rs, decl: 7, sub: 0, line: 124 } |  |  | 0.628 |
| walker |  | 9703 | 50 | Code::CodeKey { rung: Decl, file: src/timer/windows_timer.rs, decl: 4, sub: 0, line: 53 } |  |  | 0.628 |
| ns | 9790 |  | 209 | .github/workflows/CICD.yml: the complete job set and the commands each runs | 7.2 |  | 0.621 |
| walker |  | 9899 | 196 | Code::CodeKey { rung: Decl, file: src/timer/windows_timer.rs, decl: 2, sub: 0, line: 34 } |  |  | 0.621 |
| walker |  | 9970 | 71 | Code::CodeKey { rung: Names, file: src/parameter/range_step.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.622 |
| ns | 9982 |  | 192 | tests/: the shared harness helpers and the debug-mode test idiom | 7.3 |  | 0.616 |
