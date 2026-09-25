Score(3000)=0.565 I=0.787 C=0.406 ns_rows≤3K=23/67 grid(1000/1442/2080/3000/4327/6240/9000)=0.691/0.594/0.600/0.565/0.521/0.502/0.486

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
| walker |  | 1940 | 13 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 22, sub: 0, line: 547 } |  |  | 0.561 |
| walker |  | 1958 | 18 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 23, sub: 0, line: 553 } |  |  | 0.561 |
| walker |  | 2074 | 116 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 37, sub: 0, line: 684 } |  |  | 0.600 |
| walker |  | 2082 | 8 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 38, sub: 0, line: 698 } |  |  | 0.600 |
| walker |  | 2090 | 8 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 39, sub: 0, line: 706 } |  |  | 0.600 |
| walker |  | 2104 | 14 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 38, sub: 0, line: 698 } |  |  | 0.600 |
| ns | 2117 |  | 183 | Roster: the impls that make Level and LevelFilter comparable and parseable | 2.8 |  | 0.621 |
| walker |  | 2141 | 37 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 47, sub: 0, line: 860 } |  |  | 0.621 |
| walker |  | 2313 | 172 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 28, sub: 0, line: 634 } |  |  | 0.661 |
| walker |  | 2364 | 51 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 44, sub: 0, line: 780 } |  |  | 0.661 |
| ns | 2395 |  | 278 | `STATIC_MAX_LEVEL`: compile-time level resolution | 2.9 | 2.1 | 0.632 |
| walker |  | 2405 | 41 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 24, sub: 0, line: 561 } |  |  | 0.632 |
| walker |  | 2447 | 42 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 40, sub: 0, line: 714 } |  |  | 0.632 |
| ns | 2470 |  | 75 | `trait Log`: the three required methods | 3.1 | 2.1 | 0.616 |
| ns | 2657 |  | 187 | Worked example: a complete `Log` implementation | 3.2 |  | 0.586 |
| walker |  | 2697 | 250 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 49, sub: 0, line: 869 } |  |  | 0.588 |
| walker |  | 2705 | 8 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 50, sub: 0, line: 871 } |  |  | 0.588 |
| walker |  | 2713 | 8 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 51, sub: 0, line: 877 } |  |  | 0.588 |
| walker |  | 2721 | 8 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 52, sub: 0, line: 883 } |  |  | 0.588 |
| walker |  | 2729 | 8 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 53, sub: 0, line: 889 } |  |  | 0.588 |
| walker |  | 2737 | 8 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 54, sub: 0, line: 895 } |  |  | 0.588 |
| walker |  | 2745 | 8 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 55, sub: 0, line: 901 } |  |  | 0.588 |
| walker |  | 2753 | 8 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 56, sub: 0, line: 907 } |  |  | 0.588 |
| walker |  | 2761 | 8 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 57, sub: 0, line: 916 } |  |  | 0.588 |
| walker |  | 2769 | 8 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 58, sub: 0, line: 922 } |  |  | 0.588 |
| walker |  | 2777 | 8 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 59, sub: 0, line: 931 } |  |  | 0.588 |
| walker |  | 2797 | 20 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 60, sub: 0, line: 937 } |  |  | 0.588 |
| ns | 2813 |  | 156 | Why `set_max_level` must be called, and that it defaults to `Off` | 3.4 |  | 0.577 |
| walker |  | 2817 | 20 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 61, sub: 0, line: 944 } |  |  | 0.578 |
| walker |  | 2826 | 9 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 50, sub: 0, line: 871 } |  |  | 0.578 |
| walker |  | 2836 | 10 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 51, sub: 0, line: 877 } |  |  | 0.578 |
| walker |  | 2848 | 12 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 52, sub: 0, line: 883 } |  |  | 0.578 |
| walker |  | 2860 | 12 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 59, sub: 0, line: 931 } |  |  | 0.578 |
| walker |  | 2873 | 13 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 53, sub: 0, line: 889 } |  |  | 0.578 |
| walker |  | 2886 | 13 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 55, sub: 0, line: 901 } |  |  | 0.578 |
| walker |  | 2899 | 13 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 57, sub: 0, line: 916 } |  |  | 0.578 |
| walker |  | 2914 | 15 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 54, sub: 0, line: 895 } |  |  | 0.578 |
| ns | 2919 |  | 106 | Worked example: the conventional `init()` for a logger crate | 3.5 |  | 0.565 |
| walker |  | 2930 | 16 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 60, sub: 0, line: 937 } |  |  | 0.565 |
| walker |  | 2952 | 22 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 56, sub: 0, line: 907 } |  |  | 0.565 |
| walker |  | 2974 | 22 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 58, sub: 0, line: 922 } |  |  | 0.565 |
| walker |  | 2997 | 23 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 61, sub: 0, line: 944 } |  |  | 0.565 |
| walker |  | 3040 | 43 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 39, sub: 0, line: 706 } |  |  | 0.565 |
| ns | 3072 |  | 153 | The `Log` impls the crate itself provides | 3.6 |  | 0.548 |
| ns | 3186 |  | 114 | `logger()`: the acquire-load fast path | 3.7 | 2.2 | 0.537 |
| walker |  | 3341 | 301 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 12, sub: 0, line: 473 } |  |  | 0.565 |
| ns | 3373 |  | 187 | `set_logger_inner`: the compare-exchange install path | 3.8 |  | 0.544 |
| walker |  | 3439 | 98 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 12, sub: 0, line: 473 } |  |  | 0.544 |
| ns | 3518 |  | 145 | Runtime max-level get and set bodies | 3.9 | 2.2 | 0.535 |
| walker |  | 3559 | 120 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 28, sub: 0, line: 634 } |  |  | 0.535 |
| ns | 3599 |  | 81 | no_std wiring | 3.10 |  | 0.529 |
| ns | 3679 |  | 80 | Roster: the seven public macros of src/macros.rs | 4.1 |  | 0.522 |
| walker |  | 3704 | 145 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 25, sub: 0, line: 579 } |  |  | 0.522 |
| ns | 3831 |  | 152 | `log!`: all four call forms | 4.2 | 4.1 | 0.513 |
| walker |  | 3851 | 147 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 41, sub: 0, line: 732 } |  |  | 0.513 |
| ns | 4099 |  | 268 | `__log!`: the expansion every log call becomes | 4.3 |  | 0.501 |
| walker |  | 4142 | 291 | Code::CodeKey { rung: Names, file: src/lib.rs, decl: 0, sub: 2, line: 0 } |  |  | 0.523 |
| walker |  | 4150 | 8 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 113, sub: 0, line: 1374 } |  |  | 0.523 |
| walker |  | 4159 | 9 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 114, sub: 0, line: 1395 } |  |  | 0.523 |
| ns | 4170 |  | 71 | `error!`: head and matcher arms | 4.5 | 4.1 | 0.516 |
| walker |  | 4179 | 20 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 76, sub: 0, line: 1113 } |  |  | 0.516 |
| walker |  | 4199 | 20 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 89, sub: 0, line: 1242 } |  |  | 0.516 |
| walker |  | 4220 | 21 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 115, sub: 0, line: 1419 } |  |  | 0.519 |
| walker |  | 4242 | 22 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 112, sub: 0, line: 1349 } |  |  | 0.521 |
| walker |  | 4280 | 38 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 83, sub: 0, line: 1199 } |  |  | 0.521 |
| walker |  | 4329 | 49 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 78, sub: 0, line: 1157 } |  |  | 0.522 |
| walker |  | 4387 | 58 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 91, sub: 0, line: 1249 } |  |  | 0.530 |
| ns | 4406 |  | 236 | Roster: every doc(hidden) internal macro, with its feature fork | 4.6 |  | 0.514 |
| walker |  | 4450 | 63 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 79, sub: 0, line: 1163 } |  |  | 0.514 |
| walker |  | 4459 | 9 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 80, sub: 0, line: 1165 } |  |  | 0.514 |
| walker |  | 4468 | 9 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 81, sub: 0, line: 1171 } |  |  | 0.514 |
| walker |  | 4477 | 9 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 82, sub: 0, line: 1177 } |  |  | 0.514 |
| walker |  | 4487 | 10 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 80, sub: 0, line: 1165 } |  |  | 0.514 |
| walker |  | 4501 | 14 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 81, sub: 0, line: 1171 } |  |  | 0.514 |
| walker |  | 4587 | 86 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 100, sub: 0, line: 1294 } |  |  | 0.519 |
| walker |  | 4602 | 15 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 91, sub: 0, line: 1249 } |  |  | 0.525 |
| walker |  | 4618 | 16 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 82, sub: 0, line: 1177 } |  |  | 0.525 |
| walker |  | 4714 | 96 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 104, sub: 0, line: 1310 } |  |  | 0.530 |
| ns | 4763 |  | 357 | `__log_value!`: the capture-modifier dispatch table | 4.7 | 4.6 | 0.505 |
| walker |  | 4810 | 96 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 108, sub: 0, line: 1327 } |  |  | 0.511 |
| walker |  | 4913 | 103 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 84, sub: 0, line: 1204 } |  |  | 0.512 |
| walker |  | 4922 | 9 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 85, sub: 0, line: 1211 } |  |  | 0.512 |
| walker |  | 4931 | 9 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 86, sub: 0, line: 1222 } |  |  | 0.512 |
| walker |  | 4940 | 9 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 87, sub: 0, line: 1229 } |  |  | 0.512 |
| walker |  | 4949 | 9 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 88, sub: 0, line: 1236 } |  |  | 0.512 |
| walker |  | 4963 | 14 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 88, sub: 0, line: 1236 } |  |  | 0.512 |
| ns | 4964 |  | 201 | `log_enabled!`: purpose and call forms | 4.9 | 4.1 | 0.502 |
| walker |  | 4982 | 19 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 86, sub: 0, line: 1222 } |  |  | 0.502 |
| walker |  | 5001 | 19 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 87, sub: 0, line: 1229 } |  |  | 0.502 |
| walker |  | 5053 | 52 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 96, sub: 0, line: 1285 } |  |  | 0.502 |
| walker |  | 5064 | 11 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 95, sub: 0, line: 1283 } |  |  | 0.508 |
| ns | 5168 |  | 204 | `__private_api`: the three functions macros expand into | 5.1 |  | 0.495 |
| ns | 5364 |  | 196 | `log_impl`: where a `Record` is actually built | 5.2 |  | 0.482 |
| walker |  | 5426 | 362 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 63, sub: 0, line: 1007 } |  |  | 0.484 |
| walker |  | 5435 | 9 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 64, sub: 0, line: 1020 } |  |  | 0.484 |
| walker |  | 5444 | 9 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 65, sub: 0, line: 1036 } |  |  | 0.484 |
| walker |  | 5453 | 9 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 66, sub: 0, line: 1043 } |  |  | 0.484 |
| walker |  | 5462 | 9 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 67, sub: 0, line: 1050 } |  |  | 0.484 |
| walker |  | 5471 | 9 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 68, sub: 0, line: 1057 } |  |  | 0.484 |
| walker |  | 5480 | 9 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 69, sub: 0, line: 1064 } |  |  | 0.484 |
| walker |  | 5489 | 9 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 70, sub: 0, line: 1071 } |  |  | 0.484 |
| walker |  | 5498 | 9 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 71, sub: 0, line: 1078 } |  |  | 0.484 |
| walker |  | 5507 | 9 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 72, sub: 0, line: 1085 } |  |  | 0.484 |
| ns | 5513 |  | 149 | `GlobalLogger`: the zero-sized proxy for the global slot | 5.3 |  | 0.473 |
| walker |  | 5516 | 9 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 73, sub: 0, line: 1092 } |  |  | 0.473 |
| walker |  | 5525 | 9 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 75, sub: 0, line: 1107 } |  |  | 0.473 |
| walker |  | 5547 | 22 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 74, sub: 0, line: 1099 } |  |  | 0.473 |
| walker |  | 5563 | 16 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 75, sub: 0, line: 1107 } |  |  | 0.473 |
| walker |  | 5581 | 18 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 65, sub: 0, line: 1036 } |  |  | 0.473 |
| walker |  | 5599 | 18 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 71, sub: 0, line: 1078 } |  |  | 0.473 |
| walker |  | 5617 | 18 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 73, sub: 0, line: 1092 } |  |  | 0.473 |
| walker |  | 5637 | 20 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 67, sub: 0, line: 1050 } |  |  | 0.473 |
| walker |  | 5657 | 20 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 68, sub: 0, line: 1057 } |  |  | 0.473 |
| walker |  | 5677 | 20 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 69, sub: 0, line: 1064 } |  |  | 0.473 |
| walker |  | 5697 | 20 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 74, sub: 0, line: 1099 } |  |  | 0.473 |
| walker |  | 5722 | 25 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 72, sub: 0, line: 1085 } |  |  | 0.473 |
| ns | 5737 |  | 224 | `kv_support`: the capture_* functions behind every modifier | 5.4 |  | 0.466 |
| walker |  | 5749 | 27 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 70, sub: 0, line: 1071 } |  |  | 0.466 |
| walker |  | 5783 | 34 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 66, sub: 0, line: 1043 } |  |  | 0.466 |
| ns | 5843 |  | 106 | `struct Record`: every field | 6.1 | 2.1 | 0.477 |
| walker |  | 5851 | 68 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 85, sub: 0, line: 1211 } |  |  | 0.477 |
| walker |  | 5922 | 71 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 112, sub: 0, line: 1349 } |  |  | 0.477 |
| walker |  | 6003 | 81 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 94, sub: 0, line: 1279 } |  |  | 0.477 |
| ns | 6024 |  | 181 | Roster: every accessor on `Record` | 6.2 |  | 0.493 |
| walker |  | 6089 | 86 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 93, sub: 0, line: 1271 } |  |  | 0.493 |
| ns | 6125 |  | 101 | `Metadata` and its accessors | 6.3 | 2.1 | 0.502 |
| walker |  | 6310 | 221 | Code::CodeKey { rung: Names, file: src/lib.rs, decl: 0, sub: 3, line: 0 } |  |  | 0.524 |
| walker |  | 6322 | 12 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 122, sub: 0, line: 1558 } |  |  | 0.524 |
| ns | 6331 |  | 206 | Roster: `RecordBuilder` and all twelve setters | 6.4 | 2.1 | 0.539 |
| walker |  | 6334 | 12 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 126, sub: 0, line: 1575 } |  |  | 0.539 |
| walker |  | 6348 | 14 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 116, sub: 0, line: 1477 } |  |  | 0.542 |
| walker |  | 6371 | 23 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 119, sub: 0, line: 1547 } |  |  | 0.542 |
| walker |  | 6399 | 28 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 123, sub: 0, line: 1564 } |  |  | 0.542 |
| walker |  | 6431 | 32 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 120, sub: 0, line: 1551 } |  |  | 0.542 |
| ns | 6434 |  | 103 | `MetadataBuilder` and its setters | 6.5 | 2.1 | 0.549 |
| walker |  | 6463 | 32 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 124, sub: 0, line: 1568 } |  |  | 0.549 |
| walker |  | 6507 | 44 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 117, sub: 0, line: 1482 } |  |  | 0.550 |
| walker |  | 6545 | 38 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 127, sub: 0, line: 1581 } |  |  | 0.550 |
| walker |  | 6591 | 46 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 119, sub: 0, line: 1547 } |  |  | 0.545 |
| ns | 6591 |  | 157 | What structured logging means in `log` | 7.1 |  | 0.545 |
| ns | 6774 |  | 183 | The complete list of capture modifiers | 7.2 |  | 0.540 |
| walker |  | 6869 | 278 | Code::CodeKey { rung: Decl, file: src/lib.rs, decl: 128, sub: 0, line: 1611 } |  |  | 0.562 |
| walker |  | 6930 | 61 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 123, sub: 0, line: 1564 } |  |  | 0.562 |
| ns | 6979 |  | 205 | The kv module's structure and complete export list | 7.3 |  | 0.550 |
| walker |  | 7056 | 126 | Code::CodeKey { rung: Doc, file: src/lib.rs, decl: 128, sub: 0, line: 1611 } |  |  | 0.550 |
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
| walker |  | 9455 | 60 | Code::CodeKey { rung: Names, file: src/serde.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.472 |
| ns | 9463 |  | 166 | src/serde.rs: what is serialised, and how | 8.3 |  | 0.469 |
| walker |  | 9482 | 27 | Code::CodeKey { rung: Decl, file: src/serde.rs, decl: 3, sub: 0, line: 31 } |  |  | 0.469 |
| walker |  | 9507 | 25 | Code::CodeKey { rung: Decl, file: src/serde.rs, decl: 4, sub: 0, line: 32 } |  |  | 0.469 |
| walker |  | 9536 | 29 | Code::CodeKey { rung: Decl, file: src/serde.rs, decl: 7, sub: 0, line: 126 } |  |  | 0.469 |
| walker |  | 9561 | 25 | Code::CodeKey { rung: Decl, file: src/serde.rs, decl: 8, sub: 0, line: 127 } |  |  | 0.469 |
| walker |  | 9591 | 30 | Code::CodeKey { rung: Decl, file: src/serde.rs, decl: 1, sub: 0, line: 16 } |  |  | 0.469 |
| ns | 9607 |  | 144 | CI: the seven jobs | 8.4 |  | 0.464 |
| walker |  | 9613 | 22 | Code::CodeKey { rung: Decl, file: src/serde.rs, decl: 2, sub: 0, line: 17 } |  |  | 0.464 |
| walker |  | 9643 | 30 | Code::CodeKey { rung: Decl, file: src/serde.rs, decl: 5, sub: 0, line: 110 } |  |  | 0.464 |
| ns | 9663 |  | 56 | The companion crate that tests compile-time filtering | 8.5 |  | 0.462 |
| walker |  | 9665 | 22 | Code::CodeKey { rung: Decl, file: src/serde.rs, decl: 6, sub: 0, line: 111 } |  |  | 0.462 |
| ns | 9735 |  | 72 | Roster: the value benchmarks | 8.6 |  | 0.459 |
| ns | 9844 |  | 109 | The mutually-exclusive feature guards | 8.7 |  | 0.456 |
| ns | 9889 |  | 45 | CHANGELOG: format and latest release | 8.8 |  | 0.454 |
| ns | 9979 |  | 90 | Roster: the top-level sections of the structured-logging RFC | 8.9 |  | 0.452 |
