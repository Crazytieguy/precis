Score(3000)=0.622 I=0.860 C=0.449 ns_rows≤3K=21/42 (reached=9 partial=1 missing=11)

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 45 | 45 | listing of '.' |  |  | 1.000 |
| ns | 45 |  | 45 | Top-level fs listing | 1.1 |  | 1.000 |
| ns | 67 |  | 22 | README one-line description | 1.2 |  | 0.917 |
| walker |  | 84 | 39 | [features] in Cargo.toml |  |  | 0.917 |
| ns | 111 |  | 44 | src/ fs listing | 1.3 |  | 0.643 |
| ns | 174 |  | 63 | Cargo package identity (name + version + edition) | 1.4 |  | 0.581 |
| walker |  | 200 | 116 | README headline in README.md |  |  | 0.588 |
| ns | 268 |  | 94 | main.rs module declarations | 1.5 |  | 0.492 |
| ns | 330 |  | 62 | src/benchmark/ + src/export/ fs listings | 1.6 |  | 0.416 |
| walker |  | 368 | 168 | [package] in Cargo.toml |  |  | 0.495 |
| ns | 408 |  | 78 | Remaining src/ subdir fs listings | 1.7 |  | 0.424 |
| walker |  | 519 | 151 | README.md section #1 |  |  | 0.430 |
| walker |  | 544 | 25 | listing of 'doc' |  |  | 0.430 |
| ns | 586 |  | 178 | main.rs run() — first half (CLI parsing + builders) | 1.8 |  | 0.385 |
| walker |  | 588 | 44 | listing of 'src' |  |  | 0.527 |
| walker |  | 601 | 13 | listing of 'src/parameter' |  |  | 0.532 |
| walker |  | 613 | 12 | entry item at src/main.rs:53 |  |  | 0.532 |
| walker |  | 626 | 13 | entry item at src/main.rs:29 |  |  | 0.533 |
| walker |  | 635 | 9 | entry item body at src/main.rs:29 body 50 |  |  | 0.533 |
| walker |  | 645 | 10 | entry item body at src/main.rs:29 body 48 |  |  | 0.533 |
| walker |  | 657 | 12 | entry item body at src/main.rs:29 body 31 |  |  | 0.534 |
| walker |  | 669 | 12 | entry item body at src/main.rs:29 body 30 |  |  | 0.537 |
| walker |  | 681 | 12 | entry item body at src/main.rs:29 body 47 |  |  | 0.537 |
| walker |  | 693 | 12 | entry item body at src/main.rs:29 body 46 |  |  | 0.538 |
| walker |  | 709 | 16 | entry item body at src/main.rs:29 body 32 |  |  | 0.543 |
| walker |  | 725 | 16 | entry item body at src/main.rs:29 body 43 |  |  | 0.550 |
| walker |  | 742 | 17 | entry item body at src/main.rs:29 body 34 |  |  | 0.559 |
| ns | 753 |  | 167 | main.rs run() — second half (Scheduler) + main() wrapper | 1.9 | 1.8 | 0.515 |
| walker |  | 760 | 18 | entry item body at src/main.rs:29 body 35 |  |  | 0.525 |
| walker |  | 777 | 17 | entry item body at src/main.rs:29 body 36 |  |  | 0.537 |
| walker |  | 797 | 20 | entry item body at src/main.rs:29 body 45 |  |  | 0.549 |
| walker |  | 814 | 17 | listing of 'src/output' |  |  | 0.567 |
| walker |  | 834 | 20 | listing of 'src/timer' |  |  | 0.599 |
| walker |  | 869 | 35 | pub-item names surface in src/parameter/mod.rs |  |  | 0.599 |
| walker |  | 905 | 36 | pub item at src/parameter/mod.rs:8 |  |  | 0.599 |
| ns | 909 |  | 156 | README features list | 1.10 |  | 0.624 |
| walker |  | 941 | 36 | pub-item names surface in src/timer/mod.rs |  |  | 0.624 |
| walker |  | 941 | 0 | pub item at src/timer/mod.rs:83 |  |  | 0.624 |
| walker |  | 968 | 27 | listing of 'src/benchmark' |  |  | 0.642 |
| ns | 976 |  | 67 | Scheduler struct header + public method names | 2.1 |  | 0.627 |
| walker |  | 1007 | 39 | pub-item names surface in src/benchmark/mod.rs |  |  | 0.627 |
| walker |  | 1007 | 0 | pub item at src/benchmark/mod.rs:32 |  |  | 0.627 |
| ns | 1013 |  | 37 | Benchmark struct header + Benchmark::run signature | 2.2 |  | 0.621 |
| walker |  | 1057 | 50 | pub item at src/benchmark/mod.rs:34 |  |  | 0.621 |
| walker |  | 1085 | 28 | listing of 'src/util' |  |  | 0.679 |
| ns | 1108 |  | 95 | Executor trait header + impl struct names | 2.3 |  | 0.656 |
| walker |  | 1134 | 49 | entry item body at src/main.rs:29 body 37 |  |  | 0.694 |
| walker |  | 1169 | 35 | listing of 'src/export' |  |  | 0.761 |
| walker |  | 1204 | 35 | pub-item names surface in src/export/mod.rs |  |  | 0.761 |
| walker |  | 1225 | 21 | pub item at src/export/mod.rs:56 |  |  | 0.761 |
| walker |  | 1265 | 40 | pub item at src/export/mod.rs:67 |  |  | 0.762 |
| walker |  | 1278 | 13 | pub-item doc lede at src/export/mod.rs:67 |  |  | 0.762 |
| walker |  | 1293 | 15 | pub item at src/util/randomized_environment_offset.rs:6 |  |  | 0.762 |
| walker |  | 1366 | 73 | entry item body at src/main.rs:53 body 54 |  |  | 0.808 |
| walker |  | 1405 | 39 | impl method sigs in src/parameter/mod.rs |  |  | 0.808 |
| ns | 1480 |  | 372 | Options struct field list — first half | 2.4 |  | 0.713 |
| walker |  | 1492 | 87 | pub item at src/timer/mod.rs:42 |  |  | 0.713 |
| walker |  | 1517 | 25 | mod/use plumbing in src/output/mod.rs |  |  | 0.713 |
| walker |  | 1532 | 15 | pub-item doc lede at src/timer/mod.rs:42 |  |  | 0.713 |
| walker |  | 1560 | 28 | pub-item names surface in src/error.rs |  |  | 0.713 |
| walker |  | 1688 | 128 | pub item at src/export/mod.rs:28 |  |  | 0.717 |
| walker |  | 1705 | 17 | pub-item doc lede at src/export/mod.rs:28 |  |  | 0.717 |
| walker |  | 1720 | 15 | pub-item doc lede at src/timer/mod.rs:83 |  |  | 0.669 |
| ns | 1720 |  | 240 | Options struct field list — second half | 2.5 | 2.4 | 0.669 |
| walker |  | 1741 | 21 | pub item at src/export/csv.rs:13 |  |  | 0.669 |
| walker |  | 1762 | 21 | pub item at src/export/json.rs:17 |  |  | 0.669 |
| walker |  | 1783 | 21 | pub item at src/export/markdown.rs:6 |  |  | 0.669 |
| walker |  | 1802 | 19 | pub item at src/parameter/tokenize.rs:1 |  |  | 0.669 |
| walker |  | 1824 | 22 | pub item at src/export/orgmode.rs:5 |  |  | 0.669 |
| walker |  | 1837 | 13 | pub-item doc lede at src/benchmark/mod.rs:32 |  |  | 0.669 |
| ns | 1870 |  | 150 | Options sibling enum/struct headers | 2.6 |  | 0.644 |
| walker |  | 1871 | 34 | pub-item names surface in src/command.rs |  |  | 0.644 |
| walker |  | 1871 | 0 | pub item at src/command.rs:134 |  |  | 0.644 |
| walker |  | 1895 | 24 | pub item at src/export/asciidoc.rs:5 |  |  | 0.644 |
| ns | 1901 |  | 31 | Options::from_cli_arguments + validate_against_command_list signatures | 2.7 |  | 0.640 |
| walker |  | 1923 | 28 | pub item at src/timer/wall_clock_timer.rs:5 |  |  | 0.640 |
| walker |  | 1952 | 29 | pub item at src/timer/windows_timer.rs:49 |  |  | 0.640 |
| walker |  | 1976 | 24 | pub-item names surface in src/export/markup.rs |  |  | 0.640 |
| walker |  | 1993 | 17 | pub item at src/export/markup.rs:10 |  |  | 0.640 |
| walker |  | 2073 | 80 | README.md section #0 |  |  | 0.653 |
| ns | 2088 |  | 187 | Command + Commands type headers + public method names | 2.8 |  | 0.629 |
| walker |  | 2098 | 25 | pub-item names surface in src/parameter/range_step.rs |  |  | 0.629 |
| walker |  | 2123 | 25 | pub-item names surface in src/timer/unix_timer.rs |  |  | 0.629 |
| walker |  | 2140 | 17 | pub item at src/timer/unix_timer.rs:18 |  |  | 0.629 |
| walker |  | 2166 | 26 | pub-item names surface in src/output/warnings.rs |  |  | 0.629 |
| walker |  | 2196 | 30 | pub item at src/output/warnings.rs:7 |  |  | 0.629 |
| walker |  | 2240 | 44 | mod/use plumbing in src/parameter/mod.rs |  |  | 0.629 |
| walker |  | 2284 | 44 | mod/use plumbing in src/util/mod.rs |  |  | 0.629 |
| walker |  | 2317 | 33 | pub item at src/output/progress_bar.rs:13 |  |  | 0.629 |
| walker |  | 2357 | 40 | pub item at src/parameter/range_step.rs:34 |  |  | 0.629 |
| ns | 2384 |  | 296 | BenchmarkResult fields (the in-memory result row) | 2.9 |  | 0.597 |
| walker |  | 2537 | 180 | mod/use plumbing in src/main.rs |  |  | 0.645 |
| walker |  | 2591 | 54 | pub item at src/output/warnings.rs:13 |  |  | 0.646 |
| walker |  | 2661 | 70 | pub item at src/cli.rs:8 |  |  | 0.646 |
| walker |  | 2683 | 22 | pub item body at src/cli.rs:8 body 13 |  |  | 0.646 |
| ns | 2718 |  | 334 | ExportType + Exporter trait + ExportManager headers | 2.10 |  | 0.637 |
| walker |  | 2739 | 56 | pub item at src/util/number.rs:10 |  |  | 0.637 |
| walker |  | 2754 | 15 | pub-item doc lede at src/command.rs:134 |  |  | 0.637 |
| walker |  | 2791 | 37 | pub-item names surface in src/util/units.rs |  |  | 0.637 |
| walker |  | 2791 | 0 | pub item at src/util/units.rs:6 |  |  | 0.637 |
| walker |  | 2836 | 45 | pub item at src/util/units.rs:10 |  |  | 0.637 |
| walker |  | 2843 | 7 | pub-item doc lede at src/util/units.rs:10 |  |  | 0.637 |
| walker |  | 2853 | 10 | pub-item doc lede at src/output/warnings.rs:13 |  |  | 0.637 |
| ns | 2858 |  | 140 | Benchmark module-level helpers + MIN_EXECUTION_TIME | 2.11 |  | 0.622 |
| walker |  | 2925 | 72 | pub-item names surface in src/outlier_detection.rs |  |  | 0.622 |
| walker |  | 2925 | 0 | pub item at src/outlier_detection.rs:13 |  |  | 0.622 |
| walker |  | 2925 | 0 | pub item at src/outlier_detection.rs:21 |  |  | 0.622 |
| walker |  | 2925 | 0 | pub item at src/outlier_detection.rs:43 |  |  | 0.622 |
| walker |  | 2965 | 40 | pub-item names surface in src/util/min_max.rs |  |  | 0.622 |
| walker |  | 2965 | 0 | pub item at src/util/min_max.rs:2 |  |  | 0.622 |
| walker |  | 2965 | 0 | pub item at src/util/min_max.rs:10 |  |  | 0.622 |
| walker |  | 2986 | 21 | pub item body at src/util/randomized_environment_offset.rs:6 body 7 |  |  | 0.622 |
| ns | 3054 |  | 196 | relative_speed + outlier_detection signatures | 2.12 |  | 0.604 |
| walker |  | 3110 | 124 | impl method sigs in src/export/mod.rs |  |  | 0.618 |
| ns | 3131 |  | 77 | tests/ + scripts/ fs listings | 2.13 |  | 0.603 |
| walker |  | 3156 | 46 | pub-item names surface in src/util/exit_code.rs |  |  | 0.603 |
| walker |  | 3156 | 0 | pub item at src/util/exit_code.rs:4 |  |  | 0.603 |
| walker |  | 3156 | 0 | pub item at src/util/exit_code.rs:20 |  |  | 0.603 |
| walker |  | 3164 | 8 | pub item body at src/util/exit_code.rs:20 body 21 |  |  | 0.603 |
| walker |  | 3277 | 113 | pub item at src/command.rs:22 |  |  | 0.604 |
| walker |  | 3290 | 13 | pub-item doc lede at src/command.rs:22 |  |  | 0.604 |
| walker |  | 3359 | 69 | pub item at src/benchmark/scheduler.rs:13 |  |  | 0.605 |
| walker |  | 3458 | 99 | pub item at src/parameter/range_step.rs:7 |  |  | 0.605 |
| walker |  | 3530 | 72 | pub item at src/timer/unix_timer.rs:10 |  |  | 0.605 |
| ns | 3570 |  | 439 | CLI flag names — every Arg::new() line | 3.1 |  | 0.573 |
| walker |  | 3611 | 81 | man-page NAME + DESCRIPTION in doc/hyperfine.1 |  |  | 0.573 |
| walker |  | 3621 | 10 | pub-item doc lede at src/util/units.rs:6 |  |  | 0.568 |
| ns | 3621 |  | 51 | RunBounds default — '10 runs by default' | 3.2 |  | 0.568 |
| walker |  | 3683 | 62 | pub-item names surface in src/benchmark/executor.rs |  |  | 0.574 |
| walker |  | 3699 | 16 | pub item at src/benchmark/executor.rs:122 |  |  | 0.574 |
| walker |  | 3722 | 23 | pub item at src/benchmark/executor.rs:303 |  |  | 0.577 |
| walker |  | 3755 | 33 | pub item at src/benchmark/executor.rs:19 |  |  | 0.581 |
| walker |  | 3797 | 42 | pub item at src/benchmark/executor.rs:169 |  |  | 0.581 |
| walker |  | 3836 | 39 | README.md section #4 |  |  | 0.581 |
| walker |  | 3849 | 13 | pub-item doc lede at src/output/progress_bar.rs:13 |  |  | 0.581 |
| walker |  | 3863 | 14 | pub-item doc lede at src/util/min_max.rs:2 |  |  | 0.581 |
| ns | 3931 |  | 310 | Scheduler::run_benchmarks body | 3.3 | 2.1 | 0.560 |
| walker |  | 4003 | 140 | pub-item names surface in src/options.rs |  |  | 0.581 |
| walker |  | 4040 | 37 | pub item at src/options.rs:102 |  |  | 0.581 |
| walker |  | 4083 | 43 | pub item at src/options.rs:185 |  |  | 0.581 |
| ns | 4141 |  | 210 | Benchmark::run — count-of-runs formula | 3.4 | 2.2 | 0.565 |
| walker |  | 4149 | 66 | pub item at src/options.rs:24 |  |  | 0.565 |
| walker |  | 4161 | 12 | pub-item doc lede at src/options.rs:24 |  |  | 0.565 |
| walker |  | 4218 | 57 | pub item at src/options.rs:108 |  |  | 0.565 |
| walker |  | 4291 | 73 | pub item at src/options.rs:123 |  |  | 0.565 |
| walker |  | 4302 | 11 | pub-item doc lede at src/options.rs:108 |  |  | 0.565 |
| walker |  | 4398 | 96 | pub item at src/options.rs:71 |  |  | 0.565 |
| walker |  | 4412 | 14 | pub-item doc lede at src/options.rs:71 |  |  | 0.565 |
| ns | 4533 |  | 392 | Benchmark::run — warning push logic | 3.5 | 2.2 | 0.538 |
| walker |  | 4542 | 130 | pub item at src/options.rs:149 |  |  | 0.538 |
| walker |  | 4557 | 15 | pub-item doc lede at src/options.rs:149 |  |  | 0.538 |
| walker |  | 4706 | 149 | pub item at src/options.rs:84 |  |  | 0.538 |
| walker |  | 4714 | 8 | pub-item doc lede at src/options.rs:84 |  |  | 0.538 |
| walker |  | 4730 | 16 | pub-item doc lede at src/util/min_max.rs:10 |  |  | 0.538 |
| ns | 4757 |  | 224 | Benchmark::run — BenchmarkResult construction | 3.6 | 2.9 | 0.523 |
| walker |  | 4808 | 78 | pub-item names surface in src/output/format.rs |  |  | 0.523 |
| walker |  | 4808 | 0 | pub item at src/output/format.rs:5 |  |  | 0.523 |
| walker |  | 4808 | 0 | pub item at src/output/format.rs:11 |  |  | 0.523 |
| walker |  | 4808 | 0 | pub item at src/output/format.rs:18 |  |  | 0.523 |
| walker |  | 4835 | 27 | pub item body at src/output/format.rs:5 body 6 |  |  | 0.524 |
| walker |  | 4856 | 21 | listing of 'tests' |  |  | 0.526 |
| walker |  | 5103 | 247 | pub item at src/error.rs:7 |  |  | 0.527 |
| walker |  | 5121 | 18 | pub-item doc lede at src/output/format.rs:11 |  |  | 0.527 |
| walker |  | 5139 | 18 | pub-item doc lede at src/output/format.rs:18 |  |  | 0.527 |
| walker |  | 5183 | 44 | pub item body at src/util/min_max.rs:2 body 3 |  |  | 0.527 |
| walker |  | 5227 | 44 | pub item body at src/util/min_max.rs:10 body 11 |  |  | 0.527 |
| walker |  | 5274 | 47 | pub item body at src/output/format.rs:11 body 12 |  |  | 0.527 |
| ns | 5303 |  | 546 | Per-format markup exporters — Markdown / Orgmode | 3.7 | 2.10 | 0.495 |
| walker |  | 5361 | 87 | pub item body at src/outlier_detection.rs:43 body 44 |  |  | 0.495 |
| walker |  | 5465 | 104 | pub-item names surface in src/benchmark/relative_speed.rs |  |  | 0.503 |
| walker |  | 5465 | 0 | pub item at src/benchmark/relative_speed.rs:16 |  |  | 0.503 |
| walker |  | 5465 | 0 | pub item at src/benchmark/relative_speed.rs:20 |  |  | 0.503 |
| walker |  | 5504 | 39 | pub item at src/benchmark/relative_speed.rs:112 |  |  | 0.503 |
| walker |  | 5544 | 40 | pub item at src/benchmark/relative_speed.rs:98 |  |  | 0.503 |
| walker |  | 5564 | 20 | pub item body at src/benchmark/relative_speed.rs:16 body 17 |  |  | 0.503 |
| walker |  | 5618 | 54 | pub item at src/benchmark/relative_speed.rs:86 |  |  | 0.503 |
| walker |  | 5701 | 83 | pub item at src/benchmark/relative_speed.rs:7 |  |  | 0.521 |
| ns | 5705 |  | 402 | AsciidocExporter body | 3.8 | 2.10 | 0.500 |
| walker |  | 5735 | 34 | pub item body at src/benchmark/relative_speed.rs:112 body 116 |  |  | 0.500 |
| walker |  | 5754 | 19 | pub-item doc lede at src/benchmark/relative_speed.rs:112 |  |  | 0.500 |
| walker |  | 5805 | 51 | pub item body at src/benchmark/relative_speed.rs:20 body 21 |  |  | 0.500 |
| ns | 5925 |  | 220 | JsonExporter body | 3.9 | 2.10 | 0.489 |
| walker |  | 5983 | 178 | mod/use plumbing in src/timer/mod.rs |  |  | 0.489 |
| walker |  | 6123 | 140 | pub item at src/benchmark/timing_result.rs:5 |  |  | 0.489 |
| walker |  | 6135 | 12 | pub-item doc lede at src/benchmark/timing_result.rs:5 |  |  | 0.489 |
| walker |  | 6189 | 54 | README.md section #7 |  |  | 0.489 |
| walker |  | 6268 | 79 | impl method sigs in src/benchmark/scheduler.rs |  |  | 0.497 |
| walker |  | 6314 | 46 | pub-item doc lede at src/outlier_detection.rs:43 |  |  | 0.497 |
| walker |  | 6377 | 63 | pub item body at src/benchmark/relative_speed.rs:86 body 91 |  |  | 0.497 |
| ns | 6466 |  | 541 | CsvExporter body | 3.10 | 2.10 | 0.474 |
| walker |  | 6750 | 373 | pub item at src/error.rs:37 |  |  | 0.475 |
| walker |  | 6976 | 226 | mod/use plumbing in src/export/mod.rs |  |  | 0.475 |
| walker |  | 7045 | 69 | pub item body at src/benchmark/relative_speed.rs:98 body 102 |  |  | 0.475 |
| ns | 7153 |  | 687 | ShellExecutor::calibrate — shell spawning time measurement | 3.11 | 2.3 | 0.448 |
| walker |  | 7460 | 415 | impl method sigs in src/benchmark/mod.rs |  |  | 0.462 |
| ns | 7693 |  | 540 | ShellExecutor::run_command_and_measure body | 3.12 | 3.11 | 0.446 |
| walker |  | 7729 | 269 | pub item at src/benchmark/executor.rs:35 |  |  | 0.453 |
| walker |  | 7915 | 186 | impl method sigs in src/options.rs |  |  | 0.456 |
| walker |  | 7989 | 74 | README.md section #8 |  |  | 0.456 |
| walker |  | 8071 | 82 | README.md section #6 |  |  | 0.456 |
| ns | 8267 |  | 574 | MockExecutor body (debug-mode) | 3.13 | 2.3 | 0.437 |
| walker |  | 8339 | 268 | [dependencies] in Cargo.toml |  |  | 0.437 |
| walker |  | 8669 | 330 | mod/use plumbing in src/benchmark/mod.rs |  |  | 0.437 |
| ns | 8674 |  | 407 | outlier_detection module + OUTLIER_THRESHOLD const | 3.14 | 2.12 | 0.427 |
| walker |  | 8802 | 133 | impl method sigs in src/export/asciidoc.rs |  |  | 0.431 |
| walker |  | 8865 | 63 | pub-item doc lede at src/outlier_detection.rs:13 |  |  | 0.433 |
| walker |  | 8921 | 56 | listing of 'scripts' |  |  | 0.447 |
| walker |  | 9194 | 273 | impl method sigs in src/command.rs |  |  | 0.464 |
| ns | 9234 |  | 560 | format_duration auto-unit selection + Unit::short_name/format | 3.15 |  | 0.455 |
| walker |  | 9410 | 216 | pub item body at src/outlier_detection.rs:21 body 22 |  |  | 0.470 |
| ns | 9422 |  | 188 | Warnings enum + OutlierWarningOptions struct | 3.16 |  | 0.471 |
| walker |  | 9461 | 51 | pub-item doc lede at src/output/format.rs:5 |  |  | 0.471 |
| walker |  | 9585 | 124 | pub item body at src/output/format.rs:18 body 19 |  |  | 0.478 |
| ns | 9811 |  | 389 | OptionsError enum + variants | 4.1 |  | 0.492 |
| ns | 10070 |  | 259 | ParameterScanError enum + variants | 4.2 |  | 0.502 |
| ns | 10204 |  | 134 | common.rs test helpers | 4.3 |  | 0.498 |
