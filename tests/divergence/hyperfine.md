Score(3000)=0.581 I=0.844 C=0.400 ns_rows≤3K=23/42 (reached=9 partial=1 missing=13)

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
| ns | 950 |  | 55 | Scheduler struct header + public method names | 2.1 |  | 0.621 |
| walker |  | 956 | 34 | pub item at src/parameter/mod.rs:8 |  |  | 0.621 |
| ns | 979 |  | 29 | Benchmark struct header + Benchmark::run signature | 2.2 |  | 0.612 |
| walker |  | 990 | 34 | pub-item names surface in src/timer/mod.rs |  |  | 0.612 |
| walker |  | 990 | 0 | pub item at src/timer/mod.rs:83 |  |  | 0.612 |
| walker |  | 1017 | 27 | listing of 'src/benchmark' |  |  | 0.629 |
| walker |  | 1051 | 34 | pub-item names surface in src/benchmark/mod.rs |  |  | 0.632 |
| walker |  | 1051 | 0 | pub item at src/benchmark/mod.rs:32 |  |  | 0.632 |
| ns | 1058 |  | 79 | Executor trait header + impl struct names | 2.3 |  | 0.609 |
| walker |  | 1099 | 48 | pub item at src/benchmark/mod.rs:34 |  |  | 0.609 |
| walker |  | 1127 | 28 | listing of 'src/util' |  |  | 0.666 |
| walker |  | 1178 | 51 | entry item body at src/main.rs:29 body 37 |  |  | 0.705 |
| walker |  | 1213 | 35 | listing of 'src/export' |  |  | 0.772 |
| walker |  | 1246 | 33 | pub-item names surface in src/export/mod.rs |  |  | 0.772 |
| walker |  | 1265 | 19 | pub item at src/export/mod.rs:56 |  |  | 0.772 |
| walker |  | 1303 | 38 | pub item at src/export/mod.rs:67 |  |  | 0.773 |
| walker |  | 1316 | 13 | pub item at src/util/randomized_environment_offset.rs:6 |  |  | 0.773 |
| walker |  | 1329 | 13 | pub-item doc lede at src/export/mod.rs:67 |  |  | 0.773 |
| walker |  | 1400 | 71 | entry item body at src/main.rs:53 body 54 |  |  | 0.819 |
| walker |  | 1417 | 17 | pub item at src/export/csv.rs:13 |  |  | 0.819 |
| ns | 1426 |  | 368 | Options struct field list — first half | 2.4 |  | 0.724 |
| walker |  | 1434 | 17 | pub item at src/export/json.rs:17 |  |  | 0.724 |
| walker |  | 1451 | 17 | pub item at src/export/markdown.rs:6 |  |  | 0.724 |
| walker |  | 1557 | 106 | pub item at src/export/mod.rs:28 |  |  | 0.727 |
| walker |  | 1642 | 85 | pub item at src/timer/mod.rs:42 |  |  | 0.727 |
| ns | 1666 |  | 240 | Options struct field list — second half | 2.5 | 2.4 | 0.678 |
| walker |  | 1668 | 26 | pub-item names surface in src/error.rs |  |  | 0.678 |
| walker |  | 1685 | 17 | pub-item doc lede at src/export/mod.rs:28 |  |  | 0.678 |
| walker |  | 1703 | 18 | pub item at src/export/orgmode.rs:5 |  |  | 0.678 |
| walker |  | 1728 | 25 | mod/use plumbing in src/output/mod.rs |  |  | 0.678 |
| walker |  | 1769 | 41 | impl method sigs in src/parameter/mod.rs |  |  | 0.678 |
| walker |  | 1784 | 15 | pub-item doc lede at src/timer/mod.rs:42 |  |  | 0.678 |
| ns | 1798 |  | 132 | Options sibling enum/struct headers | 2.6 |  | 0.652 |
| walker |  | 1804 | 20 | pub item at src/export/asciidoc.rs:5 |  |  | 0.652 |
| walker |  | 1819 | 15 | pub-item doc lede at src/timer/mod.rs:83 |  |  | 0.652 |
| ns | 1825 |  | 27 | Options::from_cli_arguments + validate_against_command_list signatures | 2.7 |  | 0.648 |
| walker |  | 1838 | 19 | pub item at src/parameter/tokenize.rs:1 |  |  | 0.648 |
| walker |  | 1870 | 32 | pub-item names surface in src/command.rs |  |  | 0.649 |
| walker |  | 1870 | 0 | pub item at src/command.rs:134 |  |  | 0.649 |
| walker |  | 1883 | 13 | pub-item doc lede at src/benchmark/mod.rs:32 |  |  | 0.649 |
| walker |  | 1907 | 24 | pub item at src/timer/wall_clock_timer.rs:5 |  |  | 0.649 |
| walker |  | 1932 | 25 | pub item at src/timer/windows_timer.rs:49 |  |  | 0.649 |
| walker |  | 1954 | 22 | pub-item names surface in src/export/markup.rs |  |  | 0.649 |
| walker |  | 1971 | 17 | pub item at src/export/markup.rs:10 |  |  | 0.649 |
| ns | 1984 |  | 159 | Command + Commands type headers + public method names | 2.8 |  | 0.625 |
| walker |  | 1994 | 23 | pub-item names surface in src/parameter/range_step.rs |  |  | 0.625 |
| walker |  | 2017 | 23 | pub-item names surface in src/timer/unix_timer.rs |  |  | 0.625 |
| walker |  | 2032 | 15 | pub item at src/timer/unix_timer.rs:18 |  |  | 0.625 |
| walker |  | 2056 | 24 | pub-item names surface in src/output/warnings.rs |  |  | 0.625 |
| walker |  | 2084 | 28 | pub item at src/output/warnings.rs:7 |  |  | 0.625 |
| walker |  | 2125 | 41 | mod/use plumbing in src/parameter/mod.rs |  |  | 0.625 |
| walker |  | 2169 | 44 | mod/use plumbing in src/util/mod.rs |  |  | 0.625 |
| walker |  | 2200 | 31 | pub item at src/output/progress_bar.rs:13 |  |  | 0.625 |
| walker |  | 2238 | 38 | pub item at src/parameter/range_step.rs:34 |  |  | 0.625 |
| ns | 2254 |  | 270 | BenchmarkResult fields (the in-memory result row) | 2.9 |  | 0.592 |
| walker |  | 2403 | 165 | mod/use plumbing in src/main.rs |  |  | 0.641 |
| walker |  | 2453 | 50 | pub item at src/util/number.rs:10 |  |  | 0.641 |
| walker |  | 2505 | 52 | pub item at src/output/warnings.rs:13 |  |  | 0.642 |
| ns | 2570 |  | 316 | ExportType + Exporter trait + ExportManager headers | 2.10 |  | 0.626 |
| walker |  | 2573 | 68 | pub item at src/cli.rs:8 |  |  | 0.626 |
| walker |  | 2593 | 20 | pub item body at src/cli.rs:8 body 13 |  |  | 0.626 |
| walker |  | 2628 | 35 | pub-item names surface in src/util/units.rs |  |  | 0.626 |
| walker |  | 2628 | 0 | pub item at src/util/units.rs:6 |  |  | 0.626 |
| walker |  | 2671 | 43 | pub item at src/util/units.rs:10 |  |  | 0.626 |
| walker |  | 2680 | 9 | pub-item doc lede at src/util/units.rs:10 |  |  | 0.626 |
| walker |  | 2695 | 15 | pub-item doc lede at src/command.rs:134 |  |  | 0.626 |
| ns | 2696 |  | 126 | Benchmark module-level helpers + MIN_EXECUTION_TIME | 2.11 |  | 0.611 |
| walker |  | 2733 | 38 | pub-item names surface in src/util/min_max.rs |  |  | 0.611 |
| walker |  | 2733 | 0 | pub item at src/util/min_max.rs:2 |  |  | 0.611 |
| walker |  | 2733 | 0 | pub item at src/util/min_max.rs:10 |  |  | 0.611 |
| walker |  | 2752 | 19 | pub item body at src/util/randomized_environment_offset.rs:6 body 7 |  |  | 0.611 |
| walker |  | 2822 | 70 | pub-item names surface in src/outlier_detection.rs |  |  | 0.612 |
| walker |  | 2822 | 0 | pub item at src/outlier_detection.rs:13 |  |  | 0.612 |
| walker |  | 2822 | 0 | pub item at src/outlier_detection.rs:21 |  |  | 0.612 |
| walker |  | 2822 | 0 | pub item at src/outlier_detection.rs:43 |  |  | 0.612 |
| walker |  | 2866 | 44 | pub-item names surface in src/util/exit_code.rs |  |  | 0.612 |
| walker |  | 2866 | 0 | pub item at src/util/exit_code.rs:4 |  |  | 0.612 |
| walker |  | 2866 | 0 | pub item at src/util/exit_code.rs:20 |  |  | 0.612 |
| walker |  | 2872 | 6 | pub item body at src/util/exit_code.rs:20 body 21 |  |  | 0.612 |
| ns | 2874 |  | 178 | relative_speed + outlier_detection signatures | 2.12 |  | 0.595 |
| walker |  | 2884 | 12 | pub-item doc lede at src/output/warnings.rs:13 |  |  | 0.595 |
| ns | 2951 |  | 77 | tests/ + scripts/ fs listings | 2.13 |  | 0.580 |
| walker |  | 2985 | 101 | pub item at src/command.rs:22 |  |  | 0.581 |
| walker |  | 2998 | 13 | pub-item doc lede at src/command.rs:22 |  |  | 0.581 |
| walker |  | 3122 | 124 | impl method sigs in src/export/mod.rs |  |  | 0.594 |
| walker |  | 3154 | 32 | README.md section #4 |  |  | 0.594 |
| walker |  | 3219 | 65 | pub item at src/benchmark/scheduler.rs:13 |  |  | 0.594 |
| walker |  | 3286 | 67 | pub item at src/timer/unix_timer.rs:10 |  |  | 0.594 |
| ns | 3326 |  | 375 | CLI flag names — every Arg::new() line | 3.1 |  | 0.563 |
| walker |  | 3361 | 75 | man-page NAME + DESCRIPTION in doc/hyperfine.1 |  |  | 0.563 |
| ns | 3375 |  | 49 | RunBounds default — '10 runs by default' | 3.2 |  | 0.558 |
| walker |  | 3458 | 97 | pub item at src/parameter/range_step.rs:7 |  |  | 0.558 |
| walker |  | 3518 | 60 | pub-item names surface in src/benchmark/executor.rs |  |  | 0.564 |
| walker |  | 3532 | 14 | pub item at src/benchmark/executor.rs:122 |  |  | 0.564 |
| walker |  | 3553 | 21 | pub item at src/benchmark/executor.rs:303 |  |  | 0.567 |
| walker |  | 3584 | 31 | pub item at src/benchmark/executor.rs:19 |  |  | 0.572 |
| walker |  | 3624 | 40 | pub item at src/benchmark/executor.rs:169 |  |  | 0.572 |
| walker |  | 3637 | 13 | pub-item doc lede at src/output/progress_bar.rs:13 |  |  | 0.572 |
| walker |  | 3649 | 12 | pub-item doc lede at src/util/units.rs:6 |  |  | 0.572 |
| ns | 3685 |  | 310 | Scheduler::run_benchmarks body | 3.3 | 2.1 | 0.550 |
| walker |  | 3725 | 76 | pub-item names surface in src/output/format.rs |  |  | 0.550 |
| walker |  | 3725 | 0 | pub item at src/output/format.rs:5 |  |  | 0.550 |
| walker |  | 3725 | 0 | pub item at src/output/format.rs:11 |  |  | 0.550 |
| walker |  | 3725 | 0 | pub item at src/output/format.rs:18 |  |  | 0.550 |
| walker |  | 3750 | 25 | pub item body at src/output/format.rs:5 body 6 |  |  | 0.550 |
| walker |  | 3888 | 138 | pub-item names surface in src/options.rs |  |  | 0.572 |
| ns | 3893 |  | 208 | Benchmark::run — count-of-runs formula | 3.4 | 2.2 | 0.556 |
| walker |  | 3923 | 35 | pub item at src/options.rs:102 |  |  | 0.556 |
| walker |  | 3964 | 41 | pub item at src/options.rs:185 |  |  | 0.556 |
| walker |  | 4023 | 59 | pub item at src/options.rs:24 |  |  | 0.556 |
| walker |  | 4073 | 50 | pub item at src/options.rs:108 |  |  | 0.556 |
| walker |  | 4139 | 66 | pub item at src/options.rs:123 |  |  | 0.556 |
| walker |  | 4153 | 14 | pub-item doc lede at src/options.rs:24 |  |  | 0.556 |
| walker |  | 4237 | 84 | pub item at src/options.rs:71 |  |  | 0.556 |
| walker |  | 4251 | 14 | pub-item doc lede at src/options.rs:71 |  |  | 0.556 |
| walker |  | 4264 | 13 | pub-item doc lede at src/options.rs:108 |  |  | 0.556 |
| ns | 4283 |  | 390 | Benchmark::run — warning push logic | 3.5 | 2.2 | 0.530 |
| walker |  | 4377 | 113 | pub item at src/options.rs:149 |  |  | 0.530 |
| walker |  | 4392 | 15 | pub-item doc lede at src/options.rs:149 |  |  | 0.530 |
| ns | 4505 |  | 222 | Benchmark::run — BenchmarkResult construction | 3.6 | 2.9 | 0.515 |
| walker |  | 4521 | 129 | pub item at src/options.rs:84 |  |  | 0.515 |
| walker |  | 4531 | 10 | pub-item doc lede at src/options.rs:84 |  |  | 0.515 |
| walker |  | 4547 | 16 | pub-item doc lede at src/util/min_max.rs:2 |  |  | 0.515 |
| walker |  | 4563 | 16 | pub-item doc lede at src/util/min_max.rs:10 |  |  | 0.515 |
| walker |  | 4603 | 40 | pub item body at src/output/format.rs:11 body 12 |  |  | 0.515 |
| walker |  | 4624 | 21 | listing of 'tests' |  |  | 0.518 |
| walker |  | 4666 | 42 | pub item body at src/util/min_max.rs:2 body 3 |  |  | 0.518 |
| walker |  | 4708 | 42 | pub item body at src/util/min_max.rs:10 body 11 |  |  | 0.518 |
| walker |  | 4953 | 245 | pub item at src/error.rs:7 |  |  | 0.519 |
| walker |  | 4971 | 18 | pub-item doc lede at src/output/format.rs:11 |  |  | 0.519 |
| walker |  | 4989 | 18 | pub-item doc lede at src/output/format.rs:18 |  |  | 0.519 |
| ns | 5047 |  | 542 | Per-format markup exporters — Markdown / Orgmode | 3.7 | 2.10 | 0.488 |
| walker |  | 5069 | 80 | pub item body at src/outlier_detection.rs:43 body 44 |  |  | 0.488 |
| walker |  | 5220 | 151 | mod/use plumbing in src/timer/mod.rs |  |  | 0.488 |
| walker |  | 5343 | 123 | pub item at src/benchmark/timing_result.rs:5 |  |  | 0.488 |
| walker |  | 5355 | 12 | pub-item doc lede at src/benchmark/timing_result.rs:5 |  |  | 0.488 |
| walker |  | 5402 | 47 | README.md section #7 |  |  | 0.488 |
| ns | 5447 |  | 400 | AsciidocExporter body | 3.8 | 2.10 | 0.468 |
| walker |  | 5504 | 102 | pub-item names surface in src/benchmark/relative_speed.rs |  |  | 0.475 |
| walker |  | 5504 | 0 | pub item at src/benchmark/relative_speed.rs:16 |  |  | 0.475 |
| walker |  | 5504 | 0 | pub item at src/benchmark/relative_speed.rs:20 |  |  | 0.475 |
| walker |  | 5543 | 39 | pub item at src/benchmark/relative_speed.rs:112 |  |  | 0.475 |
| walker |  | 5583 | 40 | pub item at src/benchmark/relative_speed.rs:98 |  |  | 0.475 |
| walker |  | 5601 | 18 | pub item body at src/benchmark/relative_speed.rs:16 body 17 |  |  | 0.475 |
| walker |  | 5655 | 54 | pub item at src/benchmark/relative_speed.rs:86 |  |  | 0.475 |
| ns | 5665 |  | 218 | JsonExporter body | 3.9 | 2.10 | 0.464 |
| walker |  | 5682 | 27 | pub item body at src/benchmark/relative_speed.rs:112 body 116 |  |  | 0.464 |
| walker |  | 5765 | 83 | pub item at src/benchmark/relative_speed.rs:7 |  |  | 0.482 |
| walker |  | 5784 | 19 | pub-item doc lede at src/benchmark/relative_speed.rs:112 |  |  | 0.482 |
| walker |  | 5833 | 49 | pub item body at src/benchmark/relative_speed.rs:20 body 21 |  |  | 0.482 |
| walker |  | 5889 | 56 | pub item body at src/benchmark/relative_speed.rs:86 body 91 |  |  | 0.482 |
| walker |  | 5946 | 57 | pub item body at src/benchmark/relative_speed.rs:98 body 102 |  |  | 0.482 |
| walker |  | 5990 | 44 | pub-item doc lede at src/outlier_detection.rs:43 |  |  | 0.482 |
| walker |  | 6196 | 206 | mod/use plumbing in src/export/mod.rs |  |  | 0.482 |
| ns | 6202 |  | 537 | CsvExporter body | 3.10 | 2.10 | 0.459 |
| walker |  | 6277 | 81 | impl method sigs in src/benchmark/scheduler.rs |  |  | 0.467 |
| walker |  | 6650 | 373 | pub item at src/error.rs:37 |  |  | 0.468 |
| ns | 6887 |  | 685 | ShellExecutor::calibrate — shell spawning time measurement | 3.11 | 2.3 | 0.441 |
| walker |  | 6907 | 257 | pub item at src/benchmark/executor.rs:35 |  |  | 0.449 |
| walker |  | 7324 | 417 | impl method sigs in src/benchmark/mod.rs |  |  | 0.464 |
| ns | 7425 |  | 538 | ShellExecutor::run_command_and_measure body | 3.12 | 3.11 | 0.447 |
| walker |  | 7512 | 188 | impl method sigs in src/options.rs |  |  | 0.450 |
| walker |  | 7578 | 66 | README.md section #8 |  |  | 0.450 |
| walker |  | 7655 | 77 | README.md section #6 |  |  | 0.450 |
| walker |  | 7967 | 312 | mod/use plumbing in src/benchmark/mod.rs |  |  | 0.450 |
| ns | 7997 |  | 572 | MockExecutor body (debug-mode) | 3.13 | 2.3 | 0.431 |
| walker |  | 8231 | 264 | [dependencies] in Cargo.toml |  |  | 0.431 |
| walker |  | 8294 | 63 | pub-item doc lede at src/outlier_detection.rs:13 |  |  | 0.431 |
| ns | 8406 |  | 409 | outlier_detection module + OUTLIER_THRESHOLD const | 3.14 | 2.12 | 0.423 |
| walker |  | 8429 | 135 | impl method sigs in src/export/asciidoc.rs |  |  | 0.427 |
| walker |  | 8618 | 189 | pub item body at src/outlier_detection.rs:21 body 22 |  |  | 0.439 |
| walker |  | 8674 | 56 | listing of 'scripts' |  |  | 0.453 |
| ns | 8958 |  | 552 | format_duration auto-unit selection + Unit::short_name/format | 3.15 |  | 0.444 |
| ns | 9144 |  | 186 | Warnings enum + OutlierWarningOptions struct | 3.16 |  | 0.446 |
| walker |  | 9177 | 503 | pub item at src/options.rs:198 |  |  | 0.489 |
| walker |  | 9192 | 15 | pub-item doc lede at src/options.rs:198 |  |  | 0.491 |
| walker |  | 9469 | 277 | impl method sigs in src/command.rs |  |  | 0.506 |
| ns | 9531 |  | 387 | OptionsError enum + variants | 4.1 |  | 0.519 |
| walker |  | 9591 | 122 | pub item body at src/output/format.rs:18 body 19 |  |  | 0.525 |
| walker |  | 9642 | 51 | pub-item doc lede at src/output/format.rs:5 |  |  | 0.525 |
| walker |  | 9748 | 106 | pub-item doc lede at src/outlier_detection.rs:21 |  |  | 0.525 |
| ns | 9788 |  | 257 | ParameterScanError enum + variants | 4.2 |  | 0.534 |
| ns | 9922 |  | 134 | common.rs test helpers | 4.3 |  | 0.530 |
| walker |  | 9928 | 180 | pub item body at src/util/exit_code.rs:4 body 5 |  |  | 0.530 |
