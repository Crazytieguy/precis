Score(3000)=0.502 I=0.814 C=0.310 ns_rows≤3K=24/56 grid(1000/1442/2080/3000/4327/6240/9000)=0.793/0.717/0.621/0.502/0.482/0.540/0.561

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
| walker |  | 174 | 29 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.638 |
| ns | 215 |  | 91 | Complete listings of both source trees: src/, impl/, impl/src/ | 1.3 |  | 0.460 |
| walker |  | 221 | 47 | Fs::DirListing { dir: impl/src } |  |  | 0.707 |
| walker |  | 257 | 36 | Toml::Dependencies { file: Cargo.toml } |  |  | 0.708 |
| ns | 313 |  | 98 | README canonical example, head: derive, #[from], positional {0} | 1.4 |  | 0.617 |
| ns | 398 |  | 85 | README canonical example, tail: named-field variant, unit variant | 1.5 | 1.4 | 0.558 |
| walker |  | 442 | 185 | Toml::Identity { file: Cargo.toml } |  |  | 0.748 |
| ns | 476 |  | 78 | Derive entry point: #[proc_macro_derive(Error, attributes(...))] | 1.6 |  | 0.712 |
| walker |  | 514 | 72 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.757 |
| walker |  | 535 | 21 | Toml::Operational { file: impl/Cargo.toml } |  |  | 0.757 |
| ns | 612 |  | 136 | Root crate module structure and public re-export (src/lib.rs tail) | 1.7 |  | 0.662 |
| ns | 707 |  | 95 | Root crate attributes: no_std, docs.rs root, nightly cfg gate | 1.8 |  | 0.629 |
| walker |  | 731 | 196 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: true } |  |  | 0.797 |
| ns | 814 |  | 107 | Cargo manifest: features, dependency on impl, workspace members | 1.9 |  | 0.782 |
| walker |  | 851 | 120 | Code::CodeKey { rung: Names, file: impl/src/lib.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.784 |
| walker |  | 863 | 12 | Code::CodeKey { rung: Decl, file: impl/src/lib.rs, decl: 2, sub: 0, line: 45 } |  |  | 0.784 |
| ns | 870 |  | 56 | Documentation map: README section headings + rustdoc mirror in src/lib.rs | 2.1 |  | 0.766 |
| walker |  | 884 | 21 | Code::CodeKey { rung: Decl, file: impl/src/lib.rs, decl: 1, sub: 0, line: 39 } |  |  | 0.772 |
| walker |  | 912 | 28 | Code::CodeKey { rung: Decl, file: impl/src/lib.rs, decl: 3, sub: 0, line: 48 } |  |  | 0.773 |
| walker |  | 943 | 31 | Code::CodeKey { rung: Body, file: impl/src/lib.rs, decl: 1, sub: 0, line: 39 } |  |  | 0.792 |
| walker |  | 1015 | 72 | Fs::DirListing { dir: tests } |  |  | 0.794 |
| ns | 1036 |  | 166 | Details: no public API footprint, allowed error shapes, Display generation | 2.2 |  | 0.747 |
| walker |  | 1086 | 71 | Code::CodeKey { rung: Names, file: src/lib.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.774 |
| ns | 1222 |  | 186 | Display shorthand table: {var}, {0}, {var:?}, {0:?} | 2.3 |  | 0.739 |
| walker |  | 1250 | 164 | Toml::Operational { file: Cargo.toml } |  |  | 0.763 |
| walker |  | 1297 | 47 | Code::CodeKey { rung: Body, file: impl/src/lib.rs, decl: 4, sub: 0, line: 49 } |  |  | 0.764 |
| walker |  | 1380 | 83 | Code::CodeKey { rung: Names, file: build.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.764 |
| ns | 1404 |  | 182 | Details: #[from] generates From, with its field-count restriction | 2.4 |  | 0.717 |
| walker |  | 1426 | 46 | Code::CodeKey { rung: Decl, file: build.rs, decl: 1, sub: 0, line: 10 } |  |  | 0.717 |
| walker |  | 1481 | 55 | Code::CodeKey { rung: Body, file: build.rs, decl: 5, sub: 0, line: 190 } |  |  | 0.717 |
| ns | 1557 |  | 153 | Details: source() from #[source] or a field named `source` | 2.5 |  | 0.690 |
| walker |  | 1598 | 117 | Code::CodeKey { rung: Body, file: build.rs, decl: 4, sub: 0, line: 179 } |  |  | 0.690 |
| walker |  | 1647 | 49 | Toml::Dependencies { file: impl/Cargo.toml } |  |  | 0.690 |
| ns | 1709 |  | 152 | Details: #[error(transparent)] forwarding | 2.6 |  | 0.656 |
| walker |  | 1728 | 81 | Code::CodeKey { rung: Names, file: build/probe.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.656 |
| walker |  | 1757 | 29 | Code::CodeKey { rung: Decl, file: build/probe.rs, decl: 3, sub: 0, line: 14 } |  |  | 0.656 |
| walker |  | 1786 | 29 | Code::CodeKey { rung: Decl, file: build/probe.rs, decl: 5, sub: 0, line: 20 } |  |  | 0.656 |
| walker |  | 1816 | 30 | Code::CodeKey { rung: Decl, file: build/probe.rs, decl: 7, sub: 0, line: 26 } |  |  | 0.656 |
| walker |  | 1825 | 9 | Code::CodeKey { rung: Body, file: build/probe.rs, decl: 4, sub: 0, line: 15 } |  |  | 0.656 |
| walker |  | 1834 | 9 | Code::CodeKey { rung: Body, file: build/probe.rs, decl: 6, sub: 0, line: 21 } |  |  | 0.656 |
| ns | 1839 |  | 130 | Details: provide() and automatic Backtrace field detection | 2.7 |  | 0.635 |
| walker |  | 1848 | 14 | Code::CodeKey { rung: Body, file: build/probe.rs, decl: 8, sub: 0, line: 27 } |  |  | 0.635 |
| walker |  | 1882 | 34 | Code::CodeKey { rung: Names, file: src/provide.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.635 |
| walker |  | 1940 | 58 | Code::CodeKey { rung: Decl, file: src/provide.rs, decl: 1, sub: 0, line: 8 } |  |  | 0.635 |
| walker |  | 1946 | 6 | Code::CodeKey { rung: Decl, file: src/provide.rs, decl: 2, sub: 0, line: 12 } |  |  | 0.636 |
| walker |  | 1956 | 10 | Code::CodeKey { rung: Body, file: src/provide.rs, decl: 2, sub: 0, line: 12 } |  |  | 0.636 |
| ns | 1960 |  | 121 | Details: #[backtrace] on a source field forwards provide() | 2.8 |  | 0.625 |
| ns | 2005 |  | 45 | Details: #[from] variants with a Backtrace field capture in From | 2.9 |  | 0.620 |
| walker |  | 2061 | 105 | Code::CodeKey { rung: Names, file: src/display.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.621 |
| walker |  | 2070 | 9 | Code::CodeKey { rung: Decl, file: src/display.rs, decl: 8, sub: 0, line: 48 } |  |  | 0.621 |
| walker |  | 2079 | 9 | Code::CodeKey { rung: Decl, file: src/display.rs, decl: 9, sub: 0, line: 50 } |  |  | 0.621 |
| ns | 2088 |  | 83 | Details example: extra format arguments (`max = i32::MAX`) | 2.10 |  | 0.606 |
| walker |  | 2131 | 52 | Code::CodeKey { rung: Decl, file: src/display.rs, decl: 3, sub: 0, line: 25 } |  |  | 0.607 |
| walker |  | 2137 | 6 | Code::CodeKey { rung: Decl, file: src/display.rs, decl: 4, sub: 0, line: 29 } |  |  | 0.607 |
| walker |  | 2187 | 50 | Code::CodeKey { rung: Decl, file: src/display.rs, decl: 5, sub: 0, line: 35 } |  |  | 0.607 |
| walker |  | 2193 | 6 | Code::CodeKey { rung: Decl, file: src/display.rs, decl: 6, sub: 0, line: 39 } |  |  | 0.607 |
| ns | 2254 |  | 166 | Details example: referring to fields from format args via `.var` / `.0` | 2.11 |  | 0.584 |
| walker |  | 2259 | 66 | Code::CodeKey { rung: Decl, file: src/display.rs, decl: 1, sub: 0, line: 14 } |  |  | 0.584 |
| walker |  | 2267 | 8 | Code::CodeKey { rung: Body, file: src/display.rs, decl: 2, sub: 0, line: 20 } |  |  | 0.584 |
| walker |  | 2275 | 8 | Code::CodeKey { rung: Body, file: src/display.rs, decl: 4, sub: 0, line: 29 } |  |  | 0.584 |
| walker |  | 2283 | 8 | Code::CodeKey { rung: Body, file: src/display.rs, decl: 6, sub: 0, line: 39 } |  |  | 0.584 |
| walker |  | 2292 | 9 | Fs::DirListing { dir: tests/no-std } |  |  | 0.585 |
| ns | 2410 |  | 156 | When to use thiserror vs anyhow | 2.12 |  | 0.569 |
| walker |  | 2506 | 214 | Code::CodeKey { rung: Names, file: src/aserror.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.570 |
| ns | 2525 |  | 115 | impl/src/expand.rs: complete top-level function roster (names only) | 3.1 |  | 0.552 |
| walker |  | 2534 | 28 | Code::CodeKey { rung: Decl, file: src/aserror.rs, decl: 1, sub: 0, line: 9 } |  |  | 0.552 |
| walker |  | 2540 | 6 | Code::CodeKey { rung: Decl, file: src/aserror.rs, decl: 2, sub: 0, line: 10 } |  |  | 0.552 |
| walker |  | 2568 | 28 | Code::CodeKey { rung: Decl, file: src/aserror.rs, decl: 3, sub: 0, line: 16 } |  |  | 0.552 |
| walker |  | 2574 | 6 | Code::CodeKey { rung: Decl, file: src/aserror.rs, decl: 4, sub: 0, line: 17 } |  |  | 0.552 |
| walker |  | 2602 | 28 | Code::CodeKey { rung: Decl, file: src/aserror.rs, decl: 5, sub: 0, line: 23 } |  |  | 0.552 |
| walker |  | 2608 | 6 | Code::CodeKey { rung: Decl, file: src/aserror.rs, decl: 6, sub: 0, line: 24 } |  |  | 0.552 |
| walker |  | 2636 | 28 | Code::CodeKey { rung: Decl, file: src/aserror.rs, decl: 7, sub: 0, line: 30 } |  |  | 0.552 |
| walker |  | 2642 | 6 | Code::CodeKey { rung: Decl, file: src/aserror.rs, decl: 8, sub: 0, line: 31 } |  |  | 0.552 |
| walker |  | 2672 | 30 | Code::CodeKey { rung: Decl, file: src/aserror.rs, decl: 9, sub: 0, line: 37 } |  |  | 0.552 |
| walker |  | 2678 | 6 | Code::CodeKey { rung: Decl, file: src/aserror.rs, decl: 10, sub: 0, line: 38 } |  |  | 0.552 |
| walker |  | 2685 | 7 | Code::CodeKey { rung: Body, file: src/aserror.rs, decl: 2, sub: 0, line: 10 } |  |  | 0.552 |
| walker |  | 2692 | 7 | Code::CodeKey { rung: Body, file: src/aserror.rs, decl: 4, sub: 0, line: 17 } |  |  | 0.552 |
| walker |  | 2699 | 7 | Code::CodeKey { rung: Body, file: src/aserror.rs, decl: 6, sub: 0, line: 24 } |  |  | 0.552 |
| walker |  | 2706 | 7 | Code::CodeKey { rung: Body, file: src/aserror.rs, decl: 8, sub: 0, line: 31 } |  |  | 0.552 |
| ns | 2723 |  | 198 | expand.rs: derive/try_expand — the whole expansion pipeline in 18 lines | 3.2 | 3.1 | 0.527 |
| ns | 2950 |  | 227 | expand.rs: the two emitted impl shapes (struct and enum quote! tails) | 3.3 |  | 0.502 |
| walker |  | 3234 | 528 | Fs::DirListing { dir: tests/ui } |  |  | 0.502 |
| walker |  | 3241 | 7 | Code::CodeKey { rung: Body, file: src/aserror.rs, decl: 10, sub: 0, line: 38 } |  |  | 0.502 |
| ns | 3290 |  | 340 | impl/src/ast.rs: the complete IR (Input, Struct, Enum, Variant, Field) | 3.4 |  | 0.467 |
| walker |  | 3346 | 105 | Code::CodeKey { rung: Names, file: impl/src/ast.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.470 |
| walker |  | 3372 | 26 | Code::CodeKey { rung: Decl, file: impl/src/ast.rs, decl: 1, sub: 0, line: 10 } |  |  | 0.473 |
| walker |  | 3400 | 28 | Code::CodeKey { rung: Decl, file: impl/src/ast.rs, decl: 9, sub: 0, line: 174 } |  |  | 0.473 |
| walker |  | 3429 | 29 | Code::CodeKey { rung: Decl, file: impl/src/ast.rs, decl: 7, sub: 0, line: 54 } |  |  | 0.473 |
| walker |  | 3482 | 53 | Code::CodeKey { rung: Decl, file: impl/src/ast.rs, decl: 2, sub: 0, line: 15 } |  |  | 0.484 |
| ns | 3528 |  | 238 | impl/src/attr.rs: Attrs and the parsed Display attribute | 3.5 |  | 0.465 |
| walker |  | 3535 | 53 | Code::CodeKey { rung: Decl, file: impl/src/ast.rs, decl: 4, sub: 0, line: 29 } |  |  | 0.481 |
| walker |  | 3589 | 54 | Code::CodeKey { rung: Decl, file: impl/src/ast.rs, decl: 3, sub: 0, line: 22 } |  |  | 0.501 |
| walker |  | 3650 | 61 | Code::CodeKey { rung: Decl, file: impl/src/ast.rs, decl: 6, sub: 0, line: 44 } |  |  | 0.502 |
| walker |  | 3715 | 65 | Code::CodeKey { rung: Decl, file: impl/src/ast.rs, decl: 5, sub: 0, line: 36 } |  |  | 0.532 |
| walker |  | 3768 | 53 | Code::CodeKey { rung: Names, file: src/var.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.532 |
| walker |  | 3796 | 28 | Code::CodeKey { rung: Decl, file: src/var.rs, decl: 2, sub: 0, line: 5 } |  |  | 0.532 |
| walker |  | 3810 | 14 | Code::CodeKey { rung: Body, file: src/var.rs, decl: 3, sub: 0, line: 6 } |  |  | 0.532 |
| ns | 3847 |  | 319 | attr.rs: Source/From/Transparent/Fmt payloads and the Trait enum | 3.6 |  | 0.499 |
| ns | 4040 |  | 193 | attr.rs: the three accepted forms of #[error(...)] | 3.7 |  | 0.487 |
| walker |  | 4064 | 254 | Code::CodeKey { rung: Names, file: impl/src/unraw.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.487 |
| walker |  | 4081 | 17 | Code::CodeKey { rung: Decl, file: impl/src/unraw.rs, decl: 1, sub: 0, line: 10 } |  |  | 0.487 |
| walker |  | 4100 | 19 | Code::CodeKey { rung: Decl, file: impl/src/unraw.rs, decl: 22, sub: 0, line: 87 } |  |  | 0.487 |
| walker |  | 4122 | 22 | Code::CodeKey { rung: Decl, file: impl/src/unraw.rs, decl: 17, sub: 0, line: 69 } |  |  | 0.487 |
| walker |  | 4145 | 23 | Code::CodeKey { rung: Decl, file: impl/src/unraw.rs, decl: 9, sub: 0, line: 45 } |  |  | 0.487 |
| walker |  | 4168 | 23 | Code::CodeKey { rung: Decl, file: impl/src/unraw.rs, decl: 11, sub: 0, line: 51 } |  |  | 0.487 |
| ns | 4186 |  | 146 | impl/src/valid.rs: container-level diagnostic messages | 3.8 |  | 0.482 |
| walker |  | 4191 | 23 | Code::CodeKey { rung: Decl, file: impl/src/unraw.rs, decl: 13, sub: 0, line: 57 } |  |  | 0.482 |
| walker |  | 4214 | 23 | Code::CodeKey { rung: Decl, file: impl/src/unraw.rs, decl: 27, sub: 0, line: 107 } |  |  | 0.482 |
| walker |  | 4237 | 23 | Code::CodeKey { rung: Decl, file: impl/src/unraw.rs, decl: 29, sub: 0, line: 117 } |  |  | 0.482 |
| walker |  | 4261 | 24 | Code::CodeKey { rung: Decl, file: impl/src/unraw.rs, decl: 33, sub: 0, line: 135 } |  |  | 0.482 |
| walker |  | 4287 | 26 | Code::CodeKey { rung: Decl, file: impl/src/unraw.rs, decl: 19, sub: 0, line: 75 } |  |  | 0.482 |
| walker |  | 4314 | 27 | Code::CodeKey { rung: Decl, file: impl/src/unraw.rs, decl: 15, sub: 0, line: 63 } |  |  | 0.482 |
| walker |  | 4341 | 27 | Code::CodeKey { rung: Decl, file: impl/src/unraw.rs, decl: 31, sub: 0, line: 126 } |  |  | 0.482 |
| walker |  | 4369 | 28 | Code::CodeKey { rung: Decl, file: impl/src/unraw.rs, decl: 6, sub: 0, line: 37 } |  |  | 0.482 |
| walker |  | 4397 | 28 | Code::CodeKey { rung: Decl, file: impl/src/unraw.rs, decl: 24, sub: 0, line: 96 } |  |  | 0.482 |
| walker |  | 4426 | 29 | Code::CodeKey { rung: Decl, file: impl/src/unraw.rs, decl: 21, sub: 0, line: 81 } |  |  | 0.482 |
| ns | 4478 |  | 292 | valid.rs: attribute-placement and field-attribute diagnostic messages | 3.9 |  | 0.472 |
| walker |  | 4484 | 58 | Code::CodeKey { rung: Decl, file: impl/src/unraw.rs, decl: 2, sub: 0, line: 14 } |  |  | 0.472 |
| walker |  | 4604 | 120 | Toml::Identity { file: impl/Cargo.toml } |  |  | 0.473 |
| ns | 4647 |  | 169 | valid.rs: complete validate/check function roster | 3.10 |  | 0.464 |
| walker |  | 4798 | 194 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.492 |
| ns | 4859 |  | 212 | attr.rs: which attributes `get` recognises, plus the file's function roster | 3.11 |  | 0.481 |
| walker |  | 4996 | 198 | Code::CodeKey { rung: Names, file: impl/src/attr.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.485 |
| walker |  | 5020 | 24 | Code::CodeKey { rung: Decl, file: impl/src/attr.rs, decl: 11, sub: 0, line: 302 } |  |  | 0.485 |
| walker |  | 5044 | 24 | Code::CodeKey { rung: Decl, file: impl/src/attr.rs, decl: 13, sub: 0, line: 342 } |  |  | 0.485 |
| walker |  | 5080 | 36 | Code::CodeKey { rung: Decl, file: impl/src/attr.rs, decl: 6, sub: 0, line: 50 } |  |  | 0.489 |
| ns | 5096 |  | 237 | impl/src/prop.rs: complete accessor roster for source/from/backtrace fields | 3.12 |  | 0.475 |
| walker |  | 5116 | 36 | Code::CodeKey { rung: Decl, file: impl/src/attr.rs, decl: 5, sub: 0, line: 44 } |  |  | 0.481 |
| walker |  | 5152 | 36 | Code::CodeKey { rung: Decl, file: impl/src/attr.rs, decl: 4, sub: 0, line: 38 } |  |  | 0.489 |
| walker |  | 5188 | 36 | Code::CodeKey { rung: Decl, file: impl/src/attr.rs, decl: 3, sub: 0, line: 32 } |  |  | 0.499 |
| walker |  | 5278 | 90 | Code::CodeKey { rung: Decl, file: impl/src/attr.rs, decl: 1, sub: 0, line: 11 } |  |  | 0.507 |
| ns | 5305 |  | 209 | prop.rs: how the source and backtrace fields are actually chosen | 3.13 |  | 0.494 |
| walker |  | 5370 | 92 | Code::CodeKey { rung: Decl, file: impl/src/attr.rs, decl: 7, sub: 0, line: 56 } |  |  | 0.526 |
| walker |  | 5482 | 112 | Code::CodeKey { rung: Decl, file: impl/src/attr.rs, decl: 2, sub: 0, line: 20 } |  |  | 0.553 |
| walker |  | 5567 | 85 | Code::CodeKey { rung: Names, file: impl/src/generics.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.553 |
| walker |  | 5583 | 16 | Code::CodeKey { rung: Decl, file: impl/src/generics.rs, decl: 1, sub: 0, line: 8 } |  |  | 0.553 |
| ns | 5595 |  | 290 | impl/src/fmt.rs: expand_shorthand and the format-spec → Trait mapping | 3.14 |  | 0.538 |
| walker |  | 5622 | 39 | Code::CodeKey { rung: Decl, file: impl/src/generics.rs, decl: 6, sub: 0, line: 48 } |  |  | 0.539 |
| walker |  | 5668 | 46 | Code::CodeKey { rung: Decl, file: impl/src/generics.rs, decl: 2, sub: 0, line: 12 } |  |  | 0.539 |
| ns | 5726 |  | 131 | fmt.rs: FmtArguments and the remaining function roster | 3.15 | 3.14 | 0.532 |
| walker |  | 5738 | 70 | Code::CodeKey { rung: Decl, file: impl/src/generics.rs, decl: 7, sub: 0, line: 53 } |  |  | 0.532 |
| walker |  | 5770 | 32 | Code::CodeKey { rung: Body, file: impl/src/generics.rs, decl: 4, sub: 0, line: 19 } |  |  | 0.532 |
| walker |  | 5807 | 37 | Code::CodeKey { rung: Body, file: impl/src/generics.rs, decl: 3, sub: 0, line: 13 } |  |  | 0.532 |
| walker |  | 5881 | 74 | Code::CodeKey { rung: Names, file: impl/src/valid.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.534 |
| walker |  | 5905 | 24 | Code::CodeKey { rung: Decl, file: impl/src/valid.rs, decl: 1, sub: 0, line: 5 } |  |  | 0.529 |
| ns | 5905 |  | 179 | ast.rs: ContainerKind and its six display strings | 3.16 |  | 0.529 |
| walker |  | 5947 | 42 | Code::CodeKey { rung: Body, file: impl/src/valid.rs, decl: 2, sub: 0, line: 6 } |  |  | 0.529 |
| walker |  | 6160 | 213 | Code::CodeKey { rung: Names, file: impl/src/expand.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.549 |
| ns | 6198 |  | 293 | ast.rs: complete from_syn constructor roster | 3.17 |  | 0.540 |
| walker |  | 6204 | 44 | Code::CodeKey { rung: Decl, file: impl/src/expand.rs, decl: 8, sub: 0, line: 536 } |  |  | 0.540 |
| walker |  | 6219 | 15 | Code::CodeKey { rung: Body, file: impl/src/expand.rs, decl: 9, sub: 0, line: 565 } |  |  | 0.540 |
| walker |  | 6251 | 32 | Code::CodeKey { rung: Body, file: impl/src/expand.rs, decl: 10, sub: 0, line: 569 } |  |  | 0.540 |
| walker |  | 6290 | 39 | Code::CodeKey { rung: Body, file: impl/src/expand.rs, decl: 5, sub: 0, line: 505 } |  |  | 0.540 |
| walker |  | 6354 | 64 | Code::CodeKey { rung: Body, file: impl/src/expand.rs, decl: 7, sub: 0, line: 526 } |  |  | 0.540 |
| ns | 6357 |  | 159 | impl/src/generics.rs: ParamsInScope and InferredBounds | 3.18 |  | 0.550 |
| walker |  | 6422 | 68 | Code::CodeKey { rung: Body, file: impl/src/expand.rs, decl: 2, sub: 0, line: 22 } |  |  | 0.556 |
| ns | 6606 |  | 249 | impl/src/unraw.rs: IdentUnraw/MemberUnraw and the raw-identifier rule | 3.19 |  | 0.548 |
| walker |  | 6623 | 201 | Code::CodeKey { rung: ModuleDoc, file: src/lib.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.548 |
| walker |  | 6811 | 188 | Code::CodeKey { rung: Names, file: impl/src/fmt.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.555 |
| walker |  | 6825 | 14 | Code::CodeKey { rung: Decl, file: impl/src/fmt.rs, decl: 4, sub: 0, line: 171 } |  |  | 0.555 |
| walker |  | 6857 | 32 | Code::CodeKey { rung: Decl, file: impl/src/fmt.rs, decl: 3, sub: 0, line: 166 } |  |  | 0.563 |
| walker |  | 6890 | 33 | Code::CodeKey { rung: Decl, file: impl/src/fmt.rs, decl: 1, sub: 0, line: 15 } |  |  | 0.564 |
| ns | 6910 |  | 304 | impl/src/fallback.rs: the invalid-input fallback expansion | 3.20 |  | 0.550 |
| walker |  | 7083 | 193 | Code::CodeKey { rung: Names, file: impl/src/prop.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.559 |
| ns | 7121 |  | 211 | impl/src/scan_expr.rs: the Input/Action alphabet of the expression scanner | 3.21 |  | 0.545 |
| walker |  | 7125 | 42 | Code::CodeKey { rung: Decl, file: impl/src/prop.rs, decl: 15, sub: 0, line: 72 } |  |  | 0.548 |
| walker |  | 7167 | 42 | Code::CodeKey { rung: Decl, file: impl/src/prop.rs, decl: 21, sub: 0, line: 127 } |  |  | 0.548 |
| walker |  | 7228 | 61 | Code::CodeKey { rung: Decl, file: impl/src/prop.rs, decl: 6, sub: 0, line: 25 } |  |  | 0.552 |
| walker |  | 7240 | 12 | Code::CodeKey { rung: Body, file: impl/src/prop.rs, decl: 16, sub: 0, line: 73 } |  |  | 0.552 |
| walker |  | 7334 | 94 | Code::CodeKey { rung: Decl, file: impl/src/prop.rs, decl: 1, sub: 0, line: 6 } |  |  | 0.559 |
| walker |  | 7428 | 94 | Code::CodeKey { rung: Decl, file: impl/src/prop.rs, decl: 10, sub: 0, line: 53 } |  |  | 0.566 |
| walker |  | 7439 | 11 | Code::CodeKey { rung: Body, file: impl/src/prop.rs, decl: 2, sub: 0, line: 7 } |  |  | 0.566 |
| walker |  | 7450 | 11 | Code::CodeKey { rung: Body, file: impl/src/prop.rs, decl: 3, sub: 0, line: 11 } |  |  | 0.566 |
| walker |  | 7461 | 11 | Code::CodeKey { rung: Body, file: impl/src/prop.rs, decl: 11, sub: 0, line: 54 } |  |  | 0.566 |
| walker |  | 7472 | 11 | Code::CodeKey { rung: Body, file: impl/src/prop.rs, decl: 12, sub: 0, line: 58 } |  |  | 0.566 |
| ns | 7477 |  | 356 | expand.rs: the gate line for every conditionally generated impl | 3.22 | 3.1 | 0.555 |
| walker |  | 7484 | 12 | Code::CodeKey { rung: Body, file: impl/src/prop.rs, decl: 4, sub: 0, line: 15 } |  |  | 0.555 |
| ns | 7637 |  | 160 | src/private.rs: the complete generated-code support surface | 4.1 |  | 0.548 |
| walker |  | 7719 | 235 | Code::CodeKey { rung: Names, file: impl/src/scan_expr.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.548 |
| walker |  | 7751 | 32 | Code::CodeKey { rung: Decl, file: impl/src/scan_expr.rs, decl: 7, sub: 0, line: 84 } |  |  | 0.548 |
| walker |  | 7789 | 38 | Code::CodeKey { rung: Decl, file: impl/src/scan_expr.rs, decl: 10, sub: 0, line: 104 } |  |  | 0.548 |
| walker |  | 7827 | 38 | Code::CodeKey { rung: Decl, file: impl/src/scan_expr.rs, decl: 12, sub: 0, line: 114 } |  |  | 0.548 |
| ns | 7852 |  | 215 | src/aserror.rs: AsDynError and its five blanket/dyn impls | 4.2 |  | 0.547 |
| walker |  | 7868 | 41 | Code::CodeKey { rung: Decl, file: impl/src/scan_expr.rs, decl: 11, sub: 0, line: 109 } |  |  | 0.547 |
| walker |  | 7910 | 42 | Code::CodeKey { rung: Decl, file: impl/src/scan_expr.rs, decl: 2, sub: 0, line: 25 } |  |  | 0.549 |
| walker |  | 7959 | 49 | Code::CodeKey { rung: Decl, file: impl/src/scan_expr.rs, decl: 8, sub: 0, line: 89 } |  |  | 0.549 |
| walker |  | 8011 | 52 | Code::CodeKey { rung: Decl, file: impl/src/scan_expr.rs, decl: 5, sub: 0, line: 76 } |  |  | 0.549 |
| ns | 8094 |  | 242 | src/display.rs: AsDisplay and the std-only Path/PathBuf specializations | 4.3 |  | 0.545 |
| walker |  | 8116 | 105 | Code::CodeKey { rung: Decl, file: impl/src/scan_expr.rs, decl: 9, sub: 0, line: 95 } |  |  | 0.545 |
| walker |  | 8258 | 142 | Code::CodeKey { rung: Decl, file: impl/src/scan_expr.rs, decl: 1, sub: 0, line: 6 } |  |  | 0.571 |
| ns | 8376 |  | 282 | src/provide.rs (ThiserrorProvide) and src/var.rs (Var) end to end | 4.4 |  | 0.568 |
| walker |  | 8423 | 165 | Code::CodeKey { rung: Decl, file: impl/src/scan_expr.rs, decl: 4, sub: 0, line: 63 } |  |  | 0.568 |
| ns | 8489 |  | 113 | impl/src/lib.rs: the version-stamped `private` path token | 4.5 |  | 0.572 |
| ns | 8738 |  | 249 | build.rs: the generated __private module and the cfg declarations | 4.6 |  | 0.566 |
| walker |  | 8874 | 451 | Code::CodeKey { rung: Decl, file: impl/src/scan_expr.rs, decl: 3, sub: 0, line: 32 } |  |  | 0.566 |
| ns | 8946 |  | 208 | build.rs: rustc capability decisions, function roster, and the probe file | 4.7 |  | 0.561 |
| ns | 9043 |  | 97 | Complete tests/, tests/no-std/ and .github/ listings | 5.1 |  | 0.571 |
| ns | 9131 |  | 88 | tests/compiletest.rs: the trybuild UI harness in full | 5.2 |  | 0.567 |
| walker |  | 9157 | 283 | Code::CodeKey { rung: Names, file: impl/src/scan_expr.rs, decl: 0, sub: 1, line: 0 } |  |  | 0.567 |
| walker |  | 9178 | 21 | Code::CodeKey { rung: Decl, file: impl/src/scan_expr.rs, decl: 17, sub: 0, line: 136 } |  |  | 0.567 |
| walker |  | 9210 | 32 | Code::CodeKey { rung: Decl, file: impl/src/scan_expr.rs, decl: 13, sub: 0, line: 119 } |  |  | 0.567 |
| walker |  | 9242 | 32 | Code::CodeKey { rung: Decl, file: impl/src/scan_expr.rs, decl: 24, sub: 0, line: 187 } |  |  | 0.567 |
| ns | 9262 |  | 131 | tests/no-std/test.rs: the no_std smoke test's error types | 5.3 |  | 0.561 |
| walker |  | 9276 | 34 | Code::CodeKey { rung: Decl, file: impl/src/scan_expr.rs, decl: 15, sub: 0, line: 130 } |  |  | 0.561 |
| walker |  | 9322 | 46 | Code::CodeKey { rung: Decl, file: impl/src/scan_expr.rs, decl: 23, sub: 0, line: 181 } |  |  | 0.561 |
| walker |  | 9369 | 47 | Code::CodeKey { rung: Decl, file: impl/src/scan_expr.rs, decl: 22, sub: 0, line: 175 } |  |  | 0.561 |
| ns | 9389 |  | 127 | Cargo.toml: std-feature rationale and dev-dependencies | 5.4 |  | 0.559 |
| walker |  | 9417 | 48 | Code::CodeKey { rung: Decl, file: impl/src/scan_expr.rs, decl: 14, sub: 0, line: 124 } |  |  | 0.559 |
| walker |  | 9483 | 66 | Code::CodeKey { rung: Decl, file: impl/src/scan_expr.rs, decl: 19, sub: 0, line: 141 } |  |  | 0.559 |
| ns | 9515 |  | 126 | impl/Cargo.toml: the proc-macro crate's manifest | 5.5 |  | 0.564 |
| walker |  | 9575 | 92 | Code::CodeKey { rung: Decl, file: impl/src/scan_expr.rs, decl: 21, sub: 0, line: 166 } |  |  | 0.564 |
| ns | 9689 |  | 174 | CI job roster and the pinned toolchain components | 5.6 |  | 0.557 |
| walker |  | 9814 | 239 | Code::CodeKey { rung: Decl, file: impl/src/scan_expr.rs, decl: 20, sub: 0, line: 148 } |  |  | 0.557 |
| walker |  | 9990 | 176 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.567 |
