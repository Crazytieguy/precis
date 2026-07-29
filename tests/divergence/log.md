Score(3000)=0.674 I=0.849 C=0.535 ns_rows≤3K=23/67 (reached=14 partial=0 missing=9)

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| ns | 34 |  | 34 | Crate identity: README title and one-line description | 1.1 |  | 0.000 |
| walker |  | 52 | 52 | listing of '.' |  |  | 0.000 |
| walker |  | 73 | 21 | listing of 'src' |  |  | 0.000 |
| ns | 86 |  | 52 | Complete root listing | 1.2 |  | 0.574 |
| walker |  | 92 | 19 | listing of 'src/kv' |  |  | 0.634 |
| walker |  | 95 | 3 | listing of '.github' |  |  | 0.634 |
| walker |  | 98 | 3 | listing of '.github/workflows' |  |  | 0.635 |
| walker |  | 101 | 3 | listing of 'benches' |  |  | 0.637 |
| walker |  | 108 | 7 | listing of 'tests' |  |  | 0.647 |
| ns | 127 |  | 41 | Complete src/ and src/kv/ listings | 1.3 |  | 0.642 |
| ns | 158 |  | 31 | Listings for the remaining directories | 1.4 |  | 0.583 |
| walker |  | 160 | 52 | README headline in README.md |  |  | 0.911 |
| walker |  | 168 | 8 | listing of 'test_max_level_features' |  |  | 0.965 |
| walker |  | 177 | 9 | listing of 'rfcs' |  |  | 1.000 |
| ns | 225 |  | 67 | Facade semantics: the noop fallback | 1.5 |  | 0.934 |
| walker |  | 268 | 91 | crate-doc lede in src/kv/mod.rs |  |  | 0.934 |
| ns | 306 |  | 81 | Facade semantics: what a log request is | 1.6 |  | 0.878 |
| walker |  | 324 | 56 | headings outline in README.md |  |  | 0.878 |
| ns | 433 |  | 127 | Cargo.toml package block: version, licence, MSRV, edition | 1.7 |  | 0.780 |
| walker |  | 515 | 191 | [package] in Cargo.toml |  |  | 0.897 |
| ns | 567 |  | 134 | Cargo features, part 1: the twelve compile-time level filters | 1.8 |  | 0.767 |
| walker |  | 579 | 64 | README.md section #0 |  |  | 0.767 |
| walker |  | 615 | 36 | manifest config in Cargo.toml |  |  | 0.767 |
| ns | 801 |  | 234 | Cargo features, part 2: std, the kv family, the serde alias, deprecated aliases | 1.9 | 1.8 | 0.678 |
| walker |  | 863 | 248 | crate-doc lede in src/lib.rs |  |  | 0.756 |
| ns | 918 |  | 117 | The four optional runtime dependencies | 1.10 |  | 0.724 |
| ns | 1086 |  | 168 | Roster: every public type, trait, module and const in src/lib.rs | 2.1 |  | 0.671 |
| walker |  | 1094 | 231 | mod/use plumbing in src/lib.rs |  |  | 0.678 |
| ns | 1290 |  | 204 | Roster: the seven free functions of the global logger API, with their cfg gates | 2.2 |  | 0.640 |
| ns | 1376 |  | 86 | `enum Level`: all five variants and their discriminants | 2.3 | 2.1 | 0.611 |
| walker |  | 1394 | 300 | pub-item names surface in src/lib.rs |  |  | 0.721 |
| walker |  | 1394 | 0 | pub item at src/lib.rs:1375 |  |  | 0.721 |
| walker |  | 1394 | 0 | pub item at src/lib.rs:1396 |  |  | 0.721 |
| walker |  | 1394 | 0 | pub item at src/lib.rs:1529 |  |  | 0.721 |
| walker |  | 1394 | 0 | pub item at src/lib.rs:1581 |  |  | 0.721 |
| walker |  | 1408 | 14 | pub item at src/lib.rs:1351 |  |  | 0.728 |
| walker |  | 1422 | 14 | pub item at src/lib.rs:1478 |  |  | 0.736 |
| walker |  | 1445 | 23 | pub item at src/lib.rs:1549 |  |  | 0.736 |
| walker |  | 1466 | 21 | pub item at src/lib.rs:1420 |  |  | 0.748 |
| ns | 1468 |  | 92 | `enum LevelFilter`: all six variants, including `Off` | 2.4 | 2.1 | 0.713 |
| walker |  | 1493 | 27 | pub item at src/lib.rs:1003 |  |  | 0.713 |
| walker |  | 1521 | 28 | pub item at src/lib.rs:1566 |  |  | 0.713 |
| walker |  | 1561 | 40 | pub item at src/lib.rs:1200 |  |  | 0.714 |
| walker |  | 1612 | 51 | pub item at src/lib.rs:1158 |  |  | 0.714 |
| walker |  | 1624 | 12 | pub item body at src/lib.rs:1478 body 1479 |  |  | 0.714 |
| ns | 1652 |  | 184 | Roster: every method on `Level` and `LevelFilter` | 2.5 |  | 0.666 |
| walker |  | 1730 | 106 | pub item at src/lib.rs:842 |  |  | 0.668 |
| walker |  | 1746 | 16 | pub item body at src/lib.rs:1420 body 1421 |  |  | 0.668 |
| ns | 1784 |  | 132 | Module declarations and the atomics-vs-Cell import fork | 2.6 | 2.1 | 0.684 |
| walker |  | 1920 | 174 | pub item at src/lib.rs:636 |  |  | 0.728 |
| ns | 1936 |  | 152 | Global state: LOGGER, STATE, the state constants, LOG_LEVEL_NAMES | 2.7 |  | 0.700 |
| walker |  | 1940 | 20 | pub item body at src/lib.rs:1351 body 1352 |  |  | 0.700 |
| ns | 2119 |  | 183 | Roster: the impls that make Level and LevelFilter comparable and parseable | 2.8 |  | 0.672 |
| walker |  | 2243 | 303 | pub item at src/lib.rs:475 |  |  | 0.708 |
| ns | 2397 |  | 278 | `STATIC_MAX_LEVEL`: compile-time level resolution | 2.9 | 2.1 | 0.678 |
| ns | 2472 |  | 75 | `trait Log`: the three required methods | 3.1 | 2.1 | 0.662 |
| walker |  | 2658 | 415 | pub item at src/lib.rs:1249 |  |  | 0.680 |
| ns | 2659 |  | 187 | Worked example: a complete `Log` implementation | 3.2 |  | 0.647 |
| walker |  | 2675 | 17 | pub-item doc lede at src/lib.rs:1249 |  |  | 0.658 |
| ns | 2815 |  | 156 | Why `set_max_level` must be called, and that it defaults to `Off` | 3.4 |  | 0.646 |
| ns | 2921 |  | 106 | Worked example: the conventional `init()` for a logger crate | 3.5 |  | 0.632 |
| walker |  | 2951 | 276 | pub item at src/lib.rs:1611 |  |  | 0.674 |
| walker |  | 2971 | 20 | pub-item doc lede at src/lib.rs:1158 |  |  | 0.674 |
| walker |  | 2992 | 21 | pub-item doc lede at src/lib.rs:842 |  |  | 0.674 |
| walker |  | 3018 | 26 | pub-item names surface in src/kv/key.rs |  |  | 0.674 |
| walker |  | 3049 | 31 | pub item at src/kv/key.rs:7 |  |  | 0.675 |
| ns | 3074 |  | 153 | The `Log` impls the crate itself provides | 3.6 |  | 0.654 |
| walker |  | 3168 | 119 | [dependencies] in Cargo.toml |  |  | 0.670 |
| ns | 3188 |  | 114 | `logger()`: the acquire-load fast path | 3.7 | 2.2 | 0.657 |
| walker |  | 3260 | 92 | pub item body at src/lib.rs:1375 body 1376 |  |  | 0.657 |
| walker |  | 3287 | 27 | pub-item names surface in src/kv/source.rs |  |  | 0.657 |
| walker |  | 3321 | 34 | pub item at src/kv/error.rs:5 |  |  | 0.657 |
| walker |  | 3369 | 48 | pub item at src/kv/source.rs:235 |  |  | 0.657 |
| ns | 3375 |  | 187 | `set_logger_inner`: the compare-exchange install path | 3.8 |  | 0.632 |
| walker |  | 3409 | 40 | pub-item doc lede at src/lib.rs:1581 |  |  | 0.632 |
| walker |  | 3455 | 46 | pub-item doc lede at src/lib.rs:1549 |  |  | 0.632 |
| walker |  | 3495 | 40 | pub-item names surface in src/kv/value.rs |  |  | 0.632 |
| ns | 3520 |  | 145 | Runtime max-level get and set bodies | 3.9 | 2.2 | 0.622 |
| walker |  | 3526 | 31 | pub item at src/kv/value.rs:11 |  |  | 0.623 |
| walker |  | 3552 | 26 | pub item at src/kv/value.rs:119 |  |  | 0.623 |
| ns | 3601 |  | 81 | no_std wiring | 3.10 |  | 0.617 |
| ns | 3681 |  | 80 | Roster: the seven public macros of src/macros.rs | 4.1 |  | 0.608 |
| ns | 3833 |  | 152 | `log!`: all four call forms | 4.2 | 4.1 | 0.598 |
| ns | 4101 |  | 268 | `__log!`: the expansion every log call becomes | 4.3 |  | 0.584 |
| ns | 4172 |  | 71 | `error!`: head and matcher arms | 4.5 | 4.1 | 0.577 |
| ns | 4408 |  | 236 | Roster: every doc(hidden) internal macro, with its feature fork | 4.6 |  | 0.559 |
| ns | 4765 |  | 357 | `__log_value!`: the capture-modifier dispatch table | 4.7 | 4.6 | 0.532 |
| ns | 4966 |  | 201 | `log_enabled!`: purpose and call forms | 4.9 | 4.1 | 0.522 |
| ns | 5170 |  | 204 | `__private_api`: the three functions macros expand into | 5.1 |  | 0.508 |
| ns | 5366 |  | 196 | `log_impl`: where a `Record` is actually built | 5.2 |  | 0.496 |
| ns | 5515 |  | 149 | `GlobalLogger`: the zero-sized proxy for the global slot | 5.3 |  | 0.485 |
| walker |  | 5568 | 2016 | impl method sigs in src/lib.rs |  |  | 0.549 |
| walker |  | 5568 | 0 | impl method at src/lib.rs:427 |  |  | 0.549 |
| walker |  | 5568 | 0 | impl method at src/lib.rs:431 |  |  | 0.549 |
| walker |  | 5568 | 0 | impl method at src/lib.rs:435 |  |  | 0.549 |
| walker |  | 5568 | 0 | impl method at src/lib.rs:503 |  |  | 0.549 |
| walker |  | 5568 | 0 | impl method at src/lib.rs:510 |  |  | 0.549 |
| walker |  | 5568 | 0 | impl method at src/lib.rs:517 |  |  | 0.549 |
| walker |  | 5568 | 0 | impl method at src/lib.rs:529 |  |  | 0.549 |
| walker |  | 5568 | 0 | impl method at src/lib.rs:535 |  |  | 0.549 |
| walker |  | 5568 | 0 | impl method at src/lib.rs:548 |  |  | 0.549 |
| walker |  | 5568 | 0 | impl method at src/lib.rs:554 |  |  | 0.549 |
| walker |  | 5568 | 0 | impl method at src/lib.rs:561 |  |  | 0.549 |
| walker |  | 5568 | 0 | impl method at src/lib.rs:579 |  |  | 0.549 |
| walker |  | 5568 | 0 | impl method at src/lib.rs:599 |  |  | 0.549 |
| walker |  | 5568 | 0 | impl method at src/lib.rs:620 |  |  | 0.549 |
| walker |  | 5568 | 0 | impl method at src/lib.rs:653 |  |  | 0.549 |
| walker |  | 5568 | 0 | impl method at src/lib.rs:660 |  |  | 0.549 |
| walker |  | 5568 | 0 | impl method at src/lib.rs:667 |  |  | 0.549 |
| walker |  | 5568 | 0 | impl method at src/lib.rs:679 |  |  | 0.549 |
| walker |  | 5568 | 0 | impl method at src/lib.rs:685 |  |  | 0.549 |
| walker |  | 5568 | 0 | impl method at src/lib.rs:699 |  |  | 0.549 |
| walker |  | 5568 | 0 | impl method at src/lib.rs:707 |  |  | 0.549 |
| walker |  | 5568 | 0 | impl method at src/lib.rs:714 |  |  | 0.549 |
| walker |  | 5568 | 0 | impl method at src/lib.rs:732 |  |  | 0.549 |
| walker |  | 5568 | 0 | impl method at src/lib.rs:752 |  |  | 0.549 |
| walker |  | 5568 | 0 | impl method at src/lib.rs:774 |  |  | 0.549 |
| walker |  | 5568 | 0 | impl method at src/lib.rs:788 |  |  | 0.549 |
| walker |  | 5568 | 0 | impl method at src/lib.rs:862 |  |  | 0.549 |
| walker |  | 5568 | 0 | impl method at src/lib.rs:872 |  |  | 0.549 |
| walker |  | 5568 | 0 | impl method at src/lib.rs:878 |  |  | 0.549 |
| walker |  | 5568 | 0 | impl method at src/lib.rs:884 |  |  | 0.549 |
| walker |  | 5568 | 0 | impl method at src/lib.rs:890 |  |  | 0.549 |
| walker |  | 5568 | 0 | impl method at src/lib.rs:896 |  |  | 0.549 |
| walker |  | 5568 | 0 | impl method at src/lib.rs:902 |  |  | 0.549 |
| walker |  | 5568 | 0 | impl method at src/lib.rs:908 |  |  | 0.549 |
| walker |  | 5568 | 0 | impl method at src/lib.rs:917 |  |  | 0.549 |
| walker |  | 5568 | 0 | impl method at src/lib.rs:923 |  |  | 0.549 |
| walker |  | 5568 | 0 | impl method at src/lib.rs:932 |  |  | 0.549 |
| walker |  | 5568 | 0 | impl method at src/lib.rs:1021 |  |  | 0.549 |
| walker |  | 5568 | 0 | impl method at src/lib.rs:1037 |  |  | 0.549 |
| walker |  | 5568 | 0 | impl method at src/lib.rs:1044 |  |  | 0.549 |
| walker |  | 5568 | 0 | impl method at src/lib.rs:1051 |  |  | 0.549 |
| walker |  | 5568 | 0 | impl method at src/lib.rs:1058 |  |  | 0.549 |
| walker |  | 5568 | 0 | impl method at src/lib.rs:1065 |  |  | 0.549 |
| walker |  | 5568 | 0 | impl method at src/lib.rs:1072 |  |  | 0.549 |
| walker |  | 5568 | 0 | impl method at src/lib.rs:1079 |  |  | 0.549 |
| walker |  | 5568 | 0 | impl method at src/lib.rs:1086 |  |  | 0.549 |
| walker |  | 5568 | 0 | impl method at src/lib.rs:1093 |  |  | 0.549 |
| walker |  | 5568 | 0 | impl method at src/lib.rs:1108 |  |  | 0.549 |
| walker |  | 5568 | 0 | impl method at src/lib.rs:1114 |  |  | 0.549 |
| walker |  | 5568 | 0 | impl method at src/lib.rs:1166 |  |  | 0.549 |
| walker |  | 5568 | 0 | impl method at src/lib.rs:1172 |  |  | 0.549 |
| walker |  | 5568 | 0 | impl method at src/lib.rs:1178 |  |  | 0.549 |
| walker |  | 5568 | 0 | impl method at src/lib.rs:1212 |  |  | 0.549 |
| walker |  | 5568 | 0 | impl method at src/lib.rs:1223 |  |  | 0.549 |
| walker |  | 5568 | 0 | impl method at src/lib.rs:1230 |  |  | 0.549 |
| walker |  | 5568 | 0 | impl method at src/lib.rs:1237 |  |  | 0.549 |
| walker |  | 5568 | 0 | impl method at src/lib.rs:1243 |  |  | 0.549 |
| walker |  | 5568 | 0 | impl method at src/lib.rs:1286 |  |  | 0.549 |
| walker |  | 5568 | 0 | impl method at src/lib.rs:1290 |  |  | 0.549 |
| walker |  | 5568 | 0 | impl method at src/lib.rs:1291 |  |  | 0.549 |
| walker |  | 5568 | 0 | impl method at src/lib.rs:1298 |  |  | 0.549 |
| walker |  | 5568 | 0 | impl method at src/lib.rs:1302 |  |  | 0.549 |
| walker |  | 5568 | 0 | impl method at src/lib.rs:1305 |  |  | 0.549 |
| walker |  | 5568 | 0 | impl method at src/lib.rs:1315 |  |  | 0.549 |
| walker |  | 5568 | 0 | impl method at src/lib.rs:1319 |  |  | 0.549 |
| walker |  | 5568 | 0 | impl method at src/lib.rs:1322 |  |  | 0.549 |
| walker |  | 5568 | 0 | impl method at src/lib.rs:1332 |  |  | 0.549 |
| walker |  | 5568 | 0 | impl method at src/lib.rs:1336 |  |  | 0.549 |
| walker |  | 5568 | 0 | impl method at src/lib.rs:1339 |  |  | 0.549 |
| walker |  | 5568 | 0 | impl method at src/lib.rs:1552 |  |  | 0.549 |
| walker |  | 5568 | 0 | impl method at src/lib.rs:1569 |  |  | 0.549 |
| walker |  | 5582 | 14 | impl method at src/lib.rs:939 |  |  | 0.549 |
| walker |  | 5596 | 14 | impl method at src/lib.rs:946 |  |  | 0.549 |
| walker |  | 5611 | 15 | impl method at src/lib.rs:1101 |  |  | 0.550 |
| walker |  | 5619 | 8 | impl method body at src/lib.rs:932 body 933 |  |  | 0.550 |
| walker |  | 5627 | 8 | impl method body at src/lib.rs:1286 body 1287 |  |  | 0.550 |
| walker |  | 5636 | 9 | impl method body at src/lib.rs:548 body 549 |  |  | 0.550 |
| walker |  | 5645 | 9 | impl method body at src/lib.rs:653 body 654 |  |  | 0.550 |
| walker |  | 5654 | 9 | impl method body at src/lib.rs:878 body 879 |  |  | 0.550 |
| walker |  | 5663 | 9 | impl method body at src/lib.rs:884 body 885 |  |  | 0.550 |
| walker |  | 5672 | 9 | impl method body at src/lib.rs:890 body 891 |  |  | 0.550 |
| walker |  | 5681 | 9 | impl method body at src/lib.rs:896 body 897 |  |  | 0.550 |
| walker |  | 5690 | 9 | impl method body at src/lib.rs:1172 body 1173 |  |  | 0.550 |
| walker |  | 5699 | 9 | impl method body at src/lib.rs:1178 body 1179 |  |  | 0.550 |
| walker |  | 5709 | 10 | impl method body at src/lib.rs:699 body 700 |  |  | 0.550 |
| walker |  | 5719 | 10 | impl method body at src/lib.rs:872 body 873 |  |  | 0.550 |
| walker |  | 5729 | 10 | impl method body at src/lib.rs:1108 body 1109 |  |  | 0.550 |
| walker |  | 5739 | 10 | impl method body at src/lib.rs:1114 body 1115 |  |  | 0.541 |
| ns | 5739 |  | 224 | `kv_support`: the capture_* functions behind every modifier | 5.4 |  | 0.541 |
| walker |  | 5749 | 10 | impl method body at src/lib.rs:1237 body 1238 |  |  | 0.541 |
| walker |  | 5759 | 10 | impl method body at src/lib.rs:1243 body 1244 |  |  | 0.541 |
| walker |  | 5770 | 11 | impl method body at src/lib.rs:529 body 530 |  |  | 0.541 |
| walker |  | 5781 | 11 | impl method body at src/lib.rs:679 body 680 |  |  | 0.541 |
| walker |  | 5792 | 11 | impl method body at src/lib.rs:939 body 940 |  |  | 0.541 |
| walker |  | 5803 | 11 | impl method body at src/lib.rs:1166 body 1167 |  |  | 0.541 |
| walker |  | 5814 | 11 | impl method body at src/lib.rs:1305 body 1306 |  |  | 0.541 |
| walker |  | 5826 | 12 | impl method body at src/lib.rs:1298 body 1299 |  |  | 0.541 |
| walker |  | 5838 | 12 | impl method body at src/lib.rs:1302 body 1303 |  |  | 0.541 |
| ns | 5845 |  | 106 | `struct Record`: every field | 6.1 | 2.1 | 0.551 |
| walker |  | 5850 | 12 | impl method body at src/lib.rs:1322 body 1323 |  |  | 0.551 |
| walker |  | 5862 | 12 | impl method body at src/lib.rs:1339 body 1340 |  |  | 0.551 |
| walker |  | 5875 | 13 | impl method body at src/lib.rs:561 body 562 |  |  | 0.551 |
| walker |  | 5888 | 13 | impl method body at src/lib.rs:714 body 715 |  |  | 0.551 |
| walker |  | 5901 | 13 | impl method body at src/lib.rs:1315 body 1316 |  |  | 0.551 |
| walker |  | 5914 | 13 | impl method body at src/lib.rs:1319 body 1320 |  |  | 0.551 |
| walker |  | 5927 | 13 | impl method body at src/lib.rs:1332 body 1333 |  |  | 0.551 |
| walker |  | 5940 | 13 | impl method body at src/lib.rs:1336 body 1337 |  |  | 0.551 |
| walker |  | 5954 | 14 | impl method body at src/lib.rs:917 body 918 |  |  | 0.551 |
| walker |  | 5968 | 14 | impl method body at src/lib.rs:1569 body 1570 |  |  | 0.551 |
| walker |  | 5983 | 15 | impl method body at src/lib.rs:503 body 504 |  |  | 0.551 |
| walker |  | 5998 | 15 | impl method body at src/lib.rs:707 body 708 |  |  | 0.551 |
| walker |  | 6013 | 15 | impl method body at src/lib.rs:902 body 903 |  |  | 0.551 |
| ns | 6026 |  | 181 | Roster: every accessor on `Record` | 6.2 |  | 0.564 |
| walker |  | 6028 | 15 | impl method body at src/lib.rs:1552 body 1553 |  |  | 0.564 |
| walker |  | 6046 | 18 | impl method body at src/lib.rs:510 body 511 |  |  | 0.564 |
| walker |  | 6064 | 18 | impl method body at src/lib.rs:554 body 555 |  |  | 0.564 |
| walker |  | 6082 | 18 | impl method body at src/lib.rs:660 body 661 |  |  | 0.564 |
| walker |  | 6102 | 20 | impl method body at src/lib.rs:1037 body 1038 |  |  | 0.564 |
| walker |  | 6122 | 20 | impl method body at src/lib.rs:1044 body 1045 |  |  | 0.564 |
| ns | 6127 |  | 101 | `Metadata` and its accessors | 6.3 | 2.1 | 0.571 |
| walker |  | 6142 | 20 | impl method body at src/lib.rs:1093 body 1094 |  |  | 0.571 |
| walker |  | 6162 | 20 | impl method body at src/lib.rs:1223 body 1224 |  |  | 0.571 |
| walker |  | 6182 | 20 | impl method body at src/lib.rs:1230 body 1231 |  |  | 0.571 |
| walker |  | 6203 | 21 | impl method body at src/lib.rs:1051 body 1052 |  |  | 0.571 |
| walker |  | 6224 | 21 | impl method body at src/lib.rs:1058 body 1059 |  |  | 0.571 |
| walker |  | 6247 | 23 | impl method body at src/lib.rs:579 body 580 |  |  | 0.571 |
| walker |  | 6270 | 23 | impl method body at src/lib.rs:732 body 733 |  |  | 0.571 |
| walker |  | 6294 | 24 | impl method body at src/lib.rs:1101 body 1102 |  |  | 0.571 |
| walker |  | 6321 | 27 | impl method body at src/lib.rs:1086 body 1087 |  |  | 0.571 |
| ns | 6333 |  | 206 | Roster: `RecordBuilder` and all twelve setters | 6.4 | 2.1 | 0.585 |
| walker |  | 6349 | 28 | impl method body at src/lib.rs:1072 body 1073 |  |  | 0.585 |
| walker |  | 6377 | 28 | impl method body at src/lib.rs:1079 body 1080 |  |  | 0.585 |
| walker |  | 6406 | 29 | impl method body at src/lib.rs:1065 body 1066 |  |  | 0.585 |
| ns | 6436 |  | 103 | `MetadataBuilder` and its setters | 6.5 | 2.1 | 0.591 |
| walker |  | 6439 | 33 | impl method body at src/lib.rs:599 body 600 |  |  | 0.591 |
| walker |  | 6472 | 33 | impl method body at src/lib.rs:752 body 753 |  |  | 0.591 |
| walker |  | 6508 | 36 | impl method body at src/lib.rs:620 body 621 |  |  | 0.591 |
| walker |  | 6544 | 36 | impl method body at src/lib.rs:774 body 775 |  |  | 0.591 |
| walker |  | 6586 | 42 | impl method body at src/lib.rs:862 body 863 |  |  | 0.591 |
| ns | 6593 |  | 157 | What structured logging means in `log` | 7.1 |  | 0.586 |
| walker |  | 6628 | 42 | impl method body at src/lib.rs:923 body 924 |  |  | 0.586 |
| walker |  | 6671 | 43 | impl method body at src/lib.rs:908 body 909 |  |  | 0.586 |
| ns | 6776 |  | 183 | The complete list of capture modifiers | 7.2 |  | 0.580 |
| walker |  | 6820 | 149 | pub item body at src/lib.rs:1529 body 1530 |  |  | 0.580 |
| walker |  | 6876 | 56 | pub item at src/kv/key.rs:37 |  |  | 0.580 |
| ns | 6981 |  | 205 | The kv module's structure and complete export list | 7.3 |  | 0.568 |
| walker |  | 7041 | 165 | pub item body at src/lib.rs:1396 body 1397 |  |  | 0.577 |
| ns | 7072 |  | 91 | `trait Source`: the three methods | 7.4 |  | 0.572 |
| walker |  | 7095 | 54 | impl method body at src/lib.rs:1212 body 1213 |  |  | 0.572 |
| walker |  | 7158 | 63 | pub-item doc lede at src/lib.rs:1566 |  |  | 0.572 |
| ns | 7279 |  | 207 | Roster: every type that implements `Source` | 7.5 |  | 0.564 |
| ns | 7363 |  | 84 | `trait VisitSource`: the visitor side of a `Source` | 7.6 |  | 0.565 |
| walker |  | 7440 | 282 | crate-doc body in src/lib.rs |  |  | 0.565 |
| ns | 7446 |  | 83 | `Value` and `ToValue` | 7.7 |  | 0.570 |
| walker |  | 7515 | 75 | pub-item doc lede at src/lib.rs:1351 |  |  | 0.570 |
| walker |  | 7615 | 100 | pub-item doc lede at src/lib.rs:475 |  |  | 0.570 |
| walker |  | 7624 | 9 | impl method body at src/lib.rs:431 body 432 |  |  | 0.570 |
| ns | 7691 |  | 245 | Roster: every constructor and conversion on `Value` | 7.8 |  | 0.560 |
| walker |  | 7712 | 88 | pub-item doc lede at src/lib.rs:1003 |  |  | 0.560 |
| walker |  | 7726 | 14 | pub-item doc lede at src/kv/error.rs:5 |  |  | 0.560 |
| walker |  | 7821 | 95 | pub-item doc lede at src/lib.rs:1200 |  |  | 0.560 |
| walker |  | 7831 | 10 | impl method body at src/lib.rs:435 body 436 |  |  | 0.560 |
| walker |  | 7851 | 20 | pub-item doc lede at src/kv/key.rs:7 |  |  | 0.560 |
| walker |  | 7871 | 20 | pub-item doc lede at src/kv/value.rs:11 |  |  | 0.560 |
| ns | 7967 |  | 276 | The primitive conversion tables | 7.9 |  | 0.551 |
| walker |  | 7993 | 122 | pub-item doc lede at src/lib.rs:636 |  |  | 0.551 |
| walker |  | 8089 | 96 | pub-item doc lede at src/lib.rs:1529 |  |  | 0.551 |
| walker |  | 8110 | 21 | pub-item doc lede at src/kv/source.rs:235 |  |  | 0.554 |
| ns | 8160 |  | 193 | Roster: every method on `VisitValue` | 7.10 |  | 0.546 |
| walker |  | 8209 | 99 | pub-item doc lede at src/lib.rs:1375 |  |  | 0.546 |
| walker |  | 8228 | 19 | README.md section #6 |  |  | 0.546 |
| ns | 8296 |  | 136 | The two `Value` backends: `value_bag` and the dependency-free fallback | 7.11 |  | 0.543 |
| ns | 8590 |  | 294 | `Key`, `ToKey`, and their feature-gated support modules | 7.12 |  | 0.536 |
| walker |  | 8638 | 410 | [features] in Cargo.toml |  |  | 0.591 |
| walker |  | 8727 | 89 | impl method body at src/lib.rs:517 body 518 |  |  | 0.591 |
| ns | 8762 |  | 172 | `kv::Error`: every variant of the private inner enum | 7.13 |  | 0.584 |
| ns | 8839 |  | 77 | `Source::get` and `Source::count` default implementations | 7.17 | 7.4 | 0.582 |
| walker |  | 9115 | 388 | macro_export names across src |  |  | 0.603 |
| ns | 9123 |  | 284 | Roster: every test in tests/macros.rs | 8.1 |  | 0.591 |
| walker |  | 9280 | 165 | macro_export names across src/kv |  |  | 0.591 |
| ns | 9299 |  | 176 | tests/integration.rs: the capturing test logger and what it pins | 8.2 |  | 0.585 |
| walker |  | 9324 | 44 | macro_export body at src/kv/value.rs:1129 |  |  | 0.585 |
| walker |  | 9368 | 44 | macro_export body at src/kv/value.rs:1139 |  |  | 0.585 |
| walker |  | 9413 | 45 | macro_export body at src/kv/value.rs:1149 |  |  | 0.585 |
| walker |  | 9458 | 45 | macro_export body at src/kv/value.rs:1159 |  |  | 0.585 |
| ns | 9465 |  | 166 | src/serde.rs: what is serialised, and how | 8.3 |  | 0.580 |
| walker |  | 9503 | 45 | macro_export body at src/kv/value.rs:1169 |  |  | 0.580 |
| walker |  | 9593 | 90 | impl method body at src/lib.rs:667 body 668 |  |  | 0.580 |
| ns | 9609 |  | 144 | CI: the seven jobs | 8.4 |  | 0.573 |
| ns | 9665 |  | 56 | The companion crate that tests compile-time filtering | 8.5 |  | 0.570 |
| walker |  | 9704 | 111 | pub-item doc lede at src/lib.rs:1420 |  |  | 0.570 |
| ns | 9737 |  | 72 | Roster: the value benchmarks | 8.6 |  | 0.568 |
| walker |  | 9831 | 127 | pub-item doc lede at src/lib.rs:1478 |  |  | 0.568 |
| ns | 9846 |  | 109 | The mutually-exclusive feature guards | 8.7 |  | 0.563 |
| ns | 9891 |  | 45 | CHANGELOG: format and latest release | 8.8 |  | 0.562 |
| walker |  | 9914 | 83 | README.md section #1 |  |  | 0.562 |
| ns | 9981 |  | 90 | Roster: the top-level sections of the structured-logging RFC | 8.9 |  | 0.558 |
