Score(3000)=0.578 I=0.851 C=0.392 ns_rows≤3K=21/56 grid(1000/1442/2080/3000/4327/6240/9000)=0.785/0.826/0.688/0.578/0.505/0.505/0.557

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
| walker |  | 1527 | 30 | Code::CodeKey { rung: Names, file: src/export/markdown.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.826 |
| walker |  | 1535 | 8 | Code::CodeKey { rung: Decl, file: src/export/markdown.rs, decl: 1, sub: 0, line: 5 } |  |  | 0.826 |
| walker |  | 1608 | 73 | Code::CodeKey { rung: Decl, file: src/export/markdown.rs, decl: 2, sub: 0, line: 8 } |  |  | 0.826 |
| walker |  | 1640 | 32 | Code::CodeKey { rung: Names, file: src/export/orgmode.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.826 |
| walker |  | 1648 | 8 | Code::CodeKey { rung: Decl, file: src/export/orgmode.rs, decl: 1, sub: 0, line: 4 } |  |  | 0.826 |
| ns | 1672 |  | 437 | Complete option roster: every Arg::new(...) in src/cli.rs | 2.2 | 2.1 | 0.743 |
| walker |  | 1721 | 73 | Code::CodeKey { rung: Decl, file: src/export/orgmode.rs, decl: 2, sub: 0, line: 7 } |  |  | 0.743 |
| walker |  | 1776 | 55 | Code::CodeKey { rung: Decl, file: src/export/csv.rs, decl: 3, sub: 0, line: 16 } |  |  | 0.743 |
| walker |  | 1840 | 64 | Code::CodeKey { rung: Names, file: src/error.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.743 |
| ns | 1862 |  | 190 | Complete short-alias roster for the CLI options | 2.3 | 2.2 | 0.708 |
| walker |  | 1864 | 24 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 4, sub: 0, line: 30 } |  |  | 0.708 |
| walker |  | 1889 | 25 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 2, sub: 0, line: 24 } |  |  | 0.708 |
| walker |  | 1903 | 14 | Code::CodeKey { rung: Body, file: src/error.rs, decl: 3, sub: 0, line: 25 } |  |  | 0.708 |
| walker |  | 1917 | 14 | Code::CodeKey { rung: Body, file: src/error.rs, decl: 5, sub: 0, line: 31 } |  |  | 0.708 |
| walker |  | 1953 | 36 | Code::CodeKey { rung: Names, file: src/export/asciidoc.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.708 |
| walker |  | 1961 | 8 | Code::CodeKey { rung: Decl, file: src/export/asciidoc.rs, decl: 1, sub: 0, line: 4 } |  |  | 0.708 |
| ns | 2016 |  | 154 | Enumerated value sets, defaults, and the two hidden options | 2.4 | 2.2 | 0.688 |
| ns | 2146 |  | 130 | Complete set of inter-option conflicts and requirements | 2.5 | 2.2 | 0.674 |
| ns | 2366 |  | 220 | src/options.rs: the Options struct, run bounds through setup_command | 3.1 |  | 0.647 |
| walker |  | 2434 | 473 | Toml::Dependencies { file: Cargo.toml } |  |  | 0.647 |
| walker |  | 2559 | 125 | Code::CodeKey { rung: Decl, file: src/export/asciidoc.rs, decl: 2, sub: 0, line: 7 } |  |  | 0.647 |
| walker |  | 2568 | 9 | Code::CodeKey { rung: Body, file: src/export/asciidoc.rs, decl: 6, sub: 0, line: 30 } |  |  | 0.647 |
| ns | 2577 |  | 211 | src/options.rs: remaining Options fields (output, sort, executor, I/O, time unit) | 3.2 | 3.1 | 0.626 |
| walker |  | 2622 | 54 | Markdown::Section { file: README.md, section_index: 14, keeps_default_concavity: false } |  |  | 0.626 |
| ns | 2839 |  | 262 | src/options.rs: impl Default for Options -- the concrete default values | 3.3 | 3.1 | 0.595 |
| walker |  | 2867 | 245 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 1, sub: 0, line: 6 } |  |  | 0.597 |
| walker |  | 2888 | 21 | Code::CodeKey { rung: Body, file: src/util/randomized_environment_offset.rs, decl: 1, sub: 0, line: 6 } |  |  | 0.597 |
| walker |  | 2928 | 40 | Code::CodeKey { rung: Names, file: src/output/warnings.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.597 |
| walker |  | 2958 | 30 | Code::CodeKey { rung: Decl, file: src/output/warnings.rs, decl: 1, sub: 0, line: 7 } |  |  | 0.597 |
| walker |  | 2988 | 30 | Code::CodeKey { rung: Decl, file: src/output/warnings.rs, decl: 3, sub: 0, line: 20 } |  |  | 0.597 |
| ns | 2991 |  | 152 | src/options.rs: DEFAULT_SHELL per platform, the Shell enum and its parser | 3.4 |  | 0.578 |
| walker |  | 3040 | 52 | Code::CodeKey { rung: Decl, file: src/output/warnings.rs, decl: 2, sub: 0, line: 13 } |  |  | 0.578 |
| walker |  | 3050 | 10 | Code::CodeKey { rung: Doc, file: src/output/warnings.rs, decl: 2, sub: 0, line: 13 } |  |  | 0.579 |
| walker |  | 3090 | 40 | Code::CodeKey { rung: Names, file: src/util/min_max.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.579 |
| ns | 3094 |  | 103 | src/options.rs: ExecutorKind (Raw / Shell / Mock) and its default | 3.5 | 3.4 | 0.565 |
| walker |  | 3104 | 14 | Code::CodeKey { rung: Doc, file: src/util/min_max.rs, decl: 1, sub: 0, line: 2 } |  |  | 0.565 |
| walker |  | 3120 | 16 | Code::CodeKey { rung: Doc, file: src/util/min_max.rs, decl: 2, sub: 0, line: 10 } |  |  | 0.565 |
| walker |  | 3162 | 42 | Code::CodeKey { rung: Names, file: src/util/units.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.565 |
| walker |  | 3204 | 42 | Code::CodeKey { rung: Decl, file: src/util/units.rs, decl: 4, sub: 0, line: 16 } |  |  | 0.565 |
| walker |  | 3247 | 43 | Code::CodeKey { rung: Decl, file: src/util/units.rs, decl: 3, sub: 0, line: 9 } |  |  | 0.565 |
| walker |  | 3254 | 7 | Code::CodeKey { rung: Doc, file: src/util/units.rs, decl: 3, sub: 0, line: 9 } |  |  | 0.565 |
| walker |  | 3264 | 10 | Code::CodeKey { rung: Doc, file: src/util/units.rs, decl: 2, sub: 0, line: 6 } |  |  | 0.565 |
| walker |  | 3304 | 40 | Code::CodeKey { rung: Decl, file: src/export/tests.rs, decl: 1, sub: 0, line: 9 } |  |  | 0.565 |
| walker |  | 3316 | 12 | Code::CodeKey { rung: Body, file: src/export/asciidoc.rs, decl: 7, sub: 0, line: 34 } |  |  | 0.565 |
| ns | 3318 |  | 224 | src/options.rs: OutputStyleOption and SortOrder vocabularies | 3.6 |  | 0.541 |
| walker |  | 3328 | 12 | Code::CodeKey { rung: Body, file: src/export/markdown.rs, decl: 5, sub: 0, line: 26 } |  |  | 0.541 |
| walker |  | 3340 | 12 | Code::CodeKey { rung: Body, file: src/export/orgmode.rs, decl: 5, sub: 0, line: 20 } |  |  | 0.541 |
| walker |  | 3423 | 83 | Code::CodeKey { rung: Names, file: src/command.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.541 |
| walker |  | 3455 | 32 | Code::CodeKey { rung: Decl, file: src/command.rs, decl: 16, sub: 0, line: 387 } |  |  | 0.541 |
| walker |  | 3534 | 79 | Code::CodeKey { rung: Decl, file: src/command.rs, decl: 12, sub: 0, line: 136 } |  |  | 0.541 |
| walker |  | 3549 | 15 | Code::CodeKey { rung: Doc, file: src/command.rs, decl: 11, sub: 0, line: 134 } |  |  | 0.541 |
| ns | 3560 |  | 242 | src/options.rs: CommandInputPolicy and CommandOutputPolicy | 3.7 |  | 0.518 |
| walker |  | 3660 | 111 | Code::CodeKey { rung: Decl, file: src/command.rs, decl: 1, sub: 0, line: 21 } |  |  | 0.518 |
| walker |  | 3673 | 13 | Code::CodeKey { rung: Doc, file: src/command.rs, decl: 1, sub: 0, line: 21 } |  |  | 0.518 |
| ns | 3800 |  | 240 | src/options.rs: CmdFailureAction and RunBounds (default min = 10) | 3.8 |  | 0.500 |
| walker |  | 3853 | 180 | Code::CodeKey { rung: Decl, file: src/command.rs, decl: 2, sub: 0, line: 33 } |  |  | 0.501 |
| walker |  | 3908 | 55 | Code::CodeKey { rung: Decl, file: src/command.rs, decl: 4, sub: 0, line: 42 } |  |  | 0.501 |
| walker |  | 3951 | 43 | Code::CodeKey { rung: Names, file: src/export/json.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.501 |
| walker |  | 3959 | 8 | Code::CodeKey { rung: Decl, file: src/export/json.rs, decl: 2, sub: 0, line: 16 } |  |  | 0.501 |
| walker |  | 3972 | 13 | Code::CodeKey { rung: Decl, file: src/export/json.rs, decl: 3, sub: 0, line: 19 } |  |  | 0.501 |
| walker |  | 3999 | 27 | Code::CodeKey { rung: Decl, file: src/export/json.rs, decl: 1, sub: 0, line: 11 } |  |  | 0.501 |
| walker |  | 4054 | 55 | Code::CodeKey { rung: Decl, file: src/export/json.rs, decl: 4, sub: 0, line: 20 } |  |  | 0.501 |
| walker |  | 4098 | 44 | Code::CodeKey { rung: Names, file: src/util/mod.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.501 |
| walker |  | 4144 | 46 | Code::CodeKey { rung: Names, file: src/util/exit_code.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.501 |
| walker |  | 4153 | 9 | Code::CodeKey { rung: Decl, file: src/util/exit_code.rs, decl: 1, sub: 0, line: 3 } |  |  | 0.501 |
| walker |  | 4163 | 10 | Code::CodeKey { rung: Decl, file: src/util/exit_code.rs, decl: 2, sub: 0, line: 19 } |  |  | 0.501 |
| walker |  | 4171 | 8 | Code::CodeKey { rung: Body, file: src/util/exit_code.rs, decl: 2, sub: 0, line: 19 } |  |  | 0.501 |
| walker |  | 4186 | 15 | Code::CodeKey { rung: Doc, file: src/cli.rs, decl: 2, sub: 0, line: 18 } |  |  | 0.503 |
| ns | 4189 |  | 389 | src/error.rs: the complete OptionsError variant set with messages | 3.10 |  | 0.486 |
| walker |  | 4196 | 10 | Code::CodeKey { rung: Doc, file: src/util/units.rs, decl: 5, sub: 0, line: 18 } |  |  | 0.486 |
| walker |  | 4269 | 73 | Code::CodeKey { rung: Body, file: src/main.rs, decl: 2, sub: 0, line: 53 } |  |  | 0.505 |
| walker |  | 4332 | 63 | Code::CodeKey { rung: Doc, file: src/outlier_detection.rs, decl: 1, sub: 0, line: 13 } |  |  | 0.506 |
| ns | 4407 |  | 218 | src/error.rs: the complete ParameterScanError variant set | 3.11 | 3.10 | 0.527 |
| ns | 4627 |  | 220 | src/benchmark/executor.rs: the Executor trait | 4.1 |  | 0.514 |
| walker |  | 4703 | 371 | Code::CodeKey { rung: Decl, file: src/error.rs, decl: 6, sub: 0, line: 36 } |  |  | 0.558 |
| walker |  | 4720 | 17 | Code::CodeKey { rung: Body, file: src/export/markdown.rs, decl: 3, sub: 0, line: 9 } |  |  | 0.558 |
| ns | 4752 |  | 125 | The three Executor implementations and their state | 4.2 | 4.1 | 0.548 |
| walker |  | 4782 | 62 | Code::CodeKey { rung: Names, file: src/export/markup.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.548 |
| walker |  | 4795 | 13 | Code::CodeKey { rung: Decl, file: src/export/markup.rs, decl: 10, sub: 0, line: 108 } |  |  | 0.548 |
| walker |  | 4812 | 17 | Code::CodeKey { rung: Decl, file: src/export/markup.rs, decl: 1, sub: 0, line: 10 } |  |  | 0.548 |
| walker |  | 4865 | 53 | Code::CodeKey { rung: Decl, file: src/export/markup.rs, decl: 11, sub: 0, line: 109 } |  |  | 0.548 |
| ns | 4904 |  | 152 | BenchmarkIteration and the $HYPERFINE_ITERATION values | 4.3 |  | 0.538 |
| walker |  | 5021 | 156 | Code::CodeKey { rung: Decl, file: src/export/markup.rs, decl: 2, sub: 0, line: 15 } |  |  | 0.538 |
| walker |  | 5030 | 9 | Code::CodeKey { rung: Body, file: src/export/markup.rs, decl: 6, sub: 0, line: 87 } |  |  | 0.538 |
| ns | 5072 |  | 168 | The shared command runner: stdio wiring and injected environment variables | 4.4 | 4.3 | 0.529 |
| walker |  | 5104 | 74 | Markdown::Section { file: README.md, section_index: 15, keeps_default_concavity: false } |  |  | 0.529 |
| walker |  | 5186 | 82 | Markdown::Section { file: README.md, section_index: 13, keeps_default_concavity: false } |  |  | 0.529 |
| walker |  | 5195 | 9 | Code::CodeKey { rung: Body, file: src/command.rs, decl: 9, sub: 0, line: 96 } |  |  | 0.529 |
| walker |  | 5261 | 66 | Code::CodeKey { rung: Names, file: src/parameter/mod.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.529 |
| ns | 5284 |  | 212 | Non-zero exit handling and the failure message users actually see | 4.5 | 4.4 | 0.520 |
| walker |  | 5293 | 32 | Code::CodeKey { rung: Decl, file: src/parameter/mod.rs, decl: 1, sub: 0, line: 7 } |  |  | 0.520 |
| walker |  | 5327 | 34 | Code::CodeKey { rung: Decl, file: src/parameter/mod.rs, decl: 2, sub: 0, line: 13 } |  |  | 0.520 |
| walker |  | 5395 | 68 | Code::CodeKey { rung: Names, file: src/util/number.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.520 |
| walker |  | 5414 | 19 | Code::CodeKey { rung: Decl, file: src/util/number.rs, decl: 6, sub: 0, line: 30 } |  |  | 0.520 |
| walker |  | 5434 | 20 | Code::CodeKey { rung: Decl, file: src/util/number.rs, decl: 4, sub: 0, line: 24 } |  |  | 0.520 |
| walker |  | 5464 | 30 | Code::CodeKey { rung: Decl, file: src/util/number.rs, decl: 2, sub: 0, line: 15 } |  |  | 0.520 |
| ns | 5481 |  | 197 | Shell-spawning calibration: 50 probe runs, and where the overhead is subtracted | 4.6 | 4.2 | 0.512 |
| walker |  | 5505 | 41 | Code::CodeKey { rung: Decl, file: src/util/number.rs, decl: 8, sub: 0, line: 36 } |  |  | 0.512 |
| walker |  | 5556 | 51 | Code::CodeKey { rung: Decl, file: src/util/number.rs, decl: 1, sub: 0, line: 8 } |  |  | 0.512 |
| ns | 5679 |  | 198 | src/benchmark/scheduler.rs: Scheduler state and executor selection | 4.7 |  | 0.505 |
| walker |  | 5795 | 239 | Code::CodeKey { rung: Names, file: src/options.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.509 |
| walker |  | 5802 | 7 | Code::CodeKey { rung: Decl, file: src/options.rs, decl: 2, sub: 0, line: 19 } |  |  | 0.510 |
| walker |  | 5812 | 10 | Code::CodeKey { rung: Decl, file: src/options.rs, decl: 1, sub: 0, line: 16 } |  |  | 0.511 |
| walker |  | 5828 | 16 | Code::CodeKey { rung: Decl, file: src/options.rs, decl: 4, sub: 0, line: 32 } |  |  | 0.511 |
| walker |  | 5844 | 16 | Code::CodeKey { rung: Decl, file: src/options.rs, decl: 27, sub: 0, line: 251 } |  |  | 0.511 |
| walker |  | 5862 | 18 | Code::CodeKey { rung: Decl, file: src/options.rs, decl: 15, sub: 0, line: 116 } |  |  | 0.511 |
| walker |  | 5880 | 18 | Code::CodeKey { rung: Decl, file: src/options.rs, decl: 24, sub: 0, line: 191 } |  |  | 0.513 |
| walker |  | 5909 | 29 | Code::CodeKey { rung: Decl, file: src/options.rs, decl: 18, sub: 0, line: 132 } |  |  | 0.513 |
| ns | 5933 |  | 254 | run_benchmarks(): reference command first, calibrate once, export after each benchmark | 4.8 | 4.7 | 0.501 |
| walker |  | 5939 | 30 | Code::CodeKey { rung: Decl, file: src/options.rs, decl: 6, sub: 0, line: 38 } |  |  | 0.501 |
| walker |  | 5972 | 33 | Code::CodeKey { rung: Decl, file: src/options.rs, decl: 21, sub: 0, line: 164 } |  |  | 0.501 |
| walker |  | 6009 | 37 | Code::CodeKey { rung: Decl, file: src/options.rs, decl: 13, sub: 0, line: 101 } |  |  | 0.503 |
| walker |  | 6048 | 39 | Code::CodeKey { rung: Decl, file: src/options.rs, decl: 23, sub: 0, line: 184 } |  |  | 0.512 |
| walker |  | 6101 | 53 | Code::CodeKey { rung: Decl, file: src/options.rs, decl: 8, sub: 0, line: 47 } |  |  | 0.512 |
| walker |  | 6156 | 55 | Code::CodeKey { rung: Decl, file: src/options.rs, decl: 14, sub: 0, line: 108 } |  |  | 0.517 |
| ns | 6208 |  | 275 | src/benchmark/mod.rs: Benchmark struct, MIN_EXECUTION_TIME, and the complete method roster | 4.10 |  | 0.505 |
| walker |  | 6219 | 63 | Code::CodeKey { rung: Decl, file: src/options.rs, decl: 29, sub: 0, line: 275 } |  |  | 0.505 |
| walker |  | 6283 | 64 | Code::CodeKey { rung: Decl, file: src/options.rs, decl: 3, sub: 0, line: 23 } |  |  | 0.518 |
| walker |  | 6352 | 69 | Code::CodeKey { rung: Decl, file: src/options.rs, decl: 17, sub: 0, line: 122 } |  |  | 0.524 |
| ns | 6418 |  | 210 | How the number of runs is decided | 4.11 | 4.10 | 0.514 |
| walker |  | 6448 | 96 | Code::CodeKey { rung: Decl, file: src/options.rs, decl: 11, sub: 0, line: 70 } |  |  | 0.530 |
| walker |  | 6576 | 128 | Code::CodeKey { rung: Decl, file: src/options.rs, decl: 20, sub: 0, line: 148 } |  |  | 0.555 |
| ns | 6626 |  | 208 | The three warning triggers inside Benchmark::run | 4.12 | 4.10 | 0.547 |
| walker |  | 6725 | 149 | Code::CodeKey { rung: Decl, file: src/options.rs, decl: 12, sub: 0, line: 83 } |  |  | 0.574 |
| walker |  | 6734 | 9 | Code::CodeKey { rung: Body, file: src/export/markup.rs, decl: 7, sub: 0, line: 91 } |  |  | 0.574 |
| walker |  | 6747 | 13 | Code::CodeKey { rung: Body, file: src/export/asciidoc.rs, decl: 4, sub: 0, line: 22 } |  |  | 0.574 |
| walker |  | 6757 | 10 | Code::CodeKey { rung: Body, file: src/util/number.rs, decl: 5, sub: 0, line: 25 } |  |  | 0.574 |
| ns | 6779 |  | 153 | src/outlier_detection.rs: the modified Z-score method and its threshold | 4.13 | 4.12 | 0.577 |
| walker |  | 6835 | 78 | Code::CodeKey { rung: Names, file: src/output/format.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.577 |
| walker |  | 6853 | 18 | Code::CodeKey { rung: Doc, file: src/output/format.rs, decl: 2, sub: 0, line: 11 } |  |  | 0.577 |
| walker |  | 6871 | 18 | Code::CodeKey { rung: Doc, file: src/output/format.rs, decl: 3, sub: 0, line: 18 } |  |  | 0.577 |
| walker |  | 6898 | 27 | Code::CodeKey { rung: Body, file: src/output/format.rs, decl: 1, sub: 0, line: 5 } |  |  | 0.577 |
| walker |  | 6978 | 80 | Code::CodeKey { rung: Names, file: src/timer/unix_timer.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.577 |
| walker |  | 6993 | 15 | Code::CodeKey { rung: Decl, file: src/timer/unix_timer.rs, decl: 2, sub: 0, line: 18 } |  |  | 0.577 |
| walker |  | 7035 | 42 | Code::CodeKey { rung: Decl, file: src/timer/unix_timer.rs, decl: 3, sub: 0, line: 22 } |  |  | 0.577 |
| ns | 7075 |  | 296 | src/benchmark/benchmark_result.rs: the complete BenchmarkResult field set (= the JSON export schema) | 5.1 |  | 0.566 |
| walker |  | 7107 | 72 | Code::CodeKey { rung: Decl, file: src/timer/unix_timer.rs, decl: 1, sub: 0, line: 9 } |  |  | 0.566 |
| walker |  | 7151 | 44 | Code::CodeKey { rung: Body, file: src/util/min_max.rs, decl: 1, sub: 0, line: 2 } |  |  | 0.566 |
| walker |  | 7195 | 44 | Code::CodeKey { rung: Body, file: src/util/min_max.rs, decl: 2, sub: 0, line: 10 } |  |  | 0.566 |
| walker |  | 7209 | 14 | Code::CodeKey { rung: Doc, file: src/timer/unix_timer.rs, decl: 6, sub: 0, line: 41 } |  |  | 0.566 |
| ns | 7215 |  | 140 | src/benchmark/timing_result.rs: TimingResult in full | 5.2 | 5.1 | 0.559 |
| walker |  | 7296 | 87 | Code::CodeKey { rung: Names, file: src/timer/windows_timer.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.559 |
| walker |  | 7309 | 13 | Code::CodeKey { rung: Decl, file: src/timer/windows_timer.rs, decl: 3, sub: 0, line: 49 } |  |  | 0.559 |
| walker |  | 7326 | 17 | Code::CodeKey { rung: Decl, file: src/timer/windows_timer.rs, decl: 7, sub: 0, line: 124 } |  |  | 0.559 |
| walker |  | 7376 | 50 | Code::CodeKey { rung: Decl, file: src/timer/windows_timer.rs, decl: 4, sub: 0, line: 53 } |  |  | 0.559 |
| ns | 7378 |  | 163 | src/benchmark/relative_speed.rs: annotated result type and the complete public function set | 5.3 |  | 0.552 |
| walker |  | 7423 | 47 | Code::CodeKey { rung: Body, file: src/output/format.rs, decl: 2, sub: 0, line: 11 } |  |  | 0.552 |
| walker |  | 7433 | 10 | Code::CodeKey { rung: Body, file: src/command.rs, decl: 14, sub: 0, line: 250 } |  |  | 0.552 |
| walker |  | 7459 | 26 | Code::CodeKey { rung: Body, file: src/timer/wall_clock_timer.rs, decl: 3, sub: 0, line: 10 } |  |  | 0.552 |
| ns | 7536 |  | 158 | src/export/mod.rs: the ExportType enum -- the five output formats | 5.4 |  | 0.543 |
| ns | 7698 |  | 162 | src/export/mod.rs: the Exporter trait and ExportManager | 5.5 | 5.4 | 0.536 |
| walker |  | 7718 | 259 | Code::CodeKey { rung: Decl, file: src/options.rs, decl: 26, sub: 0, line: 198 } |  |  | 0.547 |
| walker |  | 7745 | 27 | Code::CodeKey { rung: Body, file: src/export/orgmode.rs, decl: 4, sub: 0, line: 16 } |  |  | 0.547 |
| ns | 7835 |  | 137 | Flag-to-exporter wiring and the '-' means stdout convention | 5.6 | 5.5 | 0.543 |
| walker |  | 7849 | 104 | Code::CodeKey { rung: Doc, file: src/outlier_detection.rs, decl: 2, sub: 0, line: 21 } |  |  | 0.545 |
| walker |  | 7859 | 10 | Code::CodeKey { rung: Body, file: src/util/number.rs, decl: 7, sub: 0, line: 31 } |  |  | 0.545 |
| walker |  | 7966 | 107 | Code::CodeKey { rung: Names, file: src/parameter/range_step.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.546 |
| walker |  | 8000 | 34 | Code::CodeKey { rung: Decl, file: src/parameter/range_step.rs, decl: 4, sub: 0, line: 40 } |  |  | 0.546 |
| walker |  | 8038 | 38 | Code::CodeKey { rung: Decl, file: src/parameter/range_step.rs, decl: 3, sub: 0, line: 33 } |  |  | 0.546 |
| ns | 8051 |  | 216 | src/export/markup.rs: the shared table shape and the blanket Exporter impl | 5.7 | 5.5 | 0.546 |
| walker |  | 8098 | 60 | Code::CodeKey { rung: Decl, file: src/parameter/range_step.rs, decl: 6, sub: 0, line: 62 } |  |  | 0.546 |
| walker |  | 8197 | 99 | Code::CodeKey { rung: Decl, file: src/parameter/range_step.rs, decl: 1, sub: 0, line: 7 } |  |  | 0.546 |
| ns | 8267 |  | 216 | The five exporter types, and the CSV column set | 5.8 | 5.4 | 0.542 |
| walker |  | 8300 | 103 | Code::CodeKey { rung: Decl, file: src/parameter/range_step.rs, decl: 2, sub: 0, line: 19 } |  |  | 0.542 |
| walker |  | 8315 | 15 | Code::CodeKey { rung: Doc, file: src/util/units.rs, decl: 6, sub: 0, line: 27 } |  |  | 0.542 |
| walker |  | 8429 | 114 | Code::CodeKey { rung: Names, file: src/output/progress_bar.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.542 |
| walker |  | 8436 | 7 | Code::CodeKey { rung: Decl, file: src/output/progress_bar.rs, decl: 2, sub: 0, line: 9 } |  |  | 0.542 |
| walker |  | 8446 | 10 | Code::CodeKey { rung: Decl, file: src/output/progress_bar.rs, decl: 1, sub: 0, line: 6 } |  |  | 0.542 |
| walker |  | 8457 | 11 | Code::CodeKey { rung: Doc, file: src/output/progress_bar.rs, decl: 3, sub: 0, line: 13 } |  |  | 0.542 |
| ns | 8524 |  | 257 | src/output/format.rs: automatic time-unit selection, and the Unit type | 5.9 |  | 0.539 |
| walker |  | 8573 | 116 | Code::CodeKey { rung: Names, file: src/timer/mod.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.539 |
| walker |  | 8660 | 87 | Code::CodeKey { rung: Decl, file: src/timer/mod.rs, decl: 2, sub: 0, line: 41 } |  |  | 0.539 |
| ns | 8661 |  | 137 | src/output/warnings.rs: the complete Warnings set | 5.10 | 4.12 | 0.546 |
| walker |  | 8675 | 15 | Code::CodeKey { rung: Doc, file: src/timer/mod.rs, decl: 2, sub: 0, line: 41 } |  |  | 0.546 |
| walker |  | 8690 | 15 | Code::CodeKey { rung: Doc, file: src/timer/mod.rs, decl: 4, sub: 0, line: 83 } |  |  | 0.546 |
| walker |  | 8702 | 12 | Code::CodeKey { rung: Doc, file: src/timer/mod.rs, decl: 3, sub: 0, line: 52 } |  |  | 0.546 |
| walker |  | 8841 | 139 | Code::CodeKey { rung: Names, file: src/export/mod.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.546 |
| walker |  | 8860 | 19 | Code::CodeKey { rung: Decl, file: src/export/mod.rs, decl: 4, sub: 0, line: 56 } |  |  | 0.546 |
| ns | 8869 |  | 208 | src/command.rs: the Command type and its complete method roster | 6.1 |  | 0.553 |
| walker |  | 8898 | 38 | Code::CodeKey { rung: Decl, file: src/export/mod.rs, decl: 6, sub: 0, line: 67 } |  |  | 0.555 |
| walker |  | 8913 | 15 | Code::CodeKey { rung: Decl, file: src/export/mod.rs, decl: 2, sub: 0, line: 46 } |  |  | 0.556 |
| walker |  | 8926 | 13 | Code::CodeKey { rung: Doc, file: src/export/mod.rs, decl: 6, sub: 0, line: 67 } |  |  | 0.557 |
| walker |  | 9005 | 79 | Code::CodeKey { rung: Decl, file: src/export/mod.rs, decl: 7, sub: 0, line: 73 } |  |  | 0.557 |
| ns | 9015 |  | 146 | How {param} placeholders are substituted, and why naively | 6.2 | 6.1 | 0.554 |
| walker |  | 9049 | 44 | Code::CodeKey { rung: Decl, file: src/export/mod.rs, decl: 8, sub: 0, line: 76 } |  |  | 0.554 |
| walker |  | 9075 | 26 | Code::CodeKey { rung: Decl, file: src/export/mod.rs, decl: 5, sub: 0, line: 61 } |  |  | 0.554 |
| ns | 9202 |  | 187 | src/command.rs: Commands and its complete method roster, with the three construction modes | 6.3 |  | 0.551 |
| walker |  | 9203 | 128 | Code::CodeKey { rung: Decl, file: src/export/mod.rs, decl: 1, sub: 0, line: 27 } |  |  | 0.564 |
| walker |  | 9253 | 50 | Code::CodeKey { rung: Decl, file: src/export/mod.rs, decl: 3, sub: 0, line: 48 } |  |  | 0.570 |
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
