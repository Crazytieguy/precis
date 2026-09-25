Score(3000)=0.577 I=0.790 C=0.422 ns_rows≤3K=23/67 grid(1000/1442/2080/3000/4327/6240/9000)=0.655/0.562/0.620/0.577/0.548/0.556/0.560

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| ns | 34 |  | 34 | Crate identity: README title and one-line description | 1.1 |  | 0.000 |
| walker |  | 46 | 46 | listing of '.' |  |  | 0.000 |
| walker |  | 67 | 21 | listing of 'src' |  |  | 0.000 |
| walker |  | 70 | 3 | listing of '.github' |  |  | 0.000 |
| walker |  | 74 | 4 | listing of '.github/workflows' |  |  | 0.000 |
| ns | 80 |  | 46 | Complete root listing | 1.2 |  | 0.575 |
| walker |  | 94 | 20 | listing of 'src/kv' |  |  | 0.635 |
| walker |  | 98 | 4 | listing of 'benches' |  |  | 0.637 |
| ns | 121 |  | 41 | Complete src/ and src/kv/ listings | 1.3 |  | 0.632 |
| walker |  | 150 | 52 | README headline in README.md |  |  | 1.000 |
| ns | 156 |  | 35 | Listings for the remaining directories | 1.4 |  | 0.877 |
| walker |  | 158 | 8 | listing of 'tests' |  |  | 0.911 |
| walker |  | 168 | 10 | rust module doc src/kv/key.rs |  |  | 0.911 |
| walker |  | 177 | 9 | listing of 'test_max_level_features' |  |  | 0.965 |
| walker |  | 187 | 10 | listing of 'rfcs' |  |  | 1.000 |
| ns | 223 |  | 67 | Facade semantics: the noop fallback | 1.5 |  | 0.934 |
| ns | 304 |  | 81 | Facade semantics: what a log request is | 1.6 |  | 0.878 |
| ns | 431 |  | 127 | Cargo.toml package block: version, licence, MSRV, edition | 1.7 |  | 0.779 |
| walker |  | 444 | 257 | rust names src/lib.rs |  |  | 0.787 |
| walker |  | 453 | 9 | rust decl src/lib.rs:462 |  |  | 0.787 |
| walker |  | 468 | 15 | rust decl src/lib.rs:442 |  |  | 0.787 |
| walker |  | 499 | 31 | rust decl src/lib.rs:420 |  |  | 0.787 |
| walker |  | 555 | 56 | headings outline in README.md |  |  | 0.787 |
| ns | 565 |  | 134 | Cargo features, part 1: the twelve compile-time level filters | 1.8 |  | 0.673 |
| walker |  | 746 | 191 | [package] in Cargo.toml |  |  | 0.774 |
| ns | 799 |  | 234 | Cargo features, part 2: std, the kv family, the serde alias, deprecated aliases | 1.9 | 1.8 | 0.684 |
| walker |  | 810 | 64 | README.md section #0 |  |  | 0.684 |
| ns | 916 |  | 117 | The four optional runtime dependencies | 1.10 |  | 0.655 |
| walker |  | 1074 | 264 | rust names src/lib.rs #1 |  |  | 0.659 |
| ns | 1084 |  | 168 | Roster: every public type, trait, module and const in src/lib.rs | 2.1 |  | 0.620 |
| walker |  | 1099 | 25 | rust decl src/lib.rs:651 |  |  | 0.620 |
| walker |  | 1105 | 6 | rust decl src/lib.rs:652 |  |  | 0.620 |
| walker |  | 1131 | 26 | rust decl src/lib.rs:501 |  |  | 0.620 |
| walker |  | 1137 | 6 | rust decl src/lib.rs:502 |  |  | 0.620 |
| walker |  | 1164 | 27 | rust decl src/lib.rs:1002 |  |  | 0.621 |
| walker |  | 1192 | 28 | rust decl src/lib.rs:528 |  |  | 0.621 |
| walker |  | 1220 | 28 | rust decl src/lib.rs:678 |  |  | 0.621 |
| walker |  | 1251 | 31 | rust decl src/lib.rs:658 |  |  | 0.621 |
| walker |  | 1257 | 6 | rust decl src/lib.rs:659 |  |  | 0.621 |
| ns | 1288 |  | 204 | Roster: the seven free functions of the global logger API, with their cfg gates | 2.2 |  | 0.586 |
| walker |  | 1289 | 32 | rust decl src/lib.rs:508 |  |  | 0.586 |
| walker |  | 1295 | 6 | rust decl src/lib.rs:509 |  |  | 0.586 |
| walker |  | 1335 | 40 | rust decl src/lib.rs:515 |  |  | 0.587 |
| ns | 1374 |  | 86 | `enum Level`: all five variants and their discriminants | 2.3 | 2.1 | 0.561 |
| walker |  | 1376 | 41 | rust decl src/lib.rs:665 |  |  | 0.562 |
| walker |  | 1395 | 19 | rust decl src/lib.rs:856 |  |  | 0.562 |
| walker |  | 1415 | 20 | rust decl src/lib.rs:464 |  |  | 0.562 |
| ns | 1466 |  | 92 | `enum LevelFilter`: all six variants, including `Off` | 2.4 | 2.1 | 0.537 |
| walker |  | 1521 | 106 | rust decl src/lib.rs:841 |  |  | 0.538 |
| walker |  | 1635 | 114 | rust decl src/lib.rs:534 |  |  | 0.540 |
| walker |  | 1643 | 8 | rust decl src/lib.rs:547 |  |  | 0.540 |
| ns | 1650 |  | 184 | Roster: every method on `Level` and `LevelFilter` | 2.5 |  | 0.526 |
| walker |  | 1651 | 8 | rust decl src/lib.rs:553 |  |  | 0.526 |
| walker |  | 1767 | 116 | rust decl src/lib.rs:684 |  |  | 0.573 |
| walker |  | 1775 | 8 | rust decl src/lib.rs:698 |  |  | 0.573 |
| ns | 1782 |  | 132 | Module declarations and the atomics-vs-Cell import fork | 2.6 | 2.1 | 0.550 |
| walker |  | 1783 | 8 | rust decl src/lib.rs:706 |  |  | 0.550 |
| walker |  | 1820 | 37 | rust decl src/lib.rs:860 |  |  | 0.550 |
| ns | 1934 |  | 152 | Global state: LOGGER, STATE, the state constants, LOG_LEVEL_NAMES | 2.7 |  | 0.576 |
| walker |  | 1992 | 172 | rust decl src/lib.rs:634 |  |  | 0.620 |
| walker |  | 2043 | 51 | rust decl src/lib.rs:780 |  |  | 0.620 |
| ns | 2117 |  | 183 | Roster: the impls that make Level and LevelFilter comparable and parseable | 2.8 |  | 0.639 |
| walker |  | 2293 | 250 | rust decl src/lib.rs:869 |  |  | 0.641 |
| walker |  | 2301 | 8 | rust decl src/lib.rs:871 |  |  | 0.641 |
| walker |  | 2309 | 8 | rust decl src/lib.rs:877 |  |  | 0.641 |
| walker |  | 2317 | 8 | rust decl src/lib.rs:883 |  |  | 0.641 |
| walker |  | 2325 | 8 | rust decl src/lib.rs:889 |  |  | 0.641 |
| walker |  | 2333 | 8 | rust decl src/lib.rs:895 |  |  | 0.641 |
| walker |  | 2341 | 8 | rust decl src/lib.rs:901 |  |  | 0.641 |
| walker |  | 2349 | 8 | rust decl src/lib.rs:907 |  |  | 0.641 |
| walker |  | 2357 | 8 | rust decl src/lib.rs:916 |  |  | 0.641 |
| walker |  | 2365 | 8 | rust decl src/lib.rs:922 |  |  | 0.641 |
| walker |  | 2373 | 8 | rust decl src/lib.rs:931 |  |  | 0.641 |
| walker |  | 2393 | 20 | rust decl src/lib.rs:937 |  |  | 0.641 |
| ns | 2395 |  | 278 | `STATIC_MAX_LEVEL`: compile-time level resolution | 2.9 | 2.1 | 0.614 |
| walker |  | 2413 | 20 | rust decl src/lib.rs:944 |  |  | 0.614 |
| ns | 2470 |  | 75 | `trait Log`: the three required methods | 3.1 | 2.1 | 0.598 |
| ns | 2657 |  | 187 | Worked example: a complete `Log` implementation | 3.2 |  | 0.569 |
| walker |  | 2714 | 301 | rust decl src/lib.rs:473 |  |  | 0.601 |
| walker |  | 2750 | 36 | manifest config in Cargo.toml |  |  | 0.601 |
| walker |  | 2810 | 60 | rust names src/serde.rs |  |  | 0.601 |
| ns | 2813 |  | 156 | Why `set_max_level` must be called, and that it defaults to `Off` | 3.4 |  | 0.590 |
| walker |  | 2837 | 27 | rust decl src/serde.rs:31 |  |  | 0.590 |
| walker |  | 2866 | 29 | rust decl src/serde.rs:126 |  |  | 0.590 |
| walker |  | 2896 | 30 | rust decl src/serde.rs:16 |  |  | 0.590 |
| ns | 2919 |  | 106 | Worked example: the conventional `init()` for a logger crate | 3.5 |  | 0.577 |
| walker |  | 2926 | 30 | rust decl src/serde.rs:110 |  |  | 0.577 |
| walker |  | 2948 | 22 | rust decl src/serde.rs:17 |  |  | 0.577 |
| walker |  | 2970 | 22 | rust decl src/serde.rs:111 |  |  | 0.577 |
| walker |  | 2995 | 25 | rust decl src/serde.rs:32 |  |  | 0.577 |
| walker |  | 3020 | 25 | rust decl src/serde.rs:127 |  |  | 0.577 |
| ns | 3072 |  | 153 | The `Log` impls the crate itself provides | 3.6 |  | 0.560 |
| walker |  | 3139 | 119 | [dependencies] in Cargo.toml |  |  | 0.577 |
| ns | 3186 |  | 114 | `logger()`: the acquire-load fast path | 3.7 | 2.2 | 0.565 |
| ns | 3373 |  | 187 | `set_logger_inner`: the compare-exchange install path | 3.8 |  | 0.544 |
| walker |  | 3430 | 291 | rust names src/lib.rs #2 |  |  | 0.567 |
| walker |  | 3438 | 8 | rust decl src/lib.rs:1374 |  |  | 0.567 |
| walker |  | 3447 | 9 | rust decl src/lib.rs:1395 |  |  | 0.567 |
| walker |  | 3467 | 20 | rust decl src/lib.rs:1113 |  |  | 0.567 |
| walker |  | 3487 | 20 | rust decl src/lib.rs:1242 |  |  | 0.567 |
| walker |  | 3508 | 21 | rust decl src/lib.rs:1419 |  |  | 0.570 |
| ns | 3518 |  | 145 | Runtime max-level get and set bodies | 3.9 | 2.2 | 0.561 |
| walker |  | 3530 | 22 | rust decl src/lib.rs:1349 |  |  | 0.563 |
| walker |  | 3568 | 38 | rust decl src/lib.rs:1199 |  |  | 0.564 |
| ns | 3599 |  | 81 | no_std wiring | 3.10 |  | 0.558 |
| walker |  | 3617 | 49 | rust decl src/lib.rs:1157 |  |  | 0.558 |
| walker |  | 3675 | 58 | rust decl src/lib.rs:1249 |  |  | 0.567 |
| ns | 3679 |  | 80 | Roster: the seven public macros of src/macros.rs | 4.1 |  | 0.559 |
| walker |  | 3738 | 63 | rust decl src/lib.rs:1163 |  |  | 0.560 |
| walker |  | 3747 | 9 | rust decl src/lib.rs:1165 |  |  | 0.560 |
| walker |  | 3756 | 9 | rust decl src/lib.rs:1171 |  |  | 0.560 |
| walker |  | 3765 | 9 | rust decl src/lib.rs:1177 |  |  | 0.560 |
| ns | 3831 |  | 152 | `log!`: all four call forms | 4.2 | 4.1 | 0.550 |
| walker |  | 3851 | 86 | rust decl src/lib.rs:1294 |  |  | 0.555 |
| walker |  | 3947 | 96 | rust decl src/lib.rs:1310 |  |  | 0.561 |
| walker |  | 4043 | 96 | rust decl src/lib.rs:1327 |  |  | 0.568 |
| ns | 4099 |  | 268 | `__log!`: the expansion every log call becomes | 4.3 |  | 0.555 |
| walker |  | 4146 | 103 | rust decl src/lib.rs:1204 |  |  | 0.555 |
| walker |  | 4155 | 9 | rust decl src/lib.rs:1211 |  |  | 0.555 |
| walker |  | 4164 | 9 | rust decl src/lib.rs:1222 |  |  | 0.555 |
| ns | 4170 |  | 71 | `error!`: head and matcher arms | 4.5 | 4.1 | 0.548 |
| walker |  | 4173 | 9 | rust decl src/lib.rs:1229 |  |  | 0.548 |
| walker |  | 4182 | 9 | rust decl src/lib.rs:1236 |  |  | 0.548 |
| walker |  | 4234 | 52 | rust decl src/lib.rs:1285 |  |  | 0.548 |
| ns | 4406 |  | 236 | Roster: every doc(hidden) internal macro, with its feature fork | 4.6 |  | 0.532 |
| walker |  | 4596 | 362 | rust decl src/lib.rs:1007 |  |  | 0.533 |
| walker |  | 4605 | 9 | rust decl src/lib.rs:1020 |  |  | 0.533 |
| walker |  | 4614 | 9 | rust decl src/lib.rs:1036 |  |  | 0.533 |
| walker |  | 4623 | 9 | rust decl src/lib.rs:1043 |  |  | 0.533 |
| walker |  | 4632 | 9 | rust decl src/lib.rs:1050 |  |  | 0.533 |
| walker |  | 4641 | 9 | rust decl src/lib.rs:1057 |  |  | 0.533 |
| walker |  | 4650 | 9 | rust decl src/lib.rs:1064 |  |  | 0.533 |
| walker |  | 4659 | 9 | rust decl src/lib.rs:1071 |  |  | 0.533 |
| walker |  | 4668 | 9 | rust decl src/lib.rs:1078 |  |  | 0.533 |
| walker |  | 4677 | 9 | rust decl src/lib.rs:1085 |  |  | 0.533 |
| walker |  | 4686 | 9 | rust decl src/lib.rs:1092 |  |  | 0.533 |
| walker |  | 4695 | 9 | rust decl src/lib.rs:1107 |  |  | 0.533 |
| walker |  | 4717 | 22 | rust decl src/lib.rs:1099 |  |  | 0.533 |
| ns | 4763 |  | 357 | `__log_value!`: the capture-modifier dispatch table | 4.7 | 4.6 | 0.508 |
| walker |  | 4797 | 80 | rust names src/macros.rs |  |  | 0.523 |
| walker |  | 4842 | 45 | rust module doc src/kv/value.rs |  |  | 0.523 |
| walker |  | 4890 | 48 | rust module doc src/kv/source.rs |  |  | 0.523 |
| ns | 4964 |  | 201 | `log_enabled!`: purpose and call forms | 4.9 | 4.1 | 0.513 |
| walker |  | 5111 | 221 | rust names src/lib.rs #3 |  |  | 0.540 |
| walker |  | 5123 | 12 | rust decl src/lib.rs:1558 |  |  | 0.540 |
| walker |  | 5135 | 12 | rust decl src/lib.rs:1575 |  |  | 0.540 |
| walker |  | 5149 | 14 | rust decl src/lib.rs:1477 |  |  | 0.544 |
| ns | 5168 |  | 204 | `__private_api`: the three functions macros expand into | 5.1 |  | 0.530 |
| walker |  | 5172 | 23 | rust decl src/lib.rs:1547 |  |  | 0.530 |
| walker |  | 5200 | 28 | rust decl src/lib.rs:1564 |  |  | 0.530 |
| walker |  | 5232 | 32 | rust decl src/lib.rs:1551 |  |  | 0.530 |
| walker |  | 5264 | 32 | rust decl src/lib.rs:1568 |  |  | 0.530 |
| walker |  | 5308 | 44 | rust decl src/lib.rs:1482 |  |  | 0.532 |
| ns | 5364 |  | 196 | `log_impl`: where a `Record` is actually built | 5.2 |  | 0.519 |
| ns | 5513 |  | 149 | `GlobalLogger`: the zero-sized proxy for the global slot | 5.3 |  | 0.507 |
| walker |  | 5586 | 278 | rust decl src/lib.rs:1611 |  |  | 0.534 |
| walker |  | 5645 | 59 | rust names src/kv/error.rs |  |  | 0.534 |
| walker |  | 5667 | 22 | rust decl src/kv/error.rs:4 |  |  | 0.535 |
| walker |  | 5689 | 22 | rust decl src/kv/error.rs:62 |  |  | 0.535 |
| walker |  | 5717 | 28 | rust decl src/kv/error.rs:48 |  |  | 0.535 |
| ns | 5737 |  | 224 | `kv_support`: the capture_* functions behind every modifier | 5.4 |  | 0.526 |
| walker |  | 5795 | 78 | rust decl src/kv/error.rs:19 |  |  | 0.526 |
| walker |  | 5809 | 14 | rust decl src/kv/error.rs:28 |  |  | 0.526 |
| walker |  | 5823 | 14 | rust decl src/kv/error.rs:36 |  |  | 0.526 |
| ns | 5843 |  | 106 | `struct Record`: every field | 6.1 | 2.1 | 0.536 |
| walker |  | 5851 | 28 | rust module doc src/__private_api.rs |  |  | 0.536 |
| walker |  | 5870 | 19 | README.md section #6 |  |  | 0.536 |
| ns | 6024 |  | 181 | Roster: every accessor on `Record` | 6.2 |  | 0.549 |
| ns | 6125 |  | 101 | `Metadata` and its accessors | 6.3 | 2.1 | 0.556 |
| walker |  | 6280 | 410 | [features] in Cargo.toml |  |  | 0.623 |
| ns | 6331 |  | 206 | Roster: `RecordBuilder` and all twelve setters | 6.4 | 2.1 | 0.634 |
| walker |  | 6363 | 83 | README.md section #1 |  |  | 0.634 |
| ns | 6434 |  | 103 | `MetadataBuilder` and its setters | 6.5 | 2.1 | 0.639 |
| walker |  | 6529 | 166 | rust names src/kv/key.rs |  |  | 0.640 |
| walker |  | 6548 | 19 | rust decl src/kv/key.rs:81 |  |  | 0.640 |
| walker |  | 6568 | 20 | rust decl src/kv/key.rs:75 |  |  | 0.640 |
| walker |  | 6589 | 21 | rust decl src/kv/key.rs:7 |  |  | 0.640 |
| ns | 6591 |  | 157 | What structured logging means in `log` | 7.1 |  | 0.633 |
| walker |  | 6611 | 22 | rust decl src/kv/key.rs:21 |  |  | 0.633 |
| walker |  | 6634 | 23 | rust decl src/kv/key.rs:87 |  |  | 0.633 |
| walker |  | 6658 | 24 | rust decl src/kv/key.rs:27 |  |  | 0.633 |
| walker |  | 6686 | 28 | rust decl src/kv/key.rs:69 |  |  | 0.633 |
| walker |  | 6732 | 46 | rust decl src/kv/key.rs:12 |  |  | 0.633 |
| ns | 6774 |  | 183 | The complete list of capture modifiers | 7.2 |  | 0.627 |
| walker |  | 6786 | 54 | rust decl src/kv/key.rs:36 |  |  | 0.627 |
| walker |  | 6854 | 68 | rust decl src/kv/key.rs:42 |  |  | 0.628 |
| ns | 6979 |  | 205 | The kv module's structure and complete export list | 7.3 |  | 0.615 |
| ns | 7070 |  | 91 | `trait Source`: the three methods | 7.4 |  | 0.609 |
| ns | 7277 |  | 207 | Roster: every type that implements `Source` | 7.5 |  | 0.601 |
| walker |  | 7318 | 464 | rust names src/kv/value.rs |  |  | 0.601 |
| walker |  | 7339 | 21 | rust decl src/kv/value.rs:11 |  |  | 0.601 |
| walker |  | 7360 | 21 | rust decl src/kv/value.rs:264 |  |  | 0.601 |
| ns | 7361 |  | 84 | `trait VisitSource`: the visitor side of a `Source` | 7.6 |  | 0.598 |
| walker |  | 7382 | 22 | rust decl src/kv/value.rs:258 |  |  | 0.598 |
| walker |  | 7404 | 22 | rust decl src/kv/value.rs:270 |  |  | 0.598 |
| walker |  | 7428 | 24 | rust decl src/kv/value.rs:25 |  |  | 0.598 |
| ns | 7444 |  | 83 | `Value` and `ToValue` | 7.7 |  | 0.596 |
| walker |  | 7452 | 24 | rust decl src/kv/value.rs:118 |  |  | 0.600 |
| walker |  | 7480 | 28 | rust decl src/kv/value.rs:222 |  |  | 0.600 |
| walker |  | 7510 | 30 | rust decl src/kv/value.rs:228 |  |  | 0.600 |
| walker |  | 7553 | 43 | rust decl src/kv/value.rs:234 |  |  | 0.600 |
| walker |  | 7596 | 43 | rust decl src/kv/value.rs:276 |  |  | 0.600 |
| walker |  | 7642 | 46 | rust decl src/kv/value.rs:16 |  |  | 0.600 |
| walker |  | 7667 | 25 | rust decl src/kv/value.rs:236 |  |  | 0.600 |
| ns | 7689 |  | 245 | Roster: every constructor and conversion on `Value` | 7.8 |  | 0.590 |
| walker |  | 7718 | 51 | rust decl src/kv/value.rs:251 |  |  | 0.590 |
| walker |  | 7774 | 56 | rust decl src/kv/value.rs:244 |  |  | 0.590 |
| walker |  | 7838 | 64 | rust decl src/kv/value.rs:376 |  |  | 0.591 |
| walker |  | 7851 | 13 | rust decl src/kv/value.rs:378 |  |  | 0.592 |
| walker |  | 7940 | 89 | rust decl src/kv/value.rs:1126 |  |  | 0.592 |
| ns | 7965 |  | 276 | The primitive conversion tables | 7.9 |  | 0.583 |
| walker |  | 8029 | 89 | rust decl src/kv/value.rs:1136 |  |  | 0.583 |
| walker |  | 8121 | 92 | rust decl src/kv/value.rs:1146 |  |  | 0.583 |
| ns | 8158 |  | 193 | Roster: every method on `VisitValue` | 7.10 |  | 0.574 |
| walker |  | 8214 | 93 | rust decl src/kv/value.rs:1155 |  |  | 0.574 |
| ns | 8294 |  | 136 | The two `Value` backends: `value_bag` and the dependency-free fallback | 7.11 |  | 0.570 |
| walker |  | 8307 | 93 | rust decl src/kv/value.rs:1166 |  |  | 0.570 |
| walker |  | 8479 | 172 | rust decl src/kv/value.rs:1051 |  |  | 0.570 |
| walker |  | 8542 | 63 | rust decl src/kv/value.rs:1053 |  |  | 0.570 |
| ns | 8588 |  | 294 | `Key`, `ToKey`, and their feature-gated support modules | 7.12 |  | 0.568 |
| walker |  | 8605 | 63 | rust decl src/kv/value.rs:1063 |  |  | 0.568 |
| walker |  | 8671 | 66 | rust decl src/kv/value.rs:1093 |  |  | 0.568 |
| walker |  | 8738 | 67 | rust decl src/kv/value.rs:1073 |  |  | 0.568 |
| ns | 8760 |  | 172 | `kv::Error`: every variant of the private inner enum | 7.13 |  | 0.562 |
| walker |  | 8805 | 67 | rust decl src/kv/value.rs:1083 |  |  | 0.562 |
| ns | 8837 |  | 77 | `Source::get` and `Source::count` default implementations | 7.17 | 7.4 | 0.560 |
| walker |  | 8873 | 68 | rust decl src/kv/value.rs:1103 |  |  | 0.560 |
| walker |  | 8941 | 68 | rust decl src/kv/value.rs:1112 |  |  | 0.560 |
| ns | 9121 |  | 284 | Roster: every test in tests/macros.rs | 8.1 |  | 0.549 |
| walker |  | 9284 | 343 | rust decl src/macros.rs:390 |  |  | 0.556 |
| ns | 9297 |  | 176 | tests/integration.rs: the capturing test logger and what it pins | 8.2 |  | 0.550 |
| ns | 9463 |  | 166 | src/serde.rs: what is serialised, and how | 8.3 |  | 0.547 |
| walker |  | 9491 | 207 | rust names src/kv/mod.rs |  |  | 0.568 |
| ns | 9607 |  | 144 | CI: the seven jobs | 8.4 |  | 0.561 |
| ns | 9663 |  | 56 | The companion crate that tests compile-time filtering | 8.5 |  | 0.559 |
| ns | 9735 |  | 72 | Roster: the value benchmarks | 8.6 |  | 0.556 |
| walker |  | 9774 | 283 | rust decl src/macros.rs:163 |  |  | 0.558 |
| ns | 9844 |  | 109 | The mutually-exclusive feature guards | 8.7 |  | 0.554 |
| ns | 9889 |  | 45 | CHANGELOG: format and latest release | 8.8 |  | 0.552 |
| ns | 9979 |  | 90 | Roster: the top-level sections of the structured-logging RFC | 8.9 |  | 0.549 |
