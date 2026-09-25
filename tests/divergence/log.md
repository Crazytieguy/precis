Score(3000)=0.565 I=0.787 C=0.406 ns_rows≤3K=23/67 grid(1000/1442/2080/3000/4327/6240/9000)=0.697/0.594/0.600/0.565/0.554/0.529/0.486

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
| ns | 799 |  | 234 | Cargo features, part 2: std, the kv family, the serde alias, deprecated aliases | 1.9 | 1.8 | 0.683 |
| ns | 916 |  | 117 | The four optional runtime dependencies | 1.10 |  | 0.691 |
| walker |  | 938 | 257 | Code::CodeKey { rung: Names, file: src/lib.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.697 |
| walker |  | 947 | 9 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 10, sub: 0, line: 462 } |  |  | 0.697 |
| walker |  | 962 | 15 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 2, sub: 0, line: 442 } |  |  | 0.697 |
| walker |  | 993 | 31 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 1, sub: 0, line: 420 } |  |  | 0.697 |
| ns | 1084 |  | 168 | Roster: every public type, trait, module and const in src/lib.rs | 2.1 |  | 0.645 |
| walker |  | 1257 | 264 | Code::CodeKey { rung: Names, file: src/lib.rs, decl: 0, sub: 1, line: 0 } |  |  | 0.659 |
| walker |  | 1282 | 25 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 29, sub: 0, line: 651 } |  |  | 0.659 |
| walker |  | 1288 | 6 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 30, sub: 0, line: 652 } |  |  | 0.622 |
| ns | 1288 |  | 204 | Roster: the seven free functions of the global logger API, with their cfg gates | 2.2 |  | 0.622 |
| walker |  | 1314 | 26 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 13, sub: 0, line: 501 } |  |  | 0.622 |
| walker |  | 1320 | 6 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 14, sub: 0, line: 502 } |  |  | 0.622 |
| walker |  | 1347 | 27 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 62, sub: 0, line: 1002 } |  |  | 0.622 |
| ns | 1374 |  | 86 | `enum Level`: all five variants and their discriminants | 2.3 | 2.1 | 0.594 |
| walker |  | 1375 | 28 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 19, sub: 0, line: 528 } |  |  | 0.594 |
| walker |  | 1403 | 28 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 35, sub: 0, line: 678 } |  |  | 0.594 |
| walker |  | 1434 | 31 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 31, sub: 0, line: 658 } |  |  | 0.594 |
| walker |  | 1440 | 6 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 32, sub: 0, line: 659 } |  |  | 0.594 |
| ns | 1466 |  | 92 | `enum LevelFilter`: all six variants, including `Off` | 2.4 | 2.1 | 0.567 |
| walker |  | 1472 | 32 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 15, sub: 0, line: 508 } |  |  | 0.567 |
| walker |  | 1478 | 6 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 16, sub: 0, line: 509 } |  |  | 0.567 |
| walker |  | 1518 | 40 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 17, sub: 0, line: 515 } |  |  | 0.568 |
| walker |  | 1559 | 41 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 33, sub: 0, line: 665 } |  |  | 0.570 |
| walker |  | 1578 | 19 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 46, sub: 0, line: 856 } |  |  | 0.570 |
| walker |  | 1598 | 20 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 11, sub: 0, line: 464 } |  |  | 0.570 |
| ns | 1650 |  | 184 | Roster: every method on `Level` and `LevelFilter` | 2.5 |  | 0.532 |
| walker |  | 1704 | 106 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 45, sub: 0, line: 841 } |  |  | 0.534 |
| ns | 1782 |  | 132 | Module declarations and the atomics-vs-Cell import fork | 2.6 | 2.1 | 0.512 |
| walker |  | 1818 | 114 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 21, sub: 0, line: 534 } |  |  | 0.533 |
| walker |  | 1826 | 8 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 22, sub: 0, line: 547 } |  |  | 0.533 |
| walker |  | 1834 | 8 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 23, sub: 0, line: 553 } |  |  | 0.533 |
| walker |  | 1847 | 13 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 22, sub: 0, line: 547 } |  |  | 0.533 |
| walker |  | 1865 | 18 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 23, sub: 0, line: 553 } |  |  | 0.533 |
| ns | 1934 |  | 152 | Global state: LOGGER, STATE, the state constants, LOG_LEVEL_NAMES | 2.7 |  | 0.561 |
| walker |  | 1981 | 116 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 37, sub: 0, line: 684 } |  |  | 0.600 |
| walker |  | 1989 | 8 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 38, sub: 0, line: 698 } |  |  | 0.600 |
| walker |  | 1997 | 8 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 39, sub: 0, line: 706 } |  |  | 0.600 |
| walker |  | 2011 | 14 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 38, sub: 0, line: 698 } |  |  | 0.600 |
| walker |  | 2048 | 37 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 47, sub: 0, line: 860 } |  |  | 0.600 |
| ns | 2117 |  | 183 | Roster: the impls that make Level and LevelFilter comparable and parseable | 2.8 |  | 0.621 |
| walker |  | 2220 | 172 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 28, sub: 0, line: 634 } |  |  | 0.661 |
| walker |  | 2271 | 51 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 44, sub: 0, line: 780 } |  |  | 0.661 |
| walker |  | 2312 | 41 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 24, sub: 0, line: 561 } |  |  | 0.661 |
| walker |  | 2354 | 42 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 40, sub: 0, line: 714 } |  |  | 0.661 |
| ns | 2395 |  | 278 | `STATIC_MAX_LEVEL`: compile-time level resolution | 2.9 | 2.1 | 0.632 |
| ns | 2470 |  | 75 | `trait Log`: the three required methods | 3.1 | 2.1 | 0.616 |
| walker |  | 2604 | 250 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 49, sub: 0, line: 869 } |  |  | 0.617 |
| walker |  | 2612 | 8 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 50, sub: 0, line: 871 } |  |  | 0.617 |
| walker |  | 2620 | 8 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 51, sub: 0, line: 877 } |  |  | 0.617 |
| walker |  | 2628 | 8 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 52, sub: 0, line: 883 } |  |  | 0.617 |
| walker |  | 2636 | 8 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 53, sub: 0, line: 889 } |  |  | 0.617 |
| walker |  | 2644 | 8 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 54, sub: 0, line: 895 } |  |  | 0.617 |
| walker |  | 2652 | 8 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 55, sub: 0, line: 901 } |  |  | 0.617 |
| ns | 2657 |  | 187 | Worked example: a complete `Log` implementation | 3.2 |  | 0.588 |
| walker |  | 2660 | 8 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 56, sub: 0, line: 907 } |  |  | 0.588 |
| walker |  | 2668 | 8 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 57, sub: 0, line: 916 } |  |  | 0.588 |
| walker |  | 2676 | 8 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 58, sub: 0, line: 922 } |  |  | 0.588 |
| walker |  | 2684 | 8 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 59, sub: 0, line: 931 } |  |  | 0.588 |
| walker |  | 2704 | 20 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 60, sub: 0, line: 937 } |  |  | 0.588 |
| walker |  | 2724 | 20 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 61, sub: 0, line: 944 } |  |  | 0.588 |
| walker |  | 2733 | 9 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 50, sub: 0, line: 871 } |  |  | 0.588 |
| walker |  | 2743 | 10 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 51, sub: 0, line: 877 } |  |  | 0.588 |
| walker |  | 2755 | 12 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 52, sub: 0, line: 883 } |  |  | 0.588 |
| walker |  | 2767 | 12 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 59, sub: 0, line: 931 } |  |  | 0.588 |
| walker |  | 2780 | 13 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 53, sub: 0, line: 889 } |  |  | 0.588 |
| walker |  | 2793 | 13 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 55, sub: 0, line: 901 } |  |  | 0.588 |
| walker |  | 2806 | 13 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 57, sub: 0, line: 916 } |  |  | 0.588 |
| ns | 2813 |  | 156 | Why `set_max_level` must be called, and that it defaults to `Off` | 3.4 |  | 0.578 |
| walker |  | 2821 | 15 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 54, sub: 0, line: 895 } |  |  | 0.578 |
| walker |  | 2837 | 16 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 60, sub: 0, line: 937 } |  |  | 0.578 |
| walker |  | 2859 | 22 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 56, sub: 0, line: 907 } |  |  | 0.578 |
| walker |  | 2881 | 22 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 58, sub: 0, line: 922 } |  |  | 0.578 |
| walker |  | 2904 | 23 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 61, sub: 0, line: 944 } |  |  | 0.578 |
| ns | 2919 |  | 106 | Worked example: the conventional `init()` for a logger crate | 3.5 |  | 0.565 |
| walker |  | 2947 | 43 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 39, sub: 0, line: 706 } |  |  | 0.565 |
| ns | 3072 |  | 153 | The `Log` impls the crate itself provides | 3.6 |  | 0.548 |
| ns | 3186 |  | 114 | `logger()`: the acquire-load fast path | 3.7 | 2.2 | 0.537 |
| walker |  | 3248 | 301 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 12, sub: 0, line: 473 } |  |  | 0.565 |
| walker |  | 3346 | 98 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 12, sub: 0, line: 473 } |  |  | 0.565 |
| ns | 3373 |  | 187 | `set_logger_inner`: the compare-exchange install path | 3.8 |  | 0.544 |
| ns | 3518 |  | 145 | Runtime max-level get and set bodies | 3.9 | 2.2 | 0.535 |
| ns | 3599 |  | 81 | no_std wiring | 3.10 |  | 0.529 |
| walker |  | 3637 | 291 | Code::CodeKey { rung: Names, file: src/lib.rs, decl: 0, sub: 2, line: 0 } |  |  | 0.552 |
| walker |  | 3645 | 8 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 113, sub: 0, line: 1374 } |  |  | 0.552 |
| walker |  | 3654 | 9 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 114, sub: 0, line: 1395 } |  |  | 0.552 |
| walker |  | 3674 | 20 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 76, sub: 0, line: 1113 } |  |  | 0.552 |
| ns | 3679 |  | 80 | Roster: the seven public macros of src/macros.rs | 4.1 |  | 0.545 |
| walker |  | 3694 | 20 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 89, sub: 0, line: 1242 } |  |  | 0.545 |
| walker |  | 3715 | 21 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 115, sub: 0, line: 1419 } |  |  | 0.547 |
| walker |  | 3737 | 22 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 112, sub: 0, line: 1349 } |  |  | 0.550 |
| walker |  | 3775 | 38 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 83, sub: 0, line: 1199 } |  |  | 0.550 |
| walker |  | 3824 | 49 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 78, sub: 0, line: 1157 } |  |  | 0.551 |
| ns | 3831 |  | 152 | `log!`: all four call forms | 4.2 | 4.1 | 0.541 |
| walker |  | 3882 | 58 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 91, sub: 0, line: 1249 } |  |  | 0.550 |
| walker |  | 3945 | 63 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 79, sub: 0, line: 1163 } |  |  | 0.550 |
| walker |  | 3954 | 9 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 80, sub: 0, line: 1165 } |  |  | 0.550 |
| walker |  | 3963 | 9 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 81, sub: 0, line: 1171 } |  |  | 0.550 |
| walker |  | 3972 | 9 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 82, sub: 0, line: 1177 } |  |  | 0.550 |
| walker |  | 3982 | 10 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 80, sub: 0, line: 1165 } |  |  | 0.550 |
| walker |  | 3996 | 14 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 81, sub: 0, line: 1171 } |  |  | 0.550 |
| walker |  | 4082 | 86 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 100, sub: 0, line: 1294 } |  |  | 0.555 |
| walker |  | 4097 | 15 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 91, sub: 0, line: 1249 } |  |  | 0.562 |
| ns | 4099 |  | 268 | `__log!`: the expansion every log call becomes | 4.3 |  | 0.548 |
| walker |  | 4113 | 16 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 82, sub: 0, line: 1177 } |  |  | 0.548 |
| ns | 4170 |  | 71 | `error!`: head and matcher arms | 4.5 | 4.1 | 0.541 |
| walker |  | 4209 | 96 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 104, sub: 0, line: 1310 } |  |  | 0.547 |
| walker |  | 4305 | 96 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 108, sub: 0, line: 1327 } |  |  | 0.554 |
| ns | 4406 |  | 236 | Roster: every doc(hidden) internal macro, with its feature fork | 4.6 |  | 0.537 |
| walker |  | 4408 | 103 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 84, sub: 0, line: 1204 } |  |  | 0.538 |
| walker |  | 4417 | 9 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 85, sub: 0, line: 1211 } |  |  | 0.538 |
| walker |  | 4426 | 9 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 86, sub: 0, line: 1222 } |  |  | 0.538 |
| walker |  | 4435 | 9 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 87, sub: 0, line: 1229 } |  |  | 0.538 |
| walker |  | 4444 | 9 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 88, sub: 0, line: 1236 } |  |  | 0.538 |
| walker |  | 4458 | 14 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 88, sub: 0, line: 1236 } |  |  | 0.538 |
| walker |  | 4477 | 19 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 86, sub: 0, line: 1222 } |  |  | 0.538 |
| walker |  | 4496 | 19 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 87, sub: 0, line: 1229 } |  |  | 0.538 |
| walker |  | 4548 | 52 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 96, sub: 0, line: 1285 } |  |  | 0.538 |
| walker |  | 4559 | 11 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 95, sub: 0, line: 1283 } |  |  | 0.544 |
| ns | 4763 |  | 357 | `__log_value!`: the capture-modifier dispatch table | 4.7 | 4.6 | 0.518 |
| walker |  | 4921 | 362 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 63, sub: 0, line: 1007 } |  |  | 0.519 |
| walker |  | 4930 | 9 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 64, sub: 0, line: 1020 } |  |  | 0.519 |
| walker |  | 4939 | 9 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 65, sub: 0, line: 1036 } |  |  | 0.519 |
| walker |  | 4948 | 9 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 66, sub: 0, line: 1043 } |  |  | 0.519 |
| walker |  | 4957 | 9 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 67, sub: 0, line: 1050 } |  |  | 0.519 |
| ns | 4964 |  | 201 | `log_enabled!`: purpose and call forms | 4.9 | 4.1 | 0.509 |
| walker |  | 4966 | 9 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 68, sub: 0, line: 1057 } |  |  | 0.509 |
| walker |  | 4975 | 9 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 69, sub: 0, line: 1064 } |  |  | 0.509 |
| walker |  | 4984 | 9 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 70, sub: 0, line: 1071 } |  |  | 0.509 |
| walker |  | 4993 | 9 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 71, sub: 0, line: 1078 } |  |  | 0.509 |
| walker |  | 5002 | 9 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 72, sub: 0, line: 1085 } |  |  | 0.509 |
| walker |  | 5011 | 9 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 73, sub: 0, line: 1092 } |  |  | 0.509 |
| walker |  | 5020 | 9 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 75, sub: 0, line: 1107 } |  |  | 0.509 |
| walker |  | 5042 | 22 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 74, sub: 0, line: 1099 } |  |  | 0.509 |
| walker |  | 5058 | 16 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 75, sub: 0, line: 1107 } |  |  | 0.509 |
| walker |  | 5076 | 18 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 65, sub: 0, line: 1036 } |  |  | 0.509 |
| walker |  | 5094 | 18 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 71, sub: 0, line: 1078 } |  |  | 0.509 |
| walker |  | 5112 | 18 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 73, sub: 0, line: 1092 } |  |  | 0.509 |
| walker |  | 5132 | 20 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 67, sub: 0, line: 1050 } |  |  | 0.509 |
| walker |  | 5152 | 20 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 68, sub: 0, line: 1057 } |  |  | 0.509 |
| ns | 5168 |  | 204 | `__private_api`: the three functions macros expand into | 5.1 |  | 0.496 |
| walker |  | 5172 | 20 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 69, sub: 0, line: 1064 } |  |  | 0.496 |
| walker |  | 5192 | 20 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 74, sub: 0, line: 1099 } |  |  | 0.496 |
| walker |  | 5217 | 25 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 72, sub: 0, line: 1085 } |  |  | 0.496 |
| walker |  | 5244 | 27 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 70, sub: 0, line: 1071 } |  |  | 0.496 |
| walker |  | 5278 | 34 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 66, sub: 0, line: 1043 } |  |  | 0.496 |
| walker |  | 5346 | 68 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 85, sub: 0, line: 1211 } |  |  | 0.496 |
| ns | 5364 |  | 196 | `log_impl`: where a `Record` is actually built | 5.2 |  | 0.484 |
| walker |  | 5417 | 71 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 112, sub: 0, line: 1349 } |  |  | 0.484 |
| walker |  | 5498 | 81 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 94, sub: 0, line: 1279 } |  |  | 0.484 |
| ns | 5513 |  | 149 | `GlobalLogger`: the zero-sized proxy for the global slot | 5.3 |  | 0.473 |
| walker |  | 5584 | 86 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 93, sub: 0, line: 1271 } |  |  | 0.473 |
| ns | 5737 |  | 224 | `kv_support`: the capture_* functions behind every modifier | 5.4 |  | 0.466 |
| walker |  | 5805 | 221 | Code::CodeKey { rung: Names, file: src/lib.rs, decl: 0, sub: 3, line: 0 } |  |  | 0.491 |
| walker |  | 5817 | 12 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 122, sub: 0, line: 1558 } |  |  | 0.491 |
| walker |  | 5829 | 12 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 126, sub: 0, line: 1575 } |  |  | 0.491 |
| walker |  | 5843 | 14 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 116, sub: 0, line: 1477 } |  |  | 0.505 |
| ns | 5843 |  | 106 | `struct Record`: every field | 6.1 | 2.1 | 0.505 |
| walker |  | 5866 | 23 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 119, sub: 0, line: 1547 } |  |  | 0.505 |
| walker |  | 5894 | 28 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 123, sub: 0, line: 1564 } |  |  | 0.505 |
| walker |  | 5926 | 32 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 120, sub: 0, line: 1551 } |  |  | 0.505 |
| walker |  | 5958 | 32 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 124, sub: 0, line: 1568 } |  |  | 0.505 |
| walker |  | 6002 | 44 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 117, sub: 0, line: 1482 } |  |  | 0.507 |
| ns | 6024 |  | 181 | Roster: every accessor on `Record` | 6.2 |  | 0.521 |
| walker |  | 6040 | 38 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 127, sub: 0, line: 1581 } |  |  | 0.521 |
| walker |  | 6086 | 46 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 119, sub: 0, line: 1547 } |  |  | 0.521 |
| ns | 6125 |  | 101 | `Metadata` and its accessors | 6.3 | 2.1 | 0.529 |
| ns | 6331 |  | 206 | Roster: `RecordBuilder` and all twelve setters | 6.4 | 2.1 | 0.544 |
| walker |  | 6364 | 278 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 128, sub: 0, line: 1611 } |  |  | 0.567 |
| walker |  | 6425 | 61 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 123, sub: 0, line: 1564 } |  |  | 0.567 |
| ns | 6434 |  | 103 | `MetadataBuilder` and its setters | 6.5 | 2.1 | 0.573 |
| walker |  | 6545 | 120 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 28, sub: 0, line: 634 } |  |  | 0.573 |
| ns | 6591 |  | 157 | What structured logging means in `log` | 7.1 |  | 0.567 |
| walker |  | 6671 | 126 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 128, sub: 0, line: 1611 } |  |  | 0.567 |
| walker |  | 6716 | 45 | Code::CodeKey { rung: ModuleDoc, file: src/kv/value.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.567 |
| ns | 6774 |  | 183 | The complete list of capture modifiers | 7.2 |  | 0.562 |
| walker |  | 6861 | 145 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 25, sub: 0, line: 579 } |  |  | 0.562 |
| ns | 6979 |  | 205 | The kv module's structure and complete export list | 7.3 |  | 0.550 |
| walker |  | 7008 | 147 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 41, sub: 0, line: 732 } |  |  | 0.550 |
| walker |  | 7056 | 48 | Code::CodeKey { rung: ModuleDoc, file: src/kv/source.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.550 |
| ns | 7070 |  | 91 | `trait Source`: the three methods | 7.4 |  | 0.545 |
| walker |  | 7220 | 164 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 115, sub: 0, line: 1419 } |  |  | 0.545 |
| ns | 7277 |  | 207 | Roster: every type that implements `Source` | 7.5 |  | 0.538 |
| ns | 7361 |  | 84 | `trait VisitSource`: the visitor side of a `Source` | 7.6 |  | 0.535 |
| walker |  | 7395 | 175 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 64, sub: 0, line: 1020 } |  |  | 0.535 |
| ns | 7444 |  | 83 | `Value` and `ToValue` | 7.7 |  | 0.531 |
| walker |  | 7573 | 178 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 92, sub: 0, line: 1262 } |  |  | 0.532 |
| ns | 7689 |  | 245 | Roster: every constructor and conversion on `Value` | 7.8 |  | 0.523 |
| walker |  | 7768 | 195 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 83, sub: 0, line: 1199 } |  |  | 0.523 |
| ns | 7965 |  | 276 | The primitive conversion tables | 7.9 |  | 0.515 |
| walker |  | 7972 | 204 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 26, sub: 0, line: 599 } |  |  | 0.515 |
| ns | 8158 |  | 193 | Roster: every method on `VisitValue` | 7.10 |  | 0.508 |
| walker |  | 8182 | 210 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 27, sub: 0, line: 620 } |  |  | 0.508 |
| ns | 8294 |  | 136 | The two `Value` backends: `value_bag` and the dependency-free fallback | 7.11 |  | 0.504 |
| walker |  | 8395 | 213 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 114, sub: 0, line: 1395 } |  |  | 0.504 |
| ns | 8588 |  | 294 | `Key`, `ToKey`, and their feature-gated support modules | 7.12 |  | 0.495 |
| walker |  | 8611 | 216 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 42, sub: 0, line: 752 } |  |  | 0.495 |
| ns | 8760 |  | 172 | `kv::Error`: every variant of the private inner enum | 7.13 |  | 0.488 |
| ns | 8837 |  | 77 | `Source::get` and `Source::count` default implementations | 7.17 | 7.4 | 0.486 |
| walker |  | 8869 | 258 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 43, sub: 0, line: 774 } |  |  | 0.486 |
| ns | 9121 |  | 284 | Roster: every test in tests/macros.rs | 8.1 |  | 0.477 |
| walker |  | 9128 | 259 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 118, sub: 0, line: 1529 } |  |  | 0.477 |
| ns | 9297 |  | 176 | tests/integration.rs: the capturing test logger and what it pins | 8.2 |  | 0.472 |
| walker |  | 9395 | 267 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 113, sub: 0, line: 1374 } |  |  | 0.472 |
| ns | 9463 |  | 166 | src/serde.rs: what is serialised, and how | 8.3 |  | 0.468 |
| ns | 9607 |  | 144 | CI: the seven jobs | 8.4 |  | 0.462 |
| ns | 9663 |  | 56 | The companion crate that tests compile-time filtering | 8.5 |  | 0.460 |
| ns | 9735 |  | 72 | Roster: the value benchmarks | 8.6 |  | 0.458 |
| walker |  | 9795 | 400 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 78, sub: 0, line: 1157 } |  |  | 0.458 |
| ns | 9844 |  | 109 | The mutually-exclusive feature guards | 8.7 |  | 0.454 |
| ns | 9889 |  | 45 | CHANGELOG: format and latest release | 8.8 |  | 0.453 |
| ns | 9979 |  | 90 | Roster: the top-level sections of the structured-logging RFC | 8.9 |  | 0.451 |
