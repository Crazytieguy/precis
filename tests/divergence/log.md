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
| walker |  | 1766 | 20 | pub item body at src/lib.rs:1351 body 1352 |  |  | 0.668 |
| ns | 1784 |  | 132 | Module declarations and the atomics-vs-Cell import fork | 2.6 | 2.1 | 0.684 |
| ns | 1936 |  | 152 | Global state: LOGGER, STATE, the state constants, LOG_LEVEL_NAMES | 2.7 |  | 0.657 |
| walker |  | 1940 | 174 | pub item at src/lib.rs:636 |  |  | 0.700 |
| ns | 2119 |  | 183 | Roster: the impls that make Level and LevelFilter comparable and parseable | 2.8 |  | 0.672 |
| walker |  | 2243 | 303 | pub item at src/lib.rs:475 |  |  | 0.708 |
| walker |  | 2263 | 20 | pub-item doc lede at src/lib.rs:1158 |  |  | 0.708 |
| ns | 2397 |  | 278 | `STATIC_MAX_LEVEL`: compile-time level resolution | 2.9 | 2.1 | 0.678 |
| ns | 2472 |  | 75 | `trait Log`: the three required methods | 3.1 | 2.1 | 0.662 |
| ns | 2659 |  | 187 | Worked example: a complete `Log` implementation | 3.2 |  | 0.630 |
| walker |  | 2678 | 415 | pub item at src/lib.rs:1249 |  |  | 0.647 |
| walker |  | 2695 | 17 | pub-item doc lede at src/lib.rs:1249 |  |  | 0.658 |
| walker |  | 2716 | 21 | pub-item doc lede at src/lib.rs:842 |  |  | 0.658 |
| ns | 2815 |  | 156 | Why `set_max_level` must be called, and that it defaults to `Off` | 3.4 |  | 0.646 |
| ns | 2921 |  | 106 | Worked example: the conventional `init()` for a logger crate | 3.5 |  | 0.632 |
| walker |  | 2992 | 276 | pub item at src/lib.rs:1611 |  |  | 0.674 |
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
| ns | 3520 |  | 145 | Runtime max-level get and set bodies | 3.9 | 2.2 | 0.622 |
| ns | 3601 |  | 81 | no_std wiring | 3.10 |  | 0.616 |
| walker |  | 3678 | 223 | private module state in src/lib.rs |  |  | 0.645 |
| ns | 3681 |  | 80 | Roster: the seven public macros of src/macros.rs | 4.1 |  | 0.636 |
| walker |  | 3718 | 40 | pub-item names surface in src/kv/value.rs |  |  | 0.636 |
| walker |  | 3749 | 31 | pub item at src/kv/value.rs:11 |  |  | 0.637 |
| walker |  | 3775 | 26 | pub item at src/kv/value.rs:119 |  |  | 0.637 |
| ns | 3833 |  | 152 | `log!`: all four call forms | 4.2 | 4.1 | 0.626 |
| walker |  | 3924 | 149 | pub item body at src/lib.rs:1529 body 1530 |  |  | 0.626 |
| walker |  | 3980 | 56 | pub item at src/kv/key.rs:37 |  |  | 0.626 |
| ns | 4101 |  | 268 | `__log!`: the expansion every log call becomes | 4.3 |  | 0.612 |
| walker |  | 4145 | 165 | pub item body at src/lib.rs:1396 body 1397 |  |  | 0.625 |
| ns | 4172 |  | 71 | `error!`: head and matcher arms | 4.5 | 4.1 | 0.617 |
| walker |  | 4208 | 63 | pub-item doc lede at src/lib.rs:1566 |  |  | 0.617 |
| ns | 4408 |  | 236 | Roster: every doc(hidden) internal macro, with its feature fork | 4.6 |  | 0.598 |
| ns | 4765 |  | 357 | `__log_value!`: the capture-modifier dispatch table | 4.7 | 4.6 | 0.569 |
| ns | 4966 |  | 201 | `log_enabled!`: purpose and call forms | 4.9 | 4.1 | 0.558 |
| ns | 5170 |  | 204 | `__private_api`: the three functions macros expand into | 5.1 |  | 0.544 |
| ns | 5366 |  | 196 | `log_impl`: where a `Record` is actually built | 5.2 |  | 0.531 |
| ns | 5515 |  | 149 | `GlobalLogger`: the zero-sized proxy for the global slot | 5.3 |  | 0.519 |
| ns | 5739 |  | 224 | `kv_support`: the capture_* functions behind every modifier | 5.4 |  | 0.511 |
| ns | 5845 |  | 106 | `struct Record`: every field | 6.1 | 2.1 | 0.521 |
| ns | 6026 |  | 181 | Roster: every accessor on `Record` | 6.2 |  | 0.512 |
| ns | 6127 |  | 101 | `Metadata` and its accessors | 6.3 | 2.1 | 0.511 |
| walker |  | 6224 | 2016 | impl method sigs in src/lib.rs |  |  | 0.593 |
| walker |  | 6224 | 0 | impl method at src/lib.rs:427 |  |  | 0.593 |
| walker |  | 6224 | 0 | impl method at src/lib.rs:431 |  |  | 0.593 |
| walker |  | 6224 | 0 | impl method at src/lib.rs:435 |  |  | 0.593 |
| walker |  | 6224 | 0 | impl method at src/lib.rs:503 |  |  | 0.593 |
| walker |  | 6224 | 0 | impl method at src/lib.rs:510 |  |  | 0.593 |
| walker |  | 6224 | 0 | impl method at src/lib.rs:517 |  |  | 0.593 |
| walker |  | 6224 | 0 | impl method at src/lib.rs:529 |  |  | 0.593 |
| walker |  | 6224 | 0 | impl method at src/lib.rs:535 |  |  | 0.593 |
| walker |  | 6224 | 0 | impl method at src/lib.rs:548 |  |  | 0.593 |
| walker |  | 6224 | 0 | impl method at src/lib.rs:554 |  |  | 0.593 |
| walker |  | 6224 | 0 | impl method at src/lib.rs:561 |  |  | 0.593 |
| walker |  | 6224 | 0 | impl method at src/lib.rs:579 |  |  | 0.593 |
| walker |  | 6224 | 0 | impl method at src/lib.rs:599 |  |  | 0.593 |
| walker |  | 6224 | 0 | impl method at src/lib.rs:620 |  |  | 0.593 |
| walker |  | 6224 | 0 | impl method at src/lib.rs:653 |  |  | 0.593 |
| walker |  | 6224 | 0 | impl method at src/lib.rs:660 |  |  | 0.593 |
| walker |  | 6224 | 0 | impl method at src/lib.rs:667 |  |  | 0.593 |
| walker |  | 6224 | 0 | impl method at src/lib.rs:679 |  |  | 0.593 |
| walker |  | 6224 | 0 | impl method at src/lib.rs:685 |  |  | 0.593 |
| walker |  | 6224 | 0 | impl method at src/lib.rs:699 |  |  | 0.593 |
| walker |  | 6224 | 0 | impl method at src/lib.rs:707 |  |  | 0.593 |
| walker |  | 6224 | 0 | impl method at src/lib.rs:714 |  |  | 0.593 |
| walker |  | 6224 | 0 | impl method at src/lib.rs:732 |  |  | 0.593 |
| walker |  | 6224 | 0 | impl method at src/lib.rs:752 |  |  | 0.593 |
| walker |  | 6224 | 0 | impl method at src/lib.rs:774 |  |  | 0.593 |
| walker |  | 6224 | 0 | impl method at src/lib.rs:788 |  |  | 0.593 |
| walker |  | 6224 | 0 | impl method at src/lib.rs:862 |  |  | 0.593 |
| walker |  | 6224 | 0 | impl method at src/lib.rs:872 |  |  | 0.593 |
| walker |  | 6224 | 0 | impl method at src/lib.rs:878 |  |  | 0.593 |
| walker |  | 6224 | 0 | impl method at src/lib.rs:884 |  |  | 0.593 |
| walker |  | 6224 | 0 | impl method at src/lib.rs:890 |  |  | 0.593 |
| walker |  | 6224 | 0 | impl method at src/lib.rs:896 |  |  | 0.593 |
| walker |  | 6224 | 0 | impl method at src/lib.rs:902 |  |  | 0.593 |
| walker |  | 6224 | 0 | impl method at src/lib.rs:908 |  |  | 0.593 |
| walker |  | 6224 | 0 | impl method at src/lib.rs:917 |  |  | 0.593 |
| walker |  | 6224 | 0 | impl method at src/lib.rs:923 |  |  | 0.593 |
| walker |  | 6224 | 0 | impl method at src/lib.rs:932 |  |  | 0.593 |
| walker |  | 6224 | 0 | impl method at src/lib.rs:1021 |  |  | 0.593 |
| walker |  | 6224 | 0 | impl method at src/lib.rs:1037 |  |  | 0.593 |
| walker |  | 6224 | 0 | impl method at src/lib.rs:1044 |  |  | 0.593 |
| walker |  | 6224 | 0 | impl method at src/lib.rs:1051 |  |  | 0.593 |
| walker |  | 6224 | 0 | impl method at src/lib.rs:1058 |  |  | 0.593 |
| walker |  | 6224 | 0 | impl method at src/lib.rs:1065 |  |  | 0.593 |
| walker |  | 6224 | 0 | impl method at src/lib.rs:1072 |  |  | 0.593 |
| walker |  | 6224 | 0 | impl method at src/lib.rs:1079 |  |  | 0.593 |
| walker |  | 6224 | 0 | impl method at src/lib.rs:1086 |  |  | 0.593 |
| walker |  | 6224 | 0 | impl method at src/lib.rs:1093 |  |  | 0.593 |
| walker |  | 6224 | 0 | impl method at src/lib.rs:1108 |  |  | 0.593 |
| walker |  | 6224 | 0 | impl method at src/lib.rs:1114 |  |  | 0.593 |
| walker |  | 6224 | 0 | impl method at src/lib.rs:1166 |  |  | 0.593 |
| walker |  | 6224 | 0 | impl method at src/lib.rs:1172 |  |  | 0.593 |
| walker |  | 6224 | 0 | impl method at src/lib.rs:1178 |  |  | 0.593 |
| walker |  | 6224 | 0 | impl method at src/lib.rs:1212 |  |  | 0.593 |
| walker |  | 6224 | 0 | impl method at src/lib.rs:1223 |  |  | 0.593 |
| walker |  | 6224 | 0 | impl method at src/lib.rs:1230 |  |  | 0.593 |
| walker |  | 6224 | 0 | impl method at src/lib.rs:1237 |  |  | 0.593 |
| walker |  | 6224 | 0 | impl method at src/lib.rs:1243 |  |  | 0.593 |
| walker |  | 6224 | 0 | impl method at src/lib.rs:1286 |  |  | 0.593 |
| walker |  | 6224 | 0 | impl method at src/lib.rs:1290 |  |  | 0.593 |
| walker |  | 6224 | 0 | impl method at src/lib.rs:1291 |  |  | 0.593 |
| walker |  | 6224 | 0 | impl method at src/lib.rs:1298 |  |  | 0.593 |
| walker |  | 6224 | 0 | impl method at src/lib.rs:1302 |  |  | 0.593 |
| walker |  | 6224 | 0 | impl method at src/lib.rs:1305 |  |  | 0.593 |
| walker |  | 6224 | 0 | impl method at src/lib.rs:1315 |  |  | 0.593 |
| walker |  | 6224 | 0 | impl method at src/lib.rs:1319 |  |  | 0.593 |
| walker |  | 6224 | 0 | impl method at src/lib.rs:1322 |  |  | 0.593 |
| walker |  | 6224 | 0 | impl method at src/lib.rs:1332 |  |  | 0.593 |
| walker |  | 6224 | 0 | impl method at src/lib.rs:1336 |  |  | 0.593 |
| walker |  | 6224 | 0 | impl method at src/lib.rs:1339 |  |  | 0.593 |
| walker |  | 6224 | 0 | impl method at src/lib.rs:1552 |  |  | 0.593 |
| walker |  | 6224 | 0 | impl method at src/lib.rs:1569 |  |  | 0.593 |
| walker |  | 6238 | 14 | impl method at src/lib.rs:939 |  |  | 0.596 |
| walker |  | 6252 | 14 | impl method at src/lib.rs:946 |  |  | 0.600 |
| walker |  | 6267 | 15 | impl method at src/lib.rs:1101 |  |  | 0.600 |
| walker |  | 6275 | 8 | impl method body at src/lib.rs:932 body 933 |  |  | 0.600 |
| walker |  | 6283 | 8 | impl method body at src/lib.rs:1286 body 1287 |  |  | 0.600 |
| walker |  | 6292 | 9 | impl method body at src/lib.rs:548 body 549 |  |  | 0.600 |
| walker |  | 6301 | 9 | impl method body at src/lib.rs:653 body 654 |  |  | 0.600 |
| walker |  | 6310 | 9 | impl method body at src/lib.rs:878 body 879 |  |  | 0.600 |
| walker |  | 6319 | 9 | impl method body at src/lib.rs:884 body 885 |  |  | 0.600 |
| walker |  | 6328 | 9 | impl method body at src/lib.rs:890 body 891 |  |  | 0.600 |
| ns | 6333 |  | 206 | Roster: `RecordBuilder` and all twelve setters | 6.4 | 2.1 | 0.612 |
| walker |  | 6337 | 9 | impl method body at src/lib.rs:896 body 897 |  |  | 0.612 |
| walker |  | 6346 | 9 | impl method body at src/lib.rs:1172 body 1173 |  |  | 0.612 |
| walker |  | 6355 | 9 | impl method body at src/lib.rs:1178 body 1179 |  |  | 0.612 |
| walker |  | 6365 | 10 | impl method body at src/lib.rs:699 body 700 |  |  | 0.612 |
| walker |  | 6375 | 10 | impl method body at src/lib.rs:872 body 873 |  |  | 0.612 |
| walker |  | 6385 | 10 | impl method body at src/lib.rs:1108 body 1109 |  |  | 0.612 |
| walker |  | 6395 | 10 | impl method body at src/lib.rs:1114 body 1115 |  |  | 0.612 |
| walker |  | 6405 | 10 | impl method body at src/lib.rs:1237 body 1238 |  |  | 0.612 |
| walker |  | 6415 | 10 | impl method body at src/lib.rs:1243 body 1244 |  |  | 0.612 |
| walker |  | 6426 | 11 | impl method body at src/lib.rs:529 body 530 |  |  | 0.612 |
| ns | 6436 |  | 103 | `MetadataBuilder` and its setters | 6.5 | 2.1 | 0.618 |
| walker |  | 6437 | 11 | impl method body at src/lib.rs:679 body 680 |  |  | 0.618 |
| walker |  | 6448 | 11 | impl method body at src/lib.rs:939 body 940 |  |  | 0.618 |
| walker |  | 6459 | 11 | impl method body at src/lib.rs:1166 body 1167 |  |  | 0.618 |
| walker |  | 6470 | 11 | impl method body at src/lib.rs:1305 body 1306 |  |  | 0.618 |
| walker |  | 6482 | 12 | impl method body at src/lib.rs:1298 body 1299 |  |  | 0.618 |
| walker |  | 6494 | 12 | impl method body at src/lib.rs:1302 body 1303 |  |  | 0.618 |
| walker |  | 6506 | 12 | impl method body at src/lib.rs:1322 body 1323 |  |  | 0.618 |
| walker |  | 6518 | 12 | impl method body at src/lib.rs:1339 body 1340 |  |  | 0.618 |
| walker |  | 6531 | 13 | impl method body at src/lib.rs:561 body 562 |  |  | 0.618 |
| walker |  | 6544 | 13 | impl method body at src/lib.rs:714 body 715 |  |  | 0.618 |
| walker |  | 6557 | 13 | impl method body at src/lib.rs:1315 body 1316 |  |  | 0.618 |
| walker |  | 6570 | 13 | impl method body at src/lib.rs:1319 body 1320 |  |  | 0.618 |
| walker |  | 6583 | 13 | impl method body at src/lib.rs:1332 body 1333 |  |  | 0.618 |
| ns | 6593 |  | 157 | What structured logging means in `log` | 7.1 |  | 0.612 |
| walker |  | 6596 | 13 | impl method body at src/lib.rs:1336 body 1337 |  |  | 0.612 |
| walker |  | 6610 | 14 | impl method body at src/lib.rs:917 body 918 |  |  | 0.612 |
| walker |  | 6624 | 14 | impl method body at src/lib.rs:1569 body 1570 |  |  | 0.612 |
| walker |  | 6639 | 15 | impl method body at src/lib.rs:503 body 504 |  |  | 0.612 |
| walker |  | 6654 | 15 | impl method body at src/lib.rs:707 body 708 |  |  | 0.612 |
| walker |  | 6669 | 15 | impl method body at src/lib.rs:902 body 903 |  |  | 0.612 |
| walker |  | 6684 | 15 | impl method body at src/lib.rs:1552 body 1553 |  |  | 0.612 |
| walker |  | 6702 | 18 | impl method body at src/lib.rs:510 body 511 |  |  | 0.612 |
| walker |  | 6720 | 18 | impl method body at src/lib.rs:554 body 555 |  |  | 0.612 |
| walker |  | 6738 | 18 | impl method body at src/lib.rs:660 body 661 |  |  | 0.612 |
| walker |  | 6758 | 20 | impl method body at src/lib.rs:1037 body 1038 |  |  | 0.612 |
| ns | 6776 |  | 183 | The complete list of capture modifiers | 7.2 |  | 0.607 |
| walker |  | 6778 | 20 | impl method body at src/lib.rs:1044 body 1045 |  |  | 0.607 |
| walker |  | 6798 | 20 | impl method body at src/lib.rs:1093 body 1094 |  |  | 0.607 |
| walker |  | 6818 | 20 | impl method body at src/lib.rs:1223 body 1224 |  |  | 0.607 |
| walker |  | 6838 | 20 | impl method body at src/lib.rs:1230 body 1231 |  |  | 0.607 |
| walker |  | 6859 | 21 | impl method body at src/lib.rs:1051 body 1052 |  |  | 0.607 |
| walker |  | 6880 | 21 | impl method body at src/lib.rs:1058 body 1059 |  |  | 0.607 |
| walker |  | 6903 | 23 | impl method body at src/lib.rs:579 body 580 |  |  | 0.607 |
| walker |  | 6926 | 23 | impl method body at src/lib.rs:732 body 733 |  |  | 0.607 |
| walker |  | 6950 | 24 | impl method body at src/lib.rs:1101 body 1102 |  |  | 0.607 |
| walker |  | 6977 | 27 | impl method body at src/lib.rs:1086 body 1087 |  |  | 0.607 |
| ns | 6981 |  | 205 | The kv module's structure and complete export list | 7.3 |  | 0.594 |
| walker |  | 7005 | 28 | impl method body at src/lib.rs:1072 body 1073 |  |  | 0.594 |
| walker |  | 7033 | 28 | impl method body at src/lib.rs:1079 body 1080 |  |  | 0.594 |
| walker |  | 7062 | 29 | impl method body at src/lib.rs:1065 body 1066 |  |  | 0.594 |
| ns | 7072 |  | 91 | `trait Source`: the three methods | 7.4 |  | 0.589 |
| walker |  | 7095 | 33 | impl method body at src/lib.rs:599 body 600 |  |  | 0.589 |
| walker |  | 7128 | 33 | impl method body at src/lib.rs:752 body 753 |  |  | 0.589 |
| walker |  | 7164 | 36 | impl method body at src/lib.rs:620 body 621 |  |  | 0.589 |
| walker |  | 7200 | 36 | impl method body at src/lib.rs:774 body 775 |  |  | 0.589 |
| walker |  | 7242 | 42 | impl method body at src/lib.rs:862 body 863 |  |  | 0.589 |
| ns | 7279 |  | 207 | Roster: every type that implements `Source` | 7.5 |  | 0.581 |
| walker |  | 7284 | 42 | impl method body at src/lib.rs:923 body 924 |  |  | 0.581 |
| walker |  | 7327 | 43 | impl method body at src/lib.rs:908 body 909 |  |  | 0.581 |
| ns | 7363 |  | 84 | `trait VisitSource`: the visitor side of a `Source` | 7.6 |  | 0.581 |
| walker |  | 7381 | 54 | impl method body at src/lib.rs:1212 body 1213 |  |  | 0.581 |
| ns | 7446 |  | 83 | `Value` and `ToValue` | 7.7 |  | 0.586 |
| walker |  | 7479 | 98 | pub-item doc lede at src/lib.rs:475 |  |  | 0.586 |
| walker |  | 7554 | 75 | pub-item doc lede at src/lib.rs:1351 |  |  | 0.586 |
| walker |  | 7563 | 9 | impl method body at src/lib.rs:431 body 432 |  |  | 0.586 |
| walker |  | 7651 | 88 | pub-item doc lede at src/lib.rs:1003 |  |  | 0.586 |
| walker |  | 7665 | 14 | pub-item doc lede at src/kv/error.rs:5 |  |  | 0.586 |
| ns | 7691 |  | 245 | Roster: every constructor and conversion on `Value` | 7.8 |  | 0.576 |
| walker |  | 7760 | 95 | pub-item doc lede at src/lib.rs:1200 |  |  | 0.576 |
| walker |  | 7770 | 10 | impl method body at src/lib.rs:435 body 436 |  |  | 0.576 |
| walker |  | 7790 | 20 | pub-item doc lede at src/kv/key.rs:7 |  |  | 0.576 |
| walker |  | 7810 | 20 | pub-item doc lede at src/kv/value.rs:11 |  |  | 0.576 |
| walker |  | 7932 | 122 | pub-item doc lede at src/lib.rs:636 |  |  | 0.576 |
| ns | 7967 |  | 276 | The primitive conversion tables | 7.9 |  | 0.567 |
| walker |  | 8028 | 96 | pub-item doc lede at src/lib.rs:1529 |  |  | 0.567 |
| walker |  | 8049 | 21 | pub-item doc lede at src/kv/source.rs:235 |  |  | 0.570 |
| walker |  | 8148 | 99 | pub-item doc lede at src/lib.rs:1375 |  |  | 0.570 |
| ns | 8160 |  | 193 | Roster: every method on `VisitValue` | 7.10 |  | 0.562 |
| walker |  | 8167 | 19 | README.md section #6 |  |  | 0.562 |
| ns | 8296 |  | 136 | The two `Value` backends: `value_bag` and the dependency-free fallback | 7.11 |  | 0.558 |
| walker |  | 8577 | 410 | [features] in Cargo.toml |  |  | 0.614 |
| ns | 8590 |  | 294 | `Key`, `ToKey`, and their feature-gated support modules | 7.12 |  | 0.606 |
| walker |  | 8666 | 89 | impl method body at src/lib.rs:517 body 518 |  |  | 0.606 |
| ns | 8762 |  | 172 | `kv::Error`: every variant of the private inner enum | 7.13 |  | 0.599 |
| ns | 8839 |  | 77 | `Source::get` and `Source::count` default implementations | 7.17 | 7.4 | 0.597 |
| walker |  | 9054 | 388 | macro_export names across src |  |  | 0.617 |
| ns | 9123 |  | 284 | Roster: every test in tests/macros.rs | 8.1 |  | 0.605 |
| walker |  | 9219 | 165 | macro_export names across src/kv |  |  | 0.605 |
| walker |  | 9263 | 44 | macro_export body at src/kv/value.rs:1129 |  |  | 0.605 |
| ns | 9299 |  | 176 | tests/integration.rs: the capturing test logger and what it pins | 8.2 |  | 0.599 |
| walker |  | 9307 | 44 | macro_export body at src/kv/value.rs:1139 |  |  | 0.599 |
| walker |  | 9352 | 45 | macro_export body at src/kv/value.rs:1149 |  |  | 0.599 |
| walker |  | 9397 | 45 | macro_export body at src/kv/value.rs:1159 |  |  | 0.599 |
| walker |  | 9442 | 45 | macro_export body at src/kv/value.rs:1169 |  |  | 0.599 |
| ns | 9465 |  | 166 | src/serde.rs: what is serialised, and how | 8.3 |  | 0.594 |
| walker |  | 9532 | 90 | impl method body at src/lib.rs:667 body 668 |  |  | 0.594 |
| ns | 9609 |  | 144 | CI: the seven jobs | 8.4 |  | 0.587 |
| walker |  | 9643 | 111 | pub-item doc lede at src/lib.rs:1420 |  |  | 0.587 |
| ns | 9665 |  | 56 | The companion crate that tests compile-time filtering | 8.5 |  | 0.584 |
| ns | 9737 |  | 72 | Roster: the value benchmarks | 8.6 |  | 0.582 |
| walker |  | 9770 | 127 | pub-item doc lede at src/lib.rs:1478 |  |  | 0.582 |
| ns | 9846 |  | 109 | The mutually-exclusive feature guards | 8.7 |  | 0.577 |
| walker |  | 9853 | 83 | README.md section #1 |  |  | 0.577 |
| ns | 9891 |  | 45 | CHANGELOG: format and latest release | 8.8 |  | 0.575 |
| ns | 9981 |  | 90 | Roster: the top-level sections of the structured-logging RFC | 8.9 |  | 0.572 |
