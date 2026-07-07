Score(3000)=0.561 I=0.840 C=0.374 ns_rows≤3K=21/45 (reached=9 partial=3 missing=9)

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 38 | 38 | listing of '.' |  |  | 1.000 |
| ns | 38 |  | 38 | Root directory listing | 1.1 |  | 1.000 |
| walker |  | 85 | 47 | README headline in README.md |  |  | 1.000 |
| ns | 110 |  | 72 | src/ and impl/src/ directory listings | 1.2 |  | 0.569 |
| walker |  | 114 | 29 | headings outline in README.md |  |  | 0.569 |
| walker |  | 139 | 25 | listing of 'src' |  |  | 0.631 |
| walker |  | 158 | 19 | listing of 'impl' |  |  | 0.631 |
| ns | 191 |  | 81 | tests/ and tests/no-std/ directory listings | 1.3 |  | 0.481 |
| ns | 207 |  | 16 | build/, .github/, .github/workflows/ listings | 1.4 |  | 0.457 |
| ns | 257 |  | 50 | Cargo.toml — package identity (name/version/authors) | 1.5 |  | 0.435 |
| walker |  | 345 | 187 | [package] in Cargo.toml |  |  | 0.511 |
| ns | 370 |  | 113 | Cargo.toml — categories/description/docs/edition/keywords/license/repo/rust-version | 1.6 |  | 0.564 |
| ns | 391 |  | 21 | Cargo.toml — [features] header | 1.7 |  | 0.547 |
| walker |  | 417 | 72 | README.md section #0 |  |  | 0.547 |
| walker |  | 479 | 62 | mod/use plumbing in src/lib.rs |  |  | 0.548 |
| ns | 536 |  | 145 | Cargo.toml — std feature doc comment | 1.8 |  | 0.494 |
| ns | 568 |  | 32 | Cargo.toml — [dependencies] | 1.9 |  | 0.487 |
| ns | 592 |  | 24 | Cargo.toml — [workspace] | 1.10 |  | 0.497 |
| ns | 671 |  | 79 | impl/Cargo.toml — package identity (name/version/authors/description/edition) | 1.11 |  | 0.473 |
| walker |  | 675 | 196 | README.md section #1 |  |  | 0.473 |
| walker |  | 683 | 8 | listing of '.github' |  |  | 0.483 |
| walker |  | 687 | 4 | listing of '.github/workflows' |  |  | 0.495 |
| ns | 744 |  | 73 | impl/Cargo.toml — [lib]/[dependencies] | 1.12 |  | 0.472 |
| walker |  | 759 | 72 | listing of 'tests' |  |  | 0.586 |
| walker |  | 806 | 47 | listing of 'impl/src' |  |  | 0.741 |
| ns | 835 |  | 91 | impl/src/lib.rs — module list | 1.13 |  | 0.690 |
| walker |  | 848 | 42 | pub item at impl/src/lib.rs:40 |  |  | 0.690 |
| walker |  | 883 | 35 | impl method sigs in impl/src/lib.rs |  |  | 0.691 |
| walker |  | 914 | 31 | pub item body at impl/src/lib.rs:40 body 41 |  |  | 0.692 |
| walker |  | 1047 | 133 | mod/use plumbing in impl/src/lib.rs |  |  | 0.748 |
| ns | 1087 |  | 252 | impl/src/lib.rs — #[proc_macro_derive] entry point | 1.14 |  | 0.704 |
| walker |  | 1314 | 267 | crate-doc lede in src/lib.rs |  |  | 0.704 |
| ns | 1323 |  | 236 | src/lib.rs — module wiring epilogue | 1.15 |  | 0.644 |
| walker |  | 1480 | 166 | [features] in Cargo.toml |  |  | 0.722 |
| ns | 1600 |  | 277 | Crate-doc lede + canonical example | 2.1 |  | 0.662 |
| walker |  | 1603 | 123 | manifest config in Cargo.toml |  |  | 0.662 |
| walker |  | 1713 | 110 | [dependencies] in Cargo.toml |  |  | 0.671 |
| walker |  | 1766 | 53 | [dependencies] in impl/Cargo.toml |  |  | 0.684 |
| walker |  | 1796 | 30 | README.md section #3 |  |  | 0.684 |
| walker |  | 1824 | 28 | pub-item names surface in src/provide.rs |  |  | 0.684 |
| ns | 1946 |  | 346 | Display-shorthand table + #[from] rule statement | 2.2 |  | 0.646 |
| walker |  | 1968 | 144 | manifest config in impl/Cargo.toml |  |  | 0.646 |
| walker |  | 1997 | 29 | pub-item names surface in src/display.rs |  |  | 0.646 |
| walker |  | 2027 | 30 | pub-item names surface in src/aserror.rs |  |  | 0.646 |
| walker |  | 2055 | 28 | pub-item names surface in impl/src/unraw.rs |  |  | 0.646 |
| walker |  | 2072 | 17 | pub item at impl/src/unraw.rs:12 |  |  | 0.646 |
| walker |  | 2105 | 33 | pub item at impl/src/unraw.rs:82 |  |  | 0.646 |
| walker |  | 2134 | 29 | pub-item names surface in impl/src/generics.rs |  |  | 0.646 |
| walker |  | 2152 | 18 | pub item at impl/src/generics.rs:8 |  |  | 0.646 |
| ns | 2173 |  | 227 | #[from] example + #[source] rule statement | 2.3 |  | 0.620 |
| walker |  | 2193 | 41 | pub item at impl/src/generics.rs:48 |  |  | 0.620 |
| ns | 2267 |  | 94 | provide()/backtrace rule statement | 2.4 |  | 0.612 |
| ns | 2451 |  | 184 | #[error(transparent)] rule + 'anything else' variant example | 2.5 |  | 0.588 |
| walker |  | 2721 | 528 | listing of 'tests/ui' |  |  | 0.593 |
| walker |  | 2765 | 44 | pub-item names surface in impl/src/expand.rs |  |  | 0.593 |
| walker |  | 2765 | 0 | pub item at impl/src/expand.rs:12 |  |  | 0.593 |
| walker |  | 2765 | 0 | pub item at impl/src/expand.rs:505 |  |  | 0.593 |
| walker |  | 2887 | 122 | [package] in impl/Cargo.toml |  |  | 0.620 |
| ns | 2912 |  | 461 | src/aserror.rs — AsDynError trait + impls | 3.1 |  | 0.561 |
| walker |  | 2946 | 59 | README.md section #11 |  |  | 0.561 |
| walker |  | 3031 | 85 | impl method sigs in impl/src/unraw.rs |  |  | 0.561 |
| ns | 3079 |  | 167 | src/provide.rs — ThiserrorProvide trait + blanket impl (cfg-gated) | 3.2 |  | 0.542 |
| walker |  | 3082 | 51 | mod/use plumbing in impl/src/valid.rs |  |  | 0.542 |
| walker |  | 3135 | 53 | mod/use plumbing in impl/src/prop.rs |  |  | 0.542 |
| walker |  | 3207 | 72 | README.md section #2 |  |  | 0.542 |
| walker |  | 3263 | 56 | mod/use plumbing in impl/src/fallback.rs |  |  | 0.542 |
| walker |  | 3272 | 9 | listing of 'tests/no-std' |  |  | 0.556 |
| ns | 3354 |  | 275 | src/private.rs (re-export surface) + src/var.rs (Var pointer wrapper) | 3.3 |  | 0.532 |
| walker |  | 3402 | 130 | impl method sigs in impl/src/generics.rs |  |  | 0.532 |
| walker |  | 3480 | 78 | pub-item names surface in impl/src/ast.rs |  |  | 0.533 |
| walker |  | 3506 | 26 | pub item at impl/src/ast.rs:10 |  |  | 0.533 |
| walker |  | 3569 | 63 | pub item at impl/src/ast.rs:45 |  |  | 0.533 |
| walker |  | 3622 | 53 | pub item at impl/src/ast.rs:15 |  |  | 0.534 |
| ns | 3644 |  | 290 | src/display.rs — AsDisplay trait + Path Display impl | 3.4 |  | 0.508 |
| walker |  | 3675 | 53 | pub item at impl/src/ast.rs:29 |  |  | 0.509 |
| walker |  | 3729 | 54 | pub item at impl/src/ast.rs:22 |  |  | 0.510 |
| walker |  | 3794 | 65 | pub item at impl/src/ast.rs:36 |  |  | 0.512 |
| walker |  | 3891 | 97 | pub item body at impl/src/expand.rs:12 body 13 |  |  | 0.512 |
| walker |  | 3968 | 77 | mod/use plumbing in impl/src/scan_expr.rs |  |  | 0.512 |
| walker |  | 3989 | 21 | pub item at impl/src/scan_expr.rs:192 |  |  | 0.512 |
| ns | 4061 |  | 417 | impl/src/ast.rs — Input/Struct/Enum/Variant/Field/ContainerKind struct defs | 4.1 |  | 0.557 |
| walker |  | 4294 | 305 | README.md section #12 |  |  | 0.557 |
| walker |  | 4406 | 112 | pub-item names surface in impl/src/attr.rs |  |  | 0.557 |
| walker |  | 4406 | 0 | pub item at impl/src/attr.rs:69 |  |  | 0.557 |
| walker |  | 4442 | 36 | pub item at impl/src/attr.rs:51 |  |  | 0.557 |
| walker |  | 4478 | 36 | pub item at impl/src/attr.rs:45 |  |  | 0.558 |
| walker |  | 4514 | 36 | pub item at impl/src/attr.rs:39 |  |  | 0.558 |
| walker |  | 4550 | 36 | pub item at impl/src/attr.rs:33 |  |  | 0.558 |
| ns | 4623 |  | 562 | impl/src/attr.rs — Attrs/Display/Source/From/Transparent/Fmt/Trait struct defs | 5.1 |  | 0.532 |
| walker |  | 4642 | 92 | pub item at impl/src/attr.rs:57 |  |  | 0.549 |
| walker |  | 4732 | 90 | pub item at impl/src/attr.rs:11 |  |  | 0.570 |
| walker |  | 4844 | 112 | pub item at impl/src/attr.rs:21 |  |  | 0.605 |
| ns | 4900 |  | 277 | impl/src/valid.rs — Struct::validate() dispatch | 6.1 |  | 0.585 |
| walker |  | 4943 | 99 | mod/use plumbing in impl/src/generics.rs |  |  | 0.585 |
| walker |  | 5043 | 100 | mod/use plumbing in impl/src/unraw.rs |  |  | 0.585 |
| walker |  | 5069 | 26 | pub item at impl/src/fallback.rs:7 |  |  | 0.585 |
| ns | 5378 |  | 478 | impl/src/valid.rs — check_non_field_attrs() | 6.2 |  | 0.557 |
| ns | 6219 |  | 841 | impl/src/valid.rs — check_field_attrs() | 6.3 |  | 0.516 |
| ns | 6441 |  | 222 | impl/src/expand.rs — derive()/try_expand() entry point | 7.1 |  | 0.514 |
| ns | 7004 |  | 563 | impl/src/expand.rs — impl_struct(): source() body construction | 7.2 |  | 0.494 |
| ns | 7333 |  | 329 | impl/src/expand.rs — from_initializer(): generated From-impl body | 7.3 |  | 0.482 |
| ns | 7429 |  | 96 | impl/src/expand.rs — impl_enum() entry: per-variant source() dispatch | 7.4 |  | 0.479 |
| walker |  | 7588 | 2519 | crate-doc body in src/lib.rs |  |  | 0.579 |
| ns | 7630 |  | 201 | impl/src/fmt.rs — expand_shorthand() signature + setup | 8.1 |  | 0.572 |
| walker |  | 7700 | 112 | mod/use plumbing in impl/src/ast.rs |  |  | 0.572 |
| walker |  | 7834 | 134 | README.md section #9 |  |  | 0.572 |
| walker |  | 7869 | 35 | impl method sigs in impl/src/ast.rs |  |  | 0.572 |
| walker |  | 7893 | 24 | pub-item names surface in tests/test_backtrace.rs |  |  | 0.572 |
| walker |  | 7905 | 12 | pub item at tests/test_backtrace.rs:8 |  |  | 0.572 |
| ns | 8044 |  | 414 | impl/src/fallback.rs (full file) | 9.1 |  | 0.558 |
| walker |  | 8050 | 145 | mod/use plumbing in impl/src/expand.rs |  |  | 0.558 |
| ns | 8073 |  | 29 | impl/src/generics.rs — ParamsInScope/InferredBounds struct locations | 9.2 |  | 0.559 |
| walker |  | 8075 | 25 | pub-item names surface in tests/test_expr.rs |  |  | 0.559 |
| walker |  | 8100 | 25 | pub-item names surface in tests/test_path.rs |  |  | 0.559 |
| ns | 8124 |  | 51 | impl/src/unraw.rs + impl/src/scan_expr.rs — type/fn locations | 9.3 |  | 0.561 |
| walker |  | 8251 | 151 | mod/use plumbing in impl/src/attr.rs |  |  | 0.561 |
| ns | 8287 |  | 163 | impl/src/prop.rs — from_field/source_field/backtrace_field/has_* predicate locations | 9.4 |  | 0.555 |
| walker |  | 8293 | 42 | impl method sigs in impl/src/fmt.rs |  |  | 0.555 |
| walker |  | 8303 | 10 | pub item body at impl/src/attr.rs:69 body 121 |  |  | 0.555 |
| walker |  | 8475 | 172 | README.md section #7 |  |  | 0.555 |
| ns | 8481 |  | 194 | build.rs — rerun-if-changed / rustc-check-cfg declarations + OUT_DIR/private.rs generation | 10.1 |  | 0.550 |
| walker |  | 8508 | 33 | pub item at tests/test_backtrace.rs:13 |  |  | 0.550 |
| walker |  | 8533 | 25 | pub item at src/aserror.rs:5 |  |  | 0.550 |
| ns | 8683 |  | 202 | ci.yml — header + every job key (roster) | 11.1 |  | 0.541 |
| walker |  | 8715 | 182 | README.md section #5 |  |  | 0.541 |
| walker |  | 8753 | 38 | pub item at tests/test_path.rs:29 |  |  | 0.541 |
| walker |  | 8945 | 192 | mod/use plumbing in impl/src/fmt.rs |  |  | 0.541 |
| ns | 8947 |  | 264 | ci.yml — primary `test` job detail | 11.2 |  | 0.533 |
| ns | 8990 |  | 43 | rust-toolchain.toml + .gitignore + FUNDING.yml | 11.3 |  | 0.531 |
| walker |  | 9146 | 201 | README.md section #8 |  |  | 0.531 |
| walker |  | 9176 | 30 | pub item at src/provide.rs:4 |  |  | 0.532 |
| ns | 9180 |  | 190 | tests/test_error.rs — canonical error shapes | 12.1 |  | 0.524 |
| walker |  | 9213 | 37 | pub-item names surface in tests/test_source.rs |  |  | 0.524 |
| walker |  | 9241 | 28 | pub item at tests/test_source.rs:7 |  |  | 0.524 |
| walker |  | 9284 | 43 | pub item at tests/test_source.rs:13 |  |  | 0.524 |
| walker |  | 9325 | 41 | pub item at tests/test_source.rs:21 |  |  | 0.524 |
| walker |  | 9349 | 24 | pub item at tests/ui/display-underscore.rs:5 |  |  | 0.524 |
| walker |  | 9373 | 24 | pub item at tests/ui/unconditional-recursion.rs:5 |  |  | 0.524 |
| ns | 9384 |  | 204 | tests/test_from.rs — #[from] shapes (struct/tuple, plain and Option<T>) | 12.2 |  | 0.516 |
| walker |  | 9397 | 24 | pub item at tests/ui/unexpected-struct-source.rs:5 |  |  | 0.516 |
| walker |  | 9422 | 25 | pub item at tests/ui/expression-fallback.rs:5 |  |  | 0.516 |
| walker |  | 9447 | 25 | pub item at tests/ui/fallback-impl-with-display.rs:6 |  |  | 0.516 |
| walker |  | 9472 | 25 | pub item at tests/ui/invalid-input-impl-anyway.rs:5 |  |  | 0.516 |
| walker |  | 9497 | 25 | pub item at tests/ui/transparent-struct-unnamed-field-not-error.rs:5 |  |  | 0.516 |
| walker |  | 9732 | 235 | README.md section #6 |  |  | 0.516 |
| walker |  | 9758 | 26 | pub item at tests/ui/numbered-positional-tuple.rs:5 |  |  | 0.516 |
| walker |  | 9784 | 26 | pub item at tests/ui/struct-with-fmt.rs:5 |  |  | 0.516 |
| walker |  | 9851 | 67 | pub item at tests/test_path.rs:35 |  |  | 0.516 |
| walker |  | 9879 | 28 | pub item at tests/ui/duplicate-transparent.rs:6 |  |  | 0.516 |
| walker |  | 9907 | 28 | pub item at tests/ui/transparent-display.rs:6 |  |  | 0.516 |
| ns | 9912 |  | 528 | tests/ui/ directory listing (all 37 compile-fail cases + their .stderr) | 13.1 |  | 0.553 |
| walker |  | 9931 | 24 | pub-item names surface in tests/ui/no-display.rs |  |  | 0.553 |
| walker |  | 9958 | 27 | pub item at tests/ui/no-display.rs:8 |  |  | 0.553 |
| walker |  | 9983 | 25 | pub item at tests/ui/no-display.rs:14 |  |  | 0.553 |
