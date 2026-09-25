Score(3000)=0.573 I=0.789 C=0.417 ns_rows≤3K=23/67 grid(1000/1442/2080/3000/4327/6240/9000)=0.658/0.563/0.615/0.573/0.545/0.554/0.568

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
| walker |  | 417 | 230 | rust names src/lib.rs |  |  | 0.885 |
| walker |  | 426 | 9 | rust decl src/lib.rs:462 |  |  | 0.885 |
| ns | 431 |  | 127 | Cargo.toml package block: version, licence, MSRV, edition | 1.7 |  | 0.786 |
| walker |  | 441 | 15 | rust decl src/lib.rs:442 |  |  | 0.786 |
| walker |  | 472 | 31 | rust decl src/lib.rs:420 |  |  | 0.786 |
| walker |  | 528 | 56 | headings outline in README.md |  |  | 0.786 |
| ns | 565 |  | 134 | Cargo features, part 1: the twelve compile-time level filters | 1.8 |  | 0.672 |
| walker |  | 719 | 191 | [package] in Cargo.toml |  |  | 0.773 |
| ns | 799 |  | 234 | Cargo features, part 2: std, the kv family, the serde alias, deprecated aliases | 1.9 | 1.8 | 0.683 |
| ns | 916 |  | 117 | The four optional runtime dependencies | 1.10 |  | 0.655 |
| walker |  | 983 | 264 | rust names src/lib.rs #1 |  |  | 0.658 |
| walker |  | 1008 | 25 | rust decl src/lib.rs:651 |  |  | 0.658 |
| walker |  | 1014 | 6 | rust decl src/lib.rs:652 |  |  | 0.658 |
| walker |  | 1040 | 26 | rust decl src/lib.rs:501 |  |  | 0.658 |
| walker |  | 1046 | 6 | rust decl src/lib.rs:502 |  |  | 0.658 |
| walker |  | 1073 | 27 | rust decl src/lib.rs:1002 |  |  | 0.658 |
| ns | 1084 |  | 168 | Roster: every public type, trait, module and const in src/lib.rs | 2.1 |  | 0.620 |
| walker |  | 1101 | 28 | rust decl src/lib.rs:528 |  |  | 0.620 |
| walker |  | 1129 | 28 | rust decl src/lib.rs:678 |  |  | 0.620 |
| walker |  | 1160 | 31 | rust decl src/lib.rs:658 |  |  | 0.620 |
| walker |  | 1166 | 6 | rust decl src/lib.rs:659 |  |  | 0.620 |
| walker |  | 1198 | 32 | rust decl src/lib.rs:508 |  |  | 0.620 |
| walker |  | 1204 | 6 | rust decl src/lib.rs:509 |  |  | 0.620 |
| walker |  | 1244 | 40 | rust decl src/lib.rs:515 |  |  | 0.621 |
| walker |  | 1285 | 41 | rust decl src/lib.rs:665 |  |  | 0.623 |
| ns | 1288 |  | 204 | Roster: the seven free functions of the global logger API, with their cfg gates | 2.2 |  | 0.588 |
| walker |  | 1304 | 19 | rust decl src/lib.rs:856 |  |  | 0.588 |
| walker |  | 1324 | 20 | rust decl src/lib.rs:464 |  |  | 0.588 |
| ns | 1374 |  | 86 | `enum Level`: all five variants and their discriminants | 2.3 | 2.1 | 0.562 |
| walker |  | 1430 | 106 | rust decl src/lib.rs:841 |  |  | 0.563 |
| ns | 1466 |  | 92 | `enum LevelFilter`: all six variants, including `Off` | 2.4 | 2.1 | 0.537 |
| walker |  | 1544 | 114 | rust decl src/lib.rs:534 |  |  | 0.539 |
| walker |  | 1552 | 8 | rust decl src/lib.rs:547 |  |  | 0.539 |
| walker |  | 1560 | 8 | rust decl src/lib.rs:553 |  |  | 0.539 |
| ns | 1650 |  | 184 | Roster: every method on `Level` and `LevelFilter` | 2.5 |  | 0.525 |
| walker |  | 1676 | 116 | rust decl src/lib.rs:684 |  |  | 0.573 |
| walker |  | 1684 | 8 | rust decl src/lib.rs:698 |  |  | 0.573 |
| walker |  | 1692 | 8 | rust decl src/lib.rs:706 |  |  | 0.573 |
| walker |  | 1729 | 37 | rust decl src/lib.rs:860 |  |  | 0.573 |
| ns | 1782 |  | 132 | Module declarations and the atomics-vs-Cell import fork | 2.6 | 2.1 | 0.543 |
| walker |  | 1901 | 172 | rust decl src/lib.rs:634 |  |  | 0.593 |
| ns | 1934 |  | 152 | Global state: LOGGER, STATE, the state constants, LOG_LEVEL_NAMES | 2.7 |  | 0.615 |
| walker |  | 1952 | 51 | rust decl src/lib.rs:780 |  |  | 0.615 |
| ns | 2117 |  | 183 | Roster: the impls that make Level and LevelFilter comparable and parseable | 2.8 |  | 0.634 |
| walker |  | 2202 | 250 | rust decl src/lib.rs:869 |  |  | 0.636 |
| walker |  | 2210 | 8 | rust decl src/lib.rs:871 |  |  | 0.636 |
| walker |  | 2218 | 8 | rust decl src/lib.rs:877 |  |  | 0.636 |
| walker |  | 2226 | 8 | rust decl src/lib.rs:883 |  |  | 0.636 |
| walker |  | 2234 | 8 | rust decl src/lib.rs:889 |  |  | 0.636 |
| walker |  | 2242 | 8 | rust decl src/lib.rs:895 |  |  | 0.636 |
| walker |  | 2250 | 8 | rust decl src/lib.rs:901 |  |  | 0.636 |
| walker |  | 2258 | 8 | rust decl src/lib.rs:907 |  |  | 0.636 |
| walker |  | 2266 | 8 | rust decl src/lib.rs:916 |  |  | 0.636 |
| walker |  | 2274 | 8 | rust decl src/lib.rs:922 |  |  | 0.636 |
| walker |  | 2282 | 8 | rust decl src/lib.rs:931 |  |  | 0.636 |
| walker |  | 2302 | 20 | rust decl src/lib.rs:937 |  |  | 0.636 |
| walker |  | 2322 | 20 | rust decl src/lib.rs:944 |  |  | 0.637 |
| ns | 2395 |  | 278 | `STATIC_MAX_LEVEL`: compile-time level resolution | 2.9 | 2.1 | 0.609 |
| ns | 2470 |  | 75 | `trait Log`: the three required methods | 3.1 | 2.1 | 0.594 |
| walker |  | 2623 | 301 | rust decl src/lib.rs:473 |  |  | 0.627 |
| ns | 2657 |  | 187 | Worked example: a complete `Log` implementation | 3.2 |  | 0.597 |
| walker |  | 2687 | 64 | README.md section #0 |  |  | 0.597 |
| walker |  | 2723 | 36 | manifest config in Cargo.toml |  |  | 0.597 |
| walker |  | 2783 | 60 | rust names src/serde.rs |  |  | 0.597 |
| walker |  | 2810 | 27 | rust decl src/serde.rs:31 |  |  | 0.597 |
| ns | 2813 |  | 156 | Why `set_max_level` must be called, and that it defaults to `Off` | 3.4 |  | 0.586 |
| walker |  | 2839 | 29 | rust decl src/serde.rs:126 |  |  | 0.586 |
| walker |  | 2869 | 30 | rust decl src/serde.rs:16 |  |  | 0.586 |
| walker |  | 2899 | 30 | rust decl src/serde.rs:110 |  |  | 0.586 |
| ns | 2919 |  | 106 | Worked example: the conventional `init()` for a logger crate | 3.5 |  | 0.573 |
| walker |  | 2921 | 22 | rust decl src/serde.rs:17 |  |  | 0.573 |
| walker |  | 2943 | 22 | rust decl src/serde.rs:111 |  |  | 0.573 |
| walker |  | 2968 | 25 | rust decl src/serde.rs:32 |  |  | 0.573 |
| walker |  | 2993 | 25 | rust decl src/serde.rs:127 |  |  | 0.573 |
| ns | 3072 |  | 153 | The `Log` impls the crate itself provides | 3.6 |  | 0.556 |
| walker |  | 3112 | 119 | [dependencies] in Cargo.toml |  |  | 0.573 |
| ns | 3186 |  | 114 | `logger()`: the acquire-load fast path | 3.7 | 2.2 | 0.562 |
| ns | 3373 |  | 187 | `set_logger_inner`: the compare-exchange install path | 3.8 |  | 0.540 |
| walker |  | 3403 | 291 | rust names src/lib.rs #2 |  |  | 0.564 |
| walker |  | 3411 | 8 | rust decl src/lib.rs:1374 |  |  | 0.564 |
| walker |  | 3420 | 9 | rust decl src/lib.rs:1395 |  |  | 0.564 |
| walker |  | 3440 | 20 | rust decl src/lib.rs:1113 |  |  | 0.564 |
| walker |  | 3460 | 20 | rust decl src/lib.rs:1242 |  |  | 0.564 |
| walker |  | 3481 | 21 | rust decl src/lib.rs:1419 |  |  | 0.566 |
| walker |  | 3503 | 22 | rust decl src/lib.rs:1349 |  |  | 0.569 |
| ns | 3518 |  | 145 | Runtime max-level get and set bodies | 3.9 | 2.2 | 0.560 |
| walker |  | 3541 | 38 | rust decl src/lib.rs:1199 |  |  | 0.560 |
| walker |  | 3590 | 49 | rust decl src/lib.rs:1157 |  |  | 0.561 |
| ns | 3599 |  | 81 | no_std wiring | 3.10 |  | 0.555 |
| walker |  | 3648 | 58 | rust decl src/lib.rs:1249 |  |  | 0.564 |
| ns | 3679 |  | 80 | Roster: the seven public macros of src/macros.rs | 4.1 |  | 0.556 |
| walker |  | 3711 | 63 | rust decl src/lib.rs:1163 |  |  | 0.557 |
| walker |  | 3720 | 9 | rust decl src/lib.rs:1165 |  |  | 0.557 |
| walker |  | 3729 | 9 | rust decl src/lib.rs:1171 |  |  | 0.557 |
| walker |  | 3738 | 9 | rust decl src/lib.rs:1177 |  |  | 0.557 |
| walker |  | 3824 | 86 | rust decl src/lib.rs:1294 |  |  | 0.561 |
| ns | 3831 |  | 152 | `log!`: all four call forms | 4.2 | 4.1 | 0.552 |
| walker |  | 3920 | 96 | rust decl src/lib.rs:1310 |  |  | 0.558 |
| walker |  | 4016 | 96 | rust decl src/lib.rs:1327 |  |  | 0.565 |
| ns | 4099 |  | 268 | `__log!`: the expansion every log call becomes | 4.3 |  | 0.552 |
| walker |  | 4119 | 103 | rust decl src/lib.rs:1204 |  |  | 0.552 |
| walker |  | 4128 | 9 | rust decl src/lib.rs:1211 |  |  | 0.552 |
| walker |  | 4137 | 9 | rust decl src/lib.rs:1222 |  |  | 0.552 |
| walker |  | 4146 | 9 | rust decl src/lib.rs:1229 |  |  | 0.552 |
| walker |  | 4155 | 9 | rust decl src/lib.rs:1236 |  |  | 0.552 |
| ns | 4170 |  | 71 | `error!`: head and matcher arms | 4.5 | 4.1 | 0.545 |
| walker |  | 4207 | 52 | rust decl src/lib.rs:1285 |  |  | 0.545 |
| ns | 4406 |  | 236 | Roster: every doc(hidden) internal macro, with its feature fork | 4.6 |  | 0.529 |
| walker |  | 4569 | 362 | rust decl src/lib.rs:1007 |  |  | 0.530 |
| walker |  | 4578 | 9 | rust decl src/lib.rs:1020 |  |  | 0.530 |
| walker |  | 4587 | 9 | rust decl src/lib.rs:1036 |  |  | 0.530 |
| walker |  | 4596 | 9 | rust decl src/lib.rs:1043 |  |  | 0.530 |
| walker |  | 4605 | 9 | rust decl src/lib.rs:1050 |  |  | 0.530 |
| walker |  | 4614 | 9 | rust decl src/lib.rs:1057 |  |  | 0.530 |
| walker |  | 4623 | 9 | rust decl src/lib.rs:1064 |  |  | 0.530 |
| walker |  | 4632 | 9 | rust decl src/lib.rs:1071 |  |  | 0.530 |
| walker |  | 4641 | 9 | rust decl src/lib.rs:1078 |  |  | 0.530 |
| walker |  | 4650 | 9 | rust decl src/lib.rs:1085 |  |  | 0.530 |
| walker |  | 4659 | 9 | rust decl src/lib.rs:1092 |  |  | 0.530 |
| walker |  | 4668 | 9 | rust decl src/lib.rs:1107 |  |  | 0.530 |
| walker |  | 4690 | 22 | rust decl src/lib.rs:1099 |  |  | 0.530 |
| ns | 4763 |  | 357 | `__log_value!`: the capture-modifier dispatch table | 4.7 | 4.6 | 0.505 |
| walker |  | 4770 | 80 | rust names src/macros.rs |  |  | 0.520 |
| walker |  | 4815 | 45 | rust module doc src/kv/value.rs |  |  | 0.520 |
| walker |  | 4863 | 48 | rust module doc src/kv/source.rs |  |  | 0.520 |
| ns | 4964 |  | 201 | `log_enabled!`: purpose and call forms | 4.9 | 4.1 | 0.510 |
| walker |  | 5084 | 221 | rust names src/lib.rs #3 |  |  | 0.538 |
| walker |  | 5096 | 12 | rust decl src/lib.rs:1558 |  |  | 0.538 |
| walker |  | 5108 | 12 | rust decl src/lib.rs:1575 |  |  | 0.538 |
| walker |  | 5122 | 14 | rust decl src/lib.rs:1477 |  |  | 0.541 |
| walker |  | 5145 | 23 | rust decl src/lib.rs:1547 |  |  | 0.541 |
| ns | 5168 |  | 204 | `__private_api`: the three functions macros expand into | 5.1 |  | 0.527 |
| walker |  | 5173 | 28 | rust decl src/lib.rs:1564 |  |  | 0.527 |
| walker |  | 5205 | 32 | rust decl src/lib.rs:1551 |  |  | 0.527 |
| walker |  | 5237 | 32 | rust decl src/lib.rs:1568 |  |  | 0.527 |
| walker |  | 5281 | 44 | rust decl src/lib.rs:1482 |  |  | 0.529 |
| ns | 5364 |  | 196 | `log_impl`: where a `Record` is actually built | 5.2 |  | 0.516 |
| ns | 5513 |  | 149 | `GlobalLogger`: the zero-sized proxy for the global slot | 5.3 |  | 0.505 |
| walker |  | 5559 | 278 | rust decl src/lib.rs:1611 |  |  | 0.532 |
| walker |  | 5618 | 59 | rust names src/kv/error.rs |  |  | 0.532 |
| walker |  | 5640 | 22 | rust decl src/kv/error.rs:4 |  |  | 0.532 |
| walker |  | 5662 | 22 | rust decl src/kv/error.rs:62 |  |  | 0.532 |
| walker |  | 5690 | 28 | rust decl src/kv/error.rs:48 |  |  | 0.532 |
| ns | 5737 |  | 224 | `kv_support`: the capture_* functions behind every modifier | 5.4 |  | 0.524 |
| walker |  | 5768 | 78 | rust decl src/kv/error.rs:19 |  |  | 0.524 |
| walker |  | 5782 | 14 | rust decl src/kv/error.rs:28 |  |  | 0.524 |
| walker |  | 5796 | 14 | rust decl src/kv/error.rs:36 |  |  | 0.524 |
| walker |  | 5824 | 28 | rust module doc src/__private_api.rs |  |  | 0.524 |
| walker |  | 5843 | 19 | README.md section #6 |  |  | 0.534 |
| ns | 5843 |  | 106 | `struct Record`: every field | 6.1 | 2.1 | 0.534 |
| ns | 6024 |  | 181 | Roster: every accessor on `Record` | 6.2 |  | 0.547 |
| ns | 6125 |  | 101 | `Metadata` and its accessors | 6.3 | 2.1 | 0.554 |
| walker |  | 6253 | 410 | [features] in Cargo.toml |  |  | 0.621 |
| ns | 6331 |  | 206 | Roster: `RecordBuilder` and all twelve setters | 6.4 | 2.1 | 0.632 |
| walker |  | 6336 | 83 | README.md section #1 |  |  | 0.632 |
| ns | 6434 |  | 103 | `MetadataBuilder` and its setters | 6.5 | 2.1 | 0.637 |
| walker |  | 6475 | 139 | rust names src/kv/mod.rs |  |  | 0.638 |
| ns | 6591 |  | 157 | What structured logging means in `log` | 7.1 |  | 0.632 |
| walker |  | 6641 | 166 | rust names src/kv/key.rs |  |  | 0.632 |
| walker |  | 6660 | 19 | rust decl src/kv/key.rs:81 |  |  | 0.632 |
| walker |  | 6680 | 20 | rust decl src/kv/key.rs:75 |  |  | 0.632 |
| walker |  | 6701 | 21 | rust decl src/kv/key.rs:7 |  |  | 0.632 |
| walker |  | 6723 | 22 | rust decl src/kv/key.rs:21 |  |  | 0.632 |
| walker |  | 6746 | 23 | rust decl src/kv/key.rs:87 |  |  | 0.632 |
| walker |  | 6770 | 24 | rust decl src/kv/key.rs:27 |  |  | 0.632 |
| ns | 6774 |  | 183 | The complete list of capture modifiers | 7.2 |  | 0.626 |
| walker |  | 6798 | 28 | rust decl src/kv/key.rs:69 |  |  | 0.626 |
| walker |  | 6844 | 46 | rust decl src/kv/key.rs:12 |  |  | 0.626 |
| walker |  | 6898 | 54 | rust decl src/kv/key.rs:36 |  |  | 0.626 |
| walker |  | 6966 | 68 | rust decl src/kv/key.rs:42 |  |  | 0.627 |
| ns | 6979 |  | 205 | The kv module's structure and complete export list | 7.3 |  | 0.624 |
| ns | 7070 |  | 91 | `trait Source`: the three methods | 7.4 |  | 0.618 |
| ns | 7277 |  | 207 | Roster: every type that implements `Source` | 7.5 |  | 0.610 |
| ns | 7361 |  | 84 | `trait VisitSource`: the visitor side of a `Source` | 7.6 |  | 0.607 |
| walker |  | 7430 | 464 | rust names src/kv/value.rs |  |  | 0.607 |
| ns | 7444 |  | 83 | `Value` and `ToValue` | 7.7 |  | 0.603 |
| walker |  | 7451 | 21 | rust decl src/kv/value.rs:11 |  |  | 0.605 |
| walker |  | 7472 | 21 | rust decl src/kv/value.rs:264 |  |  | 0.605 |
| walker |  | 7494 | 22 | rust decl src/kv/value.rs:258 |  |  | 0.605 |
| walker |  | 7516 | 22 | rust decl src/kv/value.rs:270 |  |  | 0.605 |
| walker |  | 7540 | 24 | rust decl src/kv/value.rs:25 |  |  | 0.605 |
| walker |  | 7564 | 24 | rust decl src/kv/value.rs:118 |  |  | 0.610 |
| walker |  | 7592 | 28 | rust decl src/kv/value.rs:222 |  |  | 0.610 |
| walker |  | 7622 | 30 | rust decl src/kv/value.rs:228 |  |  | 0.610 |
| walker |  | 7665 | 43 | rust decl src/kv/value.rs:234 |  |  | 0.610 |
| ns | 7689 |  | 245 | Roster: every constructor and conversion on `Value` | 7.8 |  | 0.599 |
| walker |  | 7708 | 43 | rust decl src/kv/value.rs:276 |  |  | 0.599 |
| walker |  | 7754 | 46 | rust decl src/kv/value.rs:16 |  |  | 0.599 |
| walker |  | 7779 | 25 | rust decl src/kv/value.rs:236 |  |  | 0.599 |
| walker |  | 7830 | 51 | rust decl src/kv/value.rs:251 |  |  | 0.599 |
| walker |  | 7886 | 56 | rust decl src/kv/value.rs:244 |  |  | 0.599 |
| walker |  | 7950 | 64 | rust decl src/kv/value.rs:376 |  |  | 0.600 |
| walker |  | 7963 | 13 | rust decl src/kv/value.rs:378 |  |  | 0.601 |
| ns | 7965 |  | 276 | The primitive conversion tables | 7.9 |  | 0.591 |
| walker |  | 8052 | 89 | rust decl src/kv/value.rs:1126 |  |  | 0.591 |
| walker |  | 8141 | 89 | rust decl src/kv/value.rs:1136 |  |  | 0.591 |
| ns | 8158 |  | 193 | Roster: every method on `VisitValue` | 7.10 |  | 0.583 |
| walker |  | 8233 | 92 | rust decl src/kv/value.rs:1146 |  |  | 0.583 |
| ns | 8294 |  | 136 | The two `Value` backends: `value_bag` and the dependency-free fallback | 7.11 |  | 0.579 |
| walker |  | 8326 | 93 | rust decl src/kv/value.rs:1155 |  |  | 0.579 |
| walker |  | 8419 | 93 | rust decl src/kv/value.rs:1166 |  |  | 0.579 |
| ns | 8588 |  | 294 | `Key`, `ToKey`, and their feature-gated support modules | 7.12 |  | 0.577 |
| walker |  | 8591 | 172 | rust decl src/kv/value.rs:1051 |  |  | 0.577 |
| walker |  | 8654 | 63 | rust decl src/kv/value.rs:1053 |  |  | 0.577 |
| walker |  | 8717 | 63 | rust decl src/kv/value.rs:1063 |  |  | 0.577 |
| ns | 8760 |  | 172 | `kv::Error`: every variant of the private inner enum | 7.13 |  | 0.570 |
| walker |  | 8783 | 66 | rust decl src/kv/value.rs:1093 |  |  | 0.570 |
| ns | 8837 |  | 77 | `Source::get` and `Source::count` default implementations | 7.17 | 7.4 | 0.568 |
| walker |  | 8850 | 67 | rust decl src/kv/value.rs:1073 |  |  | 0.568 |
| walker |  | 8917 | 67 | rust decl src/kv/value.rs:1083 |  |  | 0.568 |
| walker |  | 8985 | 68 | rust decl src/kv/value.rs:1103 |  |  | 0.568 |
| walker |  | 9053 | 68 | rust decl src/kv/value.rs:1112 |  |  | 0.568 |
| ns | 9121 |  | 284 | Roster: every test in tests/macros.rs | 8.1 |  | 0.557 |
| ns | 9297 |  | 176 | tests/integration.rs: the capturing test logger and what it pins | 8.2 |  | 0.551 |
| walker |  | 9396 | 343 | rust decl src/macros.rs:390 |  |  | 0.558 |
| ns | 9463 |  | 166 | src/serde.rs: what is serialised, and how | 8.3 |  | 0.554 |
| ns | 9607 |  | 144 | CI: the seven jobs | 8.4 |  | 0.548 |
| ns | 9663 |  | 56 | The companion crate that tests compile-time filtering | 8.5 |  | 0.546 |
| walker |  | 9679 | 283 | rust decl src/macros.rs:163 |  |  | 0.547 |
| ns | 9735 |  | 72 | Roster: the value benchmarks | 8.6 |  | 0.545 |
| ns | 9844 |  | 109 | The mutually-exclusive feature guards | 8.7 |  | 0.541 |
| ns | 9889 |  | 45 | CHANGELOG: format and latest release | 8.8 |  | 0.539 |
| walker |  | 9962 | 283 | rust decl src/macros.rs:202 |  |  | 0.539 |
| ns | 9979 |  | 90 | Roster: the top-level sections of the structured-logging RFC | 8.9 |  | 0.536 |
