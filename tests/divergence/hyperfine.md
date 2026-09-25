Score(3000)=0.578 I=0.850 C=0.392 ns_rows≤3K=21/56 grid(1000/1442/2080/3000/4327/6240/9000)=0.785/0.826/0.688/0.578/0.486/0.501/0.591

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 45 | 45 | Fs::DirListing { dir: . } |  |  | 0.000 |
| walker |  | 70 | 25 | Fs::DirListing { dir: doc } |  |  | 0.000 |
| ns | 96 |  | 96 | Crate identity: package name, description, homepage, licence | 1.1 |  | 0.000 |
| walker |  | 114 | 44 | Fs::DirListing { dir: src } |  |  | 0.000 |
| walker |  | 127 | 13 | Fs::DirListing { dir: src/parameter } |  |  | 0.000 |
| ns | 141 |  | 45 | Repository root listing (complete) | 1.2 |  | 0.512 |
| walker |  | 144 | 17 | Fs::DirListing { dir: src/output } |  |  | 0.518 |
| walker |  | 164 | 20 | Fs::DirListing { dir: src/timer } |  |  | 0.528 |
| walker |  | 192 | 28 | Fs::DirListing { dir: src/util } |  |  | 0.551 |
| ns | 217 |  | 76 | Cargo.toml: version 1.20.0, edition, MSRV, build script | 1.3 | 1.1 | 0.470 |
| walker |  | 227 | 35 | Fs::DirListing { dir: src/export } |  |  | 0.481 |
| walker |  | 239 | 12 | Code::CodeKey { rung: Names, file: build.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.481 |
| walker |  | 247 | 8 | Fs::DirListing { dir: .github } |  |  | 0.482 |
| walker |  | 252 | 5 | Fs::DirListing { dir: .github/workflows } |  |  | 0.482 |
| ns | 261 |  | 44 | src/ listing: the flat modules and six subdirectories | 1.4 |  | 0.523 |
| walker |  | 321 | 69 | Markdown::ReadmeHeadline { file: README.md } |  |  | 0.523 |
| walker |  | 341 | 20 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.523 |
| ns | 343 |  | 82 | README feature list, first five bullets (rest elided) | 1.5 |  | 0.473 |
| walker |  | 354 | 13 | Code::CodeKey { rung: ModuleDoc, file: src/util/units.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.473 |
| ns | 409 |  | 66 | README feature list, remaining bullets | 1.6 | 1.5 | 0.449 |
| ns | 471 |  | 62 | src/benchmark/ and src/export/ listings (complete) | 1.7 |  | 0.419 |
| walker |  | 526 | 172 | Toml::Identity { file: Cargo.toml } |  |  | 0.811 |
| ns | 549 |  | 78 | src/output/, src/parameter/, src/timer/, src/util/ listings (complete) | 1.8 |  | 0.830 |
| walker |  | 561 | 35 | Toml::Operational { file: Cargo.toml } |  |  | 0.830 |
| walker |  | 582 | 21 | Fs::DirListing { dir: tests } |  |  | 0.833 |
| ns | 643 |  | 94 | src/main.rs: module declarations and the types the entry point imports | 1.9 |  | 0.772 |
| walker |  | 731 | 149 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: true } |  |  | 0.861 |
| walker |  | 851 | 120 | Code::CodeKey { rung: Names, file: src/main.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.937 |
| ns | 899 |  | 256 | src/main.rs: the run() pipeline | 1.10 | 1.9 | 0.820 |
| walker |  | 948 | 97 | Code::CodeKey { rung: ModuleDoc, file: src/outlier_detection.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.820 |
| walker |  | 963 | 15 | Code::CodeKey { rung: Names, file: src/util/randomized_environment_offset.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.820 |
| ns | 983 |  | 84 | src/main.rs: main() error reporting and exit code | 1.11 | 1.10 | 0.785 |
| walker |  | 1034 | 71 | Toml::Config { file: Cargo.toml } |  |  | 0.785 |
| walker |  | 1052 | 18 | Code::CodeKey { rung: Names, file: src/export/tests.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.785 |
| ns | 1098 |  | 115 | tests/, doc/, scripts/ and .github/ listings (complete) | 1.12 |  | 0.744 |
| walker |  | 1108 | 56 | Fs::DirListing { dir: scripts } |  |  | 0.809 |
| walker |  | 1127 | 19 | Code::CodeKey { rung: Names, file: src/parameter/tokenize.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.809 |
| walker |  | 1166 | 39 | Markdown::Section { file: README.md, section_index: 11, keeps_default_concavity: false } |  |  | 0.809 |
| walker |  | 1193 | 27 | Fs::DirListing { dir: src/benchmark } |  |  | 0.859 |
| walker |  | 1234 | 41 | Code::CodeKey { rung: Names, file: src/cli.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.860 |
| ns | 1235 |  | 137 | src/cli.rs: get_cli_arguments() and the build_command() clap header | 2.1 |  | 0.825 |
| walker |  | 1276 | 42 | Code::CodeKey { rung: Decl, file: src/cli.rs, decl: 1, sub: 0, line: 8 } |  |  | 0.826 |
| walker |  | 1298 | 22 | Code::CodeKey { rung: Body, file: src/cli.rs, decl: 1, sub: 0, line: 8 } |  |  | 0.826 |
| walker |  | 1323 | 25 | Code::CodeKey { rung: Names, file: src/output/mod.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.826 |
| walker |  | 1348 | 25 | Code::CodeKey { rung: Names, file: src/timer/wall_clock_timer.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.826 |
| walker |  | 1360 | 12 | Code::CodeKey { rung: Decl, file: src/timer/wall_clock_timer.rs, decl: 1, sub: 0, line: 5 } |  |  | 0.826 |
| walker |  | 1395 | 35 | Code::CodeKey { rung: Decl, file: src/timer/wall_clock_timer.rs, decl: 2, sub: 0, line: 9 } |  |  | 0.826 |
| walker |  | 1445 | 50 | Code::CodeKey { rung: Names, file: src/outlier_detection.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.826 |
| walker |  | 1474 | 29 | Code::CodeKey { rung: Names, file: src/export/csv.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.826 |
| walker |  | 1482 | 8 | Code::CodeKey { rung: Decl, file: src/export/csv.rs, decl: 1, sub: 0, line: 12 } |  |  | 0.826 |
| walker |  | 1497 | 15 | Code::CodeKey { rung: Decl, file: src/export/csv.rs, decl: 2, sub: 0, line: 15 } |  |  | 0.826 |
| walker |  | 1552 | 55 | Code::CodeKey { rung: Decl, file: src/export/csv.rs, decl: 3, sub: 0, line: 16 } |  |  | 0.826 |
| walker |  | 1582 | 30 | Code::CodeKey { rung: Names, file: src/export/markdown.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.826 |
| walker |  | 1590 | 8 | Code::CodeKey { rung: Decl, file: src/export/markdown.rs, decl: 1, sub: 0, line: 5 } |  |  | 0.826 |
| walker |  | 1663 | 73 | Code::CodeKey { rung: Decl, file: src/export/markdown.rs, decl: 2, sub: 0, line: 8 } |  |  | 0.826 |
| ns | 1672 |  | 437 | Complete option roster: every Arg::new(...) in src/cli.rs | 2.2 | 2.1 | 0.743 |
| walker |  | 1675 | 12 | Code::CodeKey { rung: Body, file: src/export/markdown.rs, decl: 5, sub: 0, line: 26 } |  |  | 0.743 |
| walker |  | 1707 | 32 | Code::CodeKey { rung: Names, file: src/export/orgmode.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.743 |
| walker |  | 1715 | 8 | Code::CodeKey { rung: Decl, file: src/export/orgmode.rs, decl: 1, sub: 0, line: 4 } |  |  | 0.743 |
| walker |  | 1788 | 73 | Code::CodeKey { rung: Decl, file: src/export/orgmode.rs, decl: 2, sub: 0, line: 7 } |  |  | 0.743 |
| walker |  | 1800 | 12 | Code::CodeKey { rung: Body, file: src/export/orgmode.rs, decl: 5, sub: 0, line: 20 } |  |  | 0.743 |
| walker |  | 1817 | 17 | Code::CodeKey { rung: Body, file: src/export/markdown.rs, decl: 3, sub: 0, line: 9 } |  |  | 0.743 |
| ns | 1862 |  | 190 | Complete short-alias roster for the CLI options | 2.3 | 2.2 | 0.708 |
| walker |  | 1881 | 64 | Code::CodeKey { rung: Names, file: src/error.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.708 |
| walker |  | 1905 | 24 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 4, sub: 0, line: 30 } |  |  | 0.708 |
| walker |  | 1930 | 25 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 2, sub: 0, line: 24 } |  |  | 0.708 |
| walker |  | 1944 | 14 | Code::CodeKey { rung: Body, file: src/error.rs, decl: 3, sub: 0, line: 25 } |  |  | 0.708 |
| walker |  | 1958 | 14 | Code::CodeKey { rung: Body, file: src/error.rs, decl: 5, sub: 0, line: 31 } |  |  | 0.708 |
| walker |  | 1994 | 36 | Code::CodeKey { rung: Names, file: src/export/asciidoc.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.708 |
| walker |  | 2002 | 8 | Code::CodeKey { rung: Decl, file: src/export/asciidoc.rs, decl: 1, sub: 0, line: 4 } |  |  | 0.708 |
| ns | 2016 |  | 154 | Enumerated value sets, defaults, and the two hidden options | 2.4 | 2.2 | 0.688 |
| ns | 2146 |  | 130 | Complete set of inter-option conflicts and requirements | 2.5 | 2.2 | 0.674 |
| ns | 2366 |  | 220 | src/options.rs: the Options struct, run bounds through setup_command | 3.1 |  | 0.647 |
| walker |  | 2475 | 473 | Toml::Dependencies { file: Cargo.toml } |  |  | 0.647 |
| ns | 2577 |  | 211 | src/options.rs: remaining Options fields (output, sort, executor, I/O, time unit) | 3.2 | 3.1 | 0.626 |
| walker |  | 2600 | 125 | Code::CodeKey { rung: Decl, file: src/export/asciidoc.rs, decl: 2, sub: 0, line: 7 } |  |  | 0.626 |
| walker |  | 2609 | 9 | Code::CodeKey { rung: Body, file: src/export/asciidoc.rs, decl: 6, sub: 0, line: 30 } |  |  | 0.626 |
| walker |  | 2621 | 12 | Code::CodeKey { rung: Body, file: src/export/asciidoc.rs, decl: 7, sub: 0, line: 34 } |  |  | 0.626 |
| walker |  | 2634 | 13 | Code::CodeKey { rung: Body, file: src/export/asciidoc.rs, decl: 4, sub: 0, line: 22 } |  |  | 0.626 |
| walker |  | 2688 | 54 | Markdown::Section { file: README.md, section_index: 14, keeps_default_concavity: false } |  |  | 0.626 |
| ns | 2839 |  | 262 | src/options.rs: impl Default for Options -- the concrete default values | 3.3 | 3.1 | 0.595 |
| walker |  | 2933 | 245 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 1, sub: 0, line: 6 } |  |  | 0.597 |
| walker |  | 2954 | 21 | Code::CodeKey { rung: Body, file: src/util/randomized_environment_offset.rs, decl: 1, sub: 0, line: 6 } |  |  | 0.597 |
| ns | 2991 |  | 152 | src/options.rs: DEFAULT_SHELL per platform, the Shell enum and its parser | 3.4 |  | 0.578 |
| walker |  | 2994 | 40 | Code::CodeKey { rung: Names, file: src/output/warnings.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.578 |
| walker |  | 3024 | 30 | Code::CodeKey { rung: Decl, file: src/output/warnings.rs, decl: 1, sub: 0, line: 7 } |  |  | 0.578 |
| walker |  | 3054 | 30 | Code::CodeKey { rung: Decl, file: src/output/warnings.rs, decl: 3, sub: 0, line: 20 } |  |  | 0.578 |
| ns | 3094 |  | 103 | src/options.rs: ExecutorKind (Raw / Shell / Mock) and its default | 3.5 | 3.4 | 0.564 |
| walker |  | 3106 | 52 | Code::CodeKey { rung: Decl, file: src/output/warnings.rs, decl: 2, sub: 0, line: 13 } |  |  | 0.565 |
| walker |  | 3116 | 10 | Code::CodeKey { rung: Doc, file: src/output/warnings.rs, decl: 2, sub: 0, line: 13 } |  |  | 0.565 |
| walker |  | 3156 | 40 | Code::CodeKey { rung: Names, file: src/util/min_max.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.565 |
| walker |  | 3170 | 14 | Code::CodeKey { rung: Doc, file: src/util/min_max.rs, decl: 1, sub: 0, line: 2 } |  |  | 0.565 |
| walker |  | 3186 | 16 | Code::CodeKey { rung: Doc, file: src/util/min_max.rs, decl: 2, sub: 0, line: 10 } |  |  | 0.565 |
| walker |  | 3228 | 42 | Code::CodeKey { rung: Names, file: src/util/units.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.565 |
| walker |  | 3270 | 42 | Code::CodeKey { rung: Decl, file: src/util/units.rs, decl: 4, sub: 0, line: 16 } |  |  | 0.565 |
| walker |  | 3313 | 43 | Code::CodeKey { rung: Decl, file: src/util/units.rs, decl: 3, sub: 0, line: 9 } |  |  | 0.565 |
| ns | 3318 |  | 224 | src/options.rs: OutputStyleOption and SortOrder vocabularies | 3.6 |  | 0.541 |
| walker |  | 3320 | 7 | Code::CodeKey { rung: Doc, file: src/util/units.rs, decl: 3, sub: 0, line: 9 } |  |  | 0.541 |
| walker |  | 3330 | 10 | Code::CodeKey { rung: Doc, file: src/util/units.rs, decl: 2, sub: 0, line: 6 } |  |  | 0.541 |
| walker |  | 3340 | 10 | Code::CodeKey { rung: Doc, file: src/util/units.rs, decl: 5, sub: 0, line: 18 } |  |  | 0.541 |
| walker |  | 3355 | 15 | Code::CodeKey { rung: Doc, file: src/util/units.rs, decl: 6, sub: 0, line: 27 } |  |  | 0.541 |
| walker |  | 3395 | 40 | Code::CodeKey { rung: Decl, file: src/export/tests.rs, decl: 1, sub: 0, line: 9 } |  |  | 0.541 |
| walker |  | 3478 | 83 | Code::CodeKey { rung: Names, file: src/command.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.541 |
| walker |  | 3510 | 32 | Code::CodeKey { rung: Decl, file: src/command.rs, decl: 16, sub: 0, line: 387 } |  |  | 0.541 |
| ns | 3560 |  | 242 | src/options.rs: CommandInputPolicy and CommandOutputPolicy | 3.7 |  | 0.518 |
| walker |  | 3589 | 79 | Code::CodeKey { rung: Decl, file: src/command.rs, decl: 12, sub: 0, line: 136 } |  |  | 0.518 |
| walker |  | 3599 | 10 | Code::CodeKey { rung: Body, file: src/command.rs, decl: 14, sub: 0, line: 250 } |  |  | 0.518 |
| walker |  | 3614 | 15 | Code::CodeKey { rung: Doc, file: src/command.rs, decl: 11, sub: 0, line: 134 } |  |  | 0.518 |
| walker |  | 3725 | 111 | Code::CodeKey { rung: Decl, file: src/command.rs, decl: 1, sub: 0, line: 21 } |  |  | 0.518 |
| walker |  | 3738 | 13 | Code::CodeKey { rung: Doc, file: src/command.rs, decl: 1, sub: 0, line: 21 } |  |  | 0.518 |
| ns | 3800 |  | 240 | src/options.rs: CmdFailureAction and RunBounds (default min = 10) | 3.8 |  | 0.500 |
| walker |  | 3918 | 180 | Code::CodeKey { rung: Decl, file: src/command.rs, decl: 2, sub: 0, line: 33 } |  |  | 0.501 |
| walker |  | 3973 | 55 | Code::CodeKey { rung: Decl, file: src/command.rs, decl: 4, sub: 0, line: 42 } |  |  | 0.501 |
| walker |  | 4016 | 43 | Code::CodeKey { rung: Names, file: src/export/json.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.501 |
| walker |  | 4024 | 8 | Code::CodeKey { rung: Decl, file: src/export/json.rs, decl: 2, sub: 0, line: 16 } |  |  | 0.501 |
| walker |  | 4037 | 13 | Code::CodeKey { rung: Decl, file: src/export/json.rs, decl: 3, sub: 0, line: 19 } |  |  | 0.501 |
| walker |  | 4092 | 55 | Code::CodeKey { rung: Decl, file: src/export/json.rs, decl: 4, sub: 0, line: 20 } |  |  | 0.501 |
| walker |  | 4119 | 27 | Code::CodeKey { rung: Decl, file: src/export/json.rs, decl: 1, sub: 0, line: 11 } |  |  | 0.501 |
| walker |  | 4128 | 9 | Code::CodeKey { rung: Body, file: src/command.rs, decl: 9, sub: 0, line: 96 } |  |  | 0.501 |
| walker |  | 4172 | 44 | Code::CodeKey { rung: Names, file: src/util/mod.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.501 |
| ns | 4189 |  | 389 | src/error.rs: the complete OptionsError variant set with messages | 3.10 |  | 0.484 |
| walker |  | 4218 | 46 | Code::CodeKey { rung: Names, file: src/util/exit_code.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.484 |
| walker |  | 4227 | 9 | Code::CodeKey { rung: Decl, file: src/util/exit_code.rs, decl: 1, sub: 0, line: 3 } |  |  | 0.484 |
| walker |  | 4237 | 10 | Code::CodeKey { rung: Decl, file: src/util/exit_code.rs, decl: 2, sub: 0, line: 19 } |  |  | 0.484 |
| walker |  | 4245 | 8 | Code::CodeKey { rung: Body, file: src/util/exit_code.rs, decl: 2, sub: 0, line: 19 } |  |  | 0.484 |
| walker |  | 4260 | 15 | Code::CodeKey { rung: Doc, file: src/cli.rs, decl: 2, sub: 0, line: 18 } |  |  | 0.486 |
| walker |  | 4286 | 26 | Code::CodeKey { rung: Body, file: src/timer/wall_clock_timer.rs, decl: 3, sub: 0, line: 10 } |  |  | 0.486 |
| walker |  | 4313 | 27 | Code::CodeKey { rung: Body, file: src/export/orgmode.rs, decl: 4, sub: 0, line: 16 } |  |  | 0.486 |
| walker |  | 4386 | 73 | Code::CodeKey { rung: Body, file: src/main.rs, decl: 2, sub: 0, line: 53 } |  |  | 0.505 |
| ns | 4407 |  | 218 | src/error.rs: the complete ParameterScanError variant set | 3.11 | 3.10 | 0.526 |
| walker |  | 4449 | 63 | Code::CodeKey { rung: Doc, file: src/outlier_detection.rs, decl: 1, sub: 0, line: 13 } |  |  | 0.527 |
| ns | 4627 |  | 220 | src/benchmark/executor.rs: the Executor trait | 4.1 |  | 0.514 |
| ns | 4752 |  | 125 | The three Executor implementations and their state | 4.2 | 4.1 | 0.505 |
| walker |  | 4820 | 371 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 6, sub: 0, line: 36 } |  |  | 0.548 |
| walker |  | 4882 | 62 | Code::CodeKey { rung: Names, file: src/export/markup.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.548 |
| walker |  | 4895 | 13 | Code::CodeKey { rung: Decl, file: src/export/markup.rs, decl: 10, sub: 0, line: 108 } |  |  | 0.548 |
| ns | 4904 |  | 152 | BenchmarkIteration and the $HYPERFINE_ITERATION values | 4.3 |  | 0.538 |
| walker |  | 4912 | 17 | Code::CodeKey { rung: Decl, file: src/export/markup.rs, decl: 1, sub: 0, line: 10 } |  |  | 0.538 |
| walker |  | 4965 | 53 | Code::CodeKey { rung: Decl, file: src/export/markup.rs, decl: 11, sub: 0, line: 109 } |  |  | 0.538 |
| ns | 5072 |  | 168 | The shared command runner: stdio wiring and injected environment variables | 4.4 | 4.3 | 0.529 |
| walker |  | 5121 | 156 | Code::CodeKey { rung: Decl, file: src/export/markup.rs, decl: 2, sub: 0, line: 15 } |  |  | 0.529 |
| walker |  | 5130 | 9 | Code::CodeKey { rung: Body, file: src/export/markup.rs, decl: 6, sub: 0, line: 87 } |  |  | 0.529 |
| walker |  | 5139 | 9 | Code::CodeKey { rung: Body, file: src/export/markup.rs, decl: 7, sub: 0, line: 91 } |  |  | 0.529 |
| walker |  | 5213 | 74 | Markdown::Section { file: README.md, section_index: 15, keeps_default_concavity: false } |  |  | 0.529 |
| ns | 5284 |  | 212 | Non-zero exit handling and the failure message users actually see | 4.5 | 4.4 | 0.520 |
| walker |  | 5295 | 82 | Markdown::Section { file: README.md, section_index: 13, keeps_default_concavity: false } |  |  | 0.520 |
| walker |  | 5361 | 66 | Code::CodeKey { rung: Names, file: src/parameter/mod.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.520 |
| walker |  | 5393 | 32 | Code::CodeKey { rung: Decl, file: src/parameter/mod.rs, decl: 1, sub: 0, line: 7 } |  |  | 0.520 |
| walker |  | 5427 | 34 | Code::CodeKey { rung: Decl, file: src/parameter/mod.rs, decl: 2, sub: 0, line: 13 } |  |  | 0.520 |
| ns | 5481 |  | 197 | Shell-spawning calibration: 50 probe runs, and where the overhead is subtracted | 4.6 | 4.2 | 0.512 |
| walker |  | 5495 | 68 | Code::CodeKey { rung: Names, file: src/util/number.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.512 |
| walker |  | 5514 | 19 | Code::CodeKey { rung: Decl, file: src/util/number.rs, decl: 6, sub: 0, line: 30 } |  |  | 0.512 |
| walker |  | 5534 | 20 | Code::CodeKey { rung: Decl, file: src/util/number.rs, decl: 4, sub: 0, line: 24 } |  |  | 0.512 |
| walker |  | 5564 | 30 | Code::CodeKey { rung: Decl, file: src/util/number.rs, decl: 2, sub: 0, line: 15 } |  |  | 0.512 |
| walker |  | 5605 | 41 | Code::CodeKey { rung: Decl, file: src/util/number.rs, decl: 8, sub: 0, line: 36 } |  |  | 0.512 |
| walker |  | 5656 | 51 | Code::CodeKey { rung: Decl, file: src/util/number.rs, decl: 1, sub: 0, line: 8 } |  |  | 0.512 |
| walker |  | 5666 | 10 | Code::CodeKey { rung: Body, file: src/util/number.rs, decl: 5, sub: 0, line: 25 } |  |  | 0.512 |
| walker |  | 5676 | 10 | Code::CodeKey { rung: Body, file: src/util/number.rs, decl: 7, sub: 0, line: 31 } |  |  | 0.512 |
| ns | 5679 |  | 198 | src/benchmark/scheduler.rs: Scheduler state and executor selection | 4.7 |  | 0.505 |
| walker |  | 5915 | 239 | Code::CodeKey { rung: Names, file: src/options.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.509 |
| walker |  | 5922 | 7 | Code::CodeKey { rung: Decl, file: src/options.rs, decl: 2, sub: 0, line: 19 } |  |  | 0.510 |
| walker |  | 5932 | 10 | Code::CodeKey { rung: Decl, file: src/options.rs, decl: 1, sub: 0, line: 16 } |  |  | 0.511 |
| ns | 5933 |  | 254 | run_benchmarks(): reference command first, calibrate once, export after each benchmark | 4.8 | 4.7 | 0.498 |
| walker |  | 5948 | 16 | Code::CodeKey { rung: Decl, file: src/options.rs, decl: 4, sub: 0, line: 32 } |  |  | 0.498 |
| walker |  | 5964 | 16 | Code::CodeKey { rung: Decl, file: src/options.rs, decl: 27, sub: 0, line: 251 } |  |  | 0.499 |
| walker |  | 5982 | 18 | Code::CodeKey { rung: Decl, file: src/options.rs, decl: 15, sub: 0, line: 116 } |  |  | 0.499 |
| walker |  | 6000 | 18 | Code::CodeKey { rung: Decl, file: src/options.rs, decl: 24, sub: 0, line: 191 } |  |  | 0.501 |
| walker |  | 6029 | 29 | Code::CodeKey { rung: Decl, file: src/options.rs, decl: 18, sub: 0, line: 132 } |  |  | 0.501 |
| walker |  | 6059 | 30 | Code::CodeKey { rung: Decl, file: src/options.rs, decl: 6, sub: 0, line: 38 } |  |  | 0.501 |
| walker |  | 6092 | 33 | Code::CodeKey { rung: Decl, file: src/options.rs, decl: 21, sub: 0, line: 164 } |  |  | 0.501 |
| walker |  | 6129 | 37 | Code::CodeKey { rung: Decl, file: src/options.rs, decl: 13, sub: 0, line: 101 } |  |  | 0.503 |
| walker |  | 6168 | 39 | Code::CodeKey { rung: Decl, file: src/options.rs, decl: 23, sub: 0, line: 184 } |  |  | 0.512 |
| ns | 6208 |  | 275 | src/benchmark/mod.rs: Benchmark struct, MIN_EXECUTION_TIME, and the complete method roster | 4.10 |  | 0.501 |
| walker |  | 6221 | 53 | Code::CodeKey { rung: Decl, file: src/options.rs, decl: 8, sub: 0, line: 47 } |  |  | 0.501 |
| walker |  | 6276 | 55 | Code::CodeKey { rung: Decl, file: src/options.rs, decl: 14, sub: 0, line: 108 } |  |  | 0.505 |
| walker |  | 6339 | 63 | Code::CodeKey { rung: Decl, file: src/options.rs, decl: 29, sub: 0, line: 275 } |  |  | 0.505 |
| walker |  | 6403 | 64 | Code::CodeKey { rung: Decl, file: src/options.rs, decl: 3, sub: 0, line: 23 } |  |  | 0.518 |
| walker |  | 6414 | 11 | Code::CodeKey { rung: Doc, file: src/options.rs, decl: 14, sub: 0, line: 108 } |  |  | 0.520 |
| ns | 6418 |  | 210 | How the number of runs is decided | 4.11 | 4.10 | 0.510 |
| walker |  | 6483 | 69 | Code::CodeKey { rung: Decl, file: src/options.rs, decl: 17, sub: 0, line: 122 } |  |  | 0.516 |
| walker |  | 6495 | 12 | Code::CodeKey { rung: Doc, file: src/options.rs, decl: 3, sub: 0, line: 23 } |  |  | 0.520 |
| walker |  | 6507 | 12 | Code::CodeKey { rung: Doc, file: src/options.rs, decl: 9, sub: 0, line: 49 } |  |  | 0.520 |
| walker |  | 6603 | 96 | Code::CodeKey { rung: Decl, file: src/options.rs, decl: 11, sub: 0, line: 70 } |  |  | 0.537 |
| walker |  | 6615 | 12 | Code::CodeKey { rung: Doc, file: src/options.rs, decl: 11, sub: 0, line: 70 } |  |  | 0.541 |
| ns | 6626 |  | 208 | The three warning triggers inside Benchmark::run | 4.12 | 4.10 | 0.533 |
| walker |  | 6743 | 128 | Code::CodeKey { rung: Decl, file: src/options.rs, decl: 20, sub: 0, line: 148 } |  |  | 0.558 |
| walker |  | 6756 | 13 | Code::CodeKey { rung: Doc, file: src/options.rs, decl: 20, sub: 0, line: 148 } |  |  | 0.563 |
| ns | 6779 |  | 153 | src/outlier_detection.rs: the modified Z-score method and its threshold | 4.13 | 4.12 | 0.565 |
| walker |  | 6905 | 149 | Code::CodeKey { rung: Decl, file: src/options.rs, decl: 12, sub: 0, line: 83 } |  |  | 0.591 |
| walker |  | 6913 | 8 | Code::CodeKey { rung: Doc, file: src/options.rs, decl: 12, sub: 0, line: 83 } |  |  | 0.594 |
| walker |  | 6925 | 12 | Code::CodeKey { rung: Body, file: src/command.rs, decl: 7, sub: 0, line: 77 } |  |  | 0.594 |
| walker |  | 7003 | 78 | Code::CodeKey { rung: Names, file: src/output/format.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.594 |
| walker |  | 7021 | 18 | Code::CodeKey { rung: Doc, file: src/output/format.rs, decl: 2, sub: 0, line: 11 } |  |  | 0.594 |
| walker |  | 7039 | 18 | Code::CodeKey { rung: Doc, file: src/output/format.rs, decl: 3, sub: 0, line: 18 } |  |  | 0.594 |
| walker |  | 7066 | 27 | Code::CodeKey { rung: Body, file: src/output/format.rs, decl: 1, sub: 0, line: 5 } |  |  | 0.594 |
| ns | 7075 |  | 296 | src/benchmark/benchmark_result.rs: the complete BenchmarkResult field set (= the JSON export schema) | 5.1 |  | 0.583 |
| walker |  | 7108 | 42 | Code::CodeKey { rung: Body, file: src/timer/wall_clock_timer.rs, decl: 4, sub: 0, line: 16 } |  |  | 0.583 |
| walker |  | 7188 | 80 | Code::CodeKey { rung: Names, file: src/timer/unix_timer.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.583 |
| walker |  | 7203 | 15 | Code::CodeKey { rung: Decl, file: src/timer/unix_timer.rs, decl: 2, sub: 0, line: 18 } |  |  | 0.583 |
| ns | 7215 |  | 140 | src/benchmark/timing_result.rs: TimingResult in full | 5.2 | 5.1 | 0.576 |
| walker |  | 7245 | 42 | Code::CodeKey { rung: Decl, file: src/timer/unix_timer.rs, decl: 3, sub: 0, line: 22 } |  |  | 0.576 |
| walker |  | 7317 | 72 | Code::CodeKey { rung: Decl, file: src/timer/unix_timer.rs, decl: 1, sub: 0, line: 9 } |  |  | 0.576 |
| walker |  | 7344 | 27 | Code::CodeKey { rung: Body, file: src/timer/unix_timer.rs, decl: 4, sub: 0, line: 23 } |  |  | 0.576 |
| ns | 7378 |  | 163 | src/benchmark/relative_speed.rs: annotated result type and the complete public function set | 5.3 |  | 0.568 |
| walker |  | 7388 | 44 | Code::CodeKey { rung: Body, file: src/util/min_max.rs, decl: 1, sub: 0, line: 2 } |  |  | 0.568 |
| walker |  | 7432 | 44 | Code::CodeKey { rung: Body, file: src/util/min_max.rs, decl: 2, sub: 0, line: 10 } |  |  | 0.568 |
| walker |  | 7453 | 21 | Code::CodeKey { rung: Body, file: src/export/asciidoc.rs, decl: 5, sub: 0, line: 26 } |  |  | 0.568 |
| walker |  | 7467 | 14 | Code::CodeKey { rung: Doc, file: src/timer/unix_timer.rs, decl: 6, sub: 0, line: 41 } |  |  | 0.568 |
| ns | 7536 |  | 158 | src/export/mod.rs: the ExportType enum -- the five output formats | 5.4 |  | 0.560 |
| walker |  | 7554 | 87 | Code::CodeKey { rung: Names, file: src/timer/windows_timer.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.560 |
| walker |  | 7567 | 13 | Code::CodeKey { rung: Decl, file: src/timer/windows_timer.rs, decl: 3, sub: 0, line: 49 } |  |  | 0.560 |
| walker |  | 7584 | 17 | Code::CodeKey { rung: Decl, file: src/timer/windows_timer.rs, decl: 7, sub: 0, line: 124 } |  |  | 0.560 |
| walker |  | 7634 | 50 | Code::CodeKey { rung: Decl, file: src/timer/windows_timer.rs, decl: 4, sub: 0, line: 53 } |  |  | 0.560 |
| walker |  | 7670 | 36 | Code::CodeKey { rung: Body, file: src/timer/windows_timer.rs, decl: 8, sub: 0, line: 125 } |  |  | 0.560 |
| ns | 7698 |  | 162 | src/export/mod.rs: the Exporter trait and ExportManager | 5.5 | 5.4 | 0.552 |
| walker |  | 7717 | 47 | Code::CodeKey { rung: Body, file: src/output/format.rs, decl: 2, sub: 0, line: 11 } |  |  | 0.552 |
| walker |  | 7768 | 51 | Code::CodeKey { rung: Doc, file: src/output/format.rs, decl: 1, sub: 0, line: 5 } |  |  | 0.552 |
| ns | 7835 |  | 137 | Flag-to-exporter wiring and the '-' means stdout convention | 5.6 | 5.5 | 0.548 |
| walker |  | 8027 | 259 | Code::CodeKey { rung: Decl, file: src/options.rs, decl: 26, sub: 0, line: 198 } |  |  | 0.559 |
| walker |  | 8040 | 13 | Code::CodeKey { rung: Doc, file: src/options.rs, decl: 26, sub: 0, line: 198 } |  |  | 0.561 |
| ns | 8051 |  | 216 | src/export/markup.rs: the shared table shape and the blanket Exporter impl | 5.7 | 5.5 | 0.560 |
| ns | 8267 |  | 216 | The five exporter types, and the CSV column set | 5.8 | 5.4 | 0.557 |
| walker |  | 8359 | 319 | Code::CodeKey { rung: Decl, file: src/options.rs, decl: 26, sub: 1, line: 198 } |  |  | 0.580 |
| walker |  | 8463 | 104 | Code::CodeKey { rung: Doc, file: src/outlier_detection.rs, decl: 2, sub: 0, line: 21 } |  |  | 0.583 |
| ns | 8524 |  | 257 | src/output/format.rs: automatic time-unit selection, and the Unit type | 5.9 |  | 0.578 |
| walker |  | 8570 | 107 | Code::CodeKey { rung: Names, file: src/parameter/range_step.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.579 |
| walker |  | 8604 | 34 | Code::CodeKey { rung: Decl, file: src/parameter/range_step.rs, decl: 4, sub: 0, line: 40 } |  |  | 0.579 |
| walker |  | 8642 | 38 | Code::CodeKey { rung: Decl, file: src/parameter/range_step.rs, decl: 3, sub: 0, line: 33 } |  |  | 0.579 |
| ns | 8661 |  | 137 | src/output/warnings.rs: the complete Warnings set | 5.10 | 4.12 | 0.585 |
| walker |  | 8702 | 60 | Code::CodeKey { rung: Decl, file: src/parameter/range_step.rs, decl: 6, sub: 0, line: 62 } |  |  | 0.585 |
| walker |  | 8801 | 99 | Code::CodeKey { rung: Decl, file: src/parameter/range_step.rs, decl: 1, sub: 0, line: 7 } |  |  | 0.585 |
| ns | 8869 |  | 208 | src/command.rs: the Command type and its complete method roster | 6.1 |  | 0.591 |
| walker |  | 8904 | 103 | Code::CodeKey { rung: Decl, file: src/parameter/range_step.rs, decl: 2, sub: 0, line: 19 } |  |  | 0.591 |
| walker |  | 8922 | 18 | Code::CodeKey { rung: Body, file: src/parameter/range_step.rs, decl: 8, sub: 0, line: 75 } |  |  | 0.591 |
| ns | 9015 |  | 146 | How {param} placeholders are substituted, and why naively | 6.2 | 6.1 | 0.587 |
| walker |  | 9036 | 114 | Code::CodeKey { rung: Names, file: src/output/progress_bar.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.587 |
| walker |  | 9043 | 7 | Code::CodeKey { rung: Decl, file: src/output/progress_bar.rs, decl: 2, sub: 0, line: 9 } |  |  | 0.587 |
| walker |  | 9053 | 10 | Code::CodeKey { rung: Decl, file: src/output/progress_bar.rs, decl: 1, sub: 0, line: 6 } |  |  | 0.587 |
| walker |  | 9064 | 11 | Code::CodeKey { rung: Doc, file: src/output/progress_bar.rs, decl: 3, sub: 0, line: 13 } |  |  | 0.587 |
| walker |  | 9125 | 61 | Code::CodeKey { rung: Body, file: src/parameter/mod.rs, decl: 3, sub: 0, line: 14 } |  |  | 0.587 |
| ns | 9202 |  | 187 | src/command.rs: Commands and its complete method roster, with the three construction modes | 6.3 |  | 0.584 |
| walker |  | 9241 | 116 | Code::CodeKey { rung: Names, file: src/timer/mod.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.584 |
| walker |  | 9328 | 87 | Code::CodeKey { rung: Decl, file: src/timer/mod.rs, decl: 2, sub: 0, line: 41 } |  |  | 0.584 |
| walker |  | 9343 | 15 | Code::CodeKey { rung: Doc, file: src/timer/mod.rs, decl: 2, sub: 0, line: 41 } |  |  | 0.584 |
| walker |  | 9358 | 15 | Code::CodeKey { rung: Doc, file: src/timer/mod.rs, decl: 4, sub: 0, line: 83 } |  |  | 0.584 |
| walker |  | 9370 | 12 | Code::CodeKey { rung: Doc, file: src/timer/mod.rs, decl: 3, sub: 0, line: 52 } |  |  | 0.584 |
| walker |  | 9389 | 19 | Code::CodeKey { rung: Doc, file: src/timer/unix_timer.rs, decl: 7, sub: 0, line: 71 } |  |  | 0.584 |
| ns | 9431 |  | 229 | src/parameter/: ParameterValue, RangeStep and its 100_000 cap, tokenize() | 6.4 |  | 0.590 |
| walker |  | 9528 | 139 | Code::CodeKey { rung: Names, file: src/export/mod.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.590 |
| walker |  | 9547 | 19 | Code::CodeKey { rung: Decl, file: src/export/mod.rs, decl: 4, sub: 0, line: 56 } |  |  | 0.590 |
| ns | 9581 |  | 150 | build.rs: shell completions generated from the same clap command | 7.1 | 2.1 | 0.585 |
| walker |  | 9585 | 38 | Code::CodeKey { rung: Decl, file: src/export/mod.rs, decl: 6, sub: 0, line: 67 } |  |  | 0.587 |
| walker |  | 9600 | 15 | Code::CodeKey { rung: Decl, file: src/export/mod.rs, decl: 2, sub: 0, line: 46 } |  |  | 0.588 |
| walker |  | 9613 | 13 | Code::CodeKey { rung: Doc, file: src/export/mod.rs, decl: 6, sub: 0, line: 67 } |  |  | 0.589 |
| walker |  | 9692 | 79 | Code::CodeKey { rung: Decl, file: src/export/mod.rs, decl: 7, sub: 0, line: 73 } |  |  | 0.589 |
| walker |  | 9736 | 44 | Code::CodeKey { rung: Decl, file: src/export/mod.rs, decl: 8, sub: 0, line: 76 } |  |  | 0.589 |
| walker |  | 9751 | 15 | Code::CodeKey { rung: Doc, file: src/export/mod.rs, decl: 9, sub: 0, line: 103 } |  |  | 0.589 |
| walker |  | 9777 | 26 | Code::CodeKey { rung: Decl, file: src/export/mod.rs, decl: 5, sub: 0, line: 61 } |  |  | 0.589 |
| ns | 9790 |  | 209 | .github/workflows/CICD.yml: the complete job set and the commands each runs | 7.2 |  | 0.583 |
| walker |  | 9905 | 128 | Code::CodeKey { rung: Decl, file: src/export/mod.rs, decl: 1, sub: 0, line: 27 } |  |  | 0.595 |
| walker |  | 9922 | 17 | Code::CodeKey { rung: Doc, file: src/export/mod.rs, decl: 1, sub: 0, line: 27 } |  |  | 0.599 |
| walker |  | 9949 | 27 | Code::CodeKey { rung: Doc, file: src/export/mod.rs, decl: 8, sub: 0, line: 76 } |  |  | 0.599 |
| walker |  | 9957 | 8 | Code::CodeKey { rung: Doc, file: src/export/mod.rs, decl: 2, sub: 0, line: 46 } |  |  | 0.600 |
| ns | 9982 |  | 192 | tests/: the shared harness helpers and the debug-mode test idiom | 7.3 |  | 0.594 |
