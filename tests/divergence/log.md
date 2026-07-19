Score(3000)=0.591 I=0.818 C=0.426 ns_rows≤3K=17/47 (reached=7 partial=2 missing=8)

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 46 | 46 | listing of '.' |  |  | 1.000 |
| ns | 46 |  | 46 | Root directory listing | 1.1 |  | 1.000 |
| walker |  | 49 | 3 | listing of '.github' |  |  | 1.000 |
| walker |  | 53 | 4 | listing of '.github/workflows' |  |  | 1.000 |
| ns | 67 |  | 21 | src/ directory listing | 1.2 |  | 0.811 |
| walker |  | 74 | 21 | listing of 'src' |  |  | 1.000 |
| ns | 87 |  | 20 | src/kv/ directory listing | 1.3 |  | 0.856 |
| walker |  | 94 | 20 | listing of 'src/kv' |  |  | 1.000 |
| walker |  | 98 | 4 | listing of 'benches' |  |  | 1.000 |
| ns | 122 |  | 35 | Secondary directory listings (tests/, benches/, CI, test_max_level_features/, rfcs/) | 1.4 |  | 0.859 |
| walker |  | 150 | 52 | README headline in README.md |  |  | 0.860 |
| walker |  | 158 | 8 | listing of 'tests' |  |  | 0.898 |
| ns | 159 |  | 37 | triagebot.toml + .gitignore + .precis-pin | 1.5 |  | 0.816 |
| walker |  | 249 | 91 | crate-doc lede in src/kv/mod.rs |  |  | 0.817 |
| walker |  | 258 | 9 | listing of 'test_max_level_features' |  |  | 0.873 |
| walker |  | 268 | 10 | listing of 'rfcs' |  |  | 0.909 |
| ns | 293 |  | 134 | Cargo.toml package identity (name/version/license/repo) | 1.6 |  | 0.758 |
| walker |  | 324 | 56 | headings outline in README.md |  |  | 0.758 |
| ns | 448 |  | 155 | Crate-doc lede (lib.rs) | 1.7 |  | 0.674 |
| walker |  | 515 | 191 | [package] in Cargo.toml |  |  | 0.831 |
| ns | 551 |  | 103 | Cargo.toml package identity (MSRV, edition, docs.rs features) | 1.8 |  | 0.787 |
| walker |  | 579 | 64 | README.md section #0 |  |  | 0.787 |
| walker |  | 615 | 36 | manifest config in Cargo.toml |  |  | 0.827 |
| ns | 688 |  | 137 | Cargo.toml [features]: max_level_*/release_max_level_* | 1.9 |  | 0.731 |
| walker |  | 760 | 145 | mod/use plumbing in src/lib.rs |  |  | 0.731 |
| ns | 963 |  | 275 | Cargo.toml [features]: std/kv/kv_std/kv_sval/kv_serde | 1.10 |  | 0.649 |
| walker |  | 1006 | 246 | crate-doc lede in src/lib.rs |  |  | 0.727 |
| walker |  | 1032 | 26 | pub-item names surface in src/kv/key.rs |  |  | 0.727 |
| walker |  | 1063 | 31 | pub item at src/kv/key.rs:7 |  |  | 0.727 |
| ns | 1080 |  | 117 | Cargo.toml [dependencies] | 1.11 |  | 0.706 |
| walker |  | 1182 | 119 | [dependencies] in Cargo.toml |  |  | 0.740 |
| walker |  | 1209 | 27 | pub-item names surface in src/kv/source.rs |  |  | 0.740 |
| walker |  | 1243 | 34 | pub item at src/kv/error.rs:5 |  |  | 0.740 |
| walker |  | 1291 | 48 | pub item at src/kv/source.rs:235 |  |  | 0.740 |
| walker |  | 1360 | 69 | macro_export names across src/kv |  |  | 0.740 |
| walker |  | 1398 | 38 | pub-item names surface in src/kv/value.rs |  |  | 0.740 |
| walker |  | 1429 | 31 | pub item at src/kv/value.rs:11 |  |  | 0.740 |
| walker |  | 1455 | 26 | pub item at src/kv/value.rs:119 |  |  | 0.741 |
| ns | 1493 |  | 413 | Level enum | 2.1 |  | 0.629 |
| walker |  | 1755 | 300 | pub-item names surface in src/lib.rs |  |  | 0.629 |
| walker |  | 1755 | 0 | pub item at src/lib.rs:1351 |  |  | 0.629 |
| walker |  | 1755 | 0 | pub item at src/lib.rs:1375 |  |  | 0.629 |
| walker |  | 1755 | 0 | pub item at src/lib.rs:1396 |  |  | 0.629 |
| walker |  | 1755 | 0 | pub item at src/lib.rs:1420 |  |  | 0.629 |
| walker |  | 1755 | 0 | pub item at src/lib.rs:1478 |  |  | 0.629 |
| walker |  | 1755 | 0 | pub item at src/lib.rs:1529 |  |  | 0.629 |
| walker |  | 1755 | 0 | pub item at src/lib.rs:1581 |  |  | 0.629 |
| walker |  | 1778 | 23 | pub item at src/lib.rs:1549 |  |  | 0.629 |
| ns | 1800 |  | 307 | LevelFilter enum | 2.2 |  | 0.572 |
| walker |  | 1805 | 27 | pub item at src/lib.rs:1003 |  |  | 0.572 |
| walker |  | 1833 | 28 | pub item at src/lib.rs:1566 |  |  | 0.572 |
| walker |  | 1845 | 12 | pub item body at src/lib.rs:1478 body 1479 |  |  | 0.572 |
| walker |  | 1885 | 40 | pub item at src/lib.rs:1200 |  |  | 0.572 |
| walker |  | 1901 | 16 | pub item body at src/lib.rs:1420 body 1421 |  |  | 0.572 |
| ns | 1945 |  | 145 | Level & LevelFilter public method roster | 2.3 |  | 0.549 |
| walker |  | 1952 | 51 | pub item at src/lib.rs:1158 |  |  | 0.549 |
| walker |  | 1972 | 20 | pub item body at src/lib.rs:1351 body 1352 |  |  | 0.549 |
| walker |  | 2078 | 106 | pub item at src/lib.rs:842 |  |  | 0.550 |
| ns | 2092 |  | 147 | FromStr for Level | 2.4 |  | 0.529 |
| walker |  | 2098 | 20 | pub-item doc lede at src/lib.rs:1158 |  |  | 0.529 |
| walker |  | 2272 | 174 | pub item at src/lib.rs:636 |  |  | 0.567 |
| walker |  | 2293 | 21 | pub-item doc lede at src/lib.rs:842 |  |  | 0.567 |
| walker |  | 2385 | 92 | pub item body at src/lib.rs:1375 body 1376 |  |  | 0.567 |
| ns | 2419 |  | 327 | Record struct + KeyValues wrapper | 2.5 |  | 0.536 |
| ns | 2557 |  | 138 | Record accessor method roster | 2.6 |  | 0.519 |
| walker |  | 2688 | 303 | pub item at src/lib.rs:475 |  |  | 0.591 |
| walker |  | 2728 | 40 | pub-item doc lede at src/lib.rs:1581 |  |  | 0.591 |
| walker |  | 2774 | 46 | pub-item doc lede at src/lib.rs:1549 |  |  | 0.591 |
| ns | 3004 |  | 447 | Log trait | 2.7 |  | 0.545 |
| walker |  | 3189 | 415 | pub item at src/lib.rs:1249 |  |  | 0.634 |
| walker |  | 3206 | 17 | pub-item doc lede at src/lib.rs:1249 |  |  | 0.641 |
| ns | 3280 |  | 276 | NopLogger + blanket impl for &T | 2.8 |  | 0.604 |
| walker |  | 3484 | 278 | pub item at src/lib.rs:1611 |  |  | 0.604 |
| ns | 3615 |  | 335 | set_max_level + max_level | 2.9 |  | 0.582 |
| walker |  | 3633 | 149 | pub item body at src/lib.rs:1529 body 1530 |  |  | 0.582 |
| walker |  | 3689 | 56 | pub item at src/kv/key.rs:37 |  |  | 0.582 |
| walker |  | 3854 | 165 | pub item body at src/lib.rs:1396 body 1397 |  |  | 0.597 |
| ns | 3906 |  | 291 | set_boxed_logger + set_logger | 2.10 |  | 0.577 |
| walker |  | 3917 | 63 | pub-item doc lede at src/lib.rs:1566 |  |  | 0.577 |
| walker |  | 4175 | 258 | macro_export names across src |  |  | 0.581 |
| ns | 4179 |  | 273 | logger() fn | 2.11 |  | 0.563 |
| walker |  | 4250 | 75 | pub-item doc lede at src/lib.rs:1351 |  |  | 0.579 |
| walker |  | 4350 | 100 | pub-item doc lede at src/lib.rs:475 |  |  | 0.613 |
| ns | 4437 |  | 258 | macro_rules! definition roster | 3.1 |  | 0.630 |
| walker |  | 4438 | 88 | pub-item doc lede at src/lib.rs:1003 |  |  | 0.630 |
| walker |  | 4452 | 14 | pub-item doc lede at src/kv/error.rs:5 |  |  | 0.630 |
| walker |  | 4583 | 131 | mod/use plumbing in src/kv/mod.rs |  |  | 0.631 |
| walker |  | 4678 | 95 | pub-item doc lede at src/lib.rs:1200 |  |  | 0.631 |
| walker |  | 4698 | 20 | pub-item doc lede at src/kv/key.rs:7 |  |  | 0.631 |
| walker |  | 4718 | 20 | pub-item doc lede at src/kv/value.rs:11 |  |  | 0.631 |
| walker |  | 4840 | 122 | pub-item doc lede at src/lib.rs:636 |  |  | 0.664 |
| walker |  | 4936 | 96 | pub-item doc lede at src/lib.rs:1529 |  |  | 0.664 |
| ns | 4939 |  | 502 | log! macro | 3.2 | 3.1 | 0.624 |
| walker |  | 4980 | 44 | macro_export body at src/kv/value.rs:1129 |  |  | 0.624 |
| walker |  | 5024 | 44 | macro_export body at src/kv/value.rs:1139 |  |  | 0.624 |
| walker |  | 5045 | 21 | pub-item doc lede at src/kv/source.rs:235 |  |  | 0.625 |
| walker |  | 5090 | 45 | macro_export body at src/kv/value.rs:1149 |  |  | 0.625 |
| walker |  | 5135 | 45 | macro_export body at src/kv/value.rs:1159 |  |  | 0.625 |
| walker |  | 5180 | 45 | macro_export body at src/kv/value.rs:1169 |  |  | 0.625 |
| walker |  | 5279 | 99 | pub-item doc lede at src/lib.rs:1375 |  |  | 0.625 |
| walker |  | 5298 | 19 | README.md section #6 |  |  | 0.625 |
| ns | 5555 |  | 616 | error! macro | 3.3 | 3.1 | 0.593 |
| walker |  | 5708 | 410 | [features] in Cargo.toml |  |  | 0.665 |
| walker |  | 5819 | 111 | pub-item doc lede at src/lib.rs:1420 |  |  | 0.677 |
| ns | 5890 |  | 335 | log_enabled! macro | 3.4 | 3.1 | 0.660 |
| walker |  | 5946 | 127 | pub-item doc lede at src/lib.rs:1478 |  |  | 0.660 |
| walker |  | 6029 | 83 | README.md section #1 |  |  | 0.660 |
| ns | 6413 |  | 523 | __log_value! kv capture-modifier dispatch | 3.5 | 3.1 | 0.625 |
| ns | 6838 |  | 425 | GlobalLogger + log/enabled/loc public fns | 4.1 |  | 0.597 |
| ns | 7119 |  | 281 | kv/mod.rs doc lede | 5.1 |  | 0.586 |
| ns | 7244 |  | 125 | kv/mod.rs capturing-modifier list | 5.2 |  | 0.580 |
| ns | 7449 |  | 205 | kv/mod.rs module decls + re-exports | 5.3 |  | 0.578 |
| ns | 7822 |  | 373 | kv/key.rs: ToKey trait + Key struct | 5.4 |  | 0.563 |
| walker |  | 8045 | 2016 | impl method sigs in src/lib.rs |  |  | 0.610 |
| ns | 8055 |  | 233 | kv/error.rs: Error struct, Inner enum, msg() constructor | 5.5 |  | 0.598 |
| ns | 8101 |  | 46 | kv/source.rs: Source trait method roster | 5.6 |  | 0.596 |
| walker |  | 8169 | 124 | pub-item doc lede at src/lib.rs:1611 |  |  | 0.596 |
| ns | 8185 |  | 84 | kv/source.rs: VisitSource trait | 5.7 |  | 0.599 |
| ns | 8288 |  | 103 | kv/value.rs: ToValue trait + Value struct | 5.8 |  | 0.604 |
| ns | 8429 |  | 141 | kv/value.rs: dependency-free Inner enum (data model) | 5.9 |  | 0.597 |
| walker |  | 8451 | 282 | crate-doc body in src/lib.rs |  |  | 0.597 |
| walker |  | 8666 | 215 | pub-item doc lede at src/lib.rs:1396 |  |  | 0.597 |
| ns | 8713 |  | 284 | tests/macros.rs test-fn roster | 6.1 |  | 0.585 |
| ns | 8940 |  | 227 | tests/macros.rs: kv_common_value_types body | 6.2 | 6.1 | 0.576 |
| walker |  | 8949 | 283 | dev/build/target dependencies in Cargo.toml |  |  | 0.576 |
| walker |  | 8988 | 39 | pub-item doc lede at src/kv/value.rs:119 |  |  | 0.576 |
| ns | 9027 |  | 87 | CI workflow job-name roster | 7.1 |  | 0.573 |
| ns | 9146 |  | 119 | test_max_level_features/Cargo.toml | 7.2 |  | 0.567 |
| walker |  | 9342 | 354 | crate-doc body in src/kv/mod.rs |  |  | 0.587 |
| ns | 9393 |  | 247 | README.md lede | 8.1 |  | 0.584 |
| ns | 9632 |  | 239 | CHANGELOG.md: Unreleased + 0.4.29 | 8.2 |  | 0.578 |
| ns | 9676 |  | 44 | CHANGELOG.md: sampled older version headings | 8.3 |  | 0.577 |
| ns | 9766 |  | 90 | rfcs/0296-structured-logging.md heading roster | 8.4 |  | 0.574 |
| walker |  | 9779 | 437 | pub item at src/kv/source.rs:51 |  |  | 0.577 |
| ns | 9923 |  | 157 | RFC 0296: Summary | 8.5 | 8.4 | 0.574 |
| walker |  | 9951 | 172 | impl method sigs in src/kv/key.rs |  |  | 0.579 |
| ns | 9996 |  | 73 | src/serde.rs: serde impl roster | 9.1 |  | 0.577 |
