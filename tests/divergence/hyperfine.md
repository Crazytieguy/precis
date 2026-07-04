Score(3000)=0.590 I=0.847 C=0.410 ns_rows≤3K=23/42 (reached=9 partial=1 missing=13)

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
| walker |  | 513 | 149 | README.md section #1 |  |  | 0.430 |
| walker |  | 538 | 25 | listing of 'doc' |  |  | 0.430 |
| ns | 572 |  | 178 | main.rs run() — first half (CLI parsing + builders) | 1.8 |  | 0.385 |
| walker |  | 582 | 44 | listing of 'src' |  |  | 0.527 |
| walker |  | 592 | 10 | entry item at src/main.rs:53 |  |  | 0.527 |
| walker |  | 605 | 13 | listing of 'src/parameter' |  |  | 0.532 |
| walker |  | 618 | 13 | entry item at src/main.rs:29 |  |  | 0.533 |
| walker |  | 625 | 7 | entry item body at src/main.rs:29 body 50 |  |  | 0.533 |
| walker |  | 635 | 10 | entry item body at src/main.rs:29 body 31 |  |  | 0.534 |
| walker |  | 645 | 10 | entry item body at src/main.rs:29 body 48 |  |  | 0.534 |
| walker |  | 657 | 12 | entry item body at src/main.rs:29 body 30 |  |  | 0.537 |
| walker |  | 669 | 12 | entry item body at src/main.rs:29 body 46 |  |  | 0.537 |
| walker |  | 681 | 12 | entry item body at src/main.rs:29 body 47 |  |  | 0.538 |
| walker |  | 695 | 14 | entry item body at src/main.rs:29 body 43 |  |  | 0.543 |
| walker |  | 711 | 16 | entry item body at src/main.rs:29 body 32 |  |  | 0.550 |
| walker |  | 728 | 17 | entry item body at src/main.rs:29 body 34 |  |  | 0.559 |
| ns | 741 |  | 169 | main.rs run() — second half (Scheduler) + main() wrapper | 1.9 | 1.8 | 0.515 |
| walker |  | 745 | 17 | entry item body at src/main.rs:29 body 36 |  |  | 0.525 |
| walker |  | 763 | 18 | entry item body at src/main.rs:29 body 35 |  |  | 0.537 |
| walker |  | 785 | 22 | entry item body at src/main.rs:29 body 45 |  |  | 0.549 |
| walker |  | 802 | 17 | listing of 'src/output' |  |  | 0.567 |
| walker |  | 822 | 20 | listing of 'src/timer' |  |  | 0.599 |
| walker |  | 855 | 33 | pub-item names surface in src/parameter/mod.rs |  |  | 0.599 |
| walker |  | 889 | 34 | pub item at src/parameter/mod.rs:8 |  |  | 0.599 |
| ns | 895 |  | 154 | README features list | 1.10 |  | 0.624 |
| walker |  | 923 | 34 | pub-item names surface in src/timer/mod.rs |  |  | 0.624 |
| walker |  | 923 | 0 | pub item at src/timer/mod.rs:83 |  |  | 0.624 |
| walker |  | 950 | 27 | listing of 'src/benchmark' |  |  | 0.627 |
| ns | 950 |  | 55 | Scheduler struct header + public method names | 2.1 |  | 0.627 |
| ns | 979 |  | 29 | Benchmark struct header + Benchmark::run signature | 2.2 |  | 0.618 |
| walker |  | 987 | 37 | pub-item names surface in src/benchmark/mod.rs |  |  | 0.621 |
| walker |  | 987 | 0 | pub item at src/benchmark/mod.rs:32 |  |  | 0.621 |
| walker |  | 1035 | 48 | pub item at src/benchmark/mod.rs:34 |  |  | 0.621 |
| ns | 1058 |  | 79 | Executor trait header + impl struct names | 2.3 |  | 0.599 |
| walker |  | 1063 | 28 | listing of 'src/util' |  |  | 0.656 |
| walker |  | 1114 | 51 | entry item body at src/main.rs:29 body 37 |  |  | 0.694 |
| walker |  | 1149 | 35 | listing of 'src/export' |  |  | 0.761 |
| walker |  | 1182 | 33 | pub-item names surface in src/export/mod.rs |  |  | 0.761 |
| walker |  | 1201 | 19 | pub item at src/export/mod.rs:56 |  |  | 0.761 |
| walker |  | 1239 | 38 | pub item at src/export/mod.rs:67 |  |  | 0.762 |
| walker |  | 1252 | 13 | pub item at src/util/randomized_environment_offset.rs:6 |  |  | 0.762 |
| walker |  | 1265 | 13 | pub-item doc lede at src/export/mod.rs:67 |  |  | 0.762 |
| walker |  | 1336 | 71 | entry item body at src/main.rs:53 body 54 |  |  | 0.808 |
| walker |  | 1353 | 17 | pub item at src/export/csv.rs:13 |  |  | 0.808 |
| walker |  | 1370 | 17 | pub item at src/export/json.rs:17 |  |  | 0.808 |
| walker |  | 1387 | 17 | pub item at src/export/markdown.rs:6 |  |  | 0.808 |
| ns | 1426 |  | 368 | Options struct field list — first half | 2.4 |  | 0.713 |
| walker |  | 1472 | 85 | pub item at src/timer/mod.rs:42 |  |  | 0.713 |
| walker |  | 1498 | 26 | pub-item names surface in src/error.rs |  |  | 0.713 |
| walker |  | 1516 | 18 | pub item at src/export/orgmode.rs:5 |  |  | 0.713 |
| walker |  | 1541 | 25 | mod/use plumbing in src/output/mod.rs |  |  | 0.713 |
| walker |  | 1582 | 41 | impl method sigs in src/parameter/mod.rs |  |  | 0.713 |
| walker |  | 1597 | 15 | pub-item doc lede at src/timer/mod.rs:42 |  |  | 0.713 |
| walker |  | 1617 | 20 | pub item at src/export/asciidoc.rs:5 |  |  | 0.713 |
| ns | 1666 |  | 240 | Options struct field list — second half | 2.5 | 2.4 | 0.666 |
| walker |  | 1743 | 126 | pub item at src/export/mod.rs:28 |  |  | 0.669 |
| walker |  | 1760 | 17 | pub-item doc lede at src/export/mod.rs:28 |  |  | 0.669 |
| walker |  | 1775 | 15 | pub-item doc lede at src/timer/mod.rs:83 |  |  | 0.669 |
| walker |  | 1794 | 19 | pub item at src/parameter/tokenize.rs:1 |  |  | 0.669 |
| ns | 1798 |  | 132 | Options sibling enum/struct headers | 2.6 |  | 0.644 |
| ns | 1825 |  | 27 | Options::from_cli_arguments + validate_against_command_list signatures | 2.7 |  | 0.640 |
| walker |  | 1826 | 32 | pub-item names surface in src/command.rs |  |  | 0.640 |
| walker |  | 1826 | 0 | pub item at src/command.rs:134 |  |  | 0.640 |
| walker |  | 1839 | 13 | pub-item doc lede at src/benchmark/mod.rs:32 |  |  | 0.640 |
| walker |  | 1863 | 24 | pub item at src/timer/wall_clock_timer.rs:5 |  |  | 0.640 |
| walker |  | 1888 | 25 | pub item at src/timer/windows_timer.rs:49 |  |  | 0.640 |
| walker |  | 1910 | 22 | pub-item names surface in src/export/markup.rs |  |  | 0.640 |
| walker |  | 1927 | 17 | pub item at src/export/markup.rs:10 |  |  | 0.640 |
| walker |  | 1950 | 23 | pub-item names surface in src/parameter/range_step.rs |  |  | 0.640 |
| walker |  | 1973 | 23 | pub-item names surface in src/timer/unix_timer.rs |  |  | 0.640 |
| ns | 1984 |  | 159 | Command + Commands type headers + public method names | 2.8 |  | 0.617 |
| walker |  | 1988 | 15 | pub item at src/timer/unix_timer.rs:18 |  |  | 0.617 |
| walker |  | 2012 | 24 | pub-item names surface in src/output/warnings.rs |  |  | 0.617 |
| walker |  | 2040 | 28 | pub item at src/output/warnings.rs:7 |  |  | 0.617 |
| walker |  | 2122 | 82 | README.md section #0 |  |  | 0.629 |
| walker |  | 2166 | 44 | mod/use plumbing in src/util/mod.rs |  |  | 0.629 |
| walker |  | 2212 | 46 | mod/use plumbing in src/parameter/mod.rs |  |  | 0.629 |
| walker |  | 2243 | 31 | pub item at src/output/progress_bar.rs:13 |  |  | 0.629 |
| ns | 2254 |  | 270 | BenchmarkResult fields (the in-memory result row) | 2.9 |  | 0.597 |
| walker |  | 2281 | 38 | pub item at src/parameter/range_step.rs:34 |  |  | 0.597 |
| walker |  | 2331 | 50 | pub item at src/util/number.rs:10 |  |  | 0.597 |
| walker |  | 2383 | 52 | pub item at src/output/warnings.rs:13 |  |  | 0.597 |
| walker |  | 2563 | 180 | mod/use plumbing in src/main.rs |  |  | 0.646 |
| ns | 2570 |  | 316 | ExportType + Exporter trait + ExportManager headers | 2.10 |  | 0.637 |
| walker |  | 2631 | 68 | pub item at src/cli.rs:8 |  |  | 0.637 |
| walker |  | 2651 | 20 | pub item body at src/cli.rs:8 body 13 |  |  | 0.637 |
| walker |  | 2686 | 35 | pub-item names surface in src/util/units.rs |  |  | 0.637 |
| walker |  | 2686 | 0 | pub item at src/util/units.rs:6 |  |  | 0.637 |
| ns | 2696 |  | 126 | Benchmark module-level helpers + MIN_EXECUTION_TIME | 2.11 |  | 0.621 |
| walker |  | 2729 | 43 | pub item at src/util/units.rs:10 |  |  | 0.621 |
| walker |  | 2738 | 9 | pub-item doc lede at src/util/units.rs:10 |  |  | 0.621 |
| walker |  | 2753 | 15 | pub-item doc lede at src/command.rs:134 |  |  | 0.621 |
| walker |  | 2791 | 38 | pub-item names surface in src/util/min_max.rs |  |  | 0.621 |
| walker |  | 2791 | 0 | pub item at src/util/min_max.rs:2 |  |  | 0.621 |
| walker |  | 2791 | 0 | pub item at src/util/min_max.rs:10 |  |  | 0.621 |
| walker |  | 2810 | 19 | pub item body at src/util/randomized_environment_offset.rs:6 body 7 |  |  | 0.621 |
| ns | 2874 |  | 178 | relative_speed + outlier_detection signatures | 2.12 |  | 0.603 |
| walker |  | 2880 | 70 | pub-item names surface in src/outlier_detection.rs |  |  | 0.604 |
| walker |  | 2880 | 0 | pub item at src/outlier_detection.rs:13 |  |  | 0.604 |
| walker |  | 2880 | 0 | pub item at src/outlier_detection.rs:21 |  |  | 0.604 |
| walker |  | 2880 | 0 | pub item at src/outlier_detection.rs:43 |  |  | 0.604 |
| walker |  | 2924 | 44 | pub-item names surface in src/util/exit_code.rs |  |  | 0.604 |
| walker |  | 2924 | 0 | pub item at src/util/exit_code.rs:4 |  |  | 0.604 |
| walker |  | 2924 | 0 | pub item at src/util/exit_code.rs:20 |  |  | 0.604 |
| walker |  | 2930 | 6 | pub item body at src/util/exit_code.rs:20 body 21 |  |  | 0.604 |
| walker |  | 2942 | 12 | pub-item doc lede at src/output/warnings.rs:13 |  |  | 0.604 |
| ns | 2951 |  | 77 | tests/ + scripts/ fs listings | 2.13 |  | 0.590 |
| walker |  | 3066 | 124 | impl method sigs in src/export/mod.rs |  |  | 0.603 |
| walker |  | 3177 | 111 | pub item at src/command.rs:22 |  |  | 0.604 |
| walker |  | 3190 | 13 | pub-item doc lede at src/command.rs:22 |  |  | 0.604 |
| walker |  | 3255 | 65 | pub item at src/benchmark/scheduler.rs:13 |  |  | 0.605 |
| ns | 3326 |  | 375 | CLI flag names — every Arg::new() line | 3.1 |  | 0.573 |
| walker |  | 3330 | 75 | man-page NAME + DESCRIPTION in doc/hyperfine.1 |  |  | 0.573 |
| ns | 3375 |  | 49 | RunBounds default — '10 runs by default' | 3.2 |  | 0.568 |
| walker |  | 3427 | 97 | pub item at src/parameter/range_step.rs:7 |  |  | 0.568 |
| walker |  | 3499 | 72 | pub item at src/timer/unix_timer.rs:10 |  |  | 0.568 |
| walker |  | 3536 | 37 | README.md section #4 |  |  | 0.568 |
| walker |  | 3596 | 60 | pub-item names surface in src/benchmark/executor.rs |  |  | 0.574 |
| walker |  | 3610 | 14 | pub item at src/benchmark/executor.rs:122 |  |  | 0.574 |
| walker |  | 3631 | 21 | pub item at src/benchmark/executor.rs:303 |  |  | 0.577 |
| walker |  | 3662 | 31 | pub item at src/benchmark/executor.rs:19 |  |  | 0.581 |
| ns | 3685 |  | 310 | Scheduler::run_benchmarks body | 3.3 | 2.1 | 0.560 |
| walker |  | 3702 | 40 | pub item at src/benchmark/executor.rs:169 |  |  | 0.560 |
| walker |  | 3715 | 13 | pub-item doc lede at src/output/progress_bar.rs:13 |  |  | 0.560 |
| walker |  | 3727 | 12 | pub-item doc lede at src/util/units.rs:6 |  |  | 0.560 |
| walker |  | 3803 | 76 | pub-item names surface in src/output/format.rs |  |  | 0.560 |
| walker |  | 3803 | 0 | pub item at src/output/format.rs:5 |  |  | 0.560 |
| walker |  | 3803 | 0 | pub item at src/output/format.rs:11 |  |  | 0.560 |
| walker |  | 3803 | 0 | pub item at src/output/format.rs:18 |  |  | 0.560 |
| walker |  | 3828 | 25 | pub item body at src/output/format.rs:5 body 6 |  |  | 0.560 |
| ns | 3893 |  | 208 | Benchmark::run — count-of-runs formula | 3.4 | 2.2 | 0.544 |
| walker |  | 3966 | 138 | pub-item names surface in src/options.rs |  |  | 0.565 |
| walker |  | 4001 | 35 | pub item at src/options.rs:102 |  |  | 0.565 |
| walker |  | 4042 | 41 | pub item at src/options.rs:185 |  |  | 0.565 |
| walker |  | 4106 | 64 | pub item at src/options.rs:24 |  |  | 0.565 |
| walker |  | 4161 | 55 | pub item at src/options.rs:108 |  |  | 0.565 |
| walker |  | 4232 | 71 | pub item at src/options.rs:123 |  |  | 0.565 |
| walker |  | 4246 | 14 | pub-item doc lede at src/options.rs:24 |  |  | 0.565 |
| walker |  | 4259 | 13 | pub-item doc lede at src/options.rs:108 |  |  | 0.565 |
| ns | 4283 |  | 390 | Benchmark::run — warning push logic | 3.5 | 2.2 | 0.538 |
| walker |  | 4353 | 94 | pub item at src/options.rs:71 |  |  | 0.538 |
| walker |  | 4367 | 14 | pub-item doc lede at src/options.rs:71 |  |  | 0.538 |
| walker |  | 4495 | 128 | pub item at src/options.rs:149 |  |  | 0.538 |
| ns | 4505 |  | 222 | Benchmark::run — BenchmarkResult construction | 3.6 | 2.9 | 0.524 |
| walker |  | 4510 | 15 | pub-item doc lede at src/options.rs:149 |  |  | 0.524 |
| walker |  | 4659 | 149 | pub item at src/options.rs:84 |  |  | 0.524 |
| walker |  | 4669 | 10 | pub-item doc lede at src/options.rs:84 |  |  | 0.524 |
| walker |  | 4685 | 16 | pub-item doc lede at src/util/min_max.rs:2 |  |  | 0.524 |
| walker |  | 4701 | 16 | pub-item doc lede at src/util/min_max.rs:10 |  |  | 0.524 |
| walker |  | 4722 | 21 | listing of 'tests' |  |  | 0.526 |
| walker |  | 4764 | 42 | pub item body at src/util/min_max.rs:2 body 3 |  |  | 0.526 |
| walker |  | 4806 | 42 | pub item body at src/util/min_max.rs:10 body 11 |  |  | 0.526 |
| ns | 5047 |  | 542 | Per-format markup exporters — Markdown / Orgmode | 3.7 | 2.10 | 0.494 |
| walker |  | 5051 | 245 | pub item at src/error.rs:7 |  |  | 0.495 |
| walker |  | 5069 | 18 | pub-item doc lede at src/output/format.rs:11 |  |  | 0.495 |
| walker |  | 5087 | 18 | pub-item doc lede at src/output/format.rs:18 |  |  | 0.495 |
| walker |  | 5132 | 45 | pub item body at src/output/format.rs:11 body 12 |  |  | 0.495 |
| walker |  | 5217 | 85 | pub item body at src/outlier_detection.rs:43 body 44 |  |  | 0.495 |
| walker |  | 5383 | 166 | mod/use plumbing in src/timer/mod.rs |  |  | 0.495 |
| ns | 5447 |  | 400 | AsciidocExporter body | 3.8 | 2.10 | 0.476 |
| walker |  | 5485 | 102 | pub-item names surface in src/benchmark/relative_speed.rs |  |  | 0.483 |
| walker |  | 5485 | 0 | pub item at src/benchmark/relative_speed.rs:16 |  |  | 0.483 |
| walker |  | 5485 | 0 | pub item at src/benchmark/relative_speed.rs:20 |  |  | 0.483 |
| walker |  | 5524 | 39 | pub item at src/benchmark/relative_speed.rs:112 |  |  | 0.483 |
| walker |  | 5564 | 40 | pub item at src/benchmark/relative_speed.rs:98 |  |  | 0.483 |
| walker |  | 5582 | 18 | pub item body at src/benchmark/relative_speed.rs:16 body 17 |  |  | 0.483 |
| walker |  | 5636 | 54 | pub item at src/benchmark/relative_speed.rs:86 |  |  | 0.483 |
| ns | 5665 |  | 218 | JsonExporter body | 3.9 | 2.10 | 0.472 |
| walker |  | 5668 | 32 | pub item body at src/benchmark/relative_speed.rs:112 body 116 |  |  | 0.472 |
| walker |  | 5751 | 83 | pub item at src/benchmark/relative_speed.rs:7 |  |  | 0.489 |
| walker |  | 5770 | 19 | pub-item doc lede at src/benchmark/relative_speed.rs:112 |  |  | 0.489 |
| walker |  | 5819 | 49 | pub item body at src/benchmark/relative_speed.rs:20 body 21 |  |  | 0.489 |
| walker |  | 5957 | 138 | pub item at src/benchmark/timing_result.rs:5 |  |  | 0.489 |
| walker |  | 5969 | 12 | pub-item doc lede at src/benchmark/timing_result.rs:5 |  |  | 0.489 |
| walker |  | 6021 | 52 | README.md section #7 |  |  | 0.489 |
| walker |  | 6065 | 44 | pub-item doc lede at src/outlier_detection.rs:43 |  |  | 0.489 |
| walker |  | 6126 | 61 | pub item body at src/benchmark/relative_speed.rs:86 body 91 |  |  | 0.489 |
| ns | 6202 |  | 537 | CsvExporter body | 3.10 | 2.10 | 0.466 |
| walker |  | 6207 | 81 | impl method sigs in src/benchmark/scheduler.rs |  |  | 0.474 |
| walker |  | 6580 | 373 | pub item at src/error.rs:37 |  |  | 0.475 |
| walker |  | 6806 | 226 | mod/use plumbing in src/export/mod.rs |  |  | 0.475 |
| walker |  | 6873 | 67 | pub item body at src/benchmark/relative_speed.rs:98 body 102 |  |  | 0.475 |
| ns | 6887 |  | 685 | ShellExecutor::calibrate — shell spawning time measurement | 3.11 | 2.3 | 0.448 |
| walker |  | 7140 | 267 | pub item at src/benchmark/executor.rs:35 |  |  | 0.455 |
| ns | 7425 |  | 538 | ShellExecutor::run_command_and_measure body | 3.12 | 3.11 | 0.439 |
| walker |  | 7557 | 417 | impl method sigs in src/benchmark/mod.rs |  |  | 0.453 |
| walker |  | 7745 | 188 | impl method sigs in src/options.rs |  |  | 0.456 |
| ns | 7997 |  | 572 | MockExecutor body (debug-mode) | 3.13 | 2.3 | 0.437 |
| walker |  | 8009 | 264 | [dependencies] in Cargo.toml |  |  | 0.437 |
| walker |  | 8091 | 82 | README.md section #6 |  |  | 0.437 |
| walker |  | 8167 | 76 | README.md section #8 |  |  | 0.437 |
| ns | 8406 |  | 409 | outlier_detection module + OUTLIER_THRESHOLD const | 3.14 | 2.12 | 0.427 |
| walker |  | 8499 | 332 | mod/use plumbing in src/benchmark/mod.rs |  |  | 0.427 |
| walker |  | 8562 | 63 | pub-item doc lede at src/outlier_detection.rs:13 |  |  | 0.429 |
| walker |  | 8697 | 135 | impl method sigs in src/export/asciidoc.rs |  |  | 0.433 |
| walker |  | 8753 | 56 | listing of 'scripts' |  |  | 0.447 |
| ns | 8958 |  | 552 | format_duration auto-unit selection + Unit::short_name/format | 3.15 |  | 0.439 |
| walker |  | 9030 | 277 | impl method sigs in src/command.rs |  |  | 0.455 |
| ns | 9144 |  | 186 | Warnings enum + OutlierWarningOptions struct | 3.16 |  | 0.457 |
| walker |  | 9244 | 214 | pub item body at src/outlier_detection.rs:21 body 22 |  |  | 0.471 |
| walker |  | 9366 | 122 | pub item body at src/output/format.rs:18 body 19 |  |  | 0.478 |
| walker |  | 9417 | 51 | pub-item doc lede at src/output/format.rs:5 |  |  | 0.478 |
| ns | 9531 |  | 387 | OptionsError enum + variants | 4.1 |  | 0.492 |
| ns | 9788 |  | 257 | ParameterScanError enum + variants | 4.2 |  | 0.502 |
| ns | 9922 |  | 134 | common.rs test helpers | 4.3 |  | 0.498 |
| walker |  | 10000 | 583 | pub item at src/options.rs:198 |  |  | 0.553 |
