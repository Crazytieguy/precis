Score(3000)=0.561 I=0.832 C=0.378 ns_rows≤3K=23/42 (reached=8 partial=1 missing=14)

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
| walker |  | 436 | 72 | README.md section #0 |  |  | 0.450 |
| walker |  | 468 | 32 | README.md section #4 |  |  | 0.450 |
| walker |  | 493 | 25 | listing of 'doc' |  |  | 0.450 |
| walker |  | 568 | 75 | man-page NAME + DESCRIPTION in doc/hyperfine.1 |  |  | 0.450 |
| ns | 572 |  | 178 | main.rs run() — first half (CLI parsing + builders) | 1.8 |  | 0.403 |
| walker |  | 612 | 44 | listing of 'src' |  |  | 0.543 |
| walker |  | 622 | 10 | entry item at src/main.rs:53 |  |  | 0.543 |
| walker |  | 635 | 13 | listing of 'src/parameter' |  |  | 0.548 |
| walker |  | 648 | 13 | entry item at src/main.rs:29 |  |  | 0.548 |
| walker |  | 655 | 7 | entry item body at src/main.rs:29 body 50 |  |  | 0.549 |
| walker |  | 665 | 10 | entry item body at src/main.rs:29 body 31 |  |  | 0.550 |
| walker |  | 675 | 10 | entry item body at src/main.rs:29 body 48 |  |  | 0.550 |
| walker |  | 687 | 12 | entry item body at src/main.rs:29 body 30 |  |  | 0.552 |
| walker |  | 699 | 12 | entry item body at src/main.rs:29 body 46 |  |  | 0.553 |
| walker |  | 711 | 12 | entry item body at src/main.rs:29 body 47 |  |  | 0.553 |
| walker |  | 725 | 14 | entry item body at src/main.rs:29 body 43 |  |  | 0.558 |
| walker |  | 741 | 16 | entry item body at src/main.rs:29 body 32 |  |  | 0.520 |
| ns | 741 |  | 169 | main.rs run() — second half (Scheduler) + main() wrapper | 1.9 | 1.8 | 0.520 |
| walker |  | 758 | 17 | entry item body at src/main.rs:29 body 34 |  |  | 0.528 |
| walker |  | 775 | 17 | entry item body at src/main.rs:29 body 36 |  |  | 0.538 |
| walker |  | 793 | 18 | entry item body at src/main.rs:29 body 35 |  |  | 0.550 |
| walker |  | 815 | 22 | entry item body at src/main.rs:29 body 45 |  |  | 0.562 |
| walker |  | 832 | 17 | listing of 'src/output' |  |  | 0.580 |
| walker |  | 852 | 20 | listing of 'src/timer' |  |  | 0.611 |
| walker |  | 885 | 33 | pub-item names surface in src/parameter/mod.rs |  |  | 0.611 |
| ns | 895 |  | 154 | README features list | 1.10 |  | 0.574 |
| walker |  | 904 | 19 | pub item at src/parameter/mod.rs:8 |  |  | 0.574 |
| walker |  | 938 | 34 | pub-item names surface in src/timer/mod.rs |  |  | 0.574 |
| walker |  | 938 | 0 | pub item at src/timer/mod.rs:83 |  |  | 0.574 |
| ns | 950 |  | 55 | Scheduler struct header + public method names | 2.1 |  | 0.560 |
| walker |  | 965 | 27 | listing of 'src/benchmark' |  |  | 0.578 |
| ns | 979 |  | 29 | Benchmark struct header + Benchmark::run signature | 2.2 |  | 0.570 |
| walker |  | 999 | 34 | pub-item names surface in src/benchmark/mod.rs |  |  | 0.573 |
| walker |  | 999 | 0 | pub item at src/benchmark/mod.rs:32 |  |  | 0.573 |
| walker |  | 1047 | 48 | pub item at src/benchmark/mod.rs:34 |  |  | 0.573 |
| ns | 1058 |  | 79 | Executor trait header + impl struct names | 2.3 |  | 0.553 |
| walker |  | 1075 | 28 | listing of 'src/util' |  |  | 0.613 |
| walker |  | 1126 | 51 | entry item body at src/main.rs:29 body 37 |  |  | 0.654 |
| walker |  | 1161 | 35 | listing of 'src/export' |  |  | 0.722 |
| walker |  | 1194 | 33 | pub-item names surface in src/export/mod.rs |  |  | 0.722 |
| walker |  | 1213 | 19 | pub item at src/export/mod.rs:56 |  |  | 0.722 |
| walker |  | 1251 | 38 | pub item at src/export/mod.rs:67 |  |  | 0.723 |
| walker |  | 1264 | 13 | pub item at src/util/randomized_environment_offset.rs:6 |  |  | 0.723 |
| walker |  | 1337 | 73 | pub item at src/timer/mod.rs:42 |  |  | 0.723 |
| ns | 1426 |  | 368 | Options struct field list — first half | 2.4 |  | 0.638 |
| walker |  | 1435 | 98 | pub item at src/export/mod.rs:28 |  |  | 0.641 |
| walker |  | 1448 | 13 | pub-item doc lede at src/export/mod.rs:67 |  |  | 0.641 |
| walker |  | 1519 | 71 | entry item body at src/main.rs:53 body 54 |  |  | 0.683 |
| walker |  | 1545 | 26 | pub-item names surface in src/error.rs |  |  | 0.683 |
| walker |  | 1562 | 17 | pub-item doc lede at src/export/mod.rs:28 |  |  | 0.683 |
| walker |  | 1587 | 25 | mod/use plumbing in src/output/mod.rs |  |  | 0.683 |
| walker |  | 1628 | 41 | impl method sigs in src/parameter/mod.rs |  |  | 0.683 |
| walker |  | 1643 | 15 | pub-item doc lede at src/timer/mod.rs:42 |  |  | 0.683 |
| walker |  | 1658 | 15 | pub-item doc lede at src/timer/mod.rs:83 |  |  | 0.683 |
| ns | 1666 |  | 240 | Options struct field list — second half | 2.5 | 2.4 | 0.638 |
| walker |  | 1677 | 19 | pub item at src/parameter/tokenize.rs:1 |  |  | 0.638 |
| walker |  | 1709 | 32 | pub-item names surface in src/command.rs |  |  | 0.638 |
| walker |  | 1709 | 0 | pub item at src/command.rs:134 |  |  | 0.638 |
| walker |  | 1722 | 13 | pub-item doc lede at src/benchmark/mod.rs:32 |  |  | 0.638 |
| walker |  | 1746 | 24 | pub item at src/timer/wall_clock_timer.rs:5 |  |  | 0.638 |
| walker |  | 1777 | 31 | pub item at src/util/number.rs:10 |  |  | 0.638 |
| ns | 1798 |  | 132 | Options sibling enum/struct headers | 2.6 |  | 0.613 |
| walker |  | 1802 | 25 | pub item at src/timer/windows_timer.rs:49 |  |  | 0.613 |
| walker |  | 1824 | 22 | pub-item names surface in src/export/markup.rs |  |  | 0.613 |
| ns | 1825 |  | 27 | Options::from_cli_arguments + validate_against_command_list signatures | 2.7 |  | 0.610 |
| walker |  | 1841 | 17 | pub item at src/export/markup.rs:10 |  |  | 0.610 |
| walker |  | 1864 | 23 | pub-item names surface in src/parameter/range_step.rs |  |  | 0.610 |
| walker |  | 1887 | 23 | pub-item names surface in src/timer/unix_timer.rs |  |  | 0.610 |
| walker |  | 1902 | 15 | pub item at src/timer/unix_timer.rs:18 |  |  | 0.610 |
| walker |  | 1926 | 24 | pub-item names surface in src/output/warnings.rs |  |  | 0.610 |
| walker |  | 1954 | 28 | pub item at src/output/warnings.rs:7 |  |  | 0.610 |
| walker |  | 1984 | 30 | pub item at src/parameter/range_step.rs:34 |  |  | 0.588 |
| ns | 1984 |  | 159 | Command + Commands type headers + public method names | 2.8 |  | 0.588 |
| walker |  | 2025 | 41 | mod/use plumbing in src/parameter/mod.rs |  |  | 0.588 |
| walker |  | 2069 | 44 | mod/use plumbing in src/util/mod.rs |  |  | 0.588 |
| walker |  | 2100 | 31 | pub item at src/output/progress_bar.rs:13 |  |  | 0.588 |
| ns | 2254 |  | 270 | BenchmarkResult fields (the in-memory result row) | 2.9 |  | 0.557 |
| walker |  | 2265 | 165 | mod/use plumbing in src/main.rs |  |  | 0.607 |
| walker |  | 2317 | 52 | pub item at src/output/warnings.rs:13 |  |  | 0.607 |
| walker |  | 2385 | 68 | pub item at src/cli.rs:8 |  |  | 0.607 |
| walker |  | 2405 | 20 | pub item body at src/cli.rs:8 body 13 |  |  | 0.607 |
| walker |  | 2440 | 35 | pub-item names surface in src/util/units.rs |  |  | 0.607 |
| walker |  | 2440 | 0 | pub item at src/util/units.rs:6 |  |  | 0.607 |
| walker |  | 2466 | 26 | pub item at src/util/units.rs:10 |  |  | 0.607 |
| walker |  | 2475 | 9 | pub-item doc lede at src/util/units.rs:10 |  |  | 0.607 |
| walker |  | 2490 | 15 | pub-item doc lede at src/command.rs:134 |  |  | 0.607 |
| walker |  | 2528 | 38 | pub-item names surface in src/util/min_max.rs |  |  | 0.607 |
| walker |  | 2528 | 0 | pub item at src/util/min_max.rs:2 |  |  | 0.607 |
| walker |  | 2528 | 0 | pub item at src/util/min_max.rs:10 |  |  | 0.607 |
| ns | 2570 |  | 316 | ExportType + Exporter trait + ExportManager headers | 2.10 |  | 0.593 |
| walker |  | 2614 | 86 | pub item at src/command.rs:22 |  |  | 0.593 |
| walker |  | 2627 | 13 | pub-item doc lede at src/command.rs:22 |  |  | 0.593 |
| walker |  | 2646 | 19 | pub item body at src/util/randomized_environment_offset.rs:6 body 7 |  |  | 0.593 |
| ns | 2696 |  | 126 | Benchmark module-level helpers + MIN_EXECUTION_TIME | 2.11 |  | 0.579 |
| walker |  | 2716 | 70 | pub-item names surface in src/outlier_detection.rs |  |  | 0.579 |
| walker |  | 2716 | 0 | pub item at src/outlier_detection.rs:13 |  |  | 0.579 |
| walker |  | 2716 | 0 | pub item at src/outlier_detection.rs:21 |  |  | 0.579 |
| walker |  | 2716 | 0 | pub item at src/outlier_detection.rs:43 |  |  | 0.579 |
| walker |  | 2760 | 44 | pub-item names surface in src/util/exit_code.rs |  |  | 0.579 |
| walker |  | 2760 | 0 | pub item at src/util/exit_code.rs:4 |  |  | 0.579 |
| walker |  | 2760 | 0 | pub item at src/util/exit_code.rs:20 |  |  | 0.579 |
| walker |  | 2766 | 6 | pub item body at src/util/exit_code.rs:20 body 21 |  |  | 0.579 |
| walker |  | 2821 | 55 | pub item at src/timer/unix_timer.rs:10 |  |  | 0.579 |
| walker |  | 2833 | 12 | pub-item doc lede at src/output/warnings.rs:13 |  |  | 0.579 |
| ns | 2874 |  | 178 | relative_speed + outlier_detection signatures | 2.12 |  | 0.563 |
| ns | 2951 |  | 77 | tests/ + scripts/ fs listings | 2.13 |  | 0.549 |
| walker |  | 2957 | 124 | impl method sigs in src/export/mod.rs |  |  | 0.561 |
| walker |  | 3022 | 65 | pub item at src/benchmark/scheduler.rs:13 |  |  | 0.562 |
| walker |  | 3119 | 97 | pub item at src/parameter/range_step.rs:7 |  |  | 0.562 |
| walker |  | 3179 | 60 | pub-item names surface in src/benchmark/executor.rs |  |  | 0.569 |
| walker |  | 3192 | 13 | pub item at src/benchmark/executor.rs:303 |  |  | 0.569 |
| walker |  | 3206 | 14 | pub item at src/benchmark/executor.rs:122 |  |  | 0.569 |
| walker |  | 3237 | 31 | pub item at src/benchmark/executor.rs:19 |  |  | 0.574 |
| walker |  | 3277 | 40 | pub item at src/benchmark/executor.rs:169 |  |  | 0.574 |
| walker |  | 3290 | 13 | pub-item doc lede at src/output/progress_bar.rs:13 |  |  | 0.574 |
| walker |  | 3302 | 12 | pub-item doc lede at src/util/units.rs:6 |  |  | 0.574 |
| ns | 3326 |  | 375 | CLI flag names — every Arg::new() line | 3.1 |  | 0.544 |
| ns | 3375 |  | 49 | RunBounds default — '10 runs by default' | 3.2 |  | 0.539 |
| walker |  | 3378 | 76 | pub-item names surface in src/output/format.rs |  |  | 0.539 |
| walker |  | 3378 | 0 | pub item at src/output/format.rs:5 |  |  | 0.539 |
| walker |  | 3378 | 0 | pub item at src/output/format.rs:11 |  |  | 0.539 |
| walker |  | 3378 | 0 | pub item at src/output/format.rs:18 |  |  | 0.539 |
| walker |  | 3403 | 25 | pub item body at src/output/format.rs:5 body 6 |  |  | 0.539 |
| walker |  | 3541 | 138 | pub-item names surface in src/options.rs |  |  | 0.563 |
| walker |  | 3559 | 18 | pub item at src/options.rs:102 |  |  | 0.563 |
| walker |  | 3589 | 30 | pub item at src/options.rs:185 |  |  | 0.563 |
| walker |  | 3637 | 48 | pub item at src/options.rs:24 |  |  | 0.563 |
| ns | 3685 |  | 310 | Scheduler::run_benchmarks body | 3.3 | 2.1 | 0.542 |
| walker |  | 3688 | 51 | pub item at src/options.rs:123 |  |  | 0.542 |
| walker |  | 3738 | 50 | pub item at src/options.rs:108 |  |  | 0.542 |
| walker |  | 3807 | 69 | pub item at src/options.rs:71 |  |  | 0.542 |
| walker |  | 3821 | 14 | pub-item doc lede at src/options.rs:24 |  |  | 0.542 |
| walker |  | 3835 | 14 | pub-item doc lede at src/options.rs:71 |  |  | 0.542 |
| walker |  | 3848 | 13 | pub-item doc lede at src/options.rs:108 |  |  | 0.542 |
| ns | 3893 |  | 208 | Benchmark::run — count-of-runs formula | 3.4 | 2.2 | 0.526 |
| walker |  | 3944 | 96 | pub item at src/options.rs:149 |  |  | 0.526 |
| walker |  | 3959 | 15 | pub-item doc lede at src/options.rs:149 |  |  | 0.526 |
| walker |  | 4071 | 112 | pub item at src/options.rs:84 |  |  | 0.526 |
| walker |  | 4081 | 10 | pub-item doc lede at src/options.rs:84 |  |  | 0.526 |
| walker |  | 4097 | 16 | pub-item doc lede at src/util/min_max.rs:2 |  |  | 0.526 |
| walker |  | 4113 | 16 | pub-item doc lede at src/util/min_max.rs:10 |  |  | 0.526 |
| walker |  | 4153 | 40 | pub item body at src/output/format.rs:11 body 12 |  |  | 0.526 |
| walker |  | 4174 | 21 | listing of 'tests' |  |  | 0.529 |
| ns | 4283 |  | 390 | Benchmark::run — warning push logic | 3.5 | 2.2 | 0.504 |
| walker |  | 4409 | 235 | pub item at src/error.rs:7 |  |  | 0.505 |
| walker |  | 4451 | 42 | pub item body at src/util/min_max.rs:2 body 3 |  |  | 0.505 |
| walker |  | 4493 | 42 | pub item body at src/util/min_max.rs:10 body 11 |  |  | 0.505 |
| ns | 4505 |  | 222 | Benchmark::run — BenchmarkResult construction | 3.6 | 2.9 | 0.491 |
| walker |  | 4637 | 144 | README.md section #1 |  |  | 0.513 |
| walker |  | 4655 | 18 | pub-item doc lede at src/output/format.rs:11 |  |  | 0.513 |
| walker |  | 4673 | 18 | pub-item doc lede at src/output/format.rs:18 |  |  | 0.513 |
| walker |  | 4782 | 109 | pub item at src/benchmark/timing_result.rs:5 |  |  | 0.513 |
| walker |  | 4794 | 12 | pub-item doc lede at src/benchmark/timing_result.rs:5 |  |  | 0.513 |
| walker |  | 4874 | 80 | pub item body at src/outlier_detection.rs:43 body 44 |  |  | 0.513 |
| walker |  | 5025 | 151 | mod/use plumbing in src/timer/mod.rs |  |  | 0.513 |
| ns | 5047 |  | 542 | Per-format markup exporters — Markdown / Orgmode | 3.7 | 2.10 | 0.482 |
| walker |  | 5072 | 47 | README.md section #7 |  |  | 0.482 |
| walker |  | 5174 | 102 | pub-item names surface in src/benchmark/relative_speed.rs |  |  | 0.489 |
| walker |  | 5174 | 0 | pub item at src/benchmark/relative_speed.rs:16 |  |  | 0.489 |
| walker |  | 5174 | 0 | pub item at src/benchmark/relative_speed.rs:20 |  |  | 0.489 |
| walker |  | 5213 | 39 | pub item at src/benchmark/relative_speed.rs:112 |  |  | 0.489 |
| walker |  | 5253 | 40 | pub item at src/benchmark/relative_speed.rs:98 |  |  | 0.489 |
| walker |  | 5271 | 18 | pub item body at src/benchmark/relative_speed.rs:16 body 17 |  |  | 0.489 |
| walker |  | 5325 | 54 | pub item at src/benchmark/relative_speed.rs:86 |  |  | 0.489 |
| walker |  | 5352 | 27 | pub item body at src/benchmark/relative_speed.rs:112 body 116 |  |  | 0.489 |
| walker |  | 5427 | 75 | pub item at src/benchmark/relative_speed.rs:7 |  |  | 0.508 |
| walker |  | 5446 | 19 | pub-item doc lede at src/benchmark/relative_speed.rs:112 |  |  | 0.508 |
| ns | 5447 |  | 400 | AsciidocExporter body | 3.8 | 2.10 | 0.487 |
| walker |  | 5495 | 49 | pub item body at src/benchmark/relative_speed.rs:20 body 21 |  |  | 0.487 |
| walker |  | 5551 | 56 | pub item body at src/benchmark/relative_speed.rs:86 body 91 |  |  | 0.487 |
| walker |  | 5608 | 57 | pub item body at src/benchmark/relative_speed.rs:98 body 102 |  |  | 0.487 |
| walker |  | 5652 | 44 | pub-item doc lede at src/outlier_detection.rs:43 |  |  | 0.487 |
| ns | 5665 |  | 218 | JsonExporter body | 3.9 | 2.10 | 0.476 |
| walker |  | 5858 | 206 | mod/use plumbing in src/export/mod.rs |  |  | 0.476 |
| walker |  | 5939 | 81 | impl method sigs in src/benchmark/scheduler.rs |  |  | 0.485 |
| ns | 6202 |  | 537 | CsvExporter body | 3.10 | 2.10 | 0.462 |
| walker |  | 6302 | 363 | pub item at src/error.rs:37 |  |  | 0.463 |
| walker |  | 6559 | 257 | pub item at src/benchmark/executor.rs:35 |  |  | 0.470 |
| ns | 6887 |  | 685 | ShellExecutor::calibrate — shell spawning time measurement | 3.11 | 2.3 | 0.443 |
| walker |  | 6976 | 417 | impl method sigs in src/benchmark/mod.rs |  |  | 0.458 |
| walker |  | 7164 | 188 | impl method sigs in src/options.rs |  |  | 0.461 |
| walker |  | 7230 | 66 | README.md section #8 |  |  | 0.461 |
| walker |  | 7307 | 77 | README.md section #6 |  |  | 0.461 |
| ns | 7425 |  | 538 | ShellExecutor::run_command_and_measure body | 3.12 | 3.11 | 0.444 |
| walker |  | 7619 | 312 | mod/use plumbing in src/benchmark/mod.rs |  |  | 0.444 |
| walker |  | 7883 | 264 | [dependencies] in Cargo.toml |  |  | 0.444 |
| walker |  | 7946 | 63 | pub-item doc lede at src/outlier_detection.rs:13 |  |  | 0.444 |
| ns | 7997 |  | 572 | MockExecutor body (debug-mode) | 3.13 | 2.3 | 0.425 |
| walker |  | 8081 | 135 | impl method sigs in src/export/asciidoc.rs |  |  | 0.428 |
| walker |  | 8270 | 189 | pub item body at src/outlier_detection.rs:21 body 22 |  |  | 0.429 |
| walker |  | 8326 | 56 | listing of 'scripts' |  |  | 0.444 |
| ns | 8406 |  | 409 | outlier_detection module + OUTLIER_THRESHOLD const | 3.14 | 2.12 | 0.447 |
| walker |  | 8829 | 503 | pub item at src/options.rs:198 |  |  | 0.492 |
| walker |  | 8844 | 15 | pub-item doc lede at src/options.rs:198 |  |  | 0.494 |
| ns | 8958 |  | 552 | format_duration auto-unit selection + Unit::short_name/format | 3.15 |  | 0.484 |
| walker |  | 9121 | 277 | impl method sigs in src/command.rs |  |  | 0.497 |
| ns | 9144 |  | 186 | Warnings enum + OutlierWarningOptions struct | 3.16 |  | 0.498 |
| walker |  | 9243 | 122 | pub item body at src/output/format.rs:18 body 19 |  |  | 0.505 |
| walker |  | 9294 | 51 | pub-item doc lede at src/output/format.rs:5 |  |  | 0.505 |
| walker |  | 9400 | 106 | pub-item doc lede at src/outlier_detection.rs:21 |  |  | 0.505 |
| ns | 9531 |  | 387 | OptionsError enum + variants | 4.1 |  | 0.516 |
| ns | 9788 |  | 257 | ParameterScanError enum + variants | 4.2 |  | 0.524 |
| walker |  | 9842 | 442 | pub item at src/benchmark/benchmark_result.rs:11 |  |  | 0.543 |
| ns | 9922 |  | 134 | common.rs test helpers | 4.3 |  | 0.539 |
