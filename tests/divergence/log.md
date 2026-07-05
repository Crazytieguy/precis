Score(3000)=0.518 I=0.757 C=0.355 ns_rows≤3K=17/47 (reached=5 partial=1 missing=11)

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
| walker |  | 430 | 21 | listing of 'src' |  |  | 0.743 |
| ns | 448 |  | 155 | Crate-doc lede (lib.rs) | 1.7 |  | 0.661 |
| walker |  | 450 | 20 | listing of 'src/kv' |  |  | 0.736 |
| walker |  | 541 | 91 | crate-doc lede in src/kv/mod.rs |  |  | 0.736 |
| ns | 551 |  | 103 | Cargo.toml package identity (MSRV, edition, docs.rs features) | 1.8 |  | 0.702 |
| walker |  | 567 | 26 | pub-item names surface in src/kv/key.rs |  |  | 0.702 |
| walker |  | 598 | 31 | pub item at src/kv/key.rs:7 |  |  | 0.702 |
| walker |  | 625 | 27 | pub-item names surface in src/kv/source.rs |  |  | 0.702 |
| ns | 688 |  | 137 | Cargo.toml [features]: max_level_*/release_max_level_* | 1.9 |  | 0.620 |
| walker |  | 770 | 145 | mod/use plumbing in src/lib.rs |  |  | 0.620 |
| walker |  | 804 | 34 | pub item at src/kv/error.rs:5 |  |  | 0.620 |
| walker |  | 852 | 48 | pub item at src/kv/source.rs:235 |  |  | 0.621 |
| ns | 963 |  | 275 | Cargo.toml [features]: std/kv/kv_std/kv_sval/kv_serde | 1.10 |  | 0.551 |
| ns | 1080 |  | 117 | Cargo.toml [dependencies] | 1.11 |  | 0.535 |
| walker |  | 1098 | 246 | crate-doc lede in src/lib.rs |  |  | 0.615 |
| walker |  | 1138 | 40 | pub-item names surface in src/kv/value.rs |  |  | 0.615 |
| walker |  | 1169 | 31 | pub item at src/kv/value.rs:11 |  |  | 0.615 |
| walker |  | 1195 | 26 | pub item at src/kv/value.rs:119 |  |  | 0.615 |
| walker |  | 1262 | 67 | macro_export names across src/kv |  |  | 0.615 |
| walker |  | 1318 | 56 | pub item at src/kv/key.rs:37 |  |  | 0.616 |
| ns | 1493 |  | 413 | Level enum | 2.1 |  | 0.523 |
| walker |  | 1618 | 300 | pub-item names surface in src/lib.rs |  |  | 0.523 |
| walker |  | 1618 | 0 | pub item at src/lib.rs:1351 |  |  | 0.523 |
| walker |  | 1618 | 0 | pub item at src/lib.rs:1375 |  |  | 0.523 |
| walker |  | 1618 | 0 | pub item at src/lib.rs:1396 |  |  | 0.523 |
| walker |  | 1618 | 0 | pub item at src/lib.rs:1420 |  |  | 0.523 |
| walker |  | 1618 | 0 | pub item at src/lib.rs:1478 |  |  | 0.523 |
| walker |  | 1618 | 0 | pub item at src/lib.rs:1529 |  |  | 0.523 |
| walker |  | 1618 | 0 | pub item at src/lib.rs:1581 |  |  | 0.523 |
| walker |  | 1641 | 23 | pub item at src/lib.rs:1549 |  |  | 0.523 |
| walker |  | 1668 | 27 | pub item at src/lib.rs:1003 |  |  | 0.523 |
| walker |  | 1696 | 28 | pub item at src/lib.rs:1566 |  |  | 0.523 |
| walker |  | 1708 | 12 | pub item body at src/lib.rs:1478 body 1479 |  |  | 0.523 |
| walker |  | 1748 | 40 | pub item at src/lib.rs:1200 |  |  | 0.523 |
| walker |  | 1764 | 16 | pub item body at src/lib.rs:1420 body 1421 |  |  | 0.523 |
| ns | 1800 |  | 307 | LevelFilter enum | 2.2 |  | 0.476 |
| walker |  | 1815 | 51 | pub item at src/lib.rs:1158 |  |  | 0.476 |
| walker |  | 1835 | 20 | pub item body at src/lib.rs:1351 body 1352 |  |  | 0.476 |
| walker |  | 1941 | 106 | pub item at src/lib.rs:842 |  |  | 0.477 |
| ns | 1945 |  | 145 | Level & LevelFilter public method roster | 2.3 |  | 0.457 |
| walker |  | 1961 | 20 | pub-item doc lede at src/lib.rs:1158 |  |  | 0.457 |
| walker |  | 1982 | 21 | pub-item doc lede at src/lib.rs:842 |  |  | 0.457 |
| ns | 2092 |  | 147 | FromStr for Level | 2.4 |  | 0.440 |
| walker |  | 2156 | 174 | pub item at src/lib.rs:636 |  |  | 0.481 |
| walker |  | 2248 | 92 | pub item body at src/lib.rs:1375 body 1376 |  |  | 0.481 |
| ns | 2419 |  | 327 | Record struct + KeyValues wrapper | 2.5 |  | 0.457 |
| walker |  | 2551 | 303 | pub item at src/lib.rs:475 |  |  | 0.535 |
| ns | 2557 |  | 138 | Record accessor method roster | 2.6 |  | 0.518 |
| walker |  | 2591 | 40 | pub-item doc lede at src/lib.rs:1581 |  |  | 0.518 |
| walker |  | 2637 | 46 | pub-item doc lede at src/lib.rs:1549 |  |  | 0.518 |
| ns | 3004 |  | 447 | Log trait | 2.7 |  | 0.478 |
| walker |  | 3052 | 415 | pub item at src/lib.rs:1249 |  |  | 0.570 |
| walker |  | 3069 | 17 | pub-item doc lede at src/lib.rs:1249 |  |  | 0.578 |
| ns | 3280 |  | 276 | NopLogger + blanket impl for &T | 2.8 |  | 0.544 |
| walker |  | 3347 | 278 | pub item at src/lib.rs:1611 |  |  | 0.544 |
| walker |  | 3496 | 149 | pub item body at src/lib.rs:1529 body 1530 |  |  | 0.544 |
| walker |  | 3559 | 63 | pub-item doc lede at src/lib.rs:1566 |  |  | 0.544 |
| ns | 3615 |  | 335 | set_max_level + max_level | 2.9 |  | 0.524 |
| walker |  | 3724 | 165 | pub item body at src/lib.rs:1396 body 1397 |  |  | 0.540 |
| walker |  | 3738 | 14 | pub-item doc lede at src/kv/error.rs:5 |  |  | 0.540 |
| walker |  | 3758 | 20 | pub-item doc lede at src/kv/key.rs:7 |  |  | 0.540 |
| walker |  | 3778 | 20 | pub-item doc lede at src/kv/value.rs:11 |  |  | 0.540 |
| walker |  | 3878 | 100 | pub-item doc lede at src/lib.rs:475 |  |  | 0.577 |
| ns | 3906 |  | 291 | set_boxed_logger + set_logger | 2.10 |  | 0.558 |
| walker |  | 3953 | 75 | pub-item doc lede at src/lib.rs:1351 |  |  | 0.574 |
| walker |  | 3974 | 21 | pub-item doc lede at src/kv/source.rs:235 |  |  | 0.575 |
| ns | 4179 |  | 273 | logger() fn | 2.11 |  | 0.558 |
| walker |  | 4232 | 258 | macro_export names across src |  |  | 0.561 |
| walker |  | 4320 | 88 | pub-item doc lede at src/lib.rs:1003 |  |  | 0.561 |
| walker |  | 4364 | 44 | macro_export body at src/kv/value.rs:1129 |  |  | 0.561 |
| walker |  | 4408 | 44 | macro_export body at src/kv/value.rs:1139 |  |  | 0.561 |
| ns | 4437 |  | 258 | macro_rules! definition roster | 3.1 |  | 0.580 |
| walker |  | 4453 | 45 | macro_export body at src/kv/value.rs:1149 |  |  | 0.580 |
| walker |  | 4498 | 45 | macro_export body at src/kv/value.rs:1159 |  |  | 0.580 |
| walker |  | 4543 | 45 | macro_export body at src/kv/value.rs:1169 |  |  | 0.580 |
| walker |  | 4674 | 131 | mod/use plumbing in src/kv/mod.rs |  |  | 0.581 |
| walker |  | 4769 | 95 | pub-item doc lede at src/lib.rs:1200 |  |  | 0.581 |
| walker |  | 4891 | 122 | pub-item doc lede at src/lib.rs:636 |  |  | 0.614 |
| ns | 4939 |  | 502 | log! macro | 3.2 | 3.1 | 0.577 |
| walker |  | 4987 | 96 | pub-item doc lede at src/lib.rs:1529 |  |  | 0.577 |
| walker |  | 5086 | 99 | pub-item doc lede at src/lib.rs:1375 |  |  | 0.577 |
| walker |  | 5105 | 19 | README.md section #6 |  |  | 0.577 |
| walker |  | 5216 | 111 | pub-item doc lede at src/lib.rs:1420 |  |  | 0.590 |
| ns | 5555 |  | 616 | error! macro | 3.3 | 3.1 | 0.560 |
| walker |  | 5630 | 414 | [features] in Cargo.toml |  |  | 0.632 |
| ns | 5890 |  | 335 | log_enabled! macro | 3.4 | 3.1 | 0.616 |
| ns | 6413 |  | 523 | __log_value! kv capture-modifier dispatch | 3.5 | 3.1 | 0.584 |
| ns | 6838 |  | 425 | GlobalLogger + log/enabled/loc public fns | 4.1 |  | 0.557 |
| ns | 7119 |  | 281 | kv/mod.rs doc lede | 5.1 |  | 0.548 |
| ns | 7244 |  | 125 | kv/mod.rs capturing-modifier list | 5.2 |  | 0.542 |
| ns | 7449 |  | 205 | kv/mod.rs module decls + re-exports | 5.3 |  | 0.540 |
| walker |  | 7646 | 2016 | impl method sigs in src/lib.rs |  |  | 0.588 |
| walker |  | 7773 | 127 | pub-item doc lede at src/lib.rs:1478 |  |  | 0.588 |
| ns | 7822 |  | 373 | kv/key.rs: ToKey trait + Key struct | 5.4 |  | 0.573 |
| walker |  | 7856 | 83 | README.md section #1 |  |  | 0.573 |
| walker |  | 7980 | 124 | pub-item doc lede at src/lib.rs:1611 |  |  | 0.573 |
| walker |  | 8019 | 39 | pub-item doc lede at src/kv/value.rs:119 |  |  | 0.573 |
| ns | 8055 |  | 233 | kv/error.rs: Error struct, Inner enum, msg() constructor | 5.5 |  | 0.562 |
| ns | 8101 |  | 46 | kv/source.rs: Source trait method roster | 5.6 |  | 0.561 |
| ns | 8185 |  | 84 | kv/source.rs: VisitSource trait | 5.7 |  | 0.563 |
| walker |  | 8234 | 215 | pub-item doc lede at src/lib.rs:1396 |  |  | 0.563 |
| ns | 8288 |  | 103 | kv/value.rs: ToValue trait + Value struct | 5.8 |  | 0.568 |
| ns | 8429 |  | 141 | kv/value.rs: dependency-free Inner enum (data model) | 5.9 |  | 0.562 |
| walker |  | 8671 | 437 | pub item at src/kv/source.rs:51 |  |  | 0.566 |
| walker |  | 8674 | 3 | listing of '.github' |  |  | 0.566 |
| walker |  | 8678 | 4 | listing of '.github/workflows' |  |  | 0.566 |
| ns | 8713 |  | 284 | tests/macros.rs test-fn roster | 6.1 |  | 0.555 |
| walker |  | 8850 | 172 | impl method sigs in src/kv/key.rs |  |  | 0.561 |
| walker |  | 8878 | 28 | impl method sigs in src/kv/error.rs |  |  | 0.563 |
| ns | 8940 |  | 227 | tests/macros.rs: kv_common_value_types body | 6.2 | 6.1 | 0.553 |
| ns | 9027 |  | 87 | CI workflow job-name roster | 7.1 |  | 0.550 |
| ns | 9146 |  | 119 | test_max_level_features/Cargo.toml | 7.2 |  | 0.545 |
| walker |  | 9283 | 405 | [dependencies] in Cargo.toml |  |  | 0.552 |
| ns | 9393 |  | 247 | README.md lede | 8.1 |  | 0.550 |
| walker |  | 9618 | 335 | macro_export body at src/macros.rs:391 |  |  | 0.571 |
| walker |  | 9622 | 4 | listing of 'benches' |  |  | 0.572 |
| ns | 9632 |  | 239 | CHANGELOG.md: Unreleased + 0.4.29 | 8.2 |  | 0.566 |
| ns | 9676 |  | 44 | CHANGELOG.md: sampled older version headings | 8.3 |  | 0.565 |
| walker |  | 9715 | 93 | pub-item names surface in src/__private_api.rs |  |  | 0.566 |
| walker |  | 9715 | 0 | pub item at src/__private_api.rs:103 |  |  | 0.566 |
| walker |  | 9715 | 0 | pub item at src/__private_api.rs:108 |  |  | 0.566 |
| walker |  | 9723 | 8 | pub item at src/__private_api.rs:39 |  |  | 0.567 |
| walker |  | 9732 | 9 | pub item body at src/__private_api.rs:108 body 109 |  |  | 0.567 |
| walker |  | 9752 | 20 | pub item body at src/__private_api.rs:103 body 104 |  |  | 0.568 |
| walker |  | 9762 | 10 | pub-item doc lede at src/__private_api.rs:39 |  |  | 0.568 |
| ns | 9766 |  | 90 | rfcs/0296-structured-logging.md heading roster | 8.4 |  | 0.565 |
| walker |  | 9855 | 93 | pub item at src/__private_api.rs:84 |  |  | 0.572 |
| ns | 9923 |  | 157 | RFC 0296: Summary | 8.5 | 8.4 | 0.569 |
| ns | 9996 |  | 73 | src/serde.rs: serde impl roster | 9.1 |  | 0.567 |
