Score(3000)=0.668 I=0.893 C=0.500 ns_rows≤3K=21/56 grid(1000/1442/2080/3000/4327/6240/9000)=0.748/0.912/0.798/0.668/0.558/0.632/0.631

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
| walker |  | 772 | 140 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: true } |  |  | 0.861 |
| walker |  | 799 | 27 | Markdown::Section { file: README.md, section_index: 11, keeps_default_concavity: false } |  |  | 0.861 |
| walker |  | 896 | 97 | Code::CodeKey { rung: ModuleDoc, file: src/outlier_detection.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.861 |
| ns | 899 |  | 256 | src/main.rs: the run() pipeline | 1.10 | 1.9 | 0.754 |
| walker |  | 936 | 40 | Markdown::Section { file: README.md, section_index: 14, keeps_default_concavity: false } |  |  | 0.754 |
| ns | 983 |  | 84 | src/main.rs: main() error reporting and exit code | 1.11 | 1.10 | 0.721 |
| walker |  | 1056 | 120 | Code::CodeKey { rung: Names, file: src/main.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.785 |
| ns | 1098 |  | 115 | tests/, doc/, scripts/ and .github/ listings (complete) | 1.12 |  | 0.744 |
| walker |  | 1132 | 76 | Code::CodeKey { rung: Decl, file: src/main.rs, decl: 2, sub: 0, line: 53 } |  |  | 0.787 |
| ns | 1235 |  | 137 | src/cli.rs: get_cli_arguments() and the build_command() clap header | 2.1 |  | 0.753 |
| walker |  | 1375 | 243 | Code::CodeKey { rung: Decl, file: src/main.rs, decl: 1, sub: 0, line: 29 } |  |  | 0.855 |
| walker |  | 1431 | 56 | Fs::DirListing { dir: scripts } |  |  | 0.912 |
| walker |  | 1443 | 12 | Code::CodeKey { rung: Names, file: build.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.912 |
| walker |  | 1470 | 27 | Fs::DirListing { dir: src/benchmark } |  |  | 0.958 |
| walker |  | 1540 | 70 | Markdown::Section { file: README.md, section_index: 13, keeps_default_concavity: false } |  |  | 0.958 |
| ns | 1672 |  | 437 | Complete option roster: every Arg::new(...) in src/cli.rs | 2.2 | 2.1 | 0.861 |
| ns | 1862 |  | 190 | Complete short-alias roster for the CLI options | 2.3 | 2.2 | 0.821 |
| walker |  | 2013 | 473 | Toml::Dependencies { file: Cargo.toml } |  |  | 0.821 |
| ns | 2016 |  | 154 | Enumerated value sets, defaults, and the two hidden options | 2.4 | 2.2 | 0.798 |
| walker |  | 2096 | 83 | Code::CodeKey { rung: Names, file: src/command.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.798 |
| walker |  | 2128 | 32 | Code::CodeKey { rung: Decl, file: src/command.rs, decl: 16, sub: 0, line: 387 } |  |  | 0.798 |
| ns | 2146 |  | 130 | Complete set of inter-option conflicts and requirements | 2.5 | 2.2 | 0.781 |
| walker |  | 2207 | 79 | Code::CodeKey { rung: Decl, file: src/command.rs, decl: 12, sub: 0, line: 136 } |  |  | 0.781 |
| walker |  | 2318 | 111 | Code::CodeKey { rung: Decl, file: src/command.rs, decl: 1, sub: 0, line: 21 } |  |  | 0.781 |
| ns | 2366 |  | 220 | src/options.rs: the Options struct, run bounds through setup_command | 3.1 |  | 0.750 |
| walker |  | 2500 | 182 | Code::CodeKey { rung: Decl, file: src/command.rs, decl: 2, sub: 0, line: 33 } |  |  | 0.751 |
| walker |  | 2555 | 55 | Code::CodeKey { rung: Decl, file: src/command.rs, decl: 4, sub: 0, line: 42 } |  |  | 0.751 |
| walker |  | 2568 | 13 | Code::CodeKey { rung: Doc, file: src/command.rs, decl: 1, sub: 0, line: 21 } |  |  | 0.751 |
| ns | 2577 |  | 211 | src/options.rs: remaining Options fields (output, sort, executor, I/O, time unit) | 3.2 | 3.1 | 0.727 |
| walker |  | 2581 | 13 | Code::CodeKey { rung: Doc, file: src/command.rs, decl: 11, sub: 0, line: 134 } |  |  | 0.727 |
| walker |  | 2775 | 194 | Markdown::Section { file: README.md, section_index: 7, keeps_default_concavity: false } |  |  | 0.727 |
| walker |  | 2817 | 42 | Code::CodeKey { rung: Names, file: src/util/units.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.727 |
| ns | 2839 |  | 262 | src/options.rs: impl Default for Options -- the concrete default values | 3.3 | 3.1 | 0.690 |
| walker |  | 2859 | 42 | Code::CodeKey { rung: Decl, file: src/util/units.rs, decl: 4, sub: 0, line: 16 } |  |  | 0.690 |
| walker |  | 2902 | 43 | Code::CodeKey { rung: Decl, file: src/util/units.rs, decl: 3, sub: 0, line: 9 } |  |  | 0.691 |
| walker |  | 2909 | 7 | Code::CodeKey { rung: Doc, file: src/util/units.rs, decl: 3, sub: 0, line: 9 } |  |  | 0.691 |
| walker |  | 2919 | 10 | Code::CodeKey { rung: Doc, file: src/util/units.rs, decl: 2, sub: 0, line: 6 } |  |  | 0.691 |
| walker |  | 2929 | 10 | Code::CodeKey { rung: Doc, file: src/util/units.rs, decl: 5, sub: 0, line: 18 } |  |  | 0.691 |
| ns | 2991 |  | 152 | src/options.rs: DEFAULT_SHELL per platform, the Shell enum and its parser | 3.4 |  | 0.668 |
| ns | 3094 |  | 103 | src/options.rs: ExecutorKind (Raw / Shell / Mock) and its default | 3.5 | 3.4 | 0.653 |
| walker |  | 3140 | 211 | Markdown::Section { file: README.md, section_index: 8, keeps_default_concavity: false } |  |  | 0.653 |
| ns | 3318 |  | 224 | src/options.rs: OutputStyleOption and SortOrder vocabularies | 3.6 |  | 0.625 |
| walker |  | 3384 | 244 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.625 |
| ns | 3560 |  | 242 | src/options.rs: CommandInputPolicy and CommandOutputPolicy | 3.7 |  | 0.598 |
| walker |  | 3586 | 202 | Markdown::Section { file: README.md, section_index: 12, keeps_default_concavity: false } |  |  | 0.598 |
| walker |  | 3665 | 79 | Code::CodeKey { rung: Names, file: src/parameter/range_step.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.598 |
| walker |  | 3699 | 34 | Code::CodeKey { rung: Decl, file: src/parameter/range_step.rs, decl: 4, sub: 0, line: 40 } |  |  | 0.598 |
| walker |  | 3737 | 38 | Code::CodeKey { rung: Decl, file: src/parameter/range_step.rs, decl: 3, sub: 0, line: 33 } |  |  | 0.599 |
| walker |  | 3799 | 62 | Code::CodeKey { rung: Decl, file: src/parameter/range_step.rs, decl: 6, sub: 0, line: 62 } |  |  | 0.599 |
| ns | 3800 |  | 240 | src/options.rs: CmdFailureAction and RunBounds (default min = 10) | 3.8 |  | 0.577 |
| walker |  | 3896 | 97 | Code::CodeKey { rung: Decl, file: src/parameter/range_step.rs, decl: 1, sub: 0, line: 7 } |  |  | 0.577 |
| walker |  | 3993 | 97 | Code::CodeKey { rung: Decl, file: src/parameter/range_step.rs, decl: 2, sub: 0, line: 19 } |  |  | 0.577 |
| walker |  | 4033 | 40 | Code::CodeKey { rung: Names, file: src/output/warnings.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.577 |
| walker |  | 4063 | 30 | Code::CodeKey { rung: Decl, file: src/output/warnings.rs, decl: 1, sub: 0, line: 7 } |  |  | 0.577 |
| walker |  | 4093 | 30 | Code::CodeKey { rung: Decl, file: src/output/warnings.rs, decl: 3, sub: 0, line: 20 } |  |  | 0.577 |
| walker |  | 4145 | 52 | Code::CodeKey { rung: Decl, file: src/output/warnings.rs, decl: 2, sub: 0, line: 13 } |  |  | 0.578 |
| walker |  | 4155 | 10 | Code::CodeKey { rung: Doc, file: src/output/warnings.rs, decl: 2, sub: 0, line: 13 } |  |  | 0.578 |
| ns | 4189 |  | 389 | src/error.rs: the complete OptionsError variant set with messages | 3.10 |  | 0.558 |
| walker |  | 4223 | 68 | Code::CodeKey { rung: Names, file: src/util/number.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.558 |
| walker |  | 4242 | 19 | Code::CodeKey { rung: Decl, file: src/util/number.rs, decl: 6, sub: 0, line: 30 } |  |  | 0.558 |
| walker |  | 4262 | 20 | Code::CodeKey { rung: Decl, file: src/util/number.rs, decl: 4, sub: 0, line: 24 } |  |  | 0.558 |
| walker |  | 4292 | 30 | Code::CodeKey { rung: Decl, file: src/util/number.rs, decl: 2, sub: 0, line: 15 } |  |  | 0.558 |
| walker |  | 4333 | 41 | Code::CodeKey { rung: Decl, file: src/util/number.rs, decl: 8, sub: 0, line: 36 } |  |  | 0.558 |
| walker |  | 4384 | 51 | Code::CodeKey { rung: Decl, file: src/util/number.rs, decl: 1, sub: 0, line: 8 } |  |  | 0.559 |
| walker |  | 4399 | 15 | Code::CodeKey { rung: Doc, file: src/util/units.rs, decl: 6, sub: 0, line: 27 } |  |  | 0.559 |
| ns | 4407 |  | 218 | src/error.rs: the complete ParameterScanError variant set | 3.11 | 3.10 | 0.545 |
| walker |  | 4427 | 28 | Code::CodeKey { rung: Names, file: src/cli.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.546 |
| walker |  | 4469 | 42 | Code::CodeKey { rung: Decl, file: src/cli.rs, decl: 1, sub: 0, line: 8 } |  |  | 0.546 |
| ns | 4627 |  | 220 | src/benchmark/executor.rs: the Executor trait | 4.1 |  | 0.533 |
| walker |  | 4708 | 239 | Code::CodeKey { rung: Names, file: src/options.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.537 |
| walker |  | 4715 | 7 | Code::CodeKey { rung: Decl, file: src/options.rs, decl: 2, sub: 0, line: 19 } |  |  | 0.538 |
| walker |  | 4725 | 10 | Code::CodeKey { rung: Decl, file: src/options.rs, decl: 1, sub: 0, line: 16 } |  |  | 0.539 |
| walker |  | 4741 | 16 | Code::CodeKey { rung: Decl, file: src/options.rs, decl: 4, sub: 0, line: 32 } |  |  | 0.539 |
| ns | 4752 |  | 125 | The three Executor implementations and their state | 4.2 | 4.1 | 0.530 |
| walker |  | 4757 | 16 | Code::CodeKey { rung: Decl, file: src/options.rs, decl: 27, sub: 0, line: 251 } |  |  | 0.530 |
| walker |  | 4775 | 18 | Code::CodeKey { rung: Decl, file: src/options.rs, decl: 15, sub: 0, line: 116 } |  |  | 0.531 |
| walker |  | 4793 | 18 | Code::CodeKey { rung: Decl, file: src/options.rs, decl: 24, sub: 0, line: 191 } |  |  | 0.533 |
| walker |  | 4822 | 29 | Code::CodeKey { rung: Decl, file: src/options.rs, decl: 18, sub: 0, line: 132 } |  |  | 0.533 |
| walker |  | 4852 | 30 | Code::CodeKey { rung: Decl, file: src/options.rs, decl: 6, sub: 0, line: 38 } |  |  | 0.533 |
| walker |  | 4885 | 33 | Code::CodeKey { rung: Decl, file: src/options.rs, decl: 21, sub: 0, line: 164 } |  |  | 0.533 |
| ns | 4904 |  | 152 | BenchmarkIteration and the $HYPERFINE_ITERATION values | 4.3 |  | 0.523 |
| walker |  | 4922 | 37 | Code::CodeKey { rung: Decl, file: src/options.rs, decl: 13, sub: 0, line: 101 } |  |  | 0.525 |
| walker |  | 4961 | 39 | Code::CodeKey { rung: Decl, file: src/options.rs, decl: 23, sub: 0, line: 184 } |  |  | 0.535 |
| walker |  | 5014 | 53 | Code::CodeKey { rung: Decl, file: src/options.rs, decl: 8, sub: 0, line: 47 } |  |  | 0.535 |
| walker |  | 5069 | 55 | Code::CodeKey { rung: Decl, file: src/options.rs, decl: 14, sub: 0, line: 108 } |  |  | 0.541 |
| ns | 5072 |  | 168 | The shared command runner: stdio wiring and injected environment variables | 4.4 | 4.3 | 0.532 |
| walker |  | 5132 | 63 | Code::CodeKey { rung: Decl, file: src/options.rs, decl: 29, sub: 0, line: 275 } |  |  | 0.532 |
| walker |  | 5196 | 64 | Code::CodeKey { rung: Decl, file: src/options.rs, decl: 3, sub: 0, line: 23 } |  |  | 0.547 |
| walker |  | 5265 | 69 | Code::CodeKey { rung: Decl, file: src/options.rs, decl: 17, sub: 0, line: 122 } |  |  | 0.553 |
| ns | 5284 |  | 212 | Non-zero exit handling and the failure message users actually see | 4.5 | 4.4 | 0.543 |
| walker |  | 5361 | 96 | Code::CodeKey { rung: Decl, file: src/options.rs, decl: 11, sub: 0, line: 70 } |  |  | 0.561 |
| ns | 5481 |  | 197 | Shell-spawning calibration: 50 probe runs, and where the overhead is subtracted | 4.6 | 4.2 | 0.553 |
| walker |  | 5489 | 128 | Code::CodeKey { rung: Decl, file: src/options.rs, decl: 20, sub: 0, line: 148 } |  |  | 0.582 |
| walker |  | 5638 | 149 | Code::CodeKey { rung: Decl, file: src/options.rs, decl: 12, sub: 0, line: 83 } |  |  | 0.613 |
| walker |  | 5646 | 8 | Code::CodeKey { rung: Doc, file: src/options.rs, decl: 12, sub: 0, line: 83 } |  |  | 0.616 |
| walker |  | 5657 | 11 | Code::CodeKey { rung: Doc, file: src/options.rs, decl: 14, sub: 0, line: 108 } |  |  | 0.620 |
| walker |  | 5669 | 12 | Code::CodeKey { rung: Doc, file: src/options.rs, decl: 3, sub: 0, line: 23 } |  |  | 0.624 |
| ns | 5679 |  | 198 | src/benchmark/scheduler.rs: Scheduler state and executor selection | 4.7 |  | 0.615 |
| walker |  | 5681 | 12 | Code::CodeKey { rung: Doc, file: src/options.rs, decl: 9, sub: 0, line: 49 } |  |  | 0.615 |
| walker |  | 5693 | 12 | Code::CodeKey { rung: Doc, file: src/options.rs, decl: 11, sub: 0, line: 70 } |  |  | 0.619 |
| walker |  | 5706 | 13 | Code::CodeKey { rung: Doc, file: src/options.rs, decl: 20, sub: 0, line: 148 } |  |  | 0.624 |
| walker |  | 5931 | 225 | Code::CodeKey { rung: Decl, file: src/options.rs, decl: 26, sub: 0, line: 198 } |  |  | 0.633 |
| ns | 5933 |  | 254 | run_benchmarks(): reference command first, calibrate once, export after each benchmark | 4.8 | 4.7 | 0.618 |
| walker |  | 5944 | 13 | Code::CodeKey { rung: Doc, file: src/options.rs, decl: 26, sub: 0, line: 198 } |  |  | 0.620 |
| walker |  | 6129 | 185 | Code::CodeKey { rung: Decl, file: src/options.rs, decl: 26, sub: 1, line: 198 } |  |  | 0.637 |
| ns | 6208 |  | 275 | src/benchmark/mod.rs: Benchmark struct, MIN_EXECUTION_TIME, and the complete method roster | 4.10 |  | 0.623 |
| walker |  | 6292 | 163 | Code::CodeKey { rung: Decl, file: src/options.rs, decl: 26, sub: 2, line: 198 } |  |  | 0.640 |
| walker |  | 6314 | 22 | Code::CodeKey { rung: Body, file: src/cli.rs, decl: 1, sub: 0, line: 8 } |  |  | 0.640 |
| walker |  | 6364 | 50 | Code::CodeKey { rung: Names, file: src/outlier_detection.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.640 |
| ns | 6418 |  | 210 | How the number of runs is decided | 4.11 | 4.10 | 0.628 |
| walker |  | 6428 | 64 | Code::CodeKey { rung: Names, file: src/error.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.628 |
| walker |  | 6452 | 24 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 4, sub: 0, line: 30 } |  |  | 0.628 |
| walker |  | 6477 | 25 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 2, sub: 0, line: 24 } |  |  | 0.628 |
| ns | 6626 |  | 208 | The three warning triggers inside Benchmark::run | 4.12 | 4.10 | 0.619 |
| walker |  | 6722 | 245 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 1, sub: 0, line: 6 } |  |  | 0.641 |
| ns | 6779 |  | 153 | src/outlier_detection.rs: the modified Z-score method and its threshold | 4.13 | 4.12 | 0.636 |
| ns | 7075 |  | 296 | src/benchmark/benchmark_result.rs: the complete BenchmarkResult field set (= the JSON export schema) | 5.1 |  | 0.624 |
| walker |  | 7093 | 371 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 6, sub: 0, line: 36 } |  |  | 0.652 |
| walker |  | 7102 | 9 | Code::CodeKey { rung: Body, file: src/command.rs, decl: 9, sub: 0, line: 96 } |  |  | 0.652 |
| ns | 7215 |  | 140 | src/benchmark/timing_result.rs: TimingResult in full | 5.2 | 5.1 | 0.644 |
| ns | 7378 |  | 163 | src/benchmark/relative_speed.rs: annotated result type and the complete public function set | 5.3 |  | 0.636 |
| walker |  | 7429 | 327 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.636 |
| walker |  | 7439 | 10 | Code::CodeKey { rung: Body, file: src/util/number.rs, decl: 5, sub: 0, line: 25 } |  |  | 0.636 |
| walker |  | 7449 | 10 | Code::CodeKey { rung: Body, file: src/util/number.rs, decl: 7, sub: 0, line: 31 } |  |  | 0.636 |
| walker |  | 7536 | 87 | Code::CodeKey { rung: Names, file: src/timer/windows_timer.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.626 |
| ns | 7536 |  | 158 | src/export/mod.rs: the ExportType enum -- the five output formats | 5.4 |  | 0.626 |
| walker |  | 7549 | 13 | Code::CodeKey { rung: Decl, file: src/timer/windows_timer.rs, decl: 3, sub: 0, line: 49 } |  |  | 0.626 |
| walker |  | 7566 | 17 | Code::CodeKey { rung: Decl, file: src/timer/windows_timer.rs, decl: 7, sub: 0, line: 124 } |  |  | 0.626 |
| walker |  | 7616 | 50 | Code::CodeKey { rung: Decl, file: src/timer/windows_timer.rs, decl: 4, sub: 0, line: 53 } |  |  | 0.626 |
| ns | 7698 |  | 162 | src/export/mod.rs: the Exporter trait and ExportManager | 5.5 | 5.4 | 0.618 |
| walker |  | 7812 | 196 | Code::CodeKey { rung: Decl, file: src/timer/windows_timer.rs, decl: 2, sub: 0, line: 34 } |  |  | 0.618 |
| ns | 7835 |  | 137 | Flag-to-exporter wiring and the '-' means stdout convention | 5.6 | 5.5 | 0.613 |
| walker |  | 7913 | 101 | Code::CodeKey { rung: Names, file: src/timer/mod.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.613 |
| walker |  | 8000 | 87 | Code::CodeKey { rung: Decl, file: src/timer/mod.rs, decl: 2, sub: 0, line: 41 } |  |  | 0.613 |
| ns | 8051 |  | 216 | src/export/markup.rs: the shared table shape and the blanket Exporter impl | 5.7 | 5.5 | 0.606 |
| walker |  | 8124 | 124 | Code::CodeKey { rung: Decl, file: src/timer/mod.rs, decl: 1, sub: 0, line: 27 } |  |  | 0.606 |
| walker |  | 8137 | 13 | Code::CodeKey { rung: Doc, file: src/timer/mod.rs, decl: 2, sub: 0, line: 41 } |  |  | 0.606 |
| walker |  | 8173 | 36 | Code::CodeKey { rung: Names, file: src/timer/unix_timer.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.606 |
| walker |  | 8188 | 15 | Code::CodeKey { rung: Decl, file: src/timer/unix_timer.rs, decl: 2, sub: 0, line: 18 } |  |  | 0.606 |
| walker |  | 8230 | 42 | Code::CodeKey { rung: Decl, file: src/timer/unix_timer.rs, decl: 3, sub: 0, line: 22 } |  |  | 0.606 |
| ns | 8267 |  | 216 | The five exporter types, and the CSV column set | 5.8 | 5.4 | 0.599 |
| walker |  | 8302 | 72 | Code::CodeKey { rung: Decl, file: src/timer/unix_timer.rs, decl: 1, sub: 0, line: 9 } |  |  | 0.599 |
| walker |  | 8327 | 25 | Code::CodeKey { rung: Names, file: src/timer/wall_clock_timer.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.599 |
| walker |  | 8339 | 12 | Code::CodeKey { rung: Decl, file: src/timer/wall_clock_timer.rs, decl: 1, sub: 0, line: 5 } |  |  | 0.599 |
| walker |  | 8374 | 35 | Code::CodeKey { rung: Decl, file: src/timer/wall_clock_timer.rs, decl: 2, sub: 0, line: 9 } |  |  | 0.599 |
| walker |  | 8389 | 15 | Code::CodeKey { rung: Doc, file: src/timer/mod.rs, decl: 3, sub: 0, line: 83 } |  |  | 0.599 |
| walker |  | 8399 | 10 | Code::CodeKey { rung: Body, file: src/command.rs, decl: 14, sub: 0, line: 250 } |  |  | 0.599 |
| walker |  | 8417 | 18 | Code::CodeKey { rung: Names, file: src/export/tests.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.599 |
| walker |  | 8457 | 40 | Code::CodeKey { rung: Decl, file: src/export/tests.rs, decl: 1, sub: 0, line: 9 } |  |  | 0.599 |
| ns | 8524 |  | 257 | src/output/format.rs: automatic time-unit selection, and the Unit type | 5.9 |  | 0.593 |
| walker |  | 8571 | 114 | Code::CodeKey { rung: Names, file: src/export/mod.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.593 |
| walker |  | 8586 | 15 | Code::CodeKey { rung: Decl, file: src/export/mod.rs, decl: 2, sub: 0, line: 46 } |  |  | 0.594 |
| walker |  | 8605 | 19 | Code::CodeKey { rung: Decl, file: src/export/mod.rs, decl: 4, sub: 0, line: 56 } |  |  | 0.594 |
| walker |  | 8633 | 28 | Code::CodeKey { rung: Decl, file: src/export/mod.rs, decl: 5, sub: 0, line: 61 } |  |  | 0.594 |
| ns | 8661 |  | 137 | src/output/warnings.rs: the complete Warnings set | 5.10 | 4.12 | 0.600 |
| walker |  | 8671 | 38 | Code::CodeKey { rung: Decl, file: src/export/mod.rs, decl: 6, sub: 0, line: 67 } |  |  | 0.602 |
| walker |  | 8721 | 50 | Code::CodeKey { rung: Decl, file: src/export/mod.rs, decl: 3, sub: 0, line: 48 } |  |  | 0.608 |
| walker |  | 8800 | 79 | Code::CodeKey { rung: Decl, file: src/export/mod.rs, decl: 7, sub: 0, line: 73 } |  |  | 0.608 |
| walker |  | 8844 | 44 | Code::CodeKey { rung: Decl, file: src/export/mod.rs, decl: 8, sub: 0, line: 76 } |  |  | 0.608 |
| ns | 8869 |  | 208 | src/command.rs: the Command type and its complete method roster | 6.1 |  | 0.613 |
| walker |  | 8972 | 128 | Code::CodeKey { rung: Decl, file: src/export/mod.rs, decl: 1, sub: 0, line: 27 } |  |  | 0.626 |
| walker |  | 8980 | 8 | Code::CodeKey { rung: Doc, file: src/export/mod.rs, decl: 2, sub: 0, line: 46 } |  |  | 0.628 |
| walker |  | 8991 | 11 | Code::CodeKey { rung: Doc, file: src/export/mod.rs, decl: 6, sub: 0, line: 67 } |  |  | 0.631 |
| walker |  | 9004 | 13 | Code::CodeKey { rung: Doc, file: src/export/mod.rs, decl: 3, sub: 0, line: 48 } |  |  | 0.633 |
| ns | 9015 |  | 146 | How {param} placeholders are substituted, and why naively | 6.2 | 6.1 | 0.629 |
| walker |  | 9019 | 15 | Code::CodeKey { rung: Doc, file: src/export/mod.rs, decl: 9, sub: 0, line: 103 } |  |  | 0.629 |
| walker |  | 9061 | 42 | Code::CodeKey { rung: Names, file: src/export/markup.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.629 |
| walker |  | 9074 | 13 | Code::CodeKey { rung: Decl, file: src/export/markup.rs, decl: 9, sub: 0, line: 108 } |  |  | 0.629 |
| walker |  | 9091 | 17 | Code::CodeKey { rung: Decl, file: src/export/markup.rs, decl: 1, sub: 0, line: 10 } |  |  | 0.629 |
| walker |  | 9144 | 53 | Code::CodeKey { rung: Decl, file: src/export/markup.rs, decl: 10, sub: 0, line: 109 } |  |  | 0.630 |
| ns | 9202 |  | 187 | src/command.rs: Commands and its complete method roster, with the three construction modes | 6.3 |  | 0.626 |
| walker |  | 9302 | 158 | Code::CodeKey { rung: Decl, file: src/export/markup.rs, decl: 2, sub: 0, line: 15 } |  |  | 0.631 |
| walker |  | 9331 | 29 | Code::CodeKey { rung: Names, file: src/export/csv.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.631 |
| walker |  | 9339 | 8 | Code::CodeKey { rung: Decl, file: src/export/csv.rs, decl: 1, sub: 0, line: 12 } |  |  | 0.631 |
| walker |  | 9354 | 15 | Code::CodeKey { rung: Decl, file: src/export/csv.rs, decl: 2, sub: 0, line: 15 } |  |  | 0.631 |
| walker |  | 9409 | 55 | Code::CodeKey { rung: Decl, file: src/export/csv.rs, decl: 3, sub: 0, line: 16 } |  |  | 0.631 |
| walker |  | 9426 | 17 | Code::CodeKey { rung: Doc, file: src/export/mod.rs, decl: 1, sub: 0, line: 27 } |  |  | 0.635 |
| ns | 9431 |  | 229 | src/parameter/: ParameterValue, RangeStep and its 100_000 cap, tokenize() | 6.4 |  | 0.630 |
| walker |  | 9435 | 9 | Code::CodeKey { rung: Body, file: src/export/markup.rs, decl: 6, sub: 0, line: 87 } |  |  | 0.630 |
| walker |  | 9444 | 9 | Code::CodeKey { rung: Body, file: src/export/markup.rs, decl: 7, sub: 0, line: 91 } |  |  | 0.630 |
| walker |  | 9480 | 36 | Code::CodeKey { rung: Names, file: src/export/asciidoc.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.631 |
| walker |  | 9488 | 8 | Code::CodeKey { rung: Decl, file: src/export/asciidoc.rs, decl: 1, sub: 0, line: 4 } |  |  | 0.631 |
| ns | 9581 |  | 150 | build.rs: shell completions generated from the same clap command | 7.1 | 2.1 | 0.625 |
| walker |  | 9613 | 125 | Code::CodeKey { rung: Decl, file: src/export/asciidoc.rs, decl: 2, sub: 0, line: 7 } |  |  | 0.625 |
| walker |  | 9622 | 9 | Code::CodeKey { rung: Body, file: src/export/asciidoc.rs, decl: 6, sub: 0, line: 30 } |  |  | 0.625 |
| walker |  | 9654 | 32 | Code::CodeKey { rung: Names, file: src/export/orgmode.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.626 |
| walker |  | 9662 | 8 | Code::CodeKey { rung: Decl, file: src/export/orgmode.rs, decl: 1, sub: 0, line: 4 } |  |  | 0.626 |
| walker |  | 9735 | 73 | Code::CodeKey { rung: Decl, file: src/export/orgmode.rs, decl: 2, sub: 0, line: 7 } |  |  | 0.626 |
| walker |  | 9765 | 30 | Code::CodeKey { rung: Names, file: src/export/markdown.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.626 |
| walker |  | 9773 | 8 | Code::CodeKey { rung: Decl, file: src/export/markdown.rs, decl: 1, sub: 0, line: 5 } |  |  | 0.626 |
| ns | 9790 |  | 209 | .github/workflows/CICD.yml: the complete job set and the commands each runs | 7.2 |  | 0.619 |
| walker |  | 9846 | 73 | Code::CodeKey { rung: Decl, file: src/export/markdown.rs, decl: 2, sub: 0, line: 8 } |  |  | 0.619 |
| walker |  | 9889 | 43 | Code::CodeKey { rung: Names, file: src/export/json.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.620 |
| walker |  | 9897 | 8 | Code::CodeKey { rung: Decl, file: src/export/json.rs, decl: 2, sub: 0, line: 16 } |  |  | 0.620 |
| walker |  | 9910 | 13 | Code::CodeKey { rung: Decl, file: src/export/json.rs, decl: 3, sub: 0, line: 19 } |  |  | 0.620 |
| walker |  | 9937 | 27 | Code::CodeKey { rung: Decl, file: src/export/json.rs, decl: 1, sub: 0, line: 11 } |  |  | 0.621 |
| ns | 9982 |  | 192 | tests/: the shared harness helpers and the debug-mode test idiom | 7.3 |  | 0.614 |
| walker |  | 9992 | 55 | Code::CodeKey { rung: Decl, file: src/export/json.rs, decl: 4, sub: 0, line: 20 } |  |  | 0.614 |
