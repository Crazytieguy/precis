Score(3000)=0.577 I=0.790 C=0.422 ns_rows≤3K=23/67 grid(1000/1442/2080/3000/4327/6240/9000)=0.655/0.562/0.620/0.577/0.548/0.565/0.526

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| ns | 34 |  | 34 | Crate identity: README title and one-line description | 1.1 |  | 0.000 |
| walker |  | 46 | 46 | Fs::DirListing { dir: . } |  |  | 0.000 |
| walker |  | 67 | 21 | Fs::DirListing { dir: src } |  |  | 0.000 |
| ns | 80 |  | 46 | Complete root listing | 1.2 |  | 0.574 |
| walker |  | 87 | 20 | Fs::DirListing { dir: src/kv } |  |  | 0.634 |
| walker |  | 90 | 3 | Fs::DirListing { dir: .github } |  |  | 0.634 |
| walker |  | 94 | 4 | Fs::DirListing { dir: .github/workflows } |  |  | 0.635 |
| walker |  | 98 | 4 | Fs::DirListing { dir: benches } |  |  | 0.637 |
| ns | 121 |  | 41 | Complete src/ and src/kv/ listings | 1.3 |  | 0.632 |
| walker |  | 150 | 52 | Markdown::ReadmeHeadline { file: README.md } |  |  | 1.000 |
| ns | 156 |  | 35 | Listings for the remaining directories | 1.4 |  | 0.877 |
| walker |  | 158 | 8 | Fs::DirListing { dir: tests } |  |  | 0.911 |
| walker |  | 168 | 10 | Code::CodeKey { rung: ModuleDoc, file: src/kv/key.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.911 |
| walker |  | 177 | 9 | Fs::DirListing { dir: test_max_level_features } |  |  | 0.965 |
| walker |  | 187 | 10 | Fs::DirListing { dir: rfcs } |  |  | 1.000 |
| walker |  | 215 | 28 | Code::CodeKey { rung: ModuleDoc, file: src/__private_api.rs, decl: 0, sub: 0, line: 0 } |  |  | 1.000 |
| ns | 223 |  | 67 | Facade semantics: the noop fallback | 1.5 |  | 0.934 |
| ns | 304 |  | 81 | Facade semantics: what a log request is | 1.6 |  | 0.878 |
| ns | 431 |  | 127 | Cargo.toml package block: version, licence, MSRV, edition | 1.7 |  | 0.779 |
| walker |  | 472 | 257 | Code::CodeKey { rung: Names, file: src/lib.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.787 |
| walker |  | 481 | 9 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 10, sub: 0, line: 462 } |  |  | 0.787 |
| walker |  | 496 | 15 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 2, sub: 0, line: 442 } |  |  | 0.787 |
| walker |  | 527 | 31 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 1, sub: 0, line: 420 } |  |  | 0.787 |
| ns | 565 |  | 134 | Cargo features, part 1: the twelve compile-time level filters | 1.8 |  | 0.673 |
| walker |  | 583 | 56 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.673 |
| walker |  | 774 | 191 | Toml::Identity { file: Cargo.toml } |  |  | 0.774 |
| ns | 799 |  | 234 | Cargo features, part 2: std, the kv family, the serde alias, deprecated aliases | 1.9 | 1.8 | 0.684 |
| walker |  | 838 | 64 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.684 |
| ns | 916 |  | 117 | The four optional runtime dependencies | 1.10 |  | 0.655 |
| ns | 1084 |  | 168 | Roster: every public type, trait, module and const in src/lib.rs | 2.1 |  | 0.607 |
| walker |  | 1102 | 264 | Code::CodeKey { rung: Names, file: src/lib.rs, decl: 0, sub: 1, line: 0 } |  |  | 0.620 |
| walker |  | 1127 | 25 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 29, sub: 0, line: 651 } |  |  | 0.620 |
| walker |  | 1133 | 6 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 30, sub: 0, line: 652 } |  |  | 0.620 |
| walker |  | 1159 | 26 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 13, sub: 0, line: 501 } |  |  | 0.620 |
| walker |  | 1165 | 6 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 14, sub: 0, line: 502 } |  |  | 0.620 |
| walker |  | 1192 | 27 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 62, sub: 0, line: 1002 } |  |  | 0.621 |
| walker |  | 1220 | 28 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 19, sub: 0, line: 528 } |  |  | 0.621 |
| walker |  | 1248 | 28 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 35, sub: 0, line: 678 } |  |  | 0.621 |
| walker |  | 1279 | 31 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 31, sub: 0, line: 658 } |  |  | 0.621 |
| walker |  | 1285 | 6 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 32, sub: 0, line: 659 } |  |  | 0.621 |
| ns | 1288 |  | 204 | Roster: the seven free functions of the global logger API, with their cfg gates | 2.2 |  | 0.586 |
| walker |  | 1317 | 32 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 15, sub: 0, line: 508 } |  |  | 0.586 |
| walker |  | 1323 | 6 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 16, sub: 0, line: 509 } |  |  | 0.586 |
| walker |  | 1363 | 40 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 17, sub: 0, line: 515 } |  |  | 0.587 |
| ns | 1374 |  | 86 | `enum Level`: all five variants and their discriminants | 2.3 | 2.1 | 0.561 |
| walker |  | 1404 | 41 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 33, sub: 0, line: 665 } |  |  | 0.562 |
| walker |  | 1423 | 19 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 46, sub: 0, line: 856 } |  |  | 0.562 |
| walker |  | 1443 | 20 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 11, sub: 0, line: 464 } |  |  | 0.562 |
| ns | 1466 |  | 92 | `enum LevelFilter`: all six variants, including `Off` | 2.4 | 2.1 | 0.537 |
| walker |  | 1549 | 106 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 45, sub: 0, line: 841 } |  |  | 0.538 |
| ns | 1650 |  | 184 | Roster: every method on `Level` and `LevelFilter` | 2.5 |  | 0.503 |
| walker |  | 1663 | 114 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 21, sub: 0, line: 534 } |  |  | 0.526 |
| walker |  | 1671 | 8 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 22, sub: 0, line: 547 } |  |  | 0.526 |
| walker |  | 1679 | 8 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 23, sub: 0, line: 553 } |  |  | 0.526 |
| ns | 1782 |  | 132 | Module declarations and the atomics-vs-Cell import fork | 2.6 | 2.1 | 0.505 |
| walker |  | 1795 | 116 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 37, sub: 0, line: 684 } |  |  | 0.550 |
| walker |  | 1803 | 8 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 38, sub: 0, line: 698 } |  |  | 0.550 |
| walker |  | 1811 | 8 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 39, sub: 0, line: 706 } |  |  | 0.550 |
| walker |  | 1848 | 37 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 47, sub: 0, line: 860 } |  |  | 0.550 |
| ns | 1934 |  | 152 | Global state: LOGGER, STATE, the state constants, LOG_LEVEL_NAMES | 2.7 |  | 0.576 |
| walker |  | 2020 | 172 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 28, sub: 0, line: 634 } |  |  | 0.620 |
| walker |  | 2071 | 51 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 44, sub: 0, line: 780 } |  |  | 0.620 |
| ns | 2117 |  | 183 | Roster: the impls that make Level and LevelFilter comparable and parseable | 2.8 |  | 0.639 |
| walker |  | 2321 | 250 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 49, sub: 0, line: 869 } |  |  | 0.641 |
| walker |  | 2329 | 8 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 50, sub: 0, line: 871 } |  |  | 0.641 |
| walker |  | 2337 | 8 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 51, sub: 0, line: 877 } |  |  | 0.641 |
| walker |  | 2345 | 8 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 52, sub: 0, line: 883 } |  |  | 0.641 |
| walker |  | 2353 | 8 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 53, sub: 0, line: 889 } |  |  | 0.641 |
| walker |  | 2361 | 8 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 54, sub: 0, line: 895 } |  |  | 0.641 |
| walker |  | 2369 | 8 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 55, sub: 0, line: 901 } |  |  | 0.641 |
| walker |  | 2377 | 8 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 56, sub: 0, line: 907 } |  |  | 0.641 |
| walker |  | 2385 | 8 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 57, sub: 0, line: 916 } |  |  | 0.641 |
| walker |  | 2393 | 8 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 58, sub: 0, line: 922 } |  |  | 0.641 |
| ns | 2395 |  | 278 | `STATIC_MAX_LEVEL`: compile-time level resolution | 2.9 | 2.1 | 0.613 |
| walker |  | 2401 | 8 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 59, sub: 0, line: 931 } |  |  | 0.613 |
| walker |  | 2421 | 20 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 60, sub: 0, line: 937 } |  |  | 0.614 |
| walker |  | 2441 | 20 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 61, sub: 0, line: 944 } |  |  | 0.614 |
| ns | 2470 |  | 75 | `trait Log`: the three required methods | 3.1 | 2.1 | 0.598 |
| ns | 2657 |  | 187 | Worked example: a complete `Log` implementation | 3.2 |  | 0.569 |
| walker |  | 2742 | 301 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 12, sub: 0, line: 473 } |  |  | 0.601 |
| walker |  | 2778 | 36 | Toml::Config { file: Cargo.toml } |  |  | 0.601 |
| ns | 2813 |  | 156 | Why `set_max_level` must be called, and that it defaults to `Off` | 3.4 |  | 0.590 |
| walker |  | 2838 | 60 | Code::CodeKey { rung: Names, file: src/serde.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.590 |
| walker |  | 2865 | 27 | Code::CodeKey { rung: Decl, file: src/serde.rs, decl: 3, sub: 0, line: 31 } |  |  | 0.590 |
| walker |  | 2894 | 29 | Code::CodeKey { rung: Decl, file: src/serde.rs, decl: 7, sub: 0, line: 126 } |  |  | 0.590 |
| ns | 2919 |  | 106 | Worked example: the conventional `init()` for a logger crate | 3.5 |  | 0.577 |
| walker |  | 2924 | 30 | Code::CodeKey { rung: Decl, file: src/serde.rs, decl: 1, sub: 0, line: 16 } |  |  | 0.577 |
| walker |  | 2954 | 30 | Code::CodeKey { rung: Decl, file: src/serde.rs, decl: 5, sub: 0, line: 110 } |  |  | 0.577 |
| walker |  | 2976 | 22 | Code::CodeKey { rung: Decl, file: src/serde.rs, decl: 2, sub: 0, line: 17 } |  |  | 0.577 |
| walker |  | 2998 | 22 | Code::CodeKey { rung: Decl, file: src/serde.rs, decl: 6, sub: 0, line: 111 } |  |  | 0.577 |
| walker |  | 3023 | 25 | Code::CodeKey { rung: Decl, file: src/serde.rs, decl: 4, sub: 0, line: 32 } |  |  | 0.577 |
| walker |  | 3048 | 25 | Code::CodeKey { rung: Decl, file: src/serde.rs, decl: 8, sub: 0, line: 127 } |  |  | 0.577 |
| ns | 3072 |  | 153 | The `Log` impls the crate itself provides | 3.6 |  | 0.560 |
| walker |  | 3167 | 119 | Toml::Dependencies { file: Cargo.toml } |  |  | 0.577 |
| ns | 3186 |  | 114 | `logger()`: the acquire-load fast path | 3.7 | 2.2 | 0.565 |
| ns | 3373 |  | 187 | `set_logger_inner`: the compare-exchange install path | 3.8 |  | 0.544 |
| walker |  | 3458 | 291 | Code::CodeKey { rung: Names, file: src/lib.rs, decl: 0, sub: 2, line: 0 } |  |  | 0.567 |
| walker |  | 3466 | 8 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 113, sub: 0, line: 1374 } |  |  | 0.567 |
| walker |  | 3475 | 9 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 114, sub: 0, line: 1395 } |  |  | 0.567 |
| walker |  | 3495 | 20 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 76, sub: 0, line: 1113 } |  |  | 0.567 |
| walker |  | 3515 | 20 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 89, sub: 0, line: 1242 } |  |  | 0.567 |
| ns | 3518 |  | 145 | Runtime max-level get and set bodies | 3.9 | 2.2 | 0.558 |
| walker |  | 3536 | 21 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 115, sub: 0, line: 1419 } |  |  | 0.561 |
| walker |  | 3558 | 22 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 112, sub: 0, line: 1349 } |  |  | 0.563 |
| walker |  | 3596 | 38 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 83, sub: 0, line: 1199 } |  |  | 0.564 |
| ns | 3599 |  | 81 | no_std wiring | 3.10 |  | 0.558 |
| walker |  | 3645 | 49 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 78, sub: 0, line: 1157 } |  |  | 0.558 |
| ns | 3679 |  | 80 | Roster: the seven public macros of src/macros.rs | 4.1 |  | 0.551 |
| walker |  | 3703 | 58 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 91, sub: 0, line: 1249 } |  |  | 0.559 |
| walker |  | 3766 | 63 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 79, sub: 0, line: 1163 } |  |  | 0.560 |
| walker |  | 3775 | 9 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 80, sub: 0, line: 1165 } |  |  | 0.560 |
| walker |  | 3784 | 9 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 81, sub: 0, line: 1171 } |  |  | 0.560 |
| walker |  | 3793 | 9 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 82, sub: 0, line: 1177 } |  |  | 0.560 |
| ns | 3831 |  | 152 | `log!`: all four call forms | 4.2 | 4.1 | 0.550 |
| walker |  | 3879 | 86 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 100, sub: 0, line: 1294 } |  |  | 0.555 |
| walker |  | 3975 | 96 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 104, sub: 0, line: 1310 } |  |  | 0.561 |
| walker |  | 4071 | 96 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 108, sub: 0, line: 1327 } |  |  | 0.568 |
| ns | 4099 |  | 268 | `__log!`: the expansion every log call becomes | 4.3 |  | 0.555 |
| ns | 4170 |  | 71 | `error!`: head and matcher arms | 4.5 | 4.1 | 0.548 |
| walker |  | 4174 | 103 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 84, sub: 0, line: 1204 } |  |  | 0.548 |
| walker |  | 4183 | 9 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 85, sub: 0, line: 1211 } |  |  | 0.548 |
| walker |  | 4192 | 9 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 86, sub: 0, line: 1222 } |  |  | 0.548 |
| walker |  | 4201 | 9 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 87, sub: 0, line: 1229 } |  |  | 0.548 |
| walker |  | 4210 | 9 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 88, sub: 0, line: 1236 } |  |  | 0.548 |
| walker |  | 4262 | 52 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 96, sub: 0, line: 1285 } |  |  | 0.548 |
| ns | 4406 |  | 236 | Roster: every doc(hidden) internal macro, with its feature fork | 4.6 |  | 0.532 |
| walker |  | 4624 | 362 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 63, sub: 0, line: 1007 } |  |  | 0.533 |
| walker |  | 4633 | 9 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 64, sub: 0, line: 1020 } |  |  | 0.533 |
| walker |  | 4642 | 9 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 65, sub: 0, line: 1036 } |  |  | 0.533 |
| walker |  | 4651 | 9 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 66, sub: 0, line: 1043 } |  |  | 0.533 |
| walker |  | 4660 | 9 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 67, sub: 0, line: 1050 } |  |  | 0.533 |
| walker |  | 4669 | 9 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 68, sub: 0, line: 1057 } |  |  | 0.533 |
| walker |  | 4678 | 9 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 69, sub: 0, line: 1064 } |  |  | 0.533 |
| walker |  | 4687 | 9 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 70, sub: 0, line: 1071 } |  |  | 0.533 |
| walker |  | 4696 | 9 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 71, sub: 0, line: 1078 } |  |  | 0.533 |
| walker |  | 4705 | 9 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 72, sub: 0, line: 1085 } |  |  | 0.533 |
| walker |  | 4714 | 9 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 73, sub: 0, line: 1092 } |  |  | 0.533 |
| walker |  | 4723 | 9 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 75, sub: 0, line: 1107 } |  |  | 0.533 |
| walker |  | 4745 | 22 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 74, sub: 0, line: 1099 } |  |  | 0.533 |
| ns | 4763 |  | 357 | `__log_value!`: the capture-modifier dispatch table | 4.7 | 4.6 | 0.508 |
| walker |  | 4825 | 80 | Code::CodeKey { rung: Names, file: src/macros.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.523 |
| walker |  | 4870 | 45 | Code::CodeKey { rung: ModuleDoc, file: src/kv/value.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.523 |
| walker |  | 4918 | 48 | Code::CodeKey { rung: ModuleDoc, file: src/kv/source.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.523 |
| ns | 4964 |  | 201 | `log_enabled!`: purpose and call forms | 4.9 | 4.1 | 0.513 |
| walker |  | 5139 | 221 | Code::CodeKey { rung: Names, file: src/lib.rs, decl: 0, sub: 3, line: 0 } |  |  | 0.540 |
| walker |  | 5151 | 12 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 122, sub: 0, line: 1558 } |  |  | 0.540 |
| walker |  | 5163 | 12 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 126, sub: 0, line: 1575 } |  |  | 0.540 |
| ns | 5168 |  | 204 | `__private_api`: the three functions macros expand into | 5.1 |  | 0.526 |
| walker |  | 5177 | 14 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 116, sub: 0, line: 1477 } |  |  | 0.530 |
| walker |  | 5200 | 23 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 119, sub: 0, line: 1547 } |  |  | 0.530 |
| walker |  | 5228 | 28 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 123, sub: 0, line: 1564 } |  |  | 0.530 |
| walker |  | 5260 | 32 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 120, sub: 0, line: 1551 } |  |  | 0.530 |
| walker |  | 5292 | 32 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 124, sub: 0, line: 1568 } |  |  | 0.530 |
| walker |  | 5336 | 44 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 117, sub: 0, line: 1482 } |  |  | 0.532 |
| ns | 5364 |  | 196 | `log_impl`: where a `Record` is actually built | 5.2 |  | 0.519 |
| ns | 5513 |  | 149 | `GlobalLogger`: the zero-sized proxy for the global slot | 5.3 |  | 0.507 |
| walker |  | 5614 | 278 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 128, sub: 0, line: 1611 } |  |  | 0.534 |
| walker |  | 5673 | 59 | Code::CodeKey { rung: Names, file: src/kv/error.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.534 |
| walker |  | 5695 | 22 | Code::CodeKey { rung: Decl, file: src/kv/error.rs, decl: 1, sub: 0, line: 4 } |  |  | 0.535 |
| walker |  | 5717 | 22 | Code::CodeKey { rung: Decl, file: src/kv/error.rs, decl: 9, sub: 0, line: 62 } |  |  | 0.535 |
| ns | 5737 |  | 224 | `kv_support`: the capture_* functions behind every modifier | 5.4 |  | 0.526 |
| walker |  | 5745 | 28 | Code::CodeKey { rung: Decl, file: src/kv/error.rs, decl: 7, sub: 0, line: 48 } |  |  | 0.526 |
| walker |  | 5823 | 78 | Code::CodeKey { rung: Decl, file: src/kv/error.rs, decl: 3, sub: 0, line: 19 } |  |  | 0.526 |
| walker |  | 5837 | 14 | Code::CodeKey { rung: Decl, file: src/kv/error.rs, decl: 5, sub: 0, line: 28 } |  |  | 0.526 |
| ns | 5843 |  | 106 | `struct Record`: every field | 6.1 | 2.1 | 0.536 |
| walker |  | 5851 | 14 | Code::CodeKey { rung: Decl, file: src/kv/error.rs, decl: 6, sub: 0, line: 36 } |  |  | 0.536 |
| ns | 6024 |  | 181 | Roster: every accessor on `Record` | 6.2 |  | 0.549 |
| walker |  | 6058 | 207 | Code::CodeKey { rung: Names, file: src/__private_api.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.551 |
| walker |  | 6066 | 8 | Code::CodeKey { rung: Decl, file: src/__private_api.rs, decl: 6, sub: 0, line: 38 } |  |  | 0.552 |
| walker |  | 6075 | 9 | Code::CodeKey { rung: Decl, file: src/__private_api.rs, decl: 14, sub: 0, line: 107 } |  |  | 0.553 |
| walker |  | 6086 | 11 | Code::CodeKey { rung: Decl, file: src/__private_api.rs, decl: 1, sub: 0, line: 9 } |  |  | 0.553 |
| walker |  | 6122 | 36 | Code::CodeKey { rung: Decl, file: src/__private_api.rs, decl: 2, sub: 0, line: 21 } |  |  | 0.553 |
| ns | 6125 |  | 101 | `Metadata` and its accessors | 6.3 | 2.1 | 0.560 |
| walker |  | 6128 | 6 | Code::CodeKey { rung: Decl, file: src/__private_api.rs, decl: 3, sub: 0, line: 22 } |  |  | 0.560 |
| walker |  | 6166 | 38 | Code::CodeKey { rung: Decl, file: src/__private_api.rs, decl: 4, sub: 0, line: 28 } |  |  | 0.560 |
| walker |  | 6172 | 6 | Code::CodeKey { rung: Decl, file: src/__private_api.rs, decl: 5, sub: 0, line: 29 } |  |  | 0.560 |
| walker |  | 6228 | 56 | Code::CodeKey { rung: Decl, file: src/__private_api.rs, decl: 7, sub: 0, line: 41 } |  |  | 0.565 |
| walker |  | 6321 | 93 | Code::CodeKey { rung: Decl, file: src/__private_api.rs, decl: 12, sub: 0, line: 84 } |  |  | 0.578 |
| ns | 6331 |  | 206 | Roster: `RecordBuilder` and all twelve setters | 6.4 | 2.1 | 0.591 |
| walker |  | 6404 | 83 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.591 |
| ns | 6434 |  | 103 | `MetadataBuilder` and its setters | 6.5 | 2.1 | 0.596 |
| walker |  | 6479 | 75 | Code::CodeKey { rung: Decl, file: src/__private_api.rs, decl: 11, sub: 0, line: 56 } |  |  | 0.600 |
| ns | 6591 |  | 157 | What structured logging means in `log` | 7.1 |  | 0.594 |
| walker |  | 6645 | 166 | Code::CodeKey { rung: Names, file: src/kv/key.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.594 |
| walker |  | 6664 | 19 | Code::CodeKey { rung: Decl, file: src/kv/key.rs, decl: 18, sub: 0, line: 81 } |  |  | 0.594 |
| walker |  | 6684 | 20 | Code::CodeKey { rung: Decl, file: src/kv/key.rs, decl: 16, sub: 0, line: 75 } |  |  | 0.594 |
| walker |  | 6705 | 21 | Code::CodeKey { rung: Decl, file: src/kv/key.rs, decl: 1, sub: 0, line: 7 } |  |  | 0.594 |
| walker |  | 6727 | 22 | Code::CodeKey { rung: Decl, file: src/kv/key.rs, decl: 5, sub: 0, line: 21 } |  |  | 0.594 |
| walker |  | 6750 | 23 | Code::CodeKey { rung: Decl, file: src/kv/key.rs, decl: 20, sub: 0, line: 87 } |  |  | 0.594 |
| walker |  | 6774 | 24 | Code::CodeKey { rung: Decl, file: src/kv/key.rs, decl: 7, sub: 0, line: 27 } |  |  | 0.588 |
| ns | 6774 |  | 183 | The complete list of capture modifiers | 7.2 |  | 0.588 |
| walker |  | 6802 | 28 | Code::CodeKey { rung: Decl, file: src/kv/key.rs, decl: 14, sub: 0, line: 69 } |  |  | 0.588 |
| walker |  | 6848 | 46 | Code::CodeKey { rung: Decl, file: src/kv/key.rs, decl: 3, sub: 0, line: 12 } |  |  | 0.588 |
| walker |  | 6902 | 54 | Code::CodeKey { rung: Decl, file: src/kv/key.rs, decl: 9, sub: 0, line: 36 } |  |  | 0.589 |
| walker |  | 6970 | 68 | Code::CodeKey { rung: Decl, file: src/kv/key.rs, decl: 10, sub: 0, line: 42 } |  |  | 0.589 |
| ns | 6979 |  | 205 | The kv module's structure and complete export list | 7.3 |  | 0.577 |
| ns | 7070 |  | 91 | `trait Source`: the three methods | 7.4 |  | 0.571 |
| ns | 7277 |  | 207 | Roster: every type that implements `Source` | 7.5 |  | 0.564 |
| ns | 7361 |  | 84 | `trait VisitSource`: the visitor side of a `Source` | 7.6 |  | 0.561 |
| walker |  | 7434 | 464 | Code::CodeKey { rung: Names, file: src/kv/value.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.561 |
| ns | 7444 |  | 83 | `Value` and `ToValue` | 7.7 |  | 0.557 |
| walker |  | 7455 | 21 | Code::CodeKey { rung: Decl, file: src/kv/value.rs, decl: 1, sub: 0, line: 11 } |  |  | 0.559 |
| walker |  | 7476 | 21 | Code::CodeKey { rung: Decl, file: src/kv/value.rs, decl: 31, sub: 0, line: 264 } |  |  | 0.559 |
| walker |  | 7498 | 22 | Code::CodeKey { rung: Decl, file: src/kv/value.rs, decl: 29, sub: 0, line: 258 } |  |  | 0.559 |
| walker |  | 7520 | 22 | Code::CodeKey { rung: Decl, file: src/kv/value.rs, decl: 33, sub: 0, line: 270 } |  |  | 0.559 |
| walker |  | 7544 | 24 | Code::CodeKey { rung: Decl, file: src/kv/value.rs, decl: 5, sub: 0, line: 25 } |  |  | 0.559 |
| walker |  | 7568 | 24 | Code::CodeKey { rung: Decl, file: src/kv/value.rs, decl: 7, sub: 0, line: 118 } |  |  | 0.564 |
| walker |  | 7596 | 28 | Code::CodeKey { rung: Decl, file: src/kv/value.rs, decl: 19, sub: 0, line: 222 } |  |  | 0.564 |
| walker |  | 7626 | 30 | Code::CodeKey { rung: Decl, file: src/kv/value.rs, decl: 21, sub: 0, line: 228 } |  |  | 0.564 |
| walker |  | 7669 | 43 | Code::CodeKey { rung: Decl, file: src/kv/value.rs, decl: 23, sub: 0, line: 234 } |  |  | 0.564 |
| ns | 7689 |  | 245 | Roster: every constructor and conversion on `Value` | 7.8 |  | 0.554 |
| walker |  | 7712 | 43 | Code::CodeKey { rung: Decl, file: src/kv/value.rs, decl: 35, sub: 0, line: 276 } |  |  | 0.554 |
| walker |  | 7758 | 46 | Code::CodeKey { rung: Decl, file: src/kv/value.rs, decl: 3, sub: 0, line: 16 } |  |  | 0.554 |
| walker |  | 7783 | 25 | Code::CodeKey { rung: Decl, file: src/kv/value.rs, decl: 24, sub: 0, line: 236 } |  |  | 0.554 |
| walker |  | 7834 | 51 | Code::CodeKey { rung: Decl, file: src/kv/value.rs, decl: 27, sub: 0, line: 251 } |  |  | 0.554 |
| walker |  | 7890 | 56 | Code::CodeKey { rung: Decl, file: src/kv/value.rs, decl: 25, sub: 0, line: 244 } |  |  | 0.554 |
| walker |  | 7954 | 64 | Code::CodeKey { rung: Decl, file: src/kv/value.rs, decl: 40, sub: 0, line: 376 } |  |  | 0.555 |
| ns | 7965 |  | 276 | The primitive conversion tables | 7.9 |  | 0.546 |
| walker |  | 7967 | 13 | Code::CodeKey { rung: Decl, file: src/kv/value.rs, decl: 41, sub: 0, line: 378 } |  |  | 0.547 |
| walker |  | 8056 | 89 | Code::CodeKey { rung: Decl, file: src/kv/value.rs, decl: 79, sub: 0, line: 1126 } |  |  | 0.547 |
| walker |  | 8145 | 89 | Code::CodeKey { rung: Decl, file: src/kv/value.rs, decl: 80, sub: 0, line: 1136 } |  |  | 0.547 |
| ns | 8158 |  | 193 | Roster: every method on `VisitValue` | 7.10 |  | 0.539 |
| walker |  | 8237 | 92 | Code::CodeKey { rung: Decl, file: src/kv/value.rs, decl: 81, sub: 0, line: 1146 } |  |  | 0.539 |
| ns | 8294 |  | 136 | The two `Value` backends: `value_bag` and the dependency-free fallback | 7.11 |  | 0.535 |
| walker |  | 8330 | 93 | Code::CodeKey { rung: Decl, file: src/kv/value.rs, decl: 82, sub: 0, line: 1155 } |  |  | 0.535 |
| walker |  | 8423 | 93 | Code::CodeKey { rung: Decl, file: src/kv/value.rs, decl: 83, sub: 0, line: 1166 } |  |  | 0.535 |
| ns | 8588 |  | 294 | `Key`, `ToKey`, and their feature-gated support modules | 7.12 |  | 0.534 |
| walker |  | 8595 | 172 | Code::CodeKey { rung: Decl, file: src/kv/value.rs, decl: 71, sub: 0, line: 1051 } |  |  | 0.534 |
| walker |  | 8658 | 63 | Code::CodeKey { rung: Decl, file: src/kv/value.rs, decl: 72, sub: 0, line: 1053 } |  |  | 0.534 |
| walker |  | 8721 | 63 | Code::CodeKey { rung: Decl, file: src/kv/value.rs, decl: 73, sub: 0, line: 1063 } |  |  | 0.534 |
| ns | 8760 |  | 172 | `kv::Error`: every variant of the private inner enum | 7.13 |  | 0.528 |
| walker |  | 8787 | 66 | Code::CodeKey { rung: Decl, file: src/kv/value.rs, decl: 76, sub: 0, line: 1093 } |  |  | 0.528 |
| ns | 8837 |  | 77 | `Source::get` and `Source::count` default implementations | 7.17 | 7.4 | 0.526 |
| walker |  | 8854 | 67 | Code::CodeKey { rung: Decl, file: src/kv/value.rs, decl: 74, sub: 0, line: 1073 } |  |  | 0.526 |
| walker |  | 8921 | 67 | Code::CodeKey { rung: Decl, file: src/kv/value.rs, decl: 75, sub: 0, line: 1083 } |  |  | 0.526 |
| walker |  | 8989 | 68 | Code::CodeKey { rung: Decl, file: src/kv/value.rs, decl: 77, sub: 0, line: 1103 } |  |  | 0.526 |
| walker |  | 9057 | 68 | Code::CodeKey { rung: Decl, file: src/kv/value.rs, decl: 78, sub: 0, line: 1112 } |  |  | 0.526 |
| ns | 9121 |  | 284 | Roster: every test in tests/macros.rs | 8.1 |  | 0.516 |
| ns | 9297 |  | 176 | tests/integration.rs: the capturing test logger and what it pins | 8.2 |  | 0.510 |
| walker |  | 9303 | 246 | Code::CodeKey { rung: ModuleDoc, file: src/lib.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.528 |
| ns | 9463 |  | 166 | src/serde.rs: what is serialised, and how | 8.3 |  | 0.525 |
| ns | 9607 |  | 144 | CI: the seven jobs | 8.4 |  | 0.519 |
| walker |  | 9646 | 343 | Code::CodeKey { rung: Decl, file: src/macros.rs, decl: 7, sub: 0, line: 390 } |  |  | 0.526 |
| ns | 9663 |  | 56 | The companion crate that tests compile-time filtering | 8.5 |  | 0.523 |
| ns | 9735 |  | 72 | Roster: the value benchmarks | 8.6 |  | 0.521 |
| ns | 9844 |  | 109 | The mutually-exclusive feature guards | 8.7 |  | 0.517 |
| walker |  | 9853 | 207 | Code::CodeKey { rung: Names, file: src/kv/mod.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.537 |
| ns | 9889 |  | 45 | CHANGELOG: format and latest release | 8.8 |  | 0.536 |
| ns | 9979 |  | 90 | Roster: the top-level sections of the structured-logging RFC | 8.9 |  | 0.533 |
