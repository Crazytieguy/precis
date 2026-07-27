Score(3000)=0.664 I=0.876 C=0.503 ns_rows≤3K=21/45 (reached=12 partial=1 missing=8)

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 43 | 43 | listing of '.' |  |  | 1.000 |
| ns | 43 |  | 43 | Root directory listing | 1.1 |  | 1.000 |
| walker |  | 46 | 3 | listing of 'build' |  |  | 1.000 |
| walker |  | 65 | 19 | listing of 'impl' |  |  | 1.000 |
| walker |  | 89 | 24 | listing of 'src' |  |  | 1.000 |
| ns | 114 |  | 71 | src/ and impl/src/ directory listings | 1.2 |  | 0.632 |
| walker |  | 136 | 47 | README headline in README.md |  |  | 0.632 |
| walker |  | 172 | 36 | [dependencies] in Cargo.toml |  |  | 0.634 |
| ns | 196 |  | 82 | tests/ and tests/no-std/ directory listings | 1.3 |  | 0.483 |
| walker |  | 201 | 29 | headings outline in README.md |  |  | 0.483 |
| ns | 211 |  | 15 | build/, .github/, .github/workflows/ listings | 1.4 |  | 0.463 |
| ns | 261 |  | 50 | Cargo.toml — package identity (name/version/authors) | 1.5 |  | 0.441 |
| walker |  | 263 | 62 | mod/use plumbing in src/lib.rs |  |  | 0.442 |
| walker |  | 271 | 8 | listing of '.github' |  |  | 0.475 |
| walker |  | 274 | 3 | listing of '.github/workflows' |  |  | 0.502 |
| walker |  | 347 | 73 | listing of 'tests' |  |  | 0.682 |
| ns | 374 |  | 113 | Cargo.toml — categories/description/docs/edition/keywords/license/repo/rust-version | 1.6 |  | 0.626 |
| ns | 395 |  | 21 | Cargo.toml — [features] header | 1.7 |  | 0.608 |
| walker |  | 532 | 185 | [package] in Cargo.toml |  |  | 0.742 |
| ns | 540 |  | 145 | Cargo.toml — std feature doc comment | 1.8 |  | 0.669 |
| ns | 572 |  | 32 | Cargo.toml — [dependencies] | 1.9 |  | 0.674 |
| ns | 596 |  | 24 | Cargo.toml — [workspace] | 1.10 |  | 0.678 |
| walker |  | 604 | 72 | README.md section #0 |  |  | 0.678 |
| walker |  | 650 | 46 | listing of 'impl/src' |  |  | 0.848 |
| ns | 675 |  | 79 | impl/Cargo.toml — package identity (name/version/authors/description/edition) | 1.11 |  | 0.806 |
| walker |  | 692 | 42 | pub item at impl/src/lib.rs:40 |  |  | 0.807 |
| walker |  | 727 | 35 | impl method sigs in impl/src/lib.rs |  |  | 0.807 |
| ns | 748 |  | 73 | impl/Cargo.toml — [lib]/[dependencies] | 1.12 |  | 0.771 |
| walker |  | 758 | 31 | pub item body at impl/src/lib.rs:40 body 41 |  |  | 0.772 |
| ns | 839 |  | 91 | impl/src/lib.rs — module list | 1.13 |  | 0.719 |
| walker |  | 891 | 133 | mod/use plumbing in impl/src/lib.rs |  |  | 0.774 |
| ns | 1091 |  | 252 | impl/src/lib.rs — #[proc_macro_derive] entry point | 1.14 |  | 0.726 |
| walker |  | 1158 | 267 | crate-doc lede in src/lib.rs |  |  | 0.727 |
| ns | 1327 |  | 236 | src/lib.rs — module wiring epilogue | 1.15 |  | 0.665 |
| walker |  | 1354 | 196 | README.md section #1 |  |  | 0.665 |
| walker |  | 1518 | 164 | [features] in Cargo.toml |  |  | 0.742 |
| walker |  | 1591 | 73 | dev/build/target dependencies in Cargo.toml |  |  | 0.742 |
| ns | 1604 |  | 277 | Crate-doc lede + canonical example | 2.1 |  | 0.679 |
| walker |  | 1714 | 123 | manifest config in Cargo.toml |  |  | 0.679 |
| walker |  | 1767 | 53 | [dependencies] in impl/Cargo.toml |  |  | 0.692 |
| walker |  | 1797 | 30 | README.md section #3 |  |  | 0.692 |
| ns | 1950 |  | 346 | Display-shorthand table + #[from] rule statement | 2.2 |  | 0.654 |
| walker |  | 2130 | 333 | crate-doc body in src/lib.rs |  |  | 0.739 |
| walker |  | 2158 | 28 | pub-item names surface in src/provide.rs |  |  | 0.739 |
| ns | 2177 |  | 227 | #[from] example + #[source] rule statement | 2.3 |  | 0.710 |
| ns | 2271 |  | 94 | provide()/backtrace rule statement | 2.4 |  | 0.700 |
| walker |  | 2302 | 144 | manifest config in impl/Cargo.toml |  |  | 0.700 |
| walker |  | 2331 | 29 | pub-item names surface in src/display.rs |  |  | 0.700 |
| walker |  | 2361 | 30 | pub-item names surface in src/aserror.rs |  |  | 0.700 |
| walker |  | 2389 | 28 | pub-item names surface in impl/src/unraw.rs |  |  | 0.700 |
| walker |  | 2406 | 17 | pub item at impl/src/unraw.rs:12 |  |  | 0.700 |
| walker |  | 2439 | 33 | pub item at impl/src/unraw.rs:82 |  |  | 0.700 |
| ns | 2455 |  | 184 | #[error(transparent)] rule + 'anything else' variant example | 2.5 |  | 0.673 |
| walker |  | 2468 | 29 | pub-item names surface in impl/src/generics.rs |  |  | 0.673 |
| walker |  | 2486 | 18 | pub item at impl/src/generics.rs:8 |  |  | 0.673 |
| walker |  | 2527 | 41 | pub item at impl/src/generics.rs:48 |  |  | 0.673 |
| walker |  | 2535 | 8 | listing of 'tests/no-std' |  |  | 0.690 |
| walker |  | 2579 | 44 | pub-item names surface in impl/src/expand.rs |  |  | 0.690 |
| walker |  | 2579 | 0 | pub item at impl/src/expand.rs:12 |  |  | 0.690 |
| walker |  | 2579 | 0 | pub item at impl/src/expand.rs:505 |  |  | 0.690 |
| walker |  | 2838 | 259 | crate-doc tail at src/lib.rs:45 |  |  | 0.709 |
| ns | 2916 |  | 461 | src/aserror.rs — AsDynError trait + impls | 3.1 |  | 0.642 |
| walker |  | 2960 | 122 | [package] in impl/Cargo.toml |  |  | 0.664 |
| walker |  | 3019 | 59 | README.md section #11 |  |  | 0.664 |
| ns | 3083 |  | 167 | src/provide.rs — ThiserrorProvide trait + blanket impl (cfg-gated) | 3.2 |  | 0.641 |
| walker |  | 3104 | 85 | impl method sigs in impl/src/unraw.rs |  |  | 0.641 |
| ns | 3358 |  | 275 | src/private.rs (re-export surface) + src/var.rs (Var pointer wrapper) | 3.3 |  | 0.613 |
| walker |  | 3438 | 334 | crate-doc tail at src/lib.rs:59 |  |  | 0.613 |
| ns | 3648 |  | 290 | src/display.rs — AsDisplay trait + Path Display impl | 3.4 |  | 0.584 |
| walker |  | 3722 | 284 | crate-doc tail at src/lib.rs:99 |  |  | 0.624 |
| ns | 4065 |  | 417 | impl/src/ast.rs — Input/Struct/Enum/Variant/Field/ContainerKind struct defs | 4.1 |  | 0.583 |
| walker |  | 4249 | 527 | listing of 'tests/ui' |  |  | 0.587 |
| walker |  | 4300 | 51 | mod/use plumbing in impl/src/valid.rs |  |  | 0.587 |
| walker |  | 4353 | 53 | mod/use plumbing in impl/src/prop.rs |  |  | 0.587 |
| walker |  | 4425 | 72 | README.md section #2 |  |  | 0.587 |
| walker |  | 4481 | 56 | mod/use plumbing in impl/src/fallback.rs |  |  | 0.587 |
| ns | 4627 |  | 562 | impl/src/attr.rs — Attrs/Display/Source/From/Transparent/Fmt/Trait struct defs | 5.1 |  | 0.542 |
| walker |  | 4870 | 389 | crate-doc tail at src/lib.rs:135 |  |  | 0.569 |
| ns | 4904 |  | 277 | impl/src/valid.rs — Struct::validate() dispatch | 6.1 |  | 0.550 |
| walker |  | 5249 | 379 | crate-doc tail at src/lib.rs:176 |  |  | 0.550 |
| walker |  | 5379 | 130 | impl method sigs in impl/src/generics.rs |  |  | 0.550 |
| ns | 5382 |  | 478 | impl/src/valid.rs — check_non_field_attrs() | 6.2 |  | 0.524 |
| walker |  | 5457 | 78 | pub-item names surface in impl/src/ast.rs |  |  | 0.526 |
| walker |  | 5483 | 26 | pub item at impl/src/ast.rs:10 |  |  | 0.528 |
| walker |  | 5546 | 63 | pub item at impl/src/ast.rs:45 |  |  | 0.535 |
| walker |  | 5599 | 53 | pub item at impl/src/ast.rs:15 |  |  | 0.543 |
| walker |  | 5652 | 53 | pub item at impl/src/ast.rs:29 |  |  | 0.554 |
| walker |  | 5706 | 54 | pub item at impl/src/ast.rs:22 |  |  | 0.567 |
| walker |  | 5771 | 65 | pub item at impl/src/ast.rs:36 |  |  | 0.585 |
| walker |  | 5868 | 97 | pub item body at impl/src/expand.rs:12 body 13 |  |  | 0.586 |
| walker |  | 5945 | 77 | mod/use plumbing in impl/src/scan_expr.rs |  |  | 0.586 |
| ns | 6223 |  | 841 | impl/src/valid.rs — check_field_attrs() | 6.3 |  | 0.543 |
| ns | 6445 |  | 222 | impl/src/expand.rs — derive()/try_expand() entry point | 7.1 |  | 0.540 |
| walker |  | 6486 | 541 | crate-doc tail at src/lib.rs:210 |  |  | 0.558 |
| walker |  | 6507 | 21 | pub item at impl/src/scan_expr.rs:192 |  |  | 0.558 |
| walker |  | 6812 | 305 | README.md section #12 |  |  | 0.558 |
| walker |  | 6924 | 112 | pub-item names surface in impl/src/attr.rs |  |  | 0.559 |
| walker |  | 6924 | 0 | pub item at impl/src/attr.rs:69 |  |  | 0.559 |
| walker |  | 6960 | 36 | pub item at impl/src/attr.rs:51 |  |  | 0.561 |
| walker |  | 6996 | 36 | pub item at impl/src/attr.rs:45 |  |  | 0.563 |
| ns | 7008 |  | 563 | impl/src/expand.rs — impl_struct(): source() body construction | 7.2 |  | 0.542 |
| walker |  | 7032 | 36 | pub item at impl/src/attr.rs:39 |  |  | 0.545 |
| walker |  | 7068 | 36 | pub item at impl/src/attr.rs:33 |  |  | 0.548 |
| walker |  | 7160 | 92 | pub item at impl/src/attr.rs:57 |  |  | 0.560 |
| walker |  | 7250 | 90 | pub item at impl/src/attr.rs:11 |  |  | 0.575 |
| ns | 7337 |  | 329 | impl/src/expand.rs — from_initializer(): generated From-impl body | 7.3 |  | 0.561 |
| walker |  | 7362 | 112 | pub item at impl/src/attr.rs:21 |  |  | 0.586 |
| ns | 7433 |  | 96 | impl/src/expand.rs — impl_enum() entry: per-variant source() dispatch | 7.4 |  | 0.582 |
| walker |  | 7461 | 99 | mod/use plumbing in impl/src/generics.rs |  |  | 0.582 |
| walker |  | 7561 | 100 | mod/use plumbing in impl/src/unraw.rs |  |  | 0.582 |
| walker |  | 7587 | 26 | pub item at impl/src/fallback.rs:7 |  |  | 0.582 |
| ns | 7634 |  | 201 | impl/src/fmt.rs — expand_shorthand() signature + setup | 8.1 |  | 0.575 |
| walker |  | 7699 | 112 | mod/use plumbing in impl/src/ast.rs |  |  | 0.575 |
| walker |  | 7833 | 134 | README.md section #9 |  |  | 0.575 |
| walker |  | 7868 | 35 | impl method sigs in impl/src/ast.rs |  |  | 0.575 |
| walker |  | 7892 | 24 | pub-item names surface in tests/test_backtrace.rs |  |  | 0.575 |
| walker |  | 7904 | 12 | pub item at tests/test_backtrace.rs:8 |  |  | 0.575 |
| ns | 8048 |  | 414 | impl/src/fallback.rs (full file) | 9.1 |  | 0.561 |
| walker |  | 8049 | 145 | mod/use plumbing in impl/src/expand.rs |  |  | 0.561 |
| walker |  | 8074 | 25 | pub-item names surface in tests/test_expr.rs |  |  | 0.561 |
| ns | 8077 |  | 29 | impl/src/generics.rs — ParamsInScope/InferredBounds struct locations | 9.2 |  | 0.562 |
| walker |  | 8099 | 25 | pub-item names surface in tests/test_path.rs |  |  | 0.562 |
| ns | 8128 |  | 51 | impl/src/unraw.rs + impl/src/scan_expr.rs — type/fn locations | 9.3 |  | 0.564 |
| walker |  | 8250 | 151 | mod/use plumbing in impl/src/attr.rs |  |  | 0.564 |
| ns | 8291 |  | 163 | impl/src/prop.rs — from_field/source_field/backtrace_field/has_* predicate locations | 9.4 |  | 0.558 |
| walker |  | 8292 | 42 | impl method sigs in impl/src/fmt.rs |  |  | 0.558 |
| walker |  | 8302 | 10 | pub item body at impl/src/attr.rs:69 body 121 |  |  | 0.558 |
| walker |  | 8474 | 172 | README.md section #7 |  |  | 0.558 |
| ns | 8485 |  | 194 | build.rs — rerun-if-changed / rustc-check-cfg declarations + OUT_DIR/private.rs generation | 10.1 |  | 0.553 |
| walker |  | 8507 | 33 | pub item at tests/test_backtrace.rs:13 |  |  | 0.553 |
| walker |  | 8532 | 25 | pub item at src/aserror.rs:5 |  |  | 0.553 |
| ns | 8687 |  | 202 | ci.yml — header + every job key (roster) | 11.1 |  | 0.544 |
| walker |  | 8714 | 182 | README.md section #5 |  |  | 0.544 |
| walker |  | 8752 | 38 | pub item at tests/test_path.rs:29 |  |  | 0.544 |
| walker |  | 8944 | 192 | mod/use plumbing in impl/src/fmt.rs |  |  | 0.544 |
| ns | 8951 |  | 264 | ci.yml — primary `test` job detail | 11.2 |  | 0.537 |
| ns | 8994 |  | 43 | rust-toolchain.toml + .gitignore + FUNDING.yml | 11.3 |  | 0.535 |
| walker |  | 9145 | 201 | README.md section #8 |  |  | 0.535 |
| walker |  | 9175 | 30 | pub item at src/provide.rs:4 |  |  | 0.536 |
| ns | 9184 |  | 190 | tests/test_error.rs — canonical error shapes | 12.1 |  | 0.527 |
| walker |  | 9212 | 37 | pub-item names surface in tests/test_source.rs |  |  | 0.527 |
| walker |  | 9240 | 28 | pub item at tests/test_source.rs:7 |  |  | 0.527 |
| walker |  | 9283 | 43 | pub item at tests/test_source.rs:13 |  |  | 0.527 |
| walker |  | 9324 | 41 | pub item at tests/test_source.rs:21 |  |  | 0.527 |
| walker |  | 9348 | 24 | pub item at tests/ui/display-underscore.rs:5 |  |  | 0.527 |
| walker |  | 9372 | 24 | pub item at tests/ui/unconditional-recursion.rs:5 |  |  | 0.527 |
| ns | 9388 |  | 204 | tests/test_from.rs — #[from] shapes (struct/tuple, plain and Option<T>) | 12.2 |  | 0.519 |
| walker |  | 9396 | 24 | pub item at tests/ui/unexpected-struct-source.rs:5 |  |  | 0.519 |
| walker |  | 9421 | 25 | pub item at tests/ui/expression-fallback.rs:5 |  |  | 0.519 |
| walker |  | 9446 | 25 | pub item at tests/ui/fallback-impl-with-display.rs:6 |  |  | 0.519 |
| walker |  | 9471 | 25 | pub item at tests/ui/invalid-input-impl-anyway.rs:5 |  |  | 0.519 |
| walker |  | 9496 | 25 | pub item at tests/ui/transparent-struct-unnamed-field-not-error.rs:5 |  |  | 0.519 |
| walker |  | 9731 | 235 | README.md section #6 |  |  | 0.519 |
| walker |  | 9757 | 26 | pub item at tests/ui/numbered-positional-tuple.rs:5 |  |  | 0.519 |
| walker |  | 9783 | 26 | pub item at tests/ui/struct-with-fmt.rs:5 |  |  | 0.519 |
| walker |  | 9850 | 67 | pub item at tests/test_path.rs:35 |  |  | 0.519 |
| walker |  | 9878 | 28 | pub item at tests/ui/duplicate-transparent.rs:6 |  |  | 0.519 |
| walker |  | 9906 | 28 | pub item at tests/ui/transparent-display.rs:6 |  |  | 0.519 |
| ns | 9915 |  | 527 | tests/ui/ directory listing (all 37 compile-fail cases + their .stderr) | 13.1 |  | 0.557 |
| walker |  | 9930 | 24 | pub-item names surface in tests/ui/no-display.rs |  |  | 0.557 |
| walker |  | 9957 | 27 | pub item at tests/ui/no-display.rs:8 |  |  | 0.557 |
| walker |  | 9982 | 25 | pub item at tests/ui/no-display.rs:14 |  |  | 0.557 |
