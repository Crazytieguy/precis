Score(3000)=0.544 I=0.822 C=0.360 ns_rows≤3K=21/56 grid(1000/1442/2080/3000/4327/6240/9000)=0.785/0.716/0.597/0.544/0.595/0.521/0.559

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
| walker |  | 204 | 28 | Fs::DirListing { dir: src/util } |  |  | 0.551 |
| ns | 217 |  | 76 | Cargo.toml: version 1.20.0, edition, MSRV, build script | 1.3 | 1.1 | 0.470 |
| walker |  | 239 | 35 | Fs::DirListing { dir: src/export } |  |  | 0.481 |
| walker |  | 247 | 8 | Fs::DirListing { dir: .github } |  |  | 0.482 |
| walker |  | 252 | 5 | Fs::DirListing { dir: .github/workflows } |  |  | 0.482 |
| ns | 261 |  | 44 | src/ listing: the flat modules and six subdirectories | 1.4 |  | 0.523 |
| walker |  | 321 | 69 | Markdown::ReadmeHeadline { file: README.md } |  |  | 0.523 |
| walker |  | 341 | 20 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.523 |
| ns | 343 |  | 82 | README feature list, first five bullets (rest elided) | 1.5 |  | 0.473 |
| ns | 409 |  | 66 | README feature list, remaining bullets | 1.6 | 1.5 | 0.449 |
| walker |  | 461 | 120 | Code::CodeKey { rung: Names, file: src/main.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.465 |
| ns | 471 |  | 62 | src/benchmark/ and src/export/ listings (complete) | 1.7 |  | 0.434 |
| walker |  | 474 | 13 | Code::CodeKey { rung: ModuleDoc, file: src/util/units.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.434 |
| walker |  | 489 | 15 | Code::CodeKey { rung: Names, file: src/util/randomized_environment_offset.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.434 |
| walker |  | 507 | 18 | Code::CodeKey { rung: Names, file: src/export/tests.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.434 |
| walker |  | 526 | 19 | Code::CodeKey { rung: Names, file: src/parameter/tokenize.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.434 |
| ns | 549 |  | 78 | src/output/, src/parameter/, src/timer/, src/util/ listings (complete) | 1.8 |  | 0.470 |
| ns | 643 |  | 94 | src/main.rs: module declarations and the types the entry point imports | 1.9 |  | 0.484 |
| walker |  | 698 | 172 | Toml::Identity { file: Cargo.toml } |  |  | 0.848 |
| walker |  | 733 | 35 | Toml::Operational { file: Cargo.toml } |  |  | 0.848 |
| walker |  | 774 | 41 | Code::CodeKey { rung: Names, file: src/cli.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.848 |
| walker |  | 816 | 42 | Code::CodeKey { rung: Decl, file: src/cli.rs, decl: 1, sub: 0, line: 8 } |  |  | 0.848 |
| walker |  | 837 | 21 | Fs::DirListing { dir: tests } |  |  | 0.851 |
| ns | 899 |  | 256 | src/main.rs: the run() pipeline | 1.10 | 1.9 | 0.745 |
| ns | 983 |  | 84 | src/main.rs: main() error reporting and exit code | 1.11 | 1.10 | 0.713 |
| walker |  | 986 | 149 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: true } |  |  | 0.785 |
| walker |  | 1011 | 25 | Code::CodeKey { rung: Names, file: src/output/mod.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.785 |
| walker |  | 1036 | 25 | Code::CodeKey { rung: Names, file: src/timer/wall_clock_timer.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.785 |
| walker |  | 1048 | 12 | Code::CodeKey { rung: Decl, file: src/timer/wall_clock_timer.rs, decl: 1, sub: 0, line: 5 } |  |  | 0.785 |
| ns | 1098 |  | 115 | tests/, doc/, scripts/ and .github/ listings (complete) | 1.12 |  | 0.745 |
| walker |  | 1100 | 52 | Code::CodeKey { rung: Names, file: src/outlier_detection.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.745 |
| walker |  | 1129 | 29 | Code::CodeKey { rung: Names, file: src/export/csv.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.745 |
| walker |  | 1137 | 8 | Code::CodeKey { rung: Decl, file: src/export/csv.rs, decl: 1, sub: 0, line: 12 } |  |  | 0.745 |
| walker |  | 1152 | 15 | Code::CodeKey { rung: Decl, file: src/export/csv.rs, decl: 2, sub: 0, line: 15 } |  |  | 0.745 |
| walker |  | 1182 | 30 | Code::CodeKey { rung: Names, file: src/export/markdown.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.745 |
| walker |  | 1190 | 8 | Code::CodeKey { rung: Decl, file: src/export/markdown.rs, decl: 1, sub: 0, line: 5 } |  |  | 0.745 |
| walker |  | 1222 | 32 | Code::CodeKey { rung: Names, file: src/export/orgmode.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.745 |
| walker |  | 1230 | 8 | Code::CodeKey { rung: Decl, file: src/export/orgmode.rs, decl: 1, sub: 0, line: 4 } |  |  | 0.745 |
| ns | 1235 |  | 137 | src/cli.rs: get_cli_arguments() and the build_command() clap header | 2.1 |  | 0.716 |
| walker |  | 1294 | 64 | Code::CodeKey { rung: Names, file: src/error.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.716 |
| walker |  | 1318 | 24 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 4, sub: 0, line: 30 } |  |  | 0.716 |
| walker |  | 1343 | 25 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 2, sub: 0, line: 24 } |  |  | 0.716 |
| walker |  | 1379 | 36 | Code::CodeKey { rung: Names, file: src/export/asciidoc.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.716 |
| walker |  | 1387 | 8 | Code::CodeKey { rung: Decl, file: src/export/asciidoc.rs, decl: 1, sub: 0, line: 4 } |  |  | 0.716 |
| walker |  | 1422 | 35 | Code::CodeKey { rung: Decl, file: src/timer/wall_clock_timer.rs, decl: 2, sub: 0, line: 9 } |  |  | 0.716 |
| walker |  | 1462 | 40 | Code::CodeKey { rung: Names, file: src/output/warnings.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.716 |
| walker |  | 1492 | 30 | Code::CodeKey { rung: Decl, file: src/output/warnings.rs, decl: 1, sub: 0, line: 7 } |  |  | 0.717 |
| walker |  | 1522 | 30 | Code::CodeKey { rung: Decl, file: src/output/warnings.rs, decl: 3, sub: 0, line: 20 } |  |  | 0.717 |
| walker |  | 1562 | 40 | Code::CodeKey { rung: Names, file: src/util/min_max.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.717 |
| walker |  | 1604 | 42 | Code::CodeKey { rung: Names, file: src/util/units.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.717 |
| ns | 1672 |  | 437 | Complete option roster: every Arg::new(...) in src/cli.rs | 2.2 | 2.1 | 0.644 |
| walker |  | 1687 | 83 | Code::CodeKey { rung: Names, file: src/command.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.644 |
| walker |  | 1719 | 32 | Code::CodeKey { rung: Decl, file: src/command.rs, decl: 16, sub: 0, line: 387 } |  |  | 0.644 |
| walker |  | 1798 | 79 | Code::CodeKey { rung: Decl, file: src/command.rs, decl: 12, sub: 0, line: 136 } |  |  | 0.644 |
| walker |  | 1841 | 43 | Code::CodeKey { rung: Names, file: src/export/json.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.644 |
| walker |  | 1849 | 8 | Code::CodeKey { rung: Decl, file: src/export/json.rs, decl: 2, sub: 0, line: 16 } |  |  | 0.644 |
| walker |  | 1862 | 13 | Code::CodeKey { rung: Decl, file: src/export/json.rs, decl: 3, sub: 0, line: 19 } |  |  | 0.614 |
| ns | 1862 |  | 190 | Complete short-alias roster for the CLI options | 2.3 | 2.2 | 0.614 |
| walker |  | 1906 | 44 | Code::CodeKey { rung: Names, file: src/util/mod.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.614 |
| walker |  | 1948 | 42 | Code::CodeKey { rung: Decl, file: src/util/units.rs, decl: 4, sub: 0, line: 16 } |  |  | 0.614 |
| walker |  | 1991 | 43 | Code::CodeKey { rung: Decl, file: src/util/units.rs, decl: 3, sub: 0, line: 9 } |  |  | 0.614 |
| walker |  | 1998 | 7 | Code::CodeKey { rung: Doc, file: src/util/units.rs, decl: 3, sub: 0, line: 9 } |  |  | 0.614 |
| ns | 2016 |  | 154 | Enumerated value sets, defaults, and the two hidden options | 2.4 | 2.2 | 0.597 |
| walker |  | 2093 | 95 | Code::CodeKey { rung: ModuleDoc, file: src/outlier_detection.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.597 |
| walker |  | 2139 | 46 | Code::CodeKey { rung: Names, file: src/util/exit_code.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.597 |
| ns | 2146 |  | 130 | Complete set of inter-option conflicts and requirements | 2.5 | 2.2 | 0.585 |
| walker |  | 2148 | 9 | Code::CodeKey { rung: Decl, file: src/util/exit_code.rs, decl: 1, sub: 0, line: 3 } |  |  | 0.585 |
| walker |  | 2158 | 10 | Code::CodeKey { rung: Decl, file: src/util/exit_code.rs, decl: 2, sub: 0, line: 19 } |  |  | 0.585 |
| walker |  | 2173 | 15 | Code::CodeKey { rung: Doc, file: src/command.rs, decl: 11, sub: 0, line: 134 } |  |  | 0.585 |
| walker |  | 2181 | 8 | Code::CodeKey { rung: Body, file: src/util/exit_code.rs, decl: 2, sub: 0, line: 19 } |  |  | 0.585 |
| walker |  | 2252 | 71 | Toml::Config { file: Cargo.toml } |  |  | 0.585 |
| walker |  | 2304 | 52 | Code::CodeKey { rung: Decl, file: src/output/warnings.rs, decl: 2, sub: 0, line: 13 } |  |  | 0.585 |
| ns | 2366 |  | 220 | src/options.rs: the Options struct, run bounds through setup_command | 3.1 |  | 0.562 |
| walker |  | 2415 | 111 | Code::CodeKey { rung: Decl, file: src/command.rs, decl: 1, sub: 0, line: 21 } |  |  | 0.562 |
| walker |  | 2425 | 10 | Code::CodeKey { rung: Doc, file: src/output/warnings.rs, decl: 2, sub: 0, line: 13 } |  |  | 0.562 |
| walker |  | 2435 | 10 | Code::CodeKey { rung: Doc, file: src/util/units.rs, decl: 2, sub: 0, line: 6 } |  |  | 0.562 |
| walker |  | 2448 | 13 | Code::CodeKey { rung: Doc, file: src/command.rs, decl: 1, sub: 0, line: 21 } |  |  | 0.562 |
| walker |  | 2510 | 62 | Code::CodeKey { rung: Names, file: src/export/markup.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.563 |
| walker |  | 2523 | 13 | Code::CodeKey { rung: Decl, file: src/export/markup.rs, decl: 10, sub: 0, line: 108 } |  |  | 0.563 |
| walker |  | 2540 | 17 | Code::CodeKey { rung: Decl, file: src/export/markup.rs, decl: 1, sub: 0, line: 10 } |  |  | 0.563 |
| ns | 2577 |  | 211 | src/options.rs: remaining Options fields (output, sort, executor, I/O, time unit) | 3.2 | 3.1 | 0.544 |
| walker |  | 2596 | 56 | Fs::DirListing { dir: scripts } |  |  | 0.591 |
| walker |  | 2662 | 66 | Code::CodeKey { rung: Names, file: src/parameter/mod.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.591 |
| walker |  | 2694 | 32 | Code::CodeKey { rung: Decl, file: src/parameter/mod.rs, decl: 1, sub: 0, line: 7 } |  |  | 0.591 |
| walker |  | 2728 | 34 | Code::CodeKey { rung: Decl, file: src/parameter/mod.rs, decl: 2, sub: 0, line: 13 } |  |  | 0.591 |
| walker |  | 2796 | 68 | Code::CodeKey { rung: Names, file: src/util/number.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.591 |
| walker |  | 2815 | 19 | Code::CodeKey { rung: Decl, file: src/util/number.rs, decl: 6, sub: 0, line: 30 } |  |  | 0.591 |
| walker |  | 2835 | 20 | Code::CodeKey { rung: Decl, file: src/util/number.rs, decl: 4, sub: 0, line: 24 } |  |  | 0.591 |
| ns | 2839 |  | 262 | src/options.rs: impl Default for Options -- the concrete default values | 3.3 | 3.1 | 0.562 |
| walker |  | 2865 | 30 | Code::CodeKey { rung: Decl, file: src/util/number.rs, decl: 2, sub: 0, line: 15 } |  |  | 0.562 |
| walker |  | 2906 | 41 | Code::CodeKey { rung: Decl, file: src/util/number.rs, decl: 8, sub: 0, line: 36 } |  |  | 0.562 |
| walker |  | 2957 | 51 | Code::CodeKey { rung: Decl, file: src/util/number.rs, decl: 1, sub: 0, line: 8 } |  |  | 0.562 |
| ns | 2991 |  | 152 | src/options.rs: DEFAULT_SHELL per platform, the Shell enum and its parser | 3.4 |  | 0.544 |
| ns | 3094 |  | 103 | src/options.rs: ExecutorKind (Raw / Shell / Mock) and its default | 3.5 | 3.4 | 0.531 |
| walker |  | 3196 | 239 | Code::CodeKey { rung: Names, file: src/options.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.535 |
| walker |  | 3203 | 7 | Code::CodeKey { rung: Decl, file: src/options.rs, decl: 2, sub: 0, line: 19 } |  |  | 0.537 |
| walker |  | 3213 | 10 | Code::CodeKey { rung: Decl, file: src/options.rs, decl: 1, sub: 0, line: 16 } |  |  | 0.538 |
| walker |  | 3229 | 16 | Code::CodeKey { rung: Decl, file: src/options.rs, decl: 4, sub: 0, line: 32 } |  |  | 0.538 |
| walker |  | 3245 | 16 | Code::CodeKey { rung: Decl, file: src/options.rs, decl: 27, sub: 0, line: 251 } |  |  | 0.539 |
| walker |  | 3263 | 18 | Code::CodeKey { rung: Decl, file: src/options.rs, decl: 15, sub: 0, line: 116 } |  |  | 0.539 |
| walker |  | 3281 | 18 | Code::CodeKey { rung: Decl, file: src/options.rs, decl: 24, sub: 0, line: 191 } |  |  | 0.541 |
| walker |  | 3310 | 29 | Code::CodeKey { rung: Decl, file: src/options.rs, decl: 18, sub: 0, line: 132 } |  |  | 0.541 |
| ns | 3318 |  | 224 | src/options.rs: OutputStyleOption and SortOrder vocabularies | 3.6 |  | 0.518 |
| walker |  | 3340 | 30 | Code::CodeKey { rung: Decl, file: src/options.rs, decl: 6, sub: 0, line: 38 } |  |  | 0.518 |
| walker |  | 3373 | 33 | Code::CodeKey { rung: Decl, file: src/options.rs, decl: 21, sub: 0, line: 164 } |  |  | 0.518 |
| walker |  | 3410 | 37 | Code::CodeKey { rung: Decl, file: src/options.rs, decl: 13, sub: 0, line: 101 } |  |  | 0.522 |
| walker |  | 3449 | 39 | Code::CodeKey { rung: Decl, file: src/options.rs, decl: 23, sub: 0, line: 184 } |  |  | 0.535 |
| walker |  | 3502 | 53 | Code::CodeKey { rung: Decl, file: src/options.rs, decl: 8, sub: 0, line: 47 } |  |  | 0.535 |
| walker |  | 3557 | 55 | Code::CodeKey { rung: Decl, file: src/options.rs, decl: 14, sub: 0, line: 108 } |  |  | 0.535 |
| ns | 3560 |  | 242 | src/options.rs: CommandInputPolicy and CommandOutputPolicy | 3.7 |  | 0.514 |
| walker |  | 3620 | 63 | Code::CodeKey { rung: Decl, file: src/options.rs, decl: 29, sub: 0, line: 275 } |  |  | 0.514 |
| walker |  | 3684 | 64 | Code::CodeKey { rung: Decl, file: src/options.rs, decl: 3, sub: 0, line: 23 } |  |  | 0.533 |
| walker |  | 3753 | 69 | Code::CodeKey { rung: Decl, file: src/options.rs, decl: 17, sub: 0, line: 122 } |  |  | 0.541 |
| ns | 3800 |  | 240 | src/options.rs: CmdFailureAction and RunBounds (default min = 10) | 3.8 |  | 0.530 |
| walker |  | 3849 | 96 | Code::CodeKey { rung: Decl, file: src/options.rs, decl: 11, sub: 0, line: 70 } |  |  | 0.552 |
| walker |  | 3888 | 39 | Markdown::Section { file: README.md, section_index: 11, keeps_default_concavity: false } |  |  | 0.552 |
| walker |  | 4016 | 128 | Code::CodeKey { rung: Decl, file: src/options.rs, decl: 20, sub: 0, line: 148 } |  |  | 0.589 |
| walker |  | 4043 | 27 | Fs::DirListing { dir: src/benchmark } |  |  | 0.616 |
| walker |  | 4065 | 22 | Code::CodeKey { rung: Body, file: src/cli.rs, decl: 1, sub: 0, line: 8 } |  |  | 0.616 |
| walker |  | 4138 | 73 | Code::CodeKey { rung: Decl, file: src/export/markdown.rs, decl: 2, sub: 0, line: 8 } |  |  | 0.616 |
| ns | 4189 |  | 389 | src/error.rs: the complete OptionsError variant set with messages | 3.10 |  | 0.595 |
| walker |  | 4211 | 73 | Code::CodeKey { rung: Decl, file: src/export/orgmode.rs, decl: 2, sub: 0, line: 7 } |  |  | 0.595 |
| walker |  | 4289 | 78 | Code::CodeKey { rung: Names, file: src/output/format.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.595 |
| walker |  | 4369 | 80 | Code::CodeKey { rung: Names, file: src/timer/unix_timer.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.595 |
| walker |  | 4384 | 15 | Code::CodeKey { rung: Decl, file: src/timer/unix_timer.rs, decl: 2, sub: 0, line: 18 } |  |  | 0.595 |
| ns | 4407 |  | 218 | src/error.rs: the complete ParameterScanError variant set | 3.11 | 3.10 | 0.581 |
| walker |  | 4426 | 42 | Code::CodeKey { rung: Decl, file: src/timer/unix_timer.rs, decl: 3, sub: 0, line: 22 } |  |  | 0.581 |
| walker |  | 4498 | 72 | Code::CodeKey { rung: Decl, file: src/timer/unix_timer.rs, decl: 1, sub: 0, line: 9 } |  |  | 0.581 |
| ns | 4627 |  | 220 | src/benchmark/executor.rs: the Executor trait | 4.1 |  | 0.567 |
| walker |  | 4647 | 149 | Code::CodeKey { rung: Decl, file: src/options.rs, decl: 12, sub: 0, line: 83 } |  |  | 0.602 |
| walker |  | 4661 | 14 | Code::CodeKey { rung: Doc, file: src/util/min_max.rs, decl: 1, sub: 0, line: 2 } |  |  | 0.602 |
| walker |  | 4748 | 87 | Code::CodeKey { rung: Names, file: src/timer/windows_timer.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.602 |
| ns | 4752 |  | 125 | The three Executor implementations and their state | 4.2 | 4.1 | 0.592 |
| walker |  | 4761 | 13 | Code::CodeKey { rung: Decl, file: src/timer/windows_timer.rs, decl: 3, sub: 0, line: 49 } |  |  | 0.592 |
| walker |  | 4778 | 17 | Code::CodeKey { rung: Decl, file: src/timer/windows_timer.rs, decl: 7, sub: 0, line: 124 } |  |  | 0.592 |
| walker |  | 4828 | 50 | Code::CodeKey { rung: Decl, file: src/timer/windows_timer.rs, decl: 4, sub: 0, line: 53 } |  |  | 0.592 |
| walker |  | 4842 | 14 | Code::CodeKey { rung: Body, file: src/error.rs, decl: 3, sub: 0, line: 25 } |  |  | 0.592 |
| walker |  | 4856 | 14 | Code::CodeKey { rung: Body, file: src/error.rs, decl: 5, sub: 0, line: 31 } |  |  | 0.592 |
| walker |  | 4883 | 27 | Code::CodeKey { rung: Decl, file: src/export/json.rs, decl: 1, sub: 0, line: 11 } |  |  | 0.592 |
| walker |  | 4899 | 16 | Code::CodeKey { rung: Doc, file: src/util/min_max.rs, decl: 2, sub: 0, line: 10 } |  |  | 0.592 |
| ns | 4904 |  | 152 | BenchmarkIteration and the $HYPERFINE_ITERATION values | 4.3 |  | 0.581 |
| ns | 5072 |  | 168 | The shared command runner: stdio wiring and injected environment variables | 4.4 | 4.3 | 0.571 |
| walker |  | 5079 | 180 | Code::CodeKey { rung: Decl, file: src/command.rs, decl: 2, sub: 0, line: 33 } |  |  | 0.572 |
| walker |  | 5134 | 55 | Code::CodeKey { rung: Decl, file: src/command.rs, decl: 4, sub: 0, line: 42 } |  |  | 0.572 |
| walker |  | 5187 | 53 | Code::CodeKey { rung: Decl, file: src/export/markup.rs, decl: 11, sub: 0, line: 109 } |  |  | 0.572 |
| ns | 5284 |  | 212 | Non-zero exit handling and the failure message users actually see | 4.5 | 4.4 | 0.562 |
| walker |  | 5294 | 107 | Code::CodeKey { rung: Names, file: src/parameter/range_step.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.562 |
| walker |  | 5328 | 34 | Code::CodeKey { rung: Decl, file: src/parameter/range_step.rs, decl: 4, sub: 0, line: 40 } |  |  | 0.562 |
| walker |  | 5366 | 38 | Code::CodeKey { rung: Decl, file: src/parameter/range_step.rs, decl: 3, sub: 0, line: 33 } |  |  | 0.562 |
| walker |  | 5426 | 60 | Code::CodeKey { rung: Decl, file: src/parameter/range_step.rs, decl: 6, sub: 0, line: 62 } |  |  | 0.562 |
| ns | 5481 |  | 197 | Shell-spawning calibration: 50 probe runs, and where the overhead is subtracted | 4.6 | 4.2 | 0.554 |
| walker |  | 5525 | 99 | Code::CodeKey { rung: Decl, file: src/parameter/range_step.rs, decl: 1, sub: 0, line: 7 } |  |  | 0.554 |
| walker |  | 5628 | 103 | Code::CodeKey { rung: Decl, file: src/parameter/range_step.rs, decl: 2, sub: 0, line: 19 } |  |  | 0.554 |
| ns | 5679 |  | 198 | src/benchmark/scheduler.rs: Scheduler state and executor selection | 4.7 |  | 0.546 |
| walker |  | 5683 | 55 | Code::CodeKey { rung: Decl, file: src/export/csv.rs, decl: 3, sub: 0, line: 16 } |  |  | 0.546 |
| walker |  | 5738 | 55 | Code::CodeKey { rung: Decl, file: src/export/json.rs, decl: 4, sub: 0, line: 20 } |  |  | 0.546 |
| walker |  | 5756 | 18 | Code::CodeKey { rung: Doc, file: src/output/format.rs, decl: 2, sub: 0, line: 11 } |  |  | 0.546 |
| walker |  | 5774 | 18 | Code::CodeKey { rung: Doc, file: src/output/format.rs, decl: 3, sub: 0, line: 18 } |  |  | 0.546 |
| walker |  | 5888 | 114 | Code::CodeKey { rung: Names, file: src/output/progress_bar.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.546 |
| walker |  | 5895 | 7 | Code::CodeKey { rung: Decl, file: src/output/progress_bar.rs, decl: 2, sub: 0, line: 9 } |  |  | 0.546 |
| walker |  | 5905 | 10 | Code::CodeKey { rung: Decl, file: src/output/progress_bar.rs, decl: 1, sub: 0, line: 6 } |  |  | 0.546 |
| walker |  | 5916 | 11 | Code::CodeKey { rung: Doc, file: src/output/progress_bar.rs, decl: 3, sub: 0, line: 13 } |  |  | 0.546 |
| ns | 5933 |  | 254 | run_benchmarks(): reference command first, calibrate once, export after each benchmark | 4.8 | 4.7 | 0.533 |
| walker |  | 6032 | 116 | Code::CodeKey { rung: Names, file: src/timer/mod.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.533 |
| walker |  | 6119 | 87 | Code::CodeKey { rung: Decl, file: src/timer/mod.rs, decl: 2, sub: 0, line: 41 } |  |  | 0.533 |
| walker |  | 6134 | 15 | Code::CodeKey { rung: Doc, file: src/timer/mod.rs, decl: 2, sub: 0, line: 41 } |  |  | 0.533 |
| walker |  | 6149 | 15 | Code::CodeKey { rung: Doc, file: src/timer/mod.rs, decl: 4, sub: 0, line: 83 } |  |  | 0.533 |
| ns | 6208 |  | 275 | src/benchmark/mod.rs: Benchmark struct, MIN_EXECUTION_TIME, and the complete method roster | 4.10 |  | 0.521 |
| walker |  | 6288 | 139 | Code::CodeKey { rung: Names, file: src/export/mod.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.521 |
| walker |  | 6307 | 19 | Code::CodeKey { rung: Decl, file: src/export/mod.rs, decl: 4, sub: 0, line: 56 } |  |  | 0.521 |
| walker |  | 6345 | 38 | Code::CodeKey { rung: Decl, file: src/export/mod.rs, decl: 6, sub: 0, line: 67 } |  |  | 0.521 |
| walker |  | 6360 | 15 | Code::CodeKey { rung: Decl, file: src/export/mod.rs, decl: 2, sub: 0, line: 46 } |  |  | 0.521 |
| walker |  | 6373 | 13 | Code::CodeKey { rung: Doc, file: src/export/mod.rs, decl: 6, sub: 0, line: 67 } |  |  | 0.522 |
| ns | 6418 |  | 210 | How the number of runs is decided | 4.11 | 4.10 | 0.512 |
| walker |  | 6452 | 79 | Code::CodeKey { rung: Decl, file: src/export/mod.rs, decl: 7, sub: 0, line: 73 } |  |  | 0.512 |
| walker |  | 6496 | 44 | Code::CodeKey { rung: Decl, file: src/export/mod.rs, decl: 8, sub: 0, line: 76 } |  |  | 0.512 |
| walker |  | 6522 | 26 | Code::CodeKey { rung: Decl, file: src/export/mod.rs, decl: 5, sub: 0, line: 61 } |  |  | 0.512 |
| ns | 6626 |  | 208 | The three warning triggers inside Benchmark::run | 4.12 | 4.10 | 0.504 |
| ns | 6779 |  | 153 | src/outlier_detection.rs: the modified Z-score method and its threshold | 4.13 | 4.12 | 0.502 |
| walker |  | 6995 | 473 | Toml::Dependencies { file: Cargo.toml } |  |  | 0.502 |
| ns | 7075 |  | 296 | src/benchmark/benchmark_result.rs: the complete BenchmarkResult field set (= the JSON export schema) | 5.1 |  | 0.492 |
| walker |  | 7120 | 125 | Code::CodeKey { rung: Decl, file: src/export/asciidoc.rs, decl: 2, sub: 0, line: 7 } |  |  | 0.492 |
| walker |  | 7129 | 9 | Code::CodeKey { rung: Body, file: src/export/asciidoc.rs, decl: 6, sub: 0, line: 30 } |  |  | 0.492 |
| walker |  | 7183 | 54 | Markdown::Section { file: README.md, section_index: 14, keeps_default_concavity: false } |  |  | 0.492 |
| ns | 7215 |  | 140 | src/benchmark/timing_result.rs: TimingResult in full | 5.2 | 5.1 | 0.486 |
| ns | 7378 |  | 163 | src/benchmark/relative_speed.rs: annotated result type and the complete public function set | 5.3 |  | 0.480 |
| walker |  | 7428 | 245 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 1, sub: 0, line: 6 } |  |  | 0.503 |
| ns | 7536 |  | 158 | src/export/mod.rs: the ExportType enum -- the five output formats | 5.4 |  | 0.495 |
| walker |  | 7556 | 128 | Code::CodeKey { rung: Decl, file: src/export/mod.rs, decl: 1, sub: 0, line: 27 } |  |  | 0.512 |
| walker |  | 7577 | 21 | Code::CodeKey { rung: Body, file: src/util/randomized_environment_offset.rs, decl: 1, sub: 0, line: 6 } |  |  | 0.512 |
| walker |  | 7617 | 40 | Code::CodeKey { rung: Decl, file: src/export/tests.rs, decl: 1, sub: 0, line: 9 } |  |  | 0.512 |
| walker |  | 7629 | 12 | Code::CodeKey { rung: Body, file: src/export/asciidoc.rs, decl: 7, sub: 0, line: 34 } |  |  | 0.512 |
| walker |  | 7641 | 12 | Code::CodeKey { rung: Body, file: src/export/markdown.rs, decl: 5, sub: 0, line: 26 } |  |  | 0.512 |
| walker |  | 7653 | 12 | Code::CodeKey { rung: Body, file: src/export/orgmode.rs, decl: 5, sub: 0, line: 20 } |  |  | 0.512 |
| walker |  | 7668 | 15 | Code::CodeKey { rung: Doc, file: src/cli.rs, decl: 2, sub: 0, line: 18 } |  |  | 0.513 |
| ns | 7698 |  | 162 | src/export/mod.rs: the Exporter trait and ExportManager | 5.5 | 5.4 | 0.511 |
| walker |  | 7824 | 156 | Code::CodeKey { rung: Decl, file: src/export/markup.rs, decl: 2, sub: 0, line: 15 } |  |  | 0.512 |
| walker |  | 7833 | 9 | Code::CodeKey { rung: Body, file: src/export/markup.rs, decl: 6, sub: 0, line: 87 } |  |  | 0.512 |
| ns | 7835 |  | 137 | Flag-to-exporter wiring and the '-' means stdout convention | 5.6 | 5.5 | 0.508 |
| walker |  | 7843 | 10 | Code::CodeKey { rung: Doc, file: src/util/units.rs, decl: 5, sub: 0, line: 18 } |  |  | 0.508 |
| walker |  | 7870 | 27 | Code::CodeKey { rung: Body, file: src/output/format.rs, decl: 1, sub: 0, line: 5 } |  |  | 0.508 |
| walker |  | 7943 | 73 | Code::CodeKey { rung: Body, file: src/main.rs, decl: 2, sub: 0, line: 53 } |  |  | 0.520 |
| walker |  | 8006 | 63 | Code::CodeKey { rung: Doc, file: src/outlier_detection.rs, decl: 1, sub: 0, line: 13 } |  |  | 0.526 |
| ns | 8051 |  | 216 | src/export/markup.rs: the shared table shape and the blanket Exporter impl | 5.7 | 5.5 | 0.525 |
| ns | 8267 |  | 216 | The five exporter types, and the CSV column set | 5.8 | 5.4 | 0.522 |
| walker |  | 8377 | 371 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 6, sub: 0, line: 36 } |  |  | 0.549 |
| walker |  | 8394 | 17 | Code::CodeKey { rung: Body, file: src/export/markdown.rs, decl: 3, sub: 0, line: 9 } |  |  | 0.549 |
| walker |  | 8468 | 74 | Markdown::Section { file: README.md, section_index: 15, keeps_default_concavity: false } |  |  | 0.549 |
| ns | 8524 |  | 257 | src/output/format.rs: automatic time-unit selection, and the Unit type | 5.9 |  | 0.545 |
| walker |  | 8550 | 82 | Markdown::Section { file: README.md, section_index: 13, keeps_default_concavity: false } |  |  | 0.545 |
| walker |  | 8559 | 9 | Code::CodeKey { rung: Body, file: src/command.rs, decl: 9, sub: 0, line: 96 } |  |  | 0.545 |
| walker |  | 8568 | 9 | Code::CodeKey { rung: Body, file: src/export/markup.rs, decl: 7, sub: 0, line: 91 } |  |  | 0.545 |
| walker |  | 8581 | 13 | Code::CodeKey { rung: Body, file: src/export/asciidoc.rs, decl: 4, sub: 0, line: 22 } |  |  | 0.545 |
| walker |  | 8593 | 12 | Code::CodeKey { rung: Doc, file: src/timer/mod.rs, decl: 3, sub: 0, line: 52 } |  |  | 0.545 |
| walker |  | 8603 | 10 | Code::CodeKey { rung: Body, file: src/util/number.rs, decl: 5, sub: 0, line: 25 } |  |  | 0.545 |
| walker |  | 8647 | 44 | Code::CodeKey { rung: Body, file: src/util/min_max.rs, decl: 1, sub: 0, line: 2 } |  |  | 0.545 |
| ns | 8661 |  | 137 | src/output/warnings.rs: the complete Warnings set | 5.10 | 4.12 | 0.552 |
| walker |  | 8691 | 44 | Code::CodeKey { rung: Body, file: src/util/min_max.rs, decl: 2, sub: 0, line: 10 } |  |  | 0.552 |
| walker |  | 8705 | 14 | Code::CodeKey { rung: Doc, file: src/timer/unix_timer.rs, decl: 6, sub: 0, line: 41 } |  |  | 0.552 |
| walker |  | 8752 | 47 | Code::CodeKey { rung: Body, file: src/output/format.rs, decl: 2, sub: 0, line: 11 } |  |  | 0.552 |
| walker |  | 8762 | 10 | Code::CodeKey { rung: Body, file: src/command.rs, decl: 14, sub: 0, line: 250 } |  |  | 0.552 |
| walker |  | 8788 | 26 | Code::CodeKey { rung: Body, file: src/timer/wall_clock_timer.rs, decl: 3, sub: 0, line: 10 } |  |  | 0.552 |
| ns | 8869 |  | 208 | src/command.rs: the Command type and its complete method roster | 6.1 |  | 0.559 |
| ns | 9015 |  | 146 | How {param} placeholders are substituted, and why naively | 6.2 | 6.1 | 0.555 |
| walker |  | 9047 | 259 | Code::CodeKey { rung: Decl, file: src/options.rs, decl: 26, sub: 0, line: 198 } |  |  | 0.565 |
| walker |  | 9074 | 27 | Code::CodeKey { rung: Body, file: src/export/orgmode.rs, decl: 4, sub: 0, line: 16 } |  |  | 0.565 |
| walker |  | 9178 | 104 | Code::CodeKey { rung: Doc, file: src/outlier_detection.rs, decl: 2, sub: 0, line: 21 } |  |  | 0.567 |
| ns | 9202 |  | 187 | src/command.rs: Commands and its complete method roster, with the three construction modes | 6.3 |  | 0.564 |
| walker |  | 9228 | 50 | Code::CodeKey { rung: Decl, file: src/export/mod.rs, decl: 3, sub: 0, line: 48 } |  |  | 0.570 |
| walker |  | 9238 | 10 | Code::CodeKey { rung: Body, file: src/util/number.rs, decl: 7, sub: 0, line: 31 } |  |  | 0.570 |
| walker |  | 9253 | 15 | Code::CodeKey { rung: Doc, file: src/util/units.rs, decl: 6, sub: 0, line: 27 } |  |  | 0.570 |
| walker |  | 9289 | 36 | Code::CodeKey { rung: Body, file: src/timer/windows_timer.rs, decl: 8, sub: 0, line: 125 } |  |  | 0.570 |
| walker |  | 9411 | 122 | Code::CodeKey { rung: Decl, file: src/timer/mod.rs, decl: 1, sub: 0, line: 27 } |  |  | 0.570 |
| walker |  | 9429 | 18 | Code::CodeKey { rung: Body, file: src/parameter/range_step.rs, decl: 8, sub: 0, line: 75 } |  |  | 0.570 |
| ns | 9431 |  | 229 | src/parameter/: ParameterValue, RangeStep and its 100_000 cap, tokenize() | 6.4 |  | 0.577 |
| walker |  | 9480 | 51 | Code::CodeKey { rung: Doc, file: src/output/format.rs, decl: 1, sub: 0, line: 5 } |  |  | 0.577 |
| walker |  | 9492 | 12 | Code::CodeKey { rung: Body, file: src/command.rs, decl: 7, sub: 0, line: 77 } |  |  | 0.577 |
| ns | 9581 |  | 150 | build.rs: shell completions generated from the same clap command | 7.1 | 2.1 | 0.572 |
| ns | 9790 |  | 209 | .github/workflows/CICD.yml: the complete job set and the commands each runs | 7.2 |  | 0.566 |
| walker |  | 9811 | 319 | Code::CodeKey { rung: Decl, file: src/options.rs, decl: 26, sub: 1, line: 198 } |  |  | 0.586 |
| walker |  | 9838 | 27 | Code::CodeKey { rung: Body, file: src/timer/unix_timer.rs, decl: 4, sub: 0, line: 23 } |  |  | 0.586 |
| walker |  | 9880 | 42 | Code::CodeKey { rung: Body, file: src/timer/wall_clock_timer.rs, decl: 4, sub: 0, line: 16 } |  |  | 0.586 |
| walker |  | 9901 | 21 | Code::CodeKey { rung: Body, file: src/export/asciidoc.rs, decl: 5, sub: 0, line: 26 } |  |  | 0.586 |
| ns | 9982 |  | 192 | tests/: the shared harness helpers and the debug-mode test idiom | 7.3 |  | 0.579 |
