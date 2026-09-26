Score(3000)=0.593 I=0.846 C=0.416 ns_rows≤3K=24/56 grid(1000/1442/2080/3000/4327/6240/9000)=0.801/0.701/0.617/0.593/0.497/0.536/0.562

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 38 | 38 | Fs::DirListing { dir: . } |  |  | 0.000 |
| walker |  | 42 | 4 | Fs::DirListing { dir: build } |  |  | 0.000 |
| ns | 86 |  | 86 | Crate identity: name, one-line purpose, version | 1.1 |  | 0.000 |
| walker |  | 89 | 47 | Markdown::ReadmeHeadline { file: README.md } |  |  | 0.507 |
| walker |  | 108 | 19 | Fs::DirListing { dir: impl } |  |  | 0.512 |
| ns | 124 |  | 38 | Complete repository root listing | 1.2 |  | 0.605 |
| walker |  | 133 | 25 | Fs::DirListing { dir: src } |  |  | 0.635 |
| walker |  | 162 | 29 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.638 |
| walker |  | 195 | 33 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.696 |
| walker |  | 203 | 8 | Fs::DirListing { dir: .github } |  |  | 0.696 |
| walker |  | 207 | 4 | Fs::DirListing { dir: .github/workflows } |  |  | 0.696 |
| ns | 215 |  | 91 | Complete listings of both source trees: src/, impl/, impl/src/ | 1.3 |  | 0.498 |
| walker |  | 243 | 36 | Toml::Dependencies { file: Cargo.toml } |  |  | 0.499 |
| walker |  | 290 | 47 | Fs::DirListing { dir: impl/src } |  |  | 0.747 |
| ns | 313 |  | 98 | README canonical example, head: derive, #[from], positional {0} | 1.4 |  | 0.652 |
| ns | 398 |  | 85 | README canonical example, tail: named-field variant, unit variant | 1.5 | 1.4 | 0.589 |
| walker |  | 475 | 185 | Toml::Identity { file: Cargo.toml } |  |  | 0.795 |
| ns | 476 |  | 78 | Derive entry point: #[proc_macro_derive(Error, attributes(...))] | 1.6 |  | 0.757 |
| ns | 612 |  | 136 | Root crate module structure and public re-export (src/lib.rs tail) | 1.7 |  | 0.662 |
| walker |  | 661 | 186 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: true } |  |  | 0.838 |
| walker |  | 682 | 21 | Toml::Operational { file: impl/Cargo.toml } |  |  | 0.838 |
| ns | 707 |  | 95 | Root crate attributes: no_std, docs.rs root, nightly cfg gate | 1.8 |  | 0.797 |
| walker |  | 731 | 49 | Toml::Dependencies { file: impl/Cargo.toml } |  |  | 0.798 |
| ns | 814 |  | 107 | Cargo manifest: features, dependency on impl, workspace members | 1.9 |  | 0.783 |
| walker |  | 851 | 120 | Code::CodeKey { rung: Names, file: impl/src/lib.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.785 |
| walker |  | 863 | 12 | Code::CodeKey { rung: Decl, file: impl/src/lib.rs, decl: 2, sub: 0, line: 45 } |  |  | 0.785 |
| ns | 870 |  | 56 | Documentation map: README section headings + rustdoc mirror in src/lib.rs | 2.1 |  | 0.766 |
| walker |  | 884 | 21 | Code::CodeKey { rung: Decl, file: impl/src/lib.rs, decl: 1, sub: 0, line: 39 } |  |  | 0.773 |
| walker |  | 912 | 28 | Code::CodeKey { rung: Decl, file: impl/src/lib.rs, decl: 3, sub: 0, line: 48 } |  |  | 0.773 |
| walker |  | 983 | 71 | Code::CodeKey { rung: Names, file: src/lib.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.801 |
| ns | 1036 |  | 166 | Details: no public API footprint, allowed error shapes, Display generation | 2.2 |  | 0.754 |
| walker |  | 1055 | 72 | Fs::DirListing { dir: tests } |  |  | 0.755 |
| walker |  | 1064 | 9 | Fs::DirListing { dir: tests/no-std } |  |  | 0.756 |
| ns | 1222 |  | 186 | Display shorthand table: {var}, {0}, {var:?}, {0:?} | 2.3 |  | 0.722 |
| walker |  | 1228 | 164 | Toml::Operational { file: Cargo.toml } |  |  | 0.747 |
| walker |  | 1311 | 83 | Code::CodeKey { rung: Names, file: build.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.747 |
| walker |  | 1357 | 46 | Code::CodeKey { rung: Decl, file: build.rs, decl: 1, sub: 0, line: 10 } |  |  | 0.747 |
| ns | 1404 |  | 182 | Details: #[from] generates From, with its field-count restriction | 2.4 |  | 0.700 |
| walker |  | 1459 | 102 | Markdown::Section { file: README.md, section_index: 7, keeps_default_concavity: false } |  |  | 0.702 |
| walker |  | 1540 | 81 | Code::CodeKey { rung: Names, file: build/probe.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.702 |
| ns | 1557 |  | 153 | Details: source() from #[source] or a field named `source` | 2.5 |  | 0.676 |
| walker |  | 1569 | 29 | Code::CodeKey { rung: Decl, file: build/probe.rs, decl: 3, sub: 0, line: 14 } |  |  | 0.676 |
| walker |  | 1598 | 29 | Code::CodeKey { rung: Decl, file: build/probe.rs, decl: 5, sub: 0, line: 20 } |  |  | 0.676 |
| walker |  | 1628 | 30 | Code::CodeKey { rung: Decl, file: build/probe.rs, decl: 7, sub: 0, line: 26 } |  |  | 0.676 |
| walker |  | 1637 | 9 | Code::CodeKey { rung: Body, file: build/probe.rs, decl: 4, sub: 0, line: 15 } |  |  | 0.676 |
| walker |  | 1646 | 9 | Code::CodeKey { rung: Body, file: build/probe.rs, decl: 6, sub: 0, line: 21 } |  |  | 0.676 |
| ns | 1709 |  | 152 | Details: #[error(transparent)] forwarding | 2.6 |  | 0.642 |
| walker |  | 1773 | 127 | Code::CodeKey { rung: Names, file: src/display.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.642 |
| walker |  | 1782 | 9 | Code::CodeKey { rung: Decl, file: src/display.rs, decl: 8, sub: 0, line: 48 } |  |  | 0.642 |
| walker |  | 1791 | 9 | Code::CodeKey { rung: Decl, file: src/display.rs, decl: 9, sub: 0, line: 50 } |  |  | 0.642 |
| ns | 1839 |  | 130 | Details: provide() and automatic Backtrace field detection | 2.7 |  | 0.622 |
| walker |  | 1843 | 52 | Code::CodeKey { rung: Decl, file: src/display.rs, decl: 3, sub: 0, line: 25 } |  |  | 0.622 |
| walker |  | 1849 | 6 | Code::CodeKey { rung: Decl, file: src/display.rs, decl: 4, sub: 0, line: 29 } |  |  | 0.622 |
| walker |  | 1899 | 50 | Code::CodeKey { rung: Decl, file: src/display.rs, decl: 5, sub: 0, line: 35 } |  |  | 0.622 |
| walker |  | 1905 | 6 | Code::CodeKey { rung: Decl, file: src/display.rs, decl: 6, sub: 0, line: 39 } |  |  | 0.622 |
| ns | 1960 |  | 121 | Details: #[backtrace] on a source field forwards provide() | 2.8 |  | 0.611 |
| walker |  | 1971 | 66 | Code::CodeKey { rung: Decl, file: src/display.rs, decl: 1, sub: 0, line: 14 } |  |  | 0.611 |
| walker |  | 1979 | 8 | Code::CodeKey { rung: Body, file: src/display.rs, decl: 2, sub: 0, line: 20 } |  |  | 0.611 |
| walker |  | 1987 | 8 | Code::CodeKey { rung: Body, file: src/display.rs, decl: 4, sub: 0, line: 29 } |  |  | 0.611 |
| walker |  | 1995 | 8 | Code::CodeKey { rung: Body, file: src/display.rs, decl: 6, sub: 0, line: 39 } |  |  | 0.611 |
| ns | 2005 |  | 45 | Details: #[from] variants with a Backtrace field capture in From | 2.9 |  | 0.607 |
| ns | 2088 |  | 83 | Details example: extra format arguments (`max = i32::MAX`) | 2.10 |  | 0.593 |
| walker |  | 2189 | 194 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.643 |
| walker |  | 2220 | 31 | Code::CodeKey { rung: Body, file: impl/src/lib.rs, decl: 1, sub: 0, line: 39 } |  |  | 0.656 |
| walker |  | 2234 | 14 | Code::CodeKey { rung: Body, file: build/probe.rs, decl: 8, sub: 0, line: 27 } |  |  | 0.656 |
| ns | 2254 |  | 166 | Details example: referring to fields from format args via `.var` / `.0` | 2.11 |  | 0.632 |
| ns | 2410 |  | 156 | When to use thiserror vs anyhow | 2.12 |  | 0.627 |
| walker |  | 2448 | 214 | Code::CodeKey { rung: Names, file: src/aserror.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.628 |
| walker |  | 2476 | 28 | Code::CodeKey { rung: Decl, file: src/aserror.rs, decl: 1, sub: 0, line: 9 } |  |  | 0.628 |
| walker |  | 2482 | 6 | Code::CodeKey { rung: Decl, file: src/aserror.rs, decl: 2, sub: 0, line: 10 } |  |  | 0.628 |
| walker |  | 2510 | 28 | Code::CodeKey { rung: Decl, file: src/aserror.rs, decl: 3, sub: 0, line: 16 } |  |  | 0.628 |
| walker |  | 2516 | 6 | Code::CodeKey { rung: Decl, file: src/aserror.rs, decl: 4, sub: 0, line: 17 } |  |  | 0.628 |
| ns | 2525 |  | 115 | impl/src/expand.rs: complete top-level function roster (names only) | 3.1 |  | 0.609 |
| walker |  | 2544 | 28 | Code::CodeKey { rung: Decl, file: src/aserror.rs, decl: 5, sub: 0, line: 23 } |  |  | 0.609 |
| walker |  | 2550 | 6 | Code::CodeKey { rung: Decl, file: src/aserror.rs, decl: 6, sub: 0, line: 24 } |  |  | 0.609 |
| walker |  | 2578 | 28 | Code::CodeKey { rung: Decl, file: src/aserror.rs, decl: 7, sub: 0, line: 30 } |  |  | 0.609 |
| walker |  | 2584 | 6 | Code::CodeKey { rung: Decl, file: src/aserror.rs, decl: 8, sub: 0, line: 31 } |  |  | 0.609 |
| walker |  | 2614 | 30 | Code::CodeKey { rung: Decl, file: src/aserror.rs, decl: 9, sub: 0, line: 37 } |  |  | 0.609 |
| walker |  | 2620 | 6 | Code::CodeKey { rung: Decl, file: src/aserror.rs, decl: 10, sub: 0, line: 38 } |  |  | 0.609 |
| walker |  | 2627 | 7 | Code::CodeKey { rung: Body, file: src/aserror.rs, decl: 2, sub: 0, line: 10 } |  |  | 0.609 |
| walker |  | 2634 | 7 | Code::CodeKey { rung: Body, file: src/aserror.rs, decl: 4, sub: 0, line: 17 } |  |  | 0.609 |
| walker |  | 2641 | 7 | Code::CodeKey { rung: Body, file: src/aserror.rs, decl: 6, sub: 0, line: 24 } |  |  | 0.609 |
| walker |  | 2675 | 34 | Code::CodeKey { rung: Names, file: src/provide.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.609 |
| ns | 2723 |  | 198 | expand.rs: derive/try_expand — the whole expansion pipeline in 18 lines | 3.2 | 3.1 | 0.581 |
| walker |  | 2733 | 58 | Code::CodeKey { rung: Decl, file: src/provide.rs, decl: 1, sub: 0, line: 8 } |  |  | 0.581 |
| walker |  | 2739 | 6 | Code::CodeKey { rung: Decl, file: src/provide.rs, decl: 2, sub: 0, line: 12 } |  |  | 0.581 |
| walker |  | 2749 | 10 | Code::CodeKey { rung: Body, file: src/provide.rs, decl: 2, sub: 0, line: 12 } |  |  | 0.581 |
| walker |  | 2756 | 7 | Code::CodeKey { rung: Body, file: src/aserror.rs, decl: 8, sub: 0, line: 31 } |  |  | 0.581 |
| walker |  | 2763 | 7 | Code::CodeKey { rung: Body, file: src/aserror.rs, decl: 10, sub: 0, line: 38 } |  |  | 0.581 |
| ns | 2950 |  | 227 | expand.rs: the two emitted impl shapes (struct and enum quote! tails) | 3.3 |  | 0.553 |
| walker |  | 3014 | 251 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.602 |
| walker |  | 3067 | 53 | Code::CodeKey { rung: Names, file: src/var.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.602 |
| walker |  | 3095 | 28 | Code::CodeKey { rung: Decl, file: src/var.rs, decl: 2, sub: 0, line: 5 } |  |  | 0.603 |
| walker |  | 3109 | 14 | Code::CodeKey { rung: Body, file: src/var.rs, decl: 3, sub: 0, line: 6 } |  |  | 0.603 |
| ns | 3290 |  | 340 | impl/src/ast.rs: the complete IR (Input, Struct, Enum, Variant, Field) | 3.4 |  | 0.561 |
| ns | 3528 |  | 238 | impl/src/attr.rs: Attrs and the parsed Display attribute | 3.5 |  | 0.539 |
| walker |  | 3637 | 528 | Fs::DirListing { dir: tests/ui } |  |  | 0.539 |
| walker |  | 3757 | 120 | Toml::Identity { file: impl/Cargo.toml } |  |  | 0.540 |
| walker |  | 3812 | 55 | Code::CodeKey { rung: Body, file: build.rs, decl: 5, sub: 0, line: 190 } |  |  | 0.540 |
| ns | 3847 |  | 319 | attr.rs: Source/From/Transparent/Fmt payloads and the Trait enum | 3.6 |  | 0.506 |
| walker |  | 3859 | 47 | Code::CodeKey { rung: Body, file: impl/src/lib.rs, decl: 4, sub: 0, line: 49 } |  |  | 0.506 |
| ns | 4040 |  | 193 | attr.rs: the three accepted forms of #[error(...)] | 3.7 |  | 0.495 |
| walker |  | 4060 | 201 | Code::CodeKey { rung: ModuleDoc, file: src/lib.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.495 |
| walker |  | 4104 | 44 | Code::CodeKey { rung: Names, file: impl/src/expand.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.495 |
| ns | 4186 |  | 146 | impl/src/valid.rs: container-level diagnostic messages | 3.8 |  | 0.490 |
| walker |  | 4243 | 139 | Code::CodeKey { rung: Names, file: impl/src/attr.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.493 |
| walker |  | 4267 | 24 | Code::CodeKey { rung: Decl, file: impl/src/attr.rs, decl: 9, sub: 0, line: 302 } |  |  | 0.493 |
| walker |  | 4291 | 24 | Code::CodeKey { rung: Decl, file: impl/src/attr.rs, decl: 11, sub: 0, line: 342 } |  |  | 0.493 |
| walker |  | 4327 | 36 | Code::CodeKey { rung: Decl, file: impl/src/attr.rs, decl: 6, sub: 0, line: 50 } |  |  | 0.497 |
| walker |  | 4363 | 36 | Code::CodeKey { rung: Decl, file: impl/src/attr.rs, decl: 5, sub: 0, line: 44 } |  |  | 0.504 |
| walker |  | 4399 | 36 | Code::CodeKey { rung: Decl, file: impl/src/attr.rs, decl: 4, sub: 0, line: 38 } |  |  | 0.513 |
| walker |  | 4435 | 36 | Code::CodeKey { rung: Decl, file: impl/src/attr.rs, decl: 3, sub: 0, line: 32 } |  |  | 0.525 |
| ns | 4478 |  | 292 | valid.rs: attribute-placement and field-attribute diagnostic messages | 3.9 |  | 0.514 |
| walker |  | 4525 | 90 | Code::CodeKey { rung: Decl, file: impl/src/attr.rs, decl: 1, sub: 0, line: 11 } |  |  | 0.524 |
| walker |  | 4617 | 92 | Code::CodeKey { rung: Decl, file: impl/src/attr.rs, decl: 7, sub: 0, line: 56 } |  |  | 0.561 |
| ns | 4647 |  | 169 | valid.rs: complete validate/check function roster | 3.10 |  | 0.550 |
| walker |  | 4729 | 112 | Code::CodeKey { rung: Decl, file: impl/src/attr.rs, decl: 2, sub: 0, line: 20 } |  |  | 0.580 |
| walker |  | 4768 | 39 | Code::CodeKey { rung: Body, file: impl/src/expand.rs, decl: 2, sub: 0, line: 505 } |  |  | 0.580 |
| ns | 4859 |  | 212 | attr.rs: which attributes `get` recognises, plus the file's function roster | 3.11 |  | 0.567 |
| walker |  | 4956 | 188 | Code::CodeKey { rung: Names, file: impl/src/fmt.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.568 |
| walker |  | 4970 | 14 | Code::CodeKey { rung: Decl, file: impl/src/fmt.rs, decl: 4, sub: 0, line: 171 } |  |  | 0.568 |
| walker |  | 5002 | 32 | Code::CodeKey { rung: Decl, file: impl/src/fmt.rs, decl: 3, sub: 0, line: 166 } |  |  | 0.568 |
| walker |  | 5035 | 33 | Code::CodeKey { rung: Decl, file: impl/src/fmt.rs, decl: 1, sub: 0, line: 15 } |  |  | 0.568 |
| ns | 5096 |  | 237 | impl/src/prop.rs: complete accessor roster for source/from/backtrace fields | 3.12 |  | 0.551 |
| walker |  | 5229 | 194 | Code::CodeKey { rung: Names, file: impl/src/scan_expr.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.551 |
| walker |  | 5261 | 32 | Code::CodeKey { rung: Decl, file: impl/src/scan_expr.rs, decl: 7, sub: 0, line: 84 } |  |  | 0.551 |
| walker |  | 5301 | 40 | Code::CodeKey { rung: Decl, file: impl/src/scan_expr.rs, decl: 10, sub: 0, line: 104 } |  |  | 0.551 |
| ns | 5305 |  | 209 | prop.rs: how the source and backtrace fields are actually chosen | 3.13 |  | 0.536 |
| walker |  | 5343 | 42 | Code::CodeKey { rung: Decl, file: impl/src/scan_expr.rs, decl: 2, sub: 0, line: 25 } |  |  | 0.536 |
| walker |  | 5392 | 49 | Code::CodeKey { rung: Decl, file: impl/src/scan_expr.rs, decl: 8, sub: 0, line: 89 } |  |  | 0.536 |
| walker |  | 5444 | 52 | Code::CodeKey { rung: Decl, file: impl/src/scan_expr.rs, decl: 5, sub: 0, line: 76 } |  |  | 0.536 |
| walker |  | 5586 | 142 | Code::CodeKey { rung: Decl, file: impl/src/scan_expr.rs, decl: 1, sub: 0, line: 6 } |  |  | 0.538 |
| ns | 5595 |  | 290 | impl/src/fmt.rs: expand_shorthand and the format-spec → Trait mapping | 3.14 |  | 0.525 |
| walker |  | 5691 | 105 | Code::CodeKey { rung: Decl, file: impl/src/scan_expr.rs, decl: 9, sub: 0, line: 95 } |  |  | 0.525 |
| ns | 5726 |  | 131 | fmt.rs: FmtArguments and the remaining function roster | 3.15 | 3.14 | 0.535 |
| walker |  | 5856 | 165 | Code::CodeKey { rung: Decl, file: impl/src/scan_expr.rs, decl: 4, sub: 0, line: 63 } |  |  | 0.535 |
| ns | 5905 |  | 179 | ast.rs: ContainerKind and its six display strings | 3.16 |  | 0.525 |
| walker |  | 6006 | 150 | Code::CodeKey { rung: Decl, file: impl/src/scan_expr.rs, decl: 3, sub: 0, line: 32 } |  |  | 0.525 |
| walker |  | 6128 | 122 | Code::CodeKey { rung: Names, file: impl/src/valid.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.530 |
| walker |  | 6147 | 19 | Code::CodeKey { rung: Decl, file: impl/src/valid.rs, decl: 3, sub: 0, line: 14 } |  |  | 0.533 |
| walker |  | 6166 | 19 | Code::CodeKey { rung: Decl, file: impl/src/valid.rs, decl: 5, sub: 0, line: 45 } |  |  | 0.535 |
| walker |  | 6185 | 19 | Code::CodeKey { rung: Decl, file: impl/src/valid.rs, decl: 7, sub: 0, line: 66 } |  |  | 0.538 |
| ns | 6198 |  | 293 | ast.rs: complete from_syn constructor roster | 3.17 |  | 0.529 |
| walker |  | 6204 | 19 | Code::CodeKey { rung: Decl, file: impl/src/valid.rs, decl: 9, sub: 0, line: 91 } |  |  | 0.532 |
| walker |  | 6226 | 22 | Code::CodeKey { rung: Decl, file: impl/src/valid.rs, decl: 1, sub: 0, line: 5 } |  |  | 0.536 |
| walker |  | 6331 | 105 | Code::CodeKey { rung: Names, file: impl/src/ast.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.538 |
| walker |  | 6357 | 26 | Code::CodeKey { rung: Decl, file: impl/src/ast.rs, decl: 1, sub: 0, line: 10 } |  |  | 0.531 |
| ns | 6357 |  | 159 | impl/src/generics.rs: ParamsInScope and InferredBounds | 3.18 |  | 0.531 |
| walker |  | 6385 | 28 | Code::CodeKey { rung: Decl, file: impl/src/ast.rs, decl: 9, sub: 0, line: 174 } |  |  | 0.532 |
| walker |  | 6414 | 29 | Code::CodeKey { rung: Decl, file: impl/src/ast.rs, decl: 7, sub: 0, line: 54 } |  |  | 0.533 |
| walker |  | 6467 | 53 | Code::CodeKey { rung: Decl, file: impl/src/ast.rs, decl: 2, sub: 0, line: 15 } |  |  | 0.538 |
| walker |  | 6520 | 53 | Code::CodeKey { rung: Decl, file: impl/src/ast.rs, decl: 4, sub: 0, line: 29 } |  |  | 0.547 |
| walker |  | 6574 | 54 | Code::CodeKey { rung: Decl, file: impl/src/ast.rs, decl: 3, sub: 0, line: 22 } |  |  | 0.558 |
| ns | 6606 |  | 249 | impl/src/unraw.rs: IdentUnraw/MemberUnraw and the raw-identifier rule | 3.19 |  | 0.546 |
| walker |  | 6635 | 61 | Code::CodeKey { rung: Decl, file: impl/src/ast.rs, decl: 6, sub: 0, line: 44 } |  |  | 0.550 |
| walker |  | 6700 | 65 | Code::CodeKey { rung: Decl, file: impl/src/ast.rs, decl: 5, sub: 0, line: 36 } |  |  | 0.566 |
| walker |  | 6862 | 162 | Code::CodeKey { rung: Decl, file: impl/src/scan_expr.rs, decl: 3, sub: 1, line: 32 } |  |  | 0.566 |
| ns | 6910 |  | 304 | impl/src/fallback.rs: the invalid-input fallback expansion | 3.20 |  | 0.553 |
| walker |  | 7001 | 139 | Code::CodeKey { rung: Decl, file: impl/src/scan_expr.rs, decl: 3, sub: 2, line: 32 } |  |  | 0.553 |
| ns | 7121 |  | 211 | impl/src/scan_expr.rs: the Input/Action alphabet of the expression scanner | 3.21 |  | 0.569 |
| walker |  | 7185 | 184 | Code::CodeKey { rung: Names, file: impl/src/scan_expr.rs, decl: 0, sub: 1, line: 0 } |  |  | 0.569 |
| walker |  | 7206 | 21 | Code::CodeKey { rung: Decl, file: impl/src/scan_expr.rs, decl: 17, sub: 0, line: 136 } |  |  | 0.569 |
| walker |  | 7238 | 32 | Code::CodeKey { rung: Decl, file: impl/src/scan_expr.rs, decl: 13, sub: 0, line: 119 } |  |  | 0.569 |
| walker |  | 7272 | 34 | Code::CodeKey { rung: Decl, file: impl/src/scan_expr.rs, decl: 15, sub: 0, line: 130 } |  |  | 0.569 |
| walker |  | 7308 | 36 | Code::CodeKey { rung: Decl, file: impl/src/scan_expr.rs, decl: 12, sub: 0, line: 114 } |  |  | 0.569 |
| walker |  | 7349 | 41 | Code::CodeKey { rung: Decl, file: impl/src/scan_expr.rs, decl: 11, sub: 0, line: 109 } |  |  | 0.569 |
| walker |  | 7397 | 48 | Code::CodeKey { rung: Decl, file: impl/src/scan_expr.rs, decl: 14, sub: 0, line: 124 } |  |  | 0.569 |
| ns | 7477 |  | 356 | expand.rs: the gate line for every conditionally generated impl | 3.22 | 3.1 | 0.558 |
| walker |  | 7532 | 135 | Code::CodeKey { rung: Names, file: impl/src/scan_expr.rs, decl: 0, sub: 2, line: 0 } |  |  | 0.558 |
| walker |  | 7564 | 32 | Code::CodeKey { rung: Decl, file: impl/src/scan_expr.rs, decl: 24, sub: 0, line: 187 } |  |  | 0.558 |
| walker |  | 7610 | 46 | Code::CodeKey { rung: Decl, file: impl/src/scan_expr.rs, decl: 23, sub: 0, line: 181 } |  |  | 0.558 |
| ns | 7637 |  | 160 | src/private.rs: the complete generated-code support surface | 4.1 |  | 0.551 |
| walker |  | 7657 | 47 | Code::CodeKey { rung: Decl, file: impl/src/scan_expr.rs, decl: 22, sub: 0, line: 175 } |  |  | 0.551 |
| walker |  | 7723 | 66 | Code::CodeKey { rung: Decl, file: impl/src/scan_expr.rs, decl: 19, sub: 0, line: 141 } |  |  | 0.551 |
| walker |  | 7815 | 92 | Code::CodeKey { rung: Decl, file: impl/src/scan_expr.rs, decl: 21, sub: 0, line: 166 } |  |  | 0.551 |
| ns | 7852 |  | 215 | src/aserror.rs: AsDynError and its five blanket/dyn impls | 4.2 |  | 0.550 |
| walker |  | 8054 | 239 | Code::CodeKey { rung: Decl, file: impl/src/scan_expr.rs, decl: 20, sub: 0, line: 148 } |  |  | 0.550 |
| ns | 8094 |  | 242 | src/display.rs: AsDisplay and the std-only Path/PathBuf specializations | 4.3 |  | 0.546 |
| walker |  | 8247 | 193 | Code::CodeKey { rung: Names, file: impl/src/prop.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.553 |
| walker |  | 8289 | 42 | Code::CodeKey { rung: Decl, file: impl/src/prop.rs, decl: 15, sub: 0, line: 72 } |  |  | 0.555 |
| walker |  | 8331 | 42 | Code::CodeKey { rung: Decl, file: impl/src/prop.rs, decl: 21, sub: 0, line: 127 } |  |  | 0.555 |
| ns | 8376 |  | 282 | src/provide.rs (ThiserrorProvide) and src/var.rs (Var) end to end | 4.4 |  | 0.553 |
| walker |  | 8392 | 61 | Code::CodeKey { rung: Decl, file: impl/src/prop.rs, decl: 6, sub: 0, line: 25 } |  |  | 0.557 |
| walker |  | 8486 | 94 | Code::CodeKey { rung: Decl, file: impl/src/prop.rs, decl: 1, sub: 0, line: 6 } |  |  | 0.562 |
| ns | 8489 |  | 113 | impl/src/lib.rs: the version-stamped `private` path token | 4.5 |  | 0.567 |
| walker |  | 8580 | 94 | Code::CodeKey { rung: Decl, file: impl/src/prop.rs, decl: 10, sub: 0, line: 53 } |  |  | 0.574 |
| ns | 8738 |  | 249 | build.rs: the generated __private module and the cfg declarations | 4.6 |  | 0.567 |
| walker |  | 8834 | 254 | Code::CodeKey { rung: Names, file: impl/src/unraw.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.567 |
| walker |  | 8851 | 17 | Code::CodeKey { rung: Decl, file: impl/src/unraw.rs, decl: 1, sub: 0, line: 10 } |  |  | 0.567 |
| walker |  | 8870 | 19 | Code::CodeKey { rung: Decl, file: impl/src/unraw.rs, decl: 22, sub: 0, line: 87 } |  |  | 0.567 |
| walker |  | 8892 | 22 | Code::CodeKey { rung: Decl, file: impl/src/unraw.rs, decl: 17, sub: 0, line: 69 } |  |  | 0.567 |
| walker |  | 8915 | 23 | Code::CodeKey { rung: Decl, file: impl/src/unraw.rs, decl: 9, sub: 0, line: 45 } |  |  | 0.567 |
| walker |  | 8938 | 23 | Code::CodeKey { rung: Decl, file: impl/src/unraw.rs, decl: 11, sub: 0, line: 51 } |  |  | 0.567 |
| ns | 8946 |  | 208 | build.rs: rustc capability decisions, function roster, and the probe file | 4.7 |  | 0.562 |
| walker |  | 8961 | 23 | Code::CodeKey { rung: Decl, file: impl/src/unraw.rs, decl: 13, sub: 0, line: 57 } |  |  | 0.562 |
| walker |  | 8984 | 23 | Code::CodeKey { rung: Decl, file: impl/src/unraw.rs, decl: 27, sub: 0, line: 107 } |  |  | 0.562 |
| walker |  | 9007 | 23 | Code::CodeKey { rung: Decl, file: impl/src/unraw.rs, decl: 29, sub: 0, line: 117 } |  |  | 0.562 |
| walker |  | 9031 | 24 | Code::CodeKey { rung: Decl, file: impl/src/unraw.rs, decl: 33, sub: 0, line: 135 } |  |  | 0.562 |
| ns | 9043 |  | 97 | Complete tests/, tests/no-std/ and .github/ listings | 5.1 |  | 0.572 |
| walker |  | 9057 | 26 | Code::CodeKey { rung: Decl, file: impl/src/unraw.rs, decl: 19, sub: 0, line: 75 } |  |  | 0.572 |
| walker |  | 9084 | 27 | Code::CodeKey { rung: Decl, file: impl/src/unraw.rs, decl: 15, sub: 0, line: 63 } |  |  | 0.572 |
| walker |  | 9111 | 27 | Code::CodeKey { rung: Decl, file: impl/src/unraw.rs, decl: 31, sub: 0, line: 126 } |  |  | 0.572 |
| ns | 9131 |  | 88 | tests/compiletest.rs: the trybuild UI harness in full | 5.2 |  | 0.569 |
| walker |  | 9139 | 28 | Code::CodeKey { rung: Decl, file: impl/src/unraw.rs, decl: 6, sub: 0, line: 37 } |  |  | 0.569 |
| walker |  | 9167 | 28 | Code::CodeKey { rung: Decl, file: impl/src/unraw.rs, decl: 24, sub: 0, line: 96 } |  |  | 0.569 |
| walker |  | 9196 | 29 | Code::CodeKey { rung: Decl, file: impl/src/unraw.rs, decl: 21, sub: 0, line: 81 } |  |  | 0.571 |
| walker |  | 9254 | 58 | Code::CodeKey { rung: Decl, file: impl/src/unraw.rs, decl: 2, sub: 0, line: 14 } |  |  | 0.571 |
| ns | 9262 |  | 131 | tests/no-std/test.rs: the no_std smoke test's error types | 5.3 |  | 0.565 |
| walker |  | 9311 | 57 | Code::CodeKey { rung: Names, file: impl/src/generics.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.565 |
| walker |  | 9327 | 16 | Code::CodeKey { rung: Decl, file: impl/src/generics.rs, decl: 1, sub: 0, line: 8 } |  |  | 0.566 |
| walker |  | 9366 | 39 | Code::CodeKey { rung: Decl, file: impl/src/generics.rs, decl: 5, sub: 0, line: 48 } |  |  | 0.570 |
| ns | 9389 |  | 127 | Cargo.toml: std-feature rationale and dev-dependencies | 5.4 |  | 0.568 |
| walker |  | 9414 | 48 | Code::CodeKey { rung: Decl, file: impl/src/generics.rs, decl: 2, sub: 0, line: 12 } |  |  | 0.570 |
| walker |  | 9484 | 70 | Code::CodeKey { rung: Decl, file: impl/src/generics.rs, decl: 6, sub: 0, line: 53 } |  |  | 0.575 |
| ns | 9515 |  | 126 | impl/Cargo.toml: the proc-macro crate's manifest | 5.5 |  | 0.579 |
| ns | 9689 |  | 174 | CI job roster and the pinned toolchain components | 5.6 |  | 0.572 |
| walker |  | 9949 | 465 | Markdown::Section { file: README.md, section_index: 4, keeps_default_concavity: false } |  |  | 0.608 |
| walker |  | 9979 | 30 | Code::CodeKey { rung: Names, file: impl/src/fallback.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.608 |
| walker |  | 9993 | 14 | Code::CodeKey { rung: ModuleDoc, file: src/lib.rs, decl: 0, sub: 1, line: 0 } |  |  | 0.608 |
