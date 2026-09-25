Score(3000)=0.577 I=0.849 C=0.392 ns_rows≤3K=21/56 grid(1000/1442/2080/3000/4327/6240/9000)=0.772/0.765/0.638/0.577/0.560/0.532/0.565

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 45 | 45 | Fs::DirListing { dir: . } |  |  | 0.000 |
| walker |  | 57 | 12 | Code::CodeKey { rung: Names, file: build.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.000 |
| walker |  | 82 | 25 | Fs::DirListing { dir: doc } |  |  | 0.000 |
| ns | 96 |  | 96 | Crate identity: package name, description, homepage, licence | 1.1 |  | 0.000 |
| walker |  | 126 | 44 | Fs::DirListing { dir: src } |  |  | 0.000 |
| walker |  | 139 | 13 | Fs::DirListing { dir: src/parameter } |  |  | 0.000 |
| ns | 141 |  | 45 | Repository root listing (complete) | 1.2 |  | 0.512 |
| walker |  | 156 | 17 | Fs::DirListing { dir: src/output } |  |  | 0.518 |
| walker |  | 176 | 20 | Fs::DirListing { dir: src/timer } |  |  | 0.528 |
| walker |  | 203 | 27 | Fs::DirListing { dir: src/benchmark } |  |  | 0.536 |
| ns | 217 |  | 76 | Cargo.toml: version 1.20.0, edition, MSRV, build script | 1.3 | 1.1 | 0.458 |
| walker |  | 231 | 28 | Fs::DirListing { dir: src/util } |  |  | 0.477 |
| ns | 261 |  | 44 | src/ listing: the flat modules and six subdirectories | 1.4 |  | 0.517 |
| walker |  | 266 | 35 | Fs::DirListing { dir: src/export } |  |  | 0.546 |
| walker |  | 274 | 8 | Fs::DirListing { dir: .github } |  |  | 0.547 |
| walker |  | 279 | 5 | Fs::DirListing { dir: .github/workflows } |  |  | 0.548 |
| ns | 343 |  | 82 | README feature list, first five bullets (rest elided) | 1.5 |  | 0.495 |
| walker |  | 348 | 69 | Markdown::ReadmeHeadline { file: README.md } |  |  | 0.495 |
| walker |  | 368 | 20 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.495 |
| ns | 409 |  | 66 | README feature list, remaining bullets | 1.6 | 1.5 | 0.471 |
| ns | 471 |  | 62 | src/benchmark/ and src/export/ listings (complete) | 1.7 |  | 0.502 |
| walker |  | 488 | 120 | Code::CodeKey { rung: Names, file: src/main.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.519 |
| walker |  | 501 | 13 | Code::CodeKey { rung: ModuleDoc, file: src/util/units.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.519 |
| walker |  | 514 | 13 | Code::CodeKey { rung: Names, file: src/benchmark/benchmark_result.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.519 |
| walker |  | 527 | 13 | Code::CodeKey { rung: Names, file: src/benchmark/timing_result.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.519 |
| walker |  | 542 | 15 | Code::CodeKey { rung: Names, file: src/util/randomized_environment_offset.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.519 |
| ns | 549 |  | 78 | src/output/, src/parameter/, src/timer/, src/util/ listings (complete) | 1.8 |  | 0.537 |
| walker |  | 560 | 18 | Code::CodeKey { rung: Names, file: src/export/tests.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.537 |
| walker |  | 579 | 19 | Code::CodeKey { rung: Names, file: src/parameter/tokenize.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.537 |
| ns | 643 |  | 94 | src/main.rs: module declarations and the types the entry point imports | 1.9 |  | 0.544 |
| walker |  | 751 | 172 | Toml::Identity { file: Cargo.toml } |  |  | 0.918 |
| walker |  | 786 | 35 | Toml::Operational { file: Cargo.toml } |  |  | 0.918 |
| walker |  | 827 | 41 | Code::CodeKey { rung: Names, file: src/cli.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.918 |
| walker |  | 869 | 42 | Code::CodeKey { rung: Decl, file: src/cli.rs, decl: 1, sub: 0, line: 8 } |  |  | 0.918 |
| walker |  | 890 | 21 | Fs::DirListing { dir: tests } |  |  | 0.921 |
| ns | 899 |  | 256 | src/main.rs: the run() pipeline | 1.10 | 1.9 | 0.806 |
| ns | 983 |  | 84 | src/main.rs: main() error reporting and exit code | 1.11 | 1.10 | 0.772 |
| walker |  | 1039 | 149 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: true } |  |  | 0.843 |
| walker |  | 1064 | 25 | Code::CodeKey { rung: Names, file: src/output/mod.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.843 |
| walker |  | 1089 | 25 | Code::CodeKey { rung: Names, file: src/timer/wall_clock_timer.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.843 |
| ns | 1098 |  | 115 | tests/, doc/, scripts/ and .github/ listings (complete) | 1.12 |  | 0.796 |
| walker |  | 1101 | 12 | Code::CodeKey { rung: Decl, file: src/timer/wall_clock_timer.rs, decl: 1, sub: 0, line: 5 } |  |  | 0.796 |
| walker |  | 1153 | 52 | Code::CodeKey { rung: Names, file: src/outlier_detection.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.796 |
| walker |  | 1182 | 29 | Code::CodeKey { rung: Names, file: src/export/csv.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.796 |
| walker |  | 1190 | 8 | Code::CodeKey { rung: Decl, file: src/export/csv.rs, decl: 1, sub: 0, line: 12 } |  |  | 0.796 |
| walker |  | 1205 | 15 | Code::CodeKey { rung: Decl, file: src/export/csv.rs, decl: 2, sub: 0, line: 15 } |  |  | 0.796 |
| walker |  | 1235 | 30 | Code::CodeKey { rung: Names, file: src/benchmark/scheduler.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.765 |
| ns | 1235 |  | 137 | src/cli.rs: get_cli_arguments() and the build_command() clap header | 2.1 |  | 0.765 |
| walker |  | 1265 | 30 | Code::CodeKey { rung: Names, file: src/export/markdown.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.765 |
| walker |  | 1273 | 8 | Code::CodeKey { rung: Decl, file: src/export/markdown.rs, decl: 1, sub: 0, line: 5 } |  |  | 0.765 |
| walker |  | 1305 | 32 | Code::CodeKey { rung: Names, file: src/export/orgmode.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.765 |
| walker |  | 1313 | 8 | Code::CodeKey { rung: Decl, file: src/export/orgmode.rs, decl: 1, sub: 0, line: 4 } |  |  | 0.765 |
| walker |  | 1377 | 64 | Code::CodeKey { rung: Names, file: src/error.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.765 |
| walker |  | 1401 | 24 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 4, sub: 0, line: 30 } |  |  | 0.765 |
| walker |  | 1426 | 25 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 2, sub: 0, line: 24 } |  |  | 0.765 |
| walker |  | 1462 | 36 | Code::CodeKey { rung: Names, file: src/export/asciidoc.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.765 |
| walker |  | 1470 | 8 | Code::CodeKey { rung: Decl, file: src/export/asciidoc.rs, decl: 1, sub: 0, line: 4 } |  |  | 0.765 |
| walker |  | 1505 | 35 | Code::CodeKey { rung: Decl, file: src/timer/wall_clock_timer.rs, decl: 2, sub: 0, line: 9 } |  |  | 0.765 |
| walker |  | 1545 | 40 | Code::CodeKey { rung: Names, file: src/output/warnings.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.766 |
| walker |  | 1575 | 30 | Code::CodeKey { rung: Decl, file: src/output/warnings.rs, decl: 1, sub: 0, line: 7 } |  |  | 0.766 |
| walker |  | 1605 | 30 | Code::CodeKey { rung: Decl, file: src/output/warnings.rs, decl: 3, sub: 0, line: 20 } |  |  | 0.766 |
| walker |  | 1645 | 40 | Code::CodeKey { rung: Names, file: src/util/min_max.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.766 |
| ns | 1672 |  | 437 | Complete option roster: every Arg::new(...) in src/cli.rs | 2.2 | 2.1 | 0.688 |
| walker |  | 1687 | 42 | Code::CodeKey { rung: Names, file: src/util/units.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.688 |
| walker |  | 1770 | 83 | Code::CodeKey { rung: Names, file: src/command.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.689 |
| walker |  | 1802 | 32 | Code::CodeKey { rung: Decl, file: src/command.rs, decl: 16, sub: 0, line: 387 } |  |  | 0.689 |
| ns | 1862 |  | 190 | Complete short-alias roster for the CLI options | 2.3 | 2.2 | 0.656 |
| walker |  | 1881 | 79 | Code::CodeKey { rung: Decl, file: src/command.rs, decl: 12, sub: 0, line: 136 } |  |  | 0.656 |
| walker |  | 1924 | 43 | Code::CodeKey { rung: Names, file: src/export/json.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.656 |
| walker |  | 1932 | 8 | Code::CodeKey { rung: Decl, file: src/export/json.rs, decl: 2, sub: 0, line: 16 } |  |  | 0.656 |
| walker |  | 1945 | 13 | Code::CodeKey { rung: Decl, file: src/export/json.rs, decl: 3, sub: 0, line: 19 } |  |  | 0.656 |
| walker |  | 1989 | 44 | Code::CodeKey { rung: Names, file: src/util/mod.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.656 |
| ns | 2016 |  | 154 | Enumerated value sets, defaults, and the two hidden options | 2.4 | 2.2 | 0.638 |
| walker |  | 2031 | 42 | Code::CodeKey { rung: Decl, file: src/util/units.rs, decl: 4, sub: 0, line: 16 } |  |  | 0.638 |
| walker |  | 2074 | 43 | Code::CodeKey { rung: Decl, file: src/util/units.rs, decl: 3, sub: 0, line: 9 } |  |  | 0.638 |
| walker |  | 2081 | 7 | Code::CodeKey { rung: Doc, file: src/util/units.rs, decl: 3, sub: 0, line: 9 } |  |  | 0.638 |
| ns | 2146 |  | 130 | Complete set of inter-option conflicts and requirements | 2.5 | 2.2 | 0.625 |
| walker |  | 2176 | 95 | Code::CodeKey { rung: ModuleDoc, file: src/outlier_detection.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.625 |
| walker |  | 2222 | 46 | Code::CodeKey { rung: Names, file: src/util/exit_code.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.625 |
| walker |  | 2231 | 9 | Code::CodeKey { rung: Decl, file: src/util/exit_code.rs, decl: 1, sub: 0, line: 3 } |  |  | 0.625 |
| walker |  | 2241 | 10 | Code::CodeKey { rung: Decl, file: src/util/exit_code.rs, decl: 2, sub: 0, line: 19 } |  |  | 0.625 |
| walker |  | 2256 | 15 | Code::CodeKey { rung: Doc, file: src/command.rs, decl: 11, sub: 0, line: 134 } |  |  | 0.625 |
| walker |  | 2264 | 8 | Code::CodeKey { rung: Body, file: src/util/exit_code.rs, decl: 2, sub: 0, line: 19 } |  |  | 0.625 |
| walker |  | 2335 | 71 | Toml::Config { file: Cargo.toml } |  |  | 0.625 |
| ns | 2366 |  | 220 | src/options.rs: the Options struct, run bounds through setup_command | 3.1 |  | 0.600 |
| walker |  | 2387 | 52 | Code::CodeKey { rung: Decl, file: src/benchmark/scheduler.rs, decl: 1, sub: 0, line: 13 } |  |  | 0.600 |
| walker |  | 2439 | 52 | Code::CodeKey { rung: Decl, file: src/output/warnings.rs, decl: 2, sub: 0, line: 13 } |  |  | 0.601 |
| walker |  | 2550 | 111 | Code::CodeKey { rung: Decl, file: src/command.rs, decl: 1, sub: 0, line: 21 } |  |  | 0.601 |
| walker |  | 2560 | 10 | Code::CodeKey { rung: Doc, file: src/output/warnings.rs, decl: 2, sub: 0, line: 13 } |  |  | 0.601 |
| walker |  | 2570 | 10 | Code::CodeKey { rung: Doc, file: src/util/units.rs, decl: 2, sub: 0, line: 6 } |  |  | 0.601 |
| ns | 2577 |  | 211 | src/options.rs: remaining Options fields (output, sort, executor, I/O, time unit) | 3.2 | 3.1 | 0.581 |
| walker |  | 2583 | 13 | Code::CodeKey { rung: Doc, file: src/command.rs, decl: 1, sub: 0, line: 21 } |  |  | 0.581 |
| walker |  | 2645 | 62 | Code::CodeKey { rung: Names, file: src/export/markup.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.581 |
| walker |  | 2658 | 13 | Code::CodeKey { rung: Decl, file: src/export/markup.rs, decl: 10, sub: 0, line: 108 } |  |  | 0.581 |
| walker |  | 2675 | 17 | Code::CodeKey { rung: Decl, file: src/export/markup.rs, decl: 1, sub: 0, line: 10 } |  |  | 0.581 |
| walker |  | 2731 | 56 | Fs::DirListing { dir: scripts } |  |  | 0.628 |
| walker |  | 2797 | 66 | Code::CodeKey { rung: Names, file: src/parameter/mod.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.628 |
| walker |  | 2829 | 32 | Code::CodeKey { rung: Decl, file: src/parameter/mod.rs, decl: 1, sub: 0, line: 7 } |  |  | 0.628 |
| ns | 2839 |  | 262 | src/options.rs: impl Default for Options -- the concrete default values | 3.3 | 3.1 | 0.597 |
| walker |  | 2863 | 34 | Code::CodeKey { rung: Decl, file: src/parameter/mod.rs, decl: 2, sub: 0, line: 13 } |  |  | 0.597 |
| ns | 2991 |  | 152 | src/options.rs: DEFAULT_SHELL per platform, the Shell enum and its parser | 3.4 |  | 0.577 |
| ns | 3094 |  | 103 | src/options.rs: ExecutorKind (Raw / Shell / Mock) and its default | 3.5 | 3.4 | 0.564 |
| walker |  | 3111 | 248 | Toml::Dependencies { file: Cargo.toml } |  |  | 0.564 |
| walker |  | 3179 | 68 | Code::CodeKey { rung: Names, file: src/util/number.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.564 |
| walker |  | 3198 | 19 | Code::CodeKey { rung: Decl, file: src/util/number.rs, decl: 6, sub: 0, line: 30 } |  |  | 0.564 |
| walker |  | 3218 | 20 | Code::CodeKey { rung: Decl, file: src/util/number.rs, decl: 4, sub: 0, line: 24 } |  |  | 0.564 |
| walker |  | 3248 | 30 | Code::CodeKey { rung: Decl, file: src/util/number.rs, decl: 2, sub: 0, line: 15 } |  |  | 0.564 |
| walker |  | 3289 | 41 | Code::CodeKey { rung: Decl, file: src/util/number.rs, decl: 8, sub: 0, line: 36 } |  |  | 0.564 |
| ns | 3318 |  | 224 | src/options.rs: OutputStyleOption and SortOrder vocabularies | 3.6 |  | 0.539 |
| walker |  | 3340 | 51 | Code::CodeKey { rung: Decl, file: src/util/number.rs, decl: 1, sub: 0, line: 8 } |  |  | 0.540 |
| ns | 3560 |  | 242 | src/options.rs: CommandInputPolicy and CommandOutputPolicy | 3.7 |  | 0.517 |
| walker |  | 3579 | 239 | Code::CodeKey { rung: Names, file: src/options.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.522 |
| walker |  | 3586 | 7 | Code::CodeKey { rung: Decl, file: src/options.rs, decl: 2, sub: 0, line: 19 } |  |  | 0.523 |
| walker |  | 3596 | 10 | Code::CodeKey { rung: Decl, file: src/options.rs, decl: 1, sub: 0, line: 16 } |  |  | 0.524 |
| walker |  | 3612 | 16 | Code::CodeKey { rung: Decl, file: src/options.rs, decl: 4, sub: 0, line: 32 } |  |  | 0.524 |
| walker |  | 3628 | 16 | Code::CodeKey { rung: Decl, file: src/options.rs, decl: 27, sub: 0, line: 251 } |  |  | 0.525 |
| walker |  | 3646 | 18 | Code::CodeKey { rung: Decl, file: src/options.rs, decl: 15, sub: 0, line: 116 } |  |  | 0.525 |
| walker |  | 3664 | 18 | Code::CodeKey { rung: Decl, file: src/options.rs, decl: 24, sub: 0, line: 191 } |  |  | 0.527 |
| walker |  | 3693 | 29 | Code::CodeKey { rung: Decl, file: src/options.rs, decl: 18, sub: 0, line: 132 } |  |  | 0.527 |
| walker |  | 3723 | 30 | Code::CodeKey { rung: Decl, file: src/options.rs, decl: 6, sub: 0, line: 38 } |  |  | 0.527 |
| walker |  | 3756 | 33 | Code::CodeKey { rung: Decl, file: src/options.rs, decl: 21, sub: 0, line: 164 } |  |  | 0.527 |
| walker |  | 3793 | 37 | Code::CodeKey { rung: Decl, file: src/options.rs, decl: 13, sub: 0, line: 101 } |  |  | 0.530 |
| ns | 3800 |  | 240 | src/options.rs: CmdFailureAction and RunBounds (default min = 10) | 3.8 |  | 0.512 |
| walker |  | 3832 | 39 | Code::CodeKey { rung: Decl, file: src/options.rs, decl: 23, sub: 0, line: 184 } |  |  | 0.525 |
| walker |  | 3885 | 53 | Code::CodeKey { rung: Decl, file: src/options.rs, decl: 8, sub: 0, line: 47 } |  |  | 0.525 |
| walker |  | 3940 | 55 | Code::CodeKey { rung: Decl, file: src/options.rs, decl: 14, sub: 0, line: 108 } |  |  | 0.532 |
| walker |  | 4003 | 63 | Code::CodeKey { rung: Decl, file: src/options.rs, decl: 29, sub: 0, line: 275 } |  |  | 0.532 |
| walker |  | 4067 | 64 | Code::CodeKey { rung: Decl, file: src/options.rs, decl: 3, sub: 0, line: 23 } |  |  | 0.550 |
| walker |  | 4136 | 69 | Code::CodeKey { rung: Decl, file: src/options.rs, decl: 17, sub: 0, line: 122 } |  |  | 0.558 |
| ns | 4189 |  | 389 | src/error.rs: the complete OptionsError variant set with messages | 3.10 |  | 0.539 |
| walker |  | 4232 | 96 | Code::CodeKey { rung: Decl, file: src/options.rs, decl: 11, sub: 0, line: 70 } |  |  | 0.560 |
| walker |  | 4271 | 39 | Markdown::Section { file: README.md, section_index: 11, keeps_default_concavity: false } |  |  | 0.560 |
| walker |  | 4399 | 128 | Code::CodeKey { rung: Decl, file: src/options.rs, decl: 20, sub: 0, line: 148 } |  |  | 0.596 |
| ns | 4407 |  | 218 | src/error.rs: the complete ParameterScanError variant set | 3.11 | 3.10 | 0.581 |
| walker |  | 4421 | 22 | Code::CodeKey { rung: Body, file: src/cli.rs, decl: 1, sub: 0, line: 8 } |  |  | 0.581 |
| walker |  | 4494 | 73 | Code::CodeKey { rung: Decl, file: src/benchmark/scheduler.rs, decl: 2, sub: 0, line: 20 } |  |  | 0.582 |
| walker |  | 4567 | 73 | Code::CodeKey { rung: Decl, file: src/export/markdown.rs, decl: 2, sub: 0, line: 8 } |  |  | 0.582 |
| ns | 4627 |  | 220 | src/benchmark/executor.rs: the Executor trait | 4.1 |  | 0.567 |
| walker |  | 4640 | 73 | Code::CodeKey { rung: Decl, file: src/export/orgmode.rs, decl: 2, sub: 0, line: 7 } |  |  | 0.567 |
| walker |  | 4718 | 78 | Code::CodeKey { rung: Names, file: src/output/format.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.568 |
| ns | 4752 |  | 125 | The three Executor implementations and their state | 4.2 | 4.1 | 0.558 |
| walker |  | 4798 | 80 | Code::CodeKey { rung: Names, file: src/timer/unix_timer.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.558 |
| walker |  | 4813 | 15 | Code::CodeKey { rung: Decl, file: src/timer/unix_timer.rs, decl: 2, sub: 0, line: 18 } |  |  | 0.558 |
| walker |  | 4855 | 42 | Code::CodeKey { rung: Decl, file: src/timer/unix_timer.rs, decl: 3, sub: 0, line: 22 } |  |  | 0.558 |
| ns | 4904 |  | 152 | BenchmarkIteration and the $HYPERFINE_ITERATION values | 4.3 |  | 0.547 |
| walker |  | 4927 | 72 | Code::CodeKey { rung: Decl, file: src/timer/unix_timer.rs, decl: 1, sub: 0, line: 9 } |  |  | 0.547 |
| ns | 5072 |  | 168 | The shared command runner: stdio wiring and injected environment variables | 4.4 | 4.3 | 0.538 |
| walker |  | 5076 | 149 | Code::CodeKey { rung: Decl, file: src/options.rs, decl: 12, sub: 0, line: 83 } |  |  | 0.571 |
| walker |  | 5090 | 14 | Code::CodeKey { rung: Doc, file: src/util/min_max.rs, decl: 1, sub: 0, line: 2 } |  |  | 0.571 |
| walker |  | 5177 | 87 | Code::CodeKey { rung: Names, file: src/timer/windows_timer.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.571 |
| walker |  | 5190 | 13 | Code::CodeKey { rung: Decl, file: src/timer/windows_timer.rs, decl: 3, sub: 0, line: 49 } |  |  | 0.571 |
| walker |  | 5207 | 17 | Code::CodeKey { rung: Decl, file: src/timer/windows_timer.rs, decl: 7, sub: 0, line: 124 } |  |  | 0.571 |
| walker |  | 5257 | 50 | Code::CodeKey { rung: Decl, file: src/timer/windows_timer.rs, decl: 4, sub: 0, line: 53 } |  |  | 0.571 |
| ns | 5284 |  | 212 | Non-zero exit handling and the failure message users actually see | 4.5 | 4.4 | 0.561 |
| walker |  | 5301 | 44 | Code::CodeKey { rung: Decl, file: src/benchmark/scheduler.rs, decl: 3, sub: 0, line: 21 } |  |  | 0.561 |
| walker |  | 5315 | 14 | Code::CodeKey { rung: Body, file: src/error.rs, decl: 3, sub: 0, line: 25 } |  |  | 0.561 |
| walker |  | 5329 | 14 | Code::CodeKey { rung: Body, file: src/error.rs, decl: 5, sub: 0, line: 31 } |  |  | 0.561 |
| walker |  | 5426 | 97 | Code::CodeKey { rung: Names, file: src/benchmark/mod.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.561 |
| walker |  | 5460 | 34 | Code::CodeKey { rung: Decl, file: src/benchmark/mod.rs, decl: 3, sub: 0, line: 41 } |  |  | 0.561 |
| ns | 5481 |  | 197 | Shell-spawning calibration: 50 probe runs, and where the overhead is subtracted | 4.6 | 4.2 | 0.553 |
| walker |  | 5508 | 48 | Code::CodeKey { rung: Decl, file: src/benchmark/mod.rs, decl: 2, sub: 0, line: 34 } |  |  | 0.553 |
| walker |  | 5521 | 13 | Code::CodeKey { rung: Doc, file: src/benchmark/mod.rs, decl: 1, sub: 0, line: 32 } |  |  | 0.553 |
| walker |  | 5548 | 27 | Code::CodeKey { rung: Decl, file: src/export/json.rs, decl: 1, sub: 0, line: 11 } |  |  | 0.553 |
| walker |  | 5564 | 16 | Code::CodeKey { rung: Doc, file: src/util/min_max.rs, decl: 2, sub: 0, line: 10 } |  |  | 0.553 |
| ns | 5679 |  | 198 | src/benchmark/scheduler.rs: Scheduler state and executor selection | 4.7 |  | 0.550 |
| walker |  | 5744 | 180 | Code::CodeKey { rung: Decl, file: src/command.rs, decl: 2, sub: 0, line: 33 } |  |  | 0.550 |
| walker |  | 5799 | 55 | Code::CodeKey { rung: Decl, file: src/command.rs, decl: 4, sub: 0, line: 42 } |  |  | 0.550 |
| walker |  | 5852 | 53 | Code::CodeKey { rung: Decl, file: src/export/markup.rs, decl: 11, sub: 0, line: 109 } |  |  | 0.550 |
| ns | 5933 |  | 254 | run_benchmarks(): reference command first, calibrate once, export after each benchmark | 4.8 | 4.7 | 0.537 |
| walker |  | 5959 | 107 | Code::CodeKey { rung: Names, file: src/parameter/range_step.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.537 |
| walker |  | 5993 | 34 | Code::CodeKey { rung: Decl, file: src/parameter/range_step.rs, decl: 4, sub: 0, line: 40 } |  |  | 0.538 |
| walker |  | 6031 | 38 | Code::CodeKey { rung: Decl, file: src/parameter/range_step.rs, decl: 3, sub: 0, line: 33 } |  |  | 0.538 |
| walker |  | 6091 | 60 | Code::CodeKey { rung: Decl, file: src/parameter/range_step.rs, decl: 6, sub: 0, line: 62 } |  |  | 0.538 |
| walker |  | 6190 | 99 | Code::CodeKey { rung: Decl, file: src/parameter/range_step.rs, decl: 1, sub: 0, line: 7 } |  |  | 0.538 |
| ns | 6208 |  | 275 | src/benchmark/mod.rs: Benchmark struct, MIN_EXECUTION_TIME, and the complete method roster | 4.10 |  | 0.532 |
| walker |  | 6244 | 54 | Code::CodeKey { rung: Decl, file: src/benchmark/mod.rs, decl: 4, sub: 0, line: 42 } |  |  | 0.532 |
| walker |  | 6347 | 103 | Code::CodeKey { rung: Decl, file: src/parameter/range_step.rs, decl: 2, sub: 0, line: 19 } |  |  | 0.532 |
| walker |  | 6402 | 55 | Code::CodeKey { rung: Decl, file: src/export/csv.rs, decl: 3, sub: 0, line: 16 } |  |  | 0.532 |
| ns | 6418 |  | 210 | How the number of runs is decided | 4.11 | 4.10 | 0.521 |
| walker |  | 6457 | 55 | Code::CodeKey { rung: Decl, file: src/export/json.rs, decl: 4, sub: 0, line: 20 } |  |  | 0.521 |
| walker |  | 6475 | 18 | Code::CodeKey { rung: Doc, file: src/output/format.rs, decl: 2, sub: 0, line: 11 } |  |  | 0.521 |
| walker |  | 6493 | 18 | Code::CodeKey { rung: Doc, file: src/output/format.rs, decl: 3, sub: 0, line: 18 } |  |  | 0.521 |
| walker |  | 6607 | 114 | Code::CodeKey { rung: Names, file: src/output/progress_bar.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.521 |
| walker |  | 6614 | 7 | Code::CodeKey { rung: Decl, file: src/output/progress_bar.rs, decl: 2, sub: 0, line: 9 } |  |  | 0.521 |
| walker |  | 6624 | 10 | Code::CodeKey { rung: Decl, file: src/output/progress_bar.rs, decl: 1, sub: 0, line: 6 } |  |  | 0.521 |
| ns | 6626 |  | 208 | The three warning triggers inside Benchmark::run | 4.12 | 4.10 | 0.514 |
| walker |  | 6635 | 11 | Code::CodeKey { rung: Doc, file: src/output/progress_bar.rs, decl: 3, sub: 0, line: 13 } |  |  | 0.514 |
| walker |  | 6751 | 116 | Code::CodeKey { rung: Names, file: src/timer/mod.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.514 |
| ns | 6779 |  | 153 | src/outlier_detection.rs: the modified Z-score method and its threshold | 4.13 | 4.12 | 0.511 |
| walker |  | 6838 | 87 | Code::CodeKey { rung: Decl, file: src/timer/mod.rs, decl: 2, sub: 0, line: 41 } |  |  | 0.511 |
| walker |  | 6853 | 15 | Code::CodeKey { rung: Doc, file: src/timer/mod.rs, decl: 2, sub: 0, line: 41 } |  |  | 0.511 |
| walker |  | 6868 | 15 | Code::CodeKey { rung: Doc, file: src/timer/mod.rs, decl: 4, sub: 0, line: 83 } |  |  | 0.511 |
| walker |  | 7007 | 139 | Code::CodeKey { rung: Names, file: src/export/mod.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.511 |
| walker |  | 7026 | 19 | Code::CodeKey { rung: Decl, file: src/export/mod.rs, decl: 4, sub: 0, line: 56 } |  |  | 0.511 |
| walker |  | 7064 | 38 | Code::CodeKey { rung: Decl, file: src/export/mod.rs, decl: 6, sub: 0, line: 67 } |  |  | 0.511 |
| ns | 7075 |  | 296 | src/benchmark/benchmark_result.rs: the complete BenchmarkResult field set (= the JSON export schema) | 5.1 |  | 0.502 |
| walker |  | 7079 | 15 | Code::CodeKey { rung: Decl, file: src/export/mod.rs, decl: 2, sub: 0, line: 46 } |  |  | 0.502 |
| walker |  | 7092 | 13 | Code::CodeKey { rung: Doc, file: src/export/mod.rs, decl: 6, sub: 0, line: 67 } |  |  | 0.502 |
| walker |  | 7171 | 79 | Code::CodeKey { rung: Decl, file: src/export/mod.rs, decl: 7, sub: 0, line: 73 } |  |  | 0.502 |
| walker |  | 7215 | 44 | Code::CodeKey { rung: Decl, file: src/export/mod.rs, decl: 8, sub: 0, line: 76 } |  |  | 0.495 |
| ns | 7215 |  | 140 | src/benchmark/timing_result.rs: TimingResult in full | 5.2 | 5.1 | 0.495 |
| walker |  | 7241 | 26 | Code::CodeKey { rung: Decl, file: src/export/mod.rs, decl: 5, sub: 0, line: 61 } |  |  | 0.495 |
| walker |  | 7360 | 119 | Code::CodeKey { rung: Names, file: src/benchmark/relative_speed.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.496 |
| ns | 7378 |  | 163 | src/benchmark/relative_speed.rs: annotated result type and the complete public function set | 5.3 |  | 0.493 |
| walker |  | 7399 | 39 | Code::CodeKey { rung: Decl, file: src/benchmark/relative_speed.rs, decl: 7, sub: 0, line: 112 } |  |  | 0.493 |
| walker |  | 7439 | 40 | Code::CodeKey { rung: Decl, file: src/benchmark/relative_speed.rs, decl: 6, sub: 0, line: 98 } |  |  | 0.493 |
| walker |  | 7493 | 54 | Code::CodeKey { rung: Decl, file: src/benchmark/relative_speed.rs, decl: 5, sub: 0, line: 86 } |  |  | 0.493 |
| ns | 7536 |  | 158 | src/export/mod.rs: the ExportType enum -- the five output formats | 5.4 |  | 0.485 |
| walker |  | 7576 | 83 | Code::CodeKey { rung: Decl, file: src/benchmark/relative_speed.rs, decl: 1, sub: 0, line: 6 } |  |  | 0.500 |
| ns | 7698 |  | 162 | src/export/mod.rs: the Exporter trait and ExportManager | 5.5 | 5.4 | 0.498 |
| walker |  | 7701 | 125 | Code::CodeKey { rung: Decl, file: src/export/asciidoc.rs, decl: 2, sub: 0, line: 7 } |  |  | 0.498 |
| walker |  | 7710 | 9 | Code::CodeKey { rung: Body, file: src/export/asciidoc.rs, decl: 6, sub: 0, line: 30 } |  |  | 0.498 |
| walker |  | 7764 | 54 | Markdown::Section { file: README.md, section_index: 14, keeps_default_concavity: false } |  |  | 0.498 |
| ns | 7835 |  | 137 | Flag-to-exporter wiring and the '-' means stdout convention | 5.6 | 5.5 | 0.494 |
| walker |  | 8009 | 245 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 1, sub: 0, line: 6 } |  |  | 0.515 |
| ns | 8051 |  | 216 | src/export/markup.rs: the shared table shape and the blanket Exporter impl | 5.7 | 5.5 | 0.510 |
| walker |  | 8136 | 127 | Code::CodeKey { rung: Decl, file: src/benchmark/timing_result.rs, decl: 1, sub: 0, line: 4 } |  |  | 0.526 |
| walker |  | 8148 | 12 | Code::CodeKey { rung: Doc, file: src/benchmark/timing_result.rs, decl: 1, sub: 0, line: 4 } |  |  | 0.526 |
| ns | 8267 |  | 216 | The five exporter types, and the CSV column set | 5.8 | 5.4 | 0.523 |
| walker |  | 8276 | 128 | Code::CodeKey { rung: Decl, file: src/export/mod.rs, decl: 1, sub: 0, line: 27 } |  |  | 0.538 |
| walker |  | 8297 | 21 | Code::CodeKey { rung: Body, file: src/util/randomized_environment_offset.rs, decl: 1, sub: 0, line: 6 } |  |  | 0.538 |
| walker |  | 8466 | 169 | Code::CodeKey { rung: Names, file: src/benchmark/executor.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.539 |
| walker |  | 8480 | 14 | Code::CodeKey { rung: Decl, file: src/benchmark/executor.rs, decl: 9, sub: 0, line: 122 } |  |  | 0.541 |
| walker |  | 8501 | 21 | Code::CodeKey { rung: Decl, file: src/benchmark/executor.rs, decl: 23, sub: 0, line: 302 } |  |  | 0.544 |
| walker |  | 8522 | 21 | Code::CodeKey { rung: Decl, file: src/benchmark/executor.rs, decl: 24, sub: 0, line: 307 } |  |  | 0.544 |
| ns | 8524 |  | 257 | src/output/format.rs: automatic time-unit selection, and the Unit type | 5.9 |  | 0.540 |
| walker |  | 8544 | 22 | Code::CodeKey { rung: Decl, file: src/benchmark/executor.rs, decl: 10, sub: 0, line: 126 } |  |  | 0.540 |
| walker |  | 8568 | 24 | Code::CodeKey { rung: Decl, file: src/benchmark/executor.rs, decl: 2, sub: 0, line: 25 } |  |  | 0.541 |
| walker |  | 8596 | 28 | Code::CodeKey { rung: Decl, file: src/benchmark/executor.rs, decl: 17, sub: 0, line: 175 } |  |  | 0.541 |
| walker |  | 8627 | 31 | Code::CodeKey { rung: Decl, file: src/benchmark/executor.rs, decl: 1, sub: 0, line: 19 } |  |  | 0.544 |
| ns | 8661 |  | 137 | src/output/warnings.rs: the complete Warnings set | 5.10 | 4.12 | 0.550 |
| walker |  | 8667 | 40 | Code::CodeKey { rung: Decl, file: src/benchmark/executor.rs, decl: 16, sub: 0, line: 169 } |  |  | 0.558 |
| walker |  | 8718 | 51 | Code::CodeKey { rung: Decl, file: src/benchmark/executor.rs, decl: 12, sub: 0, line: 132 } |  |  | 0.558 |
| walker |  | 8769 | 51 | Code::CodeKey { rung: Decl, file: src/benchmark/executor.rs, decl: 19, sub: 0, line: 185 } |  |  | 0.558 |
| walker |  | 8821 | 52 | Code::CodeKey { rung: Decl, file: src/benchmark/executor.rs, decl: 4, sub: 0, line: 35 } |  |  | 0.559 |
| ns | 8869 |  | 208 | src/command.rs: the Command type and its complete method roster | 6.1 |  | 0.565 |
| walker |  | 8874 | 53 | Code::CodeKey { rung: Decl, file: src/benchmark/executor.rs, decl: 26, sub: 0, line: 322 } |  |  | 0.565 |
| walker |  | 8914 | 40 | Code::CodeKey { rung: Decl, file: src/export/tests.rs, decl: 1, sub: 0, line: 9 } |  |  | 0.565 |
| walker |  | 8926 | 12 | Code::CodeKey { rung: Body, file: src/export/asciidoc.rs, decl: 7, sub: 0, line: 34 } |  |  | 0.565 |
| walker |  | 8938 | 12 | Code::CodeKey { rung: Body, file: src/export/markdown.rs, decl: 5, sub: 0, line: 26 } |  |  | 0.565 |
| walker |  | 8950 | 12 | Code::CodeKey { rung: Body, file: src/export/orgmode.rs, decl: 5, sub: 0, line: 20 } |  |  | 0.565 |
| ns | 9015 |  | 146 | How {param} placeholders are substituted, and why naively | 6.2 | 6.1 | 0.562 |
| walker |  | 9024 | 74 | Code::CodeKey { rung: Decl, file: src/benchmark/executor.rs, decl: 5, sub: 0, line: 37 } |  |  | 0.567 |
| walker |  | 9098 | 74 | Code::CodeKey { rung: Decl, file: src/benchmark/executor.rs, decl: 13, sub: 0, line: 133 } |  |  | 0.567 |
| walker |  | 9172 | 74 | Code::CodeKey { rung: Decl, file: src/benchmark/executor.rs, decl: 20, sub: 0, line: 186 } |  |  | 0.567 |
| ns | 9202 |  | 187 | src/command.rs: Commands and its complete method roster, with the three construction modes | 6.3 |  | 0.564 |
| walker |  | 9249 | 77 | Code::CodeKey { rung: Decl, file: src/benchmark/executor.rs, decl: 27, sub: 0, line: 323 } |  |  | 0.564 |
| walker |  | 9264 | 15 | Code::CodeKey { rung: Doc, file: src/cli.rs, decl: 2, sub: 0, line: 18 } |  |  | 0.565 |
| walker |  | 9420 | 156 | Code::CodeKey { rung: Decl, file: src/export/markup.rs, decl: 2, sub: 0, line: 15 } |  |  | 0.570 |
| walker |  | 9429 | 9 | Code::CodeKey { rung: Body, file: src/export/markup.rs, decl: 6, sub: 0, line: 87 } |  |  | 0.570 |
| ns | 9431 |  | 229 | src/parameter/: ParameterValue, RangeStep and its 100_000 cap, tokenize() | 6.4 |  | 0.576 |
| walker |  | 9439 | 10 | Code::CodeKey { rung: Doc, file: src/util/units.rs, decl: 5, sub: 0, line: 18 } |  |  | 0.576 |
| walker |  | 9466 | 27 | Code::CodeKey { rung: Body, file: src/output/format.rs, decl: 1, sub: 0, line: 5 } |  |  | 0.576 |
| walker |  | 9485 | 19 | Code::CodeKey { rung: Doc, file: src/benchmark/relative_speed.rs, decl: 7, sub: 0, line: 112 } |  |  | 0.576 |
| walker |  | 9558 | 73 | Code::CodeKey { rung: Body, file: src/main.rs, decl: 2, sub: 0, line: 53 } |  |  | 0.586 |
| ns | 9581 |  | 150 | build.rs: shell completions generated from the same clap command | 7.1 | 2.1 | 0.581 |
| walker |  | 9611 | 53 | Code::CodeKey { rung: Decl, file: src/benchmark/relative_speed.rs, decl: 4, sub: 0, line: 27 } |  |  | 0.581 |
| walker |  | 9627 | 16 | Code::CodeKey { rung: Body, file: src/benchmark/scheduler.rs, decl: 6, sub: 0, line: 156 } |  |  | 0.581 |
| walker |  | 9690 | 63 | Code::CodeKey { rung: Doc, file: src/outlier_detection.rs, decl: 1, sub: 0, line: 13 } |  |  | 0.586 |
| ns | 9790 |  | 209 | .github/workflows/CICD.yml: the complete job set and the commands each runs | 7.2 |  | 0.580 |
| ns | 9982 |  | 192 | tests/: the shared harness helpers and the debug-mode test idiom | 7.3 |  | 0.573 |
