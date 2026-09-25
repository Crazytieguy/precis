Score(3000)=0.565 I=0.787 C=0.406 ns_rows≤3K=23/67 grid(1000/1442/2080/3000/4327/6240/9000)=0.694/0.594/0.561/0.565/0.561/0.553/0.486

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
| walker |  | 177 | 9 | Fs::DirListing { dir: test_max_level_features } |  |  | 1.000 |
| walker |  | 187 | 10 | Code::CodeKey { rung: ModuleDoc, file: src/kv/key.rs, decl: 0, sub: 0, line: 0 } |  |  | 1.000 |
| walker |  | 215 | 28 | Code::CodeKey { rung: ModuleDoc, file: src/__private_api.rs, decl: 0, sub: 0, line: 0 } |  |  | 1.000 |
| ns | 223 |  | 67 | Facade semantics: the noop fallback | 1.5 |  | 0.934 |
| walker |  | 271 | 56 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.934 |
| ns | 304 |  | 81 | Facade semantics: what a log request is | 1.6 |  | 0.878 |
| walker |  | 335 | 64 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.878 |
| ns | 431 |  | 127 | Cargo.toml package block: version, licence, MSRV, edition | 1.7 |  | 0.779 |
| walker |  | 526 | 191 | Toml::Identity { file: Cargo.toml } |  |  | 0.897 |
| ns | 565 |  | 134 | Cargo features, part 1: the twelve compile-time level filters | 1.8 |  | 0.767 |
| walker |  | 571 | 45 | Code::CodeKey { rung: ModuleDoc, file: src/kv/value.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.767 |
| walker |  | 619 | 48 | Code::CodeKey { rung: ModuleDoc, file: src/kv/source.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.767 |
| walker |  | 738 | 119 | Toml::Dependencies { file: Cargo.toml } |  |  | 0.772 |
| ns | 799 |  | 234 | Cargo features, part 2: std, the kv family, the serde alias, deprecated aliases | 1.9 | 1.8 | 0.683 |
| walker |  | 821 | 83 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.683 |
| ns | 916 |  | 117 | The four optional runtime dependencies | 1.10 |  | 0.691 |
| walker |  | 1078 | 257 | Code::CodeKey { rung: Names, file: src/lib.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.697 |
| ns | 1084 |  | 168 | Roster: every public type, trait, module and const in src/lib.rs | 2.1 |  | 0.645 |
| walker |  | 1087 | 9 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 10, sub: 0, line: 462 } |  |  | 0.645 |
| walker |  | 1102 | 15 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 2, sub: 0, line: 442 } |  |  | 0.645 |
| walker |  | 1133 | 31 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 1, sub: 0, line: 420 } |  |  | 0.645 |
| ns | 1288 |  | 204 | Roster: the seven free functions of the global logger API, with their cfg gates | 2.2 |  | 0.610 |
| ns | 1374 |  | 86 | `enum Level`: all five variants and their discriminants | 2.3 | 2.1 | 0.581 |
| walker |  | 1397 | 264 | Code::CodeKey { rung: Names, file: src/lib.rs, decl: 0, sub: 1, line: 0 } |  |  | 0.594 |
| walker |  | 1416 | 19 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 46, sub: 0, line: 856 } |  |  | 0.594 |
| walker |  | 1436 | 20 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 11, sub: 0, line: 464 } |  |  | 0.594 |
| walker |  | 1461 | 25 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 29, sub: 0, line: 651 } |  |  | 0.594 |
| ns | 1466 |  | 92 | `enum LevelFilter`: all six variants, including `Off` | 2.4 | 2.1 | 0.567 |
| walker |  | 1467 | 6 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 30, sub: 0, line: 652 } |  |  | 0.567 |
| walker |  | 1493 | 26 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 13, sub: 0, line: 501 } |  |  | 0.567 |
| walker |  | 1499 | 6 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 14, sub: 0, line: 502 } |  |  | 0.567 |
| walker |  | 1526 | 27 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 62, sub: 0, line: 1002 } |  |  | 0.567 |
| walker |  | 1554 | 28 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 19, sub: 0, line: 528 } |  |  | 0.567 |
| walker |  | 1582 | 28 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 35, sub: 0, line: 678 } |  |  | 0.567 |
| walker |  | 1613 | 31 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 31, sub: 0, line: 658 } |  |  | 0.567 |
| walker |  | 1619 | 6 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 32, sub: 0, line: 659 } |  |  | 0.567 |
| ns | 1650 |  | 184 | Roster: every method on `Level` and `LevelFilter` | 2.5 |  | 0.530 |
| walker |  | 1651 | 32 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 15, sub: 0, line: 508 } |  |  | 0.530 |
| walker |  | 1657 | 6 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 16, sub: 0, line: 509 } |  |  | 0.530 |
| walker |  | 1694 | 37 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 47, sub: 0, line: 860 } |  |  | 0.530 |
| walker |  | 1734 | 40 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 17, sub: 0, line: 515 } |  |  | 0.531 |
| walker |  | 1775 | 41 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 33, sub: 0, line: 665 } |  |  | 0.532 |
| ns | 1782 |  | 132 | Module declarations and the atomics-vs-Cell import fork | 2.6 | 2.1 | 0.511 |
| walker |  | 1828 | 53 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 44, sub: 0, line: 780 } |  |  | 0.511 |
| walker |  | 1934 | 106 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 45, sub: 0, line: 841 } |  |  | 0.543 |
| ns | 1934 |  | 152 | Global state: LOGGER, STATE, the state constants, LOG_LEVEL_NAMES | 2.7 |  | 0.543 |
| walker |  | 2048 | 114 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 21, sub: 0, line: 534 } |  |  | 0.561 |
| walker |  | 2056 | 8 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 22, sub: 0, line: 547 } |  |  | 0.561 |
| walker |  | 2064 | 8 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 23, sub: 0, line: 553 } |  |  | 0.561 |
| walker |  | 2077 | 13 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 22, sub: 0, line: 547 } |  |  | 0.561 |
| walker |  | 2095 | 18 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 23, sub: 0, line: 553 } |  |  | 0.561 |
| ns | 2117 |  | 183 | Roster: the impls that make Level and LevelFilter comparable and parseable | 2.8 |  | 0.586 |
| walker |  | 2209 | 114 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 37, sub: 0, line: 684 } |  |  | 0.621 |
| walker |  | 2217 | 8 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 38, sub: 0, line: 698 } |  |  | 0.621 |
| walker |  | 2225 | 8 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 39, sub: 0, line: 706 } |  |  | 0.621 |
| walker |  | 2239 | 14 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 38, sub: 0, line: 698 } |  |  | 0.621 |
| ns | 2395 |  | 278 | `STATIC_MAX_LEVEL`: compile-time level resolution | 2.9 | 2.1 | 0.594 |
| walker |  | 2411 | 172 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 28, sub: 0, line: 634 } |  |  | 0.632 |
| ns | 2470 |  | 75 | `trait Log`: the three required methods | 3.1 | 2.1 | 0.616 |
| ns | 2657 |  | 187 | Worked example: a complete `Log` implementation | 3.2 |  | 0.586 |
| walker |  | 2661 | 250 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 49, sub: 0, line: 869 } |  |  | 0.588 |
| walker |  | 2669 | 8 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 50, sub: 0, line: 871 } |  |  | 0.588 |
| walker |  | 2677 | 8 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 51, sub: 0, line: 877 } |  |  | 0.588 |
| walker |  | 2685 | 8 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 52, sub: 0, line: 883 } |  |  | 0.588 |
| walker |  | 2693 | 8 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 53, sub: 0, line: 889 } |  |  | 0.588 |
| walker |  | 2701 | 8 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 54, sub: 0, line: 895 } |  |  | 0.588 |
| walker |  | 2709 | 8 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 55, sub: 0, line: 901 } |  |  | 0.588 |
| walker |  | 2717 | 8 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 56, sub: 0, line: 907 } |  |  | 0.588 |
| walker |  | 2725 | 8 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 57, sub: 0, line: 916 } |  |  | 0.588 |
| walker |  | 2733 | 8 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 58, sub: 0, line: 922 } |  |  | 0.588 |
| walker |  | 2741 | 8 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 59, sub: 0, line: 931 } |  |  | 0.588 |
| walker |  | 2761 | 20 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 60, sub: 0, line: 937 } |  |  | 0.588 |
| walker |  | 2781 | 20 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 61, sub: 0, line: 944 } |  |  | 0.588 |
| walker |  | 2790 | 9 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 50, sub: 0, line: 871 } |  |  | 0.588 |
| walker |  | 2800 | 10 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 51, sub: 0, line: 877 } |  |  | 0.588 |
| walker |  | 2812 | 12 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 52, sub: 0, line: 883 } |  |  | 0.588 |
| ns | 2813 |  | 156 | Why `set_max_level` must be called, and that it defaults to `Off` | 3.4 |  | 0.578 |
| walker |  | 2824 | 12 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 59, sub: 0, line: 931 } |  |  | 0.578 |
| walker |  | 2837 | 13 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 53, sub: 0, line: 889 } |  |  | 0.578 |
| walker |  | 2850 | 13 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 55, sub: 0, line: 901 } |  |  | 0.578 |
| walker |  | 2863 | 13 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 57, sub: 0, line: 916 } |  |  | 0.578 |
| walker |  | 2878 | 15 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 54, sub: 0, line: 895 } |  |  | 0.578 |
| walker |  | 2894 | 16 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 60, sub: 0, line: 937 } |  |  | 0.578 |
| walker |  | 2916 | 22 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 56, sub: 0, line: 907 } |  |  | 0.578 |
| ns | 2919 |  | 106 | Worked example: the conventional `init()` for a logger crate | 3.5 |  | 0.565 |
| walker |  | 2938 | 22 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 58, sub: 0, line: 922 } |  |  | 0.565 |
| walker |  | 2961 | 23 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 61, sub: 0, line: 944 } |  |  | 0.565 |
| walker |  | 3002 | 41 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 24, sub: 0, line: 561 } |  |  | 0.565 |
| walker |  | 3044 | 42 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 40, sub: 0, line: 714 } |  |  | 0.565 |
| ns | 3072 |  | 153 | The `Log` impls the crate itself provides | 3.6 |  | 0.548 |
| walker |  | 3087 | 43 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 39, sub: 0, line: 706 } |  |  | 0.548 |
| ns | 3186 |  | 114 | `logger()`: the acquire-load fast path | 3.7 | 2.2 | 0.537 |
| ns | 3373 |  | 187 | `set_logger_inner`: the compare-exchange install path | 3.8 |  | 0.516 |
| walker |  | 3388 | 301 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 12, sub: 0, line: 473 } |  |  | 0.544 |
| ns | 3518 |  | 145 | Runtime max-level get and set bodies | 3.9 | 2.2 | 0.535 |
| ns | 3599 |  | 81 | no_std wiring | 3.10 |  | 0.529 |
| walker |  | 3679 | 291 | Code::CodeKey { rung: Names, file: src/lib.rs, decl: 0, sub: 2, line: 0 } |  |  | 0.545 |
| ns | 3679 |  | 80 | Roster: the seven public macros of src/macros.rs | 4.1 |  | 0.545 |
| walker |  | 3687 | 8 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 113, sub: 0, line: 1374 } |  |  | 0.545 |
| walker |  | 3696 | 9 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 114, sub: 0, line: 1395 } |  |  | 0.545 |
| walker |  | 3716 | 20 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 76, sub: 0, line: 1113 } |  |  | 0.545 |
| walker |  | 3736 | 20 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 89, sub: 0, line: 1242 } |  |  | 0.545 |
| walker |  | 3757 | 21 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 115, sub: 0, line: 1419 } |  |  | 0.547 |
| walker |  | 3779 | 22 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 112, sub: 0, line: 1349 } |  |  | 0.550 |
| walker |  | 3817 | 38 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 83, sub: 0, line: 1199 } |  |  | 0.550 |
| ns | 3831 |  | 152 | `log!`: all four call forms | 4.2 | 4.1 | 0.541 |
| walker |  | 3866 | 49 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 78, sub: 0, line: 1157 } |  |  | 0.541 |
| walker |  | 3918 | 52 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 96, sub: 0, line: 1285 } |  |  | 0.541 |
| walker |  | 3976 | 58 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 91, sub: 0, line: 1249 } |  |  | 0.550 |
| walker |  | 4039 | 63 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 79, sub: 0, line: 1163 } |  |  | 0.550 |
| walker |  | 4048 | 9 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 80, sub: 0, line: 1165 } |  |  | 0.550 |
| walker |  | 4057 | 9 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 81, sub: 0, line: 1171 } |  |  | 0.550 |
| walker |  | 4066 | 9 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 82, sub: 0, line: 1177 } |  |  | 0.550 |
| walker |  | 4076 | 10 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 80, sub: 0, line: 1165 } |  |  | 0.550 |
| walker |  | 4087 | 11 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 95, sub: 0, line: 1283 } |  |  | 0.554 |
| ns | 4099 |  | 268 | `__log!`: the expansion every log call becomes | 4.3 |  | 0.541 |
| walker |  | 4101 | 14 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 81, sub: 0, line: 1171 } |  |  | 0.541 |
| ns | 4170 |  | 71 | `error!`: head and matcher arms | 4.5 | 4.1 | 0.534 |
| walker |  | 4187 | 86 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 100, sub: 0, line: 1294 } |  |  | 0.540 |
| walker |  | 4202 | 15 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 91, sub: 0, line: 1249 } |  |  | 0.546 |
| walker |  | 4298 | 96 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 104, sub: 0, line: 1310 } |  |  | 0.553 |
| walker |  | 4394 | 96 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 108, sub: 0, line: 1327 } |  |  | 0.561 |
| ns | 4406 |  | 236 | Roster: every doc(hidden) internal macro, with its feature fork | 4.6 |  | 0.543 |
| walker |  | 4410 | 16 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 82, sub: 0, line: 1177 } |  |  | 0.543 |
| walker |  | 4513 | 103 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 84, sub: 0, line: 1204 } |  |  | 0.544 |
| walker |  | 4522 | 9 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 85, sub: 0, line: 1211 } |  |  | 0.544 |
| walker |  | 4531 | 9 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 86, sub: 0, line: 1222 } |  |  | 0.544 |
| walker |  | 4540 | 9 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 87, sub: 0, line: 1229 } |  |  | 0.544 |
| walker |  | 4549 | 9 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 88, sub: 0, line: 1236 } |  |  | 0.544 |
| walker |  | 4563 | 14 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 88, sub: 0, line: 1236 } |  |  | 0.544 |
| walker |  | 4582 | 19 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 86, sub: 0, line: 1222 } |  |  | 0.544 |
| walker |  | 4601 | 19 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 87, sub: 0, line: 1229 } |  |  | 0.544 |
| ns | 4763 |  | 357 | `__log_value!`: the capture-modifier dispatch table | 4.7 | 4.6 | 0.518 |
| walker |  | 4963 | 362 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 63, sub: 0, line: 1007 } |  |  | 0.519 |
| ns | 4964 |  | 201 | `log_enabled!`: purpose and call forms | 4.9 | 4.1 | 0.509 |
| walker |  | 4972 | 9 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 64, sub: 0, line: 1020 } |  |  | 0.509 |
| walker |  | 4981 | 9 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 65, sub: 0, line: 1036 } |  |  | 0.509 |
| walker |  | 4990 | 9 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 66, sub: 0, line: 1043 } |  |  | 0.509 |
| walker |  | 4999 | 9 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 67, sub: 0, line: 1050 } |  |  | 0.509 |
| walker |  | 5008 | 9 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 68, sub: 0, line: 1057 } |  |  | 0.509 |
| walker |  | 5017 | 9 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 69, sub: 0, line: 1064 } |  |  | 0.509 |
| walker |  | 5026 | 9 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 70, sub: 0, line: 1071 } |  |  | 0.509 |
| walker |  | 5035 | 9 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 71, sub: 0, line: 1078 } |  |  | 0.509 |
| walker |  | 5044 | 9 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 72, sub: 0, line: 1085 } |  |  | 0.509 |
| walker |  | 5053 | 9 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 73, sub: 0, line: 1092 } |  |  | 0.509 |
| walker |  | 5062 | 9 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 75, sub: 0, line: 1107 } |  |  | 0.509 |
| walker |  | 5084 | 22 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 74, sub: 0, line: 1099 } |  |  | 0.509 |
| walker |  | 5100 | 16 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 75, sub: 0, line: 1107 } |  |  | 0.509 |
| walker |  | 5118 | 18 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 65, sub: 0, line: 1036 } |  |  | 0.509 |
| walker |  | 5136 | 18 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 71, sub: 0, line: 1078 } |  |  | 0.509 |
| walker |  | 5154 | 18 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 73, sub: 0, line: 1092 } |  |  | 0.509 |
| ns | 5168 |  | 204 | `__private_api`: the three functions macros expand into | 5.1 |  | 0.496 |
| walker |  | 5174 | 20 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 67, sub: 0, line: 1050 } |  |  | 0.496 |
| walker |  | 5194 | 20 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 68, sub: 0, line: 1057 } |  |  | 0.496 |
| walker |  | 5214 | 20 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 69, sub: 0, line: 1064 } |  |  | 0.496 |
| walker |  | 5234 | 20 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 74, sub: 0, line: 1099 } |  |  | 0.496 |
| walker |  | 5259 | 25 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 72, sub: 0, line: 1085 } |  |  | 0.496 |
| walker |  | 5286 | 27 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 70, sub: 0, line: 1071 } |  |  | 0.496 |
| walker |  | 5320 | 34 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 66, sub: 0, line: 1043 } |  |  | 0.496 |
| ns | 5364 |  | 196 | `log_impl`: where a `Record` is actually built | 5.2 |  | 0.484 |
| walker |  | 5388 | 68 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 85, sub: 0, line: 1211 } |  |  | 0.484 |
| walker |  | 5459 | 71 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 112, sub: 0, line: 1349 } |  |  | 0.484 |
| ns | 5513 |  | 149 | `GlobalLogger`: the zero-sized proxy for the global slot | 5.3 |  | 0.473 |
| walker |  | 5680 | 221 | Code::CodeKey { rung: Names, file: src/lib.rs, decl: 0, sub: 3, line: 0 } |  |  | 0.499 |
| walker |  | 5692 | 12 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 122, sub: 0, line: 1558 } |  |  | 0.499 |
| walker |  | 5704 | 12 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 126, sub: 0, line: 1575 } |  |  | 0.499 |
| walker |  | 5718 | 14 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 116, sub: 0, line: 1477 } |  |  | 0.502 |
| ns | 5737 |  | 224 | `kv_support`: the capture_* functions behind every modifier | 5.4 |  | 0.494 |
| walker |  | 5741 | 23 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 119, sub: 0, line: 1547 } |  |  | 0.494 |
| walker |  | 5769 | 28 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 123, sub: 0, line: 1564 } |  |  | 0.494 |
| walker |  | 5801 | 32 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 120, sub: 0, line: 1551 } |  |  | 0.494 |
| walker |  | 5833 | 32 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 124, sub: 0, line: 1568 } |  |  | 0.494 |
| ns | 5843 |  | 106 | `struct Record`: every field | 6.1 | 2.1 | 0.505 |
| walker |  | 5877 | 44 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 117, sub: 0, line: 1482 } |  |  | 0.507 |
| walker |  | 5915 | 38 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 127, sub: 0, line: 1581 } |  |  | 0.507 |
| ns | 6024 |  | 181 | Roster: every accessor on `Record` | 6.2 |  | 0.521 |
| ns | 6125 |  | 101 | `Metadata` and its accessors | 6.3 | 2.1 | 0.529 |
| walker |  | 6193 | 278 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 128, sub: 0, line: 1611 } |  |  | 0.553 |
| walker |  | 6239 | 46 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 119, sub: 0, line: 1547 } |  |  | 0.553 |
| walker |  | 6300 | 61 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 123, sub: 0, line: 1564 } |  |  | 0.553 |
| ns | 6331 |  | 206 | Roster: `RecordBuilder` and all twelve setters | 6.4 | 2.1 | 0.567 |
| walker |  | 6381 | 81 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 94, sub: 0, line: 1279 } |  |  | 0.567 |
| ns | 6434 |  | 103 | `MetadataBuilder` and its setters | 6.5 | 2.1 | 0.573 |
| walker |  | 6467 | 86 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 93, sub: 0, line: 1271 } |  |  | 0.573 |
| walker |  | 6565 | 98 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 12, sub: 0, line: 473 } |  |  | 0.573 |
| ns | 6591 |  | 157 | What structured logging means in `log` | 7.1 |  | 0.567 |
| walker |  | 6685 | 120 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 28, sub: 0, line: 634 } |  |  | 0.567 |
| ns | 6774 |  | 183 | The complete list of capture modifiers | 7.2 |  | 0.562 |
| walker |  | 6811 | 126 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 128, sub: 0, line: 1611 } |  |  | 0.562 |
| walker |  | 6956 | 145 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 25, sub: 0, line: 579 } |  |  | 0.562 |
| ns | 6979 |  | 205 | The kv module's structure and complete export list | 7.3 |  | 0.550 |
| ns | 7070 |  | 91 | `trait Source`: the three methods | 7.4 |  | 0.545 |
| walker |  | 7103 | 147 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 41, sub: 0, line: 732 } |  |  | 0.545 |
| walker |  | 7267 | 164 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 115, sub: 0, line: 1419 } |  |  | 0.545 |
| ns | 7277 |  | 207 | Roster: every type that implements `Source` | 7.5 |  | 0.538 |
| ns | 7361 |  | 84 | `trait VisitSource`: the visitor side of a `Source` | 7.6 |  | 0.535 |
| walker |  | 7442 | 175 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 64, sub: 0, line: 1020 } |  |  | 0.535 |
| ns | 7444 |  | 83 | `Value` and `ToValue` | 7.7 |  | 0.531 |
| walker |  | 7620 | 178 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 92, sub: 0, line: 1262 } |  |  | 0.532 |
| ns | 7689 |  | 245 | Roster: every constructor and conversion on `Value` | 7.8 |  | 0.523 |
| walker |  | 7815 | 195 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 83, sub: 0, line: 1199 } |  |  | 0.523 |
| ns | 7965 |  | 276 | The primitive conversion tables | 7.9 |  | 0.515 |
| walker |  | 8019 | 204 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 26, sub: 0, line: 599 } |  |  | 0.515 |
| ns | 8158 |  | 193 | Roster: every method on `VisitValue` | 7.10 |  | 0.508 |
| walker |  | 8229 | 210 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 27, sub: 0, line: 620 } |  |  | 0.508 |
| ns | 8294 |  | 136 | The two `Value` backends: `value_bag` and the dependency-free fallback | 7.11 |  | 0.504 |
| walker |  | 8442 | 213 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 114, sub: 0, line: 1395 } |  |  | 0.504 |
| ns | 8588 |  | 294 | `Key`, `ToKey`, and their feature-gated support modules | 7.12 |  | 0.495 |
| walker |  | 8658 | 216 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 42, sub: 0, line: 752 } |  |  | 0.495 |
| ns | 8760 |  | 172 | `kv::Error`: every variant of the private inner enum | 7.13 |  | 0.488 |
| ns | 8837 |  | 77 | `Source::get` and `Source::count` default implementations | 7.17 | 7.4 | 0.486 |
| walker |  | 8916 | 258 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 43, sub: 0, line: 774 } |  |  | 0.486 |
| ns | 9121 |  | 284 | Roster: every test in tests/macros.rs | 8.1 |  | 0.477 |
| walker |  | 9175 | 259 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 118, sub: 0, line: 1529 } |  |  | 0.477 |
| ns | 9297 |  | 176 | tests/integration.rs: the capturing test logger and what it pins | 8.2 |  | 0.472 |
| walker |  | 9442 | 267 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 113, sub: 0, line: 1374 } |  |  | 0.472 |
| ns | 9463 |  | 166 | src/serde.rs: what is serialised, and how | 8.3 |  | 0.468 |
| walker |  | 9522 | 80 | Code::CodeKey { rung: Names, file: src/macros.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.476 |
| ns | 9607 |  | 144 | CI: the seven jobs | 8.4 |  | 0.471 |
| ns | 9663 |  | 56 | The companion crate that tests compile-time filtering | 8.5 |  | 0.469 |
| ns | 9735 |  | 72 | Roster: the value benchmarks | 8.6 |  | 0.467 |
| ns | 9844 |  | 109 | The mutually-exclusive feature guards | 8.7 |  | 0.463 |
| walker |  | 9865 | 343 | Code::CodeKey { rung: Decl, file: src/macros.rs, decl: 7, sub: 0, line: 390 } |  |  | 0.470 |
| ns | 9889 |  | 45 | CHANGELOG: format and latest release | 8.8 |  | 0.469 |
| ns | 9979 |  | 90 | Roster: the top-level sections of the structured-logging RFC | 8.9 |  | 0.466 |
| walker |  | 9999 | 134 | Toml::Operational { file: Cargo.toml } |  |  | 0.490 |
