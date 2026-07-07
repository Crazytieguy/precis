Score(3000)=0.545 I=0.791 C=0.375 ns_rows≤3K=17/47 (reached=5 partial=2 missing=10)

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
| walker |  | 484 | 4 | listing of '.github/workflows' |  |  | 0.738 |
| walker |  | 548 | 64 | README.md section #0 |  |  | 0.738 |
| ns | 551 |  | 103 | Cargo.toml package identity (MSRV, edition, docs.rs features) | 1.8 |  | 0.703 |
| walker |  | 552 | 4 | listing of 'benches' |  |  | 0.709 |
| walker |  | 588 | 36 | manifest config in Cargo.toml |  |  | 0.750 |
| walker |  | 614 | 26 | pub-item names surface in src/kv/key.rs |  |  | 0.750 |
| walker |  | 645 | 31 | pub item at src/kv/key.rs:7 |  |  | 0.750 |
| walker |  | 672 | 27 | pub-item names surface in src/kv/source.rs |  |  | 0.750 |
| ns | 688 |  | 137 | Cargo.toml [features]: max_level_*/release_max_level_* | 1.9 |  | 0.663 |
| walker |  | 817 | 145 | mod/use plumbing in src/lib.rs |  |  | 0.663 |
| walker |  | 851 | 34 | pub item at src/kv/error.rs:5 |  |  | 0.663 |
| walker |  | 899 | 48 | pub item at src/kv/source.rs:235 |  |  | 0.664 |
| ns | 963 |  | 275 | Cargo.toml [features]: std/kv/kv_std/kv_sval/kv_serde | 1.10 |  | 0.588 |
| ns | 1080 |  | 117 | Cargo.toml [dependencies] | 1.11 |  | 0.571 |
| walker |  | 1145 | 246 | crate-doc lede in src/lib.rs |  |  | 0.650 |
| walker |  | 1185 | 40 | pub-item names surface in src/kv/value.rs |  |  | 0.650 |
| walker |  | 1216 | 31 | pub item at src/kv/value.rs:11 |  |  | 0.650 |
| walker |  | 1242 | 26 | pub item at src/kv/value.rs:119 |  |  | 0.650 |
| walker |  | 1309 | 67 | macro_export names across src/kv |  |  | 0.650 |
| walker |  | 1365 | 56 | pub item at src/kv/key.rs:37 |  |  | 0.650 |
| ns | 1493 |  | 413 | Level enum | 2.1 |  | 0.552 |
| walker |  | 1665 | 300 | pub-item names surface in src/lib.rs |  |  | 0.553 |
| walker |  | 1665 | 0 | pub item at src/lib.rs:1351 |  |  | 0.553 |
| walker |  | 1665 | 0 | pub item at src/lib.rs:1375 |  |  | 0.553 |
| walker |  | 1665 | 0 | pub item at src/lib.rs:1396 |  |  | 0.553 |
| walker |  | 1665 | 0 | pub item at src/lib.rs:1420 |  |  | 0.553 |
| walker |  | 1665 | 0 | pub item at src/lib.rs:1478 |  |  | 0.553 |
| walker |  | 1665 | 0 | pub item at src/lib.rs:1529 |  |  | 0.553 |
| walker |  | 1665 | 0 | pub item at src/lib.rs:1581 |  |  | 0.553 |
| walker |  | 1688 | 23 | pub item at src/lib.rs:1549 |  |  | 0.553 |
| walker |  | 1715 | 27 | pub item at src/lib.rs:1003 |  |  | 0.553 |
| walker |  | 1743 | 28 | pub item at src/lib.rs:1566 |  |  | 0.553 |
| walker |  | 1783 | 40 | pub item at src/lib.rs:1200 |  |  | 0.553 |
| ns | 1800 |  | 307 | LevelFilter enum | 2.2 |  | 0.503 |
| walker |  | 1834 | 51 | pub item at src/lib.rs:1158 |  |  | 0.503 |
| walker |  | 1940 | 106 | pub item at src/lib.rs:842 |  |  | 0.504 |
| ns | 1945 |  | 145 | Level & LevelFilter public method roster | 2.3 |  | 0.483 |
| ns | 2092 |  | 147 | FromStr for Level | 2.4 |  | 0.464 |
| walker |  | 2114 | 174 | pub item at src/lib.rs:636 |  |  | 0.504 |
| walker |  | 2126 | 12 | pub item body at src/lib.rs:1478 body 1479 |  |  | 0.504 |
| ns | 2419 |  | 327 | Record struct + KeyValues wrapper | 2.5 |  | 0.479 |
| walker |  | 2429 | 303 | pub item at src/lib.rs:475 |  |  | 0.555 |
| ns | 2557 |  | 138 | Record accessor method roster | 2.6 |  | 0.538 |
| walker |  | 2844 | 415 | pub item at src/lib.rs:1249 |  |  | 0.545 |
| ns | 3004 |  | 447 | Log trait | 2.7 |  | 0.587 |
| walker |  | 3122 | 278 | pub item at src/lib.rs:1611 |  |  | 0.587 |
| walker |  | 3130 | 8 | listing of 'tests' |  |  | 0.595 |
| walker |  | 3139 | 9 | listing of 'test_max_level_features' |  |  | 0.609 |
| walker |  | 3149 | 10 | listing of 'rfcs' |  |  | 0.618 |
| ns | 3280 |  | 276 | NopLogger + blanket impl for &T | 2.8 |  | 0.582 |
| walker |  | 3407 | 258 | macro_export names across src |  |  | 0.585 |
| walker |  | 3538 | 131 | mod/use plumbing in src/kv/mod.rs |  |  | 0.586 |
| walker |  | 3557 | 19 | README.md section #6 |  |  | 0.586 |
| walker |  | 3573 | 16 | pub item body at src/lib.rs:1420 body 1421 |  |  | 0.586 |
| ns | 3615 |  | 335 | set_max_level + max_level | 2.9 |  | 0.564 |
| ns | 3906 |  | 291 | set_boxed_logger + set_logger | 2.10 |  | 0.545 |
| walker |  | 3985 | 412 | [features] in Cargo.toml |  |  | 0.639 |
| ns | 4179 |  | 273 | logger() fn | 2.11 |  | 0.618 |
| ns | 4437 |  | 258 | macro_rules! definition roster | 3.1 |  | 0.636 |
| ns | 4939 |  | 502 | log! macro | 3.2 | 3.1 | 0.598 |
| ns | 5555 |  | 616 | error! macro | 3.3 | 3.1 | 0.568 |
| ns | 5890 |  | 335 | log_enabled! macro | 3.4 | 3.1 | 0.553 |
| walker |  | 6001 | 2016 | impl method sigs in src/lib.rs |  |  | 0.614 |
| walker |  | 6084 | 83 | README.md section #1 |  |  | 0.614 |
| walker |  | 6104 | 20 | pub item body at src/lib.rs:1351 body 1352 |  |  | 0.615 |
| ns | 6413 |  | 523 | __log_value! kv capture-modifier dispatch | 3.5 | 3.1 | 0.583 |
| walker |  | 6541 | 437 | pub item at src/kv/source.rs:51 |  |  | 0.583 |
| walker |  | 6713 | 172 | impl method sigs in src/kv/key.rs |  |  | 0.584 |
| walker |  | 6741 | 28 | impl method sigs in src/kv/error.rs |  |  | 0.584 |
| ns | 6838 |  | 425 | GlobalLogger + log/enabled/loc public fns | 4.1 |  | 0.557 |
| ns | 7119 |  | 281 | kv/mod.rs doc lede | 5.1 |  | 0.547 |
| walker |  | 7146 | 405 | [dependencies] in Cargo.toml |  |  | 0.556 |
| walker |  | 7163 | 17 | pub-item doc lede at src/lib.rs:1249 |  |  | 0.560 |
| ns | 7244 |  | 125 | kv/mod.rs capturing-modifier list | 5.2 |  | 0.554 |
| walker |  | 7256 | 93 | pub-item names surface in src/__private_api.rs |  |  | 0.555 |
| walker |  | 7256 | 0 | pub item at src/__private_api.rs:103 |  |  | 0.555 |
| walker |  | 7256 | 0 | pub item at src/__private_api.rs:108 |  |  | 0.555 |
| walker |  | 7264 | 8 | pub item at src/__private_api.rs:39 |  |  | 0.556 |
| walker |  | 7357 | 93 | pub item at src/__private_api.rs:84 |  |  | 0.562 |
| walker |  | 7418 | 61 | README.md section #5 |  |  | 0.562 |
| ns | 7449 |  | 205 | kv/mod.rs module decls + re-exports | 5.3 |  | 0.561 |
| ns | 7822 |  | 373 | kv/key.rs: ToKey trait + Key struct | 5.4 |  | 0.553 |
| ns | 8055 |  | 233 | kv/error.rs: Error struct, Inner enum, msg() constructor | 5.5 |  | 0.542 |
| ns | 8101 |  | 46 | kv/source.rs: Source trait method roster | 5.6 |  | 0.545 |
| ns | 8185 |  | 84 | kv/source.rs: VisitSource trait | 5.7 |  | 0.546 |
| ns | 8288 |  | 103 | kv/value.rs: ToValue trait + Value struct | 5.8 |  | 0.547 |
| walker |  | 8425 | 1007 | impl method sigs in src/kv/value.rs |  |  | 0.547 |
| ns | 8429 |  | 141 | kv/value.rs: dependency-free Inner enum (data model) | 5.9 |  | 0.541 |
| ns | 8713 |  | 284 | tests/macros.rs test-fn roster | 6.1 |  | 0.531 |
| ns | 8940 |  | 227 | tests/macros.rs: kv_common_value_types body | 6.2 | 6.1 | 0.522 |
| ns | 9027 |  | 87 | CI workflow job-name roster | 7.1 |  | 0.519 |
| ns | 9146 |  | 119 | test_max_level_features/Cargo.toml | 7.2 |  | 0.514 |
| walker |  | 9327 | 902 | pub item at src/kv/value.rs:462 |  |  | 0.514 |
| ns | 9393 |  | 247 | README.md lede | 8.1 |  | 0.513 |
| walker |  | 9400 | 73 | README.md section #2 |  |  | 0.513 |
| walker |  | 9410 | 10 | CHANGELOG.md section #0 |  |  | 0.513 |
| ns | 9632 |  | 239 | CHANGELOG.md: Unreleased + 0.4.29 | 8.2 |  | 0.507 |
| ns | 9676 |  | 44 | CHANGELOG.md: sampled older version headings | 8.3 |  | 0.507 |
| ns | 9766 |  | 90 | rfcs/0296-structured-logging.md heading roster | 8.4 |  | 0.504 |
| ns | 9923 |  | 157 | RFC 0296: Summary | 8.5 | 8.4 | 0.501 |
| ns | 9996 |  | 73 | src/serde.rs: serde impl roster | 9.1 |  | 0.499 |
