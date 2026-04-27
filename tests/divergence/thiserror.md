scores: Sim=0.478 Reached=12/27 Early=2 Late=9 Partial=4 Missing=11 Used=9993/10000

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
| 1.3 | 158 | 1105 | +947 | 1.00 | late | src/ + impl/src/ listings |  |
| 2.1 | 245 | 2648 | +2403 | 0.88 | late | README example — enum head + first variant | README.md section #1 (t=2648, 7 atoms) |
| 2.2 | 339 | 2648 | +2309 | 1.00 | late | README example — remaining variants | README.md section #1 (t=2648, 10 atoms) |
| 2.4 | 735 | 1043 | +308 | 0.82 | late | Cargo.toml — std/no_std feature + workspace | [features] in Cargo.toml (t=841, 14 atoms) |
| 2.5 | 961 | — | — | 0.65 | partial | Macro entry point — derive_error in impl/src/lib.rs | mod/use plumbing in impl/src/lib.rs (t=2362, 14 atoms) |
| 2.6 | 1319 | 5603 | +4284 | 1.00 | late | Crate-doc bullets — Display + format-shorthand summary | crate-doc body in src/lib.rs (t=5603, 21 atoms) |
| 2.7 | 1606 | 5603 | +3997 | 1.00 | late | Crate-doc bullets — From + source headlines | crate-doc body in src/lib.rs (t=5603, 26 atoms) |
| 2.8 | 1983 | 5603 | +3620 | 0.95 | late | Crate-doc bullets — Backtrace + transparent headlines | crate-doc body in src/lib.rs (t=5603, 50 atoms) |
| 3.1 | 2515 | — | — | 0.71 | partial | ast.rs — Input/Struct/Enum/Variant/Field/ContainerKind type defs | pub-item names surface in impl/src/ast.rs (t=1457, 12 atoms) |
| 3.2 | 3231 | — | — | 0.67 | partial | attr.rs — Attrs / Display / Source / From / Transparent / Fmt / Trait type defs | pub-item names surface in impl/src/attr.rs (t=1868, 14 atoms) |
| 3.3 | 3478 | — | — | 0.04 | missing | ast/attr/prop — public fn name locator | pub-item names surface in impl/src/attr.rs (t=1868, 2 atoms) |
| 4.1 | 3848 | — | — | 0.04 | missing | expand.rs — derive entry + try_expand | pub-item names surface in impl/src/expand.rs (t=1381, 2 atoms) |
| 4.2 | 4095 | — | — | 0.04 | missing | expand.rs / fmt.rs / generics.rs — public fn locator | pub item at impl/src/generics.rs:48 (t=1339, 4 atoms) |
| 4.3 | 4579 | — | — | 0.00 | missing | expand.rs — impl_struct source/transparent branches |  |
| 4.4 | 4854 | — | — | 0.00 | missing | expand.rs — from_initializer (#[from] body) |  |
| 5.1 | 5637 | — | — | 0.00 | missing | valid.rs — Struct + Enum + Variant validate (the rejection rules) |  |
| 5.2 | 6828 | — | — | 0.00 | missing | valid.rs — check_non_field_attrs / check_field_attrs (cross-field rules) |  |
| 5.3 | 7356 | 9903 | +2547 | 1.00 | late | tests/ui/ — trybuild compile-fail test listing |  |
| 6.1 | 7693 | — | — | 0.00 | missing | fmt.rs — expand_shorthand entry + state setup |  |
| 6.2 | 9072 | — | — | 0.00 | missing | fmt.rs — placeholder loop ({var}/{0}/{:?} mechanic) |  |
| 7.1 | 9392 | — | — | 0.19 | missing | src/lib.rs — module decls + cfg gates + private include! | mod/use plumbing in src/lib.rs (t=887, 6 atoms) |
| 7.2 | 9711 | — | — | 0.21 | missing | src/provide.rs + var.rs — runtime helpers | pub-item names surface in src/provide.rs (t=2410, 4 atoms) |
| 8.1 | 9792 | 6738 | -3054 | 1.00 | early | tests/ listing |  |
| 8.2 | 9966 | — | — | 0.58 | partial | test_source.rs — three source-shape examples | pub-item names surface in tests/test_source.rs (t=6893, 6 atoms) |

## Walker waste rollup (by descriptor pattern)

| n | off_tokens_total | pattern |
|--:|-----------------:|:--------|
| 11 | 2154 | README.md section #<n> |
| 2 | 419 | pub item at tests/test_expr.rs:<n> |
| 2 | 129 | pub item at tests/test_generics.rs:<n> |

## Walker waste (off-NS token spend ≥ 50)

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 1476 | 0.59 | 2481 | 5603 | crate-doc body in src/lib.rs |
| 476 | 1.00 | 476 | 9006 | README.md section #4 |
| 344 | 1.00 | 344 | 7825 | README.md section #10 |
| 300 | 1.00 | 300 | 9375 | plaintext config LICENSE-MIT |
| 295 | 1.00 | 295 | 8530 | pub item at tests/test_expr.rs:11 |
| 272 | 1.00 | 272 | 3070 | README.md section #12 |
| 267 | 1.00 | 267 | 680 | crate-doc lede in src/lib.rs |
| 222 | 1.00 | 222 | 6666 | README.md section #6 |
| 193 | 1.00 | 193 | 6444 | README.md section #8 |
| 170 | 1.00 | 170 | 6185 | README.md section #5 |
| 169 | 1.00 | 169 | 7994 | pub-item names surface in tests/test_generics.rs |
| 160 | 1.00 | 160 | 6015 | README.md section #7 |
| 127 | 1.00 | 127 | 5855 | README.md section #9 |
| 124 | 1.00 | 124 | 7211 | pub item at tests/test_expr.rs:41 |
| 122 | 1.00 | 122 | 2798 | [package] in impl/Cargo.toml |
| 95 | 1.00 | 95 | 7306 | pub-item names surface in tests/test_from.rs |
| 78 | 1.00 | 78 | 8235 | pub item at tests/test_generics.rs:74 |
| 75 | 0.70 | 107 | 1043 | [dependencies] in Cargo.toml |
| 74 | 1.00 | 74 | 5677 | README.md section #2 |
| 69 | 1.00 | 69 | 9075 | [package] in tests/no-std/Cargo.toml |
| 66 | 1.00 | 66 | 6251 | pub item at src/display.rs:6 |
| 64 | 1.00 | 64 | 205 | README.md section #0 |
| 57 | 1.00 | 57 | 7024 | pub item at tests/test_path.rs:35 |
| 52 | 1.00 | 52 | 3122 | README.md section #11 |
| 51 | 1.00 | 51 | 8157 | pub item at tests/test_generics.rs:175 |
