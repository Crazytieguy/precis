Score(3000)=0.535 I=0.767 C=0.373 ns_rows≤3K=17/47 (reached=5 partial=2 missing=10)

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
| walker |  | 218 | 64 | README.md section #0 |  |  | 0.534 |
| ns | 293 |  | 134 | Cargo.toml package identity (name/version/license/repo) | 1.6 |  | 0.446 |
| walker |  | 409 | 191 | [package] in Cargo.toml |  |  | 0.650 |
| walker |  | 445 | 36 | manifest config in Cargo.toml |  |  | 0.656 |
| ns | 448 |  | 155 | Crate-doc lede (lib.rs) | 1.7 |  | 0.583 |
| walker |  | 466 | 21 | listing of 'src' |  |  | 0.666 |
| walker |  | 486 | 20 | listing of 'src/kv' |  |  | 0.741 |
| ns | 551 |  | 103 | Cargo.toml package identity (MSRV, edition, docs.rs features) | 1.8 |  | 0.743 |
| walker |  | 577 | 91 | crate-doc lede in src/kv/mod.rs |  |  | 0.743 |
| walker |  | 603 | 26 | pub-item names surface in src/kv/key.rs |  |  | 0.743 |
| walker |  | 634 | 31 | pub item at src/kv/key.rs:7 |  |  | 0.743 |
| walker |  | 661 | 27 | pub-item names surface in src/kv/source.rs |  |  | 0.743 |
| ns | 688 |  | 137 | Cargo.toml [features]: max_level_*/release_max_level_* | 1.9 |  | 0.657 |
| walker |  | 806 | 145 | mod/use plumbing in src/lib.rs |  |  | 0.657 |
| walker |  | 840 | 34 | pub item at src/kv/error.rs:5 |  |  | 0.657 |
| walker |  | 888 | 48 | pub item at src/kv/source.rs:235 |  |  | 0.657 |
| ns | 963 |  | 275 | Cargo.toml [features]: std/kv/kv_std/kv_sval/kv_serde | 1.10 |  | 0.583 |
| ns | 1080 |  | 117 | Cargo.toml [dependencies] | 1.11 |  | 0.566 |
| walker |  | 1134 | 246 | crate-doc lede in src/lib.rs |  |  | 0.644 |
| walker |  | 1174 | 40 | pub-item names surface in src/kv/value.rs |  |  | 0.644 |
| walker |  | 1205 | 31 | pub item at src/kv/value.rs:11 |  |  | 0.645 |
| walker |  | 1231 | 26 | pub item at src/kv/value.rs:119 |  |  | 0.645 |
| walker |  | 1298 | 67 | macro_export names across src/kv |  |  | 0.645 |
| walker |  | 1354 | 56 | pub item at src/kv/key.rs:37 |  |  | 0.645 |
| ns | 1493 |  | 413 | Level enum | 2.1 |  | 0.548 |
| walker |  | 1654 | 300 | pub-item names surface in src/lib.rs |  |  | 0.548 |
| walker |  | 1654 | 0 | pub item at src/lib.rs:1351 |  |  | 0.548 |
| walker |  | 1654 | 0 | pub item at src/lib.rs:1375 |  |  | 0.548 |
| walker |  | 1654 | 0 | pub item at src/lib.rs:1396 |  |  | 0.548 |
| walker |  | 1654 | 0 | pub item at src/lib.rs:1420 |  |  | 0.548 |
| walker |  | 1654 | 0 | pub item at src/lib.rs:1478 |  |  | 0.548 |
| walker |  | 1654 | 0 | pub item at src/lib.rs:1529 |  |  | 0.548 |
| walker |  | 1654 | 0 | pub item at src/lib.rs:1581 |  |  | 0.548 |
| walker |  | 1677 | 23 | pub item at src/lib.rs:1549 |  |  | 0.548 |
| walker |  | 1704 | 27 | pub item at src/lib.rs:1003 |  |  | 0.548 |
| walker |  | 1732 | 28 | pub item at src/lib.rs:1566 |  |  | 0.548 |
| walker |  | 1744 | 12 | pub item body at src/lib.rs:1478 body 1479 |  |  | 0.548 |
| walker |  | 1784 | 40 | pub item at src/lib.rs:1200 |  |  | 0.548 |
| walker |  | 1800 | 16 | pub item body at src/lib.rs:1420 body 1421 |  |  | 0.499 |
| ns | 1800 |  | 307 | LevelFilter enum | 2.2 |  | 0.499 |
| walker |  | 1851 | 51 | pub item at src/lib.rs:1158 |  |  | 0.499 |
| walker |  | 1871 | 20 | pub item body at src/lib.rs:1351 body 1352 |  |  | 0.499 |
| ns | 1945 |  | 145 | Level & LevelFilter public method roster | 2.3 |  | 0.478 |
| walker |  | 1977 | 106 | pub item at src/lib.rs:842 |  |  | 0.479 |
| walker |  | 1997 | 20 | pub-item doc lede at src/lib.rs:1158 |  |  | 0.479 |
| walker |  | 2018 | 21 | pub-item doc lede at src/lib.rs:842 |  |  | 0.479 |
| ns | 2092 |  | 147 | FromStr for Level | 2.4 |  | 0.461 |
| walker |  | 2192 | 174 | pub item at src/lib.rs:636 |  |  | 0.501 |
| walker |  | 2284 | 92 | pub item body at src/lib.rs:1375 body 1376 |  |  | 0.501 |
| ns | 2419 |  | 327 | Record struct + KeyValues wrapper | 2.5 |  | 0.476 |
| ns | 2557 |  | 138 | Record accessor method roster | 2.6 |  | 0.461 |
| walker |  | 2587 | 303 | pub item at src/lib.rs:475 |  |  | 0.535 |
| walker |  | 2627 | 40 | pub-item doc lede at src/lib.rs:1581 |  |  | 0.535 |
| walker |  | 2673 | 46 | pub-item doc lede at src/lib.rs:1549 |  |  | 0.535 |
| ns | 3004 |  | 447 | Log trait | 2.7 |  | 0.494 |
| walker |  | 3088 | 415 | pub item at src/lib.rs:1249 |  |  | 0.584 |
| walker |  | 3105 | 17 | pub-item doc lede at src/lib.rs:1249 |  |  | 0.592 |
| ns | 3280 |  | 276 | NopLogger + blanket impl for &T | 2.8 |  | 0.557 |
| walker |  | 3383 | 278 | pub item at src/lib.rs:1611 |  |  | 0.557 |
| walker |  | 3532 | 149 | pub item body at src/lib.rs:1529 body 1530 |  |  | 0.557 |
| walker |  | 3595 | 63 | pub-item doc lede at src/lib.rs:1566 |  |  | 0.557 |
| ns | 3615 |  | 335 | set_max_level + max_level | 2.9 |  | 0.537 |
| walker |  | 3760 | 165 | pub item body at src/lib.rs:1396 body 1397 |  |  | 0.552 |
| walker |  | 3774 | 14 | pub-item doc lede at src/kv/error.rs:5 |  |  | 0.552 |
| walker |  | 3794 | 20 | pub-item doc lede at src/kv/key.rs:7 |  |  | 0.552 |
| walker |  | 3814 | 20 | pub-item doc lede at src/kv/value.rs:11 |  |  | 0.553 |
| ns | 3906 |  | 291 | set_boxed_logger + set_logger | 2.10 |  | 0.535 |
| walker |  | 3914 | 100 | pub-item doc lede at src/lib.rs:475 |  |  | 0.570 |
| walker |  | 3989 | 75 | pub-item doc lede at src/lib.rs:1351 |  |  | 0.586 |
| walker |  | 4010 | 21 | pub-item doc lede at src/kv/source.rs:235 |  |  | 0.587 |
| ns | 4179 |  | 273 | logger() fn | 2.11 |  | 0.569 |
| walker |  | 4268 | 258 | macro_export names across src |  |  | 0.572 |
| walker |  | 4356 | 88 | pub-item doc lede at src/lib.rs:1003 |  |  | 0.572 |
| walker |  | 4400 | 44 | macro_export body at src/kv/value.rs:1129 |  |  | 0.572 |
| ns | 4437 |  | 258 | macro_rules! definition roster | 3.1 |  | 0.591 |
| walker |  | 4444 | 44 | macro_export body at src/kv/value.rs:1139 |  |  | 0.591 |
| walker |  | 4489 | 45 | macro_export body at src/kv/value.rs:1149 |  |  | 0.591 |
| walker |  | 4534 | 45 | macro_export body at src/kv/value.rs:1159 |  |  | 0.591 |
| walker |  | 4579 | 45 | macro_export body at src/kv/value.rs:1169 |  |  | 0.591 |
| walker |  | 4710 | 131 | mod/use plumbing in src/kv/mod.rs |  |  | 0.591 |
| walker |  | 4805 | 95 | pub-item doc lede at src/lib.rs:1200 |  |  | 0.591 |
| walker |  | 4927 | 122 | pub-item doc lede at src/lib.rs:636 |  |  | 0.624 |
| ns | 4939 |  | 502 | log! macro | 3.2 | 3.1 | 0.587 |
| walker |  | 5023 | 96 | pub-item doc lede at src/lib.rs:1529 |  |  | 0.587 |
| walker |  | 5122 | 99 | pub-item doc lede at src/lib.rs:1375 |  |  | 0.587 |
| walker |  | 5141 | 19 | README.md section #6 |  |  | 0.587 |
| walker |  | 5252 | 111 | pub-item doc lede at src/lib.rs:1420 |  |  | 0.600 |
| ns | 5555 |  | 616 | error! macro | 3.3 | 3.1 | 0.570 |
| walker |  | 5664 | 412 | [features] in Cargo.toml |  |  | 0.642 |
| ns | 5890 |  | 335 | log_enabled! macro | 3.4 | 3.1 | 0.625 |
| ns | 6413 |  | 523 | __log_value! kv capture-modifier dispatch | 3.5 | 3.1 | 0.593 |
| ns | 6838 |  | 425 | GlobalLogger + log/enabled/loc public fns | 4.1 |  | 0.565 |
| ns | 7119 |  | 281 | kv/mod.rs doc lede | 5.1 |  | 0.556 |
| ns | 7244 |  | 125 | kv/mod.rs capturing-modifier list | 5.2 |  | 0.550 |
| ns | 7449 |  | 205 | kv/mod.rs module decls + re-exports | 5.3 |  | 0.548 |
| walker |  | 7680 | 2016 | impl method sigs in src/lib.rs |  |  | 0.596 |
| walker |  | 7807 | 127 | pub-item doc lede at src/lib.rs:1478 |  |  | 0.596 |
| ns | 7822 |  | 373 | kv/key.rs: ToKey trait + Key struct | 5.4 |  | 0.581 |
| walker |  | 7890 | 83 | README.md section #1 |  |  | 0.581 |
| walker |  | 8014 | 124 | pub-item doc lede at src/lib.rs:1611 |  |  | 0.581 |
| walker |  | 8053 | 39 | pub-item doc lede at src/kv/value.rs:119 |  |  | 0.581 |
| ns | 8055 |  | 233 | kv/error.rs: Error struct, Inner enum, msg() constructor | 5.5 |  | 0.570 |
| ns | 8101 |  | 46 | kv/source.rs: Source trait method roster | 5.6 |  | 0.568 |
| ns | 8185 |  | 84 | kv/source.rs: VisitSource trait | 5.7 |  | 0.571 |
| walker |  | 8268 | 215 | pub-item doc lede at src/lib.rs:1396 |  |  | 0.571 |
| ns | 8288 |  | 103 | kv/value.rs: ToValue trait + Value struct | 5.8 |  | 0.576 |
| ns | 8429 |  | 141 | kv/value.rs: dependency-free Inner enum (data model) | 5.9 |  | 0.569 |
| walker |  | 8705 | 437 | pub item at src/kv/source.rs:51 |  |  | 0.573 |
| walker |  | 8708 | 3 | listing of '.github' |  |  | 0.573 |
| walker |  | 8712 | 4 | listing of '.github/workflows' |  |  | 0.573 |
| ns | 8713 |  | 284 | tests/macros.rs test-fn roster | 6.1 |  | 0.562 |
| walker |  | 8884 | 172 | impl method sigs in src/kv/key.rs |  |  | 0.568 |
| walker |  | 8912 | 28 | impl method sigs in src/kv/error.rs |  |  | 0.570 |
| ns | 8940 |  | 227 | tests/macros.rs: kv_common_value_types body | 6.2 | 6.1 | 0.560 |
| ns | 9027 |  | 87 | CI workflow job-name roster | 7.1 |  | 0.557 |
| ns | 9146 |  | 119 | test_max_level_features/Cargo.toml | 7.2 |  | 0.552 |
| walker |  | 9317 | 405 | [dependencies] in Cargo.toml |  |  | 0.559 |
| ns | 9393 |  | 247 | README.md lede | 8.1 |  | 0.557 |
| ns | 9632 |  | 239 | CHANGELOG.md: Unreleased + 0.4.29 | 8.2 |  | 0.551 |
| walker |  | 9652 | 335 | macro_export body at src/macros.rs:391 |  |  | 0.572 |
| walker |  | 9656 | 4 | listing of 'benches' |  |  | 0.573 |
| ns | 9676 |  | 44 | CHANGELOG.md: sampled older version headings | 8.3 |  | 0.572 |
| walker |  | 9749 | 93 | pub-item names surface in src/__private_api.rs |  |  | 0.573 |
| walker |  | 9749 | 0 | pub item at src/__private_api.rs:103 |  |  | 0.573 |
| walker |  | 9749 | 0 | pub item at src/__private_api.rs:108 |  |  | 0.573 |
| walker |  | 9757 | 8 | pub item at src/__private_api.rs:39 |  |  | 0.573 |
| walker |  | 9766 | 9 | pub item body at src/__private_api.rs:108 body 109 |  |  | 0.570 |
| ns | 9766 |  | 90 | rfcs/0296-structured-logging.md heading roster | 8.4 |  | 0.570 |
| walker |  | 9786 | 20 | pub item body at src/__private_api.rs:103 body 104 |  |  | 0.571 |
| walker |  | 9796 | 10 | pub-item doc lede at src/__private_api.rs:39 |  |  | 0.572 |
| walker |  | 9889 | 93 | pub item at src/__private_api.rs:84 |  |  | 0.578 |
| ns | 9923 |  | 157 | RFC 0296: Summary | 8.5 | 8.4 | 0.575 |
| ns | 9996 |  | 73 | src/serde.rs: serde impl roster | 9.1 |  | 0.573 |
