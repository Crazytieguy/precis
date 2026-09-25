Score(3000)=0.595 I=0.799 C=0.443 ns_rows≤3K=23/67 grid(1000/1442/2080/3000/4327/6240/9000)=0.691/0.594/0.600/0.595/0.548/0.578/0.579

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| ns | 34 |  | 34 | Crate identity: README title and one-line description | 1.1 |  | 0.000 |
| walker |  | 46 | 46 | Fs::DirListing { dir: . } |  |  | 0.000 |
| walker |  | 56 | 10 | Fs::DirListing { dir: rfcs } |  |  | 0.000 |
| walker |  | 77 | 21 | Fs::DirListing { dir: src } |  |  | 0.000 |
| ns | 80 |  | 46 | Complete root listing | 1.2 |  | 0.575 |
| walker |  | 97 | 20 | Fs::DirListing { dir: src/kv } |  |  | 0.635 |
| walker |  | 100 | 3 | Fs::DirListing { dir: .github } |  |  | 0.635 |
| walker |  | 104 | 4 | Fs::DirListing { dir: .github/workflows } |  |  | 0.637 |
| walker |  | 108 | 4 | Fs::DirListing { dir: benches } |  |  | 0.641 |
| ns | 121 |  | 41 | Complete src/ and src/kv/ listings | 1.3 |  | 0.636 |
| ns | 156 |  | 35 | Listings for the remaining directories | 1.4 |  | 0.566 |
| walker |  | 160 | 52 | Markdown::ReadmeHeadline { file: README.md } |  |  | 0.891 |
| walker |  | 168 | 8 | Fs::DirListing { dir: tests } |  |  | 0.935 |
| walker |  | 178 | 10 | Code::CodeKey { rung: ModuleDoc, file: src/kv/key.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.935 |
| walker |  | 187 | 9 | Fs::DirListing { dir: test_max_level_features } |  |  | 1.000 |
| walker |  | 215 | 28 | Code::CodeKey { rung: ModuleDoc, file: src/__private_api.rs, decl: 0, sub: 0, line: 0 } |  |  | 1.000 |
| ns | 223 |  | 67 | Facade semantics: the noop fallback | 1.5 |  | 0.934 |
| walker |  | 271 | 56 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.934 |
| ns | 304 |  | 81 | Facade semantics: what a log request is | 1.6 |  | 0.878 |
| ns | 431 |  | 127 | Cargo.toml package block: version, licence, MSRV, edition | 1.7 |  | 0.779 |
| walker |  | 462 | 191 | Toml::Identity { file: Cargo.toml } |  |  | 0.897 |
| walker |  | 526 | 64 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.897 |
| walker |  | 562 | 36 | Toml::Config { file: Cargo.toml } |  |  | 0.897 |
| ns | 565 |  | 134 | Cargo features, part 1: the twelve compile-time level filters | 1.8 |  | 0.767 |
| walker |  | 681 | 119 | Toml::Dependencies { file: Cargo.toml } |  |  | 0.772 |
| walker |  | 726 | 45 | Code::CodeKey { rung: ModuleDoc, file: src/kv/value.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.772 |
| walker |  | 774 | 48 | Code::CodeKey { rung: ModuleDoc, file: src/kv/source.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.772 |
| ns | 799 |  | 234 | Cargo features, part 2: std, the kv family, the serde alias, deprecated aliases | 1.9 | 1.8 | 0.683 |
| ns | 916 |  | 117 | The four optional runtime dependencies | 1.10 |  | 0.691 |
| walker |  | 1031 | 257 | Code::CodeKey { rung: Names, file: src/lib.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.697 |
| walker |  | 1040 | 9 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 10, sub: 0, line: 462 } |  |  | 0.697 |
| walker |  | 1055 | 15 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 2, sub: 0, line: 442 } |  |  | 0.697 |
| ns | 1084 |  | 168 | Roster: every public type, trait, module and const in src/lib.rs | 2.1 |  | 0.645 |
| walker |  | 1086 | 31 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 1, sub: 0, line: 420 } |  |  | 0.645 |
| ns | 1288 |  | 204 | Roster: the seven free functions of the global logger API, with their cfg gates | 2.2 |  | 0.610 |
| walker |  | 1350 | 264 | Code::CodeKey { rung: Names, file: src/lib.rs, decl: 0, sub: 1, line: 0 } |  |  | 0.622 |
| ns | 1374 |  | 86 | `enum Level`: all five variants and their discriminants | 2.3 | 2.1 | 0.594 |
| walker |  | 1375 | 25 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 29, sub: 0, line: 651 } |  |  | 0.594 |
| walker |  | 1381 | 6 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 30, sub: 0, line: 652 } |  |  | 0.594 |
| walker |  | 1407 | 26 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 13, sub: 0, line: 501 } |  |  | 0.594 |
| walker |  | 1413 | 6 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 14, sub: 0, line: 502 } |  |  | 0.594 |
| walker |  | 1440 | 27 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 62, sub: 0, line: 1002 } |  |  | 0.594 |
| ns | 1466 |  | 92 | `enum LevelFilter`: all six variants, including `Off` | 2.4 | 2.1 | 0.567 |
| walker |  | 1468 | 28 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 19, sub: 0, line: 528 } |  |  | 0.567 |
| walker |  | 1496 | 28 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 35, sub: 0, line: 678 } |  |  | 0.567 |
| walker |  | 1527 | 31 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 31, sub: 0, line: 658 } |  |  | 0.567 |
| walker |  | 1533 | 6 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 32, sub: 0, line: 659 } |  |  | 0.567 |
| walker |  | 1565 | 32 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 15, sub: 0, line: 508 } |  |  | 0.567 |
| walker |  | 1571 | 6 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 16, sub: 0, line: 509 } |  |  | 0.567 |
| walker |  | 1611 | 40 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 17, sub: 0, line: 515 } |  |  | 0.568 |
| ns | 1650 |  | 184 | Roster: every method on `Level` and `LevelFilter` | 2.5 |  | 0.531 |
| walker |  | 1652 | 41 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 33, sub: 0, line: 665 } |  |  | 0.532 |
| walker |  | 1671 | 19 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 46, sub: 0, line: 856 } |  |  | 0.532 |
| walker |  | 1691 | 20 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 11, sub: 0, line: 464 } |  |  | 0.532 |
| ns | 1782 |  | 132 | Module declarations and the atomics-vs-Cell import fork | 2.6 | 2.1 | 0.511 |
| walker |  | 1797 | 106 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 45, sub: 0, line: 841 } |  |  | 0.512 |
| walker |  | 1911 | 114 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 21, sub: 0, line: 534 } |  |  | 0.533 |
| walker |  | 1919 | 8 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 22, sub: 0, line: 547 } |  |  | 0.533 |
| walker |  | 1927 | 8 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 23, sub: 0, line: 553 } |  |  | 0.533 |
| ns | 1934 |  | 152 | Global state: LOGGER, STATE, the state constants, LOG_LEVEL_NAMES | 2.7 |  | 0.561 |
| walker |  | 2043 | 116 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 37, sub: 0, line: 684 } |  |  | 0.600 |
| walker |  | 2051 | 8 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 38, sub: 0, line: 698 } |  |  | 0.600 |
| walker |  | 2059 | 8 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 39, sub: 0, line: 706 } |  |  | 0.600 |
| walker |  | 2096 | 37 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 47, sub: 0, line: 860 } |  |  | 0.600 |
| ns | 2117 |  | 183 | Roster: the impls that make Level and LevelFilter comparable and parseable | 2.8 |  | 0.621 |
| walker |  | 2268 | 172 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 28, sub: 0, line: 634 } |  |  | 0.661 |
| walker |  | 2319 | 51 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 44, sub: 0, line: 780 } |  |  | 0.661 |
| ns | 2395 |  | 278 | `STATIC_MAX_LEVEL`: compile-time level resolution | 2.9 | 2.1 | 0.632 |
| ns | 2470 |  | 75 | `trait Log`: the three required methods | 3.1 | 2.1 | 0.616 |
| walker |  | 2569 | 250 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 49, sub: 0, line: 869 } |  |  | 0.617 |
| walker |  | 2577 | 8 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 50, sub: 0, line: 871 } |  |  | 0.617 |
| walker |  | 2585 | 8 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 51, sub: 0, line: 877 } |  |  | 0.617 |
| walker |  | 2593 | 8 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 52, sub: 0, line: 883 } |  |  | 0.617 |
| walker |  | 2601 | 8 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 53, sub: 0, line: 889 } |  |  | 0.617 |
| walker |  | 2609 | 8 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 54, sub: 0, line: 895 } |  |  | 0.617 |
| walker |  | 2617 | 8 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 55, sub: 0, line: 901 } |  |  | 0.617 |
| walker |  | 2625 | 8 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 56, sub: 0, line: 907 } |  |  | 0.617 |
| walker |  | 2633 | 8 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 57, sub: 0, line: 916 } |  |  | 0.617 |
| walker |  | 2641 | 8 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 58, sub: 0, line: 922 } |  |  | 0.617 |
| walker |  | 2649 | 8 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 59, sub: 0, line: 931 } |  |  | 0.617 |
| ns | 2657 |  | 187 | Worked example: a complete `Log` implementation | 3.2 |  | 0.588 |
| walker |  | 2669 | 20 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 60, sub: 0, line: 937 } |  |  | 0.588 |
| walker |  | 2689 | 20 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 61, sub: 0, line: 944 } |  |  | 0.588 |
| ns | 2813 |  | 156 | Why `set_max_level` must be called, and that it defaults to `Off` | 3.4 |  | 0.578 |
| ns | 2919 |  | 106 | Worked example: the conventional `init()` for a logger crate | 3.5 |  | 0.565 |
| walker |  | 2990 | 301 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 12, sub: 0, line: 473 } |  |  | 0.595 |
| walker |  | 3050 | 60 | Code::CodeKey { rung: Names, file: src/serde.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.595 |
| ns | 3072 |  | 153 | The `Log` impls the crate itself provides | 3.6 |  | 0.577 |
| walker |  | 3077 | 27 | Code::CodeKey { rung: Decl, file: src/serde.rs, decl: 3, sub: 0, line: 31 } |  |  | 0.577 |
| walker |  | 3106 | 29 | Code::CodeKey { rung: Decl, file: src/serde.rs, decl: 7, sub: 0, line: 126 } |  |  | 0.577 |
| walker |  | 3136 | 30 | Code::CodeKey { rung: Decl, file: src/serde.rs, decl: 1, sub: 0, line: 16 } |  |  | 0.577 |
| walker |  | 3166 | 30 | Code::CodeKey { rung: Decl, file: src/serde.rs, decl: 5, sub: 0, line: 110 } |  |  | 0.577 |
| ns | 3186 |  | 114 | `logger()`: the acquire-load fast path | 3.7 | 2.2 | 0.565 |
| walker |  | 3188 | 22 | Code::CodeKey { rung: Decl, file: src/serde.rs, decl: 2, sub: 0, line: 17 } |  |  | 0.565 |
| walker |  | 3210 | 22 | Code::CodeKey { rung: Decl, file: src/serde.rs, decl: 6, sub: 0, line: 111 } |  |  | 0.565 |
| walker |  | 3235 | 25 | Code::CodeKey { rung: Decl, file: src/serde.rs, decl: 4, sub: 0, line: 32 } |  |  | 0.565 |
| walker |  | 3260 | 25 | Code::CodeKey { rung: Decl, file: src/serde.rs, decl: 8, sub: 0, line: 127 } |  |  | 0.565 |
| ns | 3373 |  | 187 | `set_logger_inner`: the compare-exchange install path | 3.8 |  | 0.544 |
| ns | 3518 |  | 145 | Runtime max-level get and set bodies | 3.9 | 2.2 | 0.535 |
| walker |  | 3551 | 291 | Code::CodeKey { rung: Names, file: src/lib.rs, decl: 0, sub: 2, line: 0 } |  |  | 0.558 |
| walker |  | 3559 | 8 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 113, sub: 0, line: 1374 } |  |  | 0.558 |
| walker |  | 3568 | 9 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 114, sub: 0, line: 1395 } |  |  | 0.558 |
| walker |  | 3588 | 20 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 76, sub: 0, line: 1113 } |  |  | 0.558 |
| ns | 3599 |  | 81 | no_std wiring | 3.10 |  | 0.552 |
| walker |  | 3608 | 20 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 89, sub: 0, line: 1242 } |  |  | 0.552 |
| walker |  | 3629 | 21 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 115, sub: 0, line: 1419 } |  |  | 0.555 |
| walker |  | 3651 | 22 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 112, sub: 0, line: 1349 } |  |  | 0.558 |
| ns | 3679 |  | 80 | Roster: the seven public macros of src/macros.rs | 4.1 |  | 0.550 |
| walker |  | 3689 | 38 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 83, sub: 0, line: 1199 } |  |  | 0.550 |
| walker |  | 3738 | 49 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 78, sub: 0, line: 1157 } |  |  | 0.551 |
| walker |  | 3796 | 58 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 91, sub: 0, line: 1249 } |  |  | 0.559 |
| ns | 3831 |  | 152 | `log!`: all four call forms | 4.2 | 4.1 | 0.550 |
| walker |  | 3859 | 63 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 79, sub: 0, line: 1163 } |  |  | 0.550 |
| walker |  | 3868 | 9 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 80, sub: 0, line: 1165 } |  |  | 0.550 |
| walker |  | 3877 | 9 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 81, sub: 0, line: 1171 } |  |  | 0.550 |
| walker |  | 3886 | 9 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 82, sub: 0, line: 1177 } |  |  | 0.550 |
| walker |  | 3972 | 86 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 100, sub: 0, line: 1294 } |  |  | 0.555 |
| walker |  | 4068 | 96 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 104, sub: 0, line: 1310 } |  |  | 0.561 |
| ns | 4099 |  | 268 | `__log!`: the expansion every log call becomes | 4.3 |  | 0.548 |
| walker |  | 4164 | 96 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 108, sub: 0, line: 1327 } |  |  | 0.555 |
| ns | 4170 |  | 71 | `error!`: head and matcher arms | 4.5 | 4.1 | 0.548 |
| walker |  | 4267 | 103 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 84, sub: 0, line: 1204 } |  |  | 0.548 |
| walker |  | 4276 | 9 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 85, sub: 0, line: 1211 } |  |  | 0.548 |
| walker |  | 4285 | 9 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 86, sub: 0, line: 1222 } |  |  | 0.548 |
| walker |  | 4294 | 9 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 87, sub: 0, line: 1229 } |  |  | 0.548 |
| walker |  | 4303 | 9 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 88, sub: 0, line: 1236 } |  |  | 0.548 |
| walker |  | 4355 | 52 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 96, sub: 0, line: 1285 } |  |  | 0.548 |
| ns | 4406 |  | 236 | Roster: every doc(hidden) internal macro, with its feature fork | 4.6 |  | 0.532 |
| walker |  | 4717 | 362 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 63, sub: 0, line: 1007 } |  |  | 0.533 |
| walker |  | 4726 | 9 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 64, sub: 0, line: 1020 } |  |  | 0.533 |
| walker |  | 4735 | 9 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 65, sub: 0, line: 1036 } |  |  | 0.533 |
| walker |  | 4744 | 9 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 66, sub: 0, line: 1043 } |  |  | 0.533 |
| walker |  | 4753 | 9 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 67, sub: 0, line: 1050 } |  |  | 0.533 |
| walker |  | 4762 | 9 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 68, sub: 0, line: 1057 } |  |  | 0.533 |
| ns | 4763 |  | 357 | `__log_value!`: the capture-modifier dispatch table | 4.7 | 4.6 | 0.507 |
| walker |  | 4771 | 9 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 69, sub: 0, line: 1064 } |  |  | 0.507 |
| walker |  | 4780 | 9 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 70, sub: 0, line: 1071 } |  |  | 0.507 |
| walker |  | 4789 | 9 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 71, sub: 0, line: 1078 } |  |  | 0.507 |
| walker |  | 4798 | 9 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 72, sub: 0, line: 1085 } |  |  | 0.507 |
| walker |  | 4807 | 9 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 73, sub: 0, line: 1092 } |  |  | 0.507 |
| walker |  | 4816 | 9 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 75, sub: 0, line: 1107 } |  |  | 0.507 |
| walker |  | 4838 | 22 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 74, sub: 0, line: 1099 } |  |  | 0.508 |
| walker |  | 4921 | 83 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.508 |
| ns | 4964 |  | 201 | `log_enabled!`: purpose and call forms | 4.9 | 4.1 | 0.498 |
| walker |  | 5001 | 80 | Code::CodeKey { rung: Names, file: src/macros.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.513 |
| ns | 5168 |  | 204 | `__private_api`: the three functions macros expand into | 5.1 |  | 0.500 |
| walker |  | 5222 | 221 | Code::CodeKey { rung: Names, file: src/lib.rs, decl: 0, sub: 3, line: 0 } |  |  | 0.526 |
| walker |  | 5234 | 12 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 122, sub: 0, line: 1558 } |  |  | 0.526 |
| walker |  | 5246 | 12 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 126, sub: 0, line: 1575 } |  |  | 0.526 |
| walker |  | 5260 | 14 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 116, sub: 0, line: 1477 } |  |  | 0.530 |
| walker |  | 5283 | 23 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 119, sub: 0, line: 1547 } |  |  | 0.530 |
| walker |  | 5311 | 28 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 123, sub: 0, line: 1564 } |  |  | 0.530 |
| walker |  | 5343 | 32 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 120, sub: 0, line: 1551 } |  |  | 0.530 |
| ns | 5364 |  | 196 | `log_impl`: where a `Record` is actually built | 5.2 |  | 0.517 |
| walker |  | 5375 | 32 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 124, sub: 0, line: 1568 } |  |  | 0.517 |
| walker |  | 5419 | 44 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 117, sub: 0, line: 1482 } |  |  | 0.519 |
| ns | 5513 |  | 149 | `GlobalLogger`: the zero-sized proxy for the global slot | 5.3 |  | 0.507 |
| walker |  | 5697 | 278 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 128, sub: 0, line: 1611 } |  |  | 0.534 |
| ns | 5737 |  | 224 | `kv_support`: the capture_* functions behind every modifier | 5.4 |  | 0.526 |
| ns | 5843 |  | 106 | `struct Record`: every field | 6.1 | 2.1 | 0.536 |
| walker |  | 5943 | 246 | Code::CodeKey { rung: ModuleDoc, file: src/lib.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.559 |
| ns | 6024 |  | 181 | Roster: every accessor on `Record` | 6.2 |  | 0.571 |
| ns | 6125 |  | 101 | `Metadata` and its accessors | 6.3 | 2.1 | 0.578 |
| walker |  | 6286 | 343 | Code::CodeKey { rung: Decl, file: src/macros.rs, decl: 7, sub: 0, line: 390 } |  |  | 0.588 |
| ns | 6331 |  | 206 | Roster: `RecordBuilder` and all twelve setters | 6.4 | 2.1 | 0.601 |
| walker |  | 6345 | 59 | Code::CodeKey { rung: Names, file: src/kv/error.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.601 |
| walker |  | 6367 | 22 | Code::CodeKey { rung: Decl, file: src/kv/error.rs, decl: 1, sub: 0, line: 4 } |  |  | 0.601 |
| walker |  | 6389 | 22 | Code::CodeKey { rung: Decl, file: src/kv/error.rs, decl: 9, sub: 0, line: 62 } |  |  | 0.601 |
| walker |  | 6417 | 28 | Code::CodeKey { rung: Decl, file: src/kv/error.rs, decl: 7, sub: 0, line: 48 } |  |  | 0.601 |
| ns | 6434 |  | 103 | `MetadataBuilder` and its setters | 6.5 | 2.1 | 0.606 |
| walker |  | 6495 | 78 | Code::CodeKey { rung: Decl, file: src/kv/error.rs, decl: 3, sub: 0, line: 19 } |  |  | 0.606 |
| walker |  | 6509 | 14 | Code::CodeKey { rung: Decl, file: src/kv/error.rs, decl: 5, sub: 0, line: 28 } |  |  | 0.606 |
| walker |  | 6523 | 14 | Code::CodeKey { rung: Decl, file: src/kv/error.rs, decl: 6, sub: 0, line: 36 } |  |  | 0.606 |
| ns | 6591 |  | 157 | What structured logging means in `log` | 7.1 |  | 0.600 |
| ns | 6774 |  | 183 | The complete list of capture modifiers | 7.2 |  | 0.595 |
| walker |  | 6933 | 410 | Toml::Operational { file: Cargo.toml } |  |  | 0.657 |
| ns | 6979 |  | 205 | The kv module's structure and complete export list | 7.3 |  | 0.644 |
| ns | 7070 |  | 91 | `trait Source`: the three methods | 7.4 |  | 0.638 |
| walker |  | 7216 | 283 | Code::CodeKey { rung: Decl, file: src/macros.rs, decl: 2, sub: 0, line: 163 } |  |  | 0.640 |
| ns | 7277 |  | 207 | Roster: every type that implements `Source` | 7.5 |  | 0.632 |
| ns | 7361 |  | 84 | `trait VisitSource`: the visitor side of a `Source` | 7.6 |  | 0.629 |
| ns | 7444 |  | 83 | `Value` and `ToValue` | 7.7 |  | 0.624 |
| walker |  | 7499 | 283 | Code::CodeKey { rung: Decl, file: src/macros.rs, decl: 3, sub: 0, line: 202 } |  |  | 0.624 |
| ns | 7689 |  | 245 | Roster: every constructor and conversion on `Value` | 7.8 |  | 0.613 |
| walker |  | 7782 | 283 | Code::CodeKey { rung: Decl, file: src/macros.rs, decl: 4, sub: 0, line: 250 } |  |  | 0.613 |
| ns | 7965 |  | 276 | The primitive conversion tables | 7.9 |  | 0.603 |
| walker |  | 8065 | 283 | Code::CodeKey { rung: Decl, file: src/macros.rs, decl: 5, sub: 0, line: 290 } |  |  | 0.603 |
| ns | 8158 |  | 193 | Roster: every method on `VisitValue` | 7.10 |  | 0.595 |
| ns | 8294 |  | 136 | The two `Value` backends: `value_bag` and the dependency-free fallback | 7.11 |  | 0.590 |
| walker |  | 8348 | 283 | Code::CodeKey { rung: Decl, file: src/macros.rs, decl: 6, sub: 0, line: 334 } |  |  | 0.590 |
| walker |  | 8362 | 14 | Code::CodeKey { rung: Doc, file: src/kv/error.rs, decl: 1, sub: 0, line: 4 } |  |  | 0.591 |
| walker |  | 8391 | 29 | Markdown::Section { file: rfcs/0296-structured-logging.md, section_index: 41, keeps_default_concavity: false } |  |  | 0.591 |
| ns | 8588 |  | 294 | `Key`, `ToKey`, and their feature-gated support modules | 7.12 |  | 0.580 |
| walker |  | 8675 | 284 | Code::CodeKey { rung: Decl, file: src/macros.rs, decl: 1, sub: 0, line: 73 } |  |  | 0.586 |
| ns | 8760 |  | 172 | `kv::Error`: every variant of the private inner enum | 7.13 |  | 0.581 |
| ns | 8837 |  | 77 | `Source::get` and `Source::count` default implementations | 7.17 | 7.4 | 0.579 |
| walker |  | 8930 | 255 | Code::CodeKey { rung: ModuleDoc, file: src/lib.rs, decl: 0, sub: 1, line: 0 } |  |  | 0.579 |
| walker |  | 9014 | 84 | Code::CodeKey { rung: Decl, file: src/kv/error.rs, decl: 2, sub: 0, line: 9 } |  |  | 0.591 |
| ns | 9121 |  | 284 | Roster: every test in tests/macros.rs | 8.1 |  | 0.580 |
| walker |  | 9221 | 207 | Code::CodeKey { rung: Names, file: src/__private_api.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.582 |
| walker |  | 9229 | 8 | Code::CodeKey { rung: Decl, file: src/__private_api.rs, decl: 6, sub: 0, line: 38 } |  |  | 0.582 |
| walker |  | 9238 | 9 | Code::CodeKey { rung: Decl, file: src/__private_api.rs, decl: 14, sub: 0, line: 107 } |  |  | 0.583 |
| walker |  | 9249 | 11 | Code::CodeKey { rung: Decl, file: src/__private_api.rs, decl: 1, sub: 0, line: 9 } |  |  | 0.583 |
| walker |  | 9285 | 36 | Code::CodeKey { rung: Decl, file: src/__private_api.rs, decl: 2, sub: 0, line: 21 } |  |  | 0.583 |
| walker |  | 9291 | 6 | Code::CodeKey { rung: Decl, file: src/__private_api.rs, decl: 3, sub: 0, line: 22 } |  |  | 0.583 |
| ns | 9297 |  | 176 | tests/integration.rs: the capturing test logger and what it pins | 8.2 |  | 0.576 |
| walker |  | 9329 | 38 | Code::CodeKey { rung: Decl, file: src/__private_api.rs, decl: 4, sub: 0, line: 28 } |  |  | 0.576 |
| walker |  | 9335 | 6 | Code::CodeKey { rung: Decl, file: src/__private_api.rs, decl: 5, sub: 0, line: 29 } |  |  | 0.576 |
| walker |  | 9391 | 56 | Code::CodeKey { rung: Decl, file: src/__private_api.rs, decl: 7, sub: 0, line: 41 } |  |  | 0.580 |
| ns | 9463 |  | 166 | src/serde.rs: what is serialised, and how | 8.3 |  | 0.577 |
| walker |  | 9484 | 93 | Code::CodeKey { rung: Decl, file: src/__private_api.rs, decl: 12, sub: 0, line: 84 } |  |  | 0.586 |
| walker |  | 9559 | 75 | Code::CodeKey { rung: Decl, file: src/__private_api.rs, decl: 11, sub: 0, line: 56 } |  |  | 0.589 |
| walker |  | 9568 | 9 | Code::CodeKey { rung: Body, file: src/__private_api.rs, decl: 14, sub: 0, line: 107 } |  |  | 0.590 |
| walker |  | 9578 | 10 | Code::CodeKey { rung: Doc, file: src/__private_api.rs, decl: 6, sub: 0, line: 38 } |  |  | 0.592 |
| ns | 9607 |  | 144 | CI: the seven jobs | 8.4 |  | 0.585 |
| ns | 9663 |  | 56 | The companion crate that tests compile-time filtering | 8.5 |  | 0.582 |
| ns | 9735 |  | 72 | Roster: the value benchmarks | 8.6 |  | 0.580 |
| walker |  | 9736 | 158 | Code::CodeKey { rung: Decl, file: src/macros.rs, decl: 2, sub: 1, line: 163 } |  |  | 0.585 |
| ns | 9844 |  | 109 | The mutually-exclusive feature guards | 8.7 |  | 0.581 |
| ns | 9889 |  | 45 | CHANGELOG: format and latest release | 8.8 |  | 0.579 |
| walker |  | 9894 | 158 | Code::CodeKey { rung: Decl, file: src/macros.rs, decl: 3, sub: 1, line: 202 } |  |  | 0.579 |
| ns | 9979 |  | 90 | Roster: the top-level sections of the structured-logging RFC | 8.9 |  | 0.576 |
