Score(3000)=0.668 I=0.876 C=0.509 ns_rows≤3K=21/45 (reached=12 partial=2 missing=7)

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
| walker |  | 578 | 46 | listing of 'impl/src' |  |  | 0.847 |
| ns | 596 |  | 24 | Cargo.toml — [workspace] | 1.10 |  | 0.848 |
| walker |  | 620 | 42 | pub item at impl/src/lib.rs:40 |  |  | 0.848 |
| walker |  | 655 | 35 | impl method sigs in impl/src/lib.rs |  |  | 0.849 |
| walker |  | 655 | 0 | impl method at impl/src/lib.rs:49 |  |  | 0.849 |
| ns | 675 |  | 79 | impl/Cargo.toml — package identity (name/version/authors/description/edition) | 1.11 |  | 0.807 |
| walker |  | 686 | 31 | pub item body at impl/src/lib.rs:40 body 41 |  |  | 0.808 |
| ns | 748 |  | 73 | impl/Cargo.toml — [lib]/[dependencies] | 1.12 |  | 0.772 |
| walker |  | 758 | 72 | README.md section #0 |  |  | 0.772 |
| ns | 839 |  | 91 | impl/src/lib.rs — module list | 1.13 |  | 0.719 |
| walker |  | 891 | 133 | mod/use plumbing in impl/src/lib.rs |  |  | 0.774 |
| ns | 1091 |  | 252 | impl/src/lib.rs — #[proc_macro_derive] entry point | 1.14 |  | 0.726 |
| walker |  | 1158 | 267 | crate-doc lede in src/lib.rs |  |  | 0.727 |
| ns | 1327 |  | 236 | src/lib.rs — module wiring epilogue | 1.15 |  | 0.665 |
| walker |  | 1354 | 196 | README.md section #1 |  |  | 0.665 |
| walker |  | 1518 | 164 | [features] in Cargo.toml |  |  | 0.742 |
| walker |  | 1591 | 73 | dev/build/target dependencies in Cargo.toml |  |  | 0.742 |
| ns | 1604 |  | 277 | Crate-doc lede + canonical example | 2.1 |  | 0.679 |
| walker |  | 1638 | 47 | impl method body at impl/src/lib.rs:49 body 50 |  |  | 0.704 |
| walker |  | 1761 | 123 | manifest config in Cargo.toml |  |  | 0.704 |
| walker |  | 1814 | 53 | [dependencies] in impl/Cargo.toml |  |  | 0.716 |
| walker |  | 1844 | 30 | README.md section #3 |  |  | 0.716 |
| ns | 1950 |  | 346 | Display-shorthand table + #[from] rule statement | 2.2 |  | 0.677 |
| walker |  | 2177 | 333 | crate-doc body in src/lib.rs |  |  | 0.730 |
| ns | 2177 |  | 227 | #[from] example + #[source] rule statement | 2.3 |  | 0.730 |
| walker |  | 2205 | 28 | pub-item names surface in src/provide.rs |  |  | 0.730 |
| walker |  | 2234 | 29 | pub-item names surface in src/display.rs |  |  | 0.730 |
| walker |  | 2264 | 30 | pub-item names surface in src/aserror.rs |  |  | 0.730 |
| ns | 2271 |  | 94 | provide()/backtrace rule statement | 2.4 |  | 0.720 |
| walker |  | 2292 | 28 | pub-item names surface in impl/src/unraw.rs |  |  | 0.720 |
| walker |  | 2309 | 17 | pub item at impl/src/unraw.rs:12 |  |  | 0.720 |
| walker |  | 2342 | 33 | pub item at impl/src/unraw.rs:82 |  |  | 0.720 |
| walker |  | 2371 | 29 | pub-item names surface in impl/src/generics.rs |  |  | 0.720 |
| walker |  | 2389 | 18 | pub item at impl/src/generics.rs:8 |  |  | 0.720 |
| walker |  | 2430 | 41 | pub item at impl/src/generics.rs:48 |  |  | 0.720 |
| ns | 2455 |  | 184 | #[error(transparent)] rule + 'anything else' variant example | 2.5 |  | 0.693 |
| walker |  | 2591 | 161 | manifest config in impl/Cargo.toml |  |  | 0.703 |
| walker |  | 2599 | 8 | listing of 'tests/no-std' |  |  | 0.719 |
| walker |  | 2643 | 44 | pub-item names surface in impl/src/expand.rs |  |  | 0.719 |
| walker |  | 2643 | 0 | pub item at impl/src/expand.rs:12 |  |  | 0.719 |
| walker |  | 2643 | 0 | pub item at impl/src/expand.rs:505 |  |  | 0.719 |
| walker |  | 2902 | 259 | crate-doc tail at src/lib.rs:45 |  |  | 0.738 |
| ns | 2916 |  | 461 | src/aserror.rs — AsDynError trait + impls | 3.1 |  | 0.668 |
| walker |  | 3022 | 120 | [package] in impl/Cargo.toml |  |  | 0.690 |
| walker |  | 3081 | 59 | README.md section #11 |  |  | 0.690 |
| ns | 3083 |  | 167 | src/provide.rs — ThiserrorProvide trait + blanket impl (cfg-gated) | 3.2 |  | 0.666 |
| walker |  | 3166 | 85 | impl method sigs in impl/src/unraw.rs |  |  | 0.666 |
| walker |  | 3166 | 0 | impl method at impl/src/unraw.rs:15 |  |  | 0.666 |
| walker |  | 3166 | 0 | impl method at impl/src/unraw.rs:19 |  |  | 0.666 |
| walker |  | 3166 | 0 | impl method at impl/src/unraw.rs:32 |  |  | 0.666 |
| walker |  | 3166 | 0 | impl method at impl/src/unraw.rs:88 |  |  | 0.666 |
| walker |  | 3177 | 11 | impl method body at impl/src/unraw.rs:15 body 16 |  |  | 0.666 |
| walker |  | 3189 | 12 | impl method body at impl/src/unraw.rs:32 body 33 |  |  | 0.666 |
| ns | 3358 |  | 275 | src/private.rs (re-export surface) + src/var.rs (Var pointer wrapper) | 3.3 |  | 0.637 |
| ns | 3648 |  | 290 | src/display.rs — AsDisplay trait + Path Display impl | 3.4 |  | 0.607 |
| walker |  | 3716 | 527 | listing of 'tests/ui' |  |  | 0.611 |
| walker |  | 4050 | 334 | crate-doc tail at src/lib.rs:59 |  |  | 0.611 |
| ns | 4065 |  | 417 | impl/src/ast.rs — Input/Struct/Enum/Variant/Field/ContainerKind struct defs | 4.1 |  | 0.571 |
| walker |  | 4334 | 284 | crate-doc tail at src/lib.rs:99 |  |  | 0.608 |
| walker |  | 4385 | 51 | mod/use plumbing in impl/src/valid.rs |  |  | 0.608 |
| walker |  | 4438 | 53 | mod/use plumbing in impl/src/prop.rs |  |  | 0.608 |
| walker |  | 4510 | 72 | README.md section #2 |  |  | 0.608 |
| walker |  | 4566 | 56 | mod/use plumbing in impl/src/fallback.rs |  |  | 0.608 |
| ns | 4627 |  | 562 | impl/src/attr.rs — Attrs/Display/Source/From/Transparent/Fmt/Trait struct defs | 5.1 |  | 0.561 |
| ns | 4904 |  | 277 | impl/src/valid.rs — Struct::validate() dispatch | 6.1 |  | 0.542 |
| walker |  | 4955 | 389 | crate-doc tail at src/lib.rs:135 |  |  | 0.568 |
| walker |  | 5334 | 379 | crate-doc tail at src/lib.rs:176 |  |  | 0.568 |
| ns | 5382 |  | 478 | impl/src/valid.rs — check_non_field_attrs() | 6.2 |  | 0.541 |
| walker |  | 5464 | 130 | impl method sigs in impl/src/generics.rs |  |  | 0.541 |
| walker |  | 5464 | 0 | impl method at impl/src/generics.rs:13 |  |  | 0.541 |
| walker |  | 5464 | 0 | impl method at impl/src/generics.rs:19 |  |  | 0.541 |
| walker |  | 5464 | 0 | impl method at impl/src/generics.rs:54 |  |  | 0.541 |
| walker |  | 5464 | 0 | impl method at impl/src/generics.rs:61 |  |  | 0.541 |
| walker |  | 5464 | 0 | impl method at impl/src/generics.rs:74 |  |  | 0.541 |
| walker |  | 5496 | 32 | impl method body at impl/src/generics.rs:19 body 20 |  |  | 0.541 |
| walker |  | 5627 | 131 | impl method sigs in impl/src/valid.rs |  |  | 0.542 |
| walker |  | 5627 | 0 | impl method at impl/src/valid.rs:6 |  |  | 0.542 |
| walker |  | 5627 | 0 | impl method at impl/src/valid.rs:15 |  |  | 0.542 |
| walker |  | 5627 | 0 | impl method at impl/src/valid.rs:46 |  |  | 0.542 |
| walker |  | 5627 | 0 | impl method at impl/src/valid.rs:67 |  |  | 0.542 |
| walker |  | 5627 | 0 | impl method at impl/src/valid.rs:92 |  |  | 0.542 |
| walker |  | 5705 | 78 | pub-item names surface in impl/src/ast.rs |  |  | 0.544 |
| walker |  | 5731 | 26 | pub item at impl/src/ast.rs:10 |  |  | 0.545 |
| walker |  | 5794 | 63 | pub item at impl/src/ast.rs:45 |  |  | 0.553 |
| walker |  | 5847 | 53 | pub item at impl/src/ast.rs:15 |  |  | 0.561 |
| walker |  | 5900 | 53 | pub item at impl/src/ast.rs:29 |  |  | 0.571 |
| walker |  | 5954 | 54 | pub item at impl/src/ast.rs:22 |  |  | 0.584 |
| walker |  | 6019 | 65 | pub item at impl/src/ast.rs:36 |  |  | 0.602 |
| walker |  | 6116 | 97 | pub item body at impl/src/expand.rs:12 body 13 |  |  | 0.603 |
| walker |  | 6153 | 37 | impl method body at impl/src/generics.rs:13 body 14 |  |  | 0.603 |
| walker |  | 6190 | 37 | impl method body at impl/src/generics.rs:54 body 55 |  |  | 0.603 |
| ns | 6223 |  | 841 | impl/src/valid.rs — check_field_attrs() | 6.3 |  | 0.559 |
| walker |  | 6267 | 77 | mod/use plumbing in impl/src/scan_expr.rs |  |  | 0.559 |
| ns | 6445 |  | 222 | impl/src/expand.rs — derive()/try_expand() entry point | 7.1 |  | 0.555 |
| walker |  | 6808 | 541 | crate-doc tail at src/lib.rs:210 |  |  | 0.573 |
| walker |  | 6829 | 21 | pub item at impl/src/scan_expr.rs:192 |  |  | 0.573 |
| ns | 7008 |  | 563 | impl/src/expand.rs — impl_struct(): source() body construction | 7.2 |  | 0.551 |
| walker |  | 7134 | 305 | README.md section #12 |  |  | 0.551 |
| walker |  | 7246 | 112 | pub-item names surface in impl/src/attr.rs |  |  | 0.552 |
| walker |  | 7246 | 0 | pub item at impl/src/attr.rs:69 |  |  | 0.552 |
| walker |  | 7282 | 36 | pub item at impl/src/attr.rs:51 |  |  | 0.554 |
| walker |  | 7318 | 36 | pub item at impl/src/attr.rs:45 |  |  | 0.556 |
| ns | 7337 |  | 329 | impl/src/expand.rs — from_initializer(): generated From-impl body | 7.3 |  | 0.543 |
| walker |  | 7354 | 36 | pub item at impl/src/attr.rs:39 |  |  | 0.546 |
| walker |  | 7390 | 36 | pub item at impl/src/attr.rs:33 |  |  | 0.549 |
| ns | 7433 |  | 96 | impl/src/expand.rs — impl_enum() entry: per-variant source() dispatch | 7.4 |  | 0.546 |
| walker |  | 7482 | 92 | pub item at impl/src/attr.rs:57 |  |  | 0.558 |
| walker |  | 7572 | 90 | pub item at impl/src/attr.rs:11 |  |  | 0.572 |
| ns | 7634 |  | 201 | impl/src/fmt.rs — expand_shorthand() signature + setup | 8.1 |  | 0.565 |
| walker |  | 7684 | 112 | pub item at impl/src/attr.rs:21 |  |  | 0.589 |
| walker |  | 7783 | 99 | mod/use plumbing in impl/src/generics.rs |  |  | 0.589 |
| walker |  | 7883 | 100 | mod/use plumbing in impl/src/unraw.rs |  |  | 0.589 |
| walker |  | 7932 | 49 | impl method body at impl/src/unraw.rs:88 body 89 |  |  | 0.589 |
| walker |  | 7958 | 26 | pub item at impl/src/fallback.rs:7 |  |  | 0.589 |
| ns | 8048 |  | 414 | impl/src/fallback.rs (full file) | 9.1 |  | 0.574 |
| walker |  | 8070 | 112 | mod/use plumbing in impl/src/ast.rs |  |  | 0.574 |
| ns | 8077 |  | 29 | impl/src/generics.rs — ParamsInScope/InferredBounds struct locations | 9.2 |  | 0.576 |
| ns | 8128 |  | 51 | impl/src/unraw.rs + impl/src/scan_expr.rs — type/fn locations | 9.3 |  | 0.577 |
| walker |  | 8204 | 134 | README.md section #9 |  |  | 0.577 |
| ns | 8291 |  | 163 | impl/src/prop.rs — from_field/source_field/backtrace_field/has_* predicate locations | 9.4 |  | 0.571 |
| ns | 8485 |  | 194 | build.rs — rerun-if-changed / rustc-check-cfg declarations + OUT_DIR/private.rs generation | 10.1 |  | 0.565 |
| walker |  | 8521 | 317 | impl method sigs in impl/src/prop.rs |  |  | 0.579 |
| walker |  | 8521 | 0 | impl method at impl/src/prop.rs:7 |  |  | 0.579 |
| walker |  | 8521 | 0 | impl method at impl/src/prop.rs:11 |  |  | 0.579 |
| walker |  | 8521 | 0 | impl method at impl/src/prop.rs:15 |  |  | 0.579 |
| walker |  | 8521 | 0 | impl method at impl/src/prop.rs:19 |  |  | 0.579 |
| walker |  | 8521 | 0 | impl method at impl/src/prop.rs:26 |  |  | 0.579 |
| walker |  | 8521 | 0 | impl method at impl/src/prop.rs:32 |  |  | 0.579 |
| walker |  | 8521 | 0 | impl method at impl/src/prop.rs:38 |  |  | 0.579 |
| walker |  | 8521 | 0 | impl method at impl/src/prop.rs:54 |  |  | 0.579 |
| walker |  | 8521 | 0 | impl method at impl/src/prop.rs:58 |  |  | 0.579 |
| walker |  | 8521 | 0 | impl method at impl/src/prop.rs:62 |  |  | 0.579 |
| walker |  | 8521 | 0 | impl method at impl/src/prop.rs:66 |  |  | 0.579 |
| walker |  | 8521 | 0 | impl method at impl/src/prop.rs:73 |  |  | 0.579 |
| walker |  | 8521 | 0 | impl method at impl/src/prop.rs:77 |  |  | 0.579 |
| walker |  | 8545 | 24 | pub-item names surface in tests/test_backtrace.rs |  |  | 0.579 |
| walker |  | 8557 | 12 | pub item at tests/test_backtrace.rs:8 |  |  | 0.579 |
| ns | 8687 |  | 202 | ci.yml — header + every job key (roster) | 11.1 |  | 0.569 |
| walker |  | 8702 | 145 | mod/use plumbing in impl/src/expand.rs |  |  | 0.569 |
| walker |  | 8727 | 25 | pub-item names surface in tests/test_expr.rs |  |  | 0.569 |
| walker |  | 8752 | 25 | pub-item names surface in tests/test_path.rs |  |  | 0.569 |
| walker |  | 8903 | 151 | mod/use plumbing in impl/src/attr.rs |  |  | 0.569 |
| ns | 8951 |  | 264 | ci.yml — primary `test` job detail | 11.2 |  | 0.561 |
| ns | 8994 |  | 43 | rust-toolchain.toml + .gitignore + FUNDING.yml | 11.3 |  | 0.559 |
| ns | 9184 |  | 190 | tests/test_error.rs — canonical error shapes | 12.1 |  | 0.550 |
| walker |  | 9197 | 294 | impl method sigs in impl/src/ast.rs |  |  | 0.550 |
| walker |  | 9197 | 0 | impl method at impl/src/ast.rs:55 |  |  | 0.550 |
| walker |  | 9197 | 0 | impl method at impl/src/ast.rs:68 |  |  | 0.550 |
| walker |  | 9197 | 0 | impl method at impl/src/ast.rs:86 |  |  | 0.550 |
| walker |  | 9197 | 0 | impl method at impl/src/ast.rs:119 |  |  | 0.550 |
| walker |  | 9197 | 0 | impl method at impl/src/ast.rs:131 |  |  | 0.550 |
| walker |  | 9197 | 0 | impl method at impl/src/ast.rs:139 |  |  | 0.550 |
| walker |  | 9197 | 0 | impl method at impl/src/ast.rs:157 |  |  | 0.550 |
| walker |  | 9197 | 0 | impl method at impl/src/ast.rs:165 |  |  | 0.550 |
| walker |  | 9239 | 42 | impl method sigs in impl/src/fmt.rs |  |  | 0.551 |
| walker |  | 9239 | 0 | impl method at impl/src/fmt.rs:16 |  |  | 0.551 |
| walker |  | 9249 | 10 | pub item body at impl/src/attr.rs:69 body 121 |  |  | 0.551 |
| ns | 9388 |  | 204 | tests/test_from.rs — #[from] shapes (struct/tuple, plain and Option<T>) | 12.2 |  | 0.543 |
| walker |  | 9421 | 172 | README.md section #7 |  |  | 0.543 |
| walker |  | 9454 | 33 | pub item at tests/test_backtrace.rs:13 |  |  | 0.543 |
| walker |  | 9479 | 25 | pub item at src/aserror.rs:5 |  |  | 0.543 |
| walker |  | 9661 | 182 | README.md section #5 |  |  | 0.543 |
| walker |  | 9699 | 38 | pub item at tests/test_path.rs:29 |  |  | 0.543 |
| walker |  | 9891 | 192 | mod/use plumbing in impl/src/fmt.rs |  |  | 0.543 |
| ns | 9915 |  | 527 | tests/ui/ directory listing (all 37 compile-fail cases + their .stderr) | 13.1 |  | 0.578 |
