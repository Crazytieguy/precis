Score(3000)=0.509 I=0.833 C=0.311 ns_rows≤3K=24/56 grid(1000/1442/2080/3000/4327/6240/9000)=0.657/0.677/0.622/0.509/0.486/0.536/0.523

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 38 | 38 | listing of '.' |  |  | 0.000 |
| walker |  | 42 | 4 | listing of 'build' |  |  | 0.000 |
| walker |  | 61 | 19 | listing of 'impl' |  |  | 0.000 |
| walker |  | 86 | 25 | listing of 'src' |  |  | 0.000 |
| ns | 86 |  | 86 | Crate identity: name, one-line purpose, version | 1.1 |  | 0.000 |
| ns | 124 |  | 38 | Complete repository root listing | 1.2 |  | 0.453 |
| walker |  | 133 | 47 | README headline in README.md |  |  | 0.635 |
| walker |  | 204 | 71 | rust names src/lib.rs |  |  | 0.645 |
| walker |  | 212 | 8 | listing of '.github' |  |  | 0.645 |
| ns | 215 |  | 91 | Complete listings of both source trees: src/, impl/, impl/src/ | 1.3 |  | 0.465 |
| walker |  | 216 | 4 | listing of '.github/workflows' |  |  | 0.465 |
| walker |  | 252 | 36 | [dependencies] in Cargo.toml |  |  | 0.466 |
| walker |  | 281 | 29 | headings outline in README.md |  |  | 0.468 |
| ns | 313 |  | 98 | README canonical example, head: derive, #[from], positional {0} | 1.4 |  | 0.408 |
| walker |  | 328 | 47 | listing of 'impl/src' |  |  | 0.624 |
| ns | 398 |  | 85 | README canonical example, tail: named-field variant, unit variant | 1.5 | 1.4 | 0.565 |
| walker |  | 411 | 83 | rust names build.rs |  |  | 0.565 |
| ns | 476 |  | 78 | Derive entry point: #[proc_macro_derive(Error, attributes(...))] | 1.6 |  | 0.538 |
| walker |  | 596 | 185 | [package] in Cargo.toml |  |  | 0.717 |
| ns | 612 |  | 136 | Root crate module structure and public re-export (src/lib.rs tail) | 1.7 |  | 0.660 |
| walker |  | 668 | 72 | README.md section #0 |  |  | 0.700 |
| ns | 707 |  | 95 | Root crate attributes: no_std, docs.rs root, nightly cfg gate | 1.8 |  | 0.665 |
| walker |  | 721 | 53 | rust names src/var.rs |  |  | 0.665 |
| walker |  | 749 | 28 | rust decl src/var.rs:5 |  |  | 0.665 |
| ns | 814 |  | 107 | Cargo manifest: features, dependency on impl, workspace members | 1.9 |  | 0.661 |
| walker |  | 869 | 120 | rust names impl/src/lib.rs |  |  | 0.663 |
| ns | 870 |  | 56 | Documentation map: README section headings + rustdoc mirror in src/lib.rs | 2.1 |  | 0.650 |
| walker |  | 890 | 21 | rust decl impl/src/lib.rs:39 |  |  | 0.657 |
| walker |  | 902 | 12 | rust decl impl/src/lib.rs:45 |  |  | 0.657 |
| walker |  | 930 | 28 | rust decl impl/src/lib.rs:48 |  |  | 0.657 |
| walker |  | 976 | 46 | rust decl build.rs:10 |  |  | 0.657 |
| ns | 1036 |  | 166 | Details: no public API footprint, allowed error shapes, Display generation | 2.2 |  | 0.618 |
| walker |  | 1057 | 81 | rust names build/probe.rs |  |  | 0.618 |
| ns | 1222 |  | 186 | Display shorthand table: {var}, {0}, {var:?}, {0:?} | 2.3 |  | 0.591 |
| walker |  | 1253 | 196 | README.md section #1 |  |  | 0.720 |
| walker |  | 1282 | 29 | rust decl build/probe.rs:14 |  |  | 0.720 |
| walker |  | 1311 | 29 | rust decl build/probe.rs:20 |  |  | 0.720 |
| walker |  | 1341 | 30 | rust decl build/probe.rs:26 |  |  | 0.720 |
| ns | 1404 |  | 182 | Details: #[from] generates From, with its field-count restriction | 2.4 |  | 0.675 |
| walker |  | 1413 | 72 | listing of 'tests' |  |  | 0.677 |
| walker |  | 1444 | 31 | rust body impl/src/lib.rs:39 |  |  | 0.694 |
| walker |  | 1474 | 30 | rust names impl/src/fallback.rs |  |  | 0.694 |
| walker |  | 1488 | 14 | rust body src/var.rs:6 |  |  | 0.694 |
| ns | 1557 |  | 153 | Details: source() from #[source] or a field named `source` | 2.5 |  | 0.668 |
| walker |  | 1652 | 164 | [features] in Cargo.toml |  |  | 0.690 |
| ns | 1709 |  | 152 | Details: #[error(transparent)] forwarding | 2.6 |  | 0.656 |
| walker |  | 1775 | 123 | manifest config in Cargo.toml |  |  | 0.656 |
| walker |  | 1828 | 53 | [dependencies] in impl/Cargo.toml |  |  | 0.656 |
| ns | 1839 |  | 130 | Details: provide() and automatic Backtrace field detection | 2.7 |  | 0.635 |
| walker |  | 1858 | 30 | README.md section #3 |  |  | 0.636 |
| walker |  | 1867 | 9 | rust body build/probe.rs:15 |  |  | 0.636 |
| walker |  | 1941 | 74 | rust names impl/src/valid.rs |  |  | 0.637 |
| ns | 1960 |  | 121 | Details: #[backtrace] on a source field forwards provide() | 2.8 |  | 0.626 |
| ns | 2005 |  | 45 | Details: #[from] variants with a Backtrace field capture in From | 2.9 |  | 0.621 |
| walker |  | 2026 | 85 | rust names impl/src/generics.rs |  |  | 0.621 |
| walker |  | 2042 | 16 | rust decl impl/src/generics.rs:8 |  |  | 0.622 |
| walker |  | 2081 | 39 | rust decl impl/src/generics.rs:48 |  |  | 0.622 |
| ns | 2088 |  | 83 | Details example: extra format arguments (`max = i32::MAX`) | 2.10 |  | 0.608 |
| walker |  | 2127 | 46 | rust decl impl/src/generics.rs:12 |  |  | 0.608 |
| walker |  | 2197 | 70 | rust decl impl/src/generics.rs:53 |  |  | 0.609 |
| walker |  | 2221 | 24 | rust decl impl/src/valid.rs:5 |  |  | 0.609 |
| ns | 2254 |  | 166 | Details example: referring to fields from format args via `.var` / `.0` | 2.11 |  | 0.586 |
| walker |  | 2276 | 55 | rust body build.rs:190 |  |  | 0.586 |
| walker |  | 2285 | 9 | rust body build/probe.rs:21 |  |  | 0.586 |
| walker |  | 2390 | 105 | rust names impl/src/ast.rs |  |  | 0.587 |
| ns | 2410 |  | 156 | When to use thiserror vs anyhow | 2.12 |  | 0.571 |
| walker |  | 2416 | 26 | rust decl impl/src/ast.rs:10 |  |  | 0.571 |
| walker |  | 2444 | 28 | rust decl impl/src/ast.rs:174 |  |  | 0.571 |
| walker |  | 2473 | 29 | rust decl impl/src/ast.rs:54 |  |  | 0.571 |
| ns | 2525 |  | 115 | impl/src/expand.rs: complete top-level function roster (names only) | 3.1 |  | 0.554 |
| walker |  | 2526 | 53 | rust decl impl/src/ast.rs:15 |  |  | 0.555 |
| walker |  | 2579 | 53 | rust decl impl/src/ast.rs:29 |  |  | 0.556 |
| walker |  | 2633 | 54 | rust decl impl/src/ast.rs:22 |  |  | 0.557 |
| walker |  | 2694 | 61 | rust decl impl/src/ast.rs:44 |  |  | 0.558 |
| ns | 2723 |  | 198 | expand.rs: derive/try_expand — the whole expansion pipeline in 18 lines | 3.2 | 3.1 | 0.532 |
| walker |  | 2759 | 65 | rust decl impl/src/ast.rs:36 |  |  | 0.534 |
| ns | 2950 |  | 227 | expand.rs: the two emitted impl shapes (struct and enum quote! tails) | 3.3 |  | 0.509 |
| walker |  | 3033 | 274 | rust module doc src/lib.rs |  |  | 0.513 |
| walker |  | 3268 | 235 | rust names impl/src/scan_expr.rs |  |  | 0.513 |
| ns | 3290 |  | 340 | impl/src/ast.rs: the complete IR (Input, Struct, Enum, Variant, Field) | 3.4 |  | 0.558 |
| walker |  | 3300 | 32 | rust decl impl/src/scan_expr.rs:84 |  |  | 0.558 |
| walker |  | 3338 | 38 | rust decl impl/src/scan_expr.rs:104 |  |  | 0.558 |
| walker |  | 3376 | 38 | rust decl impl/src/scan_expr.rs:114 |  |  | 0.558 |
| walker |  | 3417 | 41 | rust decl impl/src/scan_expr.rs:109 |  |  | 0.558 |
| walker |  | 3459 | 42 | rust decl impl/src/scan_expr.rs:25 |  |  | 0.558 |
| ns | 3528 |  | 238 | impl/src/attr.rs: Attrs and the parsed Display attribute | 3.5 |  | 0.536 |
| walker |  | 3713 | 254 | rust names impl/src/unraw.rs |  |  | 0.536 |
| walker |  | 3730 | 17 | rust decl impl/src/unraw.rs:10 |  |  | 0.536 |
| walker |  | 3749 | 19 | rust decl impl/src/unraw.rs:87 |  |  | 0.536 |
| walker |  | 3771 | 22 | rust decl impl/src/unraw.rs:69 |  |  | 0.536 |
| walker |  | 3794 | 23 | rust decl impl/src/unraw.rs:45 |  |  | 0.536 |
| walker |  | 3817 | 23 | rust decl impl/src/unraw.rs:51 |  |  | 0.536 |
| walker |  | 3840 | 23 | rust decl impl/src/unraw.rs:57 |  |  | 0.536 |
| ns | 3847 |  | 319 | attr.rs: Source/From/Transparent/Fmt payloads and the Trait enum | 3.6 |  | 0.503 |
| walker |  | 3863 | 23 | rust decl impl/src/unraw.rs:107 |  |  | 0.503 |
| walker |  | 3886 | 23 | rust decl impl/src/unraw.rs:117 |  |  | 0.503 |
| walker |  | 3910 | 24 | rust decl impl/src/unraw.rs:135 |  |  | 0.503 |
| walker |  | 3936 | 26 | rust decl impl/src/unraw.rs:75 |  |  | 0.503 |
| walker |  | 3963 | 27 | rust decl impl/src/unraw.rs:63 |  |  | 0.503 |
| walker |  | 3990 | 27 | rust decl impl/src/unraw.rs:126 |  |  | 0.503 |
| walker |  | 4018 | 28 | rust decl impl/src/unraw.rs:37 |  |  | 0.503 |
| ns | 4040 |  | 193 | attr.rs: the three accepted forms of #[error(...)] | 3.7 |  | 0.491 |
| walker |  | 4046 | 28 | rust decl impl/src/unraw.rs:96 |  |  | 0.491 |
| walker |  | 4075 | 29 | rust decl impl/src/unraw.rs:81 |  |  | 0.491 |
| walker |  | 4133 | 58 | rust decl impl/src/unraw.rs:14 |  |  | 0.491 |
| ns | 4186 |  | 146 | impl/src/valid.rs: container-level diagnostic messages | 3.8 |  | 0.486 |
| walker |  | 4371 | 238 | rust module doc src/lib.rs #1 |  |  | 0.490 |
| ns | 4478 |  | 292 | valid.rs: attribute-placement and field-attribute diagnostic messages | 3.9 |  | 0.480 |
| walker |  | 4493 | 122 | [package] in impl/Cargo.toml |  |  | 0.480 |
| ns | 4647 |  | 169 | valid.rs: complete validate/check function roster | 3.10 |  | 0.475 |
| walker |  | 4652 | 159 | manifest config in impl/Cargo.toml |  |  | 0.475 |
| walker |  | 4711 | 59 | README.md section #11 |  |  | 0.478 |
| walker |  | 4760 | 49 | rust decl impl/src/scan_expr.rs:89 |  |  | 0.478 |
| ns | 4859 |  | 212 | attr.rs: which attributes `get` recognises, plus the file's function roster | 3.11 |  | 0.467 |
| walker |  | 4958 | 198 | rust names impl/src/attr.rs |  |  | 0.472 |
| walker |  | 4982 | 24 | rust decl impl/src/attr.rs:302 |  |  | 0.472 |
| walker |  | 5006 | 24 | rust decl impl/src/attr.rs:342 |  |  | 0.472 |
| walker |  | 5042 | 36 | rust decl impl/src/attr.rs:50 |  |  | 0.476 |
| walker |  | 5078 | 36 | rust decl impl/src/attr.rs:44 |  |  | 0.482 |
| ns | 5096 |  | 237 | impl/src/prop.rs: complete accessor roster for source/from/backtrace fields | 3.12 |  | 0.468 |
| walker |  | 5114 | 36 | rust decl impl/src/attr.rs:38 |  |  | 0.476 |
| walker |  | 5150 | 36 | rust decl impl/src/attr.rs:32 |  |  | 0.486 |
| walker |  | 5240 | 90 | rust decl impl/src/attr.rs:11 |  |  | 0.495 |
| ns | 5305 |  | 209 | prop.rs: how the source and backtrace fields are actually chosen | 3.13 |  | 0.481 |
| walker |  | 5332 | 92 | rust decl impl/src/attr.rs:56 |  |  | 0.514 |
| walker |  | 5444 | 112 | rust decl impl/src/attr.rs:20 |  |  | 0.541 |
| walker |  | 5453 | 9 | listing of 'tests/no-std' |  |  | 0.541 |
| walker |  | 5505 | 52 | rust decl impl/src/scan_expr.rs:76 |  |  | 0.541 |
| ns | 5595 |  | 290 | impl/src/fmt.rs: expand_shorthand and the format-spec → Trait mapping | 3.14 |  | 0.527 |
| walker |  | 5693 | 188 | rust names impl/src/fmt.rs |  |  | 0.527 |
| walker |  | 5726 | 33 | rust decl impl/src/fmt.rs:15 |  |  | 0.529 |
| ns | 5726 |  | 131 | fmt.rs: FmtArguments and the remaining function roster | 3.15 | 3.14 | 0.529 |
| walker |  | 5740 | 14 | rust decl impl/src/fmt.rs:171 |  |  | 0.529 |
| walker |  | 5772 | 32 | rust decl impl/src/fmt.rs:166 |  |  | 0.539 |
| ns | 5905 |  | 179 | ast.rs: ContainerKind and its six display strings | 3.16 |  | 0.531 |
| walker |  | 5965 | 193 | rust names impl/src/prop.rs |  |  | 0.541 |
| walker |  | 6007 | 42 | rust decl impl/src/prop.rs:72 |  |  | 0.544 |
| walker |  | 6049 | 42 | rust decl impl/src/prop.rs:127 |  |  | 0.544 |
| ns | 6198 |  | 293 | ast.rs: complete from_syn constructor roster | 3.17 |  | 0.536 |
| ns | 6357 |  | 159 | impl/src/generics.rs: ParamsInScope and InferredBounds | 3.18 |  | 0.546 |
| walker |  | 6577 | 528 | listing of 'tests/ui' |  |  | 0.546 |
| ns | 6606 |  | 249 | impl/src/unraw.rs: IdentUnraw/MemberUnraw and the raw-identifier rule | 3.19 |  | 0.538 |
| walker |  | 6694 | 117 | rust body build.rs:179 |  |  | 0.538 |
| walker |  | 6766 | 72 | README.md section #2 |  |  | 0.544 |
| ns | 6910 |  | 304 | impl/src/fallback.rs: the invalid-input fallback expansion | 3.20 |  | 0.532 |
| walker |  | 6979 | 213 | rust names impl/src/expand.rs |  |  | 0.549 |
| walker |  | 7023 | 44 | rust decl impl/src/expand.rs:536 |  |  | 0.549 |
| walker |  | 7084 | 61 | rust decl impl/src/prop.rs:25 |  |  | 0.553 |
| ns | 7121 |  | 211 | impl/src/scan_expr.rs: the Input/Action alphabet of the expression scanner | 3.21 |  | 0.542 |
| walker |  | 7357 | 273 | rust module doc src/lib.rs #2 |  |  | 0.542 |
| walker |  | 7371 | 14 | rust body build/probe.rs:27 |  |  | 0.542 |
| walker |  | 7387 | 16 | rust names tests/test_lints.rs |  |  | 0.542 |
| walker |  | 7402 | 15 | rust body impl/src/expand.rs:565 |  |  | 0.542 |
| walker |  | 7449 | 47 | rust body impl/src/lib.rs:49 |  |  | 0.543 |
| ns | 7477 |  | 356 | expand.rs: the gate line for every conditionally generated impl | 3.22 | 3.1 | 0.532 |
| walker |  | 7543 | 94 | rust decl impl/src/prop.rs:6 |  |  | 0.538 |
| walker |  | 7637 | 94 | rust decl impl/src/prop.rs:53 |  |  | 0.539 |
| ns | 7637 |  | 160 | src/private.rs: the complete generated-code support surface | 4.1 |  | 0.539 |
| ns | 7852 |  | 215 | src/aserror.rs: AsDynError and its five blanket/dyn impls | 4.2 |  | 0.533 |
| walker |  | 7920 | 283 | rust names impl/src/scan_expr.rs #1 |  |  | 0.533 |
| walker |  | 7941 | 21 | rust decl impl/src/scan_expr.rs:136 |  |  | 0.533 |
| walker |  | 7973 | 32 | rust decl impl/src/scan_expr.rs:119 |  |  | 0.533 |
| walker |  | 8005 | 32 | rust decl impl/src/scan_expr.rs:187 |  |  | 0.533 |
| walker |  | 8039 | 34 | rust decl impl/src/scan_expr.rs:130 |  |  | 0.533 |
| walker |  | 8085 | 46 | rust decl impl/src/scan_expr.rs:181 |  |  | 0.533 |
| ns | 8094 |  | 242 | src/display.rs: AsDisplay and the std-only Path/PathBuf specializations | 4.3 |  | 0.524 |
| walker |  | 8132 | 47 | rust decl impl/src/scan_expr.rs:175 |  |  | 0.524 |
| walker |  | 8180 | 48 | rust decl impl/src/scan_expr.rs:124 |  |  | 0.524 |
| walker |  | 8246 | 66 | rust decl impl/src/scan_expr.rs:141 |  |  | 0.524 |
| walker |  | 8338 | 92 | rust decl impl/src/scan_expr.rs:166 |  |  | 0.524 |
| ns | 8376 |  | 282 | src/provide.rs (ThiserrorProvide) and src/var.rs (Var) end to end | 4.4 |  | 0.515 |
| ns | 8489 |  | 113 | impl/src/lib.rs: the version-stamped `private` path token | 4.5 |  | 0.521 |
| walker |  | 8643 | 305 | README.md section #12 |  |  | 0.533 |
| ns | 8738 |  | 249 | build.rs: the generated __private module and the cfg declarations | 4.6 |  | 0.527 |
| walker |  | 8748 | 105 | rust decl impl/src/scan_expr.rs:95 |  |  | 0.527 |
| ns | 8946 |  | 208 | build.rs: rustc capability decisions, function roster, and the probe file | 4.7 |  | 0.523 |
| walker |  | 8981 | 233 | rust module doc src/lib.rs #3 |  |  | 0.523 |
| walker |  | 9005 | 24 | rust names tests/test_backtrace.rs |  |  | 0.523 |
| walker |  | 9024 | 19 | rust decl tests/test_backtrace.rs:6 |  |  | 0.523 |
| ns | 9043 |  | 97 | Complete tests/, tests/no-std/ and .github/ listings | 5.1 |  | 0.534 |
| walker |  | 9048 | 24 | rust names tests/test_display.rs |  |  | 0.534 |
| ns | 9131 |  | 88 | tests/compiletest.rs: the trybuild UI harness in full | 5.2 |  | 0.531 |
| walker |  | 9182 | 134 | README.md section #9 |  |  | 0.534 |
| ns | 9262 |  | 131 | tests/no-std/test.rs: the no_std smoke test's error types | 5.3 |  | 0.528 |
| walker |  | 9324 | 142 | rust decl impl/src/scan_expr.rs:6 |  |  | 0.550 |
| walker |  | 9356 | 32 | rust body impl/src/generics.rs:19 |  |  | 0.550 |
| ns | 9389 |  | 127 | Cargo.toml: std-feature rationale and dev-dependencies | 5.4 |  | 0.549 |
| ns | 9515 |  | 126 | impl/Cargo.toml: the proc-macro crate's manifest | 5.5 |  | 0.553 |
| walker |  | 9521 | 165 | rust decl impl/src/scan_expr.rs:63 |  |  | 0.553 |
| ns | 9689 |  | 174 | CI job roster and the pinned toolchain components | 5.6 |  | 0.546 |
| walker |  | 9767 | 246 | rust module doc src/lib.rs #4 |  |  | 0.546 |
| walker |  | 9864 | 97 | rust body impl/src/expand.rs:12 |  |  | 0.553 |
