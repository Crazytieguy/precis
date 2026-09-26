Score(3000)=0.554 I=0.833 C=0.368 ns_rows≤3K=24/56 grid(1000/1442/2080/3000/4327/6240/9000)=0.801/0.700/0.622/0.554/0.556/0.558/0.580

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
| walker |  | 170 | 8 | Fs::DirListing { dir: .github } |  |  | 0.638 |
| walker |  | 174 | 4 | Fs::DirListing { dir: .github/workflows } |  |  | 0.638 |
| ns | 215 |  | 91 | Complete listings of both source trees: src/, impl/, impl/src/ | 1.3 |  | 0.460 |
| walker |  | 246 | 72 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.498 |
| walker |  | 282 | 36 | Toml::Dependencies { file: Cargo.toml } |  |  | 0.499 |
| ns | 313 |  | 98 | README canonical example, head: derive, #[from], positional {0} | 1.4 |  | 0.435 |
| walker |  | 329 | 47 | Fs::DirListing { dir: impl/src } |  |  | 0.652 |
| ns | 398 |  | 85 | README canonical example, tail: named-field variant, unit variant | 1.5 | 1.4 | 0.589 |
| ns | 476 |  | 78 | Derive entry point: #[proc_macro_derive(Error, attributes(...))] | 1.6 |  | 0.561 |
| walker |  | 514 | 185 | Toml::Identity { file: Cargo.toml } |  |  | 0.757 |
| ns | 612 |  | 136 | Root crate module structure and public re-export (src/lib.rs tail) | 1.7 |  | 0.662 |
| ns | 707 |  | 95 | Root crate attributes: no_std, docs.rs root, nightly cfg gate | 1.8 |  | 0.629 |
| walker |  | 710 | 196 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: true } |  |  | 0.797 |
| walker |  | 731 | 21 | Toml::Operational { file: impl/Cargo.toml } |  |  | 0.797 |
| ns | 814 |  | 107 | Cargo manifest: features, dependency on impl, workspace members | 1.9 |  | 0.782 |
| walker |  | 851 | 120 | Code::CodeKey { rung: Names, file: impl/src/lib.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.784 |
| walker |  | 863 | 12 | Code::CodeKey { rung: Decl, file: impl/src/lib.rs, decl: 2, sub: 0, line: 45 } |  |  | 0.784 |
| ns | 870 |  | 56 | Documentation map: README section headings + rustdoc mirror in src/lib.rs | 2.1 |  | 0.766 |
| walker |  | 884 | 21 | Code::CodeKey { rung: Decl, file: impl/src/lib.rs, decl: 1, sub: 0, line: 39 } |  |  | 0.772 |
| walker |  | 912 | 28 | Code::CodeKey { rung: Decl, file: impl/src/lib.rs, decl: 3, sub: 0, line: 48 } |  |  | 0.773 |
| walker |  | 983 | 71 | Code::CodeKey { rung: Names, file: src/lib.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.801 |
| ns | 1036 |  | 166 | Details: no public API footprint, allowed error shapes, Display generation | 2.2 |  | 0.753 |
| walker |  | 1055 | 72 | Fs::DirListing { dir: tests } |  |  | 0.755 |
| walker |  | 1064 | 9 | Fs::DirListing { dir: tests/no-std } |  |  | 0.756 |
| ns | 1222 |  | 186 | Display shorthand table: {var}, {0}, {var:?}, {0:?} | 2.3 |  | 0.722 |
| walker |  | 1228 | 164 | Toml::Operational { file: Cargo.toml } |  |  | 0.746 |
| walker |  | 1311 | 83 | Code::CodeKey { rung: Names, file: build.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.746 |
| walker |  | 1357 | 46 | Code::CodeKey { rung: Decl, file: build.rs, decl: 1, sub: 0, line: 10 } |  |  | 0.747 |
| ns | 1404 |  | 182 | Details: #[from] generates From, with its field-count restriction | 2.4 |  | 0.700 |
| walker |  | 1406 | 49 | Toml::Dependencies { file: impl/Cargo.toml } |  |  | 0.700 |
| walker |  | 1544 | 138 | Markdown::Section { file: README.md, section_index: 7, keeps_default_concavity: false } |  |  | 0.702 |
| ns | 1557 |  | 153 | Details: source() from #[source] or a field named `source` | 2.5 |  | 0.676 |
| walker |  | 1625 | 81 | Code::CodeKey { rung: Names, file: build/probe.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.676 |
| walker |  | 1654 | 29 | Code::CodeKey { rung: Decl, file: build/probe.rs, decl: 3, sub: 0, line: 14 } |  |  | 0.676 |
| walker |  | 1683 | 29 | Code::CodeKey { rung: Decl, file: build/probe.rs, decl: 5, sub: 0, line: 20 } |  |  | 0.676 |
| ns | 1709 |  | 152 | Details: #[error(transparent)] forwarding | 2.6 |  | 0.642 |
| walker |  | 1713 | 30 | Code::CodeKey { rung: Decl, file: build/probe.rs, decl: 7, sub: 0, line: 26 } |  |  | 0.642 |
| walker |  | 1722 | 9 | Code::CodeKey { rung: Body, file: build/probe.rs, decl: 4, sub: 0, line: 15 } |  |  | 0.642 |
| walker |  | 1731 | 9 | Code::CodeKey { rung: Body, file: build/probe.rs, decl: 6, sub: 0, line: 21 } |  |  | 0.642 |
| walker |  | 1762 | 31 | Code::CodeKey { rung: Body, file: impl/src/lib.rs, decl: 1, sub: 0, line: 39 } |  |  | 0.657 |
| walker |  | 1796 | 34 | Code::CodeKey { rung: Names, file: src/provide.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.657 |
| ns | 1839 |  | 130 | Details: provide() and automatic Backtrace field detection | 2.7 |  | 0.636 |
| walker |  | 1854 | 58 | Code::CodeKey { rung: Decl, file: src/provide.rs, decl: 1, sub: 0, line: 8 } |  |  | 0.636 |
| walker |  | 1860 | 6 | Code::CodeKey { rung: Decl, file: src/provide.rs, decl: 2, sub: 0, line: 12 } |  |  | 0.636 |
| walker |  | 1870 | 10 | Code::CodeKey { rung: Body, file: src/provide.rs, decl: 2, sub: 0, line: 12 } |  |  | 0.636 |
| ns | 1960 |  | 121 | Details: #[backtrace] on a source field forwards provide() | 2.8 |  | 0.625 |
| walker |  | 1975 | 105 | Code::CodeKey { rung: Names, file: src/display.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.626 |
| walker |  | 1984 | 9 | Code::CodeKey { rung: Decl, file: src/display.rs, decl: 8, sub: 0, line: 48 } |  |  | 0.626 |
| walker |  | 1993 | 9 | Code::CodeKey { rung: Decl, file: src/display.rs, decl: 9, sub: 0, line: 50 } |  |  | 0.626 |
| ns | 2005 |  | 45 | Details: #[from] variants with a Backtrace field capture in From | 2.9 |  | 0.621 |
| walker |  | 2045 | 52 | Code::CodeKey { rung: Decl, file: src/display.rs, decl: 3, sub: 0, line: 25 } |  |  | 0.622 |
| walker |  | 2051 | 6 | Code::CodeKey { rung: Decl, file: src/display.rs, decl: 4, sub: 0, line: 29 } |  |  | 0.622 |
| ns | 2088 |  | 83 | Details example: extra format arguments (`max = i32::MAX`) | 2.10 |  | 0.607 |
| walker |  | 2101 | 50 | Code::CodeKey { rung: Decl, file: src/display.rs, decl: 5, sub: 0, line: 35 } |  |  | 0.607 |
| walker |  | 2107 | 6 | Code::CodeKey { rung: Decl, file: src/display.rs, decl: 6, sub: 0, line: 39 } |  |  | 0.607 |
| walker |  | 2173 | 66 | Code::CodeKey { rung: Decl, file: src/display.rs, decl: 1, sub: 0, line: 14 } |  |  | 0.607 |
| walker |  | 2181 | 8 | Code::CodeKey { rung: Body, file: src/display.rs, decl: 2, sub: 0, line: 20 } |  |  | 0.607 |
| walker |  | 2189 | 8 | Code::CodeKey { rung: Body, file: src/display.rs, decl: 4, sub: 0, line: 29 } |  |  | 0.607 |
| walker |  | 2197 | 8 | Code::CodeKey { rung: Body, file: src/display.rs, decl: 6, sub: 0, line: 39 } |  |  | 0.607 |
| walker |  | 2211 | 14 | Code::CodeKey { rung: Body, file: build/probe.rs, decl: 8, sub: 0, line: 27 } |  |  | 0.607 |
| ns | 2254 |  | 166 | Details example: referring to fields from format args via `.var` / `.0` | 2.11 |  | 0.585 |
| walker |  | 2405 | 194 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.632 |
| ns | 2410 |  | 156 | When to use thiserror vs anyhow | 2.12 |  | 0.628 |
| ns | 2525 |  | 115 | impl/src/expand.rs: complete top-level function roster (names only) | 3.1 |  | 0.608 |
| walker |  | 2619 | 214 | Code::CodeKey { rung: Names, file: src/aserror.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.609 |
| walker |  | 2647 | 28 | Code::CodeKey { rung: Decl, file: src/aserror.rs, decl: 1, sub: 0, line: 9 } |  |  | 0.609 |
| walker |  | 2653 | 6 | Code::CodeKey { rung: Decl, file: src/aserror.rs, decl: 2, sub: 0, line: 10 } |  |  | 0.609 |
| walker |  | 2681 | 28 | Code::CodeKey { rung: Decl, file: src/aserror.rs, decl: 3, sub: 0, line: 16 } |  |  | 0.609 |
| walker |  | 2687 | 6 | Code::CodeKey { rung: Decl, file: src/aserror.rs, decl: 4, sub: 0, line: 17 } |  |  | 0.609 |
| walker |  | 2715 | 28 | Code::CodeKey { rung: Decl, file: src/aserror.rs, decl: 5, sub: 0, line: 23 } |  |  | 0.609 |
| walker |  | 2721 | 6 | Code::CodeKey { rung: Decl, file: src/aserror.rs, decl: 6, sub: 0, line: 24 } |  |  | 0.609 |
| ns | 2723 |  | 198 | expand.rs: derive/try_expand — the whole expansion pipeline in 18 lines | 3.2 | 3.1 | 0.581 |
| walker |  | 2749 | 28 | Code::CodeKey { rung: Decl, file: src/aserror.rs, decl: 7, sub: 0, line: 30 } |  |  | 0.581 |
| walker |  | 2755 | 6 | Code::CodeKey { rung: Decl, file: src/aserror.rs, decl: 8, sub: 0, line: 31 } |  |  | 0.581 |
| walker |  | 2785 | 30 | Code::CodeKey { rung: Decl, file: src/aserror.rs, decl: 9, sub: 0, line: 37 } |  |  | 0.581 |
| walker |  | 2791 | 6 | Code::CodeKey { rung: Decl, file: src/aserror.rs, decl: 10, sub: 0, line: 38 } |  |  | 0.581 |
| walker |  | 2798 | 7 | Code::CodeKey { rung: Body, file: src/aserror.rs, decl: 2, sub: 0, line: 10 } |  |  | 0.581 |
| walker |  | 2805 | 7 | Code::CodeKey { rung: Body, file: src/aserror.rs, decl: 4, sub: 0, line: 17 } |  |  | 0.581 |
| walker |  | 2812 | 7 | Code::CodeKey { rung: Body, file: src/aserror.rs, decl: 6, sub: 0, line: 24 } |  |  | 0.581 |
| walker |  | 2819 | 7 | Code::CodeKey { rung: Body, file: src/aserror.rs, decl: 8, sub: 0, line: 31 } |  |  | 0.581 |
| walker |  | 2826 | 7 | Code::CodeKey { rung: Body, file: src/aserror.rs, decl: 10, sub: 0, line: 38 } |  |  | 0.581 |
| walker |  | 2931 | 105 | Code::CodeKey { rung: Names, file: impl/src/ast.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.581 |
| ns | 2950 |  | 227 | expand.rs: the two emitted impl shapes (struct and enum quote! tails) | 3.3 |  | 0.554 |
| walker |  | 2957 | 26 | Code::CodeKey { rung: Decl, file: impl/src/ast.rs, decl: 1, sub: 0, line: 10 } |  |  | 0.554 |
| walker |  | 2985 | 28 | Code::CodeKey { rung: Decl, file: impl/src/ast.rs, decl: 9, sub: 0, line: 174 } |  |  | 0.554 |
| walker |  | 3014 | 29 | Code::CodeKey { rung: Decl, file: impl/src/ast.rs, decl: 7, sub: 0, line: 54 } |  |  | 0.554 |
| walker |  | 3067 | 53 | Code::CodeKey { rung: Decl, file: impl/src/ast.rs, decl: 2, sub: 0, line: 15 } |  |  | 0.555 |
| walker |  | 3120 | 53 | Code::CodeKey { rung: Decl, file: impl/src/ast.rs, decl: 4, sub: 0, line: 29 } |  |  | 0.556 |
| walker |  | 3174 | 54 | Code::CodeKey { rung: Decl, file: impl/src/ast.rs, decl: 3, sub: 0, line: 22 } |  |  | 0.557 |
| walker |  | 3235 | 61 | Code::CodeKey { rung: Decl, file: impl/src/ast.rs, decl: 6, sub: 0, line: 44 } |  |  | 0.557 |
| ns | 3290 |  | 340 | impl/src/ast.rs: the complete IR (Input, Struct, Enum, Variant, Field) | 3.4 |  | 0.567 |
| walker |  | 3300 | 65 | Code::CodeKey { rung: Decl, file: impl/src/ast.rs, decl: 5, sub: 0, line: 36 } |  |  | 0.597 |
| ns | 3528 |  | 238 | impl/src/attr.rs: Attrs and the parsed Display attribute | 3.5 |  | 0.573 |
| walker |  | 3551 | 251 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.614 |
| walker |  | 3606 | 55 | Code::CodeKey { rung: Body, file: build.rs, decl: 5, sub: 0, line: 190 } |  |  | 0.614 |
| walker |  | 3659 | 53 | Code::CodeKey { rung: Names, file: src/var.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.614 |
| walker |  | 3687 | 28 | Code::CodeKey { rung: Decl, file: src/var.rs, decl: 2, sub: 0, line: 5 } |  |  | 0.614 |
| walker |  | 3701 | 14 | Code::CodeKey { rung: Body, file: src/var.rs, decl: 3, sub: 0, line: 6 } |  |  | 0.614 |
| ns | 3847 |  | 319 | attr.rs: Source/From/Transparent/Fmt payloads and the Trait enum | 3.6 |  | 0.576 |
| walker |  | 3955 | 254 | Code::CodeKey { rung: Names, file: impl/src/unraw.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.576 |
| walker |  | 3972 | 17 | Code::CodeKey { rung: Decl, file: impl/src/unraw.rs, decl: 1, sub: 0, line: 10 } |  |  | 0.576 |
| walker |  | 3991 | 19 | Code::CodeKey { rung: Decl, file: impl/src/unraw.rs, decl: 22, sub: 0, line: 87 } |  |  | 0.576 |
| walker |  | 4013 | 22 | Code::CodeKey { rung: Decl, file: impl/src/unraw.rs, decl: 17, sub: 0, line: 69 } |  |  | 0.576 |
| walker |  | 4036 | 23 | Code::CodeKey { rung: Decl, file: impl/src/unraw.rs, decl: 9, sub: 0, line: 45 } |  |  | 0.576 |
| ns | 4040 |  | 193 | attr.rs: the three accepted forms of #[error(...)] | 3.7 |  | 0.562 |
| walker |  | 4059 | 23 | Code::CodeKey { rung: Decl, file: impl/src/unraw.rs, decl: 11, sub: 0, line: 51 } |  |  | 0.562 |
| walker |  | 4082 | 23 | Code::CodeKey { rung: Decl, file: impl/src/unraw.rs, decl: 13, sub: 0, line: 57 } |  |  | 0.562 |
| walker |  | 4105 | 23 | Code::CodeKey { rung: Decl, file: impl/src/unraw.rs, decl: 27, sub: 0, line: 107 } |  |  | 0.562 |
| walker |  | 4128 | 23 | Code::CodeKey { rung: Decl, file: impl/src/unraw.rs, decl: 29, sub: 0, line: 117 } |  |  | 0.562 |
| walker |  | 4152 | 24 | Code::CodeKey { rung: Decl, file: impl/src/unraw.rs, decl: 33, sub: 0, line: 135 } |  |  | 0.562 |
| walker |  | 4178 | 26 | Code::CodeKey { rung: Decl, file: impl/src/unraw.rs, decl: 19, sub: 0, line: 75 } |  |  | 0.562 |
| ns | 4186 |  | 146 | impl/src/valid.rs: container-level diagnostic messages | 3.8 |  | 0.556 |
| walker |  | 4205 | 27 | Code::CodeKey { rung: Decl, file: impl/src/unraw.rs, decl: 15, sub: 0, line: 63 } |  |  | 0.556 |
| walker |  | 4232 | 27 | Code::CodeKey { rung: Decl, file: impl/src/unraw.rs, decl: 31, sub: 0, line: 126 } |  |  | 0.556 |
| walker |  | 4260 | 28 | Code::CodeKey { rung: Decl, file: impl/src/unraw.rs, decl: 6, sub: 0, line: 37 } |  |  | 0.556 |
| walker |  | 4288 | 28 | Code::CodeKey { rung: Decl, file: impl/src/unraw.rs, decl: 24, sub: 0, line: 96 } |  |  | 0.556 |
| walker |  | 4317 | 29 | Code::CodeKey { rung: Decl, file: impl/src/unraw.rs, decl: 21, sub: 0, line: 81 } |  |  | 0.556 |
| walker |  | 4375 | 58 | Code::CodeKey { rung: Decl, file: impl/src/unraw.rs, decl: 2, sub: 0, line: 14 } |  |  | 0.556 |
| walker |  | 4422 | 47 | Code::CodeKey { rung: Body, file: impl/src/lib.rs, decl: 4, sub: 0, line: 49 } |  |  | 0.557 |
| ns | 4478 |  | 292 | valid.rs: attribute-placement and field-attribute diagnostic messages | 3.9 |  | 0.546 |
| ns | 4647 |  | 169 | valid.rs: complete validate/check function roster | 3.10 |  | 0.535 |
| ns | 4859 |  | 212 | attr.rs: which attributes `get` recognises, plus the file's function roster | 3.11 |  | 0.523 |
| walker |  | 4950 | 528 | Fs::DirListing { dir: tests/ui } |  |  | 0.523 |
| walker |  | 5070 | 120 | Toml::Identity { file: impl/Cargo.toml } |  |  | 0.523 |
| ns | 5096 |  | 237 | impl/src/prop.rs: complete accessor roster for source/from/backtrace fields | 3.12 |  | 0.507 |
| walker |  | 5268 | 198 | Code::CodeKey { rung: Names, file: impl/src/attr.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.511 |
| walker |  | 5292 | 24 | Code::CodeKey { rung: Decl, file: impl/src/attr.rs, decl: 11, sub: 0, line: 302 } |  |  | 0.511 |
| ns | 5305 |  | 209 | prop.rs: how the source and backtrace fields are actually chosen | 3.13 |  | 0.497 |
| walker |  | 5316 | 24 | Code::CodeKey { rung: Decl, file: impl/src/attr.rs, decl: 13, sub: 0, line: 342 } |  |  | 0.497 |
| walker |  | 5352 | 36 | Code::CodeKey { rung: Decl, file: impl/src/attr.rs, decl: 6, sub: 0, line: 50 } |  |  | 0.501 |
| walker |  | 5388 | 36 | Code::CodeKey { rung: Decl, file: impl/src/attr.rs, decl: 5, sub: 0, line: 44 } |  |  | 0.507 |
| walker |  | 5424 | 36 | Code::CodeKey { rung: Decl, file: impl/src/attr.rs, decl: 4, sub: 0, line: 38 } |  |  | 0.514 |
| walker |  | 5460 | 36 | Code::CodeKey { rung: Decl, file: impl/src/attr.rs, decl: 3, sub: 0, line: 32 } |  |  | 0.524 |
| walker |  | 5550 | 90 | Code::CodeKey { rung: Decl, file: impl/src/attr.rs, decl: 1, sub: 0, line: 11 } |  |  | 0.532 |
| ns | 5595 |  | 290 | impl/src/fmt.rs: expand_shorthand and the format-spec → Trait mapping | 3.14 |  | 0.518 |
| walker |  | 5642 | 92 | Code::CodeKey { rung: Decl, file: impl/src/attr.rs, decl: 7, sub: 0, line: 56 } |  |  | 0.548 |
| ns | 5726 |  | 131 | fmt.rs: FmtArguments and the remaining function roster | 3.15 | 3.14 | 0.541 |
| walker |  | 5754 | 112 | Code::CodeKey { rung: Decl, file: impl/src/attr.rs, decl: 2, sub: 0, line: 20 } |  |  | 0.566 |
| walker |  | 5839 | 85 | Code::CodeKey { rung: Names, file: impl/src/generics.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.566 |
| walker |  | 5855 | 16 | Code::CodeKey { rung: Decl, file: impl/src/generics.rs, decl: 1, sub: 0, line: 8 } |  |  | 0.566 |
| walker |  | 5894 | 39 | Code::CodeKey { rung: Decl, file: impl/src/generics.rs, decl: 6, sub: 0, line: 48 } |  |  | 0.566 |
| ns | 5905 |  | 179 | ast.rs: ContainerKind and its six display strings | 3.16 |  | 0.558 |
| walker |  | 5940 | 46 | Code::CodeKey { rung: Decl, file: impl/src/generics.rs, decl: 2, sub: 0, line: 12 } |  |  | 0.559 |
| walker |  | 6010 | 70 | Code::CodeKey { rung: Decl, file: impl/src/generics.rs, decl: 7, sub: 0, line: 53 } |  |  | 0.559 |
| walker |  | 6084 | 74 | Code::CodeKey { rung: Names, file: impl/src/valid.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.561 |
| walker |  | 6108 | 24 | Code::CodeKey { rung: Decl, file: impl/src/valid.rs, decl: 1, sub: 0, line: 5 } |  |  | 0.562 |
| ns | 6198 |  | 293 | ast.rs: complete from_syn constructor roster | 3.17 |  | 0.554 |
| walker |  | 6321 | 213 | Code::CodeKey { rung: Names, file: impl/src/expand.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.572 |
| ns | 6357 |  | 159 | impl/src/generics.rs: ParamsInScope and InferredBounds | 3.18 |  | 0.581 |
| walker |  | 6365 | 44 | Code::CodeKey { rung: Decl, file: impl/src/expand.rs, decl: 8, sub: 0, line: 536 } |  |  | 0.581 |
| walker |  | 6380 | 15 | Code::CodeKey { rung: Body, file: impl/src/expand.rs, decl: 9, sub: 0, line: 565 } |  |  | 0.581 |
| walker |  | 6581 | 201 | Code::CodeKey { rung: ModuleDoc, file: src/lib.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.581 |
| ns | 6606 |  | 249 | impl/src/unraw.rs: IdentUnraw/MemberUnraw and the raw-identifier rule | 3.19 |  | 0.572 |
| walker |  | 6769 | 188 | Code::CodeKey { rung: Names, file: impl/src/fmt.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.580 |
| walker |  | 6783 | 14 | Code::CodeKey { rung: Decl, file: impl/src/fmt.rs, decl: 4, sub: 0, line: 171 } |  |  | 0.580 |
| walker |  | 6815 | 32 | Code::CodeKey { rung: Decl, file: impl/src/fmt.rs, decl: 3, sub: 0, line: 166 } |  |  | 0.588 |
| walker |  | 6848 | 33 | Code::CodeKey { rung: Decl, file: impl/src/fmt.rs, decl: 1, sub: 0, line: 15 } |  |  | 0.588 |
| walker |  | 6880 | 32 | Code::CodeKey { rung: Body, file: impl/src/expand.rs, decl: 10, sub: 0, line: 569 } |  |  | 0.588 |
| ns | 6910 |  | 304 | impl/src/fallback.rs: the invalid-input fallback expansion | 3.20 |  | 0.574 |
| walker |  | 7073 | 193 | Code::CodeKey { rung: Names, file: impl/src/prop.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.583 |
| walker |  | 7115 | 42 | Code::CodeKey { rung: Decl, file: impl/src/prop.rs, decl: 15, sub: 0, line: 72 } |  |  | 0.585 |
| ns | 7121 |  | 211 | impl/src/scan_expr.rs: the Input/Action alphabet of the expression scanner | 3.21 |  | 0.571 |
| walker |  | 7157 | 42 | Code::CodeKey { rung: Decl, file: impl/src/prop.rs, decl: 21, sub: 0, line: 127 } |  |  | 0.571 |
| walker |  | 7218 | 61 | Code::CodeKey { rung: Decl, file: impl/src/prop.rs, decl: 6, sub: 0, line: 25 } |  |  | 0.575 |
| walker |  | 7312 | 94 | Code::CodeKey { rung: Decl, file: impl/src/prop.rs, decl: 1, sub: 0, line: 6 } |  |  | 0.582 |
| walker |  | 7406 | 94 | Code::CodeKey { rung: Decl, file: impl/src/prop.rs, decl: 10, sub: 0, line: 53 } |  |  | 0.589 |
| ns | 7477 |  | 356 | expand.rs: the gate line for every conditionally generated impl | 3.22 | 3.1 | 0.578 |
| walker |  | 7600 | 194 | Code::CodeKey { rung: Names, file: impl/src/scan_expr.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.578 |
| walker |  | 7632 | 32 | Code::CodeKey { rung: Decl, file: impl/src/scan_expr.rs, decl: 7, sub: 0, line: 84 } |  |  | 0.578 |
| ns | 7637 |  | 160 | src/private.rs: the complete generated-code support surface | 4.1 |  | 0.570 |
| walker |  | 7672 | 40 | Code::CodeKey { rung: Decl, file: impl/src/scan_expr.rs, decl: 10, sub: 0, line: 104 } |  |  | 0.570 |
| walker |  | 7714 | 42 | Code::CodeKey { rung: Decl, file: impl/src/scan_expr.rs, decl: 2, sub: 0, line: 25 } |  |  | 0.572 |
| walker |  | 7763 | 49 | Code::CodeKey { rung: Decl, file: impl/src/scan_expr.rs, decl: 8, sub: 0, line: 89 } |  |  | 0.572 |
| walker |  | 7815 | 52 | Code::CodeKey { rung: Decl, file: impl/src/scan_expr.rs, decl: 5, sub: 0, line: 76 } |  |  | 0.572 |
| ns | 7852 |  | 215 | src/aserror.rs: AsDynError and its five blanket/dyn impls | 4.2 |  | 0.571 |
| walker |  | 7957 | 142 | Code::CodeKey { rung: Decl, file: impl/src/scan_expr.rs, decl: 1, sub: 0, line: 6 } |  |  | 0.597 |
| walker |  | 8062 | 105 | Code::CodeKey { rung: Decl, file: impl/src/scan_expr.rs, decl: 9, sub: 0, line: 95 } |  |  | 0.597 |
| ns | 8094 |  | 242 | src/display.rs: AsDisplay and the std-only Path/PathBuf specializations | 4.3 |  | 0.592 |
| walker |  | 8227 | 165 | Code::CodeKey { rung: Decl, file: impl/src/scan_expr.rs, decl: 4, sub: 0, line: 63 } |  |  | 0.592 |
| ns | 8376 |  | 282 | src/provide.rs (ThiserrorProvide) and src/var.rs (Var) end to end | 4.4 |  | 0.588 |
| ns | 8489 |  | 113 | impl/src/lib.rs: the version-stamped `private` path token | 4.5 |  | 0.592 |
| walker |  | 8678 | 451 | Code::CodeKey { rung: Decl, file: impl/src/scan_expr.rs, decl: 3, sub: 0, line: 32 } |  |  | 0.592 |
| ns | 8738 |  | 249 | build.rs: the generated __private module and the cfg declarations | 4.6 |  | 0.585 |
| walker |  | 8862 | 184 | Code::CodeKey { rung: Names, file: impl/src/scan_expr.rs, decl: 0, sub: 1, line: 0 } |  |  | 0.585 |
| walker |  | 8883 | 21 | Code::CodeKey { rung: Decl, file: impl/src/scan_expr.rs, decl: 17, sub: 0, line: 136 } |  |  | 0.585 |
| walker |  | 8915 | 32 | Code::CodeKey { rung: Decl, file: impl/src/scan_expr.rs, decl: 13, sub: 0, line: 119 } |  |  | 0.585 |
| ns | 8946 |  | 208 | build.rs: rustc capability decisions, function roster, and the probe file | 4.7 |  | 0.580 |
| walker |  | 8949 | 34 | Code::CodeKey { rung: Decl, file: impl/src/scan_expr.rs, decl: 15, sub: 0, line: 130 } |  |  | 0.580 |
| walker |  | 8985 | 36 | Code::CodeKey { rung: Decl, file: impl/src/scan_expr.rs, decl: 12, sub: 0, line: 114 } |  |  | 0.580 |
| walker |  | 9026 | 41 | Code::CodeKey { rung: Decl, file: impl/src/scan_expr.rs, decl: 11, sub: 0, line: 109 } |  |  | 0.580 |
| ns | 9043 |  | 97 | Complete tests/, tests/no-std/ and .github/ listings | 5.1 |  | 0.589 |
| walker |  | 9074 | 48 | Code::CodeKey { rung: Decl, file: impl/src/scan_expr.rs, decl: 14, sub: 0, line: 124 } |  |  | 0.589 |
| ns | 9131 |  | 88 | tests/compiletest.rs: the trybuild UI harness in full | 5.2 |  | 0.586 |
| walker |  | 9209 | 135 | Code::CodeKey { rung: Names, file: impl/src/scan_expr.rs, decl: 0, sub: 2, line: 0 } |  |  | 0.586 |
| walker |  | 9241 | 32 | Code::CodeKey { rung: Decl, file: impl/src/scan_expr.rs, decl: 24, sub: 0, line: 187 } |  |  | 0.586 |
| ns | 9262 |  | 131 | tests/no-std/test.rs: the no_std smoke test's error types | 5.3 |  | 0.580 |
| walker |  | 9287 | 46 | Code::CodeKey { rung: Decl, file: impl/src/scan_expr.rs, decl: 23, sub: 0, line: 181 } |  |  | 0.580 |
| walker |  | 9334 | 47 | Code::CodeKey { rung: Decl, file: impl/src/scan_expr.rs, decl: 22, sub: 0, line: 175 } |  |  | 0.580 |
| ns | 9389 |  | 127 | Cargo.toml: std-feature rationale and dev-dependencies | 5.4 |  | 0.578 |
| walker |  | 9400 | 66 | Code::CodeKey { rung: Decl, file: impl/src/scan_expr.rs, decl: 19, sub: 0, line: 141 } |  |  | 0.578 |
| walker |  | 9492 | 92 | Code::CodeKey { rung: Decl, file: impl/src/scan_expr.rs, decl: 21, sub: 0, line: 166 } |  |  | 0.578 |
| ns | 9515 |  | 126 | impl/Cargo.toml: the proc-macro crate's manifest | 5.5 |  | 0.582 |
| ns | 9689 |  | 174 | CI job roster and the pinned toolchain components | 5.6 |  | 0.575 |
| walker |  | 9731 | 239 | Code::CodeKey { rung: Decl, file: impl/src/scan_expr.rs, decl: 20, sub: 0, line: 148 } |  |  | 0.575 |
| walker |  | 9770 | 39 | Code::CodeKey { rung: Body, file: impl/src/expand.rs, decl: 5, sub: 0, line: 505 } |  |  | 0.575 |
| walker |  | 9781 | 11 | Code::CodeKey { rung: Body, file: impl/src/prop.rs, decl: 2, sub: 0, line: 7 } |  |  | 0.575 |
| walker |  | 9989 | 208 | Markdown::Section { file: README.md, section_index: 4, keeps_default_concavity: false } |  |  | 0.589 |
