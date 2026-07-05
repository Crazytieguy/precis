Score(3000)=0.612 I=0.857 C=0.437 ns_rows≤3K=21/42 (reached=9 partial=1 missing=11)

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
| walker |  | 514 | 146 | README.md section #1 |  |  | 0.429 |
| walker |  | 539 | 25 | listing of 'doc' |  |  | 0.429 |
| ns | 586 |  | 178 | main.rs run() — first half (CLI parsing + builders) | 1.8 |  | 0.385 |
| walker |  | 609 | 70 | README.md section #0 |  |  | 0.408 |
| walker |  | 653 | 44 | listing of 'src' |  |  | 0.549 |
| walker |  | 666 | 13 | listing of 'src/parameter' |  |  | 0.554 |
| walker |  | 678 | 12 | entry item at src/main.rs:53 |  |  | 0.554 |
| walker |  | 691 | 13 | entry item at src/main.rs:29 |  |  | 0.554 |
| walker |  | 700 | 9 | entry item body at src/main.rs:29 body 50 |  |  | 0.554 |
| walker |  | 710 | 10 | entry item body at src/main.rs:29 body 48 |  |  | 0.555 |
| walker |  | 722 | 12 | entry item body at src/main.rs:29 body 31 |  |  | 0.556 |
| walker |  | 734 | 12 | entry item body at src/main.rs:29 body 30 |  |  | 0.558 |
| walker |  | 746 | 12 | entry item body at src/main.rs:29 body 47 |  |  | 0.559 |
| ns | 753 |  | 167 | main.rs run() — second half (Scheduler) + main() wrapper | 1.9 | 1.8 | 0.509 |
| walker |  | 758 | 12 | entry item body at src/main.rs:29 body 46 |  |  | 0.515 |
| walker |  | 774 | 16 | entry item body at src/main.rs:29 body 32 |  |  | 0.520 |
| walker |  | 790 | 16 | entry item body at src/main.rs:29 body 43 |  |  | 0.526 |
| walker |  | 807 | 17 | entry item body at src/main.rs:29 body 34 |  |  | 0.533 |
| walker |  | 825 | 18 | entry item body at src/main.rs:29 body 35 |  |  | 0.544 |
| walker |  | 842 | 17 | entry item body at src/main.rs:29 body 36 |  |  | 0.556 |
| walker |  | 862 | 20 | entry item body at src/main.rs:29 body 45 |  |  | 0.568 |
| walker |  | 879 | 17 | listing of 'src/output' |  |  | 0.586 |
| walker |  | 899 | 20 | listing of 'src/timer' |  |  | 0.617 |
| ns | 909 |  | 156 | README features list | 1.10 |  | 0.636 |
| walker |  | 934 | 35 | pub-item names surface in src/parameter/mod.rs |  |  | 0.636 |
| walker |  | 970 | 36 | pub item at src/parameter/mod.rs:8 |  |  | 0.636 |
| ns | 976 |  | 67 | Scheduler struct header + public method names | 2.1 |  | 0.621 |
| walker |  | 1006 | 36 | pub-item names surface in src/timer/mod.rs |  |  | 0.621 |
| walker |  | 1006 | 0 | pub item at src/timer/mod.rs:83 |  |  | 0.621 |
| ns | 1013 |  | 37 | Benchmark struct header + Benchmark::run signature | 2.2 |  | 0.612 |
| walker |  | 1033 | 27 | listing of 'src/benchmark' |  |  | 0.629 |
| walker |  | 1067 | 34 | pub-item names surface in src/benchmark/mod.rs |  |  | 0.632 |
| walker |  | 1067 | 0 | pub item at src/benchmark/mod.rs:32 |  |  | 0.632 |
| ns | 1108 |  | 95 | Executor trait header + impl struct names | 2.3 |  | 0.609 |
| walker |  | 1117 | 50 | pub item at src/benchmark/mod.rs:34 |  |  | 0.609 |
| walker |  | 1145 | 28 | listing of 'src/util' |  |  | 0.666 |
| walker |  | 1194 | 49 | entry item body at src/main.rs:29 body 37 |  |  | 0.705 |
| walker |  | 1229 | 35 | listing of 'src/export' |  |  | 0.772 |
| walker |  | 1264 | 35 | pub-item names surface in src/export/mod.rs |  |  | 0.772 |
| walker |  | 1285 | 21 | pub item at src/export/mod.rs:56 |  |  | 0.772 |
| walker |  | 1325 | 40 | pub item at src/export/mod.rs:67 |  |  | 0.773 |
| walker |  | 1338 | 13 | pub-item doc lede at src/export/mod.rs:67 |  |  | 0.773 |
| walker |  | 1353 | 15 | pub item at src/util/randomized_environment_offset.rs:6 |  |  | 0.773 |
| walker |  | 1426 | 73 | entry item body at src/main.rs:53 body 54 |  |  | 0.819 |
| ns | 1480 |  | 372 | Options struct field list — first half | 2.4 |  | 0.724 |
| walker |  | 1534 | 108 | pub item at src/export/mod.rs:28 |  |  | 0.727 |
| walker |  | 1573 | 39 | impl method sigs in src/parameter/mod.rs |  |  | 0.727 |
| walker |  | 1590 | 17 | pub-item doc lede at src/export/mod.rs:28 |  |  | 0.727 |
| walker |  | 1677 | 87 | pub item at src/timer/mod.rs:42 |  |  | 0.727 |
| walker |  | 1702 | 25 | mod/use plumbing in src/output/mod.rs |  |  | 0.727 |
| walker |  | 1717 | 15 | pub-item doc lede at src/timer/mod.rs:42 |  |  | 0.727 |
| ns | 1720 |  | 240 | Options struct field list — second half | 2.5 | 2.4 | 0.678 |
| walker |  | 1745 | 28 | pub-item names surface in src/error.rs |  |  | 0.678 |
| walker |  | 1760 | 15 | pub-item doc lede at src/timer/mod.rs:83 |  |  | 0.678 |
| walker |  | 1781 | 21 | pub item at src/export/csv.rs:13 |  |  | 0.678 |
| walker |  | 1802 | 21 | pub item at src/export/json.rs:17 |  |  | 0.678 |
| walker |  | 1823 | 21 | pub item at src/export/markdown.rs:6 |  |  | 0.678 |
| walker |  | 1842 | 19 | pub item at src/parameter/tokenize.rs:1 |  |  | 0.678 |
| walker |  | 1864 | 22 | pub item at src/export/orgmode.rs:5 |  |  | 0.678 |
| ns | 1870 |  | 150 | Options sibling enum/struct headers | 2.6 |  | 0.652 |
| walker |  | 1877 | 13 | pub-item doc lede at src/benchmark/mod.rs:32 |  |  | 0.652 |
| ns | 1901 |  | 31 | Options::from_cli_arguments + validate_against_command_list signatures | 2.7 |  | 0.648 |
| walker |  | 1911 | 34 | pub-item names surface in src/command.rs |  |  | 0.649 |
| walker |  | 1911 | 0 | pub item at src/command.rs:134 |  |  | 0.649 |
| walker |  | 1935 | 24 | pub item at src/export/asciidoc.rs:5 |  |  | 0.649 |
| walker |  | 1963 | 28 | pub item at src/timer/wall_clock_timer.rs:5 |  |  | 0.649 |
| walker |  | 2002 | 39 | mod/use plumbing in src/parameter/mod.rs |  |  | 0.649 |
| walker |  | 2031 | 29 | pub item at src/timer/windows_timer.rs:49 |  |  | 0.649 |
| walker |  | 2055 | 24 | pub-item names surface in src/export/markup.rs |  |  | 0.649 |
| walker |  | 2072 | 17 | pub item at src/export/markup.rs:10 |  |  | 0.649 |
| ns | 2088 |  | 187 | Command + Commands type headers + public method names | 2.8 |  | 0.625 |
| walker |  | 2097 | 25 | pub-item names surface in src/parameter/range_step.rs |  |  | 0.625 |
| walker |  | 2122 | 25 | pub-item names surface in src/timer/unix_timer.rs |  |  | 0.625 |
| walker |  | 2139 | 17 | pub item at src/timer/unix_timer.rs:18 |  |  | 0.625 |
| walker |  | 2165 | 26 | pub-item names surface in src/output/warnings.rs |  |  | 0.625 |
| walker |  | 2195 | 30 | pub item at src/output/warnings.rs:7 |  |  | 0.625 |
| walker |  | 2239 | 44 | mod/use plumbing in src/util/mod.rs |  |  | 0.625 |
| walker |  | 2272 | 33 | pub item at src/output/progress_bar.rs:13 |  |  | 0.625 |
| ns | 2384 |  | 296 | BenchmarkResult fields (the in-memory result row) | 2.9 |  | 0.592 |
| walker |  | 2437 | 165 | mod/use plumbing in src/main.rs |  |  | 0.641 |
| walker |  | 2477 | 40 | pub item at src/parameter/range_step.rs:34 |  |  | 0.641 |
| walker |  | 2531 | 54 | pub item at src/output/warnings.rs:13 |  |  | 0.642 |
| walker |  | 2601 | 70 | pub item at src/cli.rs:8 |  |  | 0.642 |
| walker |  | 2623 | 22 | pub item body at src/cli.rs:8 body 13 |  |  | 0.642 |
| walker |  | 2679 | 56 | pub item at src/util/number.rs:10 |  |  | 0.642 |
| walker |  | 2694 | 15 | pub-item doc lede at src/command.rs:134 |  |  | 0.642 |
| ns | 2718 |  | 334 | ExportType + Exporter trait + ExportManager headers | 2.10 |  | 0.626 |
| walker |  | 2731 | 37 | pub-item names surface in src/util/units.rs |  |  | 0.626 |
| walker |  | 2731 | 0 | pub item at src/util/units.rs:6 |  |  | 0.626 |
| walker |  | 2776 | 45 | pub item at src/util/units.rs:10 |  |  | 0.626 |
| walker |  | 2783 | 7 | pub-item doc lede at src/util/units.rs:10 |  |  | 0.626 |
| walker |  | 2793 | 10 | pub-item doc lede at src/output/warnings.rs:13 |  |  | 0.627 |
| ns | 2858 |  | 140 | Benchmark module-level helpers + MIN_EXECUTION_TIME | 2.11 |  | 0.612 |
| walker |  | 2865 | 72 | pub-item names surface in src/outlier_detection.rs |  |  | 0.612 |
| walker |  | 2865 | 0 | pub item at src/outlier_detection.rs:13 |  |  | 0.612 |
| walker |  | 2865 | 0 | pub item at src/outlier_detection.rs:21 |  |  | 0.612 |
| walker |  | 2865 | 0 | pub item at src/outlier_detection.rs:43 |  |  | 0.612 |
| walker |  | 2905 | 40 | pub-item names surface in src/util/min_max.rs |  |  | 0.612 |
| walker |  | 2905 | 0 | pub item at src/util/min_max.rs:2 |  |  | 0.612 |
| walker |  | 2905 | 0 | pub item at src/util/min_max.rs:10 |  |  | 0.612 |
| walker |  | 2926 | 21 | pub item body at src/util/randomized_environment_offset.rs:6 body 7 |  |  | 0.612 |
| walker |  | 3050 | 124 | impl method sigs in src/export/mod.rs |  |  | 0.624 |
| ns | 3054 |  | 196 | relative_speed + outlier_detection signatures | 2.12 |  | 0.607 |
| ns | 3131 |  | 77 | tests/ + scripts/ fs listings | 2.13 |  | 0.592 |
| walker |  | 3153 | 103 | pub item at src/command.rs:22 |  |  | 0.594 |
| walker |  | 3166 | 13 | pub-item doc lede at src/command.rs:22 |  |  | 0.594 |
| walker |  | 3212 | 46 | pub-item names surface in src/util/exit_code.rs |  |  | 0.594 |
| walker |  | 3212 | 0 | pub item at src/util/exit_code.rs:4 |  |  | 0.594 |
| walker |  | 3212 | 0 | pub item at src/util/exit_code.rs:20 |  |  | 0.594 |
| walker |  | 3220 | 8 | pub item body at src/util/exit_code.rs:20 body 21 |  |  | 0.594 |
| walker |  | 3254 | 34 | README.md section #4 |  |  | 0.594 |
| walker |  | 3321 | 67 | pub item at src/timer/unix_timer.rs:10 |  |  | 0.594 |
| walker |  | 3390 | 69 | pub item at src/benchmark/scheduler.rs:13 |  |  | 0.594 |
| walker |  | 3489 | 99 | pub item at src/parameter/range_step.rs:7 |  |  | 0.594 |
| walker |  | 3570 | 81 | man-page NAME + DESCRIPTION in doc/hyperfine.1 |  |  | 0.563 |
| ns | 3570 |  | 439 | CLI flag names — every Arg::new() line | 3.1 |  | 0.563 |
| walker |  | 3580 | 10 | pub-item doc lede at src/util/units.rs:6 |  |  | 0.563 |
| ns | 3621 |  | 51 | RunBounds default — '10 runs by default' | 3.2 |  | 0.558 |
| walker |  | 3642 | 62 | pub-item names surface in src/benchmark/executor.rs |  |  | 0.564 |
| walker |  | 3658 | 16 | pub item at src/benchmark/executor.rs:122 |  |  | 0.564 |
| walker |  | 3681 | 23 | pub item at src/benchmark/executor.rs:303 |  |  | 0.567 |
| walker |  | 3714 | 33 | pub item at src/benchmark/executor.rs:19 |  |  | 0.572 |
| walker |  | 3756 | 42 | pub item at src/benchmark/executor.rs:169 |  |  | 0.572 |
| walker |  | 3769 | 13 | pub-item doc lede at src/output/progress_bar.rs:13 |  |  | 0.572 |
| walker |  | 3783 | 14 | pub-item doc lede at src/util/min_max.rs:2 |  |  | 0.572 |
| walker |  | 3923 | 140 | pub-item names surface in src/options.rs |  |  | 0.594 |
| ns | 3931 |  | 310 | Scheduler::run_benchmarks body | 3.3 | 2.1 | 0.572 |
| walker |  | 3960 | 37 | pub item at src/options.rs:102 |  |  | 0.572 |
| walker |  | 4003 | 43 | pub item at src/options.rs:185 |  |  | 0.572 |
| walker |  | 4064 | 61 | pub item at src/options.rs:24 |  |  | 0.572 |
| walker |  | 4116 | 52 | pub item at src/options.rs:108 |  |  | 0.572 |
| ns | 4141 |  | 210 | Benchmark::run — count-of-runs formula | 3.4 | 2.2 | 0.556 |
| walker |  | 4184 | 68 | pub item at src/options.rs:123 |  |  | 0.556 |
| walker |  | 4196 | 12 | pub-item doc lede at src/options.rs:24 |  |  | 0.556 |
| walker |  | 4207 | 11 | pub-item doc lede at src/options.rs:108 |  |  | 0.556 |
| walker |  | 4293 | 86 | pub item at src/options.rs:71 |  |  | 0.556 |
| walker |  | 4307 | 14 | pub-item doc lede at src/options.rs:71 |  |  | 0.556 |
| walker |  | 4422 | 115 | pub item at src/options.rs:149 |  |  | 0.556 |
| walker |  | 4437 | 15 | pub-item doc lede at src/options.rs:149 |  |  | 0.556 |
| ns | 4533 |  | 392 | Benchmark::run — warning push logic | 3.5 | 2.2 | 0.530 |
| walker |  | 4566 | 129 | pub item at src/options.rs:84 |  |  | 0.530 |
| walker |  | 4574 | 8 | pub-item doc lede at src/options.rs:84 |  |  | 0.530 |
| walker |  | 4590 | 16 | pub-item doc lede at src/util/min_max.rs:10 |  |  | 0.530 |
| walker |  | 4668 | 78 | pub-item names surface in src/output/format.rs |  |  | 0.530 |
| walker |  | 4668 | 0 | pub item at src/output/format.rs:5 |  |  | 0.530 |
| walker |  | 4668 | 0 | pub item at src/output/format.rs:11 |  |  | 0.530 |
| walker |  | 4668 | 0 | pub item at src/output/format.rs:18 |  |  | 0.530 |
| walker |  | 4695 | 27 | pub item body at src/output/format.rs:5 body 6 |  |  | 0.530 |
| walker |  | 4716 | 21 | listing of 'tests' |  |  | 0.532 |
| ns | 4757 |  | 224 | Benchmark::run — BenchmarkResult construction | 3.6 | 2.9 | 0.518 |
| walker |  | 4758 | 42 | pub item body at src/output/format.rs:11 body 12 |  |  | 0.518 |
| walker |  | 5005 | 247 | pub item at src/error.rs:7 |  |  | 0.519 |
| walker |  | 5023 | 18 | pub-item doc lede at src/output/format.rs:11 |  |  | 0.519 |
| walker |  | 5041 | 18 | pub-item doc lede at src/output/format.rs:18 |  |  | 0.519 |
| walker |  | 5085 | 44 | pub item body at src/util/min_max.rs:2 body 3 |  |  | 0.519 |
| walker |  | 5129 | 44 | pub item body at src/util/min_max.rs:10 body 11 |  |  | 0.519 |
| walker |  | 5211 | 82 | pub item body at src/outlier_detection.rs:43 body 44 |  |  | 0.519 |
| ns | 5303 |  | 546 | Per-format markup exporters — Markdown / Orgmode | 3.7 | 2.10 | 0.488 |
| walker |  | 5374 | 163 | mod/use plumbing in src/timer/mod.rs |  |  | 0.488 |
| walker |  | 5499 | 125 | pub item at src/benchmark/timing_result.rs:5 |  |  | 0.488 |
| walker |  | 5511 | 12 | pub-item doc lede at src/benchmark/timing_result.rs:5 |  |  | 0.488 |
| walker |  | 5615 | 104 | pub-item names surface in src/benchmark/relative_speed.rs |  |  | 0.495 |
| walker |  | 5615 | 0 | pub item at src/benchmark/relative_speed.rs:16 |  |  | 0.495 |
| walker |  | 5615 | 0 | pub item at src/benchmark/relative_speed.rs:20 |  |  | 0.495 |
| walker |  | 5654 | 39 | pub item at src/benchmark/relative_speed.rs:112 |  |  | 0.495 |
| walker |  | 5694 | 40 | pub item at src/benchmark/relative_speed.rs:98 |  |  | 0.495 |
| ns | 5705 |  | 402 | AsciidocExporter body | 3.8 | 2.10 | 0.475 |
| walker |  | 5714 | 20 | pub item body at src/benchmark/relative_speed.rs:16 body 17 |  |  | 0.475 |
| walker |  | 5768 | 54 | pub item at src/benchmark/relative_speed.rs:86 |  |  | 0.475 |
| walker |  | 5797 | 29 | pub item body at src/benchmark/relative_speed.rs:112 body 116 |  |  | 0.475 |
| walker |  | 5880 | 83 | pub item at src/benchmark/relative_speed.rs:7 |  |  | 0.493 |
| walker |  | 5899 | 19 | pub-item doc lede at src/benchmark/relative_speed.rs:112 |  |  | 0.493 |
| ns | 5925 |  | 220 | JsonExporter body | 3.9 | 2.10 | 0.482 |
| walker |  | 5950 | 51 | pub item body at src/benchmark/relative_speed.rs:20 body 21 |  |  | 0.482 |
| walker |  | 5999 | 49 | README.md section #7 |  |  | 0.482 |
| walker |  | 6057 | 58 | pub item body at src/benchmark/relative_speed.rs:86 body 91 |  |  | 0.482 |
| walker |  | 6116 | 59 | pub item body at src/benchmark/relative_speed.rs:98 body 102 |  |  | 0.482 |
| walker |  | 6195 | 79 | impl method sigs in src/benchmark/scheduler.rs |  |  | 0.490 |
| walker |  | 6401 | 206 | mod/use plumbing in src/export/mod.rs |  |  | 0.490 |
| walker |  | 6447 | 46 | pub-item doc lede at src/outlier_detection.rs:43 |  |  | 0.490 |
| ns | 6466 |  | 541 | CsvExporter body | 3.10 | 2.10 | 0.467 |
| walker |  | 6820 | 373 | pub item at src/error.rs:37 |  |  | 0.468 |
| walker |  | 7079 | 259 | pub item at src/benchmark/executor.rs:35 |  |  | 0.476 |
| ns | 7153 |  | 687 | ShellExecutor::calibrate — shell spawning time measurement | 3.11 | 2.3 | 0.449 |
| walker |  | 7494 | 415 | impl method sigs in src/benchmark/mod.rs |  |  | 0.464 |
| walker |  | 7558 | 64 | README.md section #8 |  |  | 0.464 |
| ns | 7693 |  | 540 | ShellExecutor::run_command_and_measure body | 3.12 | 3.11 | 0.447 |
| walker |  | 7744 | 186 | impl method sigs in src/options.rs |  |  | 0.450 |
| walker |  | 7821 | 77 | README.md section #6 |  |  | 0.450 |
| walker |  | 8131 | 310 | mod/use plumbing in src/benchmark/mod.rs |  |  | 0.450 |
| ns | 8267 |  | 574 | MockExecutor body (debug-mode) | 3.13 | 2.3 | 0.431 |
| walker |  | 8399 | 268 | [dependencies] in Cargo.toml |  |  | 0.431 |
| walker |  | 8532 | 133 | impl method sigs in src/export/asciidoc.rs |  |  | 0.435 |
| walker |  | 8595 | 63 | pub-item doc lede at src/outlier_detection.rs:13 |  |  | 0.435 |
| ns | 8674 |  | 407 | outlier_detection module + OUTLIER_THRESHOLD const | 3.14 | 2.12 | 0.427 |
| walker |  | 8786 | 191 | pub item body at src/outlier_detection.rs:21 body 22 |  |  | 0.439 |
| walker |  | 8842 | 56 | listing of 'scripts' |  |  | 0.453 |
| ns | 9234 |  | 560 | format_duration auto-unit selection + Unit::short_name/format | 3.15 |  | 0.444 |
| walker |  | 9347 | 505 | pub item at src/options.rs:198 |  |  | 0.488 |
| walker |  | 9362 | 15 | pub-item doc lede at src/options.rs:198 |  |  | 0.490 |
| ns | 9422 |  | 188 | Warnings enum + OutlierWarningOptions struct | 3.16 |  | 0.491 |
| walker |  | 9635 | 273 | impl method sigs in src/command.rs |  |  | 0.506 |
| walker |  | 9686 | 51 | pub-item doc lede at src/output/format.rs:5 |  |  | 0.506 |
| walker |  | 9810 | 124 | pub item body at src/output/format.rs:18 body 19 |  |  | 0.512 |
| ns | 9811 |  | 389 | OptionsError enum + variants | 4.1 |  | 0.525 |
| walker |  | 9914 | 104 | pub-item doc lede at src/outlier_detection.rs:21 |  |  | 0.525 |
| ns | 10070 |  | 259 | ParameterScanError enum + variants | 4.2 |  | 0.534 |
| ns | 10204 |  | 134 | common.rs test helpers | 4.3 |  | 0.530 |
