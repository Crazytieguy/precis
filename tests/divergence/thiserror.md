Score(3000)=0.548 I=0.820 C=0.366 ns_rows≤3K=21/45 (reached=9 partial=2 missing=10)

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 38 | 38 | listing of '.' |  |  | 1.000 |
| ns | 38 |  | 38 | Root directory listing | 1.1 |  | 1.000 |
| walker |  | 85 | 47 | README headline in README.md |  |  | 1.000 |
| ns | 110 |  | 72 | src/ and impl/src/ directory listings | 1.2 |  | 0.569 |
| walker |  | 114 | 29 | headings outline in README.md |  |  | 0.569 |
| ns | 191 |  | 81 | tests/ and tests/no-std/ directory listings | 1.3 |  | 0.434 |
| ns | 207 |  | 16 | build/, .github/, .github/workflows/ listings | 1.4 |  | 0.412 |
| ns | 257 |  | 50 | Cargo.toml — package identity (name/version/authors) | 1.5 |  | 0.393 |
| walker |  | 301 | 187 | [package] in Cargo.toml |  |  | 0.471 |
| ns | 370 |  | 113 | Cargo.toml — categories/description/docs/edition/keywords/license/repo/rust-version | 1.6 |  | 0.529 |
| walker |  | 373 | 72 | README.md section #0 |  |  | 0.529 |
| ns | 391 |  | 21 | Cargo.toml — [features] header | 1.7 |  | 0.514 |
| walker |  | 398 | 25 | listing of 'src' |  |  | 0.547 |
| walker |  | 460 | 62 | mod/use plumbing in src/lib.rs |  |  | 0.548 |
| walker |  | 479 | 19 | listing of 'impl' |  |  | 0.548 |
| ns | 536 |  | 145 | Cargo.toml — std feature doc comment | 1.8 |  | 0.494 |
| ns | 568 |  | 32 | Cargo.toml — [dependencies] | 1.9 |  | 0.487 |
| ns | 592 |  | 24 | Cargo.toml — [workspace] | 1.10 |  | 0.497 |
| walker |  | 645 | 166 | [features] in Cargo.toml |  |  | 0.635 |
| ns | 671 |  | 79 | impl/Cargo.toml — package identity (name/version/authors/description/edition) | 1.11 |  | 0.604 |
| ns | 744 |  | 73 | impl/Cargo.toml — [lib]/[dependencies] | 1.12 |  | 0.576 |
| ns | 835 |  | 91 | impl/src/lib.rs — module list | 1.13 |  | 0.536 |
| walker |  | 912 | 267 | crate-doc lede in src/lib.rs |  |  | 0.537 |
| walker |  | 984 | 72 | listing of 'tests' |  |  | 0.636 |
| walker |  | 1031 | 47 | listing of 'impl/src' |  |  | 0.777 |
| walker |  | 1073 | 42 | pub item at impl/src/lib.rs:40 |  |  | 0.777 |
| ns | 1087 |  | 252 | impl/src/lib.rs — #[proc_macro_derive] entry point | 1.14 |  | 0.697 |
| walker |  | 1108 | 35 | impl method sigs in impl/src/lib.rs |  |  | 0.702 |
| walker |  | 1139 | 31 | pub item body at impl/src/lib.rs:40 body 41 |  |  | 0.711 |
| walker |  | 1272 | 133 | mod/use plumbing in impl/src/lib.rs |  |  | 0.777 |
| ns | 1323 |  | 236 | src/lib.rs — module wiring epilogue | 1.15 |  | 0.710 |
| walker |  | 1382 | 110 | [dependencies] in Cargo.toml |  |  | 0.720 |
| walker |  | 1435 | 53 | [dependencies] in impl/Cargo.toml |  |  | 0.734 |
| walker |  | 1558 | 123 | manifest config in Cargo.toml |  |  | 0.734 |
| ns | 1600 |  | 277 | Crate-doc lede + canonical example | 2.1 |  | 0.672 |
| walker |  | 1754 | 196 | README.md section #1 |  |  | 0.672 |
| walker |  | 1784 | 30 | README.md section #3 |  |  | 0.672 |
| walker |  | 1928 | 144 | manifest config in impl/Cargo.toml |  |  | 0.672 |
| ns | 1946 |  | 346 | Display-shorthand table + #[from] rule statement | 2.2 |  | 0.635 |
| walker |  | 1956 | 28 | pub-item names surface in src/provide.rs |  |  | 0.635 |
| walker |  | 1984 | 28 | pub-item names surface in impl/src/unraw.rs |  |  | 0.635 |
| walker |  | 2001 | 17 | pub item at impl/src/unraw.rs:12 |  |  | 0.635 |
| walker |  | 2034 | 33 | pub item at impl/src/unraw.rs:82 |  |  | 0.635 |
| walker |  | 2063 | 29 | pub-item names surface in src/display.rs |  |  | 0.635 |
| walker |  | 2092 | 29 | pub-item names surface in impl/src/generics.rs |  |  | 0.635 |
| walker |  | 2110 | 18 | pub item at impl/src/generics.rs:8 |  |  | 0.635 |
| walker |  | 2151 | 41 | pub item at impl/src/generics.rs:48 |  |  | 0.635 |
| ns | 2173 |  | 227 | #[from] example + #[source] rule statement | 2.3 |  | 0.610 |
| walker |  | 2181 | 30 | pub-item names surface in src/aserror.rs |  |  | 0.610 |
| walker |  | 2225 | 44 | pub-item names surface in impl/src/expand.rs |  |  | 0.610 |
| walker |  | 2225 | 0 | pub item at impl/src/expand.rs:12 |  |  | 0.610 |
| walker |  | 2225 | 0 | pub item at impl/src/expand.rs:505 |  |  | 0.610 |
| ns | 2267 |  | 94 | provide()/backtrace rule statement | 2.4 |  | 0.602 |
| walker |  | 2310 | 85 | impl method sigs in impl/src/unraw.rs |  |  | 0.602 |
| walker |  | 2432 | 122 | [package] in impl/Cargo.toml |  |  | 0.629 |
| ns | 2451 |  | 184 | #[error(transparent)] rule + 'anything else' variant example | 2.5 |  | 0.605 |
| walker |  | 2483 | 51 | mod/use plumbing in impl/src/valid.rs |  |  | 0.605 |
| walker |  | 2542 | 59 | README.md section #11 |  |  | 0.605 |
| walker |  | 2595 | 53 | mod/use plumbing in impl/src/prop.rs |  |  | 0.605 |
| walker |  | 2651 | 56 | mod/use plumbing in impl/src/fallback.rs |  |  | 0.605 |
| walker |  | 2723 | 72 | README.md section #2 |  |  | 0.605 |
| walker |  | 2820 | 97 | pub item body at impl/src/expand.rs:12 body 13 |  |  | 0.606 |
| ns | 2912 |  | 461 | src/aserror.rs — AsDynError trait + impls | 3.1 |  | 0.548 |
| walker |  | 2950 | 130 | impl method sigs in impl/src/generics.rs |  |  | 0.548 |
| walker |  | 3028 | 78 | pub-item names surface in impl/src/ast.rs |  |  | 0.548 |
| walker |  | 3054 | 26 | pub item at impl/src/ast.rs:10 |  |  | 0.548 |
| ns | 3079 |  | 167 | src/provide.rs — ThiserrorProvide trait + blanket impl (cfg-gated) | 3.2 |  | 0.530 |
| walker |  | 3117 | 63 | pub item at impl/src/ast.rs:45 |  |  | 0.530 |
| walker |  | 3170 | 53 | pub item at impl/src/ast.rs:15 |  |  | 0.531 |
| walker |  | 3223 | 53 | pub item at impl/src/ast.rs:29 |  |  | 0.532 |
| walker |  | 3277 | 54 | pub item at impl/src/ast.rs:22 |  |  | 0.533 |
| walker |  | 3342 | 65 | pub item at impl/src/ast.rs:36 |  |  | 0.535 |
| ns | 3354 |  | 275 | src/private.rs (re-export surface) + src/var.rs (Var pointer wrapper) | 3.3 |  | 0.512 |
| walker |  | 3419 | 77 | mod/use plumbing in impl/src/scan_expr.rs |  |  | 0.512 |
| walker |  | 3427 | 8 | listing of '.github' |  |  | 0.515 |
| walker |  | 3431 | 4 | listing of '.github/workflows' |  |  | 0.520 |
| ns | 3644 |  | 290 | src/display.rs — AsDisplay trait + Path Display impl | 3.4 |  | 0.495 |
| walker |  | 3959 | 528 | listing of 'tests/ui' |  |  | 0.498 |
| walker |  | 3980 | 21 | pub item at impl/src/scan_expr.rs:192 |  |  | 0.499 |
| ns | 4061 |  | 417 | impl/src/ast.rs — Input/Struct/Enum/Variant/Field/ContainerKind struct defs | 4.1 |  | 0.545 |
| walker |  | 4079 | 99 | mod/use plumbing in impl/src/generics.rs |  |  | 0.545 |
| walker |  | 4179 | 100 | mod/use plumbing in impl/src/unraw.rs |  |  | 0.545 |
| walker |  | 4291 | 112 | pub-item names surface in impl/src/attr.rs |  |  | 0.545 |
| walker |  | 4291 | 0 | pub item at impl/src/attr.rs:69 |  |  | 0.545 |
| walker |  | 4327 | 36 | pub item at impl/src/attr.rs:51 |  |  | 0.545 |
| walker |  | 4363 | 36 | pub item at impl/src/attr.rs:45 |  |  | 0.545 |
| walker |  | 4399 | 36 | pub item at impl/src/attr.rs:39 |  |  | 0.546 |
| walker |  | 4435 | 36 | pub item at impl/src/attr.rs:33 |  |  | 0.546 |
| walker |  | 4527 | 92 | pub item at impl/src/attr.rs:57 |  |  | 0.547 |
| walker |  | 4617 | 90 | pub item at impl/src/attr.rs:11 |  |  | 0.549 |
| ns | 4623 |  | 562 | impl/src/attr.rs — Attrs/Display/Source/From/Transparent/Fmt/Trait struct defs | 5.1 |  | 0.559 |
| walker |  | 4729 | 112 | pub item at impl/src/attr.rs:21 |  |  | 0.594 |
| ns | 4900 |  | 277 | impl/src/valid.rs — Struct::validate() dispatch | 6.1 |  | 0.574 |
| walker |  | 5034 | 305 | README.md section #12 |  |  | 0.574 |
| walker |  | 5146 | 112 | mod/use plumbing in impl/src/ast.rs |  |  | 0.574 |
| walker |  | 5172 | 26 | pub item at impl/src/fallback.rs:7 |  |  | 0.574 |
| walker |  | 5207 | 35 | impl method sigs in impl/src/ast.rs |  |  | 0.574 |
| ns | 5378 |  | 478 | impl/src/valid.rs — check_non_field_attrs() | 6.2 |  | 0.547 |
| ns | 6219 |  | 841 | impl/src/valid.rs — check_field_attrs() | 6.3 |  | 0.507 |
| ns | 6441 |  | 222 | impl/src/expand.rs — derive()/try_expand() entry point | 7.1 |  | 0.504 |
| ns | 7004 |  | 563 | impl/src/expand.rs — impl_struct(): source() body construction | 7.2 |  | 0.485 |
| ns | 7333 |  | 329 | impl/src/expand.rs — from_initializer(): generated From-impl body | 7.3 |  | 0.473 |
| ns | 7429 |  | 96 | impl/src/expand.rs — impl_enum() entry: per-variant source() dispatch | 7.4 |  | 0.470 |
| ns | 7630 |  | 201 | impl/src/fmt.rs — expand_shorthand() signature + setup | 8.1 |  | 0.465 |
| walker |  | 7726 | 2519 | crate-doc body in src/lib.rs |  |  | 0.563 |
| walker |  | 7871 | 145 | mod/use plumbing in impl/src/expand.rs |  |  | 0.563 |
| walker |  | 8005 | 134 | README.md section #9 |  |  | 0.563 |
| ns | 8044 |  | 414 | impl/src/fallback.rs (full file) | 9.1 |  | 0.549 |
| ns | 8073 |  | 29 | impl/src/generics.rs — ParamsInScope/InferredBounds struct locations | 9.2 |  | 0.551 |
| ns | 8124 |  | 51 | impl/src/unraw.rs + impl/src/scan_expr.rs — type/fn locations | 9.3 |  | 0.552 |
| walker |  | 8156 | 151 | mod/use plumbing in impl/src/attr.rs |  |  | 0.552 |
| walker |  | 8198 | 42 | impl method sigs in impl/src/fmt.rs |  |  | 0.553 |
| walker |  | 8208 | 10 | pub item body at impl/src/attr.rs:69 body 121 |  |  | 0.553 |
| walker |  | 8232 | 24 | pub-item names surface in tests/test_backtrace.rs |  |  | 0.553 |
| walker |  | 8244 | 12 | pub item at tests/test_backtrace.rs:8 |  |  | 0.553 |
| walker |  | 8269 | 25 | pub-item names surface in tests/test_expr.rs |  |  | 0.553 |
| ns | 8287 |  | 163 | impl/src/prop.rs — from_field/source_field/backtrace_field/has_* predicate locations | 9.4 |  | 0.547 |
| walker |  | 8294 | 25 | pub-item names surface in tests/test_path.rs |  |  | 0.547 |
| walker |  | 8327 | 33 | pub item at tests/test_backtrace.rs:13 |  |  | 0.547 |
| walker |  | 8352 | 25 | pub item at src/aserror.rs:5 |  |  | 0.547 |
| ns | 8481 |  | 194 | build.rs — rerun-if-changed / rustc-check-cfg declarations + OUT_DIR/private.rs generation | 10.1 |  | 0.542 |
| walker |  | 8544 | 192 | mod/use plumbing in impl/src/fmt.rs |  |  | 0.542 |
| ns | 8683 |  | 202 | ci.yml — header + every job key (roster) | 11.1 |  | 0.533 |
| walker |  | 8716 | 172 | README.md section #7 |  |  | 0.533 |
| walker |  | 8754 | 38 | pub item at tests/test_path.rs:29 |  |  | 0.533 |
| walker |  | 8936 | 182 | README.md section #5 |  |  | 0.533 |
| ns | 8947 |  | 264 | ci.yml — primary `test` job detail | 11.2 |  | 0.525 |
| walker |  | 8966 | 30 | pub item at src/provide.rs:4 |  |  | 0.526 |
| walker |  | 8990 | 24 | pub item at tests/ui/display-underscore.rs:5 |  |  | 0.524 |
| ns | 8990 |  | 43 | rust-toolchain.toml + .gitignore + FUNDING.yml | 11.3 |  | 0.524 |
| walker |  | 9014 | 24 | pub item at tests/ui/unconditional-recursion.rs:5 |  |  | 0.524 |
| walker |  | 9038 | 24 | pub item at tests/ui/unexpected-struct-source.rs:5 |  |  | 0.524 |
| ns | 9180 |  | 190 | tests/test_error.rs — canonical error shapes | 12.1 |  | 0.516 |
| walker |  | 9239 | 201 | README.md section #8 |  |  | 0.516 |
| walker |  | 9264 | 25 | pub item at tests/ui/expression-fallback.rs:5 |  |  | 0.516 |
| walker |  | 9289 | 25 | pub item at tests/ui/fallback-impl-with-display.rs:6 |  |  | 0.516 |
| walker |  | 9314 | 25 | pub item at tests/ui/invalid-input-impl-anyway.rs:5 |  |  | 0.516 |
| walker |  | 9339 | 25 | pub item at tests/ui/transparent-struct-unnamed-field-not-error.rs:5 |  |  | 0.516 |
| walker |  | 9376 | 37 | pub-item names surface in tests/test_source.rs |  |  | 0.516 |
| ns | 9384 |  | 204 | tests/test_from.rs — #[from] shapes (struct/tuple, plain and Option<T>) | 12.2 |  | 0.508 |
| walker |  | 9404 | 28 | pub item at tests/test_source.rs:7 |  |  | 0.508 |
| walker |  | 9447 | 43 | pub item at tests/test_source.rs:13 |  |  | 0.508 |
| walker |  | 9488 | 41 | pub item at tests/test_source.rs:21 |  |  | 0.508 |
| walker |  | 9514 | 26 | pub item at tests/ui/numbered-positional-tuple.rs:5 |  |  | 0.508 |
| walker |  | 9540 | 26 | pub item at tests/ui/struct-with-fmt.rs:5 |  |  | 0.508 |
| walker |  | 9568 | 28 | pub item at tests/ui/duplicate-transparent.rs:6 |  |  | 0.508 |
| walker |  | 9596 | 28 | pub item at tests/ui/transparent-display.rs:6 |  |  | 0.508 |
| walker |  | 9663 | 67 | pub item at tests/test_path.rs:35 |  |  | 0.508 |
| walker |  | 9898 | 235 | README.md section #6 |  |  | 0.508 |
| ns | 9912 |  | 528 | tests/ui/ directory listing (all 37 compile-fail cases + their .stderr) | 13.1 |  | 0.546 |
| walker |  | 9922 | 24 | pub-item names surface in tests/ui/no-display.rs |  |  | 0.546 |
| walker |  | 9949 | 27 | pub item at tests/ui/no-display.rs:8 |  |  | 0.546 |
| walker |  | 9974 | 25 | pub item at tests/ui/no-display.rs:14 |  |  | 0.546 |
| walker |  | 9998 | 24 | pub-item names surface in tests/ui/source-enum-not-error.rs |  |  | 0.546 |
