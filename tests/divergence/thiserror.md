Score(3000)=0.492 I=0.824 C=0.294 ns_rows≤3K=24/56 grid(1000/1442/2080/3000/4327/6240/9000)=0.649/0.692/0.602/0.492/0.553/0.562/0.535

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 38 | 38 | listing of '.' |  |  | 0.000 |
| walker |  | 42 | 4 | listing of 'build' |  |  | 0.000 |
| walker |  | 61 | 19 | listing of 'impl' |  |  | 0.000 |
| walker |  | 86 | 25 | listing of 'src' |  |  | 0.000 |
| ns | 86 |  | 86 | Crate identity: name, one-line purpose, version | 1.1 |  | 0.000 |
| walker |  | 100 | 14 | rust names src/lib.rs |  |  | 0.000 |
| ns | 124 |  | 38 | Complete repository root listing | 1.2 |  | 0.453 |
| walker |  | 147 | 47 | README headline in README.md |  |  | 0.636 |
| walker |  | 155 | 8 | listing of '.github' |  |  | 0.636 |
| walker |  | 159 | 4 | listing of '.github/workflows' |  |  | 0.636 |
| walker |  | 195 | 36 | [dependencies] in Cargo.toml |  |  | 0.637 |
| ns | 215 |  | 91 | Complete listings of both source trees: src/, impl/, impl/src/ | 1.3 |  | 0.460 |
| walker |  | 224 | 29 | headings outline in README.md |  |  | 0.462 |
| walker |  | 271 | 47 | listing of 'impl/src' |  |  | 0.708 |
| ns | 313 |  | 98 | README canonical example, head: derive, #[from], positional {0} | 1.4 |  | 0.617 |
| walker |  | 316 | 45 | rust names impl/src/lib.rs |  |  | 0.618 |
| walker |  | 337 | 21 | rust decl impl/src/lib.rs:39 |  |  | 0.620 |
| walker |  | 349 | 12 | rust decl impl/src/lib.rs:45 |  |  | 0.620 |
| walker |  | 377 | 28 | rust decl impl/src/lib.rs:48 |  |  | 0.620 |
| ns | 398 |  | 85 | README canonical example, tail: named-field variant, unit variant | 1.5 | 1.4 | 0.561 |
| walker |  | 460 | 83 | rust names build.rs |  |  | 0.561 |
| ns | 476 |  | 78 | Derive entry point: #[proc_macro_derive(Error, attributes(...))] | 1.6 |  | 0.543 |
| ns | 612 |  | 136 | Root crate module structure and public re-export (src/lib.rs tail) | 1.7 |  | 0.475 |
| walker |  | 645 | 185 | [package] in Cargo.toml |  |  | 0.635 |
| walker |  | 676 | 31 | rust body impl/src/lib.rs:39 |  |  | 0.661 |
| ns | 707 |  | 95 | Root crate attributes: no_std, docs.rs root, nightly cfg gate | 1.8 |  | 0.628 |
| walker |  | 748 | 72 | README.md section #0 |  |  | 0.666 |
| walker |  | 801 | 53 | rust names src/var.rs |  |  | 0.666 |
| ns | 814 |  | 107 | Cargo manifest: features, dependency on impl, workspace members | 1.9 |  | 0.662 |
| walker |  | 829 | 28 | rust decl src/var.rs:5 |  |  | 0.662 |
| walker |  | 859 | 30 | rust names impl/src/fallback.rs |  |  | 0.662 |
| ns | 870 |  | 56 | Documentation map: README section headings + rustdoc mirror in src/lib.rs | 2.1 |  | 0.649 |
| walker |  | 905 | 46 | rust decl build.rs:10 |  |  | 0.649 |
| walker |  | 986 | 81 | rust names build/probe.rs |  |  | 0.649 |
| ns | 1036 |  | 166 | Details: no public API footprint, allowed error shapes, Display generation | 2.2 |  | 0.611 |
| walker |  | 1182 | 196 | README.md section #1 |  |  | 0.747 |
| ns | 1222 |  | 186 | Display shorthand table: {var}, {0}, {var:?}, {0:?} | 2.3 |  | 0.713 |
| walker |  | 1346 | 164 | [features] in Cargo.toml |  |  | 0.738 |
| walker |  | 1375 | 29 | rust decl build/probe.rs:14 |  |  | 0.738 |
| walker |  | 1404 | 29 | rust decl build/probe.rs:20 |  |  | 0.692 |
| ns | 1404 |  | 182 | Details: #[from] generates From, with its field-count restriction | 2.4 |  | 0.692 |
| walker |  | 1434 | 30 | rust decl build/probe.rs:26 |  |  | 0.692 |
| walker |  | 1506 | 72 | listing of 'tests' |  |  | 0.693 |
| ns | 1557 |  | 153 | Details: source() from #[source] or a field named `source` | 2.5 |  | 0.668 |
| walker |  | 1580 | 74 | rust names impl/src/valid.rs |  |  | 0.668 |
| walker |  | 1665 | 85 | rust names impl/src/generics.rs |  |  | 0.668 |
| walker |  | 1681 | 16 | rust decl impl/src/generics.rs:8 |  |  | 0.668 |
| ns | 1709 |  | 152 | Details: #[error(transparent)] forwarding | 2.6 |  | 0.635 |
| walker |  | 1720 | 39 | rust decl impl/src/generics.rs:48 |  |  | 0.636 |
| walker |  | 1766 | 46 | rust decl impl/src/generics.rs:12 |  |  | 0.636 |
| walker |  | 1836 | 70 | rust decl impl/src/generics.rs:53 |  |  | 0.637 |
| ns | 1839 |  | 130 | Details: provide() and automatic Backtrace field detection | 2.7 |  | 0.616 |
| walker |  | 1860 | 24 | rust decl impl/src/valid.rs:5 |  |  | 0.616 |
| walker |  | 1874 | 14 | rust body src/var.rs:6 |  |  | 0.616 |
| ns | 1960 |  | 121 | Details: #[backtrace] on a source field forwards provide() | 2.8 |  | 0.606 |
| walker |  | 1997 | 123 | manifest config in Cargo.toml |  |  | 0.606 |
| ns | 2005 |  | 45 | Details: #[from] variants with a Backtrace field capture in From | 2.9 |  | 0.602 |
| ns | 2088 |  | 83 | Details example: extra format arguments (`max = i32::MAX`) | 2.10 |  | 0.588 |
| walker |  | 2102 | 105 | rust names impl/src/ast.rs |  |  | 0.588 |
| walker |  | 2128 | 26 | rust decl impl/src/ast.rs:10 |  |  | 0.588 |
| walker |  | 2156 | 28 | rust decl impl/src/ast.rs:174 |  |  | 0.588 |
| walker |  | 2185 | 29 | rust decl impl/src/ast.rs:54 |  |  | 0.589 |
| walker |  | 2238 | 53 | rust decl impl/src/ast.rs:15 |  |  | 0.589 |
| ns | 2254 |  | 166 | Details example: referring to fields from format args via `.var` / `.0` | 2.11 |  | 0.568 |
| walker |  | 2291 | 53 | rust decl impl/src/ast.rs:29 |  |  | 0.569 |
| walker |  | 2345 | 54 | rust decl impl/src/ast.rs:22 |  |  | 0.570 |
| walker |  | 2406 | 61 | rust decl impl/src/ast.rs:44 |  |  | 0.571 |
| ns | 2410 |  | 156 | When to use thiserror vs anyhow | 2.12 |  | 0.556 |
| walker |  | 2471 | 65 | rust decl impl/src/ast.rs:36 |  |  | 0.558 |
| walker |  | 2524 | 53 | [dependencies] in impl/Cargo.toml |  |  | 0.558 |
| ns | 2525 |  | 115 | impl/src/expand.rs: complete top-level function roster (names only) | 3.1 |  | 0.541 |
| ns | 2723 |  | 198 | expand.rs: derive/try_expand — the whole expansion pipeline in 18 lines | 3.2 | 3.1 | 0.516 |
| walker |  | 2759 | 235 | rust names impl/src/scan_expr.rs |  |  | 0.516 |
| walker |  | 2791 | 32 | rust decl impl/src/scan_expr.rs:84 |  |  | 0.516 |
| walker |  | 2829 | 38 | rust decl impl/src/scan_expr.rs:104 |  |  | 0.516 |
| walker |  | 2867 | 38 | rust decl impl/src/scan_expr.rs:114 |  |  | 0.516 |
| walker |  | 2908 | 41 | rust decl impl/src/scan_expr.rs:109 |  |  | 0.516 |
| walker |  | 2950 | 42 | rust decl impl/src/scan_expr.rs:25 |  |  | 0.492 |
| ns | 2950 |  | 227 | expand.rs: the two emitted impl shapes (struct and enum quote! tails) | 3.3 |  | 0.492 |
| walker |  | 3204 | 254 | rust names impl/src/unraw.rs |  |  | 0.492 |
| walker |  | 3221 | 17 | rust decl impl/src/unraw.rs:10 |  |  | 0.492 |
| walker |  | 3240 | 19 | rust decl impl/src/unraw.rs:87 |  |  | 0.492 |
| walker |  | 3262 | 22 | rust decl impl/src/unraw.rs:69 |  |  | 0.492 |
| walker |  | 3285 | 23 | rust decl impl/src/unraw.rs:45 |  |  | 0.492 |
| ns | 3290 |  | 340 | impl/src/ast.rs: the complete IR (Input, Struct, Enum, Variant, Field) | 3.4 |  | 0.540 |
| walker |  | 3308 | 23 | rust decl impl/src/unraw.rs:51 |  |  | 0.540 |
| walker |  | 3331 | 23 | rust decl impl/src/unraw.rs:57 |  |  | 0.540 |
| walker |  | 3354 | 23 | rust decl impl/src/unraw.rs:107 |  |  | 0.540 |
| walker |  | 3377 | 23 | rust decl impl/src/unraw.rs:117 |  |  | 0.540 |
| walker |  | 3401 | 24 | rust decl impl/src/unraw.rs:135 |  |  | 0.540 |
| walker |  | 3427 | 26 | rust decl impl/src/unraw.rs:75 |  |  | 0.540 |
| walker |  | 3454 | 27 | rust decl impl/src/unraw.rs:63 |  |  | 0.540 |
| walker |  | 3481 | 27 | rust decl impl/src/unraw.rs:126 |  |  | 0.540 |
| walker |  | 3509 | 28 | rust decl impl/src/unraw.rs:37 |  |  | 0.540 |
| ns | 3528 |  | 238 | impl/src/attr.rs: Attrs and the parsed Display attribute | 3.5 |  | 0.519 |
| walker |  | 3537 | 28 | rust decl impl/src/unraw.rs:96 |  |  | 0.519 |
| walker |  | 3566 | 29 | rust decl impl/src/unraw.rs:81 |  |  | 0.519 |
| walker |  | 3624 | 58 | rust decl impl/src/unraw.rs:14 |  |  | 0.519 |
| walker |  | 3673 | 49 | rust decl impl/src/scan_expr.rs:89 |  |  | 0.519 |
| ns | 3847 |  | 319 | attr.rs: Source/From/Transparent/Fmt payloads and the Trait enum | 3.6 |  | 0.487 |
| walker |  | 3871 | 198 | rust names impl/src/attr.rs |  |  | 0.489 |
| walker |  | 3895 | 24 | rust decl impl/src/attr.rs:302 |  |  | 0.489 |
| walker |  | 3919 | 24 | rust decl impl/src/attr.rs:342 |  |  | 0.489 |
| walker |  | 3955 | 36 | rust decl impl/src/attr.rs:50 |  |  | 0.494 |
| walker |  | 3991 | 36 | rust decl impl/src/attr.rs:44 |  |  | 0.501 |
| walker |  | 4027 | 36 | rust decl impl/src/attr.rs:38 |  |  | 0.511 |
| ns | 4040 |  | 193 | attr.rs: the three accepted forms of #[error(...)] | 3.7 |  | 0.499 |
| walker |  | 4063 | 36 | rust decl impl/src/attr.rs:32 |  |  | 0.511 |
| walker |  | 4153 | 90 | rust decl impl/src/attr.rs:11 |  |  | 0.521 |
| ns | 4186 |  | 146 | impl/src/valid.rs: container-level diagnostic messages | 3.8 |  | 0.515 |
| walker |  | 4245 | 92 | rust decl impl/src/attr.rs:56 |  |  | 0.553 |
| walker |  | 4357 | 112 | rust decl impl/src/attr.rs:20 |  |  | 0.584 |
| walker |  | 4409 | 52 | rust decl impl/src/scan_expr.rs:76 |  |  | 0.584 |
| walker |  | 4439 | 30 | README.md section #3 |  |  | 0.585 |
| ns | 4478 |  | 292 | valid.rs: attribute-placement and field-attribute diagnostic messages | 3.9 |  | 0.573 |
| walker |  | 4627 | 188 | rust names impl/src/fmt.rs |  |  | 0.574 |
| ns | 4647 |  | 169 | valid.rs: complete validate/check function roster | 3.10 |  | 0.566 |
| walker |  | 4660 | 33 | rust decl impl/src/fmt.rs:15 |  |  | 0.566 |
| walker |  | 4674 | 14 | rust decl impl/src/fmt.rs:171 |  |  | 0.566 |
| walker |  | 4706 | 32 | rust decl impl/src/fmt.rs:166 |  |  | 0.567 |
| ns | 4859 |  | 212 | attr.rs: which attributes `get` recognises, plus the file's function roster | 3.11 |  | 0.555 |
| walker |  | 4899 | 193 | rust names impl/src/prop.rs |  |  | 0.556 |
| walker |  | 4941 | 42 | rust decl impl/src/prop.rs:72 |  |  | 0.556 |
| walker |  | 4983 | 42 | rust decl impl/src/prop.rs:127 |  |  | 0.556 |
| walker |  | 4992 | 9 | rust body build/probe.rs:15 |  |  | 0.556 |
| ns | 5096 |  | 237 | impl/src/prop.rs: complete accessor roster for source/from/backtrace fields | 3.12 |  | 0.553 |
| walker |  | 5205 | 213 | rust names impl/src/expand.rs |  |  | 0.575 |
| walker |  | 5249 | 44 | rust decl impl/src/expand.rs:536 |  |  | 0.575 |
| ns | 5305 |  | 209 | prop.rs: how the source and backtrace fields are actually chosen | 3.13 |  | 0.559 |
| walker |  | 5310 | 61 | rust decl impl/src/prop.rs:25 |  |  | 0.565 |
| walker |  | 5365 | 55 | rust body build.rs:190 |  |  | 0.565 |
| walker |  | 5374 | 9 | rust body build/probe.rs:21 |  |  | 0.565 |
| ns | 5595 |  | 290 | impl/src/fmt.rs: expand_shorthand and the format-spec → Trait mapping | 3.14 |  | 0.550 |
| walker |  | 5648 | 274 | rust module doc src/lib.rs |  |  | 0.553 |
| walker |  | 5663 | 15 | rust body impl/src/expand.rs:565 |  |  | 0.553 |
| walker |  | 5710 | 47 | rust body impl/src/lib.rs:49 |  |  | 0.553 |
| ns | 5726 |  | 131 | fmt.rs: FmtArguments and the remaining function roster | 3.15 | 3.14 | 0.562 |
| walker |  | 5804 | 94 | rust decl impl/src/prop.rs:6 |  |  | 0.570 |
| walker |  | 5898 | 94 | rust decl impl/src/prop.rs:53 |  |  | 0.579 |
| ns | 5905 |  | 179 | ast.rs: ContainerKind and its six display strings | 3.16 |  | 0.571 |
| walker |  | 6181 | 283 | rust names impl/src/scan_expr.rs #1 |  |  | 0.571 |
| ns | 6198 |  | 293 | ast.rs: complete from_syn constructor roster | 3.17 |  | 0.562 |
| walker |  | 6202 | 21 | rust decl impl/src/scan_expr.rs:136 |  |  | 0.562 |
| walker |  | 6234 | 32 | rust decl impl/src/scan_expr.rs:119 |  |  | 0.562 |
| walker |  | 6266 | 32 | rust decl impl/src/scan_expr.rs:187 |  |  | 0.562 |
| walker |  | 6300 | 34 | rust decl impl/src/scan_expr.rs:130 |  |  | 0.562 |
| walker |  | 6346 | 46 | rust decl impl/src/scan_expr.rs:181 |  |  | 0.562 |
| ns | 6357 |  | 159 | impl/src/generics.rs: ParamsInScope and InferredBounds | 3.18 |  | 0.571 |
| walker |  | 6393 | 47 | rust decl impl/src/scan_expr.rs:175 |  |  | 0.571 |
| walker |  | 6441 | 48 | rust decl impl/src/scan_expr.rs:124 |  |  | 0.571 |
| walker |  | 6507 | 66 | rust decl impl/src/scan_expr.rs:141 |  |  | 0.571 |
| walker |  | 6599 | 92 | rust decl impl/src/scan_expr.rs:166 |  |  | 0.571 |
| ns | 6606 |  | 249 | impl/src/unraw.rs: IdentUnraw/MemberUnraw and the raw-identifier rule | 3.19 |  | 0.562 |
| walker |  | 6704 | 105 | rust decl impl/src/scan_expr.rs:95 |  |  | 0.562 |
| ns | 6910 |  | 304 | impl/src/fallback.rs: the invalid-input fallback expansion | 3.20 |  | 0.549 |
| walker |  | 6942 | 238 | rust module doc src/lib.rs #1 |  |  | 0.552 |
| walker |  | 7064 | 122 | [package] in impl/Cargo.toml |  |  | 0.552 |
| ns | 7121 |  | 211 | impl/src/scan_expr.rs: the Input/Action alphabet of the expression scanner | 3.21 |  | 0.541 |
| walker |  | 7223 | 159 | manifest config in impl/Cargo.toml |  |  | 0.541 |
| walker |  | 7282 | 59 | README.md section #11 |  |  | 0.543 |
| walker |  | 7424 | 142 | rust decl impl/src/scan_expr.rs:6 |  |  | 0.571 |
| walker |  | 7433 | 9 | listing of 'tests/no-std' |  |  | 0.571 |
| ns | 7477 |  | 356 | expand.rs: the gate line for every conditionally generated impl | 3.22 | 3.1 | 0.560 |
| ns | 7637 |  | 160 | src/private.rs: the complete generated-code support surface | 4.1 |  | 0.553 |
| ns | 7852 |  | 215 | src/aserror.rs: AsDynError and its five blanket/dyn impls | 4.2 |  | 0.546 |
| walker |  | 7961 | 528 | listing of 'tests/ui' |  |  | 0.546 |
| walker |  | 8078 | 117 | rust body build.rs:179 |  |  | 0.546 |
| ns | 8094 |  | 242 | src/display.rs: AsDisplay and the std-only Path/PathBuf specializations | 4.3 |  | 0.537 |
| walker |  | 8150 | 72 | README.md section #2 |  |  | 0.543 |
| walker |  | 8182 | 32 | rust body impl/src/generics.rs:19 |  |  | 0.543 |
| walker |  | 8347 | 165 | rust decl impl/src/scan_expr.rs:63 |  |  | 0.543 |
| ns | 8376 |  | 282 | src/provide.rs (ThiserrorProvide) and src/var.rs (Var) end to end | 4.4 |  | 0.533 |
| ns | 8489 |  | 113 | impl/src/lib.rs: the version-stamped `private` path token | 4.5 |  | 0.539 |
| walker |  | 8620 | 273 | rust module doc src/lib.rs #2 |  |  | 0.539 |
| walker |  | 8717 | 97 | rust body impl/src/expand.rs:12 |  |  | 0.546 |
| walker |  | 8731 | 14 | rust body build/probe.rs:27 |  |  | 0.546 |
| ns | 8738 |  | 249 | build.rs: the generated __private module and the cfg declarations | 4.6 |  | 0.540 |
| walker |  | 8763 | 32 | rust body impl/src/expand.rs:569 |  |  | 0.540 |
| walker |  | 8779 | 16 | rust names tests/test_lints.rs |  |  | 0.540 |
| ns | 8946 |  | 208 | build.rs: rustc capability decisions, function roster, and the probe file | 4.7 |  | 0.535 |
| walker |  | 9018 | 239 | rust decl impl/src/scan_expr.rs:148 |  |  | 0.535 |
| ns | 9043 |  | 97 | Complete tests/, tests/no-std/ and .github/ listings | 5.1 |  | 0.546 |
| walker |  | 9055 | 37 | rust body impl/src/generics.rs:13 |  |  | 0.546 |
| walker |  | 9066 | 11 | rust body impl/src/prop.rs:7 |  |  | 0.546 |
| ns | 9131 |  | 88 | tests/compiletest.rs: the trybuild UI harness in full | 5.2 |  | 0.543 |
| ns | 9262 |  | 131 | tests/no-std/test.rs: the no_std smoke test's error types | 5.3 |  | 0.537 |
| walker |  | 9371 | 305 | README.md section #12 |  |  | 0.548 |
| ns | 9389 |  | 127 | Cargo.toml: std-feature rationale and dev-dependencies | 5.4 |  | 0.547 |
| ns | 9515 |  | 126 | impl/Cargo.toml: the proc-macro crate's manifest | 5.5 |  | 0.551 |
| walker |  | 9604 | 233 | rust module doc src/lib.rs #3 |  |  | 0.551 |
| walker |  | 9628 | 24 | rust names tests/test_backtrace.rs |  |  | 0.551 |
| walker |  | 9647 | 19 | rust decl tests/test_backtrace.rs:6 |  |  | 0.551 |
| walker |  | 9671 | 24 | rust names tests/test_display.rs |  |  | 0.551 |
| ns | 9689 |  | 174 | CI job roster and the pinned toolchain components | 5.6 |  | 0.544 |
| walker |  | 9710 | 39 | rust body impl/src/expand.rs:505 |  |  | 0.544 |
| walker |  | 9844 | 134 | README.md section #9 |  |  | 0.547 |
| walker |  | 9881 | 37 | rust body impl/src/generics.rs:54 |  |  | 0.547 |
| walker |  | 9892 | 11 | rust body impl/src/prop.rs:11 |  |  | 0.547 |
