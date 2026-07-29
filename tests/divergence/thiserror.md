Score(3000)=0.513 I=0.816 C=0.323 ns_rows≤3K=24/56 (reached=6 partial=2 missing=16)

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 43 | 43 | listing of '.' |  |  | 0.000 |
| walker |  | 46 | 3 | listing of 'build' |  |  | 0.000 |
| walker |  | 65 | 19 | listing of 'impl' |  |  | 0.000 |
| ns | 86 |  | 86 | Crate identity: name, one-line purpose, version | 1.1 |  | 0.000 |
| walker |  | 89 | 24 | listing of 'src' |  |  | 0.000 |
| ns | 129 |  | 43 | Complete repository root listing | 1.2 |  | 0.453 |
| walker |  | 136 | 47 | README headline in README.md |  |  | 0.635 |
| walker |  | 144 | 8 | listing of '.github' |  |  | 0.635 |
| walker |  | 147 | 3 | listing of '.github/workflows' |  |  | 0.636 |
| walker |  | 183 | 36 | [dependencies] in Cargo.toml |  |  | 0.637 |
| walker |  | 212 | 29 | headings outline in README.md |  |  | 0.640 |
| ns | 219 |  | 90 | Complete listings of both source trees: src/, impl/, impl/src/ | 1.3 |  | 0.461 |
| walker |  | 283 | 71 | mod/use plumbing in src/lib.rs |  |  | 0.468 |
| ns | 317 |  | 98 | README canonical example, head: derive, #[from], positional {0} | 1.4 |  | 0.408 |
| walker |  | 329 | 46 | listing of 'impl/src' |  |  | 0.624 |
| walker |  | 371 | 42 | pub item at impl/src/lib.rs:40 |  |  | 0.627 |
| ns | 402 |  | 85 | README canonical example, tail: named-field variant, unit variant | 1.5 | 1.4 | 0.567 |
| walker |  | 406 | 35 | impl method sigs in impl/src/lib.rs |  |  | 0.567 |
| walker |  | 406 | 0 | impl method at impl/src/lib.rs:49 |  |  | 0.567 |
| walker |  | 437 | 31 | pub item body at impl/src/lib.rs:40 body 41 |  |  | 0.572 |
| ns | 480 |  | 78 | Derive entry point: #[proc_macro_derive(Error, attributes(...))] | 1.6 |  | 0.574 |
| ns | 616 |  | 136 | Root crate module structure and public re-export (src/lib.rs tail) | 1.7 |  | 0.527 |
| walker |  | 622 | 185 | [package] in Cargo.toml |  |  | 0.695 |
| walker |  | 694 | 72 | README.md section #0 |  |  | 0.736 |
| ns | 711 |  | 95 | Root crate attributes: no_std, docs.rs root, nightly cfg gate | 1.8 |  | 0.700 |
| ns | 818 |  | 107 | Cargo manifest: features, dependency on impl, workspace members | 1.9 |  | 0.692 |
| walker |  | 827 | 133 | mod/use plumbing in impl/src/lib.rs |  |  | 0.692 |
| ns | 874 |  | 56 | Documentation map: README section headings + rustdoc mirror in src/lib.rs | 2.1 |  | 0.678 |
| ns | 1040 |  | 166 | Details: no public API footprint, allowed error shapes, Display generation | 2.2 |  | 0.638 |
| walker |  | 1094 | 267 | crate-doc lede in src/lib.rs |  |  | 0.638 |
| ns | 1226 |  | 186 | Display shorthand table: {var}, {0}, {var:?}, {0:?} | 2.3 |  | 0.609 |
| walker |  | 1290 | 196 | README.md section #1 |  |  | 0.737 |
| ns | 1408 |  | 182 | Details: #[from] generates From, with its field-count restriction | 2.4 |  | 0.691 |
| walker |  | 1454 | 164 | [features] in Cargo.toml |  |  | 0.714 |
| walker |  | 1527 | 73 | listing of 'tests' |  |  | 0.716 |
| ns | 1561 |  | 153 | Details: source() from #[source] or a field named `source` | 2.5 |  | 0.689 |
| walker |  | 1574 | 47 | impl method body at impl/src/lib.rs:49 body 50 |  |  | 0.689 |
| walker |  | 1697 | 123 | manifest config in Cargo.toml |  |  | 0.689 |
| ns | 1713 |  | 152 | Details: #[error(transparent)] forwarding | 2.6 |  | 0.655 |
| walker |  | 1750 | 53 | [dependencies] in impl/Cargo.toml |  |  | 0.656 |
| walker |  | 1780 | 30 | README.md section #3 |  |  | 0.657 |
| ns | 1843 |  | 130 | Details: provide() and automatic Backtrace field detection | 2.7 |  | 0.636 |
| ns | 1964 |  | 121 | Details: #[backtrace] on a source field forwards provide() | 2.8 |  | 0.625 |
| ns | 2009 |  | 45 | Details: #[from] variants with a Backtrace field capture in From | 2.9 |  | 0.621 |
| ns | 2092 |  | 83 | Details example: extra format arguments (`max = i32::MAX`) | 2.10 |  | 0.607 |
| walker |  | 2113 | 333 | crate-doc body in src/lib.rs |  |  | 0.619 |
| walker |  | 2141 | 28 | pub-item names surface in impl/src/unraw.rs |  |  | 0.619 |
| walker |  | 2158 | 17 | pub item at impl/src/unraw.rs:12 |  |  | 0.619 |
| walker |  | 2191 | 33 | pub item at impl/src/unraw.rs:82 |  |  | 0.619 |
| walker |  | 2220 | 29 | pub-item names surface in impl/src/generics.rs |  |  | 0.619 |
| walker |  | 2238 | 18 | pub item at impl/src/generics.rs:8 |  |  | 0.619 |
| ns | 2258 |  | 166 | Details example: referring to fields from format args via `.var` / `.0` | 2.11 |  | 0.596 |
| walker |  | 2279 | 41 | pub item at impl/src/generics.rs:48 |  |  | 0.597 |
| walker |  | 2287 | 8 | listing of 'tests/no-std' |  |  | 0.597 |
| ns | 2414 |  | 156 | When to use thiserror vs anyhow | 2.12 |  | 0.581 |
| ns | 2529 |  | 115 | impl/src/expand.rs: complete top-level function roster (names only) | 3.1 |  | 0.563 |
| ns | 2727 |  | 198 | expand.rs: derive/try_expand — the whole expansion pipeline in 18 lines | 3.2 | 3.1 | 0.537 |
| walker |  | 2814 | 527 | listing of 'tests/ui' |  |  | 0.537 |
| walker |  | 2858 | 44 | pub-item names surface in src/provide.rs |  |  | 0.537 |
| walker |  | 2903 | 45 | pub-item names surface in src/display.rs |  |  | 0.538 |
| walker |  | 2949 | 46 | pub-item names surface in src/aserror.rs |  |  | 0.538 |
| ns | 2954 |  | 227 | expand.rs: the two emitted impl shapes (struct and enum quote! tails) | 3.3 |  | 0.512 |
| walker |  | 2993 | 44 | pub-item names surface in impl/src/expand.rs |  |  | 0.513 |
| walker |  | 2993 | 0 | pub item at impl/src/expand.rs:12 |  |  | 0.513 |
| walker |  | 2993 | 0 | pub item at impl/src/expand.rs:505 |  |  | 0.513 |
| walker |  | 3252 | 259 | crate-doc tail at src/lib.rs:45 |  |  | 0.513 |
| ns | 3294 |  | 340 | impl/src/ast.rs: the complete IR (Input, Struct, Enum, Variant, Field) | 3.4 |  | 0.478 |
| walker |  | 3374 | 122 | [package] in impl/Cargo.toml |  |  | 0.478 |
| ns | 3532 |  | 238 | impl/src/attr.rs: Attrs and the parsed Display attribute | 3.5 |  | 0.459 |
| walker |  | 3533 | 159 | manifest config in impl/Cargo.toml |  |  | 0.460 |
| walker |  | 3592 | 59 | README.md section #11 |  |  | 0.463 |
| walker |  | 3677 | 85 | impl method sigs in impl/src/unraw.rs |  |  | 0.463 |
| walker |  | 3677 | 0 | impl method at impl/src/unraw.rs:15 |  |  | 0.463 |
| walker |  | 3677 | 0 | impl method at impl/src/unraw.rs:19 |  |  | 0.463 |
| walker |  | 3677 | 0 | impl method at impl/src/unraw.rs:32 |  |  | 0.463 |
| walker |  | 3677 | 0 | impl method at impl/src/unraw.rs:88 |  |  | 0.463 |
| walker |  | 3688 | 11 | impl method body at impl/src/unraw.rs:15 body 16 |  |  | 0.463 |
| walker |  | 3700 | 12 | impl method body at impl/src/unraw.rs:32 body 33 |  |  | 0.463 |
| ns | 3851 |  | 319 | attr.rs: Source/From/Transparent/Fmt payloads and the Trait enum | 3.6 |  | 0.434 |
| walker |  | 4034 | 334 | crate-doc tail at src/lib.rs:59 |  |  | 0.434 |
| ns | 4044 |  | 193 | attr.rs: the three accepted forms of #[error(...)] | 3.7 |  | 0.424 |
| ns | 4190 |  | 146 | impl/src/valid.rs: container-level diagnostic messages | 3.8 |  | 0.419 |
| walker |  | 4318 | 284 | crate-doc tail at src/lib.rs:99 |  |  | 0.419 |
| walker |  | 4369 | 51 | mod/use plumbing in impl/src/valid.rs |  |  | 0.419 |
| walker |  | 4422 | 53 | mod/use plumbing in impl/src/prop.rs |  |  | 0.419 |
| ns | 4482 |  | 292 | valid.rs: attribute-placement and field-attribute diagnostic messages | 3.9 |  | 0.411 |
| walker |  | 4494 | 72 | README.md section #2 |  |  | 0.422 |
| ns | 4651 |  | 169 | valid.rs: complete validate/check function roster | 3.10 |  | 0.413 |
| ns | 4863 |  | 212 | attr.rs: which attributes `get` recognises, plus the file's function roster | 3.11 |  | 0.404 |
| walker |  | 4883 | 389 | crate-doc tail at src/lib.rs:135 |  |  | 0.404 |
| ns | 5100 |  | 237 | impl/src/prop.rs: complete accessor roster for source/from/backtrace fields | 3.12 |  | 0.392 |
| walker |  | 5262 | 379 | crate-doc tail at src/lib.rs:176 |  |  | 0.392 |
| ns | 5309 |  | 209 | prop.rs: how the source and backtrace fields are actually chosen | 3.13 |  | 0.381 |
| walker |  | 5392 | 130 | impl method sigs in impl/src/generics.rs |  |  | 0.381 |
| walker |  | 5392 | 0 | impl method at impl/src/generics.rs:13 |  |  | 0.381 |
| walker |  | 5392 | 0 | impl method at impl/src/generics.rs:19 |  |  | 0.381 |
| walker |  | 5392 | 0 | impl method at impl/src/generics.rs:54 |  |  | 0.381 |
| walker |  | 5392 | 0 | impl method at impl/src/generics.rs:61 |  |  | 0.381 |
| walker |  | 5392 | 0 | impl method at impl/src/generics.rs:74 |  |  | 0.381 |
| walker |  | 5424 | 32 | impl method body at impl/src/generics.rs:19 body 20 |  |  | 0.381 |
| walker |  | 5555 | 131 | impl method sigs in impl/src/valid.rs |  |  | 0.399 |
| walker |  | 5555 | 0 | impl method at impl/src/valid.rs:6 |  |  | 0.399 |
| walker |  | 5555 | 0 | impl method at impl/src/valid.rs:15 |  |  | 0.399 |
| walker |  | 5555 | 0 | impl method at impl/src/valid.rs:46 |  |  | 0.399 |
| walker |  | 5555 | 0 | impl method at impl/src/valid.rs:67 |  |  | 0.399 |
| walker |  | 5555 | 0 | impl method at impl/src/valid.rs:92 |  |  | 0.399 |
| ns | 5599 |  | 290 | impl/src/fmt.rs: expand_shorthand and the format-spec → Trait mapping | 3.14 |  | 0.389 |
| walker |  | 5633 | 78 | pub-item names surface in impl/src/ast.rs |  |  | 0.391 |
| walker |  | 5659 | 26 | pub item at impl/src/ast.rs:10 |  |  | 0.393 |
| walker |  | 5722 | 63 | pub item at impl/src/ast.rs:45 |  |  | 0.394 |
| ns | 5730 |  | 131 | fmt.rs: FmtArguments and the remaining function roster | 3.15 | 3.14 | 0.388 |
| walker |  | 5775 | 53 | pub item at impl/src/ast.rs:15 |  |  | 0.396 |
| walker |  | 5828 | 53 | pub item at impl/src/ast.rs:29 |  |  | 0.407 |
| walker |  | 5882 | 54 | pub item at impl/src/ast.rs:22 |  |  | 0.422 |
| ns | 5909 |  | 179 | ast.rs: ContainerKind and its six display strings | 3.16 |  | 0.418 |
| walker |  | 5947 | 65 | pub item at impl/src/ast.rs:36 |  |  | 0.439 |
| walker |  | 6044 | 97 | pub item body at impl/src/expand.rs:12 body 13 |  |  | 0.448 |
| walker |  | 6081 | 37 | impl method body at impl/src/generics.rs:13 body 14 |  |  | 0.448 |
| walker |  | 6118 | 37 | impl method body at impl/src/generics.rs:54 body 55 |  |  | 0.448 |
| ns | 6202 |  | 293 | ast.rs: complete from_syn constructor roster | 3.17 |  | 0.440 |
| ns | 6361 |  | 159 | impl/src/generics.rs: ParamsInScope and InferredBounds | 3.18 |  | 0.452 |
| ns | 6610 |  | 249 | impl/src/unraw.rs: IdentUnraw/MemberUnraw and the raw-identifier rule | 3.19 |  | 0.447 |
| walker |  | 6659 | 541 | crate-doc tail at src/lib.rs:210 |  |  | 0.447 |
| ns | 6914 |  | 304 | impl/src/fallback.rs: the invalid-input fallback expansion | 3.20 |  | 0.437 |
| walker |  | 6964 | 305 | README.md section #12 |  |  | 0.453 |
| walker |  | 6987 | 23 | pub item at impl/src/scan_expr.rs:192 |  |  | 0.453 |
| walker |  | 7062 | 75 | mod/use plumbing in impl/src/scan_expr.rs |  |  | 0.453 |
| ns | 7125 |  | 211 | impl/src/scan_expr.rs: the Input/Action alphabet of the expression scanner | 3.21 |  | 0.442 |
| walker |  | 7174 | 112 | pub-item names surface in impl/src/attr.rs |  |  | 0.444 |
| walker |  | 7174 | 0 | pub item at impl/src/attr.rs:69 |  |  | 0.444 |
| walker |  | 7210 | 36 | pub item at impl/src/attr.rs:51 |  |  | 0.447 |
| walker |  | 7246 | 36 | pub item at impl/src/attr.rs:45 |  |  | 0.452 |
| walker |  | 7282 | 36 | pub item at impl/src/attr.rs:39 |  |  | 0.458 |
| walker |  | 7318 | 36 | pub item at impl/src/attr.rs:33 |  |  | 0.465 |
| walker |  | 7410 | 92 | pub item at impl/src/attr.rs:57 |  |  | 0.491 |
| ns | 7481 |  | 356 | expand.rs: the gate line for every conditionally generated impl | 3.22 | 3.1 | 0.481 |
| walker |  | 7500 | 90 | pub item at impl/src/attr.rs:11 |  |  | 0.487 |
| walker |  | 7612 | 112 | pub item at impl/src/attr.rs:21 |  |  | 0.508 |
| ns | 7641 |  | 160 | src/private.rs: the complete generated-code support surface | 4.1 |  | 0.501 |
| walker |  | 7711 | 99 | mod/use plumbing in impl/src/generics.rs |  |  | 0.501 |
| walker |  | 7811 | 100 | mod/use plumbing in impl/src/unraw.rs |  |  | 0.501 |
| ns | 7856 |  | 215 | src/aserror.rs: AsDynError and its five blanket/dyn impls | 4.2 |  | 0.496 |
| walker |  | 7860 | 49 | impl method body at impl/src/unraw.rs:88 body 89 |  |  | 0.496 |
| walker |  | 7972 | 112 | mod/use plumbing in impl/src/ast.rs |  |  | 0.496 |
| ns | 8098 |  | 242 | src/display.rs: AsDisplay and the std-only Path/PathBuf specializations | 4.3 |  | 0.489 |
| walker |  | 8106 | 134 | README.md section #9 |  |  | 0.492 |
| walker |  | 8136 | 30 | pub item at impl/src/fallback.rs:7 |  |  | 0.492 |
| walker |  | 8188 | 52 | mod/use plumbing in impl/src/fallback.rs |  |  | 0.492 |
| ns | 8380 |  | 282 | src/provide.rs (ThiserrorProvide) and src/var.rs (Var) end to end | 4.4 |  | 0.482 |
| ns | 8493 |  | 113 | impl/src/lib.rs: the version-stamped `private` path token | 4.5 |  | 0.483 |
| walker |  | 8505 | 317 | impl method sigs in impl/src/prop.rs |  |  | 0.494 |
| walker |  | 8505 | 0 | impl method at impl/src/prop.rs:7 |  |  | 0.494 |
| walker |  | 8505 | 0 | impl method at impl/src/prop.rs:11 |  |  | 0.494 |
| walker |  | 8505 | 0 | impl method at impl/src/prop.rs:15 |  |  | 0.494 |
| walker |  | 8505 | 0 | impl method at impl/src/prop.rs:19 |  |  | 0.494 |
| walker |  | 8505 | 0 | impl method at impl/src/prop.rs:26 |  |  | 0.494 |
| walker |  | 8505 | 0 | impl method at impl/src/prop.rs:32 |  |  | 0.494 |
| walker |  | 8505 | 0 | impl method at impl/src/prop.rs:38 |  |  | 0.494 |
| walker |  | 8505 | 0 | impl method at impl/src/prop.rs:54 |  |  | 0.494 |
| walker |  | 8505 | 0 | impl method at impl/src/prop.rs:58 |  |  | 0.494 |
| walker |  | 8505 | 0 | impl method at impl/src/prop.rs:62 |  |  | 0.494 |
| walker |  | 8505 | 0 | impl method at impl/src/prop.rs:66 |  |  | 0.494 |
| walker |  | 8505 | 0 | impl method at impl/src/prop.rs:73 |  |  | 0.494 |
| walker |  | 8505 | 0 | impl method at impl/src/prop.rs:77 |  |  | 0.494 |
| walker |  | 8529 | 24 | pub-item names surface in tests/test_backtrace.rs |  |  | 0.494 |
| walker |  | 8541 | 12 | pub item at tests/test_backtrace.rs:8 |  |  | 0.494 |
| walker |  | 8686 | 145 | mod/use plumbing in impl/src/expand.rs |  |  | 0.494 |
| walker |  | 8711 | 25 | pub-item names surface in tests/test_expr.rs |  |  | 0.494 |
| walker |  | 8736 | 25 | pub-item names surface in tests/test_path.rs |  |  | 0.494 |
| ns | 8742 |  | 249 | build.rs: the generated __private module and the cfg declarations | 4.6 |  | 0.487 |
| walker |  | 8887 | 151 | mod/use plumbing in impl/src/attr.rs |  |  | 0.487 |
| ns | 8950 |  | 208 | build.rs: rustc capability decisions, function roster, and the probe file | 4.7 |  | 0.483 |
| ns | 9047 |  | 97 | Complete tests/, tests/no-std/ and .github/ listings | 5.1 |  | 0.496 |
| ns | 9135 |  | 88 | tests/compiletest.rs: the trybuild UI harness in full | 5.2 |  | 0.494 |
| walker |  | 9181 | 294 | impl method sigs in impl/src/ast.rs |  |  | 0.507 |
| walker |  | 9181 | 0 | impl method at impl/src/ast.rs:55 |  |  | 0.507 |
| walker |  | 9181 | 0 | impl method at impl/src/ast.rs:68 |  |  | 0.507 |
| walker |  | 9181 | 0 | impl method at impl/src/ast.rs:86 |  |  | 0.507 |
| walker |  | 9181 | 0 | impl method at impl/src/ast.rs:119 |  |  | 0.507 |
| walker |  | 9181 | 0 | impl method at impl/src/ast.rs:131 |  |  | 0.507 |
| walker |  | 9181 | 0 | impl method at impl/src/ast.rs:139 |  |  | 0.507 |
| walker |  | 9181 | 0 | impl method at impl/src/ast.rs:157 |  |  | 0.507 |
| walker |  | 9181 | 0 | impl method at impl/src/ast.rs:165 |  |  | 0.507 |
| walker |  | 9223 | 42 | impl method sigs in impl/src/fmt.rs |  |  | 0.507 |
| walker |  | 9223 | 0 | impl method at impl/src/fmt.rs:16 |  |  | 0.507 |
| walker |  | 9233 | 10 | pub item body at impl/src/attr.rs:69 body 121 |  |  | 0.507 |
| ns | 9266 |  | 131 | tests/no-std/test.rs: the no_std smoke test's error types | 5.3 |  | 0.501 |
| ns | 9393 |  | 127 | Cargo.toml: std-feature rationale and dev-dependencies | 5.4 |  | 0.500 |
| walker |  | 9405 | 172 | README.md section #7 |  |  | 0.511 |
| walker |  | 9430 | 25 | pub item at src/aserror.rs:5 |  |  | 0.513 |
| ns | 9519 |  | 126 | impl/Cargo.toml: the proc-macro crate's manifest | 5.5 |  | 0.518 |
| walker |  | 9612 | 182 | README.md section #5 |  |  | 0.536 |
| walker |  | 9685 | 73 | dev/build/target dependencies in Cargo.toml |  |  | 0.542 |
| ns | 9693 |  | 174 | CI job roster and the pinned toolchain components | 5.6 |  | 0.536 |
| walker |  | 9723 | 38 | pub item at tests/test_path.rs:29 |  |  | 0.536 |
| walker |  | 9915 | 192 | mod/use plumbing in impl/src/fmt.rs |  |  | 0.536 |
