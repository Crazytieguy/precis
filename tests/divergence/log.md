Score(3000)=0.591 I=0.819 C=0.426 ns_rows≤3K=17/47 (reached=7 partial=2 missing=8)

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 52 | 52 | listing of '.' |  |  | 1.000 |
| ns | 52 |  | 52 | Root directory listing | 1.1 |  | 1.000 |
| walker |  | 73 | 21 | listing of 'src' |  |  | 1.000 |
| ns | 73 |  | 21 | src/ directory listing | 1.2 |  | 1.000 |
| walker |  | 92 | 19 | listing of 'src/kv' |  |  | 1.000 |
| ns | 92 |  | 19 | src/kv/ directory listing | 1.3 |  | 1.000 |
| walker |  | 95 | 3 | listing of '.github' |  |  | 1.000 |
| walker |  | 98 | 3 | listing of '.github/workflows' |  |  | 1.000 |
| walker |  | 101 | 3 | listing of 'benches' |  |  | 1.000 |
| ns | 123 |  | 31 | Secondary directory listings (tests/, benches/, CI, test_max_level_features/, rfcs/) | 1.4 |  | 0.859 |
| walker |  | 153 | 52 | README headline in README.md |  |  | 0.860 |
| walker |  | 160 | 7 | listing of 'tests' |  |  | 0.816 |
| ns | 160 |  | 37 | triagebot.toml + .gitignore + .precis-pin | 1.5 |  | 0.816 |
| walker |  | 168 | 8 | listing of 'test_max_level_features' |  |  | 0.873 |
| walker |  | 259 | 91 | crate-doc lede in src/kv/mod.rs |  |  | 0.873 |
| walker |  | 268 | 9 | listing of 'rfcs' |  |  | 0.909 |
| ns | 294 |  | 134 | Cargo.toml package identity (name/version/license/repo) | 1.6 |  | 0.758 |
| walker |  | 324 | 56 | headings outline in README.md |  |  | 0.758 |
| ns | 449 |  | 155 | Crate-doc lede (lib.rs) | 1.7 |  | 0.674 |
| walker |  | 515 | 191 | [package] in Cargo.toml |  |  | 0.831 |
| ns | 552 |  | 103 | Cargo.toml package identity (MSRV, edition, docs.rs features) | 1.8 |  | 0.787 |
| walker |  | 579 | 64 | README.md section #0 |  |  | 0.787 |
| walker |  | 615 | 36 | manifest config in Cargo.toml |  |  | 0.827 |
| ns | 689 |  | 137 | Cargo.toml [features]: max_level_*/release_max_level_* | 1.9 |  | 0.731 |
| walker |  | 863 | 248 | crate-doc lede in src/lib.rs |  |  | 0.820 |
| walker |  | 889 | 26 | pub-item names surface in src/kv/key.rs |  |  | 0.820 |
| walker |  | 920 | 31 | pub item at src/kv/key.rs:7 |  |  | 0.820 |
| ns | 964 |  | 275 | Cargo.toml [features]: std/kv/kv_std/kv_sval/kv_serde | 1.10 |  | 0.727 |
| walker |  | 1039 | 119 | [dependencies] in Cargo.toml |  |  | 0.731 |
| walker |  | 1066 | 27 | pub-item names surface in src/kv/source.rs |  |  | 0.731 |
| ns | 1081 |  | 117 | Cargo.toml [dependencies] | 1.11 |  | 0.740 |
| walker |  | 1100 | 34 | pub item at src/kv/error.rs:5 |  |  | 0.740 |
| walker |  | 1148 | 48 | pub item at src/kv/source.rs:235 |  |  | 0.740 |
| walker |  | 1379 | 231 | mod/use plumbing in src/lib.rs |  |  | 0.740 |
| walker |  | 1419 | 40 | pub-item names surface in src/kv/value.rs |  |  | 0.740 |
| walker |  | 1450 | 31 | pub item at src/kv/value.rs:11 |  |  | 0.740 |
| walker |  | 1476 | 26 | pub item at src/kv/value.rs:119 |  |  | 0.741 |
| ns | 1494 |  | 413 | Level enum | 2.1 |  | 0.629 |
| walker |  | 1776 | 300 | pub-item names surface in src/lib.rs |  |  | 0.629 |
| walker |  | 1776 | 0 | pub item at src/lib.rs:1375 |  |  | 0.629 |
| walker |  | 1776 | 0 | pub item at src/lib.rs:1396 |  |  | 0.629 |
| walker |  | 1776 | 0 | pub item at src/lib.rs:1529 |  |  | 0.629 |
| walker |  | 1776 | 0 | pub item at src/lib.rs:1581 |  |  | 0.629 |
| walker |  | 1790 | 14 | pub item at src/lib.rs:1351 |  |  | 0.629 |
| ns | 1801 |  | 307 | LevelFilter enum | 2.2 |  | 0.572 |
| walker |  | 1804 | 14 | pub item at src/lib.rs:1478 |  |  | 0.572 |
| walker |  | 1827 | 23 | pub item at src/lib.rs:1549 |  |  | 0.572 |
| walker |  | 1848 | 21 | pub item at src/lib.rs:1420 |  |  | 0.573 |
| walker |  | 1875 | 27 | pub item at src/lib.rs:1003 |  |  | 0.573 |
| walker |  | 1903 | 28 | pub item at src/lib.rs:1566 |  |  | 0.573 |
| walker |  | 1915 | 12 | pub item body at src/lib.rs:1478 body 1479 |  |  | 0.573 |
| ns | 1946 |  | 145 | Level & LevelFilter public method roster | 2.3 |  | 0.549 |
| walker |  | 1955 | 40 | pub item at src/lib.rs:1200 |  |  | 0.549 |
| walker |  | 1971 | 16 | pub item body at src/lib.rs:1420 body 1421 |  |  | 0.549 |
| walker |  | 2022 | 51 | pub item at src/lib.rs:1158 |  |  | 0.549 |
| walker |  | 2042 | 20 | pub item body at src/lib.rs:1351 body 1352 |  |  | 0.549 |
| ns | 2093 |  | 147 | FromStr for Level | 2.4 |  | 0.528 |
| walker |  | 2148 | 106 | pub item at src/lib.rs:842 |  |  | 0.529 |
| walker |  | 2168 | 20 | pub-item doc lede at src/lib.rs:1158 |  |  | 0.529 |
| walker |  | 2342 | 174 | pub item at src/lib.rs:636 |  |  | 0.567 |
| walker |  | 2363 | 21 | pub-item doc lede at src/lib.rs:842 |  |  | 0.567 |
| ns | 2420 |  | 327 | Record struct + KeyValues wrapper | 2.5 |  | 0.536 |
| walker |  | 2455 | 92 | pub item body at src/lib.rs:1375 body 1376 |  |  | 0.536 |
| ns | 2558 |  | 138 | Record accessor method roster | 2.6 |  | 0.519 |
| walker |  | 2758 | 303 | pub item at src/lib.rs:475 |  |  | 0.591 |
| walker |  | 2798 | 40 | pub-item doc lede at src/lib.rs:1581 |  |  | 0.591 |
| walker |  | 2844 | 46 | pub-item doc lede at src/lib.rs:1549 |  |  | 0.591 |
| ns | 3005 |  | 447 | Log trait | 2.7 |  | 0.546 |
| walker |  | 3259 | 415 | pub item at src/lib.rs:1249 |  |  | 0.634 |
| walker |  | 3276 | 17 | pub-item doc lede at src/lib.rs:1249 |  |  | 0.642 |
| ns | 3281 |  | 276 | NopLogger + blanket impl for &T | 2.8 |  | 0.604 |
| walker |  | 3552 | 276 | pub item at src/lib.rs:1611 |  |  | 0.604 |
| ns | 3616 |  | 335 | set_max_level + max_level | 2.9 |  | 0.583 |
| walker |  | 3701 | 149 | pub item body at src/lib.rs:1529 body 1530 |  |  | 0.583 |
| walker |  | 3757 | 56 | pub item at src/kv/key.rs:37 |  |  | 0.583 |
| ns | 3907 |  | 291 | set_boxed_logger + set_logger | 2.10 |  | 0.567 |
| walker |  | 3922 | 165 | pub item body at src/lib.rs:1396 body 1397 |  |  | 0.583 |
| walker |  | 3985 | 63 | pub-item doc lede at src/lib.rs:1566 |  |  | 0.583 |
| walker |  | 4060 | 75 | pub-item doc lede at src/lib.rs:1351 |  |  | 0.600 |
| walker |  | 4160 | 100 | pub-item doc lede at src/lib.rs:475 |  |  | 0.635 |
| ns | 4180 |  | 273 | logger() fn | 2.11 |  | 0.616 |
| walker |  | 4248 | 88 | pub-item doc lede at src/lib.rs:1003 |  |  | 0.616 |
| walker |  | 4262 | 14 | pub-item doc lede at src/kv/error.rs:5 |  |  | 0.616 |
| walker |  | 4357 | 95 | pub-item doc lede at src/lib.rs:1200 |  |  | 0.616 |
| walker |  | 4377 | 20 | pub-item doc lede at src/kv/key.rs:7 |  |  | 0.616 |
| walker |  | 4397 | 20 | pub-item doc lede at src/kv/value.rs:11 |  |  | 0.616 |
| ns | 4438 |  | 258 | macro_rules! definition roster | 3.1 |  | 0.597 |
| walker |  | 4519 | 122 | pub-item doc lede at src/lib.rs:636 |  |  | 0.630 |
| walker |  | 4615 | 96 | pub-item doc lede at src/lib.rs:1529 |  |  | 0.630 |
| walker |  | 4636 | 21 | pub-item doc lede at src/kv/source.rs:235 |  |  | 0.631 |
| walker |  | 4735 | 99 | pub-item doc lede at src/lib.rs:1375 |  |  | 0.631 |
| walker |  | 4754 | 19 | README.md section #6 |  |  | 0.631 |
| ns | 4940 |  | 502 | log! macro | 3.2 | 3.1 | 0.593 |
| walker |  | 5164 | 410 | [features] in Cargo.toml |  |  | 0.670 |
| walker |  | 5552 | 388 | macro_export names across src |  |  | 0.705 |
| ns | 5556 |  | 616 | error! macro | 3.3 | 3.1 | 0.670 |
| walker |  | 5717 | 165 | macro_export names across src/kv |  |  | 0.670 |
| walker |  | 5761 | 44 | macro_export body at src/kv/value.rs:1129 |  |  | 0.670 |
| walker |  | 5805 | 44 | macro_export body at src/kv/value.rs:1139 |  |  | 0.670 |
| walker |  | 5850 | 45 | macro_export body at src/kv/value.rs:1149 |  |  | 0.670 |
| ns | 5891 |  | 335 | log_enabled! macro | 3.4 | 3.1 | 0.652 |
| walker |  | 5895 | 45 | macro_export body at src/kv/value.rs:1159 |  |  | 0.652 |
| walker |  | 5940 | 45 | macro_export body at src/kv/value.rs:1169 |  |  | 0.652 |
| walker |  | 6051 | 111 | pub-item doc lede at src/lib.rs:1420 |  |  | 0.666 |
| walker |  | 6178 | 127 | pub-item doc lede at src/lib.rs:1478 |  |  | 0.666 |
| walker |  | 6261 | 83 | README.md section #1 |  |  | 0.666 |
| ns | 6414 |  | 523 | __log_value! kv capture-modifier dispatch | 3.5 | 3.1 | 0.632 |
| walker |  | 6466 | 205 | mod/use plumbing in src/kv/mod.rs |  |  | 0.634 |
| ns | 6839 |  | 425 | GlobalLogger + log/enabled/loc public fns | 4.1 |  | 0.604 |
| ns | 7120 |  | 281 | kv/mod.rs doc lede | 5.1 |  | 0.594 |
| ns | 7245 |  | 125 | kv/mod.rs capturing-modifier list | 5.2 |  | 0.588 |
| ns | 7450 |  | 205 | kv/mod.rs module decls + re-exports | 5.3 |  | 0.600 |
| ns | 7823 |  | 373 | kv/key.rs: ToKey trait + Key struct | 5.4 |  | 0.585 |
| ns | 8056 |  | 233 | kv/error.rs: Error struct, Inner enum, msg() constructor | 5.5 |  | 0.573 |
| ns | 8102 |  | 46 | kv/source.rs: Source trait method roster | 5.6 |  | 0.572 |
| ns | 8186 |  | 84 | kv/source.rs: VisitSource trait | 5.7 |  | 0.575 |
| ns | 8289 |  | 103 | kv/value.rs: ToValue trait + Value struct | 5.8 |  | 0.580 |
| ns | 8430 |  | 141 | kv/value.rs: dependency-free Inner enum (data model) | 5.9 |  | 0.573 |
| walker |  | 8482 | 2016 | impl method sigs in src/lib.rs |  |  | 0.616 |
| walker |  | 8482 | 0 | impl method at src/lib.rs:427 |  |  | 0.616 |
| walker |  | 8482 | 0 | impl method at src/lib.rs:431 |  |  | 0.616 |
| walker |  | 8482 | 0 | impl method at src/lib.rs:435 |  |  | 0.616 |
| walker |  | 8482 | 0 | impl method at src/lib.rs:503 |  |  | 0.616 |
| walker |  | 8482 | 0 | impl method at src/lib.rs:510 |  |  | 0.616 |
| walker |  | 8482 | 0 | impl method at src/lib.rs:517 |  |  | 0.616 |
| walker |  | 8482 | 0 | impl method at src/lib.rs:529 |  |  | 0.616 |
| walker |  | 8482 | 0 | impl method at src/lib.rs:535 |  |  | 0.616 |
| walker |  | 8482 | 0 | impl method at src/lib.rs:548 |  |  | 0.616 |
| walker |  | 8482 | 0 | impl method at src/lib.rs:554 |  |  | 0.616 |
| walker |  | 8482 | 0 | impl method at src/lib.rs:561 |  |  | 0.616 |
| walker |  | 8482 | 0 | impl method at src/lib.rs:579 |  |  | 0.616 |
| walker |  | 8482 | 0 | impl method at src/lib.rs:599 |  |  | 0.616 |
| walker |  | 8482 | 0 | impl method at src/lib.rs:620 |  |  | 0.616 |
| walker |  | 8482 | 0 | impl method at src/lib.rs:653 |  |  | 0.616 |
| walker |  | 8482 | 0 | impl method at src/lib.rs:660 |  |  | 0.616 |
| walker |  | 8482 | 0 | impl method at src/lib.rs:667 |  |  | 0.616 |
| walker |  | 8482 | 0 | impl method at src/lib.rs:679 |  |  | 0.616 |
| walker |  | 8482 | 0 | impl method at src/lib.rs:685 |  |  | 0.616 |
| walker |  | 8482 | 0 | impl method at src/lib.rs:699 |  |  | 0.616 |
| walker |  | 8482 | 0 | impl method at src/lib.rs:707 |  |  | 0.616 |
| walker |  | 8482 | 0 | impl method at src/lib.rs:714 |  |  | 0.616 |
| walker |  | 8482 | 0 | impl method at src/lib.rs:732 |  |  | 0.616 |
| walker |  | 8482 | 0 | impl method at src/lib.rs:752 |  |  | 0.616 |
| walker |  | 8482 | 0 | impl method at src/lib.rs:774 |  |  | 0.616 |
| walker |  | 8482 | 0 | impl method at src/lib.rs:788 |  |  | 0.616 |
| walker |  | 8482 | 0 | impl method at src/lib.rs:862 |  |  | 0.616 |
| walker |  | 8482 | 0 | impl method at src/lib.rs:872 |  |  | 0.616 |
| walker |  | 8482 | 0 | impl method at src/lib.rs:878 |  |  | 0.616 |
| walker |  | 8482 | 0 | impl method at src/lib.rs:884 |  |  | 0.616 |
| walker |  | 8482 | 0 | impl method at src/lib.rs:890 |  |  | 0.616 |
| walker |  | 8482 | 0 | impl method at src/lib.rs:896 |  |  | 0.616 |
| walker |  | 8482 | 0 | impl method at src/lib.rs:902 |  |  | 0.616 |
| walker |  | 8482 | 0 | impl method at src/lib.rs:908 |  |  | 0.616 |
| walker |  | 8482 | 0 | impl method at src/lib.rs:917 |  |  | 0.616 |
| walker |  | 8482 | 0 | impl method at src/lib.rs:923 |  |  | 0.616 |
| walker |  | 8482 | 0 | impl method at src/lib.rs:932 |  |  | 0.616 |
| walker |  | 8482 | 0 | impl method at src/lib.rs:1021 |  |  | 0.616 |
| walker |  | 8482 | 0 | impl method at src/lib.rs:1037 |  |  | 0.616 |
| walker |  | 8482 | 0 | impl method at src/lib.rs:1044 |  |  | 0.616 |
| walker |  | 8482 | 0 | impl method at src/lib.rs:1051 |  |  | 0.616 |
| walker |  | 8482 | 0 | impl method at src/lib.rs:1058 |  |  | 0.616 |
| walker |  | 8482 | 0 | impl method at src/lib.rs:1065 |  |  | 0.616 |
| walker |  | 8482 | 0 | impl method at src/lib.rs:1072 |  |  | 0.616 |
| walker |  | 8482 | 0 | impl method at src/lib.rs:1079 |  |  | 0.616 |
| walker |  | 8482 | 0 | impl method at src/lib.rs:1086 |  |  | 0.616 |
| walker |  | 8482 | 0 | impl method at src/lib.rs:1093 |  |  | 0.616 |
| walker |  | 8482 | 0 | impl method at src/lib.rs:1108 |  |  | 0.616 |
| walker |  | 8482 | 0 | impl method at src/lib.rs:1114 |  |  | 0.616 |
| walker |  | 8482 | 0 | impl method at src/lib.rs:1166 |  |  | 0.616 |
| walker |  | 8482 | 0 | impl method at src/lib.rs:1172 |  |  | 0.616 |
| walker |  | 8482 | 0 | impl method at src/lib.rs:1178 |  |  | 0.616 |
| walker |  | 8482 | 0 | impl method at src/lib.rs:1212 |  |  | 0.616 |
| walker |  | 8482 | 0 | impl method at src/lib.rs:1223 |  |  | 0.616 |
| walker |  | 8482 | 0 | impl method at src/lib.rs:1230 |  |  | 0.616 |
| walker |  | 8482 | 0 | impl method at src/lib.rs:1237 |  |  | 0.616 |
| walker |  | 8482 | 0 | impl method at src/lib.rs:1243 |  |  | 0.616 |
| walker |  | 8482 | 0 | impl method at src/lib.rs:1286 |  |  | 0.616 |
| walker |  | 8482 | 0 | impl method at src/lib.rs:1290 |  |  | 0.616 |
| walker |  | 8482 | 0 | impl method at src/lib.rs:1291 |  |  | 0.616 |
| walker |  | 8482 | 0 | impl method at src/lib.rs:1298 |  |  | 0.616 |
| walker |  | 8482 | 0 | impl method at src/lib.rs:1302 |  |  | 0.616 |
| walker |  | 8482 | 0 | impl method at src/lib.rs:1305 |  |  | 0.616 |
| walker |  | 8482 | 0 | impl method at src/lib.rs:1315 |  |  | 0.616 |
| walker |  | 8482 | 0 | impl method at src/lib.rs:1319 |  |  | 0.616 |
| walker |  | 8482 | 0 | impl method at src/lib.rs:1322 |  |  | 0.616 |
| walker |  | 8482 | 0 | impl method at src/lib.rs:1332 |  |  | 0.616 |
| walker |  | 8482 | 0 | impl method at src/lib.rs:1336 |  |  | 0.616 |
| walker |  | 8482 | 0 | impl method at src/lib.rs:1339 |  |  | 0.616 |
| walker |  | 8482 | 0 | impl method at src/lib.rs:1552 |  |  | 0.616 |
| walker |  | 8482 | 0 | impl method at src/lib.rs:1569 |  |  | 0.616 |
| walker |  | 8490 | 8 | impl method body at src/lib.rs:932 body 933 |  |  | 0.616 |
| walker |  | 8498 | 8 | impl method body at src/lib.rs:1286 body 1287 |  |  | 0.617 |
| walker |  | 8507 | 9 | impl method body at src/lib.rs:548 body 549 |  |  | 0.617 |
| walker |  | 8516 | 9 | impl method body at src/lib.rs:653 body 654 |  |  | 0.617 |
| walker |  | 8525 | 9 | impl method body at src/lib.rs:878 body 879 |  |  | 0.617 |
| walker |  | 8534 | 9 | impl method body at src/lib.rs:884 body 885 |  |  | 0.617 |
| walker |  | 8543 | 9 | impl method body at src/lib.rs:890 body 891 |  |  | 0.617 |
| walker |  | 8552 | 9 | impl method body at src/lib.rs:896 body 897 |  |  | 0.617 |
| walker |  | 8561 | 9 | impl method body at src/lib.rs:1172 body 1173 |  |  | 0.617 |
| walker |  | 8570 | 9 | impl method body at src/lib.rs:1178 body 1179 |  |  | 0.617 |
| walker |  | 8580 | 10 | impl method body at src/lib.rs:699 body 700 |  |  | 0.617 |
| walker |  | 8590 | 10 | impl method body at src/lib.rs:872 body 873 |  |  | 0.617 |
| walker |  | 8600 | 10 | impl method body at src/lib.rs:1108 body 1109 |  |  | 0.617 |
| walker |  | 8610 | 10 | impl method body at src/lib.rs:1114 body 1115 |  |  | 0.617 |
| walker |  | 8620 | 10 | impl method body at src/lib.rs:1237 body 1238 |  |  | 0.617 |
| walker |  | 8630 | 10 | impl method body at src/lib.rs:1243 body 1244 |  |  | 0.617 |
| walker |  | 8641 | 11 | impl method body at src/lib.rs:529 body 530 |  |  | 0.617 |
| walker |  | 8652 | 11 | impl method body at src/lib.rs:679 body 680 |  |  | 0.617 |
| walker |  | 8663 | 11 | impl method body at src/lib.rs:1166 body 1167 |  |  | 0.617 |
| walker |  | 8674 | 11 | impl method body at src/lib.rs:1305 body 1306 |  |  | 0.619 |
| walker |  | 8686 | 12 | impl method body at src/lib.rs:1298 body 1299 |  |  | 0.621 |
| walker |  | 8698 | 12 | impl method body at src/lib.rs:1302 body 1303 |  |  | 0.623 |
| walker |  | 8710 | 12 | impl method body at src/lib.rs:1322 body 1323 |  |  | 0.623 |
| ns | 8714 |  | 284 | tests/macros.rs test-fn roster | 6.1 |  | 0.611 |
| walker |  | 8722 | 12 | impl method body at src/lib.rs:1339 body 1340 |  |  | 0.611 |
| walker |  | 8735 | 13 | impl method body at src/lib.rs:561 body 562 |  |  | 0.611 |
| walker |  | 8748 | 13 | impl method body at src/lib.rs:714 body 715 |  |  | 0.611 |
| walker |  | 8761 | 13 | impl method body at src/lib.rs:1315 body 1316 |  |  | 0.611 |
| walker |  | 8774 | 13 | impl method body at src/lib.rs:1319 body 1320 |  |  | 0.611 |
| walker |  | 8787 | 13 | impl method body at src/lib.rs:1332 body 1333 |  |  | 0.611 |
| walker |  | 8800 | 13 | impl method body at src/lib.rs:1336 body 1337 |  |  | 0.611 |
| walker |  | 8814 | 14 | impl method at src/lib.rs:939 |  |  | 0.611 |
| walker |  | 8825 | 11 | impl method body at src/lib.rs:939 body 940 |  |  | 0.611 |
| walker |  | 8839 | 14 | impl method at src/lib.rs:946 |  |  | 0.611 |
| walker |  | 8854 | 15 | impl method at src/lib.rs:1101 |  |  | 0.611 |
| walker |  | 8868 | 14 | impl method body at src/lib.rs:917 body 918 |  |  | 0.611 |
| walker |  | 8882 | 14 | impl method body at src/lib.rs:1569 body 1570 |  |  | 0.611 |
| walker |  | 8897 | 15 | impl method body at src/lib.rs:503 body 504 |  |  | 0.611 |
| walker |  | 8912 | 15 | impl method body at src/lib.rs:707 body 708 |  |  | 0.611 |
| walker |  | 8927 | 15 | impl method body at src/lib.rs:902 body 903 |  |  | 0.611 |
| ns | 8941 |  | 227 | tests/macros.rs: kv_common_value_types body | 6.2 | 6.1 | 0.601 |
| walker |  | 8942 | 15 | impl method body at src/lib.rs:1552 body 1553 |  |  | 0.601 |
| walker |  | 8960 | 18 | impl method body at src/lib.rs:510 body 511 |  |  | 0.601 |
| walker |  | 8978 | 18 | impl method body at src/lib.rs:554 body 555 |  |  | 0.601 |
| walker |  | 8996 | 18 | impl method body at src/lib.rs:660 body 661 |  |  | 0.601 |
| walker |  | 9016 | 20 | impl method body at src/lib.rs:1037 body 1038 |  |  | 0.601 |
| ns | 9028 |  | 87 | CI workflow job-name roster | 7.1 |  | 0.598 |
| walker |  | 9036 | 20 | impl method body at src/lib.rs:1044 body 1045 |  |  | 0.598 |
| walker |  | 9056 | 20 | impl method body at src/lib.rs:1093 body 1094 |  |  | 0.598 |
| walker |  | 9076 | 20 | impl method body at src/lib.rs:1223 body 1224 |  |  | 0.598 |
| walker |  | 9096 | 20 | impl method body at src/lib.rs:1230 body 1231 |  |  | 0.598 |
| walker |  | 9117 | 21 | impl method body at src/lib.rs:1051 body 1052 |  |  | 0.598 |
| walker |  | 9138 | 21 | impl method body at src/lib.rs:1058 body 1059 |  |  | 0.598 |
| ns | 9147 |  | 119 | test_max_level_features/Cargo.toml | 7.2 |  | 0.592 |
| walker |  | 9161 | 23 | impl method body at src/lib.rs:579 body 580 |  |  | 0.592 |
| walker |  | 9184 | 23 | impl method body at src/lib.rs:732 body 733 |  |  | 0.592 |
| walker |  | 9208 | 24 | impl method body at src/lib.rs:1101 body 1102 |  |  | 0.592 |
| walker |  | 9235 | 27 | impl method body at src/lib.rs:1086 body 1087 |  |  | 0.592 |
| walker |  | 9263 | 28 | impl method body at src/lib.rs:1072 body 1073 |  |  | 0.592 |
| walker |  | 9291 | 28 | impl method body at src/lib.rs:1079 body 1080 |  |  | 0.592 |
| walker |  | 9320 | 29 | impl method body at src/lib.rs:1065 body 1066 |  |  | 0.592 |
| walker |  | 9353 | 33 | impl method body at src/lib.rs:599 body 600 |  |  | 0.592 |
| walker |  | 9386 | 33 | impl method body at src/lib.rs:752 body 753 |  |  | 0.592 |
| ns | 9394 |  | 247 | README.md lede | 8.1 |  | 0.589 |
| walker |  | 9422 | 36 | impl method body at src/lib.rs:620 body 621 |  |  | 0.589 |
| walker |  | 9458 | 36 | impl method body at src/lib.rs:774 body 775 |  |  | 0.589 |
| walker |  | 9500 | 42 | impl method body at src/lib.rs:862 body 863 |  |  | 0.593 |
| walker |  | 9542 | 42 | impl method body at src/lib.rs:923 body 924 |  |  | 0.593 |
| walker |  | 9585 | 43 | impl method body at src/lib.rs:908 body 909 |  |  | 0.593 |
| ns | 9633 |  | 239 | CHANGELOG.md: Unreleased + 0.4.29 | 8.2 |  | 0.587 |
| walker |  | 9639 | 54 | impl method body at src/lib.rs:1212 body 1213 |  |  | 0.587 |
| walker |  | 9648 | 9 | impl method body at src/lib.rs:431 body 432 |  |  | 0.587 |
| walker |  | 9658 | 10 | impl method body at src/lib.rs:435 body 436 |  |  | 0.587 |
| ns | 9677 |  | 44 | CHANGELOG.md: sampled older version headings | 8.3 |  | 0.586 |
| walker |  | 9747 | 89 | impl method body at src/lib.rs:517 body 518 |  |  | 0.595 |
| ns | 9767 |  | 90 | rfcs/0296-structured-logging.md heading roster | 8.4 |  | 0.592 |
| walker |  | 9837 | 90 | impl method body at src/lib.rs:667 body 668 |  |  | 0.592 |
| ns | 9924 |  | 157 | RFC 0296: Summary | 8.5 | 8.4 | 0.588 |
| walker |  | 9961 | 124 | pub-item doc lede at src/lib.rs:1611 |  |  | 0.588 |
| ns | 9997 |  | 73 | src/serde.rs: serde impl roster | 9.1 |  | 0.586 |
