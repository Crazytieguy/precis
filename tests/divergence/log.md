Score(3000)=0.572 I=0.810 C=0.404 ns_rows≤3K=17/47 (reached=6 partial=2 missing=9)

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 46 | 46 | listing of '.' |  |  | 1.000 |
| ns | 46 |  | 46 | Root directory listing | 1.1 |  | 1.000 |
| ns | 67 |  | 21 | src/ directory listing | 1.2 |  | 0.811 |
| ns | 87 |  | 20 | src/kv/ directory listing | 1.3 |  | 0.694 |
| walker |  | 98 | 52 | README headline in README.md |  |  | 0.694 |
| walker |  | 119 | 21 | listing of 'src' |  |  | 0.856 |
| ns | 122 |  | 35 | Secondary directory listings (tests/, benches/, CI, test_max_level_features/, rfcs/) | 1.4 |  | 0.725 |
| walker |  | 139 | 20 | listing of 'src/kv' |  |  | 0.847 |
| walker |  | 142 | 3 | listing of '.github' |  |  | 0.847 |
| walker |  | 146 | 4 | listing of '.github/workflows' |  |  | 0.850 |
| ns | 159 |  | 37 | triagebot.toml + .gitignore + .precis-pin | 1.5 |  | 0.772 |
| walker |  | 202 | 56 | headings outline in README.md |  |  | 0.772 |
| ns | 293 |  | 134 | Cargo.toml package identity (name/version/license/repo) | 1.6 |  | 0.644 |
| walker |  | 393 | 191 | [package] in Cargo.toml |  |  | 0.829 |
| ns | 448 |  | 155 | Crate-doc lede (lib.rs) | 1.7 |  | 0.737 |
| walker |  | 457 | 64 | README.md section #0 |  |  | 0.738 |
| walker |  | 461 | 4 | listing of 'benches' |  |  | 0.744 |
| ns | 551 |  | 103 | Cargo.toml package identity (MSRV, edition, docs.rs features) | 1.8 |  | 0.708 |
| walker |  | 552 | 91 | crate-doc lede in src/kv/mod.rs |  |  | 0.709 |
| walker |  | 560 | 8 | listing of 'tests' |  |  | 0.730 |
| walker |  | 569 | 9 | listing of 'test_max_level_features' |  |  | 0.765 |
| walker |  | 579 | 10 | listing of 'rfcs' |  |  | 0.787 |
| walker |  | 615 | 36 | manifest config in Cargo.toml |  |  | 0.827 |
| ns | 688 |  | 137 | Cargo.toml [features]: max_level_*/release_max_level_* | 1.9 |  | 0.731 |
| walker |  | 760 | 145 | mod/use plumbing in src/lib.rs |  |  | 0.731 |
| ns | 963 |  | 275 | Cargo.toml [features]: std/kv/kv_std/kv_sval/kv_serde | 1.10 |  | 0.649 |
| walker |  | 1006 | 246 | crate-doc lede in src/lib.rs |  |  | 0.727 |
| walker |  | 1032 | 26 | pub-item names surface in src/kv/key.rs |  |  | 0.727 |
| walker |  | 1063 | 31 | pub item at src/kv/key.rs:7 |  |  | 0.727 |
| ns | 1080 |  | 117 | Cargo.toml [dependencies] | 1.11 |  | 0.706 |
| walker |  | 1090 | 27 | pub-item names surface in src/kv/source.rs |  |  | 0.706 |
| walker |  | 1124 | 34 | pub item at src/kv/error.rs:5 |  |  | 0.706 |
| walker |  | 1172 | 48 | pub item at src/kv/source.rs:235 |  |  | 0.707 |
| walker |  | 1241 | 69 | macro_export names across src/kv |  |  | 0.707 |
| walker |  | 1279 | 38 | pub-item names surface in src/kv/value.rs |  |  | 0.707 |
| walker |  | 1310 | 31 | pub item at src/kv/value.rs:11 |  |  | 0.707 |
| walker |  | 1336 | 26 | pub item at src/kv/value.rs:119 |  |  | 0.707 |
| ns | 1493 |  | 413 | Level enum | 2.1 |  | 0.601 |
| walker |  | 1636 | 300 | pub-item names surface in src/lib.rs |  |  | 0.601 |
| walker |  | 1636 | 0 | pub item at src/lib.rs:1351 |  |  | 0.601 |
| walker |  | 1636 | 0 | pub item at src/lib.rs:1375 |  |  | 0.601 |
| walker |  | 1636 | 0 | pub item at src/lib.rs:1396 |  |  | 0.601 |
| walker |  | 1636 | 0 | pub item at src/lib.rs:1420 |  |  | 0.601 |
| walker |  | 1636 | 0 | pub item at src/lib.rs:1478 |  |  | 0.601 |
| walker |  | 1636 | 0 | pub item at src/lib.rs:1529 |  |  | 0.601 |
| walker |  | 1636 | 0 | pub item at src/lib.rs:1581 |  |  | 0.601 |
| walker |  | 1659 | 23 | pub item at src/lib.rs:1549 |  |  | 0.601 |
| walker |  | 1686 | 27 | pub item at src/lib.rs:1003 |  |  | 0.601 |
| walker |  | 1714 | 28 | pub item at src/lib.rs:1566 |  |  | 0.601 |
| walker |  | 1726 | 12 | pub item body at src/lib.rs:1478 body 1479 |  |  | 0.601 |
| walker |  | 1766 | 40 | pub item at src/lib.rs:1200 |  |  | 0.601 |
| walker |  | 1782 | 16 | pub item body at src/lib.rs:1420 body 1421 |  |  | 0.601 |
| ns | 1800 |  | 307 | LevelFilter enum | 2.2 |  | 0.547 |
| walker |  | 1833 | 51 | pub item at src/lib.rs:1158 |  |  | 0.547 |
| walker |  | 1853 | 20 | pub item body at src/lib.rs:1351 body 1352 |  |  | 0.547 |
| ns | 1945 |  | 145 | Level & LevelFilter public method roster | 2.3 |  | 0.524 |
| walker |  | 1959 | 106 | pub item at src/lib.rs:842 |  |  | 0.525 |
| walker |  | 1979 | 20 | pub-item doc lede at src/lib.rs:1158 |  |  | 0.525 |
| ns | 2092 |  | 147 | FromStr for Level | 2.4 |  | 0.505 |
| walker |  | 2153 | 174 | pub item at src/lib.rs:636 |  |  | 0.544 |
| walker |  | 2174 | 21 | pub-item doc lede at src/lib.rs:842 |  |  | 0.544 |
| walker |  | 2266 | 92 | pub item body at src/lib.rs:1375 body 1376 |  |  | 0.544 |
| ns | 2419 |  | 327 | Record struct + KeyValues wrapper | 2.5 |  | 0.515 |
| ns | 2557 |  | 138 | Record accessor method roster | 2.6 |  | 0.499 |
| walker |  | 2569 | 303 | pub item at src/lib.rs:475 |  |  | 0.572 |
| walker |  | 2609 | 40 | pub-item doc lede at src/lib.rs:1581 |  |  | 0.572 |
| walker |  | 2655 | 46 | pub-item doc lede at src/lib.rs:1549 |  |  | 0.572 |
| ns | 3004 |  | 447 | Log trait | 2.7 |  | 0.528 |
| walker |  | 3070 | 415 | pub item at src/lib.rs:1249 |  |  | 0.618 |
| walker |  | 3087 | 17 | pub-item doc lede at src/lib.rs:1249 |  |  | 0.626 |
| ns | 3280 |  | 276 | NopLogger + blanket impl for &T | 2.8 |  | 0.589 |
| walker |  | 3365 | 278 | pub item at src/lib.rs:1611 |  |  | 0.589 |
| walker |  | 3514 | 149 | pub item body at src/lib.rs:1529 body 1530 |  |  | 0.589 |
| walker |  | 3570 | 56 | pub item at src/kv/key.rs:37 |  |  | 0.589 |
| ns | 3615 |  | 335 | set_max_level + max_level | 2.9 |  | 0.568 |
| walker |  | 3735 | 165 | pub item body at src/lib.rs:1396 body 1397 |  |  | 0.583 |
| walker |  | 3798 | 63 | pub-item doc lede at src/lib.rs:1566 |  |  | 0.583 |
| ns | 3906 |  | 291 | set_boxed_logger + set_logger | 2.10 |  | 0.564 |
| walker |  | 4056 | 258 | macro_export names across src |  |  | 0.567 |
| walker |  | 4131 | 75 | pub-item doc lede at src/lib.rs:1351 |  |  | 0.584 |
| ns | 4179 |  | 273 | logger() fn | 2.11 |  | 0.567 |
| walker |  | 4231 | 100 | pub-item doc lede at src/lib.rs:475 |  |  | 0.601 |
| walker |  | 4319 | 88 | pub-item doc lede at src/lib.rs:1003 |  |  | 0.601 |
| walker |  | 4333 | 14 | pub-item doc lede at src/kv/error.rs:5 |  |  | 0.601 |
| ns | 4437 |  | 258 | macro_rules! definition roster | 3.1 |  | 0.618 |
| walker |  | 4464 | 131 | mod/use plumbing in src/kv/mod.rs |  |  | 0.619 |
| walker |  | 4559 | 95 | pub-item doc lede at src/lib.rs:1200 |  |  | 0.619 |
| walker |  | 4579 | 20 | pub-item doc lede at src/kv/key.rs:7 |  |  | 0.619 |
| walker |  | 4599 | 20 | pub-item doc lede at src/kv/value.rs:11 |  |  | 0.619 |
| walker |  | 4721 | 122 | pub-item doc lede at src/lib.rs:636 |  |  | 0.652 |
| walker |  | 4817 | 96 | pub-item doc lede at src/lib.rs:1529 |  |  | 0.652 |
| walker |  | 4861 | 44 | macro_export body at src/kv/value.rs:1129 |  |  | 0.652 |
| walker |  | 4905 | 44 | macro_export body at src/kv/value.rs:1139 |  |  | 0.652 |
| walker |  | 4926 | 21 | pub-item doc lede at src/kv/source.rs:235 |  |  | 0.652 |
| ns | 4939 |  | 502 | log! macro | 3.2 | 3.1 | 0.614 |
| walker |  | 4971 | 45 | macro_export body at src/kv/value.rs:1149 |  |  | 0.614 |
| walker |  | 5016 | 45 | macro_export body at src/kv/value.rs:1159 |  |  | 0.614 |
| walker |  | 5061 | 45 | macro_export body at src/kv/value.rs:1169 |  |  | 0.614 |
| walker |  | 5160 | 99 | pub-item doc lede at src/lib.rs:1375 |  |  | 0.614 |
| walker |  | 5179 | 19 | README.md section #6 |  |  | 0.614 |
| ns | 5555 |  | 616 | error! macro | 3.3 | 3.1 | 0.583 |
| walker |  | 5591 | 412 | [features] in Cargo.toml |  |  | 0.655 |
| walker |  | 5702 | 111 | pub-item doc lede at src/lib.rs:1420 |  |  | 0.667 |
| ns | 5890 |  | 335 | log_enabled! macro | 3.4 | 3.1 | 0.650 |
| ns | 6413 |  | 523 | __log_value! kv capture-modifier dispatch | 3.5 | 3.1 | 0.616 |
| ns | 6838 |  | 425 | GlobalLogger + log/enabled/loc public fns | 4.1 |  | 0.588 |
| ns | 7119 |  | 281 | kv/mod.rs doc lede | 5.1 |  | 0.578 |
| ns | 7244 |  | 125 | kv/mod.rs capturing-modifier list | 5.2 |  | 0.571 |
| ns | 7449 |  | 205 | kv/mod.rs module decls + re-exports | 5.3 |  | 0.569 |
| walker |  | 7718 | 2016 | impl method sigs in src/lib.rs |  |  | 0.618 |
| ns | 7822 |  | 373 | kv/key.rs: ToKey trait + Key struct | 5.4 |  | 0.602 |
| walker |  | 7845 | 127 | pub-item doc lede at src/lib.rs:1478 |  |  | 0.602 |
| walker |  | 7928 | 83 | README.md section #1 |  |  | 0.602 |
| walker |  | 8052 | 124 | pub-item doc lede at src/lib.rs:1611 |  |  | 0.602 |
| ns | 8055 |  | 233 | kv/error.rs: Error struct, Inner enum, msg() constructor | 5.5 |  | 0.590 |
| ns | 8101 |  | 46 | kv/source.rs: Source trait method roster | 5.6 |  | 0.588 |
| ns | 8185 |  | 84 | kv/source.rs: VisitSource trait | 5.7 |  | 0.591 |
| ns | 8288 |  | 103 | kv/value.rs: ToValue trait + Value struct | 5.8 |  | 0.596 |
| walker |  | 8334 | 282 | crate-doc body in src/lib.rs |  |  | 0.596 |
| ns | 8429 |  | 141 | kv/value.rs: dependency-free Inner enum (data model) | 5.9 |  | 0.589 |
| walker |  | 8549 | 215 | pub-item doc lede at src/lib.rs:1396 |  |  | 0.589 |
| walker |  | 8588 | 39 | pub-item doc lede at src/kv/value.rs:119 |  |  | 0.589 |
| ns | 8713 |  | 284 | tests/macros.rs test-fn roster | 6.1 |  | 0.578 |
| ns | 8940 |  | 227 | tests/macros.rs: kv_common_value_types body | 6.2 | 6.1 | 0.568 |
| walker |  | 8942 | 354 | crate-doc body in src/kv/mod.rs |  |  | 0.588 |
| ns | 9027 |  | 87 | CI workflow job-name roster | 7.1 |  | 0.585 |
| ns | 9146 |  | 119 | test_max_level_features/Cargo.toml | 7.2 |  | 0.580 |
| walker |  | 9379 | 437 | pub item at src/kv/source.rs:51 |  |  | 0.583 |
| ns | 9393 |  | 247 | README.md lede | 8.1 |  | 0.581 |
| ns | 9632 |  | 239 | CHANGELOG.md: Unreleased + 0.4.29 | 8.2 |  | 0.574 |
| ns | 9676 |  | 44 | CHANGELOG.md: sampled older version headings | 8.3 |  | 0.574 |
| ns | 9766 |  | 90 | rfcs/0296-structured-logging.md heading roster | 8.4 |  | 0.570 |
| walker |  | 9784 | 405 | [dependencies] in Cargo.toml |  |  | 0.577 |
| ns | 9923 |  | 157 | RFC 0296: Summary | 8.5 | 8.4 | 0.574 |
| walker |  | 9956 | 172 | impl method sigs in src/kv/key.rs |  |  | 0.579 |
| ns | 9996 |  | 73 | src/serde.rs: serde impl roster | 9.1 |  | 0.577 |
