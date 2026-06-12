Score(3000)=0.527 I=0.802 C=0.346 ns_rows≤3K=23/42 (reached=7 partial=1 missing=15)

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
| walker |  | 966 | 28 | listing of 'src/util' |  |  | 0.624 |
| ns | 979 |  | 29 | Benchmark struct header + Benchmark::run signature | 2.2 |  | 0.615 |
| walker |  | 1017 | 51 | entry item body at src/main.rs:29 body 37 |  |  | 0.658 |
| walker |  | 1052 | 35 | listing of 'src/export' |  |  | 0.687 |
| ns | 1058 |  | 79 | Executor trait header + impl struct names | 2.3 |  | 0.663 |
| walker |  | 1085 | 33 | pub-item names surface in src/export/mod.rs |  |  | 0.663 |
| walker |  | 1104 | 19 | pub item at src/export/mod.rs:56 |  |  | 0.663 |
| walker |  | 1142 | 38 | pub item at src/export/mod.rs:67 |  |  | 0.663 |
| walker |  | 1155 | 13 | pub item at src/util/randomized_environment_offset.rs:6 |  |  | 0.663 |
| walker |  | 1228 | 73 | pub item at src/timer/mod.rs:42 |  |  | 0.663 |
| walker |  | 1326 | 98 | pub item at src/export/mod.rs:28 |  |  | 0.666 |
| walker |  | 1339 | 13 | pub-item doc lede at src/export/mod.rs:67 |  |  | 0.666 |
| walker |  | 1410 | 71 | entry item body at src/main.rs:53 body 54 |  |  | 0.716 |
| ns | 1426 |  | 368 | Options struct field list — first half | 2.4 |  | 0.632 |
| walker |  | 1436 | 26 | pub-item names surface in src/error.rs |  |  | 0.632 |
| walker |  | 1453 | 17 | pub-item doc lede at src/export/mod.rs:28 |  |  | 0.632 |
| walker |  | 1478 | 25 | mod/use plumbing in src/output/mod.rs |  |  | 0.632 |
| walker |  | 1519 | 41 | impl method sigs in src/parameter/mod.rs |  |  | 0.632 |
| walker |  | 1534 | 15 | pub-item doc lede at src/timer/mod.rs:42 |  |  | 0.632 |
| walker |  | 1549 | 15 | pub-item doc lede at src/timer/mod.rs:83 |  |  | 0.632 |
| walker |  | 1568 | 19 | pub item at src/parameter/tokenize.rs:1 |  |  | 0.632 |
| walker |  | 1600 | 32 | pub-item names surface in src/command.rs |  |  | 0.632 |
| walker |  | 1600 | 0 | pub item at src/command.rs:134 |  |  | 0.632 |
| walker |  | 1624 | 24 | pub item at src/timer/wall_clock_timer.rs:5 |  |  | 0.632 |
| walker |  | 1655 | 31 | pub item at src/util/number.rs:10 |  |  | 0.632 |
| ns | 1666 |  | 240 | Options struct field list — second half | 2.5 | 2.4 | 0.590 |
| walker |  | 1680 | 25 | pub item at src/timer/windows_timer.rs:49 |  |  | 0.590 |
| walker |  | 1702 | 22 | pub-item names surface in src/export/markup.rs |  |  | 0.590 |
| walker |  | 1719 | 17 | pub item at src/export/markup.rs:10 |  |  | 0.590 |
| walker |  | 1742 | 23 | pub-item names surface in src/parameter/range_step.rs |  |  | 0.590 |
| walker |  | 1765 | 23 | pub-item names surface in src/timer/unix_timer.rs |  |  | 0.590 |
| walker |  | 1780 | 15 | pub item at src/timer/unix_timer.rs:18 |  |  | 0.590 |
| ns | 1798 |  | 132 | Options sibling enum/struct headers | 2.6 |  | 0.567 |
| walker |  | 1804 | 24 | pub-item names surface in src/output/warnings.rs |  |  | 0.567 |
| ns | 1825 |  | 27 | Options::from_cli_arguments + validate_against_command_list signatures | 2.7 |  | 0.564 |
| walker |  | 1832 | 28 | pub item at src/output/warnings.rs:7 |  |  | 0.564 |
| walker |  | 1862 | 30 | pub item at src/parameter/range_step.rs:34 |  |  | 0.564 |
| walker |  | 1903 | 41 | mod/use plumbing in src/parameter/mod.rs |  |  | 0.564 |
| walker |  | 1947 | 44 | mod/use plumbing in src/util/mod.rs |  |  | 0.564 |
| walker |  | 1978 | 31 | pub item at src/output/progress_bar.rs:13 |  |  | 0.564 |
| ns | 1984 |  | 159 | Command + Commands type headers + public method names | 2.8 |  | 0.544 |
| walker |  | 2143 | 165 | mod/use plumbing in src/main.rs |  |  | 0.597 |
| walker |  | 2195 | 52 | pub item at src/output/warnings.rs:13 |  |  | 0.597 |
| ns | 2254 |  | 270 | BenchmarkResult fields (the in-memory result row) | 2.9 |  | 0.566 |
| walker |  | 2263 | 68 | pub item at src/cli.rs:8 |  |  | 0.566 |
| walker |  | 2283 | 20 | pub item body at src/cli.rs:8 body 13 |  |  | 0.566 |
| walker |  | 2318 | 35 | pub-item names surface in src/util/units.rs |  |  | 0.566 |
| walker |  | 2318 | 0 | pub item at src/util/units.rs:6 |  |  | 0.566 |
| walker |  | 2344 | 26 | pub item at src/util/units.rs:10 |  |  | 0.566 |
| walker |  | 2353 | 9 | pub-item doc lede at src/util/units.rs:10 |  |  | 0.566 |
| walker |  | 2368 | 15 | pub-item doc lede at src/command.rs:134 |  |  | 0.566 |
| walker |  | 2406 | 38 | pub-item names surface in src/util/min_max.rs |  |  | 0.566 |
| walker |  | 2406 | 0 | pub item at src/util/min_max.rs:2 |  |  | 0.566 |
| walker |  | 2406 | 0 | pub item at src/util/min_max.rs:10 |  |  | 0.566 |
| walker |  | 2492 | 86 | pub item at src/command.rs:22 |  |  | 0.566 |
| walker |  | 2505 | 13 | pub-item doc lede at src/command.rs:22 |  |  | 0.566 |
| walker |  | 2524 | 19 | pub item body at src/util/randomized_environment_offset.rs:6 body 7 |  |  | 0.566 |
| ns | 2570 |  | 316 | ExportType + Exporter trait + ExportManager headers | 2.10 |  | 0.556 |
| walker |  | 2594 | 70 | pub-item names surface in src/outlier_detection.rs |  |  | 0.556 |
| walker |  | 2594 | 0 | pub item at src/outlier_detection.rs:13 |  |  | 0.556 |
| walker |  | 2594 | 0 | pub item at src/outlier_detection.rs:21 |  |  | 0.556 |
| walker |  | 2594 | 0 | pub item at src/outlier_detection.rs:43 |  |  | 0.556 |
| walker |  | 2638 | 44 | pub-item names surface in src/util/exit_code.rs |  |  | 0.556 |
| walker |  | 2638 | 0 | pub item at src/util/exit_code.rs:4 |  |  | 0.556 |
| walker |  | 2638 | 0 | pub item at src/util/exit_code.rs:20 |  |  | 0.556 |
| walker |  | 2644 | 6 | pub item body at src/util/exit_code.rs:20 body 21 |  |  | 0.556 |
| ns | 2696 |  | 126 | Benchmark module-level helpers + MIN_EXECUTION_TIME | 2.11 |  | 0.542 |
| walker |  | 2699 | 55 | pub item at src/timer/unix_timer.rs:10 |  |  | 0.542 |
| walker |  | 2711 | 12 | pub-item doc lede at src/output/warnings.rs:13 |  |  | 0.542 |
| walker |  | 2835 | 124 | impl method sigs in src/export/mod.rs |  |  | 0.555 |
| ns | 2874 |  | 178 | relative_speed + outlier_detection signatures | 2.12 |  | 0.540 |
| walker |  | 2932 | 97 | pub item at src/parameter/range_step.rs:7 |  |  | 0.540 |
| walker |  | 2945 | 13 | pub-item doc lede at src/output/progress_bar.rs:13 |  |  | 0.540 |
| ns | 2951 |  | 77 | tests/ + scripts/ fs listings | 2.13 |  | 0.527 |
| walker |  | 2957 | 12 | pub-item doc lede at src/util/units.rs:6 |  |  | 0.527 |
| walker |  | 3033 | 76 | pub-item names surface in src/output/format.rs |  |  | 0.527 |
| walker |  | 3033 | 0 | pub item at src/output/format.rs:5 |  |  | 0.527 |
| walker |  | 3033 | 0 | pub item at src/output/format.rs:11 |  |  | 0.527 |
| walker |  | 3033 | 0 | pub item at src/output/format.rs:18 |  |  | 0.527 |
| walker |  | 3058 | 25 | pub item body at src/output/format.rs:5 body 6 |  |  | 0.527 |
| walker |  | 3196 | 138 | pub-item names surface in src/options.rs |  |  | 0.553 |
| walker |  | 3214 | 18 | pub item at src/options.rs:102 |  |  | 0.553 |
| walker |  | 3244 | 30 | pub item at src/options.rs:185 |  |  | 0.553 |
| walker |  | 3292 | 48 | pub item at src/options.rs:24 |  |  | 0.553 |
| ns | 3326 |  | 375 | CLI flag names — every Arg::new() line | 3.1 |  | 0.524 |
| walker |  | 3343 | 51 | pub item at src/options.rs:123 |  |  | 0.524 |
| ns | 3375 |  | 49 | RunBounds default — '10 runs by default' | 3.2 |  | 0.519 |
| walker |  | 3393 | 50 | pub item at src/options.rs:108 |  |  | 0.519 |
| walker |  | 3462 | 69 | pub item at src/options.rs:71 |  |  | 0.519 |
| walker |  | 3476 | 14 | pub-item doc lede at src/options.rs:24 |  |  | 0.519 |
| walker |  | 3490 | 14 | pub-item doc lede at src/options.rs:71 |  |  | 0.519 |
| walker |  | 3503 | 13 | pub-item doc lede at src/options.rs:108 |  |  | 0.519 |
| walker |  | 3599 | 96 | pub item at src/options.rs:149 |  |  | 0.519 |
| walker |  | 3614 | 15 | pub-item doc lede at src/options.rs:149 |  |  | 0.519 |
| ns | 3685 |  | 310 | Scheduler::run_benchmarks body | 3.3 | 2.1 | 0.500 |
| walker |  | 3726 | 112 | pub item at src/options.rs:84 |  |  | 0.500 |
| walker |  | 3736 | 10 | pub-item doc lede at src/options.rs:84 |  |  | 0.500 |
| walker |  | 3752 | 16 | pub-item doc lede at src/util/min_max.rs:2 |  |  | 0.500 |
| walker |  | 3768 | 16 | pub-item doc lede at src/util/min_max.rs:10 |  |  | 0.500 |
| walker |  | 3808 | 40 | pub item body at src/output/format.rs:11 body 12 |  |  | 0.500 |
| walker |  | 3829 | 21 | listing of 'tests' |  |  | 0.503 |
| ns | 3893 |  | 208 | Benchmark::run — count-of-runs formula | 3.4 | 2.2 | 0.489 |
| walker |  | 4064 | 235 | pub item at src/error.rs:7 |  |  | 0.489 |
| walker |  | 4106 | 42 | pub item body at src/util/min_max.rs:2 body 3 |  |  | 0.489 |
| walker |  | 4148 | 42 | pub item body at src/util/min_max.rs:10 body 11 |  |  | 0.489 |
| ns | 4283 |  | 390 | Benchmark::run — warning push logic | 3.5 | 2.2 | 0.466 |
| walker |  | 4292 | 144 | README.md section #1 |  |  | 0.489 |
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
| walker |  | 5434 | 188 | impl method sigs in src/options.rs |  |  | 0.476 |
| ns | 5447 |  | 400 | AsciidocExporter body | 3.8 | 2.10 | 0.457 |
| walker |  | 5500 | 66 | README.md section #8 |  |  | 0.457 |
| walker |  | 5577 | 77 | README.md section #6 |  |  | 0.457 |
| ns | 5665 |  | 218 | JsonExporter body | 3.9 | 2.10 | 0.446 |
| walker |  | 5841 | 264 | [dependencies] in Cargo.toml |  |  | 0.446 |
| walker |  | 5904 | 63 | pub-item doc lede at src/outlier_detection.rs:13 |  |  | 0.446 |
| walker |  | 6039 | 135 | impl method sigs in src/export/asciidoc.rs |  |  | 0.450 |
| ns | 6202 |  | 537 | CsvExporter body | 3.10 | 2.10 | 0.429 |
| walker |  | 6228 | 189 | pub item body at src/outlier_detection.rs:21 body 22 |  |  | 0.429 |
| walker |  | 6284 | 56 | listing of 'scripts' |  |  | 0.448 |
| walker |  | 6787 | 503 | pub item at src/options.rs:198 |  |  | 0.505 |
| walker |  | 6802 | 15 | pub-item doc lede at src/options.rs:198 |  |  | 0.508 |
| ns | 6887 |  | 685 | ShellExecutor::calibrate — shell spawning time measurement | 3.11 | 2.3 | 0.479 |
| walker |  | 7079 | 277 | impl method sigs in src/command.rs |  |  | 0.495 |
| walker |  | 7201 | 122 | pub item body at src/output/format.rs:18 body 19 |  |  | 0.495 |
| walker |  | 7252 | 51 | pub-item doc lede at src/output/format.rs:5 |  |  | 0.495 |
| walker |  | 7358 | 106 | pub-item doc lede at src/outlier_detection.rs:21 |  |  | 0.495 |
| ns | 7425 |  | 538 | ShellExecutor::run_command_and_measure body | 3.12 | 3.11 | 0.478 |
| walker |  | 7538 | 180 | pub item body at src/util/exit_code.rs:4 body 5 |  |  | 0.478 |
| walker |  | 7579 | 41 | impl method sigs in src/timer/wall_clock_timer.rs |  |  | 0.478 |
| walker |  | 7621 | 42 | impl method sigs in src/util/units.rs |  |  | 0.478 |
| walker |  | 7666 | 45 | impl method sigs in src/parameter/range_step.rs |  |  | 0.478 |
| walker |  | 7712 | 46 | impl method sigs in src/timer/unix_timer.rs |  |  | 0.478 |
| ns | 7997 |  | 572 | MockExecutor body (debug-mode) | 3.13 | 2.3 | 0.457 |
| ns | 8406 |  | 409 | outlier_detection module + OUTLIER_THRESHOLD const | 3.14 | 2.12 | 0.460 |
| walker |  | 8558 | 846 | pub item at src/export/markup.rs:15 |  |  | 0.460 |
| walker |  | 8667 | 109 | pub-item doc lede at src/util/randomized_environment_offset.rs:6 |  |  | 0.460 |
| walker |  | 8861 | 194 | README.md section #5 |  |  | 0.460 |
| walker |  | 8917 | 56 | impl method sigs in src/timer/windows_timer.rs |  |  | 0.460 |
| walker |  | 8951 | 34 | pub-item names surface in src/benchmark/mod.rs |  |  | 0.461 |
| walker |  | 8951 | 0 | pub item at src/benchmark/mod.rs:32 |  |  | 0.461 |
| ns | 8958 |  | 552 | format_duration auto-unit selection + Unit::short_name/format | 3.15 |  | 0.461 |
| walker |  | 8959 | 8 | listing of '.github' |  |  | 0.461 |
| walker |  | 8964 | 5 | listing of '.github/workflows' |  |  | 0.461 |
| walker |  | 9012 | 48 | pub item at src/benchmark/mod.rs:34 |  |  | 0.461 |
| walker |  | 9092 | 80 | impl method sigs in src/export/markdown.rs |  |  | 0.462 |
| ns | 9144 |  | 186 | Warnings enum + OutlierWarningOptions struct | 3.16 |  | 0.464 |
| walker |  | 9173 | 81 | impl method sigs in src/export/orgmode.rs |  |  | 0.466 |
| walker |  | 9191 | 18 | pub item at tests/integration_tests.rs:12 |  |  | 0.466 |
| walker |  | 9204 | 13 | pub-item doc lede at src/benchmark/mod.rs:32 |  |  | 0.466 |
| walker |  | 9237 | 33 | pub-item names surface in tests/common.rs |  |  | 0.466 |
| walker |  | 9237 | 0 | pub item at tests/common.rs:5 |  |  | 0.466 |
| walker |  | 9237 | 0 | pub item at tests/common.rs:11 |  |  | 0.466 |
| walker |  | 9253 | 16 | pub item body at tests/common.rs:11 body 12 |  |  | 0.466 |
| walker |  | 9260 | 7 | pub item body at src/parameter/tokenize.rs:1 body 30 |  |  | 0.466 |
| walker |  | 9268 | 8 | pub item body at src/output/progress_bar.rs:13 body 30 |  |  | 0.466 |
| walker |  | 9359 | 91 | README headline in scripts/README.md |  |  | 0.466 |
| walker |  | 9368 | 9 | pub item body at src/parameter/tokenize.rs:1 body 28 |  |  | 0.466 |
| walker |  | 9396 | 28 | pub item body at tests/integration_tests.rs:12 body 13 |  |  | 0.466 |
| walker |  | 9406 | 10 | pub item body at src/parameter/tokenize.rs:1 body 2 |  |  | 0.466 |
| walker |  | 9441 | 35 | pub item body at tests/common.rs:5 body 6 |  |  | 0.466 |
| walker |  | 9454 | 13 | pub item body at src/output/progress_bar.rs:13 body 26 |  |  | 0.466 |
| walker |  | 9467 | 13 | pub item body at src/output/progress_bar.rs:13 body 28 |  |  | 0.466 |
| walker |  | 9480 | 13 | pub item body at src/parameter/tokenize.rs:1 body 3 |  |  | 0.466 |
| walker |  | 9493 | 13 | pub item body at src/parameter/tokenize.rs:1 body 5 |  |  | 0.466 |
| ns | 9531 |  | 387 | OptionsError enum + variants | 4.1 |  | 0.480 |
| walker |  | 9558 | 65 | pub item at src/benchmark/scheduler.rs:13 |  |  | 0.480 |
| ns | 9788 |  | 257 | ParameterScanError enum + variants | 4.2 |  | 0.489 |
| ns | 9922 |  | 134 | common.rs test helpers | 4.3 |  | 0.489 |
