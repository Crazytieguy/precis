Score(3000)=0.696 I=0.871 C=0.556 ns_rows≤3K=15/41 (reached=8 partial=1 missing=6)

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 50 | 50 | listing of '.' |  |  | 1.000 |
| ns | 50 |  | 50 | Root directory listing | 1.1 |  | 1.000 |
| walker |  | 74 | 24 | listing of 'doc' |  |  | 1.000 |
| ns | 99 |  | 49 | src/ directory listing | 1.2 |  | 0.676 |
| walker |  | 123 | 49 | listing of 'src' |  |  | 1.000 |
| walker |  | 135 | 12 | listing of 'src/parameter' |  |  | 1.000 |
| walker |  | 151 | 16 | listing of 'src/output' |  |  | 1.000 |
| ns | 159 |  | 60 | src/benchmark, src/export directory listings | 1.3 |  | 0.773 |
| walker |  | 170 | 19 | listing of 'src/timer' |  |  | 0.782 |
| walker |  | 182 | 12 | entry item at src/main.rs:53 |  |  | 0.782 |
| walker |  | 195 | 13 | entry item at src/main.rs:29 |  |  | 0.782 |
| walker |  | 204 | 9 | entry item body at src/main.rs:29 body 50 |  |  | 0.782 |
| walker |  | 214 | 10 | entry item body at src/main.rs:29 body 48 |  |  | 0.782 |
| walker |  | 226 | 12 | entry item body at src/main.rs:29 body 31 |  |  | 0.783 |
| ns | 233 |  | 74 | src/output, src/parameter, src/timer, src/util directory listings | 1.4 |  | 0.711 |
| walker |  | 238 | 12 | entry item body at src/main.rs:29 body 30 |  |  | 0.712 |
| walker |  | 250 | 12 | entry item body at src/main.rs:29 body 47 |  |  | 0.712 |
| walker |  | 262 | 12 | entry item body at src/main.rs:29 body 46 |  |  | 0.712 |
| walker |  | 278 | 16 | entry item body at src/main.rs:29 body 32 |  |  | 0.713 |
| walker |  | 294 | 16 | entry item body at src/main.rs:29 body 43 |  |  | 0.713 |
| walker |  | 311 | 17 | entry item body at src/main.rs:29 body 34 |  |  | 0.714 |
| walker |  | 329 | 18 | entry item body at src/main.rs:29 body 35 |  |  | 0.715 |
| ns | 340 |  | 107 | tests/, scripts/, doc/, .github/ directory listings | 1.5 |  | 0.607 |
| walker |  | 346 | 17 | entry item body at src/main.rs:29 body 36 |  |  | 0.608 |
| walker |  | 366 | 20 | entry item body at src/main.rs:29 body 45 |  |  | 0.608 |
| walker |  | 392 | 26 | listing of 'src/benchmark' |  |  | 0.638 |
| walker |  | 419 | 27 | listing of 'src/util' |  |  | 0.734 |
| walker |  | 454 | 35 | pub-item names surface in src/parameter/mod.rs |  |  | 0.734 |
| walker |  | 490 | 36 | pub item at src/parameter/mod.rs:8 |  |  | 0.734 |
| ns | 513 |  | 173 | README lede + feature list | 1.6 |  | 0.669 |
| walker |  | 526 | 36 | pub-item names surface in src/timer/mod.rs |  |  | 0.669 |
| walker |  | 526 | 0 | pub item at src/timer/mod.rs:83 |  |  | 0.669 |
| walker |  | 565 | 39 | pub-item names surface in src/benchmark/mod.rs |  |  | 0.669 |
| walker |  | 565 | 0 | pub item at src/benchmark/mod.rs:32 |  |  | 0.669 |
| walker |  | 599 | 34 | listing of 'src/export' |  |  | 0.774 |
| walker |  | 634 | 35 | pub-item names surface in src/export/mod.rs |  |  | 0.774 |
| walker |  | 655 | 21 | pub item at src/export/mod.rs:56 |  |  | 0.774 |
| ns | 665 |  | 152 | main.rs part 1 — module declarations + imports | 1.7 |  | 0.698 |
| walker |  | 695 | 40 | pub item at src/export/mod.rs:67 |  |  | 0.698 |
| walker |  | 745 | 50 | pub item at src/benchmark/mod.rs:34 |  |  | 0.698 |
| walker |  | 794 | 49 | entry item body at src/main.rs:29 body 37 |  |  | 0.702 |
| ns | 825 |  | 160 | main.rs part 2 — run() setup | 1.8 |  | 0.658 |
| walker |  | 833 | 39 | [features] in Cargo.toml |  |  | 0.658 |
| walker |  | 841 | 8 | listing of '.github' |  |  | 0.666 |
| walker |  | 845 | 4 | listing of '.github/workflows' |  |  | 0.666 |
| walker |  | 918 | 73 | entry item body at src/main.rs:53 body 54 |  |  | 0.671 |
| walker |  | 931 | 13 | pub-item doc lede at src/export/mod.rs:67 |  |  | 0.671 |
| walker |  | 1018 | 87 | pub item at src/timer/mod.rs:42 |  |  | 0.671 |
| walker |  | 1057 | 39 | impl method sigs in src/parameter/mod.rs |  |  | 0.671 |
| walker |  | 1057 | 0 | impl method at src/parameter/mod.rs:14 |  |  | 0.671 |
| walker |  | 1082 | 25 | mod/use plumbing in src/output/mod.rs |  |  | 0.671 |
| ns | 1097 |  | 272 | main.rs part 3 — scheduler dispatch + error printing | 1.9 |  | 0.690 |
| walker |  | 1198 | 116 | README headline in README.md |  |  | 0.690 |
| walker |  | 1213 | 15 | pub item at src/util/randomized_environment_offset.rs:6 |  |  | 0.690 |
| walker |  | 1241 | 28 | pub-item names surface in src/error.rs |  |  | 0.690 |
| walker |  | 1256 | 15 | pub-item doc lede at src/timer/mod.rs:42 |  |  | 0.690 |
| ns | 1309 |  | 212 | Cargo.toml part 1 — package metadata + feature flag | 1.10 |  | 0.651 |
| walker |  | 1384 | 128 | pub item at src/export/mod.rs:28 |  |  | 0.651 |
| walker |  | 1401 | 17 | pub-item doc lede at src/export/mod.rs:28 |  |  | 0.652 |
| walker |  | 1569 | 168 | [package] in Cargo.toml |  |  | 0.715 |
| walker |  | 1584 | 15 | pub-item doc lede at src/timer/mod.rs:83 |  |  | 0.715 |
| walker |  | 1597 | 13 | pub-item doc lede at src/benchmark/mod.rs:32 |  |  | 0.715 |
| walker |  | 1631 | 34 | pub-item names surface in src/command.rs |  |  | 0.715 |
| walker |  | 1631 | 0 | pub item at src/command.rs:134 |  |  | 0.715 |
| walker |  | 1652 | 21 | pub item at src/export/csv.rs:13 |  |  | 0.715 |
| walker |  | 1673 | 21 | pub item at src/export/json.rs:17 |  |  | 0.715 |
| ns | 1678 |  | 369 | Cargo.toml part 2 — ordinary + platform-conditional dependencies | 1.11 |  | 0.650 |
| walker |  | 1694 | 21 | pub item at src/export/markdown.rs:6 |  |  | 0.650 |
| walker |  | 1713 | 19 | pub item at src/parameter/tokenize.rs:1 |  |  | 0.650 |
| walker |  | 1735 | 22 | pub item at src/export/orgmode.rs:5 |  |  | 0.650 |
| ns | 1777 |  | 99 | Cargo.toml part 2b — clap dependency | 1.12 | 1.11 | 0.628 |
| walker |  | 1886 | 151 | README.md section #1 |  |  | 0.671 |
| walker |  | 1910 | 24 | pub item at src/export/asciidoc.rs:5 |  |  | 0.671 |
| ns | 1981 |  | 204 | Cargo.toml part 3 — dev/build deps, release profile | 1.13 |  | 0.637 |
| walker |  | 1990 | 80 | README.md section #0 |  |  | 0.643 |
| walker |  | 2018 | 28 | pub item at src/timer/wall_clock_timer.rs:5 |  |  | 0.643 |
| walker |  | 2042 | 24 | pub-item names surface in src/export/markup.rs |  |  | 0.643 |
| walker |  | 2059 | 17 | pub item at src/export/markup.rs:10 |  |  | 0.643 |
| walker |  | 2103 | 44 | mod/use plumbing in src/parameter/mod.rs |  |  | 0.643 |
| walker |  | 2147 | 44 | mod/use plumbing in src/util/mod.rs |  |  | 0.643 |
| walker |  | 2167 | 20 | listing of 'tests' |  |  | 0.657 |
| walker |  | 2196 | 29 | pub item at src/timer/windows_timer.rs:49 |  |  | 0.657 |
| walker |  | 2221 | 25 | pub-item names surface in src/parameter/range_step.rs |  |  | 0.657 |
| walker |  | 2246 | 25 | pub-item names surface in src/timer/unix_timer.rs |  |  | 0.657 |
| walker |  | 2263 | 17 | pub item at src/timer/unix_timer.rs:18 |  |  | 0.657 |
| walker |  | 2289 | 26 | pub-item names surface in src/output/warnings.rs |  |  | 0.657 |
| walker |  | 2319 | 30 | pub item at src/output/warnings.rs:7 |  |  | 0.657 |
| ns | 2420 |  | 439 | cli.rs — every flag's Arg::new(...) anchor line | 2.1 |  | 0.610 |
| walker |  | 2499 | 180 | mod/use plumbing in src/main.rs |  |  | 0.669 |
| ns | 2866 |  | 446 | cli.rs — build_command() header + positional `command` arg | 2.2 | 2.1 | 0.620 |
| walker |  | 2951 | 452 | registration roster at src/cli.rs:18 |  |  | 0.696 |
| walker |  | 2984 | 33 | pub item at src/output/progress_bar.rs:13 |  |  | 0.696 |
| walker |  | 3052 | 68 | pub item at src/cli.rs:8 |  |  | 0.698 |
| walker |  | 3074 | 22 | pub item body at src/cli.rs:8 body 13 |  |  | 0.700 |
| walker |  | 3114 | 40 | pub item at src/parameter/range_step.rs:34 |  |  | 0.700 |
| walker |  | 3168 | 54 | pub item at src/output/warnings.rs:13 |  |  | 0.700 |
| ns | 3178 |  | 312 | cli.rs — --shell/-S and -N/--shell=none | 2.3 | 2.1 | 0.670 |
| walker |  | 3205 | 37 | pub-item names surface in src/util/units.rs |  |  | 0.670 |
| walker |  | 3205 | 0 | pub item at src/util/units.rs:6 |  |  | 0.670 |
| walker |  | 3250 | 45 | pub item at src/util/units.rs:10 |  |  | 0.670 |
| walker |  | 3257 | 7 | pub-item doc lede at src/util/units.rs:10 |  |  | 0.670 |
| walker |  | 3329 | 72 | pub-item names surface in src/outlier_detection.rs |  |  | 0.670 |
| walker |  | 3329 | 0 | pub item at src/outlier_detection.rs:13 |  |  | 0.670 |
| walker |  | 3329 | 0 | pub item at src/outlier_detection.rs:21 |  |  | 0.670 |
| walker |  | 3337 | 8 | pub item at src/outlier_detection.rs:43 |  |  | 0.670 |
| ns | 3349 |  | 171 | cli.rs — --ignore-failure/-i | 2.4 | 2.1 | 0.655 |
| walker |  | 3393 | 56 | pub item at src/util/number.rs:10 |  |  | 0.655 |
| walker |  | 3408 | 15 | pub-item doc lede at src/command.rs:134 |  |  | 0.655 |
| walker |  | 3448 | 40 | pub-item names surface in src/util/min_max.rs |  |  | 0.655 |
| walker |  | 3448 | 0 | pub item at src/util/min_max.rs:2 |  |  | 0.655 |
| walker |  | 3448 | 0 | pub item at src/util/min_max.rs:10 |  |  | 0.655 |
| walker |  | 3572 | 124 | impl method sigs in src/export/mod.rs |  |  | 0.655 |
| walker |  | 3572 | 0 | impl method at src/export/mod.rs:76 |  |  | 0.655 |
| walker |  | 3572 | 0 | impl method at src/export/mod.rs:103 |  |  | 0.655 |
| walker |  | 3572 | 0 | impl method at src/export/mod.rs:132 |  |  | 0.655 |
| walker |  | 3582 | 10 | pub-item doc lede at src/output/warnings.rs:13 |  |  | 0.655 |
| ns | 3608 |  | 259 | options.rs — Options struct field names (locations) | 3.1 |  | 0.637 |
| walker |  | 3653 | 71 | manifest config in Cargo.toml |  |  | 0.642 |
| walker |  | 3699 | 46 | pub-item names surface in src/util/exit_code.rs |  |  | 0.642 |
| walker |  | 3708 | 9 | pub item at src/util/exit_code.rs:4 |  |  | 0.642 |
| walker |  | 3718 | 10 | pub item at src/util/exit_code.rs:20 |  |  | 0.642 |
| walker |  | 3726 | 8 | pub item body at src/util/exit_code.rs:20 body 21 |  |  | 0.642 |
| walker |  | 3747 | 21 | pub item body at src/util/randomized_environment_offset.rs:6 body 7 |  |  | 0.642 |
| walker |  | 3756 | 9 | impl method body at src/export/mod.rs:132 body 153 |  |  | 0.642 |
| walker |  | 3869 | 113 | pub item at src/command.rs:22 |  |  | 0.642 |
| walker |  | 3882 | 13 | pub-item doc lede at src/command.rs:22 |  |  | 0.642 |
| walker |  | 3963 | 81 | man-page NAME + DESCRIPTION in doc/hyperfine.1 |  |  | 0.642 |
| ns | 3966 |  | 358 | options.rs — --runs/--min-runs/--max-runs resolution | 3.2 |  | 0.615 |
| walker |  | 4211 | 248 | [dependencies] in Cargo.toml |  |  | 0.647 |
| walker |  | 4250 | 39 | README.md section #9 |  |  | 0.647 |
| ns | 4299 |  | 333 | options.rs — output style auto-detection | 3.3 |  | 0.623 |
| walker |  | 4319 | 69 | pub item at src/benchmark/scheduler.rs:13 |  |  | 0.623 |
| walker |  | 4418 | 99 | pub item at src/parameter/range_step.rs:7 |  |  | 0.623 |
| walker |  | 4490 | 72 | pub item at src/timer/unix_timer.rs:10 |  |  | 0.623 |
| walker |  | 4552 | 62 | pub-item names surface in src/benchmark/executor.rs |  |  | 0.623 |
| walker |  | 4568 | 16 | pub item at src/benchmark/executor.rs:122 |  |  | 0.623 |
| ns | 4582 |  | 283 | options.rs — validate_against_command_list: --prepare/--conclude counts | 3.4 |  | 0.606 |
| walker |  | 4591 | 23 | pub item at src/benchmark/executor.rs:303 |  |  | 0.606 |
| walker |  | 4624 | 33 | pub item at src/benchmark/executor.rs:19 |  |  | 0.606 |
| walker |  | 4666 | 42 | pub item at src/benchmark/executor.rs:169 |  |  | 0.606 |
| walker |  | 4676 | 10 | pub-item doc lede at src/util/units.rs:6 |  |  | 0.606 |
| walker |  | 4731 | 55 | listing of 'scripts' |  |  | 0.640 |
| ns | 4737 |  | 155 | options.rs — validate_against_command_list: --output count + normalization | 3.5 | 3.4 | 0.629 |
| walker |  | 4871 | 140 | pub-item names surface in src/options.rs |  |  | 0.629 |
| walker |  | 4878 | 7 | pub item at src/options.rs:20 |  |  | 0.629 |
| walker |  | 4888 | 10 | pub item at src/options.rs:17 |  |  | 0.629 |
| walker |  | 4925 | 37 | pub item at src/options.rs:102 |  |  | 0.629 |
| walker |  | 4968 | 43 | pub item at src/options.rs:185 |  |  | 0.629 |
| walker |  | 5034 | 66 | pub item at src/options.rs:24 |  |  | 0.629 |
| ns | 5086 |  | 349 | command.rs — replace_parameters_in (the {param} substitution algorithm) | 4.1 |  | 0.608 |
| walker |  | 5091 | 57 | pub item at src/options.rs:108 |  |  | 0.608 |
| walker |  | 5164 | 73 | pub item at src/options.rs:123 |  |  | 0.608 |
| walker |  | 5260 | 96 | pub item at src/options.rs:71 |  |  | 0.608 |
| ns | 5373 |  | 287 | parameter/tokenize.rs — tokenize() function body | 4.2 |  | 0.586 |
| walker |  | 5390 | 130 | pub item at src/options.rs:149 |  |  | 0.586 |
| walker |  | 5539 | 149 | pub item at src/options.rs:84 |  |  | 0.586 |
| ns | 5585 |  | 212 | benchmark/mod.rs — run-count determination formula | 5.1 |  | 0.574 |
| walker |  | 5617 | 78 | pub-item names surface in src/output/format.rs |  |  | 0.574 |
| walker |  | 5617 | 0 | pub item at src/output/format.rs:5 |  |  | 0.574 |
| walker |  | 5617 | 0 | pub item at src/output/format.rs:11 |  |  | 0.574 |
| walker |  | 5617 | 0 | pub item at src/output/format.rs:18 |  |  | 0.574 |
| walker |  | 5864 | 247 | pub item at src/error.rs:7 |  |  | 0.574 |
| ns | 5924 |  | 339 | benchmark/scheduler.rs — run_benchmarks (executor selection + run order) | 5.2 |  | 0.558 |
| walker |  | 5968 | 104 | pub-item names surface in src/benchmark/relative_speed.rs |  |  | 0.558 |
| walker |  | 5968 | 0 | pub item at src/benchmark/relative_speed.rs:16 |  |  | 0.558 |
| walker |  | 5968 | 0 | pub item at src/benchmark/relative_speed.rs:20 |  |  | 0.558 |
| walker |  | 6007 | 39 | pub item at src/benchmark/relative_speed.rs:112 |  |  | 0.558 |
| walker |  | 6047 | 40 | pub item at src/benchmark/relative_speed.rs:98 |  |  | 0.558 |
| walker |  | 6101 | 54 | pub item at src/benchmark/relative_speed.rs:86 |  |  | 0.558 |
| walker |  | 6184 | 83 | pub item at src/benchmark/relative_speed.rs:7 |  |  | 0.558 |
| ns | 6205 |  | 281 | benchmark/executor.rs — Executor trait | 5.3 |  | 0.545 |
| walker |  | 6238 | 54 | README.md section #12 |  |  | 0.545 |
| walker |  | 6378 | 140 | pub item at src/benchmark/timing_result.rs:5 |  |  | 0.545 |
| ns | 6457 |  | 252 | benchmark/executor.rs — ShellExecutor shell-spawn-time calibration | 5.4 | 5.3 | 0.531 |
| walker |  | 6610 | 232 | mod/use plumbing in src/export/mod.rs |  |  | 0.531 |
| walker |  | 6689 | 79 | impl method sigs in src/benchmark/scheduler.rs |  |  | 0.531 |
| walker |  | 6689 | 0 | impl method at src/benchmark/scheduler.rs:34 |  |  | 0.531 |
| walker |  | 6689 | 0 | impl method at src/benchmark/scheduler.rs:61 |  |  | 0.531 |
| walker |  | 6689 | 0 | impl method at src/benchmark/scheduler.rs:156 |  |  | 0.531 |
| ns | 6807 |  | 350 | outlier_detection.rs — modified Z-score algorithm | 6.1 |  | 0.519 |
| ns | 7056 |  | 249 | benchmark/relative_speed.rs — ratio + stddev propagation formula | 6.2 |  | 0.510 |
| walker |  | 7062 | 373 | pub item at src/error.rs:37 |  |  | 0.512 |
| walker |  | 7315 | 253 | mod/use plumbing in src/timer/mod.rs |  |  | 0.512 |
| ns | 7400 |  | 344 | benchmark/benchmark_result.rs — BenchmarkResult schema, part 1 (command/mean/stddev/median/user/system) | 6.3 |  | 0.500 |
| ns | 7643 |  | 243 | benchmark/benchmark_result.rs — remaining fields (min/max/times/memory/exit_codes/parameters) | 6.4 | 6.3 | 0.490 |
| walker |  | 7730 | 415 | impl method sigs in src/benchmark/mod.rs |  |  | 0.490 |
| walker |  | 7730 | 0 | impl method at src/benchmark/mod.rs:42 |  |  | 0.490 |
| walker |  | 7730 | 0 | impl method at src/benchmark/mod.rs:57 |  |  | 0.490 |
| walker |  | 7730 | 0 | impl method at src/benchmark/mod.rs:75 |  |  | 0.490 |
| walker |  | 7730 | 0 | impl method at src/benchmark/mod.rs:96 |  |  | 0.490 |
| walker |  | 7730 | 0 | impl method at src/benchmark/mod.rs:117 |  |  | 0.490 |
| walker |  | 7730 | 0 | impl method at src/benchmark/mod.rs:129 |  |  | 0.490 |
| walker |  | 7730 | 0 | impl method at src/benchmark/mod.rs:141 |  |  | 0.490 |
| ns | 7801 |  | 158 | output/format.rs — automatic time-unit selection thresholds | 6.5 |  | 0.487 |
| walker |  | 7916 | 186 | impl method sigs in src/options.rs |  |  | 0.487 |
| walker |  | 7916 | 0 | impl method at src/options.rs:49 |  |  | 0.487 |
| walker |  | 7916 | 0 | impl method at src/options.rs:57 |  |  | 0.487 |
| walker |  | 7916 | 0 | impl method at src/options.rs:133 |  |  | 0.487 |
| walker |  | 7916 | 0 | impl method at src/options.rs:165 |  |  | 0.487 |
| walker |  | 7916 | 0 | impl method at src/options.rs:276 |  |  | 0.487 |
| walker |  | 7916 | 0 | impl method at src/options.rs:470 |  |  | 0.487 |
| ns | 8062 |  | 261 | export/mod.rs — ExportType enum + Exporter trait | 7.1 |  | 0.487 |
| walker |  | 8185 | 269 | pub item at src/benchmark/executor.rs:35 |  |  | 0.513 |
| walker |  | 8259 | 74 | README.md section #13 |  |  | 0.513 |
| walker |  | 8341 | 82 | README.md section #11 |  |  | 0.513 |
| ns | 8373 |  | 311 | export/json.rs — full file | 7.2 |  | 0.501 |
| walker |  | 8671 | 330 | mod/use plumbing in src/benchmark/mod.rs |  |  | 0.501 |
| ns | 8762 |  | 389 | error.rs — OptionsError variants (exact user-facing error strings) | 8.1 |  | 0.516 |
| walker |  | 8804 | 133 | impl method sigs in src/export/asciidoc.rs |  |  | 0.516 |
| walker |  | 8804 | 0 | impl method at src/export/asciidoc.rs:8 |  |  | 0.516 |
| walker |  | 8804 | 0 | impl method at src/export/asciidoc.rs:22 |  |  | 0.516 |
| walker |  | 8804 | 0 | impl method at src/export/asciidoc.rs:26 |  |  | 0.516 |
| walker |  | 8804 | 0 | impl method at src/export/asciidoc.rs:30 |  |  | 0.516 |
| walker |  | 8804 | 0 | impl method at src/export/asciidoc.rs:34 |  |  | 0.516 |
| ns | 8896 |  | 134 | tests/common.rs — full file (test harness) | 9.1 |  | 0.511 |
| ns | 9164 |  | 268 | tests/execution_order_tests.rs — fullest execution-order exemplar | 9.2 |  | 0.502 |
| walker |  | 9168 | 364 | impl method sigs in src/command.rs |  |  | 0.502 |
| walker |  | 9168 | 0 | impl method at src/command.rs:34 |  |  | 0.502 |
| walker |  | 9168 | 0 | impl method at src/command.rs:54 |  |  | 0.502 |
| walker |  | 9168 | 0 | impl method at src/command.rs:61 |  |  | 0.502 |
| walker |  | 9168 | 0 | impl method at src/command.rs:77 |  |  | 0.502 |
| walker |  | 9168 | 0 | impl method at src/command.rs:81 |  |  | 0.502 |
| walker |  | 9168 | 0 | impl method at src/command.rs:96 |  |  | 0.502 |
| walker |  | 9168 | 0 | impl method at src/command.rs:100 |  |  | 0.502 |
| walker |  | 9168 | 0 | impl method at src/command.rs:106 |  |  | 0.502 |
| walker |  | 9168 | 0 | impl method at src/command.rs:137 |  |  | 0.502 |
| walker |  | 9168 | 0 | impl method at src/command.rs:250 |  |  | 0.502 |
| walker |  | 9168 | 0 | impl method at src/command.rs:254 |  |  | 0.502 |
| walker |  | 9168 | 0 | impl method at src/command.rs:260 |  |  | 0.502 |
| walker |  | 9223 | 55 | impl method at src/command.rs:42 |  |  | 0.502 |
| ns | 9516 |  | 352 | tests/integration_tests.rs — console output exemplar (time-unit formatting) | 9.3 |  | 0.492 |
| walker |  | 9551 | 328 | dev/build/target dependencies in Cargo.toml |  |  | 0.536 |
| walker |  | 9578 | 27 | pub item body at src/output/format.rs:5 body 6 |  |  | 0.536 |
| ns | 9771 |  | 255 | timer/mod.rs — execute_and_measure: platform-conditional process creation/timer start | 10.1 |  | 0.529 |
| ns | 9998 |  | 227 | README.md — Markdown export sample table | 11.1 |  | 0.526 |
