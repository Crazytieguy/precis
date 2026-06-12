Score(3000)=0.553 I=0.831 C=0.369 ns_rows≤3K=23/42 (reached=8 partial=1 missing=14)

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
| walker |  | 919 | 34 | pub item at src/parameter/mod.rs:8 |  |  | 0.574 |
| ns | 950 |  | 55 | Scheduler struct header + public method names | 2.1 |  | 0.560 |
| walker |  | 953 | 34 | pub-item names surface in src/timer/mod.rs |  |  | 0.560 |
| walker |  | 953 | 0 | pub item at src/timer/mod.rs:83 |  |  | 0.560 |
| ns | 979 |  | 29 | Benchmark struct header + Benchmark::run signature | 2.2 |  | 0.553 |
| walker |  | 980 | 27 | listing of 'src/benchmark' |  |  | 0.570 |
| walker |  | 1014 | 34 | pub-item names surface in src/benchmark/mod.rs |  |  | 0.573 |
| walker |  | 1014 | 0 | pub item at src/benchmark/mod.rs:32 |  |  | 0.573 |
| ns | 1058 |  | 79 | Executor trait header + impl struct names | 2.3 |  | 0.553 |
| walker |  | 1062 | 48 | pub item at src/benchmark/mod.rs:34 |  |  | 0.553 |
| walker |  | 1090 | 28 | listing of 'src/util' |  |  | 0.613 |
| walker |  | 1141 | 51 | entry item body at src/main.rs:29 body 37 |  |  | 0.654 |
| walker |  | 1176 | 35 | listing of 'src/export' |  |  | 0.722 |
| walker |  | 1209 | 33 | pub-item names surface in src/export/mod.rs |  |  | 0.722 |
| walker |  | 1228 | 19 | pub item at src/export/mod.rs:56 |  |  | 0.722 |
| walker |  | 1266 | 38 | pub item at src/export/mod.rs:67 |  |  | 0.723 |
| walker |  | 1279 | 13 | pub item at src/util/randomized_environment_offset.rs:6 |  |  | 0.723 |
| walker |  | 1292 | 13 | pub-item doc lede at src/export/mod.rs:67 |  |  | 0.723 |
| walker |  | 1363 | 71 | entry item body at src/main.rs:53 body 54 |  |  | 0.771 |
| walker |  | 1380 | 17 | pub item at src/export/csv.rs:13 |  |  | 0.771 |
| walker |  | 1397 | 17 | pub item at src/export/json.rs:17 |  |  | 0.771 |
| walker |  | 1414 | 17 | pub item at src/export/markdown.rs:6 |  |  | 0.771 |
| ns | 1426 |  | 368 | Options struct field list — first half | 2.4 |  | 0.681 |
| walker |  | 1520 | 106 | pub item at src/export/mod.rs:28 |  |  | 0.684 |
| walker |  | 1605 | 85 | pub item at src/timer/mod.rs:42 |  |  | 0.684 |
| walker |  | 1631 | 26 | pub-item names surface in src/error.rs |  |  | 0.684 |
| walker |  | 1648 | 17 | pub-item doc lede at src/export/mod.rs:28 |  |  | 0.684 |
| walker |  | 1666 | 18 | pub item at src/export/orgmode.rs:5 |  |  | 0.638 |
| ns | 1666 |  | 240 | Options struct field list — second half | 2.5 | 2.4 | 0.638 |
| walker |  | 1691 | 25 | mod/use plumbing in src/output/mod.rs |  |  | 0.638 |
| walker |  | 1732 | 41 | impl method sigs in src/parameter/mod.rs |  |  | 0.638 |
| walker |  | 1747 | 15 | pub-item doc lede at src/timer/mod.rs:42 |  |  | 0.638 |
| walker |  | 1767 | 20 | pub item at src/export/asciidoc.rs:5 |  |  | 0.638 |
| walker |  | 1782 | 15 | pub-item doc lede at src/timer/mod.rs:83 |  |  | 0.638 |
| ns | 1798 |  | 132 | Options sibling enum/struct headers | 2.6 |  | 0.614 |
| walker |  | 1801 | 19 | pub item at src/parameter/tokenize.rs:1 |  |  | 0.614 |
| ns | 1825 |  | 27 | Options::from_cli_arguments + validate_against_command_list signatures | 2.7 |  | 0.610 |
| walker |  | 1833 | 32 | pub-item names surface in src/command.rs |  |  | 0.610 |
| walker |  | 1833 | 0 | pub item at src/command.rs:134 |  |  | 0.610 |
| walker |  | 1846 | 13 | pub-item doc lede at src/benchmark/mod.rs:32 |  |  | 0.610 |
| walker |  | 1870 | 24 | pub item at src/timer/wall_clock_timer.rs:5 |  |  | 0.610 |
| walker |  | 1895 | 25 | pub item at src/timer/windows_timer.rs:49 |  |  | 0.610 |
| walker |  | 1917 | 22 | pub-item names surface in src/export/markup.rs |  |  | 0.610 |
| walker |  | 1934 | 17 | pub item at src/export/markup.rs:10 |  |  | 0.610 |
| walker |  | 1957 | 23 | pub-item names surface in src/parameter/range_step.rs |  |  | 0.610 |
| walker |  | 1980 | 23 | pub-item names surface in src/timer/unix_timer.rs |  |  | 0.610 |
| ns | 1984 |  | 159 | Command + Commands type headers + public method names | 2.8 |  | 0.588 |
| walker |  | 1995 | 15 | pub item at src/timer/unix_timer.rs:18 |  |  | 0.588 |
| walker |  | 2019 | 24 | pub-item names surface in src/output/warnings.rs |  |  | 0.588 |
| walker |  | 2047 | 28 | pub item at src/output/warnings.rs:7 |  |  | 0.588 |
| walker |  | 2088 | 41 | mod/use plumbing in src/parameter/mod.rs |  |  | 0.588 |
| walker |  | 2132 | 44 | mod/use plumbing in src/util/mod.rs |  |  | 0.588 |
| walker |  | 2163 | 31 | pub item at src/output/progress_bar.rs:13 |  |  | 0.588 |
| walker |  | 2201 | 38 | pub item at src/parameter/range_step.rs:34 |  |  | 0.588 |
| ns | 2254 |  | 270 | BenchmarkResult fields (the in-memory result row) | 2.9 |  | 0.557 |
| walker |  | 2366 | 165 | mod/use plumbing in src/main.rs |  |  | 0.607 |
| walker |  | 2416 | 50 | pub item at src/util/number.rs:10 |  |  | 0.607 |
| walker |  | 2468 | 52 | pub item at src/output/warnings.rs:13 |  |  | 0.607 |
| walker |  | 2536 | 68 | pub item at src/cli.rs:8 |  |  | 0.607 |
| walker |  | 2556 | 20 | pub item body at src/cli.rs:8 body 13 |  |  | 0.607 |
| ns | 2570 |  | 316 | ExportType + Exporter trait + ExportManager headers | 2.10 |  | 0.596 |
| walker |  | 2591 | 35 | pub-item names surface in src/util/units.rs |  |  | 0.596 |
| walker |  | 2591 | 0 | pub item at src/util/units.rs:6 |  |  | 0.596 |
| walker |  | 2634 | 43 | pub item at src/util/units.rs:10 |  |  | 0.596 |
| walker |  | 2643 | 9 | pub-item doc lede at src/util/units.rs:10 |  |  | 0.596 |
| walker |  | 2658 | 15 | pub-item doc lede at src/command.rs:134 |  |  | 0.596 |
| walker |  | 2696 | 38 | pub-item names surface in src/util/min_max.rs |  |  | 0.582 |
| walker |  | 2696 | 0 | pub item at src/util/min_max.rs:2 |  |  | 0.582 |
| walker |  | 2696 | 0 | pub item at src/util/min_max.rs:10 |  |  | 0.582 |
| ns | 2696 |  | 126 | Benchmark module-level helpers + MIN_EXECUTION_TIME | 2.11 |  | 0.582 |
| walker |  | 2715 | 19 | pub item body at src/util/randomized_environment_offset.rs:6 body 7 |  |  | 0.582 |
| walker |  | 2785 | 70 | pub-item names surface in src/outlier_detection.rs |  |  | 0.582 |
| walker |  | 2785 | 0 | pub item at src/outlier_detection.rs:13 |  |  | 0.582 |
| walker |  | 2785 | 0 | pub item at src/outlier_detection.rs:21 |  |  | 0.582 |
| walker |  | 2785 | 0 | pub item at src/outlier_detection.rs:43 |  |  | 0.582 |
| walker |  | 2829 | 44 | pub-item names surface in src/util/exit_code.rs |  |  | 0.582 |
| walker |  | 2829 | 0 | pub item at src/util/exit_code.rs:4 |  |  | 0.582 |
| walker |  | 2829 | 0 | pub item at src/util/exit_code.rs:20 |  |  | 0.582 |
| walker |  | 2835 | 6 | pub item body at src/util/exit_code.rs:20 body 21 |  |  | 0.582 |
| walker |  | 2847 | 12 | pub-item doc lede at src/output/warnings.rs:13 |  |  | 0.582 |
| ns | 2874 |  | 178 | relative_speed + outlier_detection signatures | 2.12 |  | 0.566 |
| walker |  | 2948 | 101 | pub item at src/command.rs:22 |  |  | 0.567 |
| ns | 2951 |  | 77 | tests/ + scripts/ fs listings | 2.13 |  | 0.553 |
| walker |  | 2961 | 13 | pub-item doc lede at src/command.rs:22 |  |  | 0.553 |
| walker |  | 3085 | 124 | impl method sigs in src/export/mod.rs |  |  | 0.566 |
| walker |  | 3150 | 65 | pub item at src/benchmark/scheduler.rs:13 |  |  | 0.566 |
| walker |  | 3217 | 67 | pub item at src/timer/unix_timer.rs:10 |  |  | 0.566 |
| walker |  | 3314 | 97 | pub item at src/parameter/range_step.rs:7 |  |  | 0.566 |
| ns | 3326 |  | 375 | CLI flag names — every Arg::new() line | 3.1 |  | 0.536 |
| walker |  | 3374 | 60 | pub-item names surface in src/benchmark/executor.rs |  |  | 0.543 |
| ns | 3375 |  | 49 | RunBounds default — '10 runs by default' | 3.2 |  | 0.539 |
| walker |  | 3388 | 14 | pub item at src/benchmark/executor.rs:122 |  |  | 0.539 |
| walker |  | 3409 | 21 | pub item at src/benchmark/executor.rs:303 |  |  | 0.541 |
| walker |  | 3440 | 31 | pub item at src/benchmark/executor.rs:19 |  |  | 0.546 |
| walker |  | 3480 | 40 | pub item at src/benchmark/executor.rs:169 |  |  | 0.546 |
| walker |  | 3493 | 13 | pub-item doc lede at src/output/progress_bar.rs:13 |  |  | 0.546 |
| walker |  | 3505 | 12 | pub-item doc lede at src/util/units.rs:6 |  |  | 0.546 |
| walker |  | 3581 | 76 | pub-item names surface in src/output/format.rs |  |  | 0.546 |
| walker |  | 3581 | 0 | pub item at src/output/format.rs:5 |  |  | 0.546 |
| walker |  | 3581 | 0 | pub item at src/output/format.rs:11 |  |  | 0.546 |
| walker |  | 3581 | 0 | pub item at src/output/format.rs:18 |  |  | 0.546 |
| walker |  | 3606 | 25 | pub item body at src/output/format.rs:5 body 6 |  |  | 0.546 |
| ns | 3685 |  | 310 | Scheduler::run_benchmarks body | 3.3 | 2.1 | 0.526 |
| walker |  | 3744 | 138 | pub-item names surface in src/options.rs |  |  | 0.548 |
| walker |  | 3779 | 35 | pub item at src/options.rs:102 |  |  | 0.548 |
| walker |  | 3820 | 41 | pub item at src/options.rs:185 |  |  | 0.548 |
| walker |  | 3879 | 59 | pub item at src/options.rs:24 |  |  | 0.548 |
| ns | 3893 |  | 208 | Benchmark::run — count-of-runs formula | 3.4 | 2.2 | 0.533 |
| walker |  | 3929 | 50 | pub item at src/options.rs:108 |  |  | 0.533 |
| walker |  | 3995 | 66 | pub item at src/options.rs:123 |  |  | 0.533 |
| walker |  | 4009 | 14 | pub-item doc lede at src/options.rs:24 |  |  | 0.533 |
| walker |  | 4093 | 84 | pub item at src/options.rs:71 |  |  | 0.533 |
| walker |  | 4107 | 14 | pub-item doc lede at src/options.rs:71 |  |  | 0.533 |
| walker |  | 4120 | 13 | pub-item doc lede at src/options.rs:108 |  |  | 0.533 |
| walker |  | 4233 | 113 | pub item at src/options.rs:149 |  |  | 0.533 |
| walker |  | 4248 | 15 | pub-item doc lede at src/options.rs:149 |  |  | 0.533 |
| ns | 4283 |  | 390 | Benchmark::run — warning push logic | 3.5 | 2.2 | 0.507 |
| walker |  | 4377 | 129 | pub item at src/options.rs:84 |  |  | 0.507 |
| walker |  | 4387 | 10 | pub-item doc lede at src/options.rs:84 |  |  | 0.507 |
| walker |  | 4403 | 16 | pub-item doc lede at src/util/min_max.rs:2 |  |  | 0.507 |
| walker |  | 4419 | 16 | pub-item doc lede at src/util/min_max.rs:10 |  |  | 0.507 |
| walker |  | 4459 | 40 | pub item body at src/output/format.rs:11 body 12 |  |  | 0.507 |
| walker |  | 4480 | 21 | listing of 'tests' |  |  | 0.510 |
| ns | 4505 |  | 222 | Benchmark::run — BenchmarkResult construction | 3.6 | 2.9 | 0.496 |
| walker |  | 4522 | 42 | pub item body at src/util/min_max.rs:2 body 3 |  |  | 0.496 |
| walker |  | 4564 | 42 | pub item body at src/util/min_max.rs:10 body 11 |  |  | 0.496 |
| walker |  | 4809 | 245 | pub item at src/error.rs:7 |  |  | 0.497 |
| walker |  | 4953 | 144 | README.md section #1 |  |  | 0.519 |
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
