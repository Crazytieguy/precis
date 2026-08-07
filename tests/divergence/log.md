Score(3000)=0.664 I=0.847 C=0.521 ns_rows≤3K=23/67 (reached=13 partial=1 missing=9)

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
| ns | 1652 |  | 184 | Roster: every method on `Level` and `LevelFilter` | 2.5 |  | 0.666 |
| walker |  | 1718 | 106 | pub item at src/lib.rs:842 |  |  | 0.668 |
| ns | 1784 |  | 132 | Module declarations and the atomics-vs-Cell import fork | 2.6 | 2.1 | 0.684 |
| walker |  | 1892 | 174 | pub item at src/lib.rs:636 |  |  | 0.728 |
| ns | 1936 |  | 152 | Global state: LOGGER, STATE, the state constants, LOG_LEVEL_NAMES | 2.7 |  | 0.700 |
| ns | 2119 |  | 183 | Roster: the impls that make Level and LevelFilter comparable and parseable | 2.8 |  | 0.672 |
| walker |  | 2195 | 303 | pub item at src/lib.rs:475 |  |  | 0.708 |
| ns | 2397 |  | 278 | `STATIC_MAX_LEVEL`: compile-time level resolution | 2.9 | 2.1 | 0.678 |
| ns | 2472 |  | 75 | `trait Log`: the three required methods | 3.1 | 2.1 | 0.661 |
| walker |  | 2610 | 415 | pub item at src/lib.rs:1249 |  |  | 0.680 |
| ns | 2659 |  | 187 | Worked example: a complete `Log` implementation | 3.2 |  | 0.647 |
| ns | 2815 |  | 156 | Why `set_max_level` must be called, and that it defaults to `Off` | 3.4 |  | 0.635 |
| walker |  | 2886 | 276 | pub item at src/lib.rs:1611 |  |  | 0.679 |
| walker |  | 2912 | 26 | pub-item names surface in src/kv/key.rs |  |  | 0.679 |
| ns | 2921 |  | 106 | Worked example: the conventional `init()` for a logger crate | 3.5 |  | 0.664 |
| walker |  | 2943 | 31 | pub item at src/kv/key.rs:7 |  |  | 0.664 |
| walker |  | 3062 | 119 | [dependencies] in Cargo.toml |  |  | 0.681 |
| ns | 3074 |  | 153 | The `Log` impls the crate itself provides | 3.6 |  | 0.660 |
| walker |  | 3089 | 27 | pub-item names surface in src/kv/source.rs |  |  | 0.660 |
| walker |  | 3123 | 34 | pub item at src/kv/error.rs:5 |  |  | 0.661 |
| walker |  | 3171 | 48 | pub item at src/kv/source.rs:235 |  |  | 0.661 |
| ns | 3188 |  | 114 | `logger()`: the acquire-load fast path | 3.7 | 2.2 | 0.647 |
| ns | 3375 |  | 187 | `set_logger_inner`: the compare-exchange install path | 3.8 |  | 0.623 |
| walker |  | 3394 | 223 | private module state in src/lib.rs |  |  | 0.653 |
| walker |  | 3434 | 40 | pub-item names surface in src/kv/value.rs |  |  | 0.653 |
| walker |  | 3465 | 31 | pub item at src/kv/value.rs:11 |  |  | 0.653 |
| walker |  | 3491 | 26 | pub item at src/kv/value.rs:119 |  |  | 0.654 |
| ns | 3520 |  | 145 | Runtime max-level get and set bodies | 3.9 | 2.2 | 0.643 |
| walker |  | 3547 | 56 | pub item at src/kv/key.rs:37 |  |  | 0.643 |
| ns | 3601 |  | 81 | no_std wiring | 3.10 |  | 0.637 |
| ns | 3681 |  | 80 | Roster: the seven public macros of src/macros.rs | 4.1 |  | 0.628 |
| ns | 3833 |  | 152 | `log!`: all four call forms | 4.2 | 4.1 | 0.617 |
| ns | 4101 |  | 268 | `__log!`: the expansion every log call becomes | 4.3 |  | 0.603 |
| ns | 4172 |  | 71 | `error!`: head and matcher arms | 4.5 | 4.1 | 0.595 |
| ns | 4408 |  | 236 | Roster: every doc(hidden) internal macro, with its feature fork | 4.6 |  | 0.577 |
| ns | 4765 |  | 357 | `__log_value!`: the capture-modifier dispatch table | 4.7 | 4.6 | 0.549 |
| ns | 4966 |  | 201 | `log_enabled!`: purpose and call forms | 4.9 | 4.1 | 0.538 |
| ns | 5170 |  | 204 | `__private_api`: the three functions macros expand into | 5.1 |  | 0.525 |
| ns | 5366 |  | 196 | `log_impl`: where a `Record` is actually built | 5.2 |  | 0.512 |
| ns | 5515 |  | 149 | `GlobalLogger`: the zero-sized proxy for the global slot | 5.3 |  | 0.500 |
| walker |  | 5563 | 2016 | impl method sigs in src/lib.rs |  |  | 0.564 |
| walker |  | 5563 | 0 | impl method at src/lib.rs:427 |  |  | 0.564 |
| walker |  | 5563 | 0 | impl method at src/lib.rs:431 |  |  | 0.564 |
| walker |  | 5563 | 0 | impl method at src/lib.rs:435 |  |  | 0.564 |
| walker |  | 5563 | 0 | impl method at src/lib.rs:503 |  |  | 0.564 |
| walker |  | 5563 | 0 | impl method at src/lib.rs:510 |  |  | 0.564 |
| walker |  | 5563 | 0 | impl method at src/lib.rs:517 |  |  | 0.564 |
| walker |  | 5563 | 0 | impl method at src/lib.rs:529 |  |  | 0.564 |
| walker |  | 5563 | 0 | impl method at src/lib.rs:535 |  |  | 0.564 |
| walker |  | 5563 | 0 | impl method at src/lib.rs:548 |  |  | 0.564 |
| walker |  | 5563 | 0 | impl method at src/lib.rs:554 |  |  | 0.564 |
| walker |  | 5563 | 0 | impl method at src/lib.rs:561 |  |  | 0.564 |
| walker |  | 5563 | 0 | impl method at src/lib.rs:579 |  |  | 0.564 |
| walker |  | 5563 | 0 | impl method at src/lib.rs:599 |  |  | 0.564 |
| walker |  | 5563 | 0 | impl method at src/lib.rs:620 |  |  | 0.564 |
| walker |  | 5563 | 0 | impl method at src/lib.rs:653 |  |  | 0.564 |
| walker |  | 5563 | 0 | impl method at src/lib.rs:660 |  |  | 0.564 |
| walker |  | 5563 | 0 | impl method at src/lib.rs:667 |  |  | 0.564 |
| walker |  | 5563 | 0 | impl method at src/lib.rs:679 |  |  | 0.564 |
| walker |  | 5563 | 0 | impl method at src/lib.rs:685 |  |  | 0.564 |
| walker |  | 5563 | 0 | impl method at src/lib.rs:699 |  |  | 0.564 |
| walker |  | 5563 | 0 | impl method at src/lib.rs:707 |  |  | 0.564 |
| walker |  | 5563 | 0 | impl method at src/lib.rs:714 |  |  | 0.564 |
| walker |  | 5563 | 0 | impl method at src/lib.rs:732 |  |  | 0.564 |
| walker |  | 5563 | 0 | impl method at src/lib.rs:752 |  |  | 0.564 |
| walker |  | 5563 | 0 | impl method at src/lib.rs:774 |  |  | 0.564 |
| walker |  | 5563 | 0 | impl method at src/lib.rs:788 |  |  | 0.564 |
| walker |  | 5563 | 0 | impl method at src/lib.rs:862 |  |  | 0.564 |
| walker |  | 5563 | 0 | impl method at src/lib.rs:872 |  |  | 0.564 |
| walker |  | 5563 | 0 | impl method at src/lib.rs:878 |  |  | 0.564 |
| walker |  | 5563 | 0 | impl method at src/lib.rs:884 |  |  | 0.564 |
| walker |  | 5563 | 0 | impl method at src/lib.rs:890 |  |  | 0.564 |
| walker |  | 5563 | 0 | impl method at src/lib.rs:896 |  |  | 0.564 |
| walker |  | 5563 | 0 | impl method at src/lib.rs:902 |  |  | 0.564 |
| walker |  | 5563 | 0 | impl method at src/lib.rs:908 |  |  | 0.564 |
| walker |  | 5563 | 0 | impl method at src/lib.rs:917 |  |  | 0.564 |
| walker |  | 5563 | 0 | impl method at src/lib.rs:923 |  |  | 0.564 |
| walker |  | 5563 | 0 | impl method at src/lib.rs:932 |  |  | 0.564 |
| walker |  | 5563 | 0 | impl method at src/lib.rs:1021 |  |  | 0.564 |
| walker |  | 5563 | 0 | impl method at src/lib.rs:1037 |  |  | 0.564 |
| walker |  | 5563 | 0 | impl method at src/lib.rs:1044 |  |  | 0.564 |
| walker |  | 5563 | 0 | impl method at src/lib.rs:1051 |  |  | 0.564 |
| walker |  | 5563 | 0 | impl method at src/lib.rs:1058 |  |  | 0.564 |
| walker |  | 5563 | 0 | impl method at src/lib.rs:1065 |  |  | 0.564 |
| walker |  | 5563 | 0 | impl method at src/lib.rs:1072 |  |  | 0.564 |
| walker |  | 5563 | 0 | impl method at src/lib.rs:1079 |  |  | 0.564 |
| walker |  | 5563 | 0 | impl method at src/lib.rs:1086 |  |  | 0.564 |
| walker |  | 5563 | 0 | impl method at src/lib.rs:1093 |  |  | 0.564 |
| walker |  | 5563 | 0 | impl method at src/lib.rs:1108 |  |  | 0.564 |
| walker |  | 5563 | 0 | impl method at src/lib.rs:1114 |  |  | 0.564 |
| walker |  | 5563 | 0 | impl method at src/lib.rs:1166 |  |  | 0.564 |
| walker |  | 5563 | 0 | impl method at src/lib.rs:1172 |  |  | 0.564 |
| walker |  | 5563 | 0 | impl method at src/lib.rs:1178 |  |  | 0.564 |
| walker |  | 5563 | 0 | impl method at src/lib.rs:1212 |  |  | 0.564 |
| walker |  | 5563 | 0 | impl method at src/lib.rs:1223 |  |  | 0.564 |
| walker |  | 5563 | 0 | impl method at src/lib.rs:1230 |  |  | 0.564 |
| walker |  | 5563 | 0 | impl method at src/lib.rs:1237 |  |  | 0.564 |
| walker |  | 5563 | 0 | impl method at src/lib.rs:1243 |  |  | 0.564 |
| walker |  | 5563 | 0 | impl method at src/lib.rs:1286 |  |  | 0.564 |
| walker |  | 5563 | 0 | impl method at src/lib.rs:1290 |  |  | 0.564 |
| walker |  | 5563 | 0 | impl method at src/lib.rs:1291 |  |  | 0.564 |
| walker |  | 5563 | 0 | impl method at src/lib.rs:1298 |  |  | 0.564 |
| walker |  | 5563 | 0 | impl method at src/lib.rs:1302 |  |  | 0.564 |
| walker |  | 5563 | 0 | impl method at src/lib.rs:1305 |  |  | 0.564 |
| walker |  | 5563 | 0 | impl method at src/lib.rs:1315 |  |  | 0.564 |
| walker |  | 5563 | 0 | impl method at src/lib.rs:1319 |  |  | 0.564 |
| walker |  | 5563 | 0 | impl method at src/lib.rs:1322 |  |  | 0.564 |
| walker |  | 5563 | 0 | impl method at src/lib.rs:1332 |  |  | 0.564 |
| walker |  | 5563 | 0 | impl method at src/lib.rs:1336 |  |  | 0.564 |
| walker |  | 5563 | 0 | impl method at src/lib.rs:1339 |  |  | 0.564 |
| walker |  | 5563 | 0 | impl method at src/lib.rs:1552 |  |  | 0.564 |
| walker |  | 5563 | 0 | impl method at src/lib.rs:1569 |  |  | 0.564 |
| walker |  | 5577 | 14 | impl method at src/lib.rs:939 |  |  | 0.564 |
| walker |  | 5591 | 14 | impl method at src/lib.rs:946 |  |  | 0.564 |
| walker |  | 5606 | 15 | impl method at src/lib.rs:1101 |  |  | 0.565 |
| walker |  | 5625 | 19 | README.md section #6 |  |  | 0.565 |
| ns | 5739 |  | 224 | `kv_support`: the capture_* functions behind every modifier | 5.4 |  | 0.556 |
| ns | 5845 |  | 106 | `struct Record`: every field | 6.1 | 2.1 | 0.565 |
| ns | 6026 |  | 181 | Roster: every accessor on `Record` | 6.2 |  | 0.578 |
| walker |  | 6035 | 410 | [features] in Cargo.toml |  |  | 0.646 |
| ns | 6127 |  | 101 | `Metadata` and its accessors | 6.3 | 2.1 | 0.652 |
| ns | 6333 |  | 206 | Roster: `RecordBuilder` and all twelve setters | 6.4 | 2.1 | 0.662 |
| walker |  | 6423 | 388 | macro_export names across src |  |  | 0.687 |
| ns | 6436 |  | 103 | `MetadataBuilder` and its setters | 6.5 | 2.1 | 0.692 |
| walker |  | 6588 | 165 | macro_export names across src/kv |  |  | 0.692 |
| ns | 6593 |  | 157 | What structured logging means in `log` | 7.1 |  | 0.686 |
| walker |  | 6671 | 83 | README.md section #1 |  |  | 0.686 |
| ns | 6776 |  | 183 | The complete list of capture modifiers | 7.2 |  | 0.679 |
| walker |  | 6876 | 205 | mod/use plumbing in src/kv/mod.rs |  |  | 0.681 |
| ns | 6981 |  | 205 | The kv module's structure and complete export list | 7.3 |  | 0.691 |
| ns | 7072 |  | 91 | `trait Source`: the three methods | 7.4 |  | 0.685 |
| walker |  | 7158 | 282 | crate-doc body in src/lib.rs |  |  | 0.685 |
| walker |  | 7170 | 12 | pub item body at src/lib.rs:1478 body 1479 |  |  | 0.685 |
| walker |  | 7178 | 8 | impl method body at src/lib.rs:932 body 933 |  |  | 0.685 |
| ns | 7279 |  | 207 | Roster: every type that implements `Source` | 7.5 |  | 0.676 |
| ns | 7363 |  | 84 | `trait VisitSource`: the visitor side of a `Source` | 7.6 |  | 0.675 |
| ns | 7446 |  | 83 | `Value` and `ToValue` | 7.7 |  | 0.679 |
| walker |  | 7532 | 354 | crate-doc body in src/kv/mod.rs |  |  | 0.689 |
| ns | 7691 |  | 245 | Roster: every constructor and conversion on `Value` | 7.8 |  | 0.677 |
| ns | 7967 |  | 276 | The primitive conversion tables | 7.9 |  | 0.666 |
| walker |  | 7969 | 437 | pub item at src/kv/source.rs:51 |  |  | 0.676 |
| walker |  | 7977 | 8 | impl method body at src/lib.rs:1286 body 1287 |  |  | 0.676 |
| ns | 8160 |  | 193 | Roster: every method on `VisitValue` | 7.10 |  | 0.667 |
| walker |  | 8168 | 191 | impl method sigs in src/kv/key.rs |  |  | 0.667 |
| walker |  | 8168 | 0 | impl method at src/kv/key.rs:16 |  |  | 0.667 |
| walker |  | 8168 | 0 | impl method at src/kv/key.rs:22 |  |  | 0.667 |
| walker |  | 8168 | 0 | impl method at src/kv/key.rs:28 |  |  | 0.667 |
| walker |  | 8168 | 0 | impl method at src/kv/key.rs:45 |  |  | 0.667 |
| walker |  | 8168 | 0 | impl method at src/kv/key.rs:53 |  |  | 0.667 |
| walker |  | 8168 | 0 | impl method at src/kv/key.rs:61 |  |  | 0.667 |
| walker |  | 8184 | 16 | pub item body at src/lib.rs:1420 body 1421 |  |  | 0.667 |
| walker |  | 8277 | 93 | pub-item names surface in src/__private_api.rs |  |  | 0.668 |
| walker |  | 8277 | 0 | pub item at src/__private_api.rs:103 |  |  | 0.668 |
| walker |  | 8277 | 0 | pub item at src/__private_api.rs:108 |  |  | 0.668 |
| walker |  | 8285 | 8 | pub item at src/__private_api.rs:39 |  |  | 0.669 |
| ns | 8296 |  | 136 | The two `Value` backends: `value_bag` and the dependency-free fallback | 7.11 |  | 0.664 |
| walker |  | 8298 | 13 | pub item at src/__private_api.rs:10 |  |  | 0.664 |
| walker |  | 8391 | 93 | pub item at src/__private_api.rs:84 |  |  | 0.673 |
| walker |  | 8452 | 61 | README.md section #5 |  |  | 0.673 |
| walker |  | 8525 | 73 | README.md section #2 |  |  | 0.673 |
| ns | 8590 |  | 294 | `Key`, `ToKey`, and their feature-gated support modules | 7.12 |  | 0.665 |
| ns | 8762 |  | 172 | `kv::Error`: every variant of the private inner enum | 7.13 |  | 0.656 |
| ns | 8839 |  | 77 | `Source::get` and `Source::count` default implementations | 7.17 | 7.4 | 0.653 |
| walker |  | 8911 | 386 | crate-doc tail at src/lib.rs:46 |  |  | 0.653 |
| ns | 9123 |  | 284 | Roster: every test in tests/macros.rs | 8.1 |  | 0.641 |
| walker |  | 9276 | 365 | crate-doc tail at src/lib.rs:85 |  |  | 0.641 |
| walker |  | 9286 | 10 | CHANGELOG.md section #0 |  |  | 0.641 |
| ns | 9299 |  | 176 | tests/integration.rs: the capturing test logger and what it pins | 8.2 |  | 0.634 |
| ns | 9465 |  | 166 | src/serde.rs: what is serialised, and how | 8.3 |  | 0.628 |
| walker |  | 9582 | 296 | crate-doc tail at src/kv/mod.rs:41 |  |  | 0.638 |
| ns | 9609 |  | 144 | CI: the seven jobs | 8.4 |  | 0.630 |
| ns | 9665 |  | 56 | The companion crate that tests compile-time filtering | 8.5 |  | 0.627 |
| ns | 9737 |  | 72 | Roster: the value benchmarks | 8.6 |  | 0.625 |
| ns | 9846 |  | 109 | The mutually-exclusive feature guards | 8.7 |  | 0.620 |
| ns | 9891 |  | 45 | CHANGELOG: format and latest release | 8.8 |  | 0.618 |
| ns | 9981 |  | 90 | Roster: the top-level sections of the structured-logging RFC | 8.9 |  | 0.614 |
