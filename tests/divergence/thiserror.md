scores: Sim=0.503 Reached=12/27 Early=1 Late=9 Partial=4 Missing=11 Used=9993/10000

## Tier rollup

| tier | batches | reached | partial | missing | avg_credit |
|-----:|--------:|--------:|--------:|--------:|-----------:|
| 1 | 3 | 3 | 0 | 0 | 0.93 |
| 2 | 8 | 7 | 1 | 0 | 0.91 |
| 3 | 3 | 0 | 2 | 1 | 0.47 |
| 4 | 4 | 0 | 0 | 4 | 0.02 |
| 5 | 3 | 1 | 0 | 2 | 0.33 |
| 6 | 2 | 0 | 0 | 2 | 0.00 |
| 7 | 2 | 0 | 0 | 2 | 0.20 |
| 8 | 2 | 1 | 1 | 0 | 0.79 |

## Arrival ledger (non-aligned or partial-credit NS batches)

| id | exp_t | reached_t | delta_t | credit | status | descriptor | nearby walker batch |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.1 | 48 | 81 | +33 | 0.80 | late | Crate-doc lede | README headline in README.md (t=81, 4 atoms) |
| 1.2 | 86 | 38 | -48 | 1.00 | early | Top-level repo listing |  |
| 1.3 | 158 | 1288 | +1130 | 1.00 | late | src/ + impl/src/ listings |  |
| 2.1 | 245 | 1119 | +874 | 0.88 | late | README example — enum head + first variant | README.md section #1 (t=1119, 7 atoms) |
| 2.2 | 339 | 1119 | +780 | 1.00 | late | README example — remaining variants | README.md section #1 (t=1119, 10 atoms) |
| 2.4 | 735 | 1226 | +491 | 0.82 | late | Cargo.toml — std/no_std feature + workspace | [features] in Cargo.toml (t=841, 14 atoms) |
| 2.5 | 961 | — | — | 0.65 | partial | Macro entry point — derive_error in impl/src/lib.rs | mod/use plumbing in impl/src/lib.rs (t=2545, 14 atoms) |
| 2.6 | 1319 | 6549 | +5230 | 1.00 | late | Crate-doc bullets — Display + format-shorthand summary | crate-doc body in src/lib.rs (t=6549, 21 atoms) |
| 2.7 | 1606 | 6549 | +4943 | 1.00 | late | Crate-doc bullets — From + source headlines | crate-doc body in src/lib.rs (t=6549, 26 atoms) |
| 2.8 | 1983 | 6549 | +4566 | 0.95 | late | Crate-doc bullets — Backtrace + transparent headlines | crate-doc body in src/lib.rs (t=6549, 50 atoms) |
| 3.1 | 2515 | — | — | 0.71 | partial | ast.rs — Input/Struct/Enum/Variant/Field/ContainerKind type defs | pub-item names surface in impl/src/ast.rs (t=1640, 12 atoms) |
| 3.2 | 3231 | — | — | 0.67 | partial | attr.rs — Attrs / Display / Source / From / Transparent / Fmt / Trait type defs | pub-item names surface in impl/src/attr.rs (t=2051, 14 atoms) |
| 3.3 | 3478 | — | — | 0.04 | missing | ast/attr/prop — public fn name locator | pub-item names surface in impl/src/attr.rs (t=2051, 2 atoms) |
| 4.1 | 3848 | — | — | 0.04 | missing | expand.rs — derive entry + try_expand | pub-item names surface in impl/src/expand.rs (t=1564, 2 atoms) |
| 4.2 | 4095 | — | — | 0.04 | missing | expand.rs / fmt.rs / generics.rs — public fn locator | pub item at impl/src/generics.rs:48 (t=1522, 4 atoms) |
| 4.3 | 4579 | — | — | 0.00 | missing | expand.rs — impl_struct source/transparent branches |  |
| 4.4 | 4854 | — | — | 0.00 | missing | expand.rs — from_initializer (#[from] body) |  |
| 5.1 | 5637 | — | — | 0.00 | missing | valid.rs — Struct + Enum + Variant validate (the rejection rules) |  |
| 5.2 | 6828 | — | — | 0.00 | missing | valid.rs — check_non_field_attrs / check_field_attrs (cross-field rules) |  |
| 5.3 | 7356 | 9903 | +2547 | 1.00 | late | tests/ui/ — trybuild compile-fail test listing |  |
| 6.1 | 7693 | — | — | 0.00 | missing | fmt.rs — expand_shorthand entry + state setup |  |
| 6.2 | 9072 | — | — | 0.00 | missing | fmt.rs — placeholder loop ({var}/{0}/{:?} mechanic) |  |
| 7.1 | 9392 | — | — | 0.19 | missing | src/lib.rs — module decls + cfg gates + private include! | mod/use plumbing in src/lib.rs (t=887, 6 atoms) |
| 7.2 | 9711 | — | — | 0.21 | missing | src/provide.rs + var.rs — runtime helpers | pub-item names surface in src/provide.rs (t=2893, 4 atoms) |
| 8.2 | 9966 | — | — | 0.58 | partial | test_source.rs — three source-shape examples | pub-item names surface in tests/test_source.rs (t=7713, 6 atoms) |

## Walker waste rollup (by descriptor pattern)

| n | off_tokens_total | pattern |
|--:|-----------------:|:--------|
| 11 | 2154 | README.md section #<n> |
| 2 | 419 | pub item at tests/test_expr.rs:<n> |
| 2 | 129 | pub item at tests/test_generics.rs:<n> |

## Walker waste (off-NS token spend ≥ 50)

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 1476 | 0.59 | 2481 | 6549 | crate-doc body in src/lib.rs |
| 476 | 1.00 | 476 | 7420 | README.md section #4 |
| 344 | 1.00 | 344 | 6916 | README.md section #10 |
| 300 | 1.00 | 300 | 9375 | plaintext config LICENSE-MIT |
| 295 | 1.00 | 295 | 9006 | pub item at tests/test_expr.rs:11 |
| 272 | 1.00 | 272 | 2845 | README.md section #12 |
| 267 | 1.00 | 267 | 680 | crate-doc lede in src/lib.rs |
| 222 | 1.00 | 222 | 4068 | README.md section #6 |
| 193 | 1.00 | 193 | 3846 | README.md section #8 |
| 170 | 1.00 | 170 | 3653 | README.md section #5 |
| 169 | 1.00 | 169 | 8470 | pub-item names surface in tests/test_generics.rs |
| 160 | 1.00 | 160 | 3483 | README.md section #7 |
| 127 | 1.00 | 127 | 3323 | README.md section #9 |
| 124 | 1.00 | 124 | 8031 | pub item at tests/test_expr.rs:41 |
| 122 | 1.00 | 122 | 3196 | [package] in impl/Cargo.toml |
| 95 | 1.00 | 95 | 8126 | pub-item names surface in tests/test_from.rs |
| 78 | 1.00 | 78 | 8711 | pub item at tests/test_generics.rs:74 |
| 75 | 0.70 | 107 | 1226 | [dependencies] in Cargo.toml |
| 74 | 1.00 | 74 | 3074 | README.md section #2 |
| 69 | 1.00 | 69 | 9075 | [package] in tests/no-std/Cargo.toml |
| 66 | 1.00 | 66 | 7486 | pub item at src/display.rs:6 |
| 64 | 1.00 | 64 | 205 | README.md section #0 |
| 57 | 1.00 | 57 | 7844 | pub item at tests/test_path.rs:35 |
| 52 | 1.00 | 52 | 3000 | README.md section #11 |
| 51 | 1.00 | 51 | 8633 | pub item at tests/test_generics.rs:175 |
