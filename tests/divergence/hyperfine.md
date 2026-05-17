Score(3000)=0.486 I=0.770 C=0.307 ns_rows≤3K=23/42 (reached=6 partial=1 missing=16)

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
| walker |  | 389 | 25 | listing of 'doc' |  |  | 0.495 |
| ns | 394 |  | 78 | Remaining src/ subdir fs listings | 1.7 |  | 0.424 |
| walker |  | 461 | 72 | README.md section #0 |  |  | 0.450 |
| walker |  | 505 | 44 | listing of 'src' |  |  | 0.606 |
| walker |  | 515 | 10 | entry item at src/main.rs:53 |  |  | 0.606 |
| walker |  | 528 | 13 | listing of 'src/parameter' |  |  | 0.611 |
| walker |  | 545 | 17 | listing of 'src/output' |  |  | 0.635 |
| walker |  | 565 | 20 | listing of 'src/timer' |  |  | 0.677 |
| ns | 572 |  | 178 | main.rs run() — first half (CLI parsing + builders) | 1.8 |  | 0.607 |
| walker |  | 598 | 33 | pub-item names surface in src/parameter/mod.rs |  |  | 0.607 |
| walker |  | 617 | 19 | pub item at src/parameter/mod.rs:8 |  |  | 0.607 |
| walker |  | 651 | 34 | pub-item names surface in src/timer/mod.rs |  |  | 0.607 |
| walker |  | 651 | 0 | pub item at src/timer/mod.rs:83 |  |  | 0.607 |
| walker |  | 679 | 28 | listing of 'src/util' |  |  | 0.689 |
| walker |  | 714 | 35 | listing of 'src/export' |  |  | 0.727 |
| ns | 741 |  | 169 | main.rs run() — second half (Scheduler) + main() wrapper | 1.9 | 1.8 | 0.652 |
| walker |  | 747 | 33 | pub-item names surface in src/export/mod.rs |  |  | 0.652 |
| walker |  | 766 | 19 | pub item at src/export/mod.rs:56 |  |  | 0.652 |
| walker |  | 804 | 38 | pub item at src/export/mod.rs:67 |  |  | 0.653 |
| walker |  | 817 | 13 | pub item at src/util/randomized_environment_offset.rs:6 |  |  | 0.653 |
| walker |  | 890 | 73 | pub item at src/timer/mod.rs:42 |  |  | 0.653 |
| ns | 895 |  | 154 | README features list | 1.10 |  | 0.613 |
| ns | 950 |  | 55 | Scheduler struct header + public method names | 2.1 |  | 0.599 |
| ns | 979 |  | 29 | Benchmark struct header + Benchmark::run signature | 2.2 |  | 0.590 |
| walker |  | 988 | 98 | pub item at src/export/mod.rs:28 |  |  | 0.593 |
| walker |  | 1001 | 13 | pub-item doc lede at src/export/mod.rs:67 |  |  | 0.593 |
| ns | 1058 |  | 79 | Executor trait header + impl struct names | 2.3 |  | 0.572 |
| walker |  | 1072 | 71 | entry item body at src/main.rs:53 body 54 |  |  | 0.593 |
| walker |  | 1098 | 26 | pub-item names surface in src/error.rs |  |  | 0.593 |
| walker |  | 1115 | 17 | pub-item doc lede at src/export/mod.rs:28 |  |  | 0.593 |
| walker |  | 1140 | 25 | mod/use plumbing in src/output/mod.rs |  |  | 0.593 |
| walker |  | 1181 | 41 | impl method sigs in src/parameter/mod.rs |  |  | 0.593 |
| walker |  | 1196 | 15 | pub-item doc lede at src/timer/mod.rs:42 |  |  | 0.593 |
| walker |  | 1211 | 15 | pub-item doc lede at src/timer/mod.rs:83 |  |  | 0.593 |
| walker |  | 1230 | 19 | pub item at src/parameter/tokenize.rs:1 |  |  | 0.593 |
| walker |  | 1262 | 32 | pub-item names surface in src/command.rs |  |  | 0.593 |
| walker |  | 1262 | 0 | pub item at src/command.rs:134 |  |  | 0.593 |
| walker |  | 1286 | 24 | pub item at src/timer/wall_clock_timer.rs:5 |  |  | 0.593 |
| walker |  | 1317 | 31 | pub item at src/util/number.rs:10 |  |  | 0.593 |
| walker |  | 1342 | 25 | pub item at src/timer/windows_timer.rs:49 |  |  | 0.593 |
| walker |  | 1364 | 22 | pub-item names surface in src/export/markup.rs |  |  | 0.593 |
| walker |  | 1381 | 17 | pub item at src/export/markup.rs:10 |  |  | 0.593 |
| walker |  | 1404 | 23 | pub-item names surface in src/parameter/range_step.rs |  |  | 0.593 |
| ns | 1426 |  | 368 | Options struct field list — first half | 2.4 |  | 0.523 |
| walker |  | 1427 | 23 | pub-item names surface in src/timer/unix_timer.rs |  |  | 0.523 |
| walker |  | 1442 | 15 | pub item at src/timer/unix_timer.rs:18 |  |  | 0.523 |
| walker |  | 1466 | 24 | pub-item names surface in src/output/warnings.rs |  |  | 0.523 |
| walker |  | 1494 | 28 | pub item at src/output/warnings.rs:7 |  |  | 0.524 |
| walker |  | 1524 | 30 | pub item at src/parameter/range_step.rs:34 |  |  | 0.524 |
| walker |  | 1565 | 41 | mod/use plumbing in src/parameter/mod.rs |  |  | 0.524 |
| walker |  | 1609 | 44 | mod/use plumbing in src/util/mod.rs |  |  | 0.524 |
| walker |  | 1640 | 31 | pub item at src/output/progress_bar.rs:13 |  |  | 0.524 |
| ns | 1666 |  | 240 | Options struct field list — second half | 2.5 | 2.4 | 0.489 |
| ns | 1798 |  | 132 | Options sibling enum/struct headers | 2.6 |  | 0.470 |
| walker |  | 1805 | 165 | mod/use plumbing in src/main.rs |  |  | 0.528 |
| ns | 1825 |  | 27 | Options::from_cli_arguments + validate_against_command_list signatures | 2.7 |  | 0.525 |
| walker |  | 1857 | 52 | pub item at src/output/warnings.rs:13 |  |  | 0.526 |
| walker |  | 1925 | 68 | pub item at src/cli.rs:8 |  |  | 0.526 |
| walker |  | 1945 | 20 | pub item body at src/cli.rs:8 body 13 |  |  | 0.526 |
| walker |  | 1980 | 35 | pub-item names surface in src/util/units.rs |  |  | 0.526 |
| walker |  | 1980 | 0 | pub item at src/util/units.rs:6 |  |  | 0.526 |
| ns | 1984 |  | 159 | Command + Commands type headers + public method names | 2.8 |  | 0.507 |
| walker |  | 2006 | 26 | pub item at src/util/units.rs:10 |  |  | 0.507 |
| walker |  | 2015 | 9 | pub-item doc lede at src/util/units.rs:10 |  |  | 0.507 |
| walker |  | 2030 | 15 | pub-item doc lede at src/command.rs:134 |  |  | 0.507 |
| walker |  | 2068 | 38 | pub-item names surface in src/util/min_max.rs |  |  | 0.507 |
| walker |  | 2068 | 0 | pub item at src/util/min_max.rs:2 |  |  | 0.507 |
| walker |  | 2068 | 0 | pub item at src/util/min_max.rs:10 |  |  | 0.507 |
| walker |  | 2154 | 86 | pub item at src/command.rs:22 |  |  | 0.507 |
| walker |  | 2167 | 13 | pub-item doc lede at src/command.rs:22 |  |  | 0.507 |
| walker |  | 2186 | 19 | pub item body at src/util/randomized_environment_offset.rs:6 body 7 |  |  | 0.507 |
| ns | 2254 |  | 270 | BenchmarkResult fields (the in-memory result row) | 2.9 |  | 0.480 |
| walker |  | 2256 | 70 | pub-item names surface in src/outlier_detection.rs |  |  | 0.480 |
| walker |  | 2256 | 0 | pub item at src/outlier_detection.rs:13 |  |  | 0.480 |
| walker |  | 2256 | 0 | pub item at src/outlier_detection.rs:21 |  |  | 0.480 |
| walker |  | 2256 | 0 | pub item at src/outlier_detection.rs:43 |  |  | 0.480 |
| walker |  | 2300 | 44 | pub-item names surface in src/util/exit_code.rs |  |  | 0.480 |
| walker |  | 2300 | 0 | pub item at src/util/exit_code.rs:4 |  |  | 0.480 |
| walker |  | 2300 | 0 | pub item at src/util/exit_code.rs:20 |  |  | 0.480 |
| walker |  | 2306 | 6 | pub item body at src/util/exit_code.rs:20 body 21 |  |  | 0.480 |
| walker |  | 2361 | 55 | pub item at src/timer/unix_timer.rs:10 |  |  | 0.480 |
| walker |  | 2373 | 12 | pub-item doc lede at src/output/warnings.rs:13 |  |  | 0.481 |
| walker |  | 2497 | 124 | impl method sigs in src/export/mod.rs |  |  | 0.481 |
| walker |  | 2529 | 32 | README.md section #4 |  |  | 0.481 |
| ns | 2570 |  | 316 | ExportType + Exporter trait + ExportManager headers | 2.10 |  | 0.495 |
| walker |  | 2626 | 97 | pub item at src/parameter/range_step.rs:7 |  |  | 0.495 |
| walker |  | 2639 | 13 | pub-item doc lede at src/output/progress_bar.rs:13 |  |  | 0.495 |
| walker |  | 2651 | 12 | pub-item doc lede at src/util/units.rs:6 |  |  | 0.495 |
| ns | 2696 |  | 126 | Benchmark module-level helpers + MIN_EXECUTION_TIME | 2.11 |  | 0.483 |
| walker |  | 2727 | 76 | pub-item names surface in src/output/format.rs |  |  | 0.483 |
| walker |  | 2727 | 0 | pub item at src/output/format.rs:5 |  |  | 0.483 |
| walker |  | 2727 | 0 | pub item at src/output/format.rs:11 |  |  | 0.483 |
| walker |  | 2727 | 0 | pub item at src/output/format.rs:18 |  |  | 0.483 |
| walker |  | 2752 | 25 | pub item body at src/output/format.rs:5 body 6 |  |  | 0.483 |
| ns | 2874 |  | 178 | relative_speed + outlier_detection signatures | 2.12 |  | 0.470 |
| walker |  | 2890 | 138 | pub-item names surface in src/options.rs |  |  | 0.498 |
| walker |  | 2908 | 18 | pub item at src/options.rs:102 |  |  | 0.498 |
| walker |  | 2938 | 30 | pub item at src/options.rs:185 |  |  | 0.498 |
| ns | 2951 |  | 77 | tests/ + scripts/ fs listings | 2.13 |  | 0.486 |
| walker |  | 2986 | 48 | pub item at src/options.rs:24 |  |  | 0.486 |
| walker |  | 3037 | 51 | pub item at src/options.rs:123 |  |  | 0.486 |
| walker |  | 3087 | 50 | pub item at src/options.rs:108 |  |  | 0.486 |
| walker |  | 3156 | 69 | pub item at src/options.rs:71 |  |  | 0.486 |
| walker |  | 3170 | 14 | pub-item doc lede at src/options.rs:24 |  |  | 0.486 |
| walker |  | 3184 | 14 | pub-item doc lede at src/options.rs:71 |  |  | 0.486 |
| walker |  | 3197 | 13 | pub-item doc lede at src/options.rs:108 |  |  | 0.486 |
| walker |  | 3293 | 96 | pub item at src/options.rs:149 |  |  | 0.486 |
| walker |  | 3308 | 15 | pub-item doc lede at src/options.rs:149 |  |  | 0.486 |
| ns | 3326 |  | 375 | CLI flag names — every Arg::new() line | 3.1 |  | 0.461 |
| ns | 3375 |  | 49 | RunBounds default — '10 runs by default' | 3.2 |  | 0.457 |
| walker |  | 3420 | 112 | pub item at src/options.rs:84 |  |  | 0.457 |
| walker |  | 3430 | 10 | pub-item doc lede at src/options.rs:84 |  |  | 0.457 |
| walker |  | 3446 | 16 | pub-item doc lede at src/util/min_max.rs:2 |  |  | 0.457 |
| walker |  | 3462 | 16 | pub-item doc lede at src/util/min_max.rs:10 |  |  | 0.457 |
| walker |  | 3502 | 40 | pub item body at src/output/format.rs:11 body 12 |  |  | 0.457 |
| walker |  | 3523 | 21 | listing of 'tests' |  |  | 0.460 |
| ns | 3685 |  | 310 | Scheduler::run_benchmarks body | 3.3 | 2.1 | 0.443 |
| walker |  | 3758 | 235 | pub item at src/error.rs:7 |  |  | 0.444 |
| walker |  | 3800 | 42 | pub item body at src/util/min_max.rs:2 body 3 |  |  | 0.444 |
| walker |  | 3842 | 42 | pub item body at src/util/min_max.rs:10 body 11 |  |  | 0.444 |
| walker |  | 3860 | 18 | pub-item doc lede at src/output/format.rs:11 |  |  | 0.444 |
| walker |  | 3878 | 18 | pub-item doc lede at src/output/format.rs:18 |  |  | 0.444 |
| ns | 3893 |  | 208 | Benchmark::run — count-of-runs formula | 3.4 | 2.2 | 0.431 |
| walker |  | 3958 | 80 | pub item body at src/outlier_detection.rs:43 body 44 |  |  | 0.431 |
| walker |  | 4109 | 151 | mod/use plumbing in src/timer/mod.rs |  |  | 0.431 |
| walker |  | 4156 | 47 | README.md section #7 |  |  | 0.431 |
| walker |  | 4200 | 44 | pub-item doc lede at src/outlier_detection.rs:43 |  |  | 0.431 |
| ns | 4283 |  | 390 | Benchmark::run — warning push logic | 3.5 | 2.2 | 0.411 |
| walker |  | 4406 | 206 | mod/use plumbing in src/export/mod.rs |  |  | 0.411 |
| ns | 4505 |  | 222 | Benchmark::run — BenchmarkResult construction | 3.6 | 2.9 | 0.399 |
| walker |  | 4769 | 363 | pub item at src/error.rs:37 |  |  | 0.400 |
| walker |  | 4796 | 27 | listing of 'src/benchmark' |  |  | 0.427 |
| walker |  | 4862 | 66 | README.md section #8 |  |  | 0.427 |
| walker |  | 4939 | 77 | README.md section #6 |  |  | 0.427 |
| ns | 5047 |  | 542 | Per-format markup exporters — Markdown / Orgmode | 3.7 | 2.10 | 0.401 |
| walker |  | 5083 | 144 | README.md section #1 |  |  | 0.423 |
| walker |  | 5347 | 264 | [dependencies] in Cargo.toml |  |  | 0.423 |
| walker |  | 5410 | 63 | pub-item doc lede at src/outlier_detection.rs:13 |  |  | 0.423 |
| ns | 5447 |  | 400 | AsciidocExporter body | 3.8 | 2.10 | 0.406 |
| walker |  | 5599 | 189 | pub item body at src/outlier_detection.rs:21 body 22 |  |  | 0.407 |
| walker |  | 5655 | 56 | listing of 'scripts' |  |  | 0.428 |
| ns | 5665 |  | 218 | JsonExporter body | 3.9 | 2.10 | 0.418 |
| walker |  | 6158 | 503 | pub item at src/options.rs:198 |  |  | 0.480 |
| walker |  | 6173 | 15 | pub-item doc lede at src/options.rs:198 |  |  | 0.484 |
| ns | 6202 |  | 537 | CsvExporter body | 3.10 | 2.10 | 0.461 |
| walker |  | 6295 | 122 | pub item body at src/output/format.rs:18 body 19 |  |  | 0.461 |
| walker |  | 6346 | 51 | pub-item doc lede at src/output/format.rs:5 |  |  | 0.461 |
| walker |  | 6452 | 106 | pub-item doc lede at src/outlier_detection.rs:21 |  |  | 0.461 |
| walker |  | 6632 | 180 | pub item body at src/util/exit_code.rs:4 body 5 |  |  | 0.461 |
| ns | 6887 |  | 685 | ShellExecutor::calibrate — shell spawning time measurement | 3.11 | 2.3 | 0.434 |
| ns | 7425 |  | 538 | ShellExecutor::run_command_and_measure body | 3.12 | 3.11 | 0.419 |
| walker |  | 7478 | 846 | pub item at src/export/markup.rs:15 |  |  | 0.419 |
| walker |  | 7587 | 109 | pub-item doc lede at src/util/randomized_environment_offset.rs:6 |  |  | 0.419 |
| walker |  | 7781 | 194 | README.md section #5 |  |  | 0.419 |
| walker |  | 7815 | 34 | pub-item names surface in src/benchmark/mod.rs |  |  | 0.420 |
| walker |  | 7815 | 0 | pub item at src/benchmark/mod.rs:32 |  |  | 0.420 |
| walker |  | 7823 | 8 | listing of '.github' |  |  | 0.420 |
| walker |  | 7828 | 5 | listing of '.github/workflows' |  |  | 0.420 |
| walker |  | 7876 | 48 | pub item at src/benchmark/mod.rs:34 |  |  | 0.420 |
| walker |  | 7894 | 18 | pub item at tests/integration_tests.rs:12 |  |  | 0.420 |
| walker |  | 7907 | 13 | pub-item doc lede at src/benchmark/mod.rs:32 |  |  | 0.420 |
| walker |  | 7940 | 33 | pub-item names surface in tests/common.rs |  |  | 0.420 |
| walker |  | 7940 | 0 | pub item at tests/common.rs:5 |  |  | 0.420 |
| walker |  | 7940 | 0 | pub item at tests/common.rs:11 |  |  | 0.420 |
| walker |  | 7956 | 16 | pub item body at tests/common.rs:11 body 12 |  |  | 0.420 |
| walker |  | 7963 | 7 | pub item body at src/parameter/tokenize.rs:1 body 30 |  |  | 0.420 |
| walker |  | 7971 | 8 | pub item body at src/output/progress_bar.rs:13 body 30 |  |  | 0.420 |
| ns | 7997 |  | 572 | MockExecutor body (debug-mode) | 3.13 | 2.3 | 0.402 |
| walker |  | 8062 | 91 | README headline in scripts/README.md |  |  | 0.402 |
| walker |  | 8071 | 9 | pub item body at src/parameter/tokenize.rs:1 body 28 |  |  | 0.402 |
| walker |  | 8099 | 28 | pub item body at tests/integration_tests.rs:12 body 13 |  |  | 0.402 |
| walker |  | 8109 | 10 | pub item body at src/parameter/tokenize.rs:1 body 2 |  |  | 0.402 |
| walker |  | 8144 | 35 | pub item body at tests/common.rs:5 body 6 |  |  | 0.402 |
| walker |  | 8157 | 13 | pub item body at src/output/progress_bar.rs:13 body 26 |  |  | 0.402 |
| walker |  | 8170 | 13 | pub item body at src/output/progress_bar.rs:13 body 28 |  |  | 0.402 |
| walker |  | 8183 | 13 | pub item body at src/parameter/tokenize.rs:1 body 3 |  |  | 0.402 |
| walker |  | 8196 | 13 | pub item body at src/parameter/tokenize.rs:1 body 5 |  |  | 0.402 |
| walker |  | 8261 | 65 | pub item at src/benchmark/scheduler.rs:13 |  |  | 0.403 |
| ns | 8406 |  | 409 | outlier_detection module + OUTLIER_THRESHOLD const | 3.14 | 2.12 | 0.407 |
| ns | 8958 |  | 552 | format_duration auto-unit selection + Unit::short_name/format | 3.15 |  | 0.407 |
| ns | 9144 |  | 186 | Warnings enum + OutlierWarningOptions struct | 3.16 |  | 0.410 |
| ns | 9531 |  | 387 | OptionsError enum + variants | 4.1 |  | 0.425 |
| walker |  | 9717 | 1456 | README.md section #3 |  |  | 0.425 |
| walker |  | 9777 | 60 | pub-item names surface in src/benchmark/executor.rs |  |  | 0.428 |
| ns | 9788 |  | 257 | ParameterScanError enum + variants | 4.2 |  | 0.439 |
| walker |  | 9790 | 13 | pub item at src/benchmark/executor.rs:303 |  |  | 0.439 |
| walker |  | 9804 | 14 | pub item at src/benchmark/executor.rs:122 |  |  | 0.439 |
| walker |  | 9835 | 31 | pub item at src/benchmark/executor.rs:19 |  |  | 0.441 |
| walker |  | 9875 | 40 | pub item at src/benchmark/executor.rs:169 |  |  | 0.441 |
| walker |  | 9884 | 9 | pub item body at src/timer/mod.rs:83 body 84 |  |  | 0.441 |
| ns | 9922 |  | 134 | common.rs test helpers | 4.3 |  | 0.442 |
