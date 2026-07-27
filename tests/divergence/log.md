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
| walker |  | 1448 | 69 | macro_export names across src/kv |  |  | 0.740 |
| walker |  | 1486 | 38 | pub-item names surface in src/kv/value.rs |  |  | 0.740 |
| ns | 1494 |  | 413 | Level enum | 2.1 |  | 0.628 |
| walker |  | 1517 | 31 | pub item at src/kv/value.rs:11 |  |  | 0.629 |
| walker |  | 1543 | 26 | pub item at src/kv/value.rs:119 |  |  | 0.629 |
| ns | 1801 |  | 307 | LevelFilter enum | 2.2 |  | 0.572 |
| walker |  | 1843 | 300 | pub-item names surface in src/lib.rs |  |  | 0.572 |
| walker |  | 1843 | 0 | pub item at src/lib.rs:1375 |  |  | 0.572 |
| walker |  | 1843 | 0 | pub item at src/lib.rs:1396 |  |  | 0.572 |
| walker |  | 1843 | 0 | pub item at src/lib.rs:1529 |  |  | 0.572 |
| walker |  | 1843 | 0 | pub item at src/lib.rs:1581 |  |  | 0.572 |
| walker |  | 1857 | 14 | pub item at src/lib.rs:1351 |  |  | 0.572 |
| walker |  | 1871 | 14 | pub item at src/lib.rs:1478 |  |  | 0.572 |
| walker |  | 1894 | 23 | pub item at src/lib.rs:1549 |  |  | 0.572 |
| walker |  | 1915 | 21 | pub item at src/lib.rs:1420 |  |  | 0.573 |
| walker |  | 1942 | 27 | pub item at src/lib.rs:1003 |  |  | 0.573 |
| ns | 1946 |  | 145 | Level & LevelFilter public method roster | 2.3 |  | 0.549 |
| walker |  | 1970 | 28 | pub item at src/lib.rs:1566 |  |  | 0.549 |
| walker |  | 1982 | 12 | pub item body at src/lib.rs:1478 body 1479 |  |  | 0.549 |
| walker |  | 2022 | 40 | pub item at src/lib.rs:1200 |  |  | 0.549 |
| walker |  | 2038 | 16 | pub item body at src/lib.rs:1420 body 1421 |  |  | 0.549 |
| walker |  | 2089 | 51 | pub item at src/lib.rs:1158 |  |  | 0.549 |
| ns | 2093 |  | 147 | FromStr for Level | 2.4 |  | 0.528 |
| walker |  | 2109 | 20 | pub item body at src/lib.rs:1351 body 1352 |  |  | 0.528 |
| walker |  | 2215 | 106 | pub item at src/lib.rs:842 |  |  | 0.529 |
| walker |  | 2235 | 20 | pub-item doc lede at src/lib.rs:1158 |  |  | 0.529 |
| walker |  | 2409 | 174 | pub item at src/lib.rs:636 |  |  | 0.567 |
| ns | 2420 |  | 327 | Record struct + KeyValues wrapper | 2.5 |  | 0.536 |
| walker |  | 2430 | 21 | pub-item doc lede at src/lib.rs:842 |  |  | 0.536 |
| walker |  | 2522 | 92 | pub item body at src/lib.rs:1375 body 1376 |  |  | 0.536 |
| ns | 2558 |  | 138 | Record accessor method roster | 2.6 |  | 0.519 |
| walker |  | 2825 | 303 | pub item at src/lib.rs:475 |  |  | 0.591 |
| walker |  | 2865 | 40 | pub-item doc lede at src/lib.rs:1581 |  |  | 0.591 |
| walker |  | 2911 | 46 | pub-item doc lede at src/lib.rs:1549 |  |  | 0.591 |
| ns | 3005 |  | 447 | Log trait | 2.7 |  | 0.546 |
| ns | 3281 |  | 276 | NopLogger + blanket impl for &T | 2.8 |  | 0.514 |
| walker |  | 3326 | 415 | pub item at src/lib.rs:1249 |  |  | 0.597 |
| walker |  | 3343 | 17 | pub-item doc lede at src/lib.rs:1249 |  |  | 0.604 |
| ns | 3616 |  | 335 | set_max_level + max_level | 2.9 |  | 0.583 |
| walker |  | 3619 | 276 | pub item at src/lib.rs:1611 |  |  | 0.583 |
| walker |  | 3768 | 149 | pub item body at src/lib.rs:1529 body 1530 |  |  | 0.583 |
| walker |  | 3824 | 56 | pub item at src/kv/key.rs:37 |  |  | 0.583 |
| ns | 3907 |  | 291 | set_boxed_logger + set_logger | 2.10 |  | 0.567 |
| walker |  | 3989 | 165 | pub item body at src/lib.rs:1396 body 1397 |  |  | 0.583 |
| walker |  | 4052 | 63 | pub-item doc lede at src/lib.rs:1566 |  |  | 0.583 |
| ns | 4180 |  | 273 | logger() fn | 2.11 |  | 0.566 |
| walker |  | 4310 | 258 | macro_export names across src |  |  | 0.569 |
| walker |  | 4385 | 75 | pub-item doc lede at src/lib.rs:1351 |  |  | 0.586 |
| ns | 4438 |  | 258 | macro_rules! definition roster | 3.1 |  | 0.604 |
| walker |  | 4485 | 100 | pub-item doc lede at src/lib.rs:475 |  |  | 0.636 |
| walker |  | 4573 | 88 | pub-item doc lede at src/lib.rs:1003 |  |  | 0.636 |
| walker |  | 4587 | 14 | pub-item doc lede at src/kv/error.rs:5 |  |  | 0.636 |
| walker |  | 4682 | 95 | pub-item doc lede at src/lib.rs:1200 |  |  | 0.636 |
| walker |  | 4702 | 20 | pub-item doc lede at src/kv/key.rs:7 |  |  | 0.636 |
| walker |  | 4722 | 20 | pub-item doc lede at src/kv/value.rs:11 |  |  | 0.636 |
| walker |  | 4844 | 122 | pub-item doc lede at src/lib.rs:636 |  |  | 0.669 |
| walker |  | 4940 | 96 | pub-item doc lede at src/lib.rs:1529 |  |  | 0.629 |
| ns | 4940 |  | 502 | log! macro | 3.2 | 3.1 | 0.629 |
| walker |  | 4984 | 44 | macro_export body at src/kv/value.rs:1129 |  |  | 0.629 |
| walker |  | 5028 | 44 | macro_export body at src/kv/value.rs:1139 |  |  | 0.629 |
| walker |  | 5049 | 21 | pub-item doc lede at src/kv/source.rs:235 |  |  | 0.629 |
| walker |  | 5094 | 45 | macro_export body at src/kv/value.rs:1149 |  |  | 0.629 |
| walker |  | 5139 | 45 | macro_export body at src/kv/value.rs:1159 |  |  | 0.629 |
| walker |  | 5184 | 45 | macro_export body at src/kv/value.rs:1169 |  |  | 0.629 |
| walker |  | 5283 | 99 | pub-item doc lede at src/lib.rs:1375 |  |  | 0.629 |
| walker |  | 5302 | 19 | README.md section #6 |  |  | 0.629 |
| ns | 5556 |  | 616 | error! macro | 3.3 | 3.1 | 0.598 |
| walker |  | 5712 | 410 | [features] in Cargo.toml |  |  | 0.670 |
| walker |  | 5823 | 111 | pub-item doc lede at src/lib.rs:1420 |  |  | 0.684 |
| ns | 5891 |  | 335 | log_enabled! macro | 3.4 | 3.1 | 0.666 |
| walker |  | 5950 | 127 | pub-item doc lede at src/lib.rs:1478 |  |  | 0.666 |
| walker |  | 6033 | 83 | README.md section #1 |  |  | 0.666 |
| walker |  | 6238 | 205 | mod/use plumbing in src/kv/mod.rs |  |  | 0.668 |
| ns | 6414 |  | 523 | __log_value! kv capture-modifier dispatch | 3.5 | 3.1 | 0.634 |
| ns | 6839 |  | 425 | GlobalLogger + log/enabled/loc public fns | 4.1 |  | 0.604 |
| ns | 7120 |  | 281 | kv/mod.rs doc lede | 5.1 |  | 0.594 |
| ns | 7245 |  | 125 | kv/mod.rs capturing-modifier list | 5.2 |  | 0.588 |
| ns | 7450 |  | 205 | kv/mod.rs module decls + re-exports | 5.3 |  | 0.600 |
| ns | 7823 |  | 373 | kv/key.rs: ToKey trait + Key struct | 5.4 |  | 0.585 |
| ns | 8056 |  | 233 | kv/error.rs: Error struct, Inner enum, msg() constructor | 5.5 |  | 0.573 |
| ns | 8102 |  | 46 | kv/source.rs: Source trait method roster | 5.6 |  | 0.572 |
| ns | 8186 |  | 84 | kv/source.rs: VisitSource trait | 5.7 |  | 0.575 |
| walker |  | 8254 | 2016 | impl method sigs in src/lib.rs |  |  | 0.619 |
| walker |  | 8254 | 0 | impl method at src/lib.rs:427 |  |  | 0.619 |
| walker |  | 8254 | 0 | impl method at src/lib.rs:431 |  |  | 0.619 |
| walker |  | 8254 | 0 | impl method at src/lib.rs:435 |  |  | 0.619 |
| walker |  | 8254 | 0 | impl method at src/lib.rs:503 |  |  | 0.619 |
| walker |  | 8254 | 0 | impl method at src/lib.rs:510 |  |  | 0.619 |
| walker |  | 8254 | 0 | impl method at src/lib.rs:517 |  |  | 0.619 |
| walker |  | 8254 | 0 | impl method at src/lib.rs:529 |  |  | 0.619 |
| walker |  | 8254 | 0 | impl method at src/lib.rs:535 |  |  | 0.619 |
| walker |  | 8254 | 0 | impl method at src/lib.rs:548 |  |  | 0.619 |
| walker |  | 8254 | 0 | impl method at src/lib.rs:554 |  |  | 0.619 |
| walker |  | 8254 | 0 | impl method at src/lib.rs:561 |  |  | 0.619 |
| walker |  | 8254 | 0 | impl method at src/lib.rs:579 |  |  | 0.619 |
| walker |  | 8254 | 0 | impl method at src/lib.rs:599 |  |  | 0.619 |
| walker |  | 8254 | 0 | impl method at src/lib.rs:620 |  |  | 0.619 |
| walker |  | 8254 | 0 | impl method at src/lib.rs:653 |  |  | 0.619 |
| walker |  | 8254 | 0 | impl method at src/lib.rs:660 |  |  | 0.619 |
| walker |  | 8254 | 0 | impl method at src/lib.rs:667 |  |  | 0.619 |
| walker |  | 8254 | 0 | impl method at src/lib.rs:679 |  |  | 0.619 |
| walker |  | 8254 | 0 | impl method at src/lib.rs:685 |  |  | 0.619 |
| walker |  | 8254 | 0 | impl method at src/lib.rs:699 |  |  | 0.619 |
| walker |  | 8254 | 0 | impl method at src/lib.rs:707 |  |  | 0.619 |
| walker |  | 8254 | 0 | impl method at src/lib.rs:714 |  |  | 0.619 |
| walker |  | 8254 | 0 | impl method at src/lib.rs:732 |  |  | 0.619 |
| walker |  | 8254 | 0 | impl method at src/lib.rs:752 |  |  | 0.619 |
| walker |  | 8254 | 0 | impl method at src/lib.rs:774 |  |  | 0.619 |
| walker |  | 8254 | 0 | impl method at src/lib.rs:788 |  |  | 0.619 |
| walker |  | 8254 | 0 | impl method at src/lib.rs:862 |  |  | 0.619 |
| walker |  | 8254 | 0 | impl method at src/lib.rs:872 |  |  | 0.619 |
| walker |  | 8254 | 0 | impl method at src/lib.rs:878 |  |  | 0.619 |
| walker |  | 8254 | 0 | impl method at src/lib.rs:884 |  |  | 0.619 |
| walker |  | 8254 | 0 | impl method at src/lib.rs:890 |  |  | 0.619 |
| walker |  | 8254 | 0 | impl method at src/lib.rs:896 |  |  | 0.619 |
| walker |  | 8254 | 0 | impl method at src/lib.rs:902 |  |  | 0.619 |
| walker |  | 8254 | 0 | impl method at src/lib.rs:908 |  |  | 0.619 |
| walker |  | 8254 | 0 | impl method at src/lib.rs:917 |  |  | 0.619 |
| walker |  | 8254 | 0 | impl method at src/lib.rs:923 |  |  | 0.619 |
| walker |  | 8254 | 0 | impl method at src/lib.rs:932 |  |  | 0.619 |
| walker |  | 8254 | 0 | impl method at src/lib.rs:1021 |  |  | 0.619 |
| walker |  | 8254 | 0 | impl method at src/lib.rs:1037 |  |  | 0.619 |
| walker |  | 8254 | 0 | impl method at src/lib.rs:1044 |  |  | 0.619 |
| walker |  | 8254 | 0 | impl method at src/lib.rs:1051 |  |  | 0.619 |
| walker |  | 8254 | 0 | impl method at src/lib.rs:1058 |  |  | 0.619 |
| walker |  | 8254 | 0 | impl method at src/lib.rs:1065 |  |  | 0.619 |
| walker |  | 8254 | 0 | impl method at src/lib.rs:1072 |  |  | 0.619 |
| walker |  | 8254 | 0 | impl method at src/lib.rs:1079 |  |  | 0.619 |
| walker |  | 8254 | 0 | impl method at src/lib.rs:1086 |  |  | 0.619 |
| walker |  | 8254 | 0 | impl method at src/lib.rs:1093 |  |  | 0.619 |
| walker |  | 8254 | 0 | impl method at src/lib.rs:1108 |  |  | 0.619 |
| walker |  | 8254 | 0 | impl method at src/lib.rs:1114 |  |  | 0.619 |
| walker |  | 8254 | 0 | impl method at src/lib.rs:1166 |  |  | 0.619 |
| walker |  | 8254 | 0 | impl method at src/lib.rs:1172 |  |  | 0.619 |
| walker |  | 8254 | 0 | impl method at src/lib.rs:1178 |  |  | 0.619 |
| walker |  | 8254 | 0 | impl method at src/lib.rs:1212 |  |  | 0.619 |
| walker |  | 8254 | 0 | impl method at src/lib.rs:1223 |  |  | 0.619 |
| walker |  | 8254 | 0 | impl method at src/lib.rs:1230 |  |  | 0.619 |
| walker |  | 8254 | 0 | impl method at src/lib.rs:1237 |  |  | 0.619 |
| walker |  | 8254 | 0 | impl method at src/lib.rs:1243 |  |  | 0.619 |
| walker |  | 8254 | 0 | impl method at src/lib.rs:1286 |  |  | 0.619 |
| walker |  | 8254 | 0 | impl method at src/lib.rs:1290 |  |  | 0.619 |
| walker |  | 8254 | 0 | impl method at src/lib.rs:1291 |  |  | 0.619 |
| walker |  | 8254 | 0 | impl method at src/lib.rs:1298 |  |  | 0.619 |
| walker |  | 8254 | 0 | impl method at src/lib.rs:1302 |  |  | 0.619 |
| walker |  | 8254 | 0 | impl method at src/lib.rs:1305 |  |  | 0.619 |
| walker |  | 8254 | 0 | impl method at src/lib.rs:1315 |  |  | 0.619 |
| walker |  | 8254 | 0 | impl method at src/lib.rs:1319 |  |  | 0.619 |
| walker |  | 8254 | 0 | impl method at src/lib.rs:1322 |  |  | 0.619 |
| walker |  | 8254 | 0 | impl method at src/lib.rs:1332 |  |  | 0.619 |
| walker |  | 8254 | 0 | impl method at src/lib.rs:1336 |  |  | 0.619 |
| walker |  | 8254 | 0 | impl method at src/lib.rs:1339 |  |  | 0.619 |
| walker |  | 8254 | 0 | impl method at src/lib.rs:1552 |  |  | 0.619 |
| walker |  | 8254 | 0 | impl method at src/lib.rs:1569 |  |  | 0.619 |
| walker |  | 8262 | 8 | impl method body at src/lib.rs:932 body 933 |  |  | 0.619 |
| walker |  | 8270 | 8 | impl method body at src/lib.rs:1286 body 1287 |  |  | 0.620 |
| walker |  | 8279 | 9 | impl method body at src/lib.rs:548 body 549 |  |  | 0.620 |
| walker |  | 8288 | 9 | impl method body at src/lib.rs:653 body 654 |  |  | 0.620 |
| ns | 8289 |  | 103 | kv/value.rs: ToValue trait + Value struct | 5.8 |  | 0.624 |
| walker |  | 8297 | 9 | impl method body at src/lib.rs:878 body 879 |  |  | 0.624 |
| walker |  | 8306 | 9 | impl method body at src/lib.rs:884 body 885 |  |  | 0.624 |
| walker |  | 8315 | 9 | impl method body at src/lib.rs:890 body 891 |  |  | 0.624 |
| walker |  | 8324 | 9 | impl method body at src/lib.rs:896 body 897 |  |  | 0.624 |
| walker |  | 8333 | 9 | impl method body at src/lib.rs:1172 body 1173 |  |  | 0.624 |
| walker |  | 8342 | 9 | impl method body at src/lib.rs:1178 body 1179 |  |  | 0.624 |
| walker |  | 8352 | 10 | impl method body at src/lib.rs:699 body 700 |  |  | 0.624 |
| walker |  | 8362 | 10 | impl method body at src/lib.rs:872 body 873 |  |  | 0.624 |
| walker |  | 8372 | 10 | impl method body at src/lib.rs:1108 body 1109 |  |  | 0.624 |
| walker |  | 8382 | 10 | impl method body at src/lib.rs:1114 body 1115 |  |  | 0.624 |
| walker |  | 8392 | 10 | impl method body at src/lib.rs:1237 body 1238 |  |  | 0.624 |
| walker |  | 8402 | 10 | impl method body at src/lib.rs:1243 body 1244 |  |  | 0.624 |
| walker |  | 8413 | 11 | impl method body at src/lib.rs:529 body 530 |  |  | 0.624 |
| walker |  | 8424 | 11 | impl method body at src/lib.rs:679 body 680 |  |  | 0.624 |
| ns | 8430 |  | 141 | kv/value.rs: dependency-free Inner enum (data model) | 5.9 |  | 0.617 |
| walker |  | 8435 | 11 | impl method body at src/lib.rs:1166 body 1167 |  |  | 0.617 |
| walker |  | 8446 | 11 | impl method body at src/lib.rs:1305 body 1306 |  |  | 0.619 |
| walker |  | 8458 | 12 | impl method body at src/lib.rs:1298 body 1299 |  |  | 0.621 |
| walker |  | 8470 | 12 | impl method body at src/lib.rs:1302 body 1303 |  |  | 0.623 |
| walker |  | 8482 | 12 | impl method body at src/lib.rs:1322 body 1323 |  |  | 0.623 |
| walker |  | 8494 | 12 | impl method body at src/lib.rs:1339 body 1340 |  |  | 0.623 |
| walker |  | 8507 | 13 | impl method body at src/lib.rs:561 body 562 |  |  | 0.623 |
| walker |  | 8520 | 13 | impl method body at src/lib.rs:714 body 715 |  |  | 0.623 |
| walker |  | 8533 | 13 | impl method body at src/lib.rs:1315 body 1316 |  |  | 0.623 |
| walker |  | 8546 | 13 | impl method body at src/lib.rs:1319 body 1320 |  |  | 0.623 |
| walker |  | 8559 | 13 | impl method body at src/lib.rs:1332 body 1333 |  |  | 0.623 |
| walker |  | 8572 | 13 | impl method body at src/lib.rs:1336 body 1337 |  |  | 0.623 |
| walker |  | 8586 | 14 | impl method at src/lib.rs:939 |  |  | 0.623 |
| walker |  | 8597 | 11 | impl method body at src/lib.rs:939 body 940 |  |  | 0.623 |
| walker |  | 8611 | 14 | impl method at src/lib.rs:946 |  |  | 0.623 |
| walker |  | 8626 | 15 | impl method at src/lib.rs:1101 |  |  | 0.623 |
| walker |  | 8640 | 14 | impl method body at src/lib.rs:917 body 918 |  |  | 0.623 |
| walker |  | 8654 | 14 | impl method body at src/lib.rs:1569 body 1570 |  |  | 0.623 |
| walker |  | 8669 | 15 | impl method body at src/lib.rs:503 body 504 |  |  | 0.623 |
| walker |  | 8684 | 15 | impl method body at src/lib.rs:707 body 708 |  |  | 0.623 |
| walker |  | 8699 | 15 | impl method body at src/lib.rs:902 body 903 |  |  | 0.623 |
| walker |  | 8714 | 15 | impl method body at src/lib.rs:1552 body 1553 |  |  | 0.611 |
| ns | 8714 |  | 284 | tests/macros.rs test-fn roster | 6.1 |  | 0.611 |
| walker |  | 8732 | 18 | impl method body at src/lib.rs:510 body 511 |  |  | 0.611 |
| walker |  | 8750 | 18 | impl method body at src/lib.rs:554 body 555 |  |  | 0.611 |
| walker |  | 8768 | 18 | impl method body at src/lib.rs:660 body 661 |  |  | 0.611 |
| walker |  | 8788 | 20 | impl method body at src/lib.rs:1037 body 1038 |  |  | 0.611 |
| walker |  | 8808 | 20 | impl method body at src/lib.rs:1044 body 1045 |  |  | 0.611 |
| walker |  | 8828 | 20 | impl method body at src/lib.rs:1093 body 1094 |  |  | 0.611 |
| walker |  | 8848 | 20 | impl method body at src/lib.rs:1223 body 1224 |  |  | 0.611 |
| walker |  | 8868 | 20 | impl method body at src/lib.rs:1230 body 1231 |  |  | 0.611 |
| walker |  | 8889 | 21 | impl method body at src/lib.rs:1051 body 1052 |  |  | 0.611 |
| walker |  | 8910 | 21 | impl method body at src/lib.rs:1058 body 1059 |  |  | 0.611 |
| walker |  | 8933 | 23 | impl method body at src/lib.rs:579 body 580 |  |  | 0.611 |
| ns | 8941 |  | 227 | tests/macros.rs: kv_common_value_types body | 6.2 | 6.1 | 0.601 |
| walker |  | 8956 | 23 | impl method body at src/lib.rs:732 body 733 |  |  | 0.601 |
| walker |  | 8980 | 24 | impl method body at src/lib.rs:1101 body 1102 |  |  | 0.601 |
| walker |  | 9007 | 27 | impl method body at src/lib.rs:1086 body 1087 |  |  | 0.601 |
| ns | 9028 |  | 87 | CI workflow job-name roster | 7.1 |  | 0.598 |
| walker |  | 9035 | 28 | impl method body at src/lib.rs:1072 body 1073 |  |  | 0.598 |
| walker |  | 9063 | 28 | impl method body at src/lib.rs:1079 body 1080 |  |  | 0.598 |
| walker |  | 9092 | 29 | impl method body at src/lib.rs:1065 body 1066 |  |  | 0.598 |
| walker |  | 9125 | 33 | impl method body at src/lib.rs:599 body 600 |  |  | 0.598 |
| ns | 9147 |  | 119 | test_max_level_features/Cargo.toml | 7.2 |  | 0.592 |
| walker |  | 9158 | 33 | impl method body at src/lib.rs:752 body 753 |  |  | 0.592 |
| walker |  | 9194 | 36 | impl method body at src/lib.rs:620 body 621 |  |  | 0.592 |
| walker |  | 9230 | 36 | impl method body at src/lib.rs:774 body 775 |  |  | 0.592 |
| walker |  | 9272 | 42 | impl method body at src/lib.rs:862 body 863 |  |  | 0.596 |
| walker |  | 9314 | 42 | impl method body at src/lib.rs:923 body 924 |  |  | 0.596 |
| walker |  | 9357 | 43 | impl method body at src/lib.rs:908 body 909 |  |  | 0.596 |
| ns | 9394 |  | 247 | README.md lede | 8.1 |  | 0.593 |
| walker |  | 9411 | 54 | impl method body at src/lib.rs:1212 body 1213 |  |  | 0.593 |
| walker |  | 9420 | 9 | impl method body at src/lib.rs:431 body 432 |  |  | 0.593 |
| walker |  | 9430 | 10 | impl method body at src/lib.rs:435 body 436 |  |  | 0.593 |
| walker |  | 9519 | 89 | impl method body at src/lib.rs:517 body 518 |  |  | 0.602 |
| walker |  | 9609 | 90 | impl method body at src/lib.rs:667 body 668 |  |  | 0.602 |
| ns | 9633 |  | 239 | CHANGELOG.md: Unreleased + 0.4.29 | 8.2 |  | 0.596 |
| ns | 9677 |  | 44 | CHANGELOG.md: sampled older version headings | 8.3 |  | 0.595 |
| walker |  | 9733 | 124 | pub-item doc lede at src/lib.rs:1611 |  |  | 0.595 |
| ns | 9767 |  | 90 | rfcs/0296-structured-logging.md heading roster | 8.4 |  | 0.592 |
| walker |  | 9854 | 121 | impl method body at src/lib.rs:946 body 947 |  |  | 0.592 |
| ns | 9924 |  | 157 | RFC 0296: Summary | 8.5 | 8.4 | 0.588 |
| ns | 9997 |  | 73 | src/serde.rs: serde impl roster | 9.1 |  | 0.586 |
