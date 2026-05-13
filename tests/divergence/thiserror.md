scores: Score(3000)=0.610 ns_rows≤3K=12/27 (reached=4 partial=3 missing=5)

## Per-budget scores

| B | A_B | I(B) | C(B) | compl(B) | Score(B) | walker_used |
|--:|----:|-----:|-----:|---------:|---------:|------------:|
| 1000 | 109 | 0.815 | 0.540 | 0.828 | 0.663 | 988 |
| 1442 | 130 | 0.814 | 0.557 | 0.942 | 0.673 | 1416 |
| 2080 | 168 | 0.812 | 0.455 | 0.964 | 0.608 | 2075 |
| 3000 | 220 | 0.796 | 0.467 | 0.928 | 0.610 | 2991 |
| 4327 | 370 | 0.825 | 0.538 | 0.806 | 0.666 | 4147 |
| 6240 | 506 | 0.787 | 0.393 | 0.806 | 0.556 | 4508 |
| 9000 | 709 | 0.818 | 0.384 | 0.801 | 0.561 | 8974 |

## Top opportunities

### Additive (close partial / missing rows)

| intervention | rows | gap@1k | gap@3k | gap@9k | evidence | top row ids |
|:-------------|-----:|-------:|-------:|-------:|:---------|:------------|
| split wrong-slice walker batches | 15 | 1.80 | 1.41 | 1.14 | nearby candidates have low exact atom overlap | 5.2, 5.1, 3.2, 6.2, 3.1, ... |

### Subtractive (suppress consistently off-NS batches)

| pattern | batches | freed@1k | freed@3k | freed@9k | evidence | top batch ids |
|:--------|--------:|---------:|---------:|---------:|:---------|:--------------|
| crate-doc lede in src/lib.rs | 1 | 267 | 267 | 267 | off_3k=267 | crate-doc lede in src/lib.rs |
| mod/use plumbing in impl/src/unraw.rs | 1 | 0 | 102 | 102 | off_3k=102 | mod/use plumbing in impl/src/unraw.rs |
| mod/use plumbing in impl/src/generics.rs | 1 | 0 | 101 | 101 | off_3k=101 | mod/use plumbing in impl/src/generics.rs |
| [dependencies] in Cargo.toml | 1 | 0 | 75 | 75 | off_3k=75 | [dependencies] in Cargo.toml |
| mod/use plumbing in impl/src/scan_expr.rs | 1 | 0 | 75 | 75 | off_3k=75 | mod/use plumbing in impl/src/scan_expr.rs |

Top missed paths (NS rows ≤ 3K): src/lib.rs (3 rows, 59 atoms), impl/src/ast.rs (1 row, 52 atoms), README.md (3 rows, 23 atoms), impl/src/lib.rs (1 row, 23 atoms)

## Arrival ledger by diagnosis

### wrong-slice / granularity

| id | ns_t | credit | status | descriptor | candidate hint |
|----|-----:|-------:|:-------|:-----------|:---------------|
| 2.5 | 961 | 0.74 | partial | Macro entry point — derive_error in impl/src/lib.rs | [scheduled bbox exact=14/23] mod/use plumbing in impl/src/lib.rs (t=1367, 14 atoms) |
| 3.1 | 2515 | 0.71 | partial | ast.rs — Input/Struct/Enum/Variant/Field/ContainerKind type defs | [scheduled bbox exact=12/52] pub-item names surface in impl/src/ast.rs (t=1492, 12 atoms) |
| 3.2 | 3231 | 0.67 | missing | attr.rs — Attrs / Display / Source / From / Transparent / Fmt / Trait type defs | [scheduled bbox exact=14/67] pub-item names surface in impl/src/attr.rs (t=2025, 14 atoms) |
| 3.3 | 3478 | 0.04 | missing | ast/attr/prop — public fn name locator | [scheduled bbox exact=13/28] impl method sigs in impl/src/prop.rs (t=8336, 29 atoms) |
| 4.1 | 3848 | 0.28 | missing | expand.rs — derive entry + try_expand | [scheduled bbox exact=10/29] mod/use plumbing in impl/src/expand.rs (t=3735, 10 atoms) |
| 4.2 | 4095 | 0.04 | missing | expand.rs / fmt.rs / generics.rs — public fn locator | [scheduled bbox exact=5/26] impl method sigs in impl/src/generics.rs (t=7464, 10 atoms) |
| 4.3 | 4579 | 0.00 | missing | expand.rs — impl_struct source/transparent branches | [scheduled same-file] mod/use plumbing in impl/src/expand.rs (t=3735, 10 atoms) |
| 4.4 | 4854 | 0.00 | missing | expand.rs — from_initializer (#[from] body) | [scheduled same-file] mod/use plumbing in impl/src/expand.rs (t=3735, 10 atoms) |
| 5.1 | 5637 | 0.00 | missing | valid.rs — Struct + Enum + Variant validate (the rejection rules) | [scheduled bbox exact=9/76] impl method sigs in impl/src/valid.rs (t=7330, 9 atoms) |
| 5.2 | 6828 | 0.00 | missing | valid.rs — check_non_field_attrs / check_field_attrs (cross-field rules) | [scheduled same-file] impl method sigs in impl/src/valid.rs (t=7330, 15 atoms) |
| 6.1 | 7693 | 0.00 | missing | fmt.rs — expand_shorthand entry + state setup | [scheduled bbox exact=2/26] impl method sigs in impl/src/fmt.rs (t=3588, 2 atoms) |
| 6.2 | 9072 | 0.00 | missing | fmt.rs — placeholder loop ({var}/{0}/{:?} mechanic) | [scheduled same-file] mod/use plumbing in impl/src/fmt.rs (t=4082, 13 atoms) |
| 7.1 | 9392 | 0.19 | missing | src/lib.rs — module decls + cfg gates + private include! | [scheduled bbox exact=6/31] mod/use plumbing in src/lib.rs (t=276, 6 atoms) |
| 7.2 | 9711 | 0.14 | missing | src/provide.rs + var.rs — runtime helpers | [scheduled bbox exact=4/29] pub-item names surface in src/provide.rs (t=2688, 4 atoms) |
| 8.2 | 9966 | 0.00 | missing | test_source.rs — three source-shape examples | [scheduled bbox exact=6/19] pub-item names surface in tests/test_source.rs (t=7536, 6 atoms) |

### fs/listing

| id | ns_t | credit | status | descriptor | candidate hint |
|----|-----:|-------:|:-------|:-----------|:---------------|
| 5.3 | 7356 | 0.00 | missing | tests/ui/ — trybuild compile-fail test listing | fs-only |
| 8.1 | 9792 | 0.88 | partial | tests/ listing | fs-only |

### mixed/unknown

| id | ns_t | credit | status | descriptor | candidate hint |
|----|-----:|-------:|:-------|:-----------|:---------------|
| 1.1 | 48 | 0.80 | partial | Crate-doc lede | [scheduled bbox exact=4/5] README headline in README.md (t=81, 4 atoms) |
| 2.1 | 245 | 0.00 | missing | README example — enum head + first variant | [scheduled bbox exact=7/8] README.md section #1 (t=3390, 7 atoms) |
| 2.2 | 339 | 0.00 | missing | README example — remaining variants | [scheduled bbox exact=10/10] README.md section #1 (t=3390, 10 atoms) |
| 2.6 | 1319 | 0.00 | missing | Crate-doc bullets — Display + format-shorthand summary | [scheduled bbox exact=21/21] crate-doc body in src/lib.rs (t=6989, 21 atoms) |
| 2.7 | 1606 | 0.00 | missing | Crate-doc bullets — From + source headlines | [scheduled bbox exact=17/17] crate-doc body in src/lib.rs (t=6989, 26 atoms) |
| 2.8 | 1983 | 0.00 | missing | Crate-doc bullets — Backtrace + transparent headlines | [scheduled bbox exact=20/21] crate-doc body in src/lib.rs (t=6989, 50 atoms) |

Top wasted paths (off-NS at 3K): impl/src/attr.rs (375t, 4 batches), src/lib.rs (267t, 1 batch), impl/src/unraw.rs (102t, 1 batch), impl/src/generics.rs (101t, 1 batch), impl/src/expand.rs (95t, 1 batch), Cargo.toml (75t, 1 batch), impl/src/scan_expr.rs (75t, 1 batch), tests (72t, 1 batch), +3 more

## Walker waste rollup (by descriptor pattern)

| n | off_3k_total | pattern |
|--:|-------------:|:--------|
| 3 | 265 | pub item at impl/src/attr.rs:<n> |

## Walker waste (walker_t ≤ 3000, off_3k ≥ 50)

| off_3k | off_3k_ratio | off_any | cost | walker_t | batch |
|-------:|-------------:|--------:|-----:|---------:|:------|
| 267 | 1.00 | 267 | 267 | 459 | crate-doc lede in src/lib.rs |
| 110 | 1.00 | 2 | 110 | 1915 | pub-item names surface in impl/src/attr.rs |
| 106 | 1.00 | 0 | 106 | 2285 | pub item at impl/src/attr.rs:21 |
| 102 | 1.00 | 102 | 102 | 2991 | mod/use plumbing in impl/src/unraw.rs |
| 101 | 1.00 | 101 | 101 | 2890 | mod/use plumbing in impl/src/generics.rs |
| 95 | 1.00 | 0 | 95 | 2567 | pub item body at impl/src/expand.rs:12 body 13 |
| 88 | 1.00 | 0 | 88 | 2197 | pub item at impl/src/attr.rs:11 |
| 75 | 0.70 | 75 | 107 | 1793 | [dependencies] in Cargo.toml |
| 75 | 1.00 | 75 | 75 | 2743 | mod/use plumbing in impl/src/scan_expr.rs |
| 72 | 1.00 | 0 | 72 | 2818 | listing of 'tests' |
| 240 | — | — | — | — | +4 more rows |
