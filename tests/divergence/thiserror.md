Score(3000)=0.514 I=0.819 C=0.323 ns_rows≤3K=24/56 (reached=6 partial=2 missing=16)

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 43 | 43 | listing of '.' |  |  | 0.000 |
| walker |  | 46 | 3 | listing of 'build' |  |  | 0.000 |
| walker |  | 65 | 19 | listing of 'impl' |  |  | 0.000 |
| ns | 86 |  | 86 | Crate identity: name, one-line purpose, version | 1.1 |  | 0.000 |
| walker |  | 89 | 24 | listing of 'src' |  |  | 0.000 |
| ns | 129 |  | 43 | Complete repository root listing | 1.2 |  | 0.453 |
| walker |  | 136 | 47 | README headline in README.md |  |  | 0.635 |
| walker |  | 172 | 36 | [dependencies] in Cargo.toml |  |  | 0.637 |
| walker |  | 201 | 29 | headings outline in README.md |  |  | 0.640 |
| walker |  | 209 | 8 | listing of '.github' |  |  | 0.640 |
| walker |  | 212 | 3 | listing of '.github/workflows' |  |  | 0.640 |
| ns | 219 |  | 90 | Complete listings of both source trees: src/, impl/, impl/src/ | 1.3 |  | 0.461 |
| walker |  | 283 | 71 | mod/use plumbing in src/lib.rs |  |  | 0.468 |
| ns | 317 |  | 98 | README canonical example, head: derive, #[from], positional {0} | 1.4 |  | 0.408 |
| walker |  | 356 | 73 | listing of 'tests' |  |  | 0.411 |
| ns | 402 |  | 85 | README canonical example, tail: named-field variant, unit variant | 1.5 | 1.4 | 0.371 |
| ns | 480 |  | 78 | Derive entry point: #[proc_macro_derive(Error, attributes(...))] | 1.6 |  | 0.353 |
| walker |  | 541 | 185 | [package] in Cargo.toml |  |  | 0.510 |
| walker |  | 587 | 46 | listing of 'impl/src' |  |  | 0.719 |
| ns | 616 |  | 136 | Root crate module structure and public re-export (src/lib.rs tail) | 1.7 |  | 0.661 |
| walker |  | 629 | 42 | pub item at impl/src/lib.rs:40 |  |  | 0.672 |
| walker |  | 664 | 35 | impl method sigs in impl/src/lib.rs |  |  | 0.672 |
| walker |  | 664 | 0 | impl method at impl/src/lib.rs:49 |  |  | 0.672 |
| walker |  | 695 | 31 | pub item body at impl/src/lib.rs:40 body 41 |  |  | 0.697 |
| ns | 711 |  | 95 | Root crate attributes: no_std, docs.rs root, nightly cfg gate | 1.8 |  | 0.663 |
| walker |  | 767 | 72 | README.md section #0 |  |  | 0.701 |
| ns | 818 |  | 107 | Cargo manifest: features, dependency on impl, workspace members | 1.9 |  | 0.694 |
| ns | 874 |  | 56 | Documentation map: README section headings + rustdoc mirror in src/lib.rs | 2.1 |  | 0.679 |
| walker |  | 900 | 133 | mod/use plumbing in impl/src/lib.rs |  |  | 0.679 |
| ns | 1040 |  | 166 | Details: no public API footprint, allowed error shapes, Display generation | 2.2 |  | 0.639 |
| walker |  | 1167 | 267 | crate-doc lede in src/lib.rs |  |  | 0.639 |
| ns | 1226 |  | 186 | Display shorthand table: {var}, {0}, {var:?}, {0:?} | 2.3 |  | 0.611 |
| walker |  | 1363 | 196 | README.md section #1 |  |  | 0.739 |
| ns | 1408 |  | 182 | Details: #[from] generates From, with its field-count restriction | 2.4 |  | 0.693 |
| walker |  | 1527 | 164 | [features] in Cargo.toml |  |  | 0.716 |
| ns | 1561 |  | 153 | Details: source() from #[source] or a field named `source` | 2.5 |  | 0.689 |
| walker |  | 1600 | 73 | dev/build/target dependencies in Cargo.toml |  |  | 0.690 |
| walker |  | 1647 | 47 | impl method body at impl/src/lib.rs:49 body 50 |  |  | 0.690 |
| ns | 1713 |  | 152 | Details: #[error(transparent)] forwarding | 2.6 |  | 0.656 |
| walker |  | 1770 | 123 | manifest config in Cargo.toml |  |  | 0.656 |
| walker |  | 1823 | 53 | [dependencies] in impl/Cargo.toml |  |  | 0.656 |
| ns | 1843 |  | 130 | Details: provide() and automatic Backtrace field detection | 2.7 |  | 0.635 |
| walker |  | 1853 | 30 | README.md section #3 |  |  | 0.637 |
| ns | 1964 |  | 121 | Details: #[backtrace] on a source field forwards provide() | 2.8 |  | 0.626 |
| ns | 2009 |  | 45 | Details: #[from] variants with a Backtrace field capture in From | 2.9 |  | 0.621 |
| ns | 2092 |  | 83 | Details example: extra format arguments (`max = i32::MAX`) | 2.10 |  | 0.607 |
| walker |  | 2186 | 333 | crate-doc body in src/lib.rs |  |  | 0.619 |
| walker |  | 2214 | 28 | pub-item names surface in impl/src/unraw.rs |  |  | 0.619 |
| walker |  | 2231 | 17 | pub item at impl/src/unraw.rs:12 |  |  | 0.619 |
| ns | 2258 |  | 166 | Details example: referring to fields from format args via `.var` / `.0` | 2.11 |  | 0.596 |
| walker |  | 2264 | 33 | pub item at impl/src/unraw.rs:82 |  |  | 0.597 |
| walker |  | 2293 | 29 | pub-item names surface in impl/src/generics.rs |  |  | 0.597 |
| walker |  | 2311 | 18 | pub item at impl/src/generics.rs:8 |  |  | 0.597 |
| walker |  | 2352 | 41 | pub item at impl/src/generics.rs:48 |  |  | 0.597 |
| walker |  | 2396 | 44 | pub-item names surface in src/provide.rs |  |  | 0.597 |
| ns | 2414 |  | 156 | When to use thiserror vs anyhow | 2.12 |  | 0.581 |
| walker |  | 2441 | 45 | pub-item names surface in src/display.rs |  |  | 0.581 |
| walker |  | 2487 | 46 | pub-item names surface in src/aserror.rs |  |  | 0.582 |
| walker |  | 2495 | 8 | listing of 'tests/no-std' |  |  | 0.582 |
| ns | 2529 |  | 115 | impl/src/expand.rs: complete top-level function roster (names only) | 3.1 |  | 0.564 |
| walker |  | 2539 | 44 | pub-item names surface in impl/src/expand.rs |  |  | 0.565 |
| walker |  | 2539 | 0 | pub item at impl/src/expand.rs:12 |  |  | 0.565 |
| walker |  | 2539 | 0 | pub item at impl/src/expand.rs:505 |  |  | 0.565 |
| ns | 2727 |  | 198 | expand.rs: derive/try_expand — the whole expansion pipeline in 18 lines | 3.2 | 3.1 | 0.539 |
| walker |  | 2798 | 259 | crate-doc tail at src/lib.rs:45 |  |  | 0.539 |
| walker |  | 2920 | 122 | [package] in impl/Cargo.toml |  |  | 0.539 |
| ns | 2954 |  | 227 | expand.rs: the two emitted impl shapes (struct and enum quote! tails) | 3.3 |  | 0.514 |
| walker |  | 3079 | 159 | manifest config in impl/Cargo.toml |  |  | 0.514 |
| walker |  | 3138 | 59 | README.md section #11 |  |  | 0.518 |
| walker |  | 3223 | 85 | impl method sigs in impl/src/unraw.rs |  |  | 0.518 |
| walker |  | 3223 | 0 | impl method at impl/src/unraw.rs:15 |  |  | 0.518 |
| walker |  | 3223 | 0 | impl method at impl/src/unraw.rs:19 |  |  | 0.518 |
| walker |  | 3223 | 0 | impl method at impl/src/unraw.rs:32 |  |  | 0.518 |
| walker |  | 3223 | 0 | impl method at impl/src/unraw.rs:88 |  |  | 0.518 |
| walker |  | 3234 | 11 | impl method body at impl/src/unraw.rs:15 body 16 |  |  | 0.518 |
| walker |  | 3246 | 12 | impl method body at impl/src/unraw.rs:32 body 33 |  |  | 0.518 |
| ns | 3294 |  | 340 | impl/src/ast.rs: the complete IR (Input, Struct, Enum, Variant, Field) | 3.4 |  | 0.482 |
| ns | 3532 |  | 238 | impl/src/attr.rs: Attrs and the parsed Display attribute | 3.5 |  | 0.464 |
| walker |  | 3773 | 527 | listing of 'tests/ui' |  |  | 0.464 |
| ns | 3851 |  | 319 | attr.rs: Source/From/Transparent/Fmt payloads and the Trait enum | 3.6 |  | 0.435 |
| ns | 4044 |  | 193 | attr.rs: the three accepted forms of #[error(...)] | 3.7 |  | 0.425 |
| walker |  | 4107 | 334 | crate-doc tail at src/lib.rs:59 |  |  | 0.425 |
| ns | 4190 |  | 146 | impl/src/valid.rs: container-level diagnostic messages | 3.8 |  | 0.420 |
| walker |  | 4391 | 284 | crate-doc tail at src/lib.rs:99 |  |  | 0.420 |
| walker |  | 4442 | 51 | mod/use plumbing in impl/src/valid.rs |  |  | 0.420 |
| ns | 4482 |  | 292 | valid.rs: attribute-placement and field-attribute diagnostic messages | 3.9 |  | 0.411 |
| walker |  | 4495 | 53 | mod/use plumbing in impl/src/prop.rs |  |  | 0.411 |
| walker |  | 4567 | 72 | README.md section #2 |  |  | 0.422 |
| ns | 4651 |  | 169 | valid.rs: complete validate/check function roster | 3.10 |  | 0.414 |
| ns | 4863 |  | 212 | attr.rs: which attributes `get` recognises, plus the file's function roster | 3.11 |  | 0.404 |
| walker |  | 4956 | 389 | crate-doc tail at src/lib.rs:135 |  |  | 0.404 |
| ns | 5100 |  | 237 | impl/src/prop.rs: complete accessor roster for source/from/backtrace fields | 3.12 |  | 0.392 |
| ns | 5309 |  | 209 | prop.rs: how the source and backtrace fields are actually chosen | 3.13 |  | 0.381 |
| walker |  | 5335 | 379 | crate-doc tail at src/lib.rs:176 |  |  | 0.381 |
| walker |  | 5465 | 130 | impl method sigs in impl/src/generics.rs |  |  | 0.382 |
| walker |  | 5465 | 0 | impl method at impl/src/generics.rs:13 |  |  | 0.382 |
| walker |  | 5465 | 0 | impl method at impl/src/generics.rs:19 |  |  | 0.382 |
| walker |  | 5465 | 0 | impl method at impl/src/generics.rs:54 |  |  | 0.382 |
| walker |  | 5465 | 0 | impl method at impl/src/generics.rs:61 |  |  | 0.382 |
| walker |  | 5465 | 0 | impl method at impl/src/generics.rs:74 |  |  | 0.382 |
| walker |  | 5497 | 32 | impl method body at impl/src/generics.rs:19 body 20 |  |  | 0.382 |
| ns | 5599 |  | 290 | impl/src/fmt.rs: expand_shorthand and the format-spec → Trait mapping | 3.14 |  | 0.372 |
| walker |  | 5628 | 131 | impl method sigs in impl/src/valid.rs |  |  | 0.389 |
| walker |  | 5628 | 0 | impl method at impl/src/valid.rs:6 |  |  | 0.389 |
| walker |  | 5628 | 0 | impl method at impl/src/valid.rs:15 |  |  | 0.389 |
| walker |  | 5628 | 0 | impl method at impl/src/valid.rs:46 |  |  | 0.389 |
| walker |  | 5628 | 0 | impl method at impl/src/valid.rs:67 |  |  | 0.389 |
| walker |  | 5628 | 0 | impl method at impl/src/valid.rs:92 |  |  | 0.389 |
| walker |  | 5706 | 78 | pub-item names surface in impl/src/ast.rs |  |  | 0.391 |
| ns | 5730 |  | 131 | fmt.rs: FmtArguments and the remaining function roster | 3.15 | 3.14 | 0.386 |
| walker |  | 5732 | 26 | pub item at impl/src/ast.rs:10 |  |  | 0.389 |
| walker |  | 5795 | 63 | pub item at impl/src/ast.rs:45 |  |  | 0.389 |
| walker |  | 5848 | 53 | pub item at impl/src/ast.rs:15 |  |  | 0.396 |
| walker |  | 5901 | 53 | pub item at impl/src/ast.rs:29 |  |  | 0.407 |
| ns | 5909 |  | 179 | ast.rs: ContainerKind and its six display strings | 3.16 |  | 0.404 |
| walker |  | 5955 | 54 | pub item at impl/src/ast.rs:22 |  |  | 0.418 |
| walker |  | 6020 | 65 | pub item at impl/src/ast.rs:36 |  |  | 0.439 |
| walker |  | 6117 | 97 | pub item body at impl/src/expand.rs:12 body 13 |  |  | 0.449 |
| walker |  | 6154 | 37 | impl method body at impl/src/generics.rs:13 body 14 |  |  | 0.449 |
| walker |  | 6191 | 37 | impl method body at impl/src/generics.rs:54 body 55 |  |  | 0.449 |
| ns | 6202 |  | 293 | ast.rs: complete from_syn constructor roster | 3.17 |  | 0.440 |
| ns | 6361 |  | 159 | impl/src/generics.rs: ParamsInScope and InferredBounds | 3.18 |  | 0.453 |
| ns | 6610 |  | 249 | impl/src/unraw.rs: IdentUnraw/MemberUnraw and the raw-identifier rule | 3.19 |  | 0.447 |
| walker |  | 6732 | 541 | crate-doc tail at src/lib.rs:210 |  |  | 0.447 |
| ns | 6914 |  | 304 | impl/src/fallback.rs: the invalid-input fallback expansion | 3.20 |  | 0.437 |
| walker |  | 7037 | 305 | README.md section #12 |  |  | 0.454 |
| walker |  | 7060 | 23 | pub item at impl/src/scan_expr.rs:192 |  |  | 0.454 |
| ns | 7125 |  | 211 | impl/src/scan_expr.rs: the Input/Action alphabet of the expression scanner | 3.21 |  | 0.443 |
| walker |  | 7135 | 75 | mod/use plumbing in impl/src/scan_expr.rs |  |  | 0.443 |
| walker |  | 7247 | 112 | pub-item names surface in impl/src/attr.rs |  |  | 0.445 |
| walker |  | 7247 | 0 | pub item at impl/src/attr.rs:69 |  |  | 0.445 |
| walker |  | 7283 | 36 | pub item at impl/src/attr.rs:51 |  |  | 0.448 |
| walker |  | 7319 | 36 | pub item at impl/src/attr.rs:45 |  |  | 0.452 |
| walker |  | 7355 | 36 | pub item at impl/src/attr.rs:39 |  |  | 0.458 |
| walker |  | 7391 | 36 | pub item at impl/src/attr.rs:33 |  |  | 0.466 |
| ns | 7481 |  | 356 | expand.rs: the gate line for every conditionally generated impl | 3.22 | 3.1 | 0.457 |
| walker |  | 7483 | 92 | pub item at impl/src/attr.rs:57 |  |  | 0.481 |
| walker |  | 7573 | 90 | pub item at impl/src/attr.rs:11 |  |  | 0.487 |
| ns | 7641 |  | 160 | src/private.rs: the complete generated-code support surface | 4.1 |  | 0.481 |
| walker |  | 7685 | 112 | pub item at impl/src/attr.rs:21 |  |  | 0.502 |
| walker |  | 7784 | 99 | mod/use plumbing in impl/src/generics.rs |  |  | 0.502 |
| ns | 7856 |  | 215 | src/aserror.rs: AsDynError and its five blanket/dyn impls | 4.2 |  | 0.497 |
| walker |  | 7884 | 100 | mod/use plumbing in impl/src/unraw.rs |  |  | 0.497 |
| walker |  | 7933 | 49 | impl method body at impl/src/unraw.rs:88 body 89 |  |  | 0.497 |
| walker |  | 8045 | 112 | mod/use plumbing in impl/src/ast.rs |  |  | 0.497 |
| ns | 8098 |  | 242 | src/display.rs: AsDisplay and the std-only Path/PathBuf specializations | 4.3 |  | 0.489 |
| walker |  | 8179 | 134 | README.md section #9 |  |  | 0.492 |
| walker |  | 8209 | 30 | pub item at impl/src/fallback.rs:7 |  |  | 0.493 |
| walker |  | 8261 | 52 | mod/use plumbing in impl/src/fallback.rs |  |  | 0.493 |
| ns | 8380 |  | 282 | src/provide.rs (ThiserrorProvide) and src/var.rs (Var) end to end | 4.4 |  | 0.482 |
| ns | 8493 |  | 113 | impl/src/lib.rs: the version-stamped `private` path token | 4.5 |  | 0.483 |
| walker |  | 8578 | 317 | impl method sigs in impl/src/prop.rs |  |  | 0.495 |
| walker |  | 8578 | 0 | impl method at impl/src/prop.rs:7 |  |  | 0.495 |
| walker |  | 8578 | 0 | impl method at impl/src/prop.rs:11 |  |  | 0.495 |
| walker |  | 8578 | 0 | impl method at impl/src/prop.rs:15 |  |  | 0.495 |
| walker |  | 8578 | 0 | impl method at impl/src/prop.rs:19 |  |  | 0.495 |
| walker |  | 8578 | 0 | impl method at impl/src/prop.rs:26 |  |  | 0.495 |
| walker |  | 8578 | 0 | impl method at impl/src/prop.rs:32 |  |  | 0.495 |
| walker |  | 8578 | 0 | impl method at impl/src/prop.rs:38 |  |  | 0.495 |
| walker |  | 8578 | 0 | impl method at impl/src/prop.rs:54 |  |  | 0.495 |
| walker |  | 8578 | 0 | impl method at impl/src/prop.rs:58 |  |  | 0.495 |
| walker |  | 8578 | 0 | impl method at impl/src/prop.rs:62 |  |  | 0.495 |
| walker |  | 8578 | 0 | impl method at impl/src/prop.rs:66 |  |  | 0.495 |
| walker |  | 8578 | 0 | impl method at impl/src/prop.rs:73 |  |  | 0.495 |
| walker |  | 8578 | 0 | impl method at impl/src/prop.rs:77 |  |  | 0.495 |
| walker |  | 8602 | 24 | pub-item names surface in tests/test_backtrace.rs |  |  | 0.495 |
| walker |  | 8614 | 12 | pub item at tests/test_backtrace.rs:8 |  |  | 0.495 |
| ns | 8742 |  | 249 | build.rs: the generated __private module and the cfg declarations | 4.6 |  | 0.488 |
| walker |  | 8759 | 145 | mod/use plumbing in impl/src/expand.rs |  |  | 0.488 |
| walker |  | 8784 | 25 | pub-item names surface in tests/test_expr.rs |  |  | 0.488 |
| walker |  | 8809 | 25 | pub-item names surface in tests/test_path.rs |  |  | 0.488 |
| ns | 8950 |  | 208 | build.rs: rustc capability decisions, function roster, and the probe file | 4.7 |  | 0.483 |
| walker |  | 8960 | 151 | mod/use plumbing in impl/src/attr.rs |  |  | 0.483 |
| ns | 9047 |  | 97 | Complete tests/, tests/no-std/ and .github/ listings | 5.1 |  | 0.497 |
| ns | 9135 |  | 88 | tests/compiletest.rs: the trybuild UI harness in full | 5.2 |  | 0.494 |
| walker |  | 9254 | 294 | impl method sigs in impl/src/ast.rs |  |  | 0.507 |
| walker |  | 9254 | 0 | impl method at impl/src/ast.rs:55 |  |  | 0.507 |
| walker |  | 9254 | 0 | impl method at impl/src/ast.rs:68 |  |  | 0.507 |
| walker |  | 9254 | 0 | impl method at impl/src/ast.rs:86 |  |  | 0.507 |
| walker |  | 9254 | 0 | impl method at impl/src/ast.rs:119 |  |  | 0.507 |
| walker |  | 9254 | 0 | impl method at impl/src/ast.rs:131 |  |  | 0.507 |
| walker |  | 9254 | 0 | impl method at impl/src/ast.rs:139 |  |  | 0.507 |
| walker |  | 9254 | 0 | impl method at impl/src/ast.rs:157 |  |  | 0.507 |
| walker |  | 9254 | 0 | impl method at impl/src/ast.rs:165 |  |  | 0.507 |
| ns | 9266 |  | 131 | tests/no-std/test.rs: the no_std smoke test's error types | 5.3 |  | 0.502 |
| walker |  | 9296 | 42 | impl method sigs in impl/src/fmt.rs |  |  | 0.502 |
| walker |  | 9296 | 0 | impl method at impl/src/fmt.rs:16 |  |  | 0.502 |
| walker |  | 9306 | 10 | pub item body at impl/src/attr.rs:69 body 121 |  |  | 0.502 |
| ns | 9393 |  | 127 | Cargo.toml: std-feature rationale and dev-dependencies | 5.4 |  | 0.507 |
| walker |  | 9478 | 172 | README.md section #7 |  |  | 0.518 |
| walker |  | 9503 | 25 | pub item at src/aserror.rs:5 |  |  | 0.519 |
| ns | 9519 |  | 126 | impl/Cargo.toml: the proc-macro crate's manifest | 5.5 |  | 0.524 |
| walker |  | 9685 | 182 | README.md section #5 |  |  | 0.542 |
| ns | 9693 |  | 174 | CI job roster and the pinned toolchain components | 5.6 |  | 0.536 |
| walker |  | 9723 | 38 | pub item at tests/test_path.rs:29 |  |  | 0.536 |
| walker |  | 9915 | 192 | mod/use plumbing in impl/src/fmt.rs |  |  | 0.536 |
