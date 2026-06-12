Score(3000)=0.555 I=0.815 C=0.378 ns_rows≤3K=23/42 (reached=8 partial=1 missing=14)

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 45 | 45 | listing of '.' |  |  | 1.000 |
| ns | 45 |  | 45 | Top-level fs listing | 1.1 |  | 1.000 |
| ns | 63 |  | 18 | README one-line description | 1.2 |  | 0.917 |
| walker |  | 80 | 35 | [features] in Cargo.toml |  |  | 0.917 |
| ns | 107 |  | 44 | src/ fs listing | 1.3 |  | 0.643 |
| ns | 164 |  | 57 | Cargo package identity (name + version + edition) | 1.4 |  | 0.581 |
| walker |  | 194 | 114 | README headline in README.md |  |  | 0.588 |
| ns | 254 |  | 90 | main.rs module declarations | 1.5 |  | 0.492 |
| ns | 316 |  | 62 | src/benchmark/ + src/export/ fs listings | 1.6 |  | 0.416 |
| walker |  | 364 | 170 | [package] in Cargo.toml |  |  | 0.495 |
| ns | 394 |  | 78 | Remaining src/ subdir fs listings | 1.7 |  | 0.424 |
| walker |  | 508 | 144 | README.md section #1 |  |  | 0.429 |
| walker |  | 533 | 25 | listing of 'doc' |  |  | 0.429 |
| ns | 572 |  | 178 | main.rs run() — first half (CLI parsing + builders) | 1.8 |  | 0.385 |
| walker |  | 605 | 72 | README.md section #0 |  |  | 0.408 |
| walker |  | 649 | 44 | listing of 'src' |  |  | 0.549 |
| walker |  | 659 | 10 | entry item at src/main.rs:53 |  |  | 0.549 |
| walker |  | 672 | 13 | listing of 'src/parameter' |  |  | 0.554 |
| walker |  | 685 | 13 | entry item at src/main.rs:29 |  |  | 0.554 |
| walker |  | 692 | 7 | entry item body at src/main.rs:29 body 50 |  |  | 0.554 |
| walker |  | 702 | 10 | entry item body at src/main.rs:29 body 31 |  |  | 0.555 |
| walker |  | 712 | 10 | entry item body at src/main.rs:29 body 48 |  |  | 0.556 |
| walker |  | 724 | 12 | entry item body at src/main.rs:29 body 30 |  |  | 0.558 |
| walker |  | 736 | 12 | entry item body at src/main.rs:29 body 46 |  |  | 0.559 |
| ns | 741 |  | 169 | main.rs run() — second half (Scheduler) + main() wrapper | 1.9 | 1.8 | 0.508 |
| walker |  | 748 | 12 | entry item body at src/main.rs:29 body 47 |  |  | 0.515 |
| walker |  | 762 | 14 | entry item body at src/main.rs:29 body 43 |  |  | 0.519 |
| walker |  | 778 | 16 | entry item body at src/main.rs:29 body 32 |  |  | 0.526 |
| walker |  | 795 | 17 | entry item body at src/main.rs:29 body 34 |  |  | 0.533 |
| walker |  | 812 | 17 | entry item body at src/main.rs:29 body 36 |  |  | 0.544 |
| walker |  | 830 | 18 | entry item body at src/main.rs:29 body 35 |  |  | 0.556 |
| walker |  | 852 | 22 | entry item body at src/main.rs:29 body 45 |  |  | 0.568 |
| walker |  | 869 | 17 | listing of 'src/output' |  |  | 0.586 |
| walker |  | 889 | 20 | listing of 'src/timer' |  |  | 0.617 |
| ns | 895 |  | 154 | README features list | 1.10 |  | 0.636 |
| walker |  | 922 | 33 | pub-item names surface in src/parameter/mod.rs |  |  | 0.636 |
| walker |  | 941 | 19 | pub item at src/parameter/mod.rs:8 |  |  | 0.636 |
| ns | 950 |  | 55 | Scheduler struct header + public method names | 2.1 |  | 0.621 |
| walker |  | 975 | 34 | pub-item names surface in src/timer/mod.rs |  |  | 0.621 |
| walker |  | 975 | 0 | pub item at src/timer/mod.rs:83 |  |  | 0.621 |
| ns | 979 |  | 29 | Benchmark struct header + Benchmark::run signature | 2.2 |  | 0.612 |
| walker |  | 1003 | 28 | listing of 'src/util' |  |  | 0.672 |
| walker |  | 1054 | 51 | entry item body at src/main.rs:29 body 37 |  |  | 0.713 |
| ns | 1058 |  | 79 | Executor trait header + impl struct names | 2.3 |  | 0.687 |
| walker |  | 1089 | 35 | listing of 'src/export' |  |  | 0.714 |
| walker |  | 1122 | 33 | pub-item names surface in src/export/mod.rs |  |  | 0.714 |
| walker |  | 1141 | 19 | pub item at src/export/mod.rs:56 |  |  | 0.714 |
| walker |  | 1179 | 38 | pub item at src/export/mod.rs:67 |  |  | 0.715 |
| walker |  | 1192 | 13 | pub item at src/util/randomized_environment_offset.rs:6 |  |  | 0.715 |
| walker |  | 1265 | 73 | pub item at src/timer/mod.rs:42 |  |  | 0.715 |
| walker |  | 1363 | 98 | pub item at src/export/mod.rs:28 |  |  | 0.718 |
| walker |  | 1376 | 13 | pub-item doc lede at src/export/mod.rs:67 |  |  | 0.718 |
| ns | 1426 |  | 368 | Options struct field list — first half | 2.4 |  | 0.634 |
| walker |  | 1447 | 71 | entry item body at src/main.rs:53 body 54 |  |  | 0.676 |
| walker |  | 1473 | 26 | pub-item names surface in src/error.rs |  |  | 0.676 |
| walker |  | 1490 | 17 | pub-item doc lede at src/export/mod.rs:28 |  |  | 0.676 |
| walker |  | 1515 | 25 | mod/use plumbing in src/output/mod.rs |  |  | 0.676 |
| walker |  | 1556 | 41 | impl method sigs in src/parameter/mod.rs |  |  | 0.676 |
| walker |  | 1571 | 15 | pub-item doc lede at src/timer/mod.rs:42 |  |  | 0.676 |
| walker |  | 1586 | 15 | pub-item doc lede at src/timer/mod.rs:83 |  |  | 0.676 |
| walker |  | 1605 | 19 | pub item at src/parameter/tokenize.rs:1 |  |  | 0.676 |
| walker |  | 1637 | 32 | pub-item names surface in src/command.rs |  |  | 0.676 |
| walker |  | 1637 | 0 | pub item at src/command.rs:134 |  |  | 0.676 |
| walker |  | 1661 | 24 | pub item at src/timer/wall_clock_timer.rs:5 |  |  | 0.676 |
| ns | 1666 |  | 240 | Options struct field list — second half | 2.5 | 2.4 | 0.631 |
| walker |  | 1692 | 31 | pub item at src/util/number.rs:10 |  |  | 0.631 |
| walker |  | 1717 | 25 | pub item at src/timer/windows_timer.rs:49 |  |  | 0.631 |
| walker |  | 1739 | 22 | pub-item names surface in src/export/markup.rs |  |  | 0.631 |
| walker |  | 1756 | 17 | pub item at src/export/markup.rs:10 |  |  | 0.631 |
| walker |  | 1779 | 23 | pub-item names surface in src/parameter/range_step.rs |  |  | 0.631 |
| ns | 1798 |  | 132 | Options sibling enum/struct headers | 2.6 |  | 0.607 |
| walker |  | 1802 | 23 | pub-item names surface in src/timer/unix_timer.rs |  |  | 0.607 |
| walker |  | 1817 | 15 | pub item at src/timer/unix_timer.rs:18 |  |  | 0.607 |
| ns | 1825 |  | 27 | Options::from_cli_arguments + validate_against_command_list signatures | 2.7 |  | 0.603 |
| walker |  | 1841 | 24 | pub-item names surface in src/output/warnings.rs |  |  | 0.603 |
| walker |  | 1869 | 28 | pub item at src/output/warnings.rs:7 |  |  | 0.604 |
| walker |  | 1899 | 30 | pub item at src/parameter/range_step.rs:34 |  |  | 0.604 |
| walker |  | 1940 | 41 | mod/use plumbing in src/parameter/mod.rs |  |  | 0.604 |
| walker |  | 1984 | 44 | mod/use plumbing in src/util/mod.rs |  |  | 0.582 |
| ns | 1984 |  | 159 | Command + Commands type headers + public method names | 2.8 |  | 0.582 |
| walker |  | 2015 | 31 | pub item at src/output/progress_bar.rs:13 |  |  | 0.582 |
| walker |  | 2180 | 165 | mod/use plumbing in src/main.rs |  |  | 0.634 |
| walker |  | 2232 | 52 | pub item at src/output/warnings.rs:13 |  |  | 0.634 |
| ns | 2254 |  | 270 | BenchmarkResult fields (the in-memory result row) | 2.9 |  | 0.601 |
| walker |  | 2300 | 68 | pub item at src/cli.rs:8 |  |  | 0.601 |
| walker |  | 2320 | 20 | pub item body at src/cli.rs:8 body 13 |  |  | 0.601 |
| walker |  | 2355 | 35 | pub-item names surface in src/util/units.rs |  |  | 0.601 |
| walker |  | 2355 | 0 | pub item at src/util/units.rs:6 |  |  | 0.601 |
| walker |  | 2381 | 26 | pub item at src/util/units.rs:10 |  |  | 0.601 |
| walker |  | 2390 | 9 | pub-item doc lede at src/util/units.rs:10 |  |  | 0.601 |
| walker |  | 2405 | 15 | pub-item doc lede at src/command.rs:134 |  |  | 0.601 |
| walker |  | 2443 | 38 | pub-item names surface in src/util/min_max.rs |  |  | 0.601 |
| walker |  | 2443 | 0 | pub item at src/util/min_max.rs:2 |  |  | 0.601 |
| walker |  | 2443 | 0 | pub item at src/util/min_max.rs:10 |  |  | 0.601 |
| walker |  | 2529 | 86 | pub item at src/command.rs:22 |  |  | 0.601 |
| walker |  | 2542 | 13 | pub-item doc lede at src/command.rs:22 |  |  | 0.601 |
| walker |  | 2561 | 19 | pub item body at src/util/randomized_environment_offset.rs:6 body 7 |  |  | 0.601 |
| ns | 2570 |  | 316 | ExportType + Exporter trait + ExportManager headers | 2.10 |  | 0.587 |
| walker |  | 2631 | 70 | pub-item names surface in src/outlier_detection.rs |  |  | 0.587 |
| walker |  | 2631 | 0 | pub item at src/outlier_detection.rs:13 |  |  | 0.587 |
| walker |  | 2631 | 0 | pub item at src/outlier_detection.rs:21 |  |  | 0.587 |
| walker |  | 2631 | 0 | pub item at src/outlier_detection.rs:43 |  |  | 0.587 |
| walker |  | 2675 | 44 | pub-item names surface in src/util/exit_code.rs |  |  | 0.587 |
| walker |  | 2675 | 0 | pub item at src/util/exit_code.rs:4 |  |  | 0.587 |
| walker |  | 2675 | 0 | pub item at src/util/exit_code.rs:20 |  |  | 0.587 |
| walker |  | 2681 | 6 | pub item body at src/util/exit_code.rs:20 body 21 |  |  | 0.587 |
| ns | 2696 |  | 126 | Benchmark module-level helpers + MIN_EXECUTION_TIME | 2.11 |  | 0.573 |
| walker |  | 2736 | 55 | pub item at src/timer/unix_timer.rs:10 |  |  | 0.573 |
| walker |  | 2748 | 12 | pub-item doc lede at src/output/warnings.rs:13 |  |  | 0.573 |
| walker |  | 2872 | 124 | impl method sigs in src/export/mod.rs |  |  | 0.585 |
| ns | 2874 |  | 178 | relative_speed + outlier_detection signatures | 2.12 |  | 0.569 |
| walker |  | 2904 | 32 | README.md section #4 |  |  | 0.569 |
| ns | 2951 |  | 77 | tests/ + scripts/ fs listings | 2.13 |  | 0.555 |
| walker |  | 2979 | 75 | man-page NAME + DESCRIPTION in doc/hyperfine.1 |  |  | 0.555 |
| walker |  | 3076 | 97 | pub item at src/parameter/range_step.rs:7 |  |  | 0.555 |
| walker |  | 3089 | 13 | pub-item doc lede at src/output/progress_bar.rs:13 |  |  | 0.555 |
| walker |  | 3101 | 12 | pub-item doc lede at src/util/units.rs:6 |  |  | 0.555 |
| walker |  | 3177 | 76 | pub-item names surface in src/output/format.rs |  |  | 0.555 |
| walker |  | 3177 | 0 | pub item at src/output/format.rs:5 |  |  | 0.555 |
| walker |  | 3177 | 0 | pub item at src/output/format.rs:11 |  |  | 0.555 |
| walker |  | 3177 | 0 | pub item at src/output/format.rs:18 |  |  | 0.555 |
| walker |  | 3202 | 25 | pub item body at src/output/format.rs:5 body 6 |  |  | 0.555 |
| ns | 3326 |  | 375 | CLI flag names — every Arg::new() line | 3.1 |  | 0.526 |
| walker |  | 3340 | 138 | pub-item names surface in src/options.rs |  |  | 0.550 |
| walker |  | 3358 | 18 | pub item at src/options.rs:102 |  |  | 0.550 |
| ns | 3375 |  | 49 | RunBounds default — '10 runs by default' | 3.2 |  | 0.545 |
| walker |  | 3388 | 30 | pub item at src/options.rs:185 |  |  | 0.545 |
| walker |  | 3436 | 48 | pub item at src/options.rs:24 |  |  | 0.545 |
| walker |  | 3487 | 51 | pub item at src/options.rs:123 |  |  | 0.545 |
| walker |  | 3537 | 50 | pub item at src/options.rs:108 |  |  | 0.545 |
| walker |  | 3606 | 69 | pub item at src/options.rs:71 |  |  | 0.545 |
| walker |  | 3620 | 14 | pub-item doc lede at src/options.rs:24 |  |  | 0.545 |
| walker |  | 3634 | 14 | pub-item doc lede at src/options.rs:71 |  |  | 0.545 |
| walker |  | 3647 | 13 | pub-item doc lede at src/options.rs:108 |  |  | 0.545 |
| ns | 3685 |  | 310 | Scheduler::run_benchmarks body | 3.3 | 2.1 | 0.525 |
| walker |  | 3743 | 96 | pub item at src/options.rs:149 |  |  | 0.525 |
| walker |  | 3758 | 15 | pub-item doc lede at src/options.rs:149 |  |  | 0.525 |
| walker |  | 3870 | 112 | pub item at src/options.rs:84 |  |  | 0.525 |
| walker |  | 3880 | 10 | pub-item doc lede at src/options.rs:84 |  |  | 0.525 |
| ns | 3893 |  | 208 | Benchmark::run — count-of-runs formula | 3.4 | 2.2 | 0.510 |
| walker |  | 3896 | 16 | pub-item doc lede at src/util/min_max.rs:2 |  |  | 0.510 |
| walker |  | 3912 | 16 | pub-item doc lede at src/util/min_max.rs:10 |  |  | 0.510 |
| walker |  | 3952 | 40 | pub item body at src/output/format.rs:11 body 12 |  |  | 0.510 |
| walker |  | 3973 | 21 | listing of 'tests' |  |  | 0.513 |
| walker |  | 4208 | 235 | pub item at src/error.rs:7 |  |  | 0.514 |
| walker |  | 4250 | 42 | pub item body at src/util/min_max.rs:2 body 3 |  |  | 0.514 |
| ns | 4283 |  | 390 | Benchmark::run — warning push logic | 3.5 | 2.2 | 0.489 |
| walker |  | 4292 | 42 | pub item body at src/util/min_max.rs:10 body 11 |  |  | 0.489 |
| walker |  | 4310 | 18 | pub-item doc lede at src/output/format.rs:11 |  |  | 0.489 |
| walker |  | 4328 | 18 | pub-item doc lede at src/output/format.rs:18 |  |  | 0.489 |
| walker |  | 4408 | 80 | pub item body at src/outlier_detection.rs:43 body 44 |  |  | 0.489 |
| ns | 4505 |  | 222 | Benchmark::run — BenchmarkResult construction | 3.6 | 2.9 | 0.476 |
| walker |  | 4559 | 151 | mod/use plumbing in src/timer/mod.rs |  |  | 0.476 |
| walker |  | 4606 | 47 | README.md section #7 |  |  | 0.476 |
| walker |  | 4650 | 44 | pub-item doc lede at src/outlier_detection.rs:43 |  |  | 0.476 |
| walker |  | 4856 | 206 | mod/use plumbing in src/export/mod.rs |  |  | 0.476 |
| ns | 5047 |  | 542 | Per-format markup exporters — Markdown / Orgmode | 3.7 | 2.10 | 0.447 |
| walker |  | 5219 | 363 | pub item at src/error.rs:37 |  |  | 0.448 |
| walker |  | 5246 | 27 | listing of 'src/benchmark' |  |  | 0.472 |
| walker |  | 5312 | 66 | README.md section #8 |  |  | 0.472 |
| walker |  | 5389 | 77 | README.md section #6 |  |  | 0.472 |
| ns | 5447 |  | 400 | AsciidocExporter body | 3.8 | 2.10 | 0.453 |
| walker |  | 5653 | 264 | [dependencies] in Cargo.toml |  |  | 0.453 |
| ns | 5665 |  | 218 | JsonExporter body | 3.9 | 2.10 | 0.443 |
| walker |  | 5716 | 63 | pub-item doc lede at src/outlier_detection.rs:13 |  |  | 0.443 |
| walker |  | 5905 | 189 | pub item body at src/outlier_detection.rs:21 body 22 |  |  | 0.443 |
| walker |  | 5961 | 56 | listing of 'scripts' |  |  | 0.463 |
| ns | 6202 |  | 537 | CsvExporter body | 3.10 | 2.10 | 0.441 |
| walker |  | 6464 | 503 | pub item at src/options.rs:198 |  |  | 0.498 |
| walker |  | 6479 | 15 | pub-item doc lede at src/options.rs:198 |  |  | 0.502 |
| walker |  | 6601 | 122 | pub item body at src/output/format.rs:18 body 19 |  |  | 0.502 |
| walker |  | 6652 | 51 | pub-item doc lede at src/output/format.rs:5 |  |  | 0.502 |
| walker |  | 6758 | 106 | pub-item doc lede at src/outlier_detection.rs:21 |  |  | 0.502 |
| ns | 6887 |  | 685 | ShellExecutor::calibrate — shell spawning time measurement | 3.11 | 2.3 | 0.473 |
| walker |  | 6938 | 180 | pub item body at src/util/exit_code.rs:4 body 5 |  |  | 0.473 |
| ns | 7425 |  | 538 | ShellExecutor::run_command_and_measure body | 3.12 | 3.11 | 0.456 |
| walker |  | 7784 | 846 | pub item at src/export/markup.rs:15 |  |  | 0.456 |
| walker |  | 7893 | 109 | pub-item doc lede at src/util/randomized_environment_offset.rs:6 |  |  | 0.456 |
| ns | 7997 |  | 572 | MockExecutor body (debug-mode) | 3.13 | 2.3 | 0.437 |
| walker |  | 8087 | 194 | README.md section #5 |  |  | 0.437 |
| walker |  | 8121 | 34 | pub-item names surface in src/benchmark/mod.rs |  |  | 0.438 |
| walker |  | 8121 | 0 | pub item at src/benchmark/mod.rs:32 |  |  | 0.438 |
| walker |  | 8129 | 8 | listing of '.github' |  |  | 0.438 |
| walker |  | 8134 | 5 | listing of '.github/workflows' |  |  | 0.438 |
| walker |  | 8182 | 48 | pub item at src/benchmark/mod.rs:34 |  |  | 0.438 |
| walker |  | 8200 | 18 | pub item at tests/integration_tests.rs:12 |  |  | 0.438 |
| walker |  | 8213 | 13 | pub-item doc lede at src/benchmark/mod.rs:32 |  |  | 0.438 |
| walker |  | 8246 | 33 | pub-item names surface in tests/common.rs |  |  | 0.438 |
| walker |  | 8246 | 0 | pub item at tests/common.rs:5 |  |  | 0.438 |
| walker |  | 8246 | 0 | pub item at tests/common.rs:11 |  |  | 0.438 |
| walker |  | 8262 | 16 | pub item body at tests/common.rs:11 body 12 |  |  | 0.438 |
| walker |  | 8269 | 7 | pub item body at src/parameter/tokenize.rs:1 body 30 |  |  | 0.438 |
| walker |  | 8277 | 8 | pub item body at src/output/progress_bar.rs:13 body 30 |  |  | 0.438 |
| walker |  | 8368 | 91 | README headline in scripts/README.md |  |  | 0.438 |
| walker |  | 8377 | 9 | pub item body at src/parameter/tokenize.rs:1 body 28 |  |  | 0.438 |
| walker |  | 8405 | 28 | pub item body at tests/integration_tests.rs:12 body 13 |  |  | 0.438 |
| ns | 8406 |  | 409 | outlier_detection module + OUTLIER_THRESHOLD const | 3.14 | 2.12 | 0.441 |
| walker |  | 8415 | 10 | pub item body at src/parameter/tokenize.rs:1 body 2 |  |  | 0.441 |
| walker |  | 8450 | 35 | pub item body at tests/common.rs:5 body 6 |  |  | 0.441 |
| walker |  | 8463 | 13 | pub item body at src/output/progress_bar.rs:13 body 26 |  |  | 0.441 |
| walker |  | 8476 | 13 | pub item body at src/output/progress_bar.rs:13 body 28 |  |  | 0.441 |
| walker |  | 8489 | 13 | pub item body at src/parameter/tokenize.rs:1 body 3 |  |  | 0.441 |
| walker |  | 8502 | 13 | pub item body at src/parameter/tokenize.rs:1 body 5 |  |  | 0.441 |
| walker |  | 8567 | 65 | pub item at src/benchmark/scheduler.rs:13 |  |  | 0.442 |
| ns | 8958 |  | 552 | format_duration auto-unit selection + Unit::short_name/format | 3.15 |  | 0.440 |
| ns | 9144 |  | 186 | Warnings enum + OutlierWarningOptions struct | 3.16 |  | 0.442 |
| ns | 9531 |  | 387 | OptionsError enum + variants | 4.1 |  | 0.457 |
| ns | 9788 |  | 257 | ParameterScanError enum + variants | 4.2 |  | 0.466 |
| ns | 9922 |  | 134 | common.rs test helpers | 4.3 |  | 0.467 |
