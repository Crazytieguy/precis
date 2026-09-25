Score(3000)=0.695 I=0.900 C=0.537 ns_rows≤3K=21/56 grid(1000/1442/2080/3000/4327/6240/9000)=0.518/0.462/0.691/0.695/0.614/0.657/0.669

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 50 | 50 | listing of '.' |  |  | 0.000 |
| walker |  | 74 | 24 | listing of 'doc' |  |  | 0.000 |
| ns | 96 |  | 96 | Crate identity: package name, description, homepage, licence | 1.1 |  | 0.000 |
| walker |  | 123 | 49 | listing of 'src' |  |  | 0.000 |
| walker |  | 135 | 12 | listing of 'src/parameter' |  |  | 0.000 |
| ns | 146 |  | 50 | Repository root listing (complete) | 1.2 |  | 0.512 |
| walker |  | 151 | 16 | listing of 'src/output' |  |  | 0.518 |
| walker |  | 170 | 19 | listing of 'src/timer' |  |  | 0.528 |
| walker |  | 196 | 26 | listing of 'src/benchmark' |  |  | 0.536 |
| ns | 222 |  | 76 | Cargo.toml: version 1.20.0, edition, MSRV, build script | 1.3 | 1.1 | 0.458 |
| walker |  | 223 | 27 | listing of 'src/util' |  |  | 0.477 |
| walker |  | 235 | 12 | entry item at src/main.rs:53 |  |  | 0.477 |
| walker |  | 248 | 13 | entry item at src/main.rs:29 |  |  | 0.477 |
| walker |  | 257 | 9 | entry item body at src/main.rs:29 body 50 |  |  | 0.477 |
| walker |  | 267 | 10 | entry item body at src/main.rs:29 body 48 |  |  | 0.477 |
| ns | 271 |  | 49 | src/ listing: the flat modules and six subdirectories | 1.4 |  | 0.518 |
| walker |  | 279 | 12 | entry item body at src/main.rs:29 body 31 |  |  | 0.518 |
| walker |  | 291 | 12 | entry item body at src/main.rs:29 body 30 |  |  | 0.518 |
| walker |  | 303 | 12 | entry item body at src/main.rs:29 body 47 |  |  | 0.519 |
| walker |  | 315 | 12 | entry item body at src/main.rs:29 body 46 |  |  | 0.520 |
| walker |  | 331 | 16 | entry item body at src/main.rs:29 body 32 |  |  | 0.521 |
| walker |  | 347 | 16 | entry item body at src/main.rs:29 body 43 |  |  | 0.523 |
| ns | 353 |  | 82 | README feature list, first five bullets (rest elided) | 1.5 |  | 0.472 |
| walker |  | 364 | 17 | entry item body at src/main.rs:29 body 34 |  |  | 0.474 |
| walker |  | 382 | 18 | entry item body at src/main.rs:29 body 35 |  |  | 0.476 |
| walker |  | 399 | 17 | entry item body at src/main.rs:29 body 36 |  |  | 0.478 |
| walker |  | 419 | 20 | entry item body at src/main.rs:29 body 45 |  |  | 0.457 |
| ns | 419 |  | 66 | README feature list, remaining bullets | 1.6 | 1.5 | 0.457 |
| walker |  | 453 | 34 | listing of 'src/export' |  |  | 0.481 |
| ns | 479 |  | 60 | src/benchmark/ and src/export/ listings (complete) | 1.7 |  | 0.514 |
| walker |  | 488 | 35 | pub-item names surface in src/export/mod.rs |  |  | 0.514 |
| walker |  | 509 | 21 | pub item at src/export/mod.rs:56 |  |  | 0.514 |
| walker |  | 544 | 35 | pub-item names surface in src/parameter/mod.rs |  |  | 0.514 |
| ns | 553 |  | 74 | src/output/, src/parameter/, src/timer/, src/util/ listings (complete) | 1.8 |  | 0.532 |
| walker |  | 580 | 36 | pub item at src/parameter/mod.rs:8 |  |  | 0.533 |
| walker |  | 620 | 40 | pub item at src/export/mod.rs:67 |  |  | 0.533 |
| ns | 647 |  | 94 | src/main.rs: module declarations and the types the entry point imports | 1.9 |  | 0.494 |
| walker |  | 656 | 36 | pub-item names surface in src/timer/mod.rs |  |  | 0.494 |
| walker |  | 656 | 0 | pub item at src/timer/mod.rs:83 |  |  | 0.494 |
| walker |  | 695 | 39 | pub-item names surface in src/benchmark/mod.rs |  |  | 0.494 |
| walker |  | 695 | 0 | pub item at src/benchmark/mod.rs:32 |  |  | 0.494 |
| walker |  | 745 | 50 | pub item at src/benchmark/mod.rs:34 |  |  | 0.494 |
| walker |  | 753 | 8 | listing of '.github' |  |  | 0.495 |
| walker |  | 757 | 4 | listing of '.github/workflows' |  |  | 0.496 |
| walker |  | 806 | 49 | entry item body at src/main.rs:29 body 37 |  |  | 0.505 |
| walker |  | 845 | 39 | [features] in Cargo.toml |  |  | 0.505 |
| ns | 903 |  | 256 | src/main.rs: the run() pipeline | 1.10 | 1.9 | 0.507 |
| walker |  | 918 | 73 | entry item body at src/main.rs:53 body 54 |  |  | 0.514 |
| walker |  | 931 | 13 | pub-item doc lede at src/export/mod.rs:67 |  |  | 0.515 |
| ns | 987 |  | 84 | src/main.rs: main() error reporting and exit code | 1.11 | 1.10 | 0.518 |
| walker |  | 1018 | 87 | pub item at src/timer/mod.rs:42 |  |  | 0.518 |
| walker |  | 1057 | 39 | impl method sigs in src/parameter/mod.rs |  |  | 0.518 |
| walker |  | 1057 | 0 | impl method at src/parameter/mod.rs:14 |  |  | 0.518 |
| walker |  | 1082 | 25 | mod/use plumbing in src/output/mod.rs |  |  | 0.518 |
| ns | 1099 |  | 112 | tests/, doc/, scripts/ and .github/ listings (complete) | 1.12 |  | 0.481 |
| walker |  | 1198 | 116 | README headline in README.md |  |  | 0.481 |
| walker |  | 1213 | 15 | pub item at src/util/randomized_environment_offset.rs:6 |  |  | 0.481 |
| ns | 1236 |  | 137 | src/cli.rs: get_cli_arguments() and the build_command() clap header | 2.1 |  | 0.460 |
| walker |  | 1241 | 28 | pub-item names surface in src/error.rs |  |  | 0.460 |
| walker |  | 1256 | 15 | pub-item doc lede at src/timer/mod.rs:42 |  |  | 0.460 |
| walker |  | 1384 | 128 | pub item at src/export/mod.rs:28 |  |  | 0.462 |
| walker |  | 1552 | 168 | [package] in Cargo.toml |  |  | 0.752 |
| walker |  | 1567 | 15 | pub-item doc lede at src/timer/mod.rs:83 |  |  | 0.752 |
| walker |  | 1580 | 13 | pub-item doc lede at src/benchmark/mod.rs:32 |  |  | 0.752 |
| walker |  | 1600 | 20 | listing of 'tests' |  |  | 0.769 |
| walker |  | 1617 | 17 | pub-item doc lede at src/export/mod.rs:28 |  |  | 0.769 |
| walker |  | 1651 | 34 | pub-item names surface in src/command.rs |  |  | 0.769 |
| walker |  | 1651 | 0 | pub item at src/command.rs:134 |  |  | 0.769 |
| walker |  | 1672 | 21 | pub item at src/export/csv.rs:13 |  |  | 0.769 |
| ns | 1673 |  | 437 | Complete option roster: every Arg::new(...) in src/cli.rs | 2.2 | 2.1 | 0.692 |
| walker |  | 1693 | 21 | pub item at src/export/json.rs:17 |  |  | 0.692 |
| walker |  | 1714 | 21 | pub item at src/export/markdown.rs:6 |  |  | 0.692 |
| walker |  | 1733 | 19 | pub item at src/parameter/tokenize.rs:1 |  |  | 0.692 |
| walker |  | 1755 | 22 | pub item at src/export/orgmode.rs:5 |  |  | 0.692 |
| ns | 1863 |  | 190 | Complete short-alias roster for the CLI options | 2.3 | 2.2 | 0.659 |
| walker |  | 1906 | 151 | README.md section #1 |  |  | 0.711 |
| walker |  | 1930 | 24 | pub item at src/export/asciidoc.rs:5 |  |  | 0.711 |
| walker |  | 2010 | 80 | README.md section #0 |  |  | 0.711 |
| ns | 2017 |  | 154 | Enumerated value sets, defaults, and the two hidden options | 2.4 | 2.2 | 0.691 |
| walker |  | 2038 | 28 | pub item at src/timer/wall_clock_timer.rs:5 |  |  | 0.691 |
| walker |  | 2062 | 24 | pub-item names surface in src/export/markup.rs |  |  | 0.691 |
| walker |  | 2079 | 17 | pub item at src/export/markup.rs:10 |  |  | 0.691 |
| walker |  | 2123 | 44 | mod/use plumbing in src/parameter/mod.rs |  |  | 0.691 |
| ns | 2147 |  | 130 | Complete set of inter-option conflicts and requirements | 2.5 | 2.2 | 0.676 |
| walker |  | 2167 | 44 | mod/use plumbing in src/util/mod.rs |  |  | 0.676 |
| walker |  | 2196 | 29 | pub item at src/timer/windows_timer.rs:49 |  |  | 0.676 |
| walker |  | 2221 | 25 | pub-item names surface in src/parameter/range_step.rs |  |  | 0.676 |
| walker |  | 2246 | 25 | pub-item names surface in src/timer/unix_timer.rs |  |  | 0.676 |
| walker |  | 2263 | 17 | pub item at src/timer/unix_timer.rs:18 |  |  | 0.676 |
| walker |  | 2289 | 26 | pub-item names surface in src/output/warnings.rs |  |  | 0.676 |
| walker |  | 2319 | 30 | pub item at src/output/warnings.rs:7 |  |  | 0.676 |
| ns | 2367 |  | 220 | src/options.rs: the Options struct, run bounds through setup_command | 3.1 |  | 0.649 |
| walker |  | 2499 | 180 | mod/use plumbing in src/main.rs |  |  | 0.688 |
| ns | 2578 |  | 211 | src/options.rs: remaining Options fields (output, sort, executor, I/O, time unit) | 3.2 | 3.1 | 0.666 |
| ns | 2840 |  | 262 | src/options.rs: impl Default for Options -- the concrete default values | 3.3 | 3.1 | 0.632 |
| walker |  | 2951 | 452 | registration roster at src/cli.rs:18 |  |  | 0.719 |
| walker |  | 2984 | 33 | pub item at src/output/progress_bar.rs:13 |  |  | 0.719 |
| ns | 2992 |  | 152 | src/options.rs: DEFAULT_SHELL per platform, the Shell enum and its parser | 3.4 |  | 0.695 |
| walker |  | 3052 | 68 | pub item at src/cli.rs:8 |  |  | 0.697 |
| walker |  | 3074 | 22 | pub item body at src/cli.rs:8 body 13 |  |  | 0.697 |
| ns | 3095 |  | 103 | src/options.rs: ExecutorKind (Raw / Shell / Mock) and its default | 3.5 | 3.4 | 0.681 |
| walker |  | 3114 | 40 | pub item at src/parameter/range_step.rs:34 |  |  | 0.681 |
| walker |  | 3168 | 54 | pub item at src/output/warnings.rs:13 |  |  | 0.682 |
| walker |  | 3205 | 37 | pub-item names surface in src/util/units.rs |  |  | 0.682 |
| walker |  | 3205 | 0 | pub item at src/util/units.rs:6 |  |  | 0.682 |
| walker |  | 3250 | 45 | pub item at src/util/units.rs:10 |  |  | 0.682 |
| walker |  | 3257 | 7 | pub-item doc lede at src/util/units.rs:10 |  |  | 0.682 |
| ns | 3319 |  | 224 | src/options.rs: OutputStyleOption and SortOrder vocabularies | 3.6 |  | 0.652 |
| walker |  | 3329 | 72 | pub-item names surface in src/outlier_detection.rs |  |  | 0.652 |
| walker |  | 3329 | 0 | pub item at src/outlier_detection.rs:13 |  |  | 0.652 |
| walker |  | 3329 | 0 | pub item at src/outlier_detection.rs:21 |  |  | 0.652 |
| walker |  | 3337 | 8 | pub item at src/outlier_detection.rs:43 |  |  | 0.652 |
| walker |  | 3393 | 56 | pub item at src/util/number.rs:10 |  |  | 0.653 |
| walker |  | 3408 | 15 | pub-item doc lede at src/command.rs:134 |  |  | 0.653 |
| walker |  | 3448 | 40 | pub-item names surface in src/util/min_max.rs |  |  | 0.653 |
| walker |  | 3448 | 0 | pub item at src/util/min_max.rs:2 |  |  | 0.653 |
| walker |  | 3448 | 0 | pub item at src/util/min_max.rs:10 |  |  | 0.653 |
| ns | 3561 |  | 242 | src/options.rs: CommandInputPolicy and CommandOutputPolicy | 3.7 |  | 0.625 |
| walker |  | 3572 | 124 | impl method sigs in src/export/mod.rs |  |  | 0.625 |
| walker |  | 3572 | 0 | impl method at src/export/mod.rs:76 |  |  | 0.625 |
| walker |  | 3572 | 0 | impl method at src/export/mod.rs:103 |  |  | 0.625 |
| walker |  | 3572 | 0 | impl method at src/export/mod.rs:132 |  |  | 0.625 |
| walker |  | 3582 | 10 | pub-item doc lede at src/output/warnings.rs:13 |  |  | 0.626 |
| walker |  | 3653 | 71 | manifest config in Cargo.toml |  |  | 0.626 |
| walker |  | 3699 | 46 | pub-item names surface in src/util/exit_code.rs |  |  | 0.626 |
| walker |  | 3708 | 9 | pub item at src/util/exit_code.rs:4 |  |  | 0.626 |
| walker |  | 3718 | 10 | pub item at src/util/exit_code.rs:20 |  |  | 0.626 |
| walker |  | 3726 | 8 | pub item body at src/util/exit_code.rs:20 body 21 |  |  | 0.626 |
| walker |  | 3781 | 55 | listing of 'scripts' |  |  | 0.659 |
| ns | 3801 |  | 240 | src/options.rs: CmdFailureAction and RunBounds (default min = 10) | 3.8 |  | 0.635 |
| walker |  | 3894 | 113 | pub item at src/command.rs:22 |  |  | 0.636 |
| walker |  | 3975 | 81 | man-page NAME + DESCRIPTION in doc/hyperfine.1 |  |  | 0.636 |
| ns | 4190 |  | 389 | src/error.rs: the complete OptionsError variant set with messages | 3.10 |  | 0.614 |
| walker |  | 4223 | 248 | [dependencies] in Cargo.toml |  |  | 0.614 |
| walker |  | 4262 | 39 | README.md section #11 |  |  | 0.614 |
| walker |  | 4331 | 69 | pub item at src/benchmark/scheduler.rs:13 |  |  | 0.614 |
| ns | 4408 |  | 218 | src/error.rs: the complete ParameterScanError variant set | 3.11 | 3.10 | 0.600 |
| walker |  | 4430 | 99 | pub item at src/parameter/range_step.rs:7 |  |  | 0.600 |
| walker |  | 4502 | 72 | pub item at src/timer/unix_timer.rs:10 |  |  | 0.600 |
| walker |  | 4564 | 62 | pub-item names surface in src/benchmark/executor.rs |  |  | 0.600 |
| walker |  | 4580 | 16 | pub item at src/benchmark/executor.rs:122 |  |  | 0.600 |
| walker |  | 4603 | 23 | pub item at src/benchmark/executor.rs:303 |  |  | 0.600 |
| ns | 4628 |  | 220 | src/benchmark/executor.rs: the Executor trait | 4.1 |  | 0.586 |
| walker |  | 4636 | 33 | pub item at src/benchmark/executor.rs:19 |  |  | 0.586 |
| walker |  | 4678 | 42 | pub item at src/benchmark/executor.rs:169 |  |  | 0.587 |
| ns | 4753 |  | 125 | The three Executor implementations and their state | 4.2 | 4.1 | 0.597 |
| walker |  | 4818 | 140 | pub-item names surface in src/options.rs |  |  | 0.601 |
| walker |  | 4825 | 7 | pub item at src/options.rs:20 |  |  | 0.602 |
| walker |  | 4835 | 10 | pub item at src/options.rs:17 |  |  | 0.603 |
| walker |  | 4872 | 37 | pub item at src/options.rs:102 |  |  | 0.605 |
| ns | 4905 |  | 152 | BenchmarkIteration and the $HYPERFINE_ITERATION values | 4.3 |  | 0.596 |
| walker |  | 4915 | 43 | pub item at src/options.rs:185 |  |  | 0.600 |
| walker |  | 4981 | 66 | pub item at src/options.rs:24 |  |  | 0.616 |
| walker |  | 5038 | 57 | pub item at src/options.rs:108 |  |  | 0.620 |
| ns | 5073 |  | 168 | The shared command runner: stdio wiring and injected environment variables | 4.4 | 4.3 | 0.609 |
| walker |  | 5111 | 73 | pub item at src/options.rs:123 |  |  | 0.615 |
| walker |  | 5207 | 96 | pub item at src/options.rs:71 |  |  | 0.630 |
| ns | 5285 |  | 212 | Non-zero exit handling and the failure message users actually see | 4.5 | 4.4 | 0.619 |
| walker |  | 5337 | 130 | pub item at src/options.rs:149 |  |  | 0.647 |
| ns | 5482 |  | 197 | Shell-spawning calibration: 50 probe runs, and where the overhead is subtracted | 4.6 | 4.2 | 0.637 |
| walker |  | 5486 | 149 | pub item at src/options.rs:84 |  |  | 0.667 |
| walker |  | 5564 | 78 | pub-item names surface in src/output/format.rs |  |  | 0.667 |
| walker |  | 5564 | 0 | pub item at src/output/format.rs:5 |  |  | 0.667 |
| walker |  | 5564 | 0 | pub item at src/output/format.rs:11 |  |  | 0.667 |
| walker |  | 5564 | 0 | pub item at src/output/format.rs:18 |  |  | 0.667 |
| ns | 5680 |  | 198 | src/benchmark/scheduler.rs: Scheduler state and executor selection | 4.7 |  | 0.659 |
| walker |  | 5811 | 247 | pub item at src/error.rs:7 |  |  | 0.683 |
| walker |  | 5915 | 104 | pub-item names surface in src/benchmark/relative_speed.rs |  |  | 0.683 |
| walker |  | 5915 | 0 | pub item at src/benchmark/relative_speed.rs:16 |  |  | 0.683 |
| walker |  | 5915 | 0 | pub item at src/benchmark/relative_speed.rs:20 |  |  | 0.683 |
| ns | 5934 |  | 254 | run_benchmarks(): reference command first, calibrate once, export after each benchmark | 4.8 | 4.7 | 0.667 |
| walker |  | 5954 | 39 | pub item at src/benchmark/relative_speed.rs:112 |  |  | 0.667 |
| walker |  | 5994 | 40 | pub item at src/benchmark/relative_speed.rs:98 |  |  | 0.667 |
| walker |  | 6048 | 54 | pub item at src/benchmark/relative_speed.rs:86 |  |  | 0.667 |
| walker |  | 6131 | 83 | pub item at src/benchmark/relative_speed.rs:7 |  |  | 0.668 |
| walker |  | 6185 | 54 | README.md section #14 |  |  | 0.668 |
| ns | 6209 |  | 275 | src/benchmark/mod.rs: Benchmark struct, MIN_EXECUTION_TIME, and the complete method roster | 4.10 |  | 0.657 |
| walker |  | 6325 | 140 | pub item at src/benchmark/timing_result.rs:5 |  |  | 0.658 |
| ns | 6419 |  | 210 | How the number of runs is decided | 4.11 | 4.10 | 0.646 |
| walker |  | 6557 | 232 | mod/use plumbing in src/export/mod.rs |  |  | 0.646 |
| ns | 6627 |  | 208 | The three warning triggers inside Benchmark::run | 4.12 | 4.10 | 0.636 |
| walker |  | 6636 | 79 | impl method sigs in src/benchmark/scheduler.rs |  |  | 0.638 |
| walker |  | 6636 | 0 | impl method at src/benchmark/scheduler.rs:34 |  |  | 0.638 |
| walker |  | 6636 | 0 | impl method at src/benchmark/scheduler.rs:61 |  |  | 0.638 |
| walker |  | 6636 | 0 | impl method at src/benchmark/scheduler.rs:156 |  |  | 0.638 |
| ns | 6780 |  | 153 | src/outlier_detection.rs: the modified Z-score method and its threshold | 4.13 | 4.12 | 0.633 |
| walker |  | 7009 | 373 | pub item at src/error.rs:37 |  |  | 0.662 |
| ns | 7076 |  | 296 | src/benchmark/benchmark_result.rs: the complete BenchmarkResult field set (= the JSON export schema) | 5.1 |  | 0.649 |
| ns | 7216 |  | 140 | src/benchmark/timing_result.rs: TimingResult in full | 5.2 | 5.1 | 0.656 |
| walker |  | 7262 | 253 | mod/use plumbing in src/timer/mod.rs |  |  | 0.656 |
| ns | 7379 |  | 163 | src/benchmark/relative_speed.rs: annotated result type and the complete public function set | 5.3 |  | 0.662 |
| ns | 7537 |  | 158 | src/export/mod.rs: the ExportType enum -- the five output formats | 5.4 |  | 0.670 |
| walker |  | 7677 | 415 | impl method sigs in src/benchmark/mod.rs |  |  | 0.676 |
| walker |  | 7677 | 0 | impl method at src/benchmark/mod.rs:42 |  |  | 0.676 |
| walker |  | 7677 | 0 | impl method at src/benchmark/mod.rs:57 |  |  | 0.676 |
| walker |  | 7677 | 0 | impl method at src/benchmark/mod.rs:75 |  |  | 0.676 |
| walker |  | 7677 | 0 | impl method at src/benchmark/mod.rs:96 |  |  | 0.676 |
| walker |  | 7677 | 0 | impl method at src/benchmark/mod.rs:117 |  |  | 0.676 |
| walker |  | 7677 | 0 | impl method at src/benchmark/mod.rs:129 |  |  | 0.676 |
| walker |  | 7677 | 0 | impl method at src/benchmark/mod.rs:141 |  |  | 0.676 |
| ns | 7699 |  | 162 | src/export/mod.rs: the Exporter trait and ExportManager | 5.5 | 5.4 | 0.669 |
| ns | 7836 |  | 137 | Flag-to-exporter wiring and the '-' means stdout convention | 5.6 | 5.5 | 0.664 |
| walker |  | 7946 | 269 | pub item at src/benchmark/executor.rs:35 |  |  | 0.683 |
| walker |  | 8020 | 74 | README.md section #15 |  |  | 0.683 |
| ns | 8052 |  | 216 | src/export/markup.rs: the shared table shape and the blanket Exporter impl | 5.7 | 5.5 | 0.675 |
| walker |  | 8102 | 82 | README.md section #13 |  |  | 0.675 |
| ns | 8268 |  | 216 | The five exporter types, and the CSV column set | 5.8 | 5.4 | 0.669 |
| walker |  | 8432 | 330 | mod/use plumbing in src/benchmark/mod.rs |  |  | 0.669 |
| ns | 8525 |  | 257 | src/output/format.rs: automatic time-unit selection, and the Unit type | 5.9 |  | 0.663 |
| walker |  | 8565 | 133 | impl method sigs in src/export/asciidoc.rs |  |  | 0.663 |
| walker |  | 8565 | 0 | impl method at src/export/asciidoc.rs:8 |  |  | 0.663 |
| walker |  | 8565 | 0 | impl method at src/export/asciidoc.rs:22 |  |  | 0.663 |
| walker |  | 8565 | 0 | impl method at src/export/asciidoc.rs:26 |  |  | 0.663 |
| walker |  | 8565 | 0 | impl method at src/export/asciidoc.rs:30 |  |  | 0.663 |
| walker |  | 8565 | 0 | impl method at src/export/asciidoc.rs:34 |  |  | 0.663 |
| ns | 8662 |  | 137 | src/output/warnings.rs: the complete Warnings set | 5.10 | 4.12 | 0.664 |
| ns | 8870 |  | 208 | src/command.rs: the Command type and its complete method roster | 6.1 |  | 0.658 |
| walker |  | 8929 | 364 | impl method sigs in src/command.rs |  |  | 0.669 |
| walker |  | 8929 | 0 | impl method at src/command.rs:34 |  |  | 0.669 |
| walker |  | 8929 | 0 | impl method at src/command.rs:54 |  |  | 0.669 |
| walker |  | 8929 | 0 | impl method at src/command.rs:61 |  |  | 0.669 |
| walker |  | 8929 | 0 | impl method at src/command.rs:77 |  |  | 0.669 |
| walker |  | 8929 | 0 | impl method at src/command.rs:81 |  |  | 0.669 |
| walker |  | 8929 | 0 | impl method at src/command.rs:96 |  |  | 0.669 |
| walker |  | 8929 | 0 | impl method at src/command.rs:100 |  |  | 0.669 |
| walker |  | 8929 | 0 | impl method at src/command.rs:106 |  |  | 0.669 |
| walker |  | 8929 | 0 | impl method at src/command.rs:137 |  |  | 0.669 |
| walker |  | 8929 | 0 | impl method at src/command.rs:250 |  |  | 0.669 |
| walker |  | 8929 | 0 | impl method at src/command.rs:254 |  |  | 0.669 |
| walker |  | 8929 | 0 | impl method at src/command.rs:260 |  |  | 0.669 |
| walker |  | 8984 | 55 | impl method at src/command.rs:42 |  |  | 0.669 |
| ns | 9016 |  | 146 | How {param} placeholders are substituted, and why naively | 6.2 | 6.1 | 0.664 |
| ns | 9203 |  | 187 | src/command.rs: Commands and its complete method roster, with the three construction modes | 6.3 |  | 0.663 |
| walker |  | 9254 | 270 | impl method sigs in src/options.rs |  |  | 0.670 |
| walker |  | 9254 | 0 | impl method at src/options.rs:33 |  |  | 0.670 |
| walker |  | 9254 | 0 | impl method at src/options.rs:49 |  |  | 0.670 |
| walker |  | 9254 | 0 | impl method at src/options.rs:57 |  |  | 0.670 |
| walker |  | 9254 | 0 | impl method at src/options.rs:117 |  |  | 0.670 |
| walker |  | 9254 | 0 | impl method at src/options.rs:133 |  |  | 0.670 |
| walker |  | 9254 | 0 | impl method at src/options.rs:165 |  |  | 0.670 |
| walker |  | 9254 | 0 | impl method at src/options.rs:192 |  |  | 0.670 |
| walker |  | 9254 | 0 | impl method at src/options.rs:252 |  |  | 0.670 |
| walker |  | 9254 | 0 | impl method at src/options.rs:276 |  |  | 0.670 |
| walker |  | 9254 | 0 | impl method at src/options.rs:470 |  |  | 0.670 |
| walker |  | 9281 | 27 | pub item body at src/output/format.rs:5 body 6 |  |  | 0.670 |
| ns | 9432 |  | 229 | src/parameter/: ParameterValue, RangeStep and its 100_000 cap, tokenize() | 6.4 |  | 0.671 |
| ns | 9582 |  | 150 | build.rs: shell completions generated from the same clap command | 7.1 | 2.1 | 0.665 |
| ns | 9791 |  | 209 | .github/workflows/CICD.yml: the complete job set and the commands each runs | 7.2 |  | 0.658 |
| walker |  | 9864 | 583 | pub item at src/options.rs:198 |  |  | 0.685 |
| walker |  | 9908 | 44 | impl method at src/benchmark/scheduler.rs:21 |  |  | 0.685 |
| walker |  | 9921 | 13 | pub-item doc lede at src/output/progress_bar.rs:13 |  |  | 0.685 |
| walker |  | 9982 | 61 | impl method body at src/parameter/mod.rs:14 body 15 |  |  | 0.685 |
| ns | 9983 |  | 192 | tests/: the shared harness helpers and the debug-mode test idiom | 7.3 |  | 0.678 |
