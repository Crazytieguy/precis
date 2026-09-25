Score(3000)=0.510 I=0.814 C=0.320 ns_rows≤3K=24/56 grid(1000/1442/2080/3000/4327/6240/9000)=0.794/0.693/0.620/0.510/0.487/0.475/0.582

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| walker |  | 38 | 38 | Fs::DirListing { dir: . } |  |  | 0.000 |
| walker |  | 42 | 4 | Fs::DirListing { dir: build } |  |  | 0.000 |
| walker |  | 61 | 19 | Fs::DirListing { dir: impl } |  |  | 0.000 |
| walker |  | 86 | 25 | Fs::DirListing { dir: src } |  |  | 0.000 |
| ns | 86 |  | 86 | Crate identity: name, one-line purpose, version | 1.1 |  | 0.000 |
| ns | 124 |  | 38 | Complete repository root listing | 1.2 |  | 0.453 |
| walker |  | 133 | 47 | Markdown::ReadmeHeadline { file: README.md } |  |  | 0.635 |
| walker |  | 141 | 8 | Fs::DirListing { dir: .github } |  |  | 0.635 |
| walker |  | 145 | 4 | Fs::DirListing { dir: .github/workflows } |  |  | 0.636 |
| walker |  | 181 | 36 | Toml::Dependencies { file: Cargo.toml } |  |  | 0.637 |
| walker |  | 210 | 29 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.640 |
| ns | 215 |  | 91 | Complete listings of both source trees: src/, impl/, impl/src/ | 1.3 |  | 0.461 |
| walker |  | 257 | 47 | Fs::DirListing { dir: impl/src } |  |  | 0.708 |
| ns | 313 |  | 98 | README canonical example, head: derive, #[from], positional {0} | 1.4 |  | 0.617 |
| ns | 398 |  | 85 | README canonical example, tail: named-field variant, unit variant | 1.5 | 1.4 | 0.558 |
| walker |  | 442 | 185 | Toml::Identity { file: Cargo.toml } |  |  | 0.748 |
| ns | 476 |  | 78 | Derive entry point: #[proc_macro_derive(Error, attributes(...))] | 1.6 |  | 0.712 |
| walker |  | 514 | 72 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.757 |
| walker |  | 585 | 71 | Code::CodeKey { rung: Names, file: src/lib.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.762 |
| ns | 612 |  | 136 | Root crate module structure and public re-export (src/lib.rs tail) | 1.7 |  | 0.700 |
| ns | 707 |  | 95 | Root crate attributes: no_std, docs.rs root, nightly cfg gate | 1.8 |  | 0.665 |
| walker |  | 781 | 196 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: true } |  |  | 0.830 |
| ns | 814 |  | 107 | Cargo manifest: features, dependency on impl, workspace members | 1.9 |  | 0.812 |
| walker |  | 864 | 83 | Code::CodeKey { rung: Names, file: build.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.812 |
| ns | 870 |  | 56 | Documentation map: README section headings + rustdoc mirror in src/lib.rs | 2.1 |  | 0.792 |
| walker |  | 910 | 46 | Code::CodeKey { rung: Decl, file: build.rs, decl: 1, sub: 0, line: 10 } |  |  | 0.792 |
| walker |  | 982 | 72 | Fs::DirListing { dir: tests } |  |  | 0.794 |
| ns | 1036 |  | 166 | Details: no public API footprint, allowed error shapes, Display generation | 2.2 |  | 0.747 |
| walker |  | 1146 | 164 | Toml::Operational { file: Cargo.toml } |  |  | 0.773 |
| ns | 1222 |  | 186 | Display shorthand table: {var}, {0}, {var:?}, {0:?} | 2.3 |  | 0.739 |
| walker |  | 1269 | 123 | Toml::Config { file: Cargo.toml } |  |  | 0.739 |
| walker |  | 1322 | 53 | Code::CodeKey { rung: Names, file: src/var.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.739 |
| walker |  | 1350 | 28 | Code::CodeKey { rung: Decl, file: src/var.rs, decl: 2, sub: 0, line: 5 } |  |  | 0.739 |
| walker |  | 1364 | 14 | Code::CodeKey { rung: Body, file: src/var.rs, decl: 3, sub: 0, line: 6 } |  |  | 0.739 |
| ns | 1404 |  | 182 | Details: #[from] generates From, with its field-count restriction | 2.4 |  | 0.693 |
| walker |  | 1417 | 53 | Toml::Dependencies { file: impl/Cargo.toml } |  |  | 0.693 |
| walker |  | 1537 | 120 | Code::CodeKey { rung: Names, file: impl/src/lib.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.694 |
| ns | 1557 |  | 153 | Details: source() from #[source] or a field named `source` | 2.5 |  | 0.669 |
| walker |  | 1558 | 21 | Code::CodeKey { rung: Decl, file: impl/src/lib.rs, decl: 1, sub: 0, line: 39 } |  |  | 0.674 |
| walker |  | 1570 | 12 | Code::CodeKey { rung: Decl, file: impl/src/lib.rs, decl: 2, sub: 0, line: 45 } |  |  | 0.674 |
| walker |  | 1598 | 28 | Code::CodeKey { rung: Decl, file: impl/src/lib.rs, decl: 3, sub: 0, line: 48 } |  |  | 0.674 |
| walker |  | 1629 | 31 | Code::CodeKey { rung: Body, file: impl/src/lib.rs, decl: 1, sub: 0, line: 39 } |  |  | 0.690 |
| ns | 1709 |  | 152 | Details: #[error(transparent)] forwarding | 2.6 |  | 0.656 |
| walker |  | 1710 | 81 | Code::CodeKey { rung: Names, file: build/probe.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.656 |
| walker |  | 1739 | 29 | Code::CodeKey { rung: Decl, file: build/probe.rs, decl: 3, sub: 0, line: 14 } |  |  | 0.656 |
| walker |  | 1768 | 29 | Code::CodeKey { rung: Decl, file: build/probe.rs, decl: 5, sub: 0, line: 20 } |  |  | 0.656 |
| walker |  | 1798 | 30 | Code::CodeKey { rung: Decl, file: build/probe.rs, decl: 7, sub: 0, line: 26 } |  |  | 0.656 |
| walker |  | 1807 | 9 | Code::CodeKey { rung: Body, file: build/probe.rs, decl: 4, sub: 0, line: 15 } |  |  | 0.656 |
| ns | 1839 |  | 130 | Details: provide() and automatic Backtrace field detection | 2.7 |  | 0.635 |
| walker |  | 1862 | 55 | Code::CodeKey { rung: Body, file: build.rs, decl: 5, sub: 0, line: 190 } |  |  | 0.635 |
| walker |  | 1892 | 30 | Code::CodeKey { rung: Names, file: impl/src/fallback.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.635 |
| walker |  | 1901 | 9 | Code::CodeKey { rung: Body, file: build/probe.rs, decl: 6, sub: 0, line: 21 } |  |  | 0.635 |
| ns | 1960 |  | 121 | Details: #[backtrace] on a source field forwards provide() | 2.8 |  | 0.624 |
| ns | 2005 |  | 45 | Details: #[from] variants with a Backtrace field capture in From | 2.9 |  | 0.620 |
| ns | 2088 |  | 83 | Details example: extra format arguments (`max = i32::MAX`) | 2.10 |  | 0.605 |
| walker |  | 2175 | 274 | Code::CodeKey { rung: ModuleDoc, file: src/lib.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.611 |
| walker |  | 2184 | 9 | Fs::DirListing { dir: tests/no-std } |  |  | 0.611 |
| ns | 2254 |  | 166 | Details example: referring to fields from format args via `.var` / `.0` | 2.11 |  | 0.589 |
| ns | 2410 |  | 156 | When to use thiserror vs anyhow | 2.12 |  | 0.573 |
| ns | 2525 |  | 115 | impl/src/expand.rs: complete top-level function roster (names only) | 3.1 |  | 0.556 |
| walker |  | 2712 | 528 | Fs::DirListing { dir: tests/ui } |  |  | 0.556 |
| ns | 2723 |  | 198 | expand.rs: derive/try_expand — the whole expansion pipeline in 18 lines | 3.2 | 3.1 | 0.530 |
| walker |  | 2950 | 238 | Code::CodeKey { rung: ModuleDoc, file: src/lib.rs, decl: 0, sub: 1, line: 0 } |  |  | 0.510 |
| ns | 2950 |  | 227 | expand.rs: the two emitted impl shapes (struct and enum quote! tails) | 3.3 |  | 0.510 |
| walker |  | 3072 | 122 | Toml::Identity { file: impl/Cargo.toml } |  |  | 0.511 |
| walker |  | 3231 | 159 | Toml::Config { file: impl/Cargo.toml } |  |  | 0.511 |
| ns | 3290 |  | 340 | impl/src/ast.rs: the complete IR (Input, Struct, Enum, Variant, Field) | 3.4 |  | 0.476 |
| walker |  | 3425 | 194 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.513 |
| ns | 3528 |  | 238 | impl/src/attr.rs: Attrs and the parsed Display attribute | 3.5 |  | 0.493 |
| walker |  | 3676 | 251 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: true } |  |  | 0.537 |
| walker |  | 3793 | 117 | Code::CodeKey { rung: Body, file: build.rs, decl: 4, sub: 0, line: 179 } |  |  | 0.537 |
| ns | 3847 |  | 319 | attr.rs: Source/From/Transparent/Fmt payloads and the Trait enum | 3.6 |  | 0.503 |
| ns | 4040 |  | 193 | attr.rs: the three accepted forms of #[error(...)] | 3.7 |  | 0.492 |
| walker |  | 4066 | 273 | Code::CodeKey { rung: ModuleDoc, file: src/lib.rs, decl: 0, sub: 2, line: 0 } |  |  | 0.492 |
| walker |  | 4080 | 14 | Code::CodeKey { rung: Body, file: build/probe.rs, decl: 8, sub: 0, line: 27 } |  |  | 0.492 |
| walker |  | 4154 | 74 | Code::CodeKey { rung: Names, file: impl/src/valid.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.492 |
| walker |  | 4178 | 24 | Code::CodeKey { rung: Decl, file: impl/src/valid.rs, decl: 1, sub: 0, line: 5 } |  |  | 0.492 |
| ns | 4186 |  | 146 | impl/src/valid.rs: container-level diagnostic messages | 3.8 |  | 0.486 |
| walker |  | 4263 | 85 | Code::CodeKey { rung: Names, file: impl/src/generics.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.486 |
| walker |  | 4279 | 16 | Code::CodeKey { rung: Decl, file: impl/src/generics.rs, decl: 1, sub: 0, line: 8 } |  |  | 0.486 |
| walker |  | 4318 | 39 | Code::CodeKey { rung: Decl, file: impl/src/generics.rs, decl: 6, sub: 0, line: 48 } |  |  | 0.487 |
| walker |  | 4364 | 46 | Code::CodeKey { rung: Decl, file: impl/src/generics.rs, decl: 2, sub: 0, line: 12 } |  |  | 0.487 |
| walker |  | 4434 | 70 | Code::CodeKey { rung: Decl, file: impl/src/generics.rs, decl: 7, sub: 0, line: 53 } |  |  | 0.487 |
| ns | 4478 |  | 292 | valid.rs: attribute-placement and field-attribute diagnostic messages | 3.9 |  | 0.478 |
| walker |  | 4481 | 47 | Code::CodeKey { rung: Body, file: impl/src/lib.rs, decl: 4, sub: 0, line: 49 } |  |  | 0.478 |
| ns | 4647 |  | 169 | valid.rs: complete validate/check function roster | 3.10 |  | 0.473 |
| walker |  | 4786 | 305 | Markdown::Section { file: README.md, section_index: 7, keeps_default_concavity: false } |  |  | 0.486 |
| ns | 4859 |  | 212 | attr.rs: which attributes `get` recognises, plus the file's function roster | 3.11 |  | 0.475 |
| walker |  | 4891 | 105 | Code::CodeKey { rung: Names, file: impl/src/ast.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.477 |
| walker |  | 4917 | 26 | Code::CodeKey { rung: Decl, file: impl/src/ast.rs, decl: 1, sub: 0, line: 10 } |  |  | 0.479 |
| walker |  | 4945 | 28 | Code::CodeKey { rung: Decl, file: impl/src/ast.rs, decl: 9, sub: 0, line: 174 } |  |  | 0.479 |
| walker |  | 4974 | 29 | Code::CodeKey { rung: Decl, file: impl/src/ast.rs, decl: 7, sub: 0, line: 54 } |  |  | 0.479 |
| walker |  | 5027 | 53 | Code::CodeKey { rung: Decl, file: impl/src/ast.rs, decl: 2, sub: 0, line: 15 } |  |  | 0.487 |
| walker |  | 5080 | 53 | Code::CodeKey { rung: Decl, file: impl/src/ast.rs, decl: 4, sub: 0, line: 29 } |  |  | 0.499 |
| ns | 5096 |  | 237 | impl/src/prop.rs: complete accessor roster for source/from/backtrace fields | 3.12 |  | 0.484 |
| walker |  | 5134 | 54 | Code::CodeKey { rung: Decl, file: impl/src/ast.rs, decl: 3, sub: 0, line: 22 } |  |  | 0.498 |
| walker |  | 5195 | 61 | Code::CodeKey { rung: Decl, file: impl/src/ast.rs, decl: 6, sub: 0, line: 44 } |  |  | 0.499 |
| walker |  | 5260 | 65 | Code::CodeKey { rung: Decl, file: impl/src/ast.rs, decl: 5, sub: 0, line: 36 } |  |  | 0.521 |
| ns | 5305 |  | 209 | prop.rs: how the source and backtrace fields are actually chosen | 3.13 |  | 0.507 |
| walker |  | 5495 | 235 | Code::CodeKey { rung: Names, file: impl/src/scan_expr.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.507 |
| walker |  | 5527 | 32 | Code::CodeKey { rung: Decl, file: impl/src/scan_expr.rs, decl: 7, sub: 0, line: 84 } |  |  | 0.507 |
| walker |  | 5565 | 38 | Code::CodeKey { rung: Decl, file: impl/src/scan_expr.rs, decl: 10, sub: 0, line: 104 } |  |  | 0.507 |
| ns | 5595 |  | 290 | impl/src/fmt.rs: expand_shorthand and the format-spec → Trait mapping | 3.14 |  | 0.494 |
| walker |  | 5603 | 38 | Code::CodeKey { rung: Decl, file: impl/src/scan_expr.rs, decl: 12, sub: 0, line: 114 } |  |  | 0.494 |
| walker |  | 5644 | 41 | Code::CodeKey { rung: Decl, file: impl/src/scan_expr.rs, decl: 11, sub: 0, line: 109 } |  |  | 0.494 |
| walker |  | 5686 | 42 | Code::CodeKey { rung: Decl, file: impl/src/scan_expr.rs, decl: 2, sub: 0, line: 25 } |  |  | 0.494 |
| ns | 5726 |  | 131 | fmt.rs: FmtArguments and the remaining function roster | 3.15 | 3.14 | 0.487 |
| walker |  | 5735 | 49 | Code::CodeKey { rung: Decl, file: impl/src/scan_expr.rs, decl: 8, sub: 0, line: 89 } |  |  | 0.487 |
| walker |  | 5787 | 52 | Code::CodeKey { rung: Decl, file: impl/src/scan_expr.rs, decl: 5, sub: 0, line: 76 } |  |  | 0.487 |
| walker |  | 5892 | 105 | Code::CodeKey { rung: Decl, file: impl/src/scan_expr.rs, decl: 9, sub: 0, line: 95 } |  |  | 0.487 |
| ns | 5905 |  | 179 | ast.rs: ContainerKind and its six display strings | 3.16 |  | 0.482 |
| walker |  | 6125 | 233 | Code::CodeKey { rung: ModuleDoc, file: src/lib.rs, decl: 0, sub: 3, line: 0 } |  |  | 0.482 |
| ns | 6198 |  | 293 | ast.rs: complete from_syn constructor roster | 3.17 |  | 0.475 |
| walker |  | 6267 | 142 | Code::CodeKey { rung: Decl, file: impl/src/scan_expr.rs, decl: 1, sub: 0, line: 6 } |  |  | 0.476 |
| ns | 6357 |  | 159 | impl/src/generics.rs: ParamsInScope and InferredBounds | 3.18 |  | 0.489 |
| walker |  | 6521 | 254 | Code::CodeKey { rung: Names, file: impl/src/unraw.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.489 |
| walker |  | 6538 | 17 | Code::CodeKey { rung: Decl, file: impl/src/unraw.rs, decl: 1, sub: 0, line: 10 } |  |  | 0.489 |
| walker |  | 6557 | 19 | Code::CodeKey { rung: Decl, file: impl/src/unraw.rs, decl: 22, sub: 0, line: 87 } |  |  | 0.489 |
| walker |  | 6579 | 22 | Code::CodeKey { rung: Decl, file: impl/src/unraw.rs, decl: 17, sub: 0, line: 69 } |  |  | 0.489 |
| walker |  | 6602 | 23 | Code::CodeKey { rung: Decl, file: impl/src/unraw.rs, decl: 9, sub: 0, line: 45 } |  |  | 0.489 |
| ns | 6606 |  | 249 | impl/src/unraw.rs: IdentUnraw/MemberUnraw and the raw-identifier rule | 3.19 |  | 0.480 |
| walker |  | 6625 | 23 | Code::CodeKey { rung: Decl, file: impl/src/unraw.rs, decl: 11, sub: 0, line: 51 } |  |  | 0.480 |
| walker |  | 6648 | 23 | Code::CodeKey { rung: Decl, file: impl/src/unraw.rs, decl: 13, sub: 0, line: 57 } |  |  | 0.480 |
| walker |  | 6671 | 23 | Code::CodeKey { rung: Decl, file: impl/src/unraw.rs, decl: 27, sub: 0, line: 107 } |  |  | 0.480 |
| walker |  | 6694 | 23 | Code::CodeKey { rung: Decl, file: impl/src/unraw.rs, decl: 29, sub: 0, line: 117 } |  |  | 0.480 |
| walker |  | 6718 | 24 | Code::CodeKey { rung: Decl, file: impl/src/unraw.rs, decl: 33, sub: 0, line: 135 } |  |  | 0.480 |
| walker |  | 6744 | 26 | Code::CodeKey { rung: Decl, file: impl/src/unraw.rs, decl: 19, sub: 0, line: 75 } |  |  | 0.480 |
| walker |  | 6771 | 27 | Code::CodeKey { rung: Decl, file: impl/src/unraw.rs, decl: 15, sub: 0, line: 63 } |  |  | 0.480 |
| walker |  | 6798 | 27 | Code::CodeKey { rung: Decl, file: impl/src/unraw.rs, decl: 31, sub: 0, line: 126 } |  |  | 0.480 |
| walker |  | 6826 | 28 | Code::CodeKey { rung: Decl, file: impl/src/unraw.rs, decl: 6, sub: 0, line: 37 } |  |  | 0.480 |
| walker |  | 6854 | 28 | Code::CodeKey { rung: Decl, file: impl/src/unraw.rs, decl: 24, sub: 0, line: 96 } |  |  | 0.480 |
| walker |  | 6883 | 29 | Code::CodeKey { rung: Decl, file: impl/src/unraw.rs, decl: 21, sub: 0, line: 81 } |  |  | 0.482 |
| ns | 6910 |  | 304 | impl/src/fallback.rs: the invalid-input fallback expansion | 3.20 |  | 0.471 |
| walker |  | 6941 | 58 | Code::CodeKey { rung: Decl, file: impl/src/unraw.rs, decl: 2, sub: 0, line: 14 } |  |  | 0.472 |
| ns | 7121 |  | 211 | impl/src/scan_expr.rs: the Input/Action alphabet of the expression scanner | 3.21 |  | 0.494 |
| walker |  | 7406 | 465 | Markdown::Section { file: README.md, section_index: 4, keeps_default_concavity: false } |  |  | 0.543 |
| walker |  | 7438 | 32 | Code::CodeKey { rung: Body, file: impl/src/generics.rs, decl: 4, sub: 0, line: 19 } |  |  | 0.543 |
| ns | 7477 |  | 356 | expand.rs: the gate line for every conditionally generated impl | 3.22 | 3.1 | 0.532 |
| walker |  | 7603 | 165 | Code::CodeKey { rung: Decl, file: impl/src/scan_expr.rs, decl: 4, sub: 0, line: 63 } |  |  | 0.532 |
| ns | 7637 |  | 160 | src/private.rs: the complete generated-code support surface | 4.1 |  | 0.525 |
| ns | 7852 |  | 215 | src/aserror.rs: AsDynError and its five blanket/dyn impls | 4.2 |  | 0.519 |
| ns | 8094 |  | 242 | src/display.rs: AsDisplay and the std-only Path/PathBuf specializations | 4.3 |  | 0.510 |
| walker |  | 8111 | 508 | Markdown::Section { file: README.md, section_index: 5, keeps_default_concavity: false } |  |  | 0.536 |
| walker |  | 8309 | 198 | Code::CodeKey { rung: Names, file: impl/src/attr.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.538 |
| walker |  | 8333 | 24 | Code::CodeKey { rung: Decl, file: impl/src/attr.rs, decl: 11, sub: 0, line: 302 } |  |  | 0.538 |
| walker |  | 8357 | 24 | Code::CodeKey { rung: Decl, file: impl/src/attr.rs, decl: 13, sub: 0, line: 342 } |  |  | 0.538 |
| ns | 8376 |  | 282 | src/provide.rs (ThiserrorProvide) and src/var.rs (Var) end to end | 4.4 |  | 0.529 |
| walker |  | 8393 | 36 | Code::CodeKey { rung: Decl, file: impl/src/attr.rs, decl: 6, sub: 0, line: 50 } |  |  | 0.531 |
| walker |  | 8429 | 36 | Code::CodeKey { rung: Decl, file: impl/src/attr.rs, decl: 5, sub: 0, line: 44 } |  |  | 0.535 |
| walker |  | 8465 | 36 | Code::CodeKey { rung: Decl, file: impl/src/attr.rs, decl: 4, sub: 0, line: 38 } |  |  | 0.540 |
| ns | 8489 |  | 113 | impl/src/lib.rs: the version-stamped `private` path token | 4.5 |  | 0.546 |
| walker |  | 8501 | 36 | Code::CodeKey { rung: Decl, file: impl/src/attr.rs, decl: 3, sub: 0, line: 32 } |  |  | 0.552 |
| walker |  | 8591 | 90 | Code::CodeKey { rung: Decl, file: impl/src/attr.rs, decl: 1, sub: 0, line: 11 } |  |  | 0.557 |
| walker |  | 8683 | 92 | Code::CodeKey { rung: Decl, file: impl/src/attr.rs, decl: 7, sub: 0, line: 56 } |  |  | 0.577 |
| ns | 8738 |  | 249 | build.rs: the generated __private module and the cfg declarations | 4.6 |  | 0.570 |
| walker |  | 8795 | 112 | Code::CodeKey { rung: Decl, file: impl/src/attr.rs, decl: 2, sub: 0, line: 20 } |  |  | 0.587 |
| ns | 8946 |  | 208 | build.rs: rustc capability decisions, function roster, and the probe file | 4.7 |  | 0.582 |
| walker |  | 9041 | 246 | Code::CodeKey { rung: ModuleDoc, file: src/lib.rs, decl: 0, sub: 4, line: 0 } |  |  | 0.582 |
| ns | 9043 |  | 97 | Complete tests/, tests/no-std/ and .github/ listings | 5.1 |  | 0.592 |
| ns | 9131 |  | 88 | tests/compiletest.rs: the trybuild UI harness in full | 5.2 |  | 0.589 |
| walker |  | 9229 | 188 | Code::CodeKey { rung: Names, file: impl/src/fmt.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.594 |
| walker |  | 9262 | 33 | Code::CodeKey { rung: Decl, file: impl/src/fmt.rs, decl: 1, sub: 0, line: 15 } |  |  | 0.588 |
| ns | 9262 |  | 131 | tests/no-std/test.rs: the no_std smoke test's error types | 5.3 |  | 0.588 |
| walker |  | 9276 | 14 | Code::CodeKey { rung: Decl, file: impl/src/fmt.rs, decl: 4, sub: 0, line: 171 } |  |  | 0.588 |
| walker |  | 9308 | 32 | Code::CodeKey { rung: Decl, file: impl/src/fmt.rs, decl: 3, sub: 0, line: 166 } |  |  | 0.593 |
| ns | 9389 |  | 127 | Cargo.toml: std-feature rationale and dev-dependencies | 5.4 |  | 0.592 |
| walker |  | 9501 | 193 | Code::CodeKey { rung: Names, file: impl/src/prop.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.598 |
| ns | 9515 |  | 126 | impl/Cargo.toml: the proc-macro crate's manifest | 5.5 |  | 0.602 |
| walker |  | 9543 | 42 | Code::CodeKey { rung: Decl, file: impl/src/prop.rs, decl: 15, sub: 0, line: 72 } |  |  | 0.603 |
| walker |  | 9585 | 42 | Code::CodeKey { rung: Decl, file: impl/src/prop.rs, decl: 21, sub: 0, line: 127 } |  |  | 0.603 |
| walker |  | 9646 | 61 | Code::CodeKey { rung: Decl, file: impl/src/prop.rs, decl: 6, sub: 0, line: 25 } |  |  | 0.607 |
| ns | 9689 |  | 174 | CI job roster and the pinned toolchain components | 5.6 |  | 0.599 |
| walker |  | 9740 | 94 | Code::CodeKey { rung: Decl, file: impl/src/prop.rs, decl: 1, sub: 0, line: 6 } |  |  | 0.604 |
| walker |  | 9834 | 94 | Code::CodeKey { rung: Decl, file: impl/src/prop.rs, decl: 10, sub: 0, line: 53 } |  |  | 0.610 |
