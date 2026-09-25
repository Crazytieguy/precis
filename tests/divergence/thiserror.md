Score(3000)=0.508 I=0.832 C=0.310 ns_rows≤3K=24/56 grid(1000/1442/2080/3000/4327/6240/9000)=0.678/0.692/0.621/0.508/0.526/0.571/0.541

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
| walker |  | 448 | 120 | rust names impl/src/lib.rs |  |  | 0.565 |
| walker |  | 469 | 21 | rust decl impl/src/lib.rs:39 |  |  | 0.567 |
| ns | 476 |  | 78 | Derive entry point: #[proc_macro_derive(Error, attributes(...))] | 1.6 |  | 0.548 |
| walker |  | 481 | 12 | rust decl impl/src/lib.rs:45 |  |  | 0.548 |
| walker |  | 509 | 28 | rust decl impl/src/lib.rs:48 |  |  | 0.549 |
| walker |  | 592 | 83 | rust names build.rs |  |  | 0.549 |
| ns | 612 |  | 136 | Root crate module structure and public re-export (src/lib.rs tail) | 1.7 |  | 0.506 |
| ns | 707 |  | 95 | Root crate attributes: no_std, docs.rs root, nightly cfg gate | 1.8 |  | 0.481 |
| walker |  | 777 | 185 | [package] in Cargo.toml |  |  | 0.638 |
| walker |  | 808 | 31 | rust body impl/src/lib.rs:39 |  |  | 0.662 |
| ns | 814 |  | 107 | Cargo manifest: features, dependency on impl, workspace members | 1.9 |  | 0.656 |
| ns | 870 |  | 56 | Documentation map: README section headings + rustdoc mirror in src/lib.rs | 2.1 |  | 0.642 |
| walker |  | 880 | 72 | README.md section #0 |  |  | 0.678 |
| walker |  | 933 | 53 | rust names src/var.rs |  |  | 0.678 |
| walker |  | 961 | 28 | rust decl src/var.rs:5 |  |  | 0.678 |
| walker |  | 991 | 30 | rust names impl/src/fallback.rs |  |  | 0.678 |
| ns | 1036 |  | 166 | Details: no public API footprint, allowed error shapes, Display generation | 2.2 |  | 0.638 |
| walker |  | 1037 | 46 | rust decl build.rs:10 |  |  | 0.638 |
| walker |  | 1118 | 81 | rust names build/probe.rs |  |  | 0.638 |
| ns | 1222 |  | 186 | Display shorthand table: {var}, {0}, {var:?}, {0:?} | 2.3 |  | 0.610 |
| walker |  | 1314 | 196 | README.md section #1 |  |  | 0.738 |
| ns | 1404 |  | 182 | Details: #[from] generates From, with its field-count restriction | 2.4 |  | 0.692 |
| walker |  | 1478 | 164 | [features] in Cargo.toml |  |  | 0.715 |
| walker |  | 1507 | 29 | rust decl build/probe.rs:14 |  |  | 0.715 |
| walker |  | 1536 | 29 | rust decl build/probe.rs:20 |  |  | 0.715 |
| ns | 1557 |  | 153 | Details: source() from #[source] or a field named `source` | 2.5 |  | 0.688 |
| walker |  | 1566 | 30 | rust decl build/probe.rs:26 |  |  | 0.688 |
| walker |  | 1638 | 72 | listing of 'tests' |  |  | 0.690 |
| ns | 1709 |  | 152 | Details: #[error(transparent)] forwarding | 2.6 |  | 0.656 |
| walker |  | 1712 | 74 | rust names impl/src/valid.rs |  |  | 0.656 |
| walker |  | 1797 | 85 | rust names impl/src/generics.rs |  |  | 0.656 |
| walker |  | 1813 | 16 | rust decl impl/src/generics.rs:8 |  |  | 0.656 |
| ns | 1839 |  | 130 | Details: provide() and automatic Backtrace field detection | 2.7 |  | 0.635 |
| walker |  | 1852 | 39 | rust decl impl/src/generics.rs:48 |  |  | 0.635 |
| walker |  | 1898 | 46 | rust decl impl/src/generics.rs:12 |  |  | 0.636 |
| ns | 1960 |  | 121 | Details: #[backtrace] on a source field forwards provide() | 2.8 |  | 0.625 |
| walker |  | 1968 | 70 | rust decl impl/src/generics.rs:53 |  |  | 0.625 |
| walker |  | 1992 | 24 | rust decl impl/src/valid.rs:5 |  |  | 0.626 |
| ns | 2005 |  | 45 | Details: #[from] variants with a Backtrace field capture in From | 2.9 |  | 0.621 |
| walker |  | 2006 | 14 | rust body src/var.rs:6 |  |  | 0.621 |
| ns | 2088 |  | 83 | Details example: extra format arguments (`max = i32::MAX`) | 2.10 |  | 0.607 |
| walker |  | 2129 | 123 | manifest config in Cargo.toml |  |  | 0.607 |
| walker |  | 2234 | 105 | rust names impl/src/ast.rs |  |  | 0.607 |
| ns | 2254 |  | 166 | Details example: referring to fields from format args via `.var` / `.0` | 2.11 |  | 0.585 |
| walker |  | 2260 | 26 | rust decl impl/src/ast.rs:10 |  |  | 0.585 |
| walker |  | 2288 | 28 | rust decl impl/src/ast.rs:174 |  |  | 0.585 |
| walker |  | 2317 | 29 | rust decl impl/src/ast.rs:54 |  |  | 0.585 |
| walker |  | 2370 | 53 | rust decl impl/src/ast.rs:15 |  |  | 0.586 |
| ns | 2410 |  | 156 | When to use thiserror vs anyhow | 2.12 |  | 0.571 |
| walker |  | 2423 | 53 | rust decl impl/src/ast.rs:29 |  |  | 0.572 |
| walker |  | 2477 | 54 | rust decl impl/src/ast.rs:22 |  |  | 0.573 |
| ns | 2525 |  | 115 | impl/src/expand.rs: complete top-level function roster (names only) | 3.1 |  | 0.556 |
| walker |  | 2538 | 61 | rust decl impl/src/ast.rs:44 |  |  | 0.556 |
| walker |  | 2603 | 65 | rust decl impl/src/ast.rs:36 |  |  | 0.558 |
| walker |  | 2656 | 53 | [dependencies] in impl/Cargo.toml |  |  | 0.558 |
| ns | 2723 |  | 198 | expand.rs: derive/try_expand — the whole expansion pipeline in 18 lines | 3.2 | 3.1 | 0.533 |
| walker |  | 2891 | 235 | rust names impl/src/scan_expr.rs |  |  | 0.533 |
| walker |  | 2923 | 32 | rust decl impl/src/scan_expr.rs:84 |  |  | 0.533 |
| ns | 2950 |  | 227 | expand.rs: the two emitted impl shapes (struct and enum quote! tails) | 3.3 |  | 0.508 |
| walker |  | 2961 | 38 | rust decl impl/src/scan_expr.rs:104 |  |  | 0.508 |
| walker |  | 2999 | 38 | rust decl impl/src/scan_expr.rs:114 |  |  | 0.508 |
| walker |  | 3040 | 41 | rust decl impl/src/scan_expr.rs:109 |  |  | 0.508 |
| walker |  | 3082 | 42 | rust decl impl/src/scan_expr.rs:25 |  |  | 0.508 |
| ns | 3290 |  | 340 | impl/src/ast.rs: the complete IR (Input, Struct, Enum, Variant, Field) | 3.4 |  | 0.553 |
| walker |  | 3336 | 254 | rust names impl/src/unraw.rs |  |  | 0.553 |
| walker |  | 3353 | 17 | rust decl impl/src/unraw.rs:10 |  |  | 0.553 |
| walker |  | 3372 | 19 | rust decl impl/src/unraw.rs:87 |  |  | 0.553 |
| walker |  | 3394 | 22 | rust decl impl/src/unraw.rs:69 |  |  | 0.553 |
| walker |  | 3417 | 23 | rust decl impl/src/unraw.rs:45 |  |  | 0.553 |
| walker |  | 3440 | 23 | rust decl impl/src/unraw.rs:51 |  |  | 0.553 |
| walker |  | 3463 | 23 | rust decl impl/src/unraw.rs:57 |  |  | 0.553 |
| walker |  | 3486 | 23 | rust decl impl/src/unraw.rs:107 |  |  | 0.553 |
| walker |  | 3509 | 23 | rust decl impl/src/unraw.rs:117 |  |  | 0.553 |
| ns | 3528 |  | 238 | impl/src/attr.rs: Attrs and the parsed Display attribute | 3.5 |  | 0.532 |
| walker |  | 3533 | 24 | rust decl impl/src/unraw.rs:135 |  |  | 0.532 |
| walker |  | 3559 | 26 | rust decl impl/src/unraw.rs:75 |  |  | 0.532 |
| walker |  | 3586 | 27 | rust decl impl/src/unraw.rs:63 |  |  | 0.532 |
| walker |  | 3613 | 27 | rust decl impl/src/unraw.rs:126 |  |  | 0.532 |
| walker |  | 3641 | 28 | rust decl impl/src/unraw.rs:37 |  |  | 0.532 |
| walker |  | 3669 | 28 | rust decl impl/src/unraw.rs:96 |  |  | 0.532 |
| walker |  | 3698 | 29 | rust decl impl/src/unraw.rs:81 |  |  | 0.532 |
| walker |  | 3756 | 58 | rust decl impl/src/unraw.rs:14 |  |  | 0.532 |
| walker |  | 3805 | 49 | rust decl impl/src/scan_expr.rs:89 |  |  | 0.532 |
| ns | 3847 |  | 319 | attr.rs: Source/From/Transparent/Fmt payloads and the Trait enum | 3.6 |  | 0.499 |
| walker |  | 4003 | 198 | rust names impl/src/attr.rs |  |  | 0.502 |
| walker |  | 4027 | 24 | rust decl impl/src/attr.rs:302 |  |  | 0.502 |
| ns | 4040 |  | 193 | attr.rs: the three accepted forms of #[error(...)] | 3.7 |  | 0.490 |
| walker |  | 4051 | 24 | rust decl impl/src/attr.rs:342 |  |  | 0.490 |
| walker |  | 4087 | 36 | rust decl impl/src/attr.rs:50 |  |  | 0.495 |
| walker |  | 4123 | 36 | rust decl impl/src/attr.rs:44 |  |  | 0.502 |
| walker |  | 4159 | 36 | rust decl impl/src/attr.rs:38 |  |  | 0.511 |
| ns | 4186 |  | 146 | impl/src/valid.rs: container-level diagnostic messages | 3.8 |  | 0.505 |
| walker |  | 4195 | 36 | rust decl impl/src/attr.rs:32 |  |  | 0.517 |
| walker |  | 4285 | 90 | rust decl impl/src/attr.rs:11 |  |  | 0.526 |
| walker |  | 4377 | 92 | rust decl impl/src/attr.rs:56 |  |  | 0.564 |
| ns | 4478 |  | 292 | valid.rs: attribute-placement and field-attribute diagnostic messages | 3.9 |  | 0.552 |
| walker |  | 4489 | 112 | rust decl impl/src/attr.rs:20 |  |  | 0.583 |
| walker |  | 4541 | 52 | rust decl impl/src/scan_expr.rs:76 |  |  | 0.583 |
| walker |  | 4571 | 30 | README.md section #3 |  |  | 0.584 |
| ns | 4647 |  | 169 | valid.rs: complete validate/check function roster | 3.10 |  | 0.576 |
| walker |  | 4759 | 188 | rust names impl/src/fmt.rs |  |  | 0.576 |
| walker |  | 4792 | 33 | rust decl impl/src/fmt.rs:15 |  |  | 0.576 |
| walker |  | 4806 | 14 | rust decl impl/src/fmt.rs:171 |  |  | 0.576 |
| walker |  | 4838 | 32 | rust decl impl/src/fmt.rs:166 |  |  | 0.577 |
| ns | 4859 |  | 212 | attr.rs: which attributes `get` recognises, plus the file's function roster | 3.11 |  | 0.565 |
| walker |  | 5031 | 193 | rust names impl/src/prop.rs |  |  | 0.566 |
| walker |  | 5073 | 42 | rust decl impl/src/prop.rs:72 |  |  | 0.566 |
| ns | 5096 |  | 237 | impl/src/prop.rs: complete accessor roster for source/from/backtrace fields | 3.12 |  | 0.563 |
| walker |  | 5115 | 42 | rust decl impl/src/prop.rs:127 |  |  | 0.563 |
| walker |  | 5124 | 9 | rust body build/probe.rs:15 |  |  | 0.563 |
| ns | 5305 |  | 209 | prop.rs: how the source and backtrace fields are actually chosen | 3.13 |  | 0.547 |
| walker |  | 5337 | 213 | rust names impl/src/expand.rs |  |  | 0.569 |
| walker |  | 5381 | 44 | rust decl impl/src/expand.rs:536 |  |  | 0.569 |
| walker |  | 5442 | 61 | rust decl impl/src/prop.rs:25 |  |  | 0.574 |
| walker |  | 5497 | 55 | rust body build.rs:190 |  |  | 0.574 |
| walker |  | 5506 | 9 | rust body build/probe.rs:21 |  |  | 0.574 |
| ns | 5595 |  | 290 | impl/src/fmt.rs: expand_shorthand and the format-spec → Trait mapping | 3.14 |  | 0.559 |
| ns | 5726 |  | 131 | fmt.rs: FmtArguments and the remaining function roster | 3.15 | 3.14 | 0.568 |
| walker |  | 5780 | 274 | rust module doc src/lib.rs |  |  | 0.571 |
| walker |  | 5795 | 15 | rust body impl/src/expand.rs:565 |  |  | 0.571 |
| walker |  | 5842 | 47 | rust body impl/src/lib.rs:49 |  |  | 0.571 |
| ns | 5905 |  | 179 | ast.rs: ContainerKind and its six display strings | 3.16 |  | 0.563 |
| walker |  | 5936 | 94 | rust decl impl/src/prop.rs:6 |  |  | 0.571 |
| walker |  | 6030 | 94 | rust decl impl/src/prop.rs:53 |  |  | 0.580 |
| ns | 6198 |  | 293 | ast.rs: complete from_syn constructor roster | 3.17 |  | 0.571 |
| walker |  | 6313 | 283 | rust names impl/src/scan_expr.rs #1 |  |  | 0.571 |
| walker |  | 6334 | 21 | rust decl impl/src/scan_expr.rs:136 |  |  | 0.571 |
| ns | 6357 |  | 159 | impl/src/generics.rs: ParamsInScope and InferredBounds | 3.18 |  | 0.579 |
| walker |  | 6366 | 32 | rust decl impl/src/scan_expr.rs:119 |  |  | 0.579 |
| walker |  | 6398 | 32 | rust decl impl/src/scan_expr.rs:187 |  |  | 0.579 |
| walker |  | 6432 | 34 | rust decl impl/src/scan_expr.rs:130 |  |  | 0.579 |
| walker |  | 6478 | 46 | rust decl impl/src/scan_expr.rs:181 |  |  | 0.579 |
| walker |  | 6525 | 47 | rust decl impl/src/scan_expr.rs:175 |  |  | 0.579 |
| walker |  | 6573 | 48 | rust decl impl/src/scan_expr.rs:124 |  |  | 0.579 |
| ns | 6606 |  | 249 | impl/src/unraw.rs: IdentUnraw/MemberUnraw and the raw-identifier rule | 3.19 |  | 0.570 |
| walker |  | 6639 | 66 | rust decl impl/src/scan_expr.rs:141 |  |  | 0.570 |
| walker |  | 6731 | 92 | rust decl impl/src/scan_expr.rs:166 |  |  | 0.570 |
| walker |  | 6836 | 105 | rust decl impl/src/scan_expr.rs:95 |  |  | 0.570 |
| ns | 6910 |  | 304 | impl/src/fallback.rs: the invalid-input fallback expansion | 3.20 |  | 0.557 |
| walker |  | 7074 | 238 | rust module doc src/lib.rs #1 |  |  | 0.560 |
| ns | 7121 |  | 211 | impl/src/scan_expr.rs: the Input/Action alphabet of the expression scanner | 3.21 |  | 0.548 |
| walker |  | 7196 | 122 | [package] in impl/Cargo.toml |  |  | 0.549 |
| walker |  | 7355 | 159 | manifest config in impl/Cargo.toml |  |  | 0.549 |
| walker |  | 7414 | 59 | README.md section #11 |  |  | 0.550 |
| ns | 7477 |  | 356 | expand.rs: the gate line for every conditionally generated impl | 3.22 | 3.1 | 0.540 |
| walker |  | 7556 | 142 | rust decl impl/src/scan_expr.rs:6 |  |  | 0.567 |
| walker |  | 7565 | 9 | listing of 'tests/no-std' |  |  | 0.567 |
| ns | 7637 |  | 160 | src/private.rs: the complete generated-code support surface | 4.1 |  | 0.560 |
| ns | 7852 |  | 215 | src/aserror.rs: AsDynError and its five blanket/dyn impls | 4.2 |  | 0.553 |
| walker |  | 8093 | 528 | listing of 'tests/ui' |  |  | 0.553 |
| ns | 8094 |  | 242 | src/display.rs: AsDisplay and the std-only Path/PathBuf specializations | 4.3 |  | 0.544 |
| walker |  | 8210 | 117 | rust body build.rs:179 |  |  | 0.544 |
| walker |  | 8282 | 72 | README.md section #2 |  |  | 0.550 |
| walker |  | 8314 | 32 | rust body impl/src/generics.rs:19 |  |  | 0.550 |
| ns | 8376 |  | 282 | src/provide.rs (ThiserrorProvide) and src/var.rs (Var) end to end | 4.4 |  | 0.540 |
| walker |  | 8479 | 165 | rust decl impl/src/scan_expr.rs:63 |  |  | 0.540 |
| ns | 8489 |  | 113 | impl/src/lib.rs: the version-stamped `private` path token | 4.5 |  | 0.545 |
| ns | 8738 |  | 249 | build.rs: the generated __private module and the cfg declarations | 4.6 |  | 0.539 |
| walker |  | 8752 | 273 | rust module doc src/lib.rs #2 |  |  | 0.539 |
| walker |  | 8849 | 97 | rust body impl/src/expand.rs:12 |  |  | 0.546 |
| walker |  | 8863 | 14 | rust body build/probe.rs:27 |  |  | 0.546 |
| walker |  | 8895 | 32 | rust body impl/src/expand.rs:569 |  |  | 0.546 |
| walker |  | 8911 | 16 | rust names tests/test_lints.rs |  |  | 0.546 |
| ns | 8946 |  | 208 | build.rs: rustc capability decisions, function roster, and the probe file | 4.7 |  | 0.541 |
| ns | 9043 |  | 97 | Complete tests/, tests/no-std/ and .github/ listings | 5.1 |  | 0.552 |
| ns | 9131 |  | 88 | tests/compiletest.rs: the trybuild UI harness in full | 5.2 |  | 0.549 |
| walker |  | 9150 | 239 | rust decl impl/src/scan_expr.rs:148 |  |  | 0.549 |
| walker |  | 9187 | 37 | rust body impl/src/generics.rs:13 |  |  | 0.549 |
| walker |  | 9198 | 11 | rust body impl/src/prop.rs:7 |  |  | 0.549 |
| ns | 9262 |  | 131 | tests/no-std/test.rs: the no_std smoke test's error types | 5.3 |  | 0.543 |
| ns | 9389 |  | 127 | Cargo.toml: std-feature rationale and dev-dependencies | 5.4 |  | 0.541 |
| walker |  | 9503 | 305 | README.md section #12 |  |  | 0.553 |
| ns | 9515 |  | 126 | impl/Cargo.toml: the proc-macro crate's manifest | 5.5 |  | 0.557 |
| ns | 9689 |  | 174 | CI job roster and the pinned toolchain components | 5.6 |  | 0.551 |
| walker |  | 9736 | 233 | rust module doc src/lib.rs #3 |  |  | 0.551 |
| walker |  | 9760 | 24 | rust names tests/test_backtrace.rs |  |  | 0.551 |
| walker |  | 9779 | 19 | rust decl tests/test_backtrace.rs:6 |  |  | 0.551 |
| walker |  | 9803 | 24 | rust names tests/test_display.rs |  |  | 0.551 |
| walker |  | 9842 | 39 | rust body impl/src/expand.rs:505 |  |  | 0.551 |
| walker |  | 9976 | 134 | README.md section #9 |  |  | 0.553 |
