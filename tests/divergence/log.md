Score(3000)=0.614 I=0.803 C=0.469 ns_rows≤3K=23/67 grid(1000/1442/2080/3000/4327/6240/9000)=0.693/0.587/0.630/0.614/0.522/0.552/0.563

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| ns | 34 |  | 34 | Crate identity: README title and one-line description | 1.1 |  | 0.000 |
| walker |  | 46 | 46 | Fs::DirListing { dir: . } |  |  | 0.000 |
| ns | 80 |  | 46 | Complete root listing | 1.2 |  | 0.549 |
| walker |  | 98 | 52 | Markdown::ReadmeHeadline { file: README.md } |  |  | 1.000 |
| walker |  | 108 | 10 | Fs::DirListing { dir: rfcs } |  |  | 1.000 |
| ns | 121 |  | 41 | Complete src/ and src/kv/ listings | 1.3 |  | 0.746 |
| walker |  | 129 | 21 | Fs::DirListing { dir: src } |  |  | 0.814 |
| walker |  | 149 | 20 | Fs::DirListing { dir: src/kv } |  |  | 1.000 |
| walker |  | 153 | 4 | Fs::DirListing { dir: benches } |  |  | 1.000 |
| ns | 156 |  | 35 | Listings for the remaining directories | 1.4 |  | 0.877 |
| walker |  | 159 | 6 | Fs::DirListing { dir: .github/workflows } |  |  | 0.891 |
| walker |  | 167 | 8 | Fs::DirListing { dir: tests } |  |  | 0.935 |
| walker |  | 177 | 10 | Code::CodeKey { rung: ModuleDoc, file: src/kv/key.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.935 |
| walker |  | 186 | 9 | Fs::DirListing { dir: test_max_level_features } |  |  | 1.000 |
| walker |  | 214 | 28 | Code::CodeKey { rung: ModuleDoc, file: src/__private_api.rs, decl: 0, sub: 0, line: 0 } |  |  | 1.000 |
| ns | 223 |  | 67 | Facade semantics: the noop fallback | 1.5 |  | 0.934 |
| walker |  | 270 | 56 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.934 |
| ns | 304 |  | 81 | Facade semantics: what a log request is | 1.6 |  | 0.878 |
| walker |  | 334 | 64 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.878 |
| ns | 431 |  | 127 | Cargo.toml package block: version, licence, MSRV, edition | 1.7 |  | 0.779 |
| walker |  | 525 | 191 | Toml::Identity { file: Cargo.toml } |  |  | 0.897 |
| ns | 565 |  | 134 | Cargo features, part 1: the twelve compile-time level filters | 1.8 |  | 0.767 |
| walker |  | 570 | 45 | Code::CodeKey { rung: ModuleDoc, file: src/kv/value.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.767 |
| walker |  | 618 | 48 | Code::CodeKey { rung: ModuleDoc, file: src/kv/source.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.767 |
| walker |  | 737 | 119 | Toml::Dependencies { file: Cargo.toml } |  |  | 0.772 |
| ns | 799 |  | 234 | Cargo features, part 2: std, the kv family, the serde alias, deprecated aliases | 1.9 | 1.8 | 0.683 |
| walker |  | 820 | 83 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.683 |
| ns | 916 |  | 117 | The four optional runtime dependencies | 1.10 |  | 0.691 |
| walker |  | 1047 | 227 | Code::CodeKey { rung: Names, file: src/lib.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.697 |
| walker |  | 1062 | 15 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 2, sub: 0, line: 442 } |  |  | 0.697 |
| ns | 1084 |  | 168 | Roster: every public type, trait, module and const in src/lib.rs | 2.1 |  | 0.645 |
| walker |  | 1093 | 31 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 1, sub: 0, line: 420 } |  |  | 0.645 |
| ns | 1288 |  | 204 | Roster: the seven free functions of the global logger API, with their cfg gates | 2.2 |  | 0.609 |
| walker |  | 1302 | 209 | Code::CodeKey { rung: Names, file: src/lib.rs, decl: 0, sub: 1, line: 0 } |  |  | 0.614 |
| walker |  | 1309 | 7 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 10, sub: 0, line: 462 } |  |  | 0.614 |
| walker |  | 1329 | 20 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 11, sub: 0, line: 464 } |  |  | 0.614 |
| walker |  | 1354 | 25 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 29, sub: 0, line: 651 } |  |  | 0.614 |
| walker |  | 1360 | 6 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 30, sub: 0, line: 652 } |  |  | 0.614 |
| ns | 1374 |  | 86 | `enum Level`: all five variants and their discriminants | 2.3 | 2.1 | 0.587 |
| walker |  | 1386 | 26 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 13, sub: 0, line: 501 } |  |  | 0.587 |
| walker |  | 1392 | 6 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 14, sub: 0, line: 502 } |  |  | 0.587 |
| walker |  | 1420 | 28 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 19, sub: 0, line: 528 } |  |  | 0.587 |
| walker |  | 1448 | 28 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 35, sub: 0, line: 678 } |  |  | 0.587 |
| ns | 1466 |  | 92 | `enum LevelFilter`: all six variants, including `Off` | 2.4 | 2.1 | 0.560 |
| walker |  | 1479 | 31 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 31, sub: 0, line: 658 } |  |  | 0.560 |
| walker |  | 1485 | 6 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 32, sub: 0, line: 659 } |  |  | 0.560 |
| walker |  | 1517 | 32 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 15, sub: 0, line: 508 } |  |  | 0.560 |
| walker |  | 1523 | 6 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 16, sub: 0, line: 509 } |  |  | 0.560 |
| walker |  | 1563 | 40 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 17, sub: 0, line: 515 } |  |  | 0.561 |
| walker |  | 1604 | 41 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 33, sub: 0, line: 665 } |  |  | 0.563 |
| ns | 1650 |  | 184 | Roster: every method on `Level` and `LevelFilter` | 2.5 |  | 0.526 |
| walker |  | 1657 | 53 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 44, sub: 0, line: 780 } |  |  | 0.526 |
| walker |  | 1771 | 114 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 21, sub: 0, line: 534 } |  |  | 0.548 |
| walker |  | 1779 | 8 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 22, sub: 0, line: 547 } |  |  | 0.548 |
| ns | 1782 |  | 132 | Module declarations and the atomics-vs-Cell import fork | 2.6 | 2.1 | 0.526 |
| walker |  | 1787 | 8 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 23, sub: 0, line: 553 } |  |  | 0.526 |
| walker |  | 1901 | 114 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 37, sub: 0, line: 684 } |  |  | 0.569 |
| walker |  | 1909 | 8 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 38, sub: 0, line: 698 } |  |  | 0.569 |
| walker |  | 1917 | 8 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 39, sub: 0, line: 706 } |  |  | 0.569 |
| ns | 1934 |  | 152 | Global state: LOGGER, STATE, the state constants, LOG_LEVEL_NAMES | 2.7 |  | 0.593 |
| walker |  | 2089 | 172 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 28, sub: 0, line: 634 } |  |  | 0.637 |
| walker |  | 2102 | 13 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 22, sub: 0, line: 547 } |  |  | 0.637 |
| ns | 2117 |  | 183 | Roster: the impls that make Level and LevelFilter comparable and parseable | 2.8 |  | 0.655 |
| ns | 2395 |  | 278 | `STATIC_MAX_LEVEL`: compile-time level resolution | 2.9 | 2.1 | 0.627 |
| walker |  | 2403 | 301 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 12, sub: 0, line: 473 } |  |  | 0.661 |
| walker |  | 2417 | 14 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 38, sub: 0, line: 698 } |  |  | 0.661 |
| walker |  | 2435 | 18 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 23, sub: 0, line: 553 } |  |  | 0.661 |
| ns | 2470 |  | 75 | `trait Log`: the three required methods | 3.1 | 2.1 | 0.643 |
| walker |  | 2643 | 208 | Code::CodeKey { rung: Names, file: src/lib.rs, decl: 0, sub: 2, line: 0 } |  |  | 0.659 |
| ns | 2657 |  | 187 | Worked example: a complete `Log` implementation | 3.2 |  | 0.627 |
| walker |  | 2662 | 19 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 46, sub: 0, line: 856 } |  |  | 0.627 |
| walker |  | 2682 | 20 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 76, sub: 0, line: 1113 } |  |  | 0.627 |
| walker |  | 2702 | 20 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 89, sub: 0, line: 1242 } |  |  | 0.627 |
| walker |  | 2727 | 25 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 62, sub: 0, line: 1002 } |  |  | 0.627 |
| walker |  | 2764 | 37 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 47, sub: 0, line: 860 } |  |  | 0.627 |
| walker |  | 2802 | 38 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 83, sub: 0, line: 1199 } |  |  | 0.628 |
| ns | 2813 |  | 156 | Why `set_max_level` must be called, and that it defaults to `Off` | 3.4 |  | 0.617 |
| walker |  | 2851 | 49 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 78, sub: 0, line: 1157 } |  |  | 0.617 |
| walker |  | 2909 | 58 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 91, sub: 0, line: 1249 } |  |  | 0.627 |
| ns | 2919 |  | 106 | Worked example: the conventional `init()` for a logger crate | 3.5 |  | 0.613 |
| walker |  | 2972 | 63 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 79, sub: 0, line: 1163 } |  |  | 0.614 |
| walker |  | 2981 | 9 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 80, sub: 0, line: 1165 } |  |  | 0.614 |
| walker |  | 2990 | 9 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 81, sub: 0, line: 1171 } |  |  | 0.614 |
| walker |  | 2999 | 9 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 82, sub: 0, line: 1177 } |  |  | 0.614 |
| ns | 3072 |  | 153 | The `Log` impls the crate itself provides | 3.6 |  | 0.595 |
| walker |  | 3102 | 103 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 84, sub: 0, line: 1204 } |  |  | 0.596 |
| walker |  | 3111 | 9 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 85, sub: 0, line: 1211 } |  |  | 0.596 |
| walker |  | 3120 | 9 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 86, sub: 0, line: 1222 } |  |  | 0.596 |
| walker |  | 3129 | 9 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 87, sub: 0, line: 1229 } |  |  | 0.596 |
| walker |  | 3138 | 9 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 88, sub: 0, line: 1236 } |  |  | 0.596 |
| ns | 3186 |  | 114 | `logger()`: the acquire-load fast path | 3.7 | 2.2 | 0.584 |
| walker |  | 3244 | 106 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 45, sub: 0, line: 841 } |  |  | 0.585 |
| walker |  | 3254 | 10 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 80, sub: 0, line: 1165 } |  |  | 0.585 |
| ns | 3373 |  | 187 | `set_logger_inner`: the compare-exchange install path | 3.8 |  | 0.563 |
| walker |  | 3504 | 250 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 49, sub: 0, line: 869 } |  |  | 0.564 |
| walker |  | 3512 | 8 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 50, sub: 0, line: 871 } |  |  | 0.564 |
| ns | 3518 |  | 145 | Runtime max-level get and set bodies | 3.9 | 2.2 | 0.555 |
| walker |  | 3520 | 8 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 51, sub: 0, line: 877 } |  |  | 0.555 |
| walker |  | 3528 | 8 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 52, sub: 0, line: 883 } |  |  | 0.555 |
| walker |  | 3536 | 8 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 53, sub: 0, line: 889 } |  |  | 0.555 |
| walker |  | 3544 | 8 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 54, sub: 0, line: 895 } |  |  | 0.555 |
| walker |  | 3552 | 8 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 55, sub: 0, line: 901 } |  |  | 0.555 |
| walker |  | 3560 | 8 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 56, sub: 0, line: 907 } |  |  | 0.555 |
| walker |  | 3568 | 8 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 57, sub: 0, line: 916 } |  |  | 0.555 |
| walker |  | 3576 | 8 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 58, sub: 0, line: 922 } |  |  | 0.555 |
| walker |  | 3584 | 8 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 59, sub: 0, line: 931 } |  |  | 0.555 |
| ns | 3599 |  | 81 | no_std wiring | 3.10 |  | 0.549 |
| walker |  | 3604 | 20 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 60, sub: 0, line: 937 } |  |  | 0.550 |
| walker |  | 3624 | 20 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 61, sub: 0, line: 944 } |  |  | 0.550 |
| walker |  | 3633 | 9 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 50, sub: 0, line: 871 } |  |  | 0.550 |
| walker |  | 3643 | 10 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 51, sub: 0, line: 877 } |  |  | 0.550 |
| walker |  | 3655 | 12 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 52, sub: 0, line: 883 } |  |  | 0.550 |
| walker |  | 3667 | 12 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 59, sub: 0, line: 931 } |  |  | 0.550 |
| ns | 3679 |  | 80 | Roster: the seven public macros of src/macros.rs | 4.1 |  | 0.542 |
| walker |  | 3680 | 13 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 53, sub: 0, line: 889 } |  |  | 0.542 |
| walker |  | 3693 | 13 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 55, sub: 0, line: 901 } |  |  | 0.542 |
| walker |  | 3706 | 13 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 57, sub: 0, line: 916 } |  |  | 0.542 |
| walker |  | 3720 | 14 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 81, sub: 0, line: 1171 } |  |  | 0.542 |
| walker |  | 3734 | 14 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 88, sub: 0, line: 1236 } |  |  | 0.542 |
| ns | 3831 |  | 152 | `log!`: all four call forms | 4.2 | 4.1 | 0.533 |
| walker |  | 4096 | 362 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 63, sub: 0, line: 1007 } |  |  | 0.535 |
| ns | 4099 |  | 268 | `__log!`: the expansion every log call becomes | 4.3 |  | 0.522 |
| walker |  | 4105 | 9 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 64, sub: 0, line: 1020 } |  |  | 0.522 |
| walker |  | 4114 | 9 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 65, sub: 0, line: 1036 } |  |  | 0.522 |
| walker |  | 4123 | 9 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 66, sub: 0, line: 1043 } |  |  | 0.522 |
| walker |  | 4132 | 9 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 67, sub: 0, line: 1050 } |  |  | 0.522 |
| walker |  | 4141 | 9 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 68, sub: 0, line: 1057 } |  |  | 0.522 |
| walker |  | 4150 | 9 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 69, sub: 0, line: 1064 } |  |  | 0.522 |
| walker |  | 4159 | 9 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 70, sub: 0, line: 1071 } |  |  | 0.522 |
| walker |  | 4168 | 9 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 71, sub: 0, line: 1078 } |  |  | 0.522 |
| ns | 4170 |  | 71 | `error!`: head and matcher arms | 4.5 | 4.1 | 0.515 |
| walker |  | 4177 | 9 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 72, sub: 0, line: 1085 } |  |  | 0.515 |
| walker |  | 4186 | 9 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 73, sub: 0, line: 1092 } |  |  | 0.515 |
| walker |  | 4195 | 9 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 75, sub: 0, line: 1107 } |  |  | 0.515 |
| walker |  | 4217 | 22 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 74, sub: 0, line: 1099 } |  |  | 0.516 |
| walker |  | 4232 | 15 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 54, sub: 0, line: 895 } |  |  | 0.516 |
| walker |  | 4247 | 15 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 91, sub: 0, line: 1249 } |  |  | 0.522 |
| walker |  | 4263 | 16 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 60, sub: 0, line: 937 } |  |  | 0.522 |
| walker |  | 4279 | 16 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 75, sub: 0, line: 1107 } |  |  | 0.522 |
| walker |  | 4295 | 16 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 82, sub: 0, line: 1177 } |  |  | 0.522 |
| walker |  | 4313 | 18 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 65, sub: 0, line: 1036 } |  |  | 0.522 |
| walker |  | 4331 | 18 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 71, sub: 0, line: 1078 } |  |  | 0.522 |
| walker |  | 4349 | 18 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 73, sub: 0, line: 1092 } |  |  | 0.522 |
| walker |  | 4368 | 19 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 86, sub: 0, line: 1222 } |  |  | 0.522 |
| walker |  | 4387 | 19 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 87, sub: 0, line: 1229 } |  |  | 0.522 |
| ns | 4406 |  | 236 | Roster: every doc(hidden) internal macro, with its feature fork | 4.6 |  | 0.506 |
| walker |  | 4609 | 222 | Code::CodeKey { rung: Names, file: src/lib.rs, decl: 0, sub: 3, line: 0 } |  |  | 0.525 |
| walker |  | 4617 | 8 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 113, sub: 0, line: 1374 } |  |  | 0.525 |
| walker |  | 4626 | 9 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 114, sub: 0, line: 1395 } |  |  | 0.525 |
| walker |  | 4640 | 14 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 116, sub: 0, line: 1477 } |  |  | 0.528 |
| walker |  | 4661 | 21 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 115, sub: 0, line: 1419 } |  |  | 0.532 |
| walker |  | 4683 | 22 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 112, sub: 0, line: 1349 } |  |  | 0.536 |
| walker |  | 4735 | 52 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 96, sub: 0, line: 1285 } |  |  | 0.536 |
| ns | 4763 |  | 357 | `__log_value!`: the capture-modifier dispatch table | 4.7 | 4.6 | 0.510 |
| walker |  | 4821 | 86 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 100, sub: 0, line: 1294 } |  |  | 0.514 |
| walker |  | 4917 | 96 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 104, sub: 0, line: 1310 } |  |  | 0.519 |
| ns | 4964 |  | 201 | `log_enabled!`: purpose and call forms | 4.9 | 4.1 | 0.509 |
| walker |  | 5013 | 96 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 108, sub: 0, line: 1327 } |  |  | 0.515 |
| walker |  | 5024 | 11 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 95, sub: 0, line: 1283 } |  |  | 0.521 |
| walker |  | 5044 | 20 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 67, sub: 0, line: 1050 } |  |  | 0.521 |
| walker |  | 5064 | 20 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 68, sub: 0, line: 1057 } |  |  | 0.521 |
| walker |  | 5084 | 20 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 69, sub: 0, line: 1064 } |  |  | 0.521 |
| walker |  | 5104 | 20 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 74, sub: 0, line: 1099 } |  |  | 0.521 |
| walker |  | 5126 | 22 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 56, sub: 0, line: 907 } |  |  | 0.521 |
| walker |  | 5148 | 22 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 58, sub: 0, line: 922 } |  |  | 0.521 |
| ns | 5168 |  | 204 | `__private_api`: the three functions macros expand into | 5.1 |  | 0.508 |
| walker |  | 5289 | 141 | Code::CodeKey { rung: Names, file: src/lib.rs, decl: 0, sub: 4, line: 0 } |  |  | 0.526 |
| walker |  | 5301 | 12 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 121, sub: 0, line: 1558 } |  |  | 0.526 |
| walker |  | 5313 | 12 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 125, sub: 0, line: 1575 } |  |  | 0.526 |
| walker |  | 5336 | 23 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 118, sub: 0, line: 1547 } |  |  | 0.526 |
| walker |  | 5364 | 28 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 122, sub: 0, line: 1564 } |  |  | 0.513 |
| ns | 5364 |  | 196 | `log_impl`: where a `Record` is actually built | 5.2 |  | 0.513 |
| walker |  | 5396 | 32 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 119, sub: 0, line: 1551 } |  |  | 0.513 |
| walker |  | 5428 | 32 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 123, sub: 0, line: 1568 } |  |  | 0.513 |
| ns | 5513 |  | 149 | `GlobalLogger`: the zero-sized proxy for the global slot | 5.3 |  | 0.502 |
| walker |  | 5706 | 278 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 127, sub: 0, line: 1611 } |  |  | 0.529 |
| walker |  | 5729 | 23 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 61, sub: 0, line: 944 } |  |  | 0.529 |
| ns | 5737 |  | 224 | `kv_support`: the capture_* functions behind every modifier | 5.4 |  | 0.521 |
| walker |  | 5754 | 25 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 72, sub: 0, line: 1085 } |  |  | 0.521 |
| walker |  | 5781 | 27 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 70, sub: 0, line: 1071 } |  |  | 0.521 |
| walker |  | 5815 | 34 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 66, sub: 0, line: 1043 } |  |  | 0.521 |
| ns | 5843 |  | 106 | `struct Record`: every field | 6.1 | 2.1 | 0.531 |
| walker |  | 5853 | 38 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 126, sub: 0, line: 1581 } |  |  | 0.531 |
| walker |  | 5894 | 41 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 24, sub: 0, line: 561 } |  |  | 0.531 |
| walker |  | 5936 | 42 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 40, sub: 0, line: 714 } |  |  | 0.531 |
| walker |  | 5979 | 43 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 39, sub: 0, line: 706 } |  |  | 0.531 |
| ns | 6024 |  | 181 | Roster: every accessor on `Record` | 6.2 |  | 0.544 |
| walker |  | 6025 | 46 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 118, sub: 0, line: 1547 } |  |  | 0.544 |
| walker |  | 6086 | 61 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 122, sub: 0, line: 1564 } |  |  | 0.544 |
| ns | 6125 |  | 101 | `Metadata` and its accessors | 6.3 | 2.1 | 0.551 |
| walker |  | 6154 | 68 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 85, sub: 0, line: 1211 } |  |  | 0.551 |
| walker |  | 6225 | 71 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 112, sub: 0, line: 1349 } |  |  | 0.551 |
| walker |  | 6305 | 80 | Code::CodeKey { rung: Names, file: src/macros.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.563 |
| ns | 6331 |  | 206 | Roster: `RecordBuilder` and all twelve setters | 6.4 | 2.1 | 0.576 |
| walker |  | 6386 | 81 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 94, sub: 0, line: 1279 } |  |  | 0.576 |
| ns | 6434 |  | 103 | `MetadataBuilder` and its setters | 6.5 | 2.1 | 0.582 |
| ns | 6591 |  | 157 | What structured logging means in `log` | 7.1 |  | 0.576 |
| walker |  | 6729 | 343 | Code::CodeKey { rung: Decl, file: src/macros.rs, decl: 7, sub: 0, line: 390 } |  |  | 0.585 |
| ns | 6774 |  | 183 | The complete list of capture modifiers | 7.2 |  | 0.580 |
| walker |  | 6815 | 86 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 93, sub: 0, line: 1271 } |  |  | 0.580 |
| walker |  | 6913 | 98 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 12, sub: 0, line: 473 } |  |  | 0.580 |
| ns | 6979 |  | 205 | The kv module's structure and complete export list | 7.3 |  | 0.567 |
| ns | 7070 |  | 91 | `trait Source`: the three methods | 7.4 |  | 0.562 |
| ns | 7277 |  | 207 | Roster: every type that implements `Source` | 7.5 |  | 0.555 |
| walker |  | 7325 | 412 | Toml::Operational { file: Cargo.toml } |  |  | 0.615 |
| ns | 7361 |  | 84 | `trait VisitSource`: the visitor side of a `Source` | 7.6 |  | 0.612 |
| walker |  | 7385 | 60 | Code::CodeKey { rung: Names, file: src/serde.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.612 |
| walker |  | 7412 | 27 | Code::CodeKey { rung: Decl, file: src/serde.rs, decl: 3, sub: 0, line: 31 } |  |  | 0.612 |
| walker |  | 7437 | 25 | Code::CodeKey { rung: Decl, file: src/serde.rs, decl: 4, sub: 0, line: 32 } |  |  | 0.612 |
| ns | 7444 |  | 83 | `Value` and `ToValue` | 7.7 |  | 0.607 |
| walker |  | 7466 | 29 | Code::CodeKey { rung: Decl, file: src/serde.rs, decl: 7, sub: 0, line: 126 } |  |  | 0.607 |
| walker |  | 7491 | 25 | Code::CodeKey { rung: Decl, file: src/serde.rs, decl: 8, sub: 0, line: 127 } |  |  | 0.607 |
| walker |  | 7521 | 30 | Code::CodeKey { rung: Decl, file: src/serde.rs, decl: 1, sub: 0, line: 16 } |  |  | 0.607 |
| walker |  | 7543 | 22 | Code::CodeKey { rung: Decl, file: src/serde.rs, decl: 2, sub: 0, line: 17 } |  |  | 0.607 |
| walker |  | 7573 | 30 | Code::CodeKey { rung: Decl, file: src/serde.rs, decl: 5, sub: 0, line: 110 } |  |  | 0.607 |
| walker |  | 7595 | 22 | Code::CodeKey { rung: Decl, file: src/serde.rs, decl: 6, sub: 0, line: 111 } |  |  | 0.607 |
| ns | 7689 |  | 245 | Roster: every constructor and conversion on `Value` | 7.8 |  | 0.597 |
| walker |  | 7759 | 164 | Code::CodeKey { rung: Decl, file: src/macros.rs, decl: 2, sub: 0, line: 163 } |  |  | 0.599 |
| walker |  | 7923 | 164 | Code::CodeKey { rung: Decl, file: src/macros.rs, decl: 3, sub: 0, line: 202 } |  |  | 0.599 |
| ns | 7965 |  | 276 | The primitive conversion tables | 7.9 |  | 0.590 |
| walker |  | 8087 | 164 | Code::CodeKey { rung: Decl, file: src/macros.rs, decl: 4, sub: 0, line: 250 } |  |  | 0.590 |
| ns | 8158 |  | 193 | Roster: every method on `VisitValue` | 7.10 |  | 0.581 |
| walker |  | 8251 | 164 | Code::CodeKey { rung: Decl, file: src/macros.rs, decl: 5, sub: 0, line: 290 } |  |  | 0.581 |
| ns | 8294 |  | 136 | The two `Value` backends: `value_bag` and the dependency-free fallback | 7.11 |  | 0.577 |
| walker |  | 8415 | 164 | Code::CodeKey { rung: Decl, file: src/macros.rs, decl: 6, sub: 0, line: 334 } |  |  | 0.577 |
| ns | 8588 |  | 294 | `Key`, `ToKey`, and their feature-gated support modules | 7.12 |  | 0.567 |
| walker |  | 8692 | 277 | Code::CodeKey { rung: Decl, file: src/macros.rs, decl: 2, sub: 1, line: 163 } |  |  | 0.573 |
| ns | 8760 |  | 172 | `kv::Error`: every variant of the private inner enum | 7.13 |  | 0.565 |
| ns | 8837 |  | 77 | `Source::get` and `Source::count` default implementations | 7.17 | 7.4 | 0.563 |
| walker |  | 8969 | 277 | Code::CodeKey { rung: Decl, file: src/macros.rs, decl: 3, sub: 1, line: 202 } |  |  | 0.563 |
| ns | 9121 |  | 284 | Roster: every test in tests/macros.rs | 8.1 |  | 0.552 |
| walker |  | 9246 | 277 | Code::CodeKey { rung: Decl, file: src/macros.rs, decl: 4, sub: 1, line: 250 } |  |  | 0.552 |
| ns | 9297 |  | 176 | tests/integration.rs: the capturing test logger and what it pins | 8.2 |  | 0.546 |
| ns | 9463 |  | 166 | src/serde.rs: what is serialised, and how | 8.3 |  | 0.542 |
| walker |  | 9523 | 277 | Code::CodeKey { rung: Decl, file: src/macros.rs, decl: 5, sub: 1, line: 290 } |  |  | 0.542 |
| ns | 9607 |  | 144 | CI: the seven jobs | 8.4 |  | 0.536 |
| ns | 9663 |  | 56 | The companion crate that tests compile-time filtering | 8.5 |  | 0.534 |
| ns | 9735 |  | 72 | Roster: the value benchmarks | 8.6 |  | 0.531 |
| walker |  | 9800 | 277 | Code::CodeKey { rung: Decl, file: src/macros.rs, decl: 6, sub: 1, line: 334 } |  |  | 0.531 |
| ns | 9844 |  | 109 | The mutually-exclusive feature guards | 8.7 |  | 0.527 |
| ns | 9889 |  | 45 | CHANGELOG: format and latest release | 8.8 |  | 0.526 |
| ns | 9979 |  | 90 | Roster: the top-level sections of the structured-logging RFC | 8.9 |  | 0.522 |
| walker |  | 9984 | 184 | Code::CodeKey { rung: Names, file: src/__private_api.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.524 |
