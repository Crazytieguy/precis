Score(3000)=0.695 I=0.900 C=0.537 ns_rows≤3K=21/56 grid(1000/1442/2080/3000/4327/6240/9000)=0.518/0.462/0.691/0.695/0.614/0.657/0.669

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 45 | 45 | listing of '.' |  |  | 0.000 |
| walker |  | 70 | 25 | listing of 'doc' |  |  | 0.000 |
| ns | 96 |  | 96 | Crate identity: package name, description, homepage, licence | 1.1 |  | 0.000 |
| walker |  | 114 | 44 | listing of 'src' |  |  | 0.000 |
| walker |  | 127 | 13 | listing of 'src/parameter' |  |  | 0.000 |
| ns | 141 |  | 45 | Repository root listing (complete) | 1.2 |  | 0.512 |
| walker |  | 144 | 17 | listing of 'src/output' |  |  | 0.518 |
| walker |  | 164 | 20 | listing of 'src/timer' |  |  | 0.528 |
| walker |  | 191 | 27 | listing of 'src/benchmark' |  |  | 0.536 |
| ns | 217 |  | 76 | Cargo.toml: version 1.20.0, edition, MSRV, build script | 1.3 | 1.1 | 0.458 |
| walker |  | 219 | 28 | listing of 'src/util' |  |  | 0.477 |
| walker |  | 231 | 12 | entry item at src/main.rs:53 |  |  | 0.477 |
| walker |  | 244 | 13 | entry item at src/main.rs:29 |  |  | 0.477 |
| walker |  | 253 | 9 | entry item body at src/main.rs:29 body 50 |  |  | 0.477 |
| ns | 261 |  | 44 | src/ listing: the flat modules and six subdirectories | 1.4 |  | 0.517 |
| walker |  | 263 | 10 | entry item body at src/main.rs:29 body 48 |  |  | 0.518 |
| walker |  | 275 | 12 | entry item body at src/main.rs:29 body 31 |  |  | 0.518 |
| walker |  | 287 | 12 | entry item body at src/main.rs:29 body 30 |  |  | 0.518 |
| walker |  | 299 | 12 | entry item body at src/main.rs:29 body 47 |  |  | 0.519 |
| walker |  | 311 | 12 | entry item body at src/main.rs:29 body 46 |  |  | 0.520 |
| walker |  | 327 | 16 | entry item body at src/main.rs:29 body 32 |  |  | 0.521 |
| walker |  | 343 | 16 | entry item body at src/main.rs:29 body 43 |  |  | 0.472 |
| ns | 343 |  | 82 | README feature list, first five bullets (rest elided) | 1.5 |  | 0.472 |
| walker |  | 360 | 17 | entry item body at src/main.rs:29 body 34 |  |  | 0.474 |
| walker |  | 378 | 18 | entry item body at src/main.rs:29 body 35 |  |  | 0.476 |
| walker |  | 395 | 17 | entry item body at src/main.rs:29 body 36 |  |  | 0.478 |
| ns | 409 |  | 66 | README feature list, remaining bullets | 1.6 | 1.5 | 0.454 |
| walker |  | 415 | 20 | entry item body at src/main.rs:29 body 45 |  |  | 0.457 |
| walker |  | 450 | 35 | listing of 'src/export' |  |  | 0.481 |
| ns | 471 |  | 62 | src/benchmark/ and src/export/ listings (complete) | 1.7 |  | 0.514 |
| walker |  | 485 | 35 | pub-item names surface in src/export/mod.rs |  |  | 0.514 |
| walker |  | 506 | 21 | pub item at src/export/mod.rs:56 |  |  | 0.514 |
| walker |  | 541 | 35 | pub-item names surface in src/parameter/mod.rs |  |  | 0.514 |
| ns | 549 |  | 78 | src/output/, src/parameter/, src/timer/, src/util/ listings (complete) | 1.8 |  | 0.532 |
| walker |  | 577 | 36 | pub item at src/parameter/mod.rs:8 |  |  | 0.533 |
| walker |  | 617 | 40 | pub item at src/export/mod.rs:67 |  |  | 0.533 |
| ns | 643 |  | 94 | src/main.rs: module declarations and the types the entry point imports | 1.9 |  | 0.494 |
| walker |  | 653 | 36 | pub-item names surface in src/timer/mod.rs |  |  | 0.494 |
| walker |  | 653 | 0 | pub item at src/timer/mod.rs:83 |  |  | 0.494 |
| walker |  | 692 | 39 | pub-item names surface in src/benchmark/mod.rs |  |  | 0.494 |
| walker |  | 692 | 0 | pub item at src/benchmark/mod.rs:32 |  |  | 0.494 |
| walker |  | 742 | 50 | pub item at src/benchmark/mod.rs:34 |  |  | 0.494 |
| walker |  | 750 | 8 | listing of '.github' |  |  | 0.495 |
| walker |  | 755 | 5 | listing of '.github/workflows' |  |  | 0.496 |
| walker |  | 804 | 49 | entry item body at src/main.rs:29 body 37 |  |  | 0.505 |
| walker |  | 843 | 39 | [features] in Cargo.toml |  |  | 0.505 |
| ns | 899 |  | 256 | src/main.rs: the run() pipeline | 1.10 | 1.9 | 0.507 |
| walker |  | 916 | 73 | entry item body at src/main.rs:53 body 54 |  |  | 0.514 |
| walker |  | 929 | 13 | pub-item doc lede at src/export/mod.rs:67 |  |  | 0.515 |
| ns | 983 |  | 84 | src/main.rs: main() error reporting and exit code | 1.11 | 1.10 | 0.518 |
| walker |  | 1016 | 87 | pub item at src/timer/mod.rs:42 |  |  | 0.518 |
| walker |  | 1055 | 39 | impl method sigs in src/parameter/mod.rs |  |  | 0.518 |
| walker |  | 1055 | 0 | impl method at src/parameter/mod.rs:14 |  |  | 0.518 |
| walker |  | 1080 | 25 | mod/use plumbing in src/output/mod.rs |  |  | 0.518 |
| ns | 1098 |  | 115 | tests/, doc/, scripts/ and .github/ listings (complete) | 1.12 |  | 0.481 |
| walker |  | 1196 | 116 | README headline in README.md |  |  | 0.481 |
| walker |  | 1211 | 15 | pub item at src/util/randomized_environment_offset.rs:6 |  |  | 0.481 |
| ns | 1235 |  | 137 | src/cli.rs: get_cli_arguments() and the build_command() clap header | 2.1 |  | 0.460 |
| walker |  | 1239 | 28 | pub-item names surface in src/error.rs |  |  | 0.460 |
| walker |  | 1254 | 15 | pub-item doc lede at src/timer/mod.rs:42 |  |  | 0.460 |
| walker |  | 1382 | 128 | pub item at src/export/mod.rs:28 |  |  | 0.462 |
| walker |  | 1550 | 168 | [package] in Cargo.toml |  |  | 0.752 |
| walker |  | 1565 | 15 | pub-item doc lede at src/timer/mod.rs:83 |  |  | 0.752 |
| walker |  | 1578 | 13 | pub-item doc lede at src/benchmark/mod.rs:32 |  |  | 0.752 |
| walker |  | 1612 | 34 | pub-item names surface in src/command.rs |  |  | 0.752 |
| walker |  | 1612 | 0 | pub item at src/command.rs:134 |  |  | 0.752 |
| walker |  | 1633 | 21 | listing of 'tests' |  |  | 0.769 |
| walker |  | 1650 | 17 | pub-item doc lede at src/export/mod.rs:28 |  |  | 0.769 |
| walker |  | 1671 | 21 | pub item at src/export/csv.rs:13 |  |  | 0.769 |
| ns | 1672 |  | 437 | Complete option roster: every Arg::new(...) in src/cli.rs | 2.2 | 2.1 | 0.692 |
| walker |  | 1692 | 21 | pub item at src/export/json.rs:17 |  |  | 0.692 |
| walker |  | 1713 | 21 | pub item at src/export/markdown.rs:6 |  |  | 0.692 |
| walker |  | 1732 | 19 | pub item at src/parameter/tokenize.rs:1 |  |  | 0.692 |
| walker |  | 1754 | 22 | pub item at src/export/orgmode.rs:5 |  |  | 0.692 |
| ns | 1862 |  | 190 | Complete short-alias roster for the CLI options | 2.3 | 2.2 | 0.659 |
| walker |  | 1905 | 151 | README.md section #1 |  |  | 0.711 |
| walker |  | 1929 | 24 | pub item at src/export/asciidoc.rs:5 |  |  | 0.711 |
| walker |  | 2009 | 80 | README.md section #0 |  |  | 0.711 |
| ns | 2016 |  | 154 | Enumerated value sets, defaults, and the two hidden options | 2.4 | 2.2 | 0.691 |
| walker |  | 2037 | 28 | pub item at src/timer/wall_clock_timer.rs:5 |  |  | 0.691 |
| walker |  | 2061 | 24 | pub-item names surface in src/export/markup.rs |  |  | 0.691 |
| walker |  | 2078 | 17 | pub item at src/export/markup.rs:10 |  |  | 0.691 |
| walker |  | 2122 | 44 | mod/use plumbing in src/parameter/mod.rs |  |  | 0.691 |
| ns | 2146 |  | 130 | Complete set of inter-option conflicts and requirements | 2.5 | 2.2 | 0.676 |
| walker |  | 2166 | 44 | mod/use plumbing in src/util/mod.rs |  |  | 0.676 |
| walker |  | 2195 | 29 | pub item at src/timer/windows_timer.rs:49 |  |  | 0.676 |
| walker |  | 2220 | 25 | pub-item names surface in src/parameter/range_step.rs |  |  | 0.676 |
| walker |  | 2245 | 25 | pub-item names surface in src/timer/unix_timer.rs |  |  | 0.676 |
| walker |  | 2262 | 17 | pub item at src/timer/unix_timer.rs:18 |  |  | 0.676 |
| walker |  | 2288 | 26 | pub-item names surface in src/output/warnings.rs |  |  | 0.676 |
| walker |  | 2318 | 30 | pub item at src/output/warnings.rs:7 |  |  | 0.676 |
| ns | 2366 |  | 220 | src/options.rs: the Options struct, run bounds through setup_command | 3.1 |  | 0.649 |
| walker |  | 2498 | 180 | mod/use plumbing in src/main.rs |  |  | 0.688 |
| ns | 2577 |  | 211 | src/options.rs: remaining Options fields (output, sort, executor, I/O, time unit) | 3.2 | 3.1 | 0.666 |
| ns | 2839 |  | 262 | src/options.rs: impl Default for Options -- the concrete default values | 3.3 | 3.1 | 0.632 |
| walker |  | 2950 | 452 | registration roster at src/cli.rs:18 |  |  | 0.719 |
| walker |  | 2983 | 33 | pub item at src/output/progress_bar.rs:13 |  |  | 0.719 |
| ns | 2991 |  | 152 | src/options.rs: DEFAULT_SHELL per platform, the Shell enum and its parser | 3.4 |  | 0.695 |
| walker |  | 3051 | 68 | pub item at src/cli.rs:8 |  |  | 0.697 |
| walker |  | 3073 | 22 | pub item body at src/cli.rs:8 body 13 |  |  | 0.697 |
| ns | 3094 |  | 103 | src/options.rs: ExecutorKind (Raw / Shell / Mock) and its default | 3.5 | 3.4 | 0.681 |
| walker |  | 3113 | 40 | pub item at src/parameter/range_step.rs:34 |  |  | 0.681 |
| walker |  | 3167 | 54 | pub item at src/output/warnings.rs:13 |  |  | 0.682 |
| walker |  | 3204 | 37 | pub-item names surface in src/util/units.rs |  |  | 0.682 |
| walker |  | 3204 | 0 | pub item at src/util/units.rs:6 |  |  | 0.682 |
| walker |  | 3249 | 45 | pub item at src/util/units.rs:10 |  |  | 0.682 |
| walker |  | 3256 | 7 | pub-item doc lede at src/util/units.rs:10 |  |  | 0.682 |
| ns | 3318 |  | 224 | src/options.rs: OutputStyleOption and SortOrder vocabularies | 3.6 |  | 0.652 |
| walker |  | 3328 | 72 | pub-item names surface in src/outlier_detection.rs |  |  | 0.652 |
| walker |  | 3328 | 0 | pub item at src/outlier_detection.rs:13 |  |  | 0.652 |
| walker |  | 3328 | 0 | pub item at src/outlier_detection.rs:21 |  |  | 0.652 |
| walker |  | 3336 | 8 | pub item at src/outlier_detection.rs:43 |  |  | 0.652 |
| walker |  | 3392 | 56 | pub item at src/util/number.rs:10 |  |  | 0.653 |
| walker |  | 3407 | 15 | pub-item doc lede at src/command.rs:134 |  |  | 0.653 |
| walker |  | 3447 | 40 | pub-item names surface in src/util/min_max.rs |  |  | 0.653 |
| walker |  | 3447 | 0 | pub item at src/util/min_max.rs:2 |  |  | 0.653 |
| walker |  | 3447 | 0 | pub item at src/util/min_max.rs:10 |  |  | 0.653 |
| ns | 3560 |  | 242 | src/options.rs: CommandInputPolicy and CommandOutputPolicy | 3.7 |  | 0.625 |
| walker |  | 3571 | 124 | impl method sigs in src/export/mod.rs |  |  | 0.625 |
| walker |  | 3571 | 0 | impl method at src/export/mod.rs:76 |  |  | 0.625 |
| walker |  | 3571 | 0 | impl method at src/export/mod.rs:103 |  |  | 0.625 |
| walker |  | 3571 | 0 | impl method at src/export/mod.rs:132 |  |  | 0.625 |
| walker |  | 3581 | 10 | pub-item doc lede at src/output/warnings.rs:13 |  |  | 0.626 |
| walker |  | 3652 | 71 | manifest config in Cargo.toml |  |  | 0.626 |
| walker |  | 3698 | 46 | pub-item names surface in src/util/exit_code.rs |  |  | 0.626 |
| walker |  | 3707 | 9 | pub item at src/util/exit_code.rs:4 |  |  | 0.626 |
| walker |  | 3717 | 10 | pub item at src/util/exit_code.rs:20 |  |  | 0.626 |
| walker |  | 3725 | 8 | pub item body at src/util/exit_code.rs:20 body 21 |  |  | 0.626 |
| walker |  | 3746 | 21 | pub item body at src/util/randomized_environment_offset.rs:6 body 7 |  |  | 0.626 |
| ns | 3800 |  | 240 | src/options.rs: CmdFailureAction and RunBounds (default min = 10) | 3.8 |  | 0.603 |
| walker |  | 3802 | 56 | listing of 'scripts' |  |  | 0.635 |
| walker |  | 3915 | 113 | pub item at src/command.rs:22 |  |  | 0.636 |
| walker |  | 3996 | 81 | man-page NAME + DESCRIPTION in doc/hyperfine.1 |  |  | 0.636 |
| ns | 4189 |  | 389 | src/error.rs: the complete OptionsError variant set with messages | 3.10 |  | 0.614 |
| walker |  | 4244 | 248 | [dependencies] in Cargo.toml |  |  | 0.614 |
| walker |  | 4283 | 39 | README.md section #11 |  |  | 0.614 |
| walker |  | 4352 | 69 | pub item at src/benchmark/scheduler.rs:13 |  |  | 0.614 |
| ns | 4407 |  | 218 | src/error.rs: the complete ParameterScanError variant set | 3.11 | 3.10 | 0.600 |
| walker |  | 4451 | 99 | pub item at src/parameter/range_step.rs:7 |  |  | 0.600 |
| walker |  | 4523 | 72 | pub item at src/timer/unix_timer.rs:10 |  |  | 0.600 |
| walker |  | 4585 | 62 | pub-item names surface in src/benchmark/executor.rs |  |  | 0.600 |
| walker |  | 4601 | 16 | pub item at src/benchmark/executor.rs:122 |  |  | 0.600 |
| walker |  | 4624 | 23 | pub item at src/benchmark/executor.rs:303 |  |  | 0.600 |
| ns | 4627 |  | 220 | src/benchmark/executor.rs: the Executor trait | 4.1 |  | 0.586 |
| walker |  | 4657 | 33 | pub item at src/benchmark/executor.rs:19 |  |  | 0.586 |
| walker |  | 4699 | 42 | pub item at src/benchmark/executor.rs:169 |  |  | 0.587 |
| ns | 4752 |  | 125 | The three Executor implementations and their state | 4.2 | 4.1 | 0.597 |
| walker |  | 4839 | 140 | pub-item names surface in src/options.rs |  |  | 0.601 |
| walker |  | 4846 | 7 | pub item at src/options.rs:20 |  |  | 0.602 |
| walker |  | 4856 | 10 | pub item at src/options.rs:17 |  |  | 0.603 |
| walker |  | 4893 | 37 | pub item at src/options.rs:102 |  |  | 0.605 |
| ns | 4904 |  | 152 | BenchmarkIteration and the $HYPERFINE_ITERATION values | 4.3 |  | 0.596 |
| walker |  | 4936 | 43 | pub item at src/options.rs:185 |  |  | 0.600 |
| walker |  | 5002 | 66 | pub item at src/options.rs:24 |  |  | 0.616 |
| walker |  | 5059 | 57 | pub item at src/options.rs:108 |  |  | 0.620 |
| ns | 5072 |  | 168 | The shared command runner: stdio wiring and injected environment variables | 4.4 | 4.3 | 0.609 |
| walker |  | 5132 | 73 | pub item at src/options.rs:123 |  |  | 0.615 |
| walker |  | 5228 | 96 | pub item at src/options.rs:71 |  |  | 0.630 |
| ns | 5284 |  | 212 | Non-zero exit handling and the failure message users actually see | 4.5 | 4.4 | 0.619 |
| walker |  | 5358 | 130 | pub item at src/options.rs:149 |  |  | 0.647 |
| ns | 5481 |  | 197 | Shell-spawning calibration: 50 probe runs, and where the overhead is subtracted | 4.6 | 4.2 | 0.637 |
| walker |  | 5507 | 149 | pub item at src/options.rs:84 |  |  | 0.667 |
| walker |  | 5585 | 78 | pub-item names surface in src/output/format.rs |  |  | 0.667 |
| walker |  | 5585 | 0 | pub item at src/output/format.rs:5 |  |  | 0.667 |
| walker |  | 5585 | 0 | pub item at src/output/format.rs:11 |  |  | 0.667 |
| walker |  | 5585 | 0 | pub item at src/output/format.rs:18 |  |  | 0.667 |
| ns | 5679 |  | 198 | src/benchmark/scheduler.rs: Scheduler state and executor selection | 4.7 |  | 0.659 |
| walker |  | 5832 | 247 | pub item at src/error.rs:7 |  |  | 0.683 |
| ns | 5933 |  | 254 | run_benchmarks(): reference command first, calibrate once, export after each benchmark | 4.8 | 4.7 | 0.667 |
| walker |  | 5936 | 104 | pub-item names surface in src/benchmark/relative_speed.rs |  |  | 0.667 |
| walker |  | 5936 | 0 | pub item at src/benchmark/relative_speed.rs:16 |  |  | 0.667 |
| walker |  | 5936 | 0 | pub item at src/benchmark/relative_speed.rs:20 |  |  | 0.667 |
| walker |  | 5975 | 39 | pub item at src/benchmark/relative_speed.rs:112 |  |  | 0.667 |
| walker |  | 6015 | 40 | pub item at src/benchmark/relative_speed.rs:98 |  |  | 0.667 |
| walker |  | 6069 | 54 | pub item at src/benchmark/relative_speed.rs:86 |  |  | 0.667 |
| walker |  | 6152 | 83 | pub item at src/benchmark/relative_speed.rs:7 |  |  | 0.668 |
| walker |  | 6206 | 54 | README.md section #14 |  |  | 0.668 |
| ns | 6208 |  | 275 | src/benchmark/mod.rs: Benchmark struct, MIN_EXECUTION_TIME, and the complete method roster | 4.10 |  | 0.657 |
| walker |  | 6346 | 140 | pub item at src/benchmark/timing_result.rs:5 |  |  | 0.658 |
| ns | 6418 |  | 210 | How the number of runs is decided | 4.11 | 4.10 | 0.646 |
| walker |  | 6578 | 232 | mod/use plumbing in src/export/mod.rs |  |  | 0.646 |
| ns | 6626 |  | 208 | The three warning triggers inside Benchmark::run | 4.12 | 4.10 | 0.636 |
| walker |  | 6657 | 79 | impl method sigs in src/benchmark/scheduler.rs |  |  | 0.638 |
| walker |  | 6657 | 0 | impl method at src/benchmark/scheduler.rs:34 |  |  | 0.638 |
| walker |  | 6657 | 0 | impl method at src/benchmark/scheduler.rs:61 |  |  | 0.638 |
| walker |  | 6657 | 0 | impl method at src/benchmark/scheduler.rs:156 |  |  | 0.638 |
| ns | 6779 |  | 153 | src/outlier_detection.rs: the modified Z-score method and its threshold | 4.13 | 4.12 | 0.633 |
| walker |  | 7030 | 373 | pub item at src/error.rs:37 |  |  | 0.662 |
| ns | 7075 |  | 296 | src/benchmark/benchmark_result.rs: the complete BenchmarkResult field set (= the JSON export schema) | 5.1 |  | 0.649 |
| ns | 7215 |  | 140 | src/benchmark/timing_result.rs: TimingResult in full | 5.2 | 5.1 | 0.656 |
| walker |  | 7283 | 253 | mod/use plumbing in src/timer/mod.rs |  |  | 0.656 |
| ns | 7378 |  | 163 | src/benchmark/relative_speed.rs: annotated result type and the complete public function set | 5.3 |  | 0.662 |
| ns | 7536 |  | 158 | src/export/mod.rs: the ExportType enum -- the five output formats | 5.4 |  | 0.670 |
| walker |  | 7698 | 415 | impl method sigs in src/benchmark/mod.rs |  |  | 0.669 |
| walker |  | 7698 | 0 | impl method at src/benchmark/mod.rs:42 |  |  | 0.669 |
| walker |  | 7698 | 0 | impl method at src/benchmark/mod.rs:57 |  |  | 0.669 |
| walker |  | 7698 | 0 | impl method at src/benchmark/mod.rs:75 |  |  | 0.669 |
| walker |  | 7698 | 0 | impl method at src/benchmark/mod.rs:96 |  |  | 0.669 |
| walker |  | 7698 | 0 | impl method at src/benchmark/mod.rs:117 |  |  | 0.669 |
| walker |  | 7698 | 0 | impl method at src/benchmark/mod.rs:129 |  |  | 0.669 |
| walker |  | 7698 | 0 | impl method at src/benchmark/mod.rs:141 |  |  | 0.669 |
| ns | 7698 |  | 162 | src/export/mod.rs: the Exporter trait and ExportManager | 5.5 | 5.4 | 0.669 |
| ns | 7835 |  | 137 | Flag-to-exporter wiring and the '-' means stdout convention | 5.6 | 5.5 | 0.664 |
| walker |  | 7967 | 269 | pub item at src/benchmark/executor.rs:35 |  |  | 0.683 |
| walker |  | 8041 | 74 | README.md section #15 |  |  | 0.683 |
| ns | 8051 |  | 216 | src/export/markup.rs: the shared table shape and the blanket Exporter impl | 5.7 | 5.5 | 0.675 |
| walker |  | 8123 | 82 | README.md section #13 |  |  | 0.675 |
| ns | 8267 |  | 216 | The five exporter types, and the CSV column set | 5.8 | 5.4 | 0.669 |
| walker |  | 8453 | 330 | mod/use plumbing in src/benchmark/mod.rs |  |  | 0.669 |
| ns | 8524 |  | 257 | src/output/format.rs: automatic time-unit selection, and the Unit type | 5.9 |  | 0.663 |
| walker |  | 8586 | 133 | impl method sigs in src/export/asciidoc.rs |  |  | 0.663 |
| walker |  | 8586 | 0 | impl method at src/export/asciidoc.rs:8 |  |  | 0.663 |
| walker |  | 8586 | 0 | impl method at src/export/asciidoc.rs:22 |  |  | 0.663 |
| walker |  | 8586 | 0 | impl method at src/export/asciidoc.rs:26 |  |  | 0.663 |
| walker |  | 8586 | 0 | impl method at src/export/asciidoc.rs:30 |  |  | 0.663 |
| walker |  | 8586 | 0 | impl method at src/export/asciidoc.rs:34 |  |  | 0.663 |
| ns | 8661 |  | 137 | src/output/warnings.rs: the complete Warnings set | 5.10 | 4.12 | 0.664 |
| ns | 8869 |  | 208 | src/command.rs: the Command type and its complete method roster | 6.1 |  | 0.658 |
| walker |  | 8950 | 364 | impl method sigs in src/command.rs |  |  | 0.669 |
| walker |  | 8950 | 0 | impl method at src/command.rs:34 |  |  | 0.669 |
| walker |  | 8950 | 0 | impl method at src/command.rs:54 |  |  | 0.669 |
| walker |  | 8950 | 0 | impl method at src/command.rs:61 |  |  | 0.669 |
| walker |  | 8950 | 0 | impl method at src/command.rs:77 |  |  | 0.669 |
| walker |  | 8950 | 0 | impl method at src/command.rs:81 |  |  | 0.669 |
| walker |  | 8950 | 0 | impl method at src/command.rs:96 |  |  | 0.669 |
| walker |  | 8950 | 0 | impl method at src/command.rs:100 |  |  | 0.669 |
| walker |  | 8950 | 0 | impl method at src/command.rs:106 |  |  | 0.669 |
| walker |  | 8950 | 0 | impl method at src/command.rs:137 |  |  | 0.669 |
| walker |  | 8950 | 0 | impl method at src/command.rs:250 |  |  | 0.669 |
| walker |  | 8950 | 0 | impl method at src/command.rs:254 |  |  | 0.669 |
| walker |  | 8950 | 0 | impl method at src/command.rs:260 |  |  | 0.669 |
| walker |  | 9005 | 55 | impl method at src/command.rs:42 |  |  | 0.669 |
| ns | 9015 |  | 146 | How {param} placeholders are substituted, and why naively | 6.2 | 6.1 | 0.664 |
| ns | 9202 |  | 187 | src/command.rs: Commands and its complete method roster, with the three construction modes | 6.3 |  | 0.663 |
| walker |  | 9275 | 270 | impl method sigs in src/options.rs |  |  | 0.670 |
| walker |  | 9275 | 0 | impl method at src/options.rs:33 |  |  | 0.670 |
| walker |  | 9275 | 0 | impl method at src/options.rs:49 |  |  | 0.670 |
| walker |  | 9275 | 0 | impl method at src/options.rs:57 |  |  | 0.670 |
| walker |  | 9275 | 0 | impl method at src/options.rs:117 |  |  | 0.670 |
| walker |  | 9275 | 0 | impl method at src/options.rs:133 |  |  | 0.670 |
| walker |  | 9275 | 0 | impl method at src/options.rs:165 |  |  | 0.670 |
| walker |  | 9275 | 0 | impl method at src/options.rs:192 |  |  | 0.670 |
| walker |  | 9275 | 0 | impl method at src/options.rs:252 |  |  | 0.670 |
| walker |  | 9275 | 0 | impl method at src/options.rs:276 |  |  | 0.670 |
| walker |  | 9275 | 0 | impl method at src/options.rs:470 |  |  | 0.670 |
| walker |  | 9302 | 27 | pub item body at src/output/format.rs:5 body 6 |  |  | 0.670 |
| ns | 9431 |  | 229 | src/parameter/: ParameterValue, RangeStep and its 100_000 cap, tokenize() | 6.4 |  | 0.671 |
| ns | 9581 |  | 150 | build.rs: shell completions generated from the same clap command | 7.1 | 2.1 | 0.665 |
| ns | 9790 |  | 209 | .github/workflows/CICD.yml: the complete job set and the commands each runs | 7.2 |  | 0.658 |
| walker |  | 9885 | 583 | pub item at src/options.rs:198 |  |  | 0.685 |
| walker |  | 9929 | 44 | impl method at src/benchmark/scheduler.rs:21 |  |  | 0.685 |
| walker |  | 9942 | 13 | pub-item doc lede at src/output/progress_bar.rs:13 |  |  | 0.685 |
| ns | 9982 |  | 192 | tests/: the shared harness helpers and the debug-mode test idiom | 7.3 |  | 0.678 |
