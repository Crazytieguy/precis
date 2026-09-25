Score(3000)=0.502 I=0.813 C=0.310 ns_rows≤3K=24/56 grid(1000/1442/2080/3000/4327/6240/9000)=0.794/0.717/0.620/0.502/0.482/0.521/0.561

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
| ns | 612 |  | 136 | Root crate module structure and public re-export (src/lib.rs tail) | 1.7 |  | 0.662 |
| ns | 707 |  | 95 | Root crate attributes: no_std, docs.rs root, nightly cfg gate | 1.8 |  | 0.629 |
| walker |  | 710 | 196 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: true } |  |  | 0.797 |
| ns | 814 |  | 107 | Cargo manifest: features, dependency on impl, workspace members | 1.9 |  | 0.782 |
| walker |  | 830 | 120 | Code::CodeKey { rung: Names, file: impl/src/lib.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.784 |
| walker |  | 842 | 12 | Code::CodeKey { rung: Decl, file: impl/src/lib.rs, decl: 2, sub: 0, line: 45 } |  |  | 0.784 |
| walker |  | 863 | 21 | Code::CodeKey { rung: Decl, file: impl/src/lib.rs, decl: 1, sub: 0, line: 39 } |  |  | 0.791 |
| ns | 870 |  | 56 | Documentation map: README section headings + rustdoc mirror in src/lib.rs | 2.1 |  | 0.772 |
| walker |  | 891 | 28 | Code::CodeKey { rung: Decl, file: impl/src/lib.rs, decl: 3, sub: 0, line: 48 } |  |  | 0.772 |
| walker |  | 922 | 31 | Code::CodeKey { rung: Body, file: impl/src/lib.rs, decl: 1, sub: 0, line: 39 } |  |  | 0.792 |
| walker |  | 994 | 72 | Fs::DirListing { dir: tests } |  |  | 0.794 |
| ns | 1036 |  | 166 | Details: no public API footprint, allowed error shapes, Display generation | 2.2 |  | 0.747 |
| walker |  | 1065 | 71 | Code::CodeKey { rung: Names, file: src/lib.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.774 |
| ns | 1222 |  | 186 | Display shorthand table: {var}, {0}, {var:?}, {0:?} | 2.3 |  | 0.739 |
| walker |  | 1229 | 164 | Toml::Operational { file: Cargo.toml } |  |  | 0.763 |
| walker |  | 1352 | 123 | Toml::Config { file: Cargo.toml } |  |  | 0.763 |
| walker |  | 1399 | 47 | Code::CodeKey { rung: Body, file: impl/src/lib.rs, decl: 4, sub: 0, line: 49 } |  |  | 0.764 |
| ns | 1404 |  | 182 | Details: #[from] generates From, with its field-count restriction | 2.4 |  | 0.717 |
| walker |  | 1452 | 53 | Toml::Dependencies { file: impl/Cargo.toml } |  |  | 0.717 |
| walker |  | 1535 | 83 | Code::CodeKey { rung: Names, file: build.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.717 |
| ns | 1557 |  | 153 | Details: source() from #[source] or a field named `source` | 2.5 |  | 0.690 |
| walker |  | 1581 | 46 | Code::CodeKey { rung: Decl, file: build.rs, decl: 1, sub: 0, line: 10 } |  |  | 0.690 |
| walker |  | 1636 | 55 | Code::CodeKey { rung: Body, file: build.rs, decl: 5, sub: 0, line: 190 } |  |  | 0.690 |
| ns | 1709 |  | 152 | Details: #[error(transparent)] forwarding | 2.6 |  | 0.656 |
| walker |  | 1753 | 117 | Code::CodeKey { rung: Body, file: build.rs, decl: 4, sub: 0, line: 179 } |  |  | 0.656 |
| walker |  | 1834 | 81 | Code::CodeKey { rung: Names, file: build/probe.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.656 |
| ns | 1839 |  | 130 | Details: provide() and automatic Backtrace field detection | 2.7 |  | 0.635 |
| walker |  | 1863 | 29 | Code::CodeKey { rung: Decl, file: build/probe.rs, decl: 3, sub: 0, line: 14 } |  |  | 0.635 |
| walker |  | 1892 | 29 | Code::CodeKey { rung: Decl, file: build/probe.rs, decl: 5, sub: 0, line: 20 } |  |  | 0.635 |
| walker |  | 1922 | 30 | Code::CodeKey { rung: Decl, file: build/probe.rs, decl: 7, sub: 0, line: 26 } |  |  | 0.635 |
| walker |  | 1931 | 9 | Code::CodeKey { rung: Body, file: build/probe.rs, decl: 4, sub: 0, line: 15 } |  |  | 0.635 |
| walker |  | 1940 | 9 | Code::CodeKey { rung: Body, file: build/probe.rs, decl: 6, sub: 0, line: 21 } |  |  | 0.635 |
| walker |  | 1954 | 14 | Code::CodeKey { rung: Body, file: build/probe.rs, decl: 8, sub: 0, line: 27 } |  |  | 0.635 |
| ns | 1960 |  | 121 | Details: #[backtrace] on a source field forwards provide() | 2.8 |  | 0.624 |
| walker |  | 1988 | 34 | Code::CodeKey { rung: Names, file: src/provide.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.624 |
| ns | 2005 |  | 45 | Details: #[from] variants with a Backtrace field capture in From | 2.9 |  | 0.620 |
| walker |  | 2046 | 58 | Code::CodeKey { rung: Decl, file: src/provide.rs, decl: 1, sub: 0, line: 8 } |  |  | 0.620 |
| walker |  | 2052 | 6 | Code::CodeKey { rung: Decl, file: src/provide.rs, decl: 2, sub: 0, line: 12 } |  |  | 0.620 |
| walker |  | 2062 | 10 | Code::CodeKey { rung: Body, file: src/provide.rs, decl: 2, sub: 0, line: 12 } |  |  | 0.620 |
| ns | 2088 |  | 83 | Details example: extra format arguments (`max = i32::MAX`) | 2.10 |  | 0.606 |
| walker |  | 2167 | 105 | Code::CodeKey { rung: Names, file: src/display.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.606 |
| walker |  | 2176 | 9 | Code::CodeKey { rung: Decl, file: src/display.rs, decl: 8, sub: 0, line: 48 } |  |  | 0.606 |
| walker |  | 2185 | 9 | Code::CodeKey { rung: Decl, file: src/display.rs, decl: 9, sub: 0, line: 50 } |  |  | 0.606 |
| walker |  | 2237 | 52 | Code::CodeKey { rung: Decl, file: src/display.rs, decl: 3, sub: 0, line: 25 } |  |  | 0.606 |
| walker |  | 2243 | 6 | Code::CodeKey { rung: Decl, file: src/display.rs, decl: 4, sub: 0, line: 29 } |  |  | 0.606 |
| ns | 2254 |  | 166 | Details example: referring to fields from format args via `.var` / `.0` | 2.11 |  | 0.584 |
| walker |  | 2293 | 50 | Code::CodeKey { rung: Decl, file: src/display.rs, decl: 5, sub: 0, line: 35 } |  |  | 0.584 |
| walker |  | 2299 | 6 | Code::CodeKey { rung: Decl, file: src/display.rs, decl: 6, sub: 0, line: 39 } |  |  | 0.584 |
| walker |  | 2365 | 66 | Code::CodeKey { rung: Decl, file: src/display.rs, decl: 1, sub: 0, line: 14 } |  |  | 0.584 |
| walker |  | 2373 | 8 | Code::CodeKey { rung: Body, file: src/display.rs, decl: 2, sub: 0, line: 20 } |  |  | 0.584 |
| walker |  | 2381 | 8 | Code::CodeKey { rung: Body, file: src/display.rs, decl: 4, sub: 0, line: 29 } |  |  | 0.584 |
| walker |  | 2389 | 8 | Code::CodeKey { rung: Body, file: src/display.rs, decl: 6, sub: 0, line: 39 } |  |  | 0.584 |
| walker |  | 2398 | 9 | Fs::DirListing { dir: tests/no-std } |  |  | 0.584 |
| ns | 2410 |  | 156 | When to use thiserror vs anyhow | 2.12 |  | 0.569 |
| ns | 2525 |  | 115 | impl/src/expand.rs: complete top-level function roster (names only) | 3.1 |  | 0.552 |
| walker |  | 2612 | 214 | Code::CodeKey { rung: Names, file: src/aserror.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.552 |
| walker |  | 2640 | 28 | Code::CodeKey { rung: Decl, file: src/aserror.rs, decl: 1, sub: 0, line: 9 } |  |  | 0.552 |
| walker |  | 2646 | 6 | Code::CodeKey { rung: Decl, file: src/aserror.rs, decl: 2, sub: 0, line: 10 } |  |  | 0.552 |
| walker |  | 2674 | 28 | Code::CodeKey { rung: Decl, file: src/aserror.rs, decl: 3, sub: 0, line: 16 } |  |  | 0.552 |
| walker |  | 2680 | 6 | Code::CodeKey { rung: Decl, file: src/aserror.rs, decl: 4, sub: 0, line: 17 } |  |  | 0.552 |
| walker |  | 2708 | 28 | Code::CodeKey { rung: Decl, file: src/aserror.rs, decl: 5, sub: 0, line: 23 } |  |  | 0.552 |
| walker |  | 2714 | 6 | Code::CodeKey { rung: Decl, file: src/aserror.rs, decl: 6, sub: 0, line: 24 } |  |  | 0.552 |
| ns | 2723 |  | 198 | expand.rs: derive/try_expand — the whole expansion pipeline in 18 lines | 3.2 | 3.1 | 0.527 |
| walker |  | 2742 | 28 | Code::CodeKey { rung: Decl, file: src/aserror.rs, decl: 7, sub: 0, line: 30 } |  |  | 0.527 |
| walker |  | 2748 | 6 | Code::CodeKey { rung: Decl, file: src/aserror.rs, decl: 8, sub: 0, line: 31 } |  |  | 0.527 |
| walker |  | 2778 | 30 | Code::CodeKey { rung: Decl, file: src/aserror.rs, decl: 9, sub: 0, line: 37 } |  |  | 0.527 |
| walker |  | 2784 | 6 | Code::CodeKey { rung: Decl, file: src/aserror.rs, decl: 10, sub: 0, line: 38 } |  |  | 0.527 |
| walker |  | 2791 | 7 | Code::CodeKey { rung: Body, file: src/aserror.rs, decl: 2, sub: 0, line: 10 } |  |  | 0.527 |
| walker |  | 2798 | 7 | Code::CodeKey { rung: Body, file: src/aserror.rs, decl: 4, sub: 0, line: 17 } |  |  | 0.527 |
| walker |  | 2805 | 7 | Code::CodeKey { rung: Body, file: src/aserror.rs, decl: 6, sub: 0, line: 24 } |  |  | 0.527 |
| walker |  | 2812 | 7 | Code::CodeKey { rung: Body, file: src/aserror.rs, decl: 8, sub: 0, line: 31 } |  |  | 0.527 |
| ns | 2950 |  | 227 | expand.rs: the two emitted impl shapes (struct and enum quote! tails) | 3.3 |  | 0.502 |
| ns | 3290 |  | 340 | impl/src/ast.rs: the complete IR (Input, Struct, Enum, Variant, Field) | 3.4 |  | 0.467 |
| walker |  | 3340 | 528 | Fs::DirListing { dir: tests/ui } |  |  | 0.467 |
| walker |  | 3347 | 7 | Code::CodeKey { rung: Body, file: src/aserror.rs, decl: 10, sub: 0, line: 38 } |  |  | 0.467 |
| walker |  | 3452 | 105 | Code::CodeKey { rung: Names, file: impl/src/ast.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.470 |
| walker |  | 3478 | 26 | Code::CodeKey { rung: Decl, file: impl/src/ast.rs, decl: 1, sub: 0, line: 10 } |  |  | 0.473 |
| walker |  | 3506 | 28 | Code::CodeKey { rung: Decl, file: impl/src/ast.rs, decl: 9, sub: 0, line: 174 } |  |  | 0.473 |
| ns | 3528 |  | 238 | impl/src/attr.rs: Attrs and the parsed Display attribute | 3.5 |  | 0.455 |
| walker |  | 3535 | 29 | Code::CodeKey { rung: Decl, file: impl/src/ast.rs, decl: 7, sub: 0, line: 54 } |  |  | 0.455 |
| walker |  | 3588 | 53 | Code::CodeKey { rung: Decl, file: impl/src/ast.rs, decl: 2, sub: 0, line: 15 } |  |  | 0.465 |
| walker |  | 3641 | 53 | Code::CodeKey { rung: Decl, file: impl/src/ast.rs, decl: 4, sub: 0, line: 29 } |  |  | 0.481 |
| walker |  | 3695 | 54 | Code::CodeKey { rung: Decl, file: impl/src/ast.rs, decl: 3, sub: 0, line: 22 } |  |  | 0.501 |
| walker |  | 3756 | 61 | Code::CodeKey { rung: Decl, file: impl/src/ast.rs, decl: 6, sub: 0, line: 44 } |  |  | 0.501 |
| walker |  | 3821 | 65 | Code::CodeKey { rung: Decl, file: impl/src/ast.rs, decl: 5, sub: 0, line: 36 } |  |  | 0.532 |
| ns | 3847 |  | 319 | attr.rs: Source/From/Transparent/Fmt payloads and the Trait enum | 3.6 |  | 0.498 |
| walker |  | 3874 | 53 | Code::CodeKey { rung: Names, file: src/var.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.499 |
| walker |  | 3902 | 28 | Code::CodeKey { rung: Decl, file: src/var.rs, decl: 2, sub: 0, line: 5 } |  |  | 0.499 |
| walker |  | 3916 | 14 | Code::CodeKey { rung: Body, file: src/var.rs, decl: 3, sub: 0, line: 6 } |  |  | 0.499 |
| ns | 4040 |  | 193 | attr.rs: the three accepted forms of #[error(...)] | 3.7 |  | 0.487 |
| walker |  | 4170 | 254 | Code::CodeKey { rung: Names, file: impl/src/unraw.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.487 |
| ns | 4186 |  | 146 | impl/src/valid.rs: container-level diagnostic messages | 3.8 |  | 0.482 |
| walker |  | 4187 | 17 | Code::CodeKey { rung: Decl, file: impl/src/unraw.rs, decl: 1, sub: 0, line: 10 } |  |  | 0.482 |
| walker |  | 4206 | 19 | Code::CodeKey { rung: Decl, file: impl/src/unraw.rs, decl: 22, sub: 0, line: 87 } |  |  | 0.482 |
| walker |  | 4228 | 22 | Code::CodeKey { rung: Decl, file: impl/src/unraw.rs, decl: 17, sub: 0, line: 69 } |  |  | 0.482 |
| walker |  | 4251 | 23 | Code::CodeKey { rung: Decl, file: impl/src/unraw.rs, decl: 9, sub: 0, line: 45 } |  |  | 0.482 |
| walker |  | 4274 | 23 | Code::CodeKey { rung: Decl, file: impl/src/unraw.rs, decl: 11, sub: 0, line: 51 } |  |  | 0.482 |
| walker |  | 4297 | 23 | Code::CodeKey { rung: Decl, file: impl/src/unraw.rs, decl: 13, sub: 0, line: 57 } |  |  | 0.482 |
| walker |  | 4320 | 23 | Code::CodeKey { rung: Decl, file: impl/src/unraw.rs, decl: 27, sub: 0, line: 107 } |  |  | 0.482 |
| walker |  | 4343 | 23 | Code::CodeKey { rung: Decl, file: impl/src/unraw.rs, decl: 29, sub: 0, line: 117 } |  |  | 0.482 |
| walker |  | 4367 | 24 | Code::CodeKey { rung: Decl, file: impl/src/unraw.rs, decl: 33, sub: 0, line: 135 } |  |  | 0.482 |
| walker |  | 4393 | 26 | Code::CodeKey { rung: Decl, file: impl/src/unraw.rs, decl: 19, sub: 0, line: 75 } |  |  | 0.482 |
| walker |  | 4420 | 27 | Code::CodeKey { rung: Decl, file: impl/src/unraw.rs, decl: 15, sub: 0, line: 63 } |  |  | 0.482 |
| walker |  | 4447 | 27 | Code::CodeKey { rung: Decl, file: impl/src/unraw.rs, decl: 31, sub: 0, line: 126 } |  |  | 0.482 |
| walker |  | 4475 | 28 | Code::CodeKey { rung: Decl, file: impl/src/unraw.rs, decl: 6, sub: 0, line: 37 } |  |  | 0.482 |
| ns | 4478 |  | 292 | valid.rs: attribute-placement and field-attribute diagnostic messages | 3.9 |  | 0.472 |
| walker |  | 4503 | 28 | Code::CodeKey { rung: Decl, file: impl/src/unraw.rs, decl: 24, sub: 0, line: 96 } |  |  | 0.472 |
| walker |  | 4532 | 29 | Code::CodeKey { rung: Decl, file: impl/src/unraw.rs, decl: 21, sub: 0, line: 81 } |  |  | 0.472 |
| walker |  | 4590 | 58 | Code::CodeKey { rung: Decl, file: impl/src/unraw.rs, decl: 2, sub: 0, line: 14 } |  |  | 0.472 |
| ns | 4647 |  | 169 | valid.rs: complete validate/check function roster | 3.10 |  | 0.463 |
| walker |  | 4712 | 122 | Toml::Identity { file: impl/Cargo.toml } |  |  | 0.463 |
| ns | 4859 |  | 212 | attr.rs: which attributes `get` recognises, plus the file's function roster | 3.11 |  | 0.453 |
| walker |  | 4871 | 159 | Toml::Config { file: impl/Cargo.toml } |  |  | 0.453 |
| walker |  | 5065 | 194 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.481 |
| ns | 5096 |  | 237 | impl/src/prop.rs: complete accessor roster for source/from/backtrace fields | 3.12 |  | 0.466 |
| walker |  | 5263 | 198 | Code::CodeKey { rung: Names, file: impl/src/attr.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.471 |
| walker |  | 5287 | 24 | Code::CodeKey { rung: Decl, file: impl/src/attr.rs, decl: 11, sub: 0, line: 302 } |  |  | 0.471 |
| ns | 5305 |  | 209 | prop.rs: how the source and backtrace fields are actually chosen | 3.13 |  | 0.458 |
| walker |  | 5311 | 24 | Code::CodeKey { rung: Decl, file: impl/src/attr.rs, decl: 13, sub: 0, line: 342 } |  |  | 0.458 |
| walker |  | 5347 | 36 | Code::CodeKey { rung: Decl, file: impl/src/attr.rs, decl: 6, sub: 0, line: 50 } |  |  | 0.462 |
| walker |  | 5383 | 36 | Code::CodeKey { rung: Decl, file: impl/src/attr.rs, decl: 5, sub: 0, line: 44 } |  |  | 0.468 |
| walker |  | 5419 | 36 | Code::CodeKey { rung: Decl, file: impl/src/attr.rs, decl: 4, sub: 0, line: 38 } |  |  | 0.476 |
| walker |  | 5455 | 36 | Code::CodeKey { rung: Decl, file: impl/src/attr.rs, decl: 3, sub: 0, line: 32 } |  |  | 0.485 |
| walker |  | 5545 | 90 | Code::CodeKey { rung: Decl, file: impl/src/attr.rs, decl: 1, sub: 0, line: 11 } |  |  | 0.494 |
| ns | 5595 |  | 290 | impl/src/fmt.rs: expand_shorthand and the format-spec → Trait mapping | 3.14 |  | 0.481 |
| walker |  | 5637 | 92 | Code::CodeKey { rung: Decl, file: impl/src/attr.rs, decl: 7, sub: 0, line: 56 } |  |  | 0.512 |
| ns | 5726 |  | 131 | fmt.rs: FmtArguments and the remaining function roster | 3.15 | 3.14 | 0.505 |
| walker |  | 5749 | 112 | Code::CodeKey { rung: Decl, file: impl/src/attr.rs, decl: 2, sub: 0, line: 20 } |  |  | 0.531 |
| walker |  | 5834 | 85 | Code::CodeKey { rung: Names, file: impl/src/generics.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.531 |
| walker |  | 5850 | 16 | Code::CodeKey { rung: Decl, file: impl/src/generics.rs, decl: 1, sub: 0, line: 8 } |  |  | 0.531 |
| walker |  | 5889 | 39 | Code::CodeKey { rung: Decl, file: impl/src/generics.rs, decl: 6, sub: 0, line: 48 } |  |  | 0.532 |
| ns | 5905 |  | 179 | ast.rs: ContainerKind and its six display strings | 3.16 |  | 0.525 |
| walker |  | 5935 | 46 | Code::CodeKey { rung: Decl, file: impl/src/generics.rs, decl: 2, sub: 0, line: 12 } |  |  | 0.525 |
| walker |  | 6005 | 70 | Code::CodeKey { rung: Decl, file: impl/src/generics.rs, decl: 7, sub: 0, line: 53 } |  |  | 0.526 |
| walker |  | 6037 | 32 | Code::CodeKey { rung: Body, file: impl/src/generics.rs, decl: 4, sub: 0, line: 19 } |  |  | 0.526 |
| walker |  | 6074 | 37 | Code::CodeKey { rung: Body, file: impl/src/generics.rs, decl: 3, sub: 0, line: 13 } |  |  | 0.526 |
| walker |  | 6148 | 74 | Code::CodeKey { rung: Names, file: impl/src/valid.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.527 |
| walker |  | 6172 | 24 | Code::CodeKey { rung: Decl, file: impl/src/valid.rs, decl: 1, sub: 0, line: 5 } |  |  | 0.529 |
| ns | 6198 |  | 293 | ast.rs: complete from_syn constructor roster | 3.17 |  | 0.521 |
| walker |  | 6214 | 42 | Code::CodeKey { rung: Body, file: impl/src/valid.rs, decl: 2, sub: 0, line: 6 } |  |  | 0.521 |
| ns | 6357 |  | 159 | impl/src/generics.rs: ParamsInScope and InferredBounds | 3.18 |  | 0.531 |
| walker |  | 6427 | 213 | Code::CodeKey { rung: Names, file: impl/src/expand.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.550 |
| walker |  | 6471 | 44 | Code::CodeKey { rung: Decl, file: impl/src/expand.rs, decl: 8, sub: 0, line: 536 } |  |  | 0.550 |
| walker |  | 6486 | 15 | Code::CodeKey { rung: Body, file: impl/src/expand.rs, decl: 9, sub: 0, line: 565 } |  |  | 0.550 |
| walker |  | 6518 | 32 | Code::CodeKey { rung: Body, file: impl/src/expand.rs, decl: 10, sub: 0, line: 569 } |  |  | 0.550 |
| walker |  | 6557 | 39 | Code::CodeKey { rung: Body, file: impl/src/expand.rs, decl: 5, sub: 0, line: 505 } |  |  | 0.550 |
| ns | 6606 |  | 249 | impl/src/unraw.rs: IdentUnraw/MemberUnraw and the raw-identifier rule | 3.19 |  | 0.542 |
| walker |  | 6621 | 64 | Code::CodeKey { rung: Body, file: impl/src/expand.rs, decl: 7, sub: 0, line: 526 } |  |  | 0.542 |
| walker |  | 6689 | 68 | Code::CodeKey { rung: Body, file: impl/src/expand.rs, decl: 2, sub: 0, line: 22 } |  |  | 0.548 |
| walker |  | 6890 | 201 | Code::CodeKey { rung: ModuleDoc, file: src/lib.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.548 |
| ns | 6910 |  | 304 | impl/src/fallback.rs: the invalid-input fallback expansion | 3.20 |  | 0.535 |
| walker |  | 7078 | 188 | Code::CodeKey { rung: Names, file: impl/src/fmt.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.542 |
| walker |  | 7092 | 14 | Code::CodeKey { rung: Decl, file: impl/src/fmt.rs, decl: 4, sub: 0, line: 171 } |  |  | 0.542 |
| ns | 7121 |  | 211 | impl/src/scan_expr.rs: the Input/Action alphabet of the expression scanner | 3.21 |  | 0.529 |
| walker |  | 7124 | 32 | Code::CodeKey { rung: Decl, file: impl/src/fmt.rs, decl: 3, sub: 0, line: 166 } |  |  | 0.537 |
| walker |  | 7157 | 33 | Code::CodeKey { rung: Decl, file: impl/src/fmt.rs, decl: 1, sub: 0, line: 15 } |  |  | 0.537 |
| walker |  | 7350 | 193 | Code::CodeKey { rung: Names, file: impl/src/prop.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.545 |
| walker |  | 7392 | 42 | Code::CodeKey { rung: Decl, file: impl/src/prop.rs, decl: 15, sub: 0, line: 72 } |  |  | 0.548 |
| walker |  | 7434 | 42 | Code::CodeKey { rung: Decl, file: impl/src/prop.rs, decl: 21, sub: 0, line: 127 } |  |  | 0.548 |
| ns | 7477 |  | 356 | expand.rs: the gate line for every conditionally generated impl | 3.22 | 3.1 | 0.537 |
| walker |  | 7495 | 61 | Code::CodeKey { rung: Decl, file: impl/src/prop.rs, decl: 6, sub: 0, line: 25 } |  |  | 0.541 |
| walker |  | 7507 | 12 | Code::CodeKey { rung: Body, file: impl/src/prop.rs, decl: 16, sub: 0, line: 73 } |  |  | 0.541 |
| walker |  | 7601 | 94 | Code::CodeKey { rung: Decl, file: impl/src/prop.rs, decl: 1, sub: 0, line: 6 } |  |  | 0.548 |
| ns | 7637 |  | 160 | src/private.rs: the complete generated-code support surface | 4.1 |  | 0.541 |
| walker |  | 7695 | 94 | Code::CodeKey { rung: Decl, file: impl/src/prop.rs, decl: 10, sub: 0, line: 53 } |  |  | 0.548 |
| walker |  | 7706 | 11 | Code::CodeKey { rung: Body, file: impl/src/prop.rs, decl: 2, sub: 0, line: 7 } |  |  | 0.548 |
| walker |  | 7717 | 11 | Code::CodeKey { rung: Body, file: impl/src/prop.rs, decl: 3, sub: 0, line: 11 } |  |  | 0.548 |
| walker |  | 7728 | 11 | Code::CodeKey { rung: Body, file: impl/src/prop.rs, decl: 11, sub: 0, line: 54 } |  |  | 0.548 |
| walker |  | 7739 | 11 | Code::CodeKey { rung: Body, file: impl/src/prop.rs, decl: 12, sub: 0, line: 58 } |  |  | 0.548 |
| walker |  | 7751 | 12 | Code::CodeKey { rung: Body, file: impl/src/prop.rs, decl: 4, sub: 0, line: 15 } |  |  | 0.548 |
| ns | 7852 |  | 215 | src/aserror.rs: AsDynError and its five blanket/dyn impls | 4.2 |  | 0.547 |
| walker |  | 7986 | 235 | Code::CodeKey { rung: Names, file: impl/src/scan_expr.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.547 |
| walker |  | 8018 | 32 | Code::CodeKey { rung: Decl, file: impl/src/scan_expr.rs, decl: 7, sub: 0, line: 84 } |  |  | 0.547 |
| walker |  | 8056 | 38 | Code::CodeKey { rung: Decl, file: impl/src/scan_expr.rs, decl: 10, sub: 0, line: 104 } |  |  | 0.547 |
| walker |  | 8094 | 38 | Code::CodeKey { rung: Decl, file: impl/src/scan_expr.rs, decl: 12, sub: 0, line: 114 } |  |  | 0.543 |
| ns | 8094 |  | 242 | src/display.rs: AsDisplay and the std-only Path/PathBuf specializations | 4.3 |  | 0.543 |
| walker |  | 8135 | 41 | Code::CodeKey { rung: Decl, file: impl/src/scan_expr.rs, decl: 11, sub: 0, line: 109 } |  |  | 0.543 |
| walker |  | 8177 | 42 | Code::CodeKey { rung: Decl, file: impl/src/scan_expr.rs, decl: 2, sub: 0, line: 25 } |  |  | 0.545 |
| walker |  | 8226 | 49 | Code::CodeKey { rung: Decl, file: impl/src/scan_expr.rs, decl: 8, sub: 0, line: 89 } |  |  | 0.545 |
| walker |  | 8278 | 52 | Code::CodeKey { rung: Decl, file: impl/src/scan_expr.rs, decl: 5, sub: 0, line: 76 } |  |  | 0.545 |
| ns | 8376 |  | 282 | src/provide.rs (ThiserrorProvide) and src/var.rs (Var) end to end | 4.4 |  | 0.543 |
| walker |  | 8383 | 105 | Code::CodeKey { rung: Decl, file: impl/src/scan_expr.rs, decl: 9, sub: 0, line: 95 } |  |  | 0.543 |
| ns | 8489 |  | 113 | impl/src/lib.rs: the version-stamped `private` path token | 4.5 |  | 0.548 |
| walker |  | 8525 | 142 | Code::CodeKey { rung: Decl, file: impl/src/scan_expr.rs, decl: 1, sub: 0, line: 6 } |  |  | 0.572 |
| walker |  | 8690 | 165 | Code::CodeKey { rung: Decl, file: impl/src/scan_expr.rs, decl: 4, sub: 0, line: 63 } |  |  | 0.572 |
| ns | 8738 |  | 249 | build.rs: the generated __private module and the cfg declarations | 4.6 |  | 0.566 |
| ns | 8946 |  | 208 | build.rs: rustc capability decisions, function roster, and the probe file | 4.7 |  | 0.561 |
| ns | 9043 |  | 97 | Complete tests/, tests/no-std/ and .github/ listings | 5.1 |  | 0.571 |
| ns | 9131 |  | 88 | tests/compiletest.rs: the trybuild UI harness in full | 5.2 |  | 0.567 |
| walker |  | 9141 | 451 | Code::CodeKey { rung: Decl, file: impl/src/scan_expr.rs, decl: 3, sub: 0, line: 32 } |  |  | 0.567 |
| ns | 9262 |  | 131 | tests/no-std/test.rs: the no_std smoke test's error types | 5.3 |  | 0.561 |
| ns | 9389 |  | 127 | Cargo.toml: std-feature rationale and dev-dependencies | 5.4 |  | 0.559 |
| walker |  | 9424 | 283 | Code::CodeKey { rung: Names, file: impl/src/scan_expr.rs, decl: 0, sub: 1, line: 0 } |  |  | 0.559 |
| walker |  | 9445 | 21 | Code::CodeKey { rung: Decl, file: impl/src/scan_expr.rs, decl: 17, sub: 0, line: 136 } |  |  | 0.559 |
| walker |  | 9477 | 32 | Code::CodeKey { rung: Decl, file: impl/src/scan_expr.rs, decl: 13, sub: 0, line: 119 } |  |  | 0.559 |
| walker |  | 9509 | 32 | Code::CodeKey { rung: Decl, file: impl/src/scan_expr.rs, decl: 24, sub: 0, line: 187 } |  |  | 0.559 |
| ns | 9515 |  | 126 | impl/Cargo.toml: the proc-macro crate's manifest | 5.5 |  | 0.564 |
| walker |  | 9543 | 34 | Code::CodeKey { rung: Decl, file: impl/src/scan_expr.rs, decl: 15, sub: 0, line: 130 } |  |  | 0.564 |
| walker |  | 9589 | 46 | Code::CodeKey { rung: Decl, file: impl/src/scan_expr.rs, decl: 23, sub: 0, line: 181 } |  |  | 0.564 |
| walker |  | 9636 | 47 | Code::CodeKey { rung: Decl, file: impl/src/scan_expr.rs, decl: 22, sub: 0, line: 175 } |  |  | 0.564 |
| walker |  | 9684 | 48 | Code::CodeKey { rung: Decl, file: impl/src/scan_expr.rs, decl: 14, sub: 0, line: 124 } |  |  | 0.564 |
| ns | 9689 |  | 174 | CI job roster and the pinned toolchain components | 5.6 |  | 0.557 |
| walker |  | 9750 | 66 | Code::CodeKey { rung: Decl, file: impl/src/scan_expr.rs, decl: 19, sub: 0, line: 141 } |  |  | 0.557 |
| walker |  | 9842 | 92 | Code::CodeKey { rung: Decl, file: impl/src/scan_expr.rs, decl: 21, sub: 0, line: 166 } |  |  | 0.557 |
