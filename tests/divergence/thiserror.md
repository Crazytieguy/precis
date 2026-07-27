Score(3000)=0.661 I=0.872 C=0.502 ns_rows≤3K=21/45 (reached=11 partial=2 missing=8)

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
| walker |  | 209 | 8 | listing of '.github' |  |  | 0.486 |
| ns | 211 |  | 15 | build/, .github/, .github/workflows/ listings | 1.4 |  | 0.497 |
| walker |  | 212 | 3 | listing of '.github/workflows' |  |  | 0.526 |
| ns | 261 |  | 50 | Cargo.toml — package identity (name/version/authors) | 1.5 |  | 0.501 |
| walker |  | 283 | 71 | mod/use plumbing in src/lib.rs |  |  | 0.502 |
| walker |  | 356 | 73 | listing of 'tests' |  |  | 0.683 |
| ns | 374 |  | 113 | Cargo.toml — categories/description/docs/edition/keywords/license/repo/rust-version | 1.6 |  | 0.627 |
| ns | 395 |  | 21 | Cargo.toml — [features] header | 1.7 |  | 0.609 |
| ns | 540 |  | 145 | Cargo.toml — std feature doc comment | 1.8 |  | 0.549 |
| walker |  | 541 | 185 | [package] in Cargo.toml |  |  | 0.669 |
| ns | 572 |  | 32 | Cargo.toml — [dependencies] | 1.9 |  | 0.674 |
| walker |  | 587 | 46 | listing of 'impl/src' |  |  | 0.847 |
| ns | 596 |  | 24 | Cargo.toml — [workspace] | 1.10 |  | 0.848 |
| walker |  | 629 | 42 | pub item at impl/src/lib.rs:40 |  |  | 0.849 |
| walker |  | 664 | 35 | impl method sigs in impl/src/lib.rs |  |  | 0.849 |
| walker |  | 664 | 0 | impl method at impl/src/lib.rs:49 |  |  | 0.849 |
| ns | 675 |  | 79 | impl/Cargo.toml — package identity (name/version/authors/description/edition) | 1.11 |  | 0.808 |
| walker |  | 695 | 31 | pub item body at impl/src/lib.rs:40 body 41 |  |  | 0.809 |
| ns | 748 |  | 73 | impl/Cargo.toml — [lib]/[dependencies] | 1.12 |  | 0.772 |
| walker |  | 767 | 72 | README.md section #0 |  |  | 0.772 |
| ns | 839 |  | 91 | impl/src/lib.rs — module list | 1.13 |  | 0.719 |
| walker |  | 900 | 133 | mod/use plumbing in impl/src/lib.rs |  |  | 0.774 |
| ns | 1091 |  | 252 | impl/src/lib.rs — #[proc_macro_derive] entry point | 1.14 |  | 0.727 |
| walker |  | 1167 | 267 | crate-doc lede in src/lib.rs |  |  | 0.727 |
| ns | 1327 |  | 236 | src/lib.rs — module wiring epilogue | 1.15 |  | 0.668 |
| walker |  | 1363 | 196 | README.md section #1 |  |  | 0.668 |
| walker |  | 1527 | 164 | [features] in Cargo.toml |  |  | 0.746 |
| walker |  | 1600 | 73 | dev/build/target dependencies in Cargo.toml |  |  | 0.746 |
| ns | 1604 |  | 277 | Crate-doc lede + canonical example | 2.1 |  | 0.683 |
| walker |  | 1647 | 47 | impl method body at impl/src/lib.rs:49 body 50 |  |  | 0.707 |
| walker |  | 1770 | 123 | manifest config in Cargo.toml |  |  | 0.707 |
| walker |  | 1823 | 53 | [dependencies] in impl/Cargo.toml |  |  | 0.719 |
| walker |  | 1853 | 30 | README.md section #3 |  |  | 0.719 |
| ns | 1950 |  | 346 | Display-shorthand table + #[from] rule statement | 2.2 |  | 0.680 |
| ns | 2177 |  | 227 | #[from] example + #[source] rule statement | 2.3 |  | 0.653 |
| walker |  | 2186 | 333 | crate-doc body in src/lib.rs |  |  | 0.733 |
| ns | 2271 |  | 94 | provide()/backtrace rule statement | 2.4 |  | 0.723 |
| walker |  | 2330 | 144 | manifest config in impl/Cargo.toml |  |  | 0.723 |
| walker |  | 2358 | 28 | pub-item names surface in impl/src/unraw.rs |  |  | 0.723 |
| walker |  | 2375 | 17 | pub item at impl/src/unraw.rs:12 |  |  | 0.723 |
| walker |  | 2408 | 33 | pub item at impl/src/unraw.rs:82 |  |  | 0.723 |
| walker |  | 2437 | 29 | pub-item names surface in impl/src/generics.rs |  |  | 0.723 |
| walker |  | 2455 | 18 | pub item at impl/src/generics.rs:8 |  |  | 0.695 |
| ns | 2455 |  | 184 | #[error(transparent)] rule + 'anything else' variant example | 2.5 |  | 0.695 |
| walker |  | 2496 | 41 | pub item at impl/src/generics.rs:48 |  |  | 0.695 |
| walker |  | 2540 | 44 | pub-item names surface in src/provide.rs |  |  | 0.695 |
| walker |  | 2585 | 45 | pub-item names surface in src/display.rs |  |  | 0.695 |
| walker |  | 2631 | 46 | pub-item names surface in src/aserror.rs |  |  | 0.695 |
| walker |  | 2639 | 8 | listing of 'tests/no-std' |  |  | 0.712 |
| walker |  | 2683 | 44 | pub-item names surface in impl/src/expand.rs |  |  | 0.712 |
| walker |  | 2683 | 0 | pub item at impl/src/expand.rs:12 |  |  | 0.712 |
| walker |  | 2683 | 0 | pub item at impl/src/expand.rs:505 |  |  | 0.712 |
| ns | 2916 |  | 461 | src/aserror.rs — AsDynError trait + impls | 3.1 |  | 0.644 |
| walker |  | 2942 | 259 | crate-doc tail at src/lib.rs:45 |  |  | 0.661 |
| walker |  | 3064 | 122 | [package] in impl/Cargo.toml |  |  | 0.683 |
| ns | 3083 |  | 167 | src/provide.rs — ThiserrorProvide trait + blanket impl (cfg-gated) | 3.2 |  | 0.661 |
| walker |  | 3123 | 59 | README.md section #11 |  |  | 0.661 |
| walker |  | 3208 | 85 | impl method sigs in impl/src/unraw.rs |  |  | 0.661 |
| walker |  | 3208 | 0 | impl method at impl/src/unraw.rs:15 |  |  | 0.661 |
| walker |  | 3208 | 0 | impl method at impl/src/unraw.rs:19 |  |  | 0.661 |
| walker |  | 3208 | 0 | impl method at impl/src/unraw.rs:32 |  |  | 0.661 |
| walker |  | 3208 | 0 | impl method at impl/src/unraw.rs:88 |  |  | 0.661 |
| walker |  | 3219 | 11 | impl method body at impl/src/unraw.rs:15 body 16 |  |  | 0.661 |
| walker |  | 3231 | 12 | impl method body at impl/src/unraw.rs:32 body 33 |  |  | 0.661 |
| ns | 3358 |  | 275 | src/private.rs (re-export surface) + src/var.rs (Var pointer wrapper) | 3.3 |  | 0.632 |
| ns | 3648 |  | 290 | src/display.rs — AsDisplay trait + Path Display impl | 3.4 |  | 0.602 |
| walker |  | 3758 | 527 | listing of 'tests/ui' |  |  | 0.606 |
| ns | 4065 |  | 417 | impl/src/ast.rs — Input/Struct/Enum/Variant/Field/ContainerKind struct defs | 4.1 |  | 0.566 |
| walker |  | 4092 | 334 | crate-doc tail at src/lib.rs:59 |  |  | 0.566 |
| walker |  | 4376 | 284 | crate-doc tail at src/lib.rs:99 |  |  | 0.603 |
| walker |  | 4427 | 51 | mod/use plumbing in impl/src/valid.rs |  |  | 0.603 |
| walker |  | 4480 | 53 | mod/use plumbing in impl/src/prop.rs |  |  | 0.603 |
| walker |  | 4552 | 72 | README.md section #2 |  |  | 0.603 |
| walker |  | 4608 | 56 | mod/use plumbing in impl/src/fallback.rs |  |  | 0.603 |
| ns | 4627 |  | 562 | impl/src/attr.rs — Attrs/Display/Source/From/Transparent/Fmt/Trait struct defs | 5.1 |  | 0.557 |
| ns | 4904 |  | 277 | impl/src/valid.rs — Struct::validate() dispatch | 6.1 |  | 0.538 |
| walker |  | 4997 | 389 | crate-doc tail at src/lib.rs:135 |  |  | 0.564 |
| walker |  | 5376 | 379 | crate-doc tail at src/lib.rs:176 |  |  | 0.564 |
| ns | 5382 |  | 478 | impl/src/valid.rs — check_non_field_attrs() | 6.2 |  | 0.538 |
| walker |  | 5506 | 130 | impl method sigs in impl/src/generics.rs |  |  | 0.538 |
| walker |  | 5506 | 0 | impl method at impl/src/generics.rs:13 |  |  | 0.538 |
| walker |  | 5506 | 0 | impl method at impl/src/generics.rs:19 |  |  | 0.538 |
| walker |  | 5506 | 0 | impl method at impl/src/generics.rs:54 |  |  | 0.538 |
| walker |  | 5506 | 0 | impl method at impl/src/generics.rs:61 |  |  | 0.538 |
| walker |  | 5506 | 0 | impl method at impl/src/generics.rs:74 |  |  | 0.538 |
| walker |  | 5538 | 32 | impl method body at impl/src/generics.rs:19 body 20 |  |  | 0.538 |
| walker |  | 5669 | 131 | impl method sigs in impl/src/valid.rs |  |  | 0.538 |
| walker |  | 5669 | 0 | impl method at impl/src/valid.rs:6 |  |  | 0.538 |
| walker |  | 5669 | 0 | impl method at impl/src/valid.rs:15 |  |  | 0.538 |
| walker |  | 5669 | 0 | impl method at impl/src/valid.rs:46 |  |  | 0.538 |
| walker |  | 5669 | 0 | impl method at impl/src/valid.rs:67 |  |  | 0.538 |
| walker |  | 5669 | 0 | impl method at impl/src/valid.rs:92 |  |  | 0.538 |
| walker |  | 5747 | 78 | pub-item names surface in impl/src/ast.rs |  |  | 0.540 |
| walker |  | 5773 | 26 | pub item at impl/src/ast.rs:10 |  |  | 0.542 |
| walker |  | 5836 | 63 | pub item at impl/src/ast.rs:45 |  |  | 0.549 |
| walker |  | 5889 | 53 | pub item at impl/src/ast.rs:15 |  |  | 0.557 |
| walker |  | 5942 | 53 | pub item at impl/src/ast.rs:29 |  |  | 0.568 |
| walker |  | 5996 | 54 | pub item at impl/src/ast.rs:22 |  |  | 0.581 |
| walker |  | 6061 | 65 | pub item at impl/src/ast.rs:36 |  |  | 0.599 |
| walker |  | 6158 | 97 | pub item body at impl/src/expand.rs:12 body 13 |  |  | 0.599 |
| walker |  | 6195 | 37 | impl method body at impl/src/generics.rs:13 body 14 |  |  | 0.599 |
| ns | 6223 |  | 841 | impl/src/valid.rs — check_field_attrs() | 6.3 |  | 0.555 |
| walker |  | 6232 | 37 | impl method body at impl/src/generics.rs:54 body 55 |  |  | 0.555 |
| walker |  | 6309 | 77 | mod/use plumbing in impl/src/scan_expr.rs |  |  | 0.555 |
| ns | 6445 |  | 222 | impl/src/expand.rs — derive()/try_expand() entry point | 7.1 |  | 0.552 |
| walker |  | 6850 | 541 | crate-doc tail at src/lib.rs:210 |  |  | 0.570 |
| walker |  | 6871 | 21 | pub item at impl/src/scan_expr.rs:192 |  |  | 0.570 |
| ns | 7008 |  | 563 | impl/src/expand.rs — impl_struct(): source() body construction | 7.2 |  | 0.548 |
| walker |  | 7176 | 305 | README.md section #12 |  |  | 0.548 |
| walker |  | 7288 | 112 | pub-item names surface in impl/src/attr.rs |  |  | 0.549 |
| walker |  | 7288 | 0 | pub item at impl/src/attr.rs:69 |  |  | 0.549 |
| walker |  | 7324 | 36 | pub item at impl/src/attr.rs:51 |  |  | 0.551 |
| ns | 7337 |  | 329 | impl/src/expand.rs — from_initializer(): generated From-impl body | 7.3 |  | 0.537 |
| walker |  | 7360 | 36 | pub item at impl/src/attr.rs:45 |  |  | 0.540 |
| walker |  | 7396 | 36 | pub item at impl/src/attr.rs:39 |  |  | 0.543 |
| walker |  | 7432 | 36 | pub item at impl/src/attr.rs:33 |  |  | 0.546 |
| ns | 7433 |  | 96 | impl/src/expand.rs — impl_enum() entry: per-variant source() dispatch | 7.4 |  | 0.543 |
| walker |  | 7524 | 92 | pub item at impl/src/attr.rs:57 |  |  | 0.555 |
| walker |  | 7614 | 90 | pub item at impl/src/attr.rs:11 |  |  | 0.569 |
| ns | 7634 |  | 201 | impl/src/fmt.rs — expand_shorthand() signature + setup | 8.1 |  | 0.562 |
| walker |  | 7726 | 112 | pub item at impl/src/attr.rs:21 |  |  | 0.586 |
| walker |  | 7825 | 99 | mod/use plumbing in impl/src/generics.rs |  |  | 0.586 |
| walker |  | 7925 | 100 | mod/use plumbing in impl/src/unraw.rs |  |  | 0.586 |
| walker |  | 7974 | 49 | impl method body at impl/src/unraw.rs:88 body 89 |  |  | 0.586 |
| walker |  | 8000 | 26 | pub item at impl/src/fallback.rs:7 |  |  | 0.586 |
| ns | 8048 |  | 414 | impl/src/fallback.rs (full file) | 9.1 |  | 0.572 |
| ns | 8077 |  | 29 | impl/src/generics.rs — ParamsInScope/InferredBounds struct locations | 9.2 |  | 0.573 |
| walker |  | 8112 | 112 | mod/use plumbing in impl/src/ast.rs |  |  | 0.573 |
| ns | 8128 |  | 51 | impl/src/unraw.rs + impl/src/scan_expr.rs — type/fn locations | 9.3 |  | 0.574 |
| walker |  | 8246 | 134 | README.md section #9 |  |  | 0.574 |
| ns | 8291 |  | 163 | impl/src/prop.rs — from_field/source_field/backtrace_field/has_* predicate locations | 9.4 |  | 0.568 |
| ns | 8485 |  | 194 | build.rs — rerun-if-changed / rustc-check-cfg declarations + OUT_DIR/private.rs generation | 10.1 |  | 0.563 |
| walker |  | 8563 | 317 | impl method sigs in impl/src/prop.rs |  |  | 0.576 |
| walker |  | 8563 | 0 | impl method at impl/src/prop.rs:7 |  |  | 0.576 |
| walker |  | 8563 | 0 | impl method at impl/src/prop.rs:11 |  |  | 0.576 |
| walker |  | 8563 | 0 | impl method at impl/src/prop.rs:15 |  |  | 0.576 |
| walker |  | 8563 | 0 | impl method at impl/src/prop.rs:19 |  |  | 0.576 |
| walker |  | 8563 | 0 | impl method at impl/src/prop.rs:26 |  |  | 0.576 |
| walker |  | 8563 | 0 | impl method at impl/src/prop.rs:32 |  |  | 0.576 |
| walker |  | 8563 | 0 | impl method at impl/src/prop.rs:38 |  |  | 0.576 |
| walker |  | 8563 | 0 | impl method at impl/src/prop.rs:54 |  |  | 0.576 |
| walker |  | 8563 | 0 | impl method at impl/src/prop.rs:58 |  |  | 0.576 |
| walker |  | 8563 | 0 | impl method at impl/src/prop.rs:62 |  |  | 0.576 |
| walker |  | 8563 | 0 | impl method at impl/src/prop.rs:66 |  |  | 0.576 |
| walker |  | 8563 | 0 | impl method at impl/src/prop.rs:73 |  |  | 0.576 |
| walker |  | 8563 | 0 | impl method at impl/src/prop.rs:77 |  |  | 0.576 |
| walker |  | 8587 | 24 | pub-item names surface in tests/test_backtrace.rs |  |  | 0.576 |
| walker |  | 8599 | 12 | pub item at tests/test_backtrace.rs:8 |  |  | 0.576 |
| ns | 8687 |  | 202 | ci.yml — header + every job key (roster) | 11.1 |  | 0.567 |
| walker |  | 8744 | 145 | mod/use plumbing in impl/src/expand.rs |  |  | 0.567 |
| walker |  | 8769 | 25 | pub-item names surface in tests/test_expr.rs |  |  | 0.567 |
| walker |  | 8794 | 25 | pub-item names surface in tests/test_path.rs |  |  | 0.567 |
| walker |  | 8945 | 151 | mod/use plumbing in impl/src/attr.rs |  |  | 0.567 |
| ns | 8951 |  | 264 | ci.yml — primary `test` job detail | 11.2 |  | 0.559 |
| ns | 8994 |  | 43 | rust-toolchain.toml + .gitignore + FUNDING.yml | 11.3 |  | 0.557 |
| ns | 9184 |  | 190 | tests/test_error.rs — canonical error shapes | 12.1 |  | 0.548 |
| walker |  | 9239 | 294 | impl method sigs in impl/src/ast.rs |  |  | 0.548 |
| walker |  | 9239 | 0 | impl method at impl/src/ast.rs:55 |  |  | 0.548 |
| walker |  | 9239 | 0 | impl method at impl/src/ast.rs:68 |  |  | 0.548 |
| walker |  | 9239 | 0 | impl method at impl/src/ast.rs:86 |  |  | 0.548 |
| walker |  | 9239 | 0 | impl method at impl/src/ast.rs:119 |  |  | 0.548 |
| walker |  | 9239 | 0 | impl method at impl/src/ast.rs:131 |  |  | 0.548 |
| walker |  | 9239 | 0 | impl method at impl/src/ast.rs:139 |  |  | 0.548 |
| walker |  | 9239 | 0 | impl method at impl/src/ast.rs:157 |  |  | 0.548 |
| walker |  | 9239 | 0 | impl method at impl/src/ast.rs:165 |  |  | 0.548 |
| walker |  | 9281 | 42 | impl method sigs in impl/src/fmt.rs |  |  | 0.548 |
| walker |  | 9281 | 0 | impl method at impl/src/fmt.rs:16 |  |  | 0.548 |
| walker |  | 9291 | 10 | pub item body at impl/src/attr.rs:69 body 121 |  |  | 0.548 |
| ns | 9388 |  | 204 | tests/test_from.rs — #[from] shapes (struct/tuple, plain and Option<T>) | 12.2 |  | 0.540 |
| walker |  | 9463 | 172 | README.md section #7 |  |  | 0.540 |
| walker |  | 9488 | 25 | pub item at src/aserror.rs:5 |  |  | 0.540 |
| walker |  | 9670 | 182 | README.md section #5 |  |  | 0.540 |
| walker |  | 9708 | 38 | pub item at tests/test_path.rs:29 |  |  | 0.540 |
| walker |  | 9900 | 192 | mod/use plumbing in impl/src/fmt.rs |  |  | 0.540 |
| ns | 9915 |  | 527 | tests/ui/ directory listing (all 37 compile-fail cases + their .stderr) | 13.1 |  | 0.575 |
