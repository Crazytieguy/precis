Score(3000)=0.536 I=0.768 C=0.373 ns_rows≤3K=17/47 (reached=5 partial=2 missing=10)

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 46 | 46 | listing of '.' |  |  | 1.000 |
| ns | 46 |  | 46 | Root directory listing | 1.1 |  | 1.000 |
| ns | 67 |  | 21 | src/ directory listing | 1.2 |  | 0.811 |
| ns | 87 |  | 20 | src/kv/ directory listing | 1.3 |  | 0.694 |
| walker |  | 98 | 52 | README headline in README.md |  |  | 0.694 |
| ns | 122 |  | 35 | Secondary directory listings (tests/, benches/, CI, test_max_level_features/, rfcs/) | 1.4 |  | 0.587 |
| walker |  | 154 | 56 | headings outline in README.md |  |  | 0.587 |
| ns | 159 |  | 37 | triagebot.toml + .gitignore + .precis-pin | 1.5 |  | 0.534 |
| ns | 293 |  | 134 | Cargo.toml package identity (name/version/license/repo) | 1.6 |  | 0.445 |
| walker |  | 345 | 191 | [package] in Cargo.toml |  |  | 0.650 |
| walker |  | 366 | 21 | listing of 'src' |  |  | 0.743 |
| walker |  | 386 | 20 | listing of 'src/kv' |  |  | 0.827 |
| ns | 448 |  | 155 | Crate-doc lede (lib.rs) | 1.7 |  | 0.735 |
| walker |  | 477 | 91 | crate-doc lede in src/kv/mod.rs |  |  | 0.736 |
| walker |  | 480 | 3 | listing of '.github' |  |  | 0.736 |
| walker |  | 544 | 64 | README.md section #0 |  |  | 0.736 |
| walker |  | 548 | 4 | listing of 'benches' |  |  | 0.738 |
| ns | 551 |  | 103 | Cargo.toml package identity (MSRV, edition, docs.rs features) | 1.8 |  | 0.703 |
| walker |  | 584 | 36 | manifest config in Cargo.toml |  |  | 0.745 |
| walker |  | 610 | 26 | pub-item names surface in src/kv/key.rs |  |  | 0.745 |
| walker |  | 641 | 31 | pub item at src/kv/key.rs:7 |  |  | 0.745 |
| walker |  | 668 | 27 | pub-item names surface in src/kv/source.rs |  |  | 0.745 |
| ns | 688 |  | 137 | Cargo.toml [features]: max_level_*/release_max_level_* | 1.9 |  | 0.659 |
| walker |  | 813 | 145 | mod/use plumbing in src/lib.rs |  |  | 0.659 |
| walker |  | 847 | 34 | pub item at src/kv/error.rs:5 |  |  | 0.659 |
| walker |  | 895 | 48 | pub item at src/kv/source.rs:235 |  |  | 0.659 |
| ns | 963 |  | 275 | Cargo.toml [features]: std/kv/kv_std/kv_sval/kv_serde | 1.10 |  | 0.584 |
| ns | 1080 |  | 117 | Cargo.toml [dependencies] | 1.11 |  | 0.567 |
| walker |  | 1141 | 246 | crate-doc lede in src/lib.rs |  |  | 0.646 |
| walker |  | 1181 | 40 | pub-item names surface in src/kv/value.rs |  |  | 0.646 |
| walker |  | 1212 | 31 | pub item at src/kv/value.rs:11 |  |  | 0.646 |
| walker |  | 1238 | 26 | pub item at src/kv/value.rs:119 |  |  | 0.646 |
| walker |  | 1305 | 67 | macro_export names across src/kv |  |  | 0.646 |
| walker |  | 1361 | 56 | pub item at src/kv/key.rs:37 |  |  | 0.647 |
| ns | 1493 |  | 413 | Level enum | 2.1 |  | 0.549 |
| walker |  | 1661 | 300 | pub-item names surface in src/lib.rs |  |  | 0.549 |
| walker |  | 1661 | 0 | pub item at src/lib.rs:1351 |  |  | 0.549 |
| walker |  | 1661 | 0 | pub item at src/lib.rs:1375 |  |  | 0.549 |
| walker |  | 1661 | 0 | pub item at src/lib.rs:1396 |  |  | 0.549 |
| walker |  | 1661 | 0 | pub item at src/lib.rs:1420 |  |  | 0.549 |
| walker |  | 1661 | 0 | pub item at src/lib.rs:1478 |  |  | 0.549 |
| walker |  | 1661 | 0 | pub item at src/lib.rs:1529 |  |  | 0.549 |
| walker |  | 1661 | 0 | pub item at src/lib.rs:1581 |  |  | 0.549 |
| walker |  | 1684 | 23 | pub item at src/lib.rs:1549 |  |  | 0.549 |
| walker |  | 1711 | 27 | pub item at src/lib.rs:1003 |  |  | 0.549 |
| walker |  | 1739 | 28 | pub item at src/lib.rs:1566 |  |  | 0.549 |
| walker |  | 1751 | 12 | pub item body at src/lib.rs:1478 body 1479 |  |  | 0.549 |
| walker |  | 1791 | 40 | pub item at src/lib.rs:1200 |  |  | 0.549 |
| ns | 1800 |  | 307 | LevelFilter enum | 2.2 |  | 0.500 |
| walker |  | 1807 | 16 | pub item body at src/lib.rs:1420 body 1421 |  |  | 0.500 |
| walker |  | 1858 | 51 | pub item at src/lib.rs:1158 |  |  | 0.500 |
| walker |  | 1878 | 20 | pub item body at src/lib.rs:1351 body 1352 |  |  | 0.500 |
| ns | 1945 |  | 145 | Level & LevelFilter public method roster | 2.3 |  | 0.479 |
| walker |  | 1984 | 106 | pub item at src/lib.rs:842 |  |  | 0.480 |
| walker |  | 2004 | 20 | pub-item doc lede at src/lib.rs:1158 |  |  | 0.480 |
| ns | 2092 |  | 147 | FromStr for Level | 2.4 |  | 0.462 |
| walker |  | 2178 | 174 | pub item at src/lib.rs:636 |  |  | 0.502 |
| walker |  | 2199 | 21 | pub-item doc lede at src/lib.rs:842 |  |  | 0.502 |
| walker |  | 2291 | 92 | pub item body at src/lib.rs:1375 body 1376 |  |  | 0.502 |
| ns | 2419 |  | 327 | Record struct + KeyValues wrapper | 2.5 |  | 0.477 |
| ns | 2557 |  | 138 | Record accessor method roster | 2.6 |  | 0.461 |
| walker |  | 2594 | 303 | pub item at src/lib.rs:475 |  |  | 0.535 |
| walker |  | 2634 | 40 | pub-item doc lede at src/lib.rs:1581 |  |  | 0.536 |
| walker |  | 2680 | 46 | pub-item doc lede at src/lib.rs:1549 |  |  | 0.536 |
| ns | 3004 |  | 447 | Log trait | 2.7 |  | 0.494 |
| walker |  | 3095 | 415 | pub item at src/lib.rs:1249 |  |  | 0.585 |
| walker |  | 3112 | 17 | pub-item doc lede at src/lib.rs:1249 |  |  | 0.592 |
| ns | 3280 |  | 276 | NopLogger + blanket impl for &T | 2.8 |  | 0.558 |
| walker |  | 3390 | 278 | pub item at src/lib.rs:1611 |  |  | 0.558 |
| walker |  | 3398 | 8 | listing of 'tests' |  |  | 0.563 |
| walker |  | 3547 | 149 | pub item body at src/lib.rs:1529 body 1530 |  |  | 0.563 |
| ns | 3615 |  | 335 | set_max_level + max_level | 2.9 |  | 0.543 |
| walker |  | 3712 | 165 | pub item body at src/lib.rs:1396 body 1397 |  |  | 0.558 |
| walker |  | 3721 | 9 | listing of 'test_max_level_features' |  |  | 0.568 |
| walker |  | 3784 | 63 | pub-item doc lede at src/lib.rs:1566 |  |  | 0.568 |
| walker |  | 3794 | 10 | listing of 'rfcs' |  |  | 0.575 |
| walker |  | 3808 | 14 | pub-item doc lede at src/kv/error.rs:5 |  |  | 0.575 |
| walker |  | 3812 | 4 | listing of '.github/workflows' |  |  | 0.583 |
| ns | 3906 |  | 291 | set_boxed_logger + set_logger | 2.10 |  | 0.564 |
| walker |  | 4070 | 258 | macro_export names across src |  |  | 0.567 |
| walker |  | 4145 | 75 | pub-item doc lede at src/lib.rs:1351 |  |  | 0.584 |
| ns | 4179 |  | 273 | logger() fn | 2.11 |  | 0.567 |
| walker |  | 4245 | 100 | pub-item doc lede at src/lib.rs:475 |  |  | 0.601 |
| walker |  | 4265 | 20 | pub-item doc lede at src/kv/key.rs:7 |  |  | 0.601 |
| walker |  | 4285 | 20 | pub-item doc lede at src/kv/value.rs:11 |  |  | 0.601 |
| walker |  | 4373 | 88 | pub-item doc lede at src/lib.rs:1003 |  |  | 0.601 |
| walker |  | 4417 | 44 | macro_export body at src/kv/value.rs:1129 |  |  | 0.601 |
| ns | 4437 |  | 258 | macro_rules! definition roster | 3.1 |  | 0.618 |
| walker |  | 4461 | 44 | macro_export body at src/kv/value.rs:1139 |  |  | 0.618 |
| walker |  | 4482 | 21 | pub-item doc lede at src/kv/source.rs:235 |  |  | 0.619 |
| walker |  | 4527 | 45 | macro_export body at src/kv/value.rs:1149 |  |  | 0.619 |
| walker |  | 4572 | 45 | macro_export body at src/kv/value.rs:1159 |  |  | 0.619 |
| walker |  | 4617 | 45 | macro_export body at src/kv/value.rs:1169 |  |  | 0.619 |
| walker |  | 4748 | 131 | mod/use plumbing in src/kv/mod.rs |  |  | 0.619 |
| walker |  | 4843 | 95 | pub-item doc lede at src/lib.rs:1200 |  |  | 0.619 |
| ns | 4939 |  | 502 | log! macro | 3.2 | 3.1 | 0.583 |
| walker |  | 4965 | 122 | pub-item doc lede at src/lib.rs:636 |  |  | 0.614 |
| walker |  | 5061 | 96 | pub-item doc lede at src/lib.rs:1529 |  |  | 0.614 |
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
| walker |  | 8091 | 39 | pub-item doc lede at src/kv/value.rs:119 |  |  | 0.590 |
| ns | 8101 |  | 46 | kv/source.rs: Source trait method roster | 5.6 |  | 0.588 |
| ns | 8185 |  | 84 | kv/source.rs: VisitSource trait | 5.7 |  | 0.591 |
| ns | 8288 |  | 103 | kv/value.rs: ToValue trait + Value struct | 5.8 |  | 0.596 |
| walker |  | 8306 | 215 | pub-item doc lede at src/lib.rs:1396 |  |  | 0.596 |
| ns | 8429 |  | 141 | kv/value.rs: dependency-free Inner enum (data model) | 5.9 |  | 0.589 |
| ns | 8713 |  | 284 | tests/macros.rs test-fn roster | 6.1 |  | 0.578 |
| walker |  | 8743 | 437 | pub item at src/kv/source.rs:51 |  |  | 0.582 |
| walker |  | 8915 | 172 | impl method sigs in src/kv/key.rs |  |  | 0.588 |
| ns | 8940 |  | 227 | tests/macros.rs: kv_common_value_types body | 6.2 | 6.1 | 0.578 |
| walker |  | 8943 | 28 | impl method sigs in src/kv/error.rs |  |  | 0.579 |
| ns | 9027 |  | 87 | CI workflow job-name roster | 7.1 |  | 0.576 |
| ns | 9146 |  | 119 | test_max_level_features/Cargo.toml | 7.2 |  | 0.571 |
| walker |  | 9348 | 405 | [dependencies] in Cargo.toml |  |  | 0.578 |
| ns | 9393 |  | 247 | README.md lede | 8.1 |  | 0.575 |
| ns | 9632 |  | 239 | CHANGELOG.md: Unreleased + 0.4.29 | 8.2 |  | 0.569 |
| ns | 9676 |  | 44 | CHANGELOG.md: sampled older version headings | 8.3 |  | 0.568 |
| walker |  | 9683 | 335 | macro_export body at src/macros.rs:391 |  |  | 0.589 |
| ns | 9766 |  | 90 | rfcs/0296-structured-logging.md heading roster | 8.4 |  | 0.586 |
| walker |  | 9776 | 93 | pub-item names surface in src/__private_api.rs |  |  | 0.587 |
| walker |  | 9776 | 0 | pub item at src/__private_api.rs:103 |  |  | 0.587 |
| walker |  | 9776 | 0 | pub item at src/__private_api.rs:108 |  |  | 0.587 |
| walker |  | 9784 | 8 | pub item at src/__private_api.rs:39 |  |  | 0.587 |
| walker |  | 9793 | 9 | pub item body at src/__private_api.rs:108 body 109 |  |  | 0.588 |
| walker |  | 9813 | 20 | pub item body at src/__private_api.rs:103 body 104 |  |  | 0.589 |
| walker |  | 9823 | 10 | pub-item doc lede at src/__private_api.rs:39 |  |  | 0.589 |
| walker |  | 9916 | 93 | pub item at src/__private_api.rs:84 |  |  | 0.596 |
| ns | 9923 |  | 157 | RFC 0296: Summary | 8.5 | 8.4 | 0.592 |
| ns | 9996 |  | 73 | src/serde.rs: serde impl roster | 9.1 |  | 0.590 |
