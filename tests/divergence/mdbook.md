Score(3000)=0.652 I=0.773 C=0.549 ns_rows≤3K=25/63 grid(1000/1442/2080/3000/4327/6240/9000)=0.691/0.679/0.668/0.652/0.587/0.611/0.565

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| ns | 74 |  | 74 | README identity lede | 1.1 |  | 0.000 |
| walker |  | 79 | 79 | Fs::DirListing { dir: . } |  |  | 0.000 |
| walker |  | 86 | 7 | Fs::DirListing { dir: src } |  |  | 0.000 |
| walker |  | 98 | 12 | Code::CodeKey { rung: ModuleDoc, file: src/main.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.000 |
| walker |  | 110 | 12 | Fs::DirListing { dir: guide } |  |  | 0.000 |
| walker |  | 115 | 5 | Fs::DirListing { dir: .cargo } |  |  | 0.000 |
| walker |  | 153 | 38 | Fs::DirListing { dir: src/cmd } |  |  | 0.610 |
| ns | 153 |  | 79 | Repository root listing | 1.2 |  | 0.610 |
| walker |  | 162 | 9 | Fs::DirListing { dir: src/cmd/watch } |  |  | 0.620 |
| ns | 201 |  | 48 | The nine workspace crates | 1.3 |  | 0.519 |
| walker |  | 210 | 48 | Fs::DirListing { dir: crates } |  |  | 0.661 |
| walker |  | 222 | 12 | Fs::DirListing { dir: crates/mdbook-compare } |  |  | 0.661 |
| walker |  | 226 | 4 | Fs::DirListing { dir: crates/mdbook-compare/src } |  |  | 0.662 |
| walker |  | 238 | 12 | Fs::DirListing { dir: crates/mdbook-core } |  |  | 0.662 |
| walker |  | 250 | 12 | Fs::DirListing { dir: crates/mdbook-driver } |  |  | 0.662 |
| walker |  | 262 | 12 | Fs::DirListing { dir: crates/mdbook-markdown } |  |  | 0.662 |
| walker |  | 266 | 4 | Fs::DirListing { dir: crates/mdbook-markdown/src } |  |  | 0.662 |
| walker |  | 278 | 12 | Fs::DirListing { dir: crates/mdbook-preprocessor } |  |  | 0.662 |
| walker |  | 282 | 4 | Fs::DirListing { dir: crates/mdbook-preprocessor/src } |  |  | 0.663 |
| walker |  | 294 | 12 | Fs::DirListing { dir: crates/mdbook-renderer } |  |  | 0.663 |
| walker |  | 298 | 4 | Fs::DirListing { dir: crates/mdbook-renderer/src } |  |  | 0.664 |
| walker |  | 310 | 12 | Fs::DirListing { dir: crates/mdbook-summary } |  |  | 0.664 |
| walker |  | 314 | 4 | Fs::DirListing { dir: crates/mdbook-summary/src } |  |  | 0.666 |
| walker |  | 326 | 12 | Fs::DirListing { dir: crates/xtask } |  |  | 0.666 |
| walker |  | 342 | 16 | Fs::DirListing { dir: crates/mdbook-html } |  |  | 0.666 |
| ns | 349 |  | 148 | Crate-purpose map (driver crate docs), first half | 1.4 |  | 0.565 |
| walker |  | 362 | 20 | Code::CodeKey { rung: ModuleDoc, file: crates/mdbook-compare/src/main.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.565 |
| walker |  | 371 | 9 | Fs::DirListing { dir: crates/xtask/src } |  |  | 0.568 |
| walker |  | 383 | 12 | Code::CodeKey { rung: ModuleDoc, file: crates/xtask/src/main.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.568 |
| walker |  | 390 | 7 | Fs::DirListing { dir: tests } |  |  | 0.568 |
| ns | 403 |  | 54 | Crate-purpose map, `mdbook_core` entry | 1.5 | 1.4 | 0.547 |
| ns | 457 |  | 54 | Binary crate source listing (`src/`, `src/cmd/`, `src/cmd/watch/`) | 1.6 |  | 0.567 |
| walker |  | 469 | 79 | Markdown::ReadmeHeadline { file: README.md } |  |  | 0.798 |
| walker |  | 493 | 24 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.798 |
| walker |  | 504 | 11 | Fs::DirListing { dir: examples } |  |  | 0.798 |
| walker |  | 522 | 18 | Fs::DirListing { dir: crates/mdbook-core/src } |  |  | 0.801 |
| walker |  | 526 | 4 | Fs::DirListing { dir: crates/mdbook-core/src/book } |  |  | 0.802 |
| walker |  | 544 | 18 | Fs::DirListing { dir: crates/mdbook-core/src/utils } |  |  | 0.810 |
| walker |  | 565 | 21 | Code::CodeKey { rung: ModuleDoc, file: crates/mdbook-core/src/lib.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.741 |
| ns | 565 |  | 108 | Workspace members and root package identity | 1.7 |  | 0.741 |
| walker |  | 584 | 19 | Fs::DirListing { dir: crates/mdbook-html/src } |  |  | 0.742 |
| walker |  | 602 | 18 | Fs::DirListing { dir: crates/mdbook-html/src/theme } |  |  | 0.744 |
| walker |  | 624 | 22 | Fs::DirListing { dir: crates/mdbook-html/src/html_handlebars } |  |  | 0.747 |
| walker |  | 636 | 12 | Code::CodeKey { rung: ModuleDoc, file: crates/mdbook-html/src/lib.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.747 |
| ns | 667 |  | 102 | Cargo features | 1.8 |  | 0.718 |
| walker |  | 670 | 34 | Fs::DirListing { dir: crates/mdbook-html/src/html } |  |  | 0.726 |
| walker |  | 682 | 12 | Fs::DirListing { dir: .github } |  |  | 0.726 |
| walker |  | 696 | 14 | Fs::DirListing { dir: .github/workflows } |  |  | 0.726 |
| ns | 707 |  | 40 | mdbook-core source roster | 2.1 |  | 0.743 |
| walker |  | 733 | 37 | Fs::DirListing { dir: guide/src } |  |  | 0.744 |
| walker |  | 737 | 4 | Fs::DirListing { dir: guide/src/misc } |  |  | 0.744 |
| walker |  | 753 | 16 | Fs::DirListing { dir: guide/src/guide } |  |  | 0.745 |
| walker |  | 773 | 20 | Fs::DirListing { dir: guide/src/for_developers } |  |  | 0.746 |
| ns | 775 |  | 68 | mdbook-driver source roster | 2.2 |  | 0.676 |
| walker |  | 781 | 8 | Fs::DirListing { dir: guide/src/for_developers/mdbook-wordcount } |  |  | 0.676 |
| walker |  | 785 | 4 | Fs::DirListing { dir: guide/src/for_developers/mdbook-wordcount/src } |  |  | 0.676 |
| walker |  | 802 | 17 | Code::CodeKey { rung: ModuleDoc, file: src/cmd/command_prelude.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.676 |
| walker |  | 843 | 41 | Code::CodeKey { rung: Names, file: guide/src/for_developers/mdbook-wordcount/src/main.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.676 |
| ns | 885 |  | 110 | mdbook-html source roster | 2.3 |  | 0.680 |
| walker |  | 899 | 56 | Code::CodeKey { rung: Decl, file: guide/src/for_developers/mdbook-wordcount/src/main.rs, decl: 3, sub: 0, line: 44 } |  |  | 0.680 |
| ns | 914 |  | 29 | Single-file crates and dev-tool crates | 2.4 |  | 0.689 |
| walker |  | 922 | 23 | Fs::DirListing { dir: crates/mdbook-html/front-end } |  |  | 0.689 |
| walker |  | 930 | 8 | Fs::DirListing { dir: crates/mdbook-html/front-end/images } |  |  | 0.690 |
| walker |  | 943 | 13 | Fs::DirListing { dir: crates/mdbook-html/front-end/js } |  |  | 0.690 |
| walker |  | 961 | 18 | Code::CodeKey { rung: ModuleDoc, file: src/cmd/mod.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.690 |
| walker |  | 978 | 17 | Fs::DirListing { dir: crates/mdbook-html/front-end/searcher } |  |  | 0.691 |
| walker |  | 1009 | 31 | Fs::DirListing { dir: crates/mdbook-driver/src } |  |  | 0.706 |
| walker |  | 1013 | 4 | Fs::DirListing { dir: crates/mdbook-driver/src/mdbook } |  |  | 0.711 |
| walker |  | 1022 | 9 | Fs::DirListing { dir: crates/mdbook-driver/src/builtin_renderers } |  |  | 0.722 |
| walker |  | 1041 | 19 | Fs::DirListing { dir: crates/mdbook-driver/src/builtin_preprocessors } |  |  | 0.760 |
| walker |  | 1046 | 5 | Fs::DirListing { dir: crates/mdbook-driver/src/builtin_preprocessors/links } |  |  | 0.769 |
| ns | 1055 |  | 141 | Integration testsuite listing | 2.5 |  | 0.664 |
| walker |  | 1122 | 76 | Code::CodeKey { rung: ModuleDoc, file: crates/mdbook-summary/src/lib.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.664 |
| walker |  | 1157 | 35 | Fs::DirListing { dir: guide/src/format } |  |  | 0.667 |
| walker |  | 1164 | 7 | Fs::DirListing { dir: guide/src/format/images } |  |  | 0.667 |
| walker |  | 1184 | 20 | Fs::DirListing { dir: guide/src/format/theme } |  |  | 0.669 |
| walker |  | 1200 | 16 | Code::CodeKey { rung: ModuleDoc, file: src/cmd/watch/native.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.669 |
| ns | 1213 |  | 158 | Browser GUI test listing | 2.6 |  | 0.626 |
| walker |  | 1224 | 24 | Fs::DirListing { dir: ci } |  |  | 0.626 |
| walker |  | 1263 | 39 | Fs::DirListing { dir: guide/src/cli } |  |  | 0.631 |
| walker |  | 1287 | 24 | Fs::DirListing { dir: guide/src/format/configuration } |  |  | 0.634 |
| walker |  | 1387 | 100 | Code::CodeKey { rung: ModuleDoc, file: crates/mdbook-renderer/src/lib.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.634 |
| ns | 1420 |  | 207 | Guide (user documentation) tree listing | 2.7 |  | 0.679 |
| walker |  | 1490 | 103 | Code::CodeKey { rung: ModuleDoc, file: crates/mdbook-preprocessor/src/lib.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.679 |
| ns | 1573 |  | 153 | Bundled front-end asset listing | 2.8 |  | 0.645 |
| walker |  | 1595 | 105 | Code::CodeKey { rung: ModuleDoc, file: crates/mdbook-markdown/src/lib.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.645 |
| walker |  | 1612 | 17 | Fs::DirListing { dir: crates/mdbook-html/src/html_handlebars/helpers } |  |  | 0.665 |
| walker |  | 1625 | 13 | Code::CodeKey { rung: Body, file: guide/src/for_developers/mdbook-wordcount/src/main.rs, decl: 2, sub: 0, line: 40 } |  |  | 0.665 |
| ns | 1626 |  | 53 | Examples tree listing | 2.9 |  | 0.651 |
| walker |  | 1654 | 29 | Fs::DirListing { dir: crates/mdbook-html/front-end/playground_editor } |  |  | 0.663 |
| walker |  | 1705 | 51 | Markdown::ReadmeHeadline { file: crates/mdbook-compare/README.md } |  |  | 0.663 |
| walker |  | 1736 | 31 | Fs::DirListing { dir: crates/mdbook-html/front-end/css } |  |  | 0.672 |
| ns | 1736 |  | 110 | mdbook-core crate root | 3.1 |  | 0.672 |
| walker |  | 1791 | 55 | Markdown::ReadmeHeadline { file: crates/xtask/README.md } |  |  | 0.672 |
| walker |  | 1823 | 32 | Fs::DirListing { dir: crates/mdbook-html/front-end/templates } |  |  | 0.696 |
| ns | 1867 |  | 131 | mdbook-html crate roots (whole files) | 3.2 |  | 0.676 |
| ns | 1942 |  | 75 | mdbook-driver modules and re-exports | 3.3 | 1.5 | 0.668 |
| ns | 2107 |  | 165 | The `Preprocessor` trait | 3.4 |  | 0.654 |
| walker |  | 2178 | 355 | Toml::Identity { file: Cargo.toml } |  |  | 0.681 |
| walker |  | 2239 | 61 | Markdown::ReadmeHeadline { file: crates/mdbook-renderer/README.md } |  |  | 0.681 |
| ns | 2259 |  | 152 | `PreprocessorContext` and `parse_input` | 3.5 |  | 0.669 |
| walker |  | 2300 | 61 | Markdown::ReadmeHeadline { file: crates/mdbook-summary/README.md } |  |  | 0.669 |
| walker |  | 2311 | 11 | Toml::Dependencies { file: crates/mdbook-compare/Cargo.toml } |  |  | 0.669 |
| walker |  | 2322 | 11 | Toml::Dependencies { file: crates/xtask/Cargo.toml } |  |  | 0.669 |
| ns | 2367 |  | 108 | The `Renderer` trait | 3.6 |  | 0.660 |
| walker |  | 2384 | 62 | Markdown::ReadmeHeadline { file: crates/mdbook-markdown/README.md } |  |  | 0.660 |
| walker |  | 2446 | 62 | Markdown::ReadmeHeadline { file: crates/mdbook-preprocessor/README.md } |  |  | 0.660 |
| ns | 2506 |  | 139 | `RenderContext` fields and methods | 3.7 |  | 0.649 |
| walker |  | 2509 | 63 | Markdown::ReadmeHeadline { file: crates/mdbook-core/README.md } |  |  | 0.649 |
| walker |  | 2623 | 114 | Code::CodeKey { rung: Names, file: src/main.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.649 |
| walker |  | 2639 | 16 | Code::CodeKey { rung: Doc, file: src/main.rs, decl: 3, sub: 0, line: 58 } |  |  | 0.649 |
| walker |  | 2711 | 72 | Markdown::ReadmeHeadline { file: crates/mdbook-html/README.md } |  |  | 0.649 |
| walker |  | 2732 | 21 | Toml::Operational { file: crates/mdbook-driver/Cargo.toml } |  |  | 0.649 |
| walker |  | 2744 | 12 | Code::CodeKey { rung: ModuleDoc, file: crates/mdbook-html/src/utils.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.649 |
| ns | 2784 |  | 278 | Which markdown extensions mdBook enables | 3.8 |  | 0.628 |
| walker |  | 2846 | 102 | Toml::Operational { file: Cargo.toml } |  |  | 0.640 |
| walker |  | 2897 | 51 | Code::CodeKey { rung: Names, file: crates/mdbook-html/src/lib.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.646 |
| walker |  | 2949 | 52 | Code::CodeKey { rung: Names, file: crates/mdbook-core/src/lib.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.652 |
| walker |  | 2959 | 10 | Code::CodeKey { rung: Names, file: guide/src/format/example.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.652 |
| walker |  | 2983 | 24 | Toml::Operational { file: crates/mdbook-html/Cargo.toml } |  |  | 0.652 |
| walker |  | 2997 | 14 | Code::CodeKey { rung: ModuleDoc, file: crates/mdbook-core/src/book.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.652 |
| walker |  | 3011 | 14 | Code::CodeKey { rung: ModuleDoc, file: crates/mdbook-driver/src/init.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.652 |
| walker |  | 3035 | 24 | Fs::DirListing { dir: examples/remove-emphasis } |  |  | 0.659 |
| walker |  | 3045 | 10 | Fs::DirListing { dir: examples/remove-emphasis/src } |  |  | 0.664 |
| ns | 3061 |  | 277 | mdbook-summary public types | 3.9 |  | 0.643 |
| walker |  | 3074 | 29 | Markdown::HeadingsOutline { file: guide/src/SUMMARY.md } |  |  | 0.643 |
| walker |  | 3176 | 102 | Markdown::ReadmeHeadline { file: crates/mdbook-driver/README.md } |  |  | 0.643 |
| walker |  | 3230 | 54 | Toml::Identity { file: guide/src/for_developers/mdbook-wordcount/Cargo.toml } |  |  | 0.643 |
| walker |  | 3247 | 17 | Code::CodeKey { rung: ModuleDoc, file: crates/mdbook-driver/src/mdbook.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.643 |
| walker |  | 3295 | 48 | Markdown::Section { file: crates/mdbook-driver/README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.643 |
| ns | 3303 |  | 242 | Subcommand roster with descriptions | 4.1 |  | 0.630 |
| walker |  | 3343 | 48 | Markdown::Section { file: crates/mdbook-markdown/README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.630 |
| walker |  | 3391 | 48 | Markdown::Section { file: crates/mdbook-preprocessor/README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.630 |
| walker |  | 3439 | 48 | Markdown::Section { file: crates/mdbook-renderer/README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.630 |
| walker |  | 3487 | 48 | Markdown::Section { file: crates/mdbook-summary/README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.630 |
| ns | 3518 |  | 215 | `main()` dispatch | 4.2 |  | 0.619 |
| walker |  | 3537 | 50 | Markdown::Section { file: crates/mdbook-core/README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.619 |
| walker |  | 3587 | 50 | Markdown::Section { file: crates/mdbook-html/README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.619 |
| walker |  | 3605 | 18 | Code::CodeKey { rung: ModuleDoc, file: crates/xtask/src/changelog.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.619 |
| walker |  | 3617 | 12 | Code::CodeKey { rung: ModuleDoc, file: crates/mdbook-core/src/utils/mod.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.619 |
| walker |  | 3629 | 12 | Code::CodeKey { rung: ModuleDoc, file: crates/mdbook-driver/src/builtin_preprocessors/mod.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.619 |
| walker |  | 3641 | 12 | Code::CodeKey { rung: ModuleDoc, file: crates/mdbook-html/src/theme/mod.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.619 |
| walker |  | 3720 | 79 | Code::CodeKey { rung: Names, file: crates/mdbook-markdown/src/lib.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.619 |
| walker |  | 3739 | 19 | Code::CodeKey { rung: Decl, file: crates/mdbook-markdown/src/lib.rs, decl: 2, sub: 0, line: 33 } |  |  | 0.619 |
| walker |  | 3756 | 17 | Code::CodeKey { rung: Doc, file: crates/mdbook-markdown/src/lib.rs, decl: 4, sub: 0, line: 44 } |  |  | 0.619 |
| ns | 3791 |  | 273 | Shared CLI argument builders | 4.3 |  | 0.603 |
| walker |  | 3942 | 186 | Code::CodeKey { rung: Decl, file: crates/mdbook-markdown/src/lib.rs, decl: 1, sub: 0, line: 14 } |  |  | 0.605 |
| walker |  | 3950 | 8 | Code::CodeKey { rung: Doc, file: crates/mdbook-markdown/src/lib.rs, decl: 1, sub: 0, line: 14 } |  |  | 0.605 |
| walker |  | 3963 | 13 | Markdown::Section { file: guide/src/SUMMARY.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.605 |
| walker |  | 4008 | 45 | Code::CodeKey { rung: Doc, file: crates/mdbook-core/src/lib.rs, decl: 1, sub: 0, line: 7 } |  |  | 0.605 |
| walker |  | 4021 | 13 | Code::CodeKey { rung: ModuleDoc, file: crates/mdbook-core/src/utils/fs.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.605 |
| walker |  | 4034 | 13 | Code::CodeKey { rung: ModuleDoc, file: crates/mdbook-core/src/utils/html.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.605 |
| walker |  | 4047 | 13 | Code::CodeKey { rung: ModuleDoc, file: crates/mdbook-html/src/html/hide_lines.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.605 |
| walker |  | 4060 | 13 | Code::CodeKey { rung: ModuleDoc, file: crates/mdbook-html/src/html_handlebars/static_files.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.605 |
| ns | 4064 |  | 273 | `init` and `serve` subcommand flags | 4.4 |  | 0.589 |
| walker |  | 4143 | 83 | Code::CodeKey { rung: Names, file: crates/mdbook-renderer/src/lib.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.594 |
| walker |  | 4187 | 44 | Code::CodeKey { rung: Decl, file: crates/mdbook-renderer/src/lib.rs, decl: 1, sub: 0, line: 28 } |  |  | 0.601 |
| walker |  | 4199 | 12 | Code::CodeKey { rung: Doc, file: crates/mdbook-renderer/src/lib.rs, decl: 2, sub: 0, line: 30 } |  |  | 0.601 |
| walker |  | 4282 | 83 | Code::CodeKey { rung: Decl, file: crates/mdbook-renderer/src/lib.rs, decl: 5, sub: 0, line: 63 } |  |  | 0.603 |
| ns | 4296 |  | 232 | `test`, `build`, `clean` and `watch` flags | 4.5 |  | 0.587 |
| walker |  | 4317 | 35 | Code::CodeKey { rung: Decl, file: crates/mdbook-renderer/src/lib.rs, decl: 6, sub: 0, line: 65 } |  |  | 0.587 |
| walker |  | 4328 | 11 | Code::CodeKey { rung: Doc, file: crates/mdbook-renderer/src/lib.rs, decl: 6, sub: 0, line: 65 } |  |  | 0.587 |
| walker |  | 4345 | 17 | Code::CodeKey { rung: Doc, file: crates/mdbook-renderer/src/lib.rs, decl: 8, sub: 0, line: 86 } |  |  | 0.587 |
| walker |  | 4363 | 18 | Code::CodeKey { rung: Doc, file: crates/mdbook-renderer/src/lib.rs, decl: 7, sub: 0, line: 81 } |  |  | 0.587 |
| walker |  | 4390 | 27 | Code::CodeKey { rung: Doc, file: crates/mdbook-renderer/src/lib.rs, decl: 3, sub: 0, line: 34 } |  |  | 0.587 |
| walker |  | 4404 | 14 | Code::CodeKey { rung: ModuleDoc, file: crates/mdbook-html/src/theme/playground_editor.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.587 |
| walker |  | 4469 | 65 | Code::CodeKey { rung: ModuleDoc, file: src/cmd/watch/poller.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.587 |
| ns | 4483 |  | 187 | CLI function roster | 4.6 |  | 0.575 |
| walker |  | 4484 | 15 | Code::CodeKey { rung: ModuleDoc, file: crates/mdbook-core/src/utils/toml_ext.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.575 |
| walker |  | 4618 | 134 | Fs::DirListing { dir: tests/testsuite } |  |  | 0.637 |
| ns | 4742 |  | 259 | Clap app assembly and `MDBOOK_LOG` | 4.7 |  | 0.626 |
| ns | 4811 |  | 69 | `Book` — the tree type | 5.1 |  | 0.622 |
| walker |  | 4935 | 317 | Code::CodeKey { rung: Decl, file: crates/mdbook-renderer/src/lib.rs, decl: 4, sub: 0, line: 38 } |  |  | 0.635 |
| ns | 4938 |  | 127 | `Book` method roster | 5.2 | 5.1 | 0.630 |
| walker |  | 4946 | 11 | Code::CodeKey { rung: Doc, file: crates/mdbook-renderer/src/lib.rs, decl: 4, sub: 0, line: 38 } |  |  | 0.630 |
| walker |  | 4982 | 36 | Code::CodeKey { rung: Names, file: src/cmd/build.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.630 |
| ns | 5009 |  | 71 | `BookItem` enum | 5.3 |  | 0.624 |
| walker |  | 5018 | 36 | Code::CodeKey { rung: Names, file: src/cmd/test.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.625 |
| walker |  | 5131 | 113 | Code::CodeKey { rung: Names, file: crates/mdbook-driver/src/lib.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.634 |
| ns | 5150 |  | 141 | `Chapter` — first fields | 5.4 |  | 0.626 |
| walker |  | 5202 | 71 | Code::CodeKey { rung: Decl, file: crates/mdbook-driver/src/lib.rs, decl: 2, sub: 0, line: 107 } |  |  | 0.626 |
| walker |  | 5217 | 15 | Code::CodeKey { rung: Doc, file: crates/mdbook-driver/src/lib.rs, decl: 2, sub: 0, line: 107 } |  |  | 0.626 |
| walker |  | 5235 | 18 | Code::CodeKey { rung: Doc, file: crates/mdbook-driver/src/lib.rs, decl: 1, sub: 0, line: 82 } |  |  | 0.626 |
| ns | 5335 |  | 185 | `Chapter` — `path` vs `source_path`, `parent_names` | 5.5 | 5.4 | 0.620 |
| walker |  | 5349 | 114 | Code::CodeKey { rung: Names, file: crates/mdbook-preprocessor/src/lib.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.623 |
| walker |  | 5382 | 33 | Code::CodeKey { rung: Decl, file: crates/mdbook-preprocessor/src/lib.rs, decl: 6, sub: 0, line: 68 } |  |  | 0.623 |
| walker |  | 5394 | 12 | Code::CodeKey { rung: Doc, file: crates/mdbook-preprocessor/src/lib.rs, decl: 7, sub: 0, line: 70 } |  |  | 0.623 |
| walker |  | 5406 | 12 | Code::CodeKey { rung: Doc, file: crates/mdbook-preprocessor/src/lib.rs, decl: 8, sub: 0, line: 82 } |  |  | 0.624 |
| ns | 5450 |  | 115 | `Chapter` methods, `SectionNumber`, `BookItems` | 5.6 |  | 0.619 |
| walker |  | 5480 | 74 | Code::CodeKey { rung: Decl, file: crates/mdbook-preprocessor/src/lib.rs, decl: 1, sub: 0, line: 30 } |  |  | 0.626 |
| walker |  | 5488 | 8 | Code::CodeKey { rung: Body, file: crates/mdbook-preprocessor/src/lib.rs, decl: 4, sub: 0, line: 42 } |  |  | 0.628 |
| walker |  | 5502 | 14 | Code::CodeKey { rung: Doc, file: crates/mdbook-preprocessor/src/lib.rs, decl: 2, sub: 0, line: 32 } |  |  | 0.630 |
| walker |  | 5533 | 31 | Code::CodeKey { rung: Doc, file: crates/mdbook-preprocessor/src/lib.rs, decl: 3, sub: 0, line: 36 } |  |  | 0.630 |
| ns | 5593 |  | 143 | `Config` — the `book.toml` root | 6.1 |  | 0.624 |
| ns | 5731 |  | 138 | `Config` method roster | 6.2 | 6.1 | 0.619 |
| walker |  | 5751 | 218 | Code::CodeKey { rung: Decl, file: crates/mdbook-preprocessor/src/lib.rs, decl: 5, sub: 0, line: 49 } |  |  | 0.626 |
| walker |  | 5778 | 27 | Code::CodeKey { rung: Doc, file: crates/mdbook-preprocessor/src/lib.rs, decl: 5, sub: 0, line: 49 } |  |  | 0.626 |
| walker |  | 5827 | 49 | Code::CodeKey { rung: Doc, file: crates/mdbook-preprocessor/src/lib.rs, decl: 4, sub: 0, line: 42 } |  |  | 0.626 |
| walker |  | 5876 | 49 | Code::CodeKey { rung: Body, file: src/main.rs, decl: 6, sub: 0, line: 139 } |  |  | 0.626 |
| ns | 5944 |  | 213 | `[book]`, `[build]` and `[rust]` keys | 6.3 |  | 0.616 |
| walker |  | 5993 | 117 | Code::CodeKey { rung: Names, file: crates/mdbook-compare/src/main.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.616 |
| ns | 6056 |  | 112 | `RustEdition` variants | 6.4 |  | 0.611 |
| walker |  | 6059 | 66 | Code::CodeKey { rung: Decl, file: crates/mdbook-compare/src/main.rs, decl: 1, sub: 0, line: 6 } |  |  | 0.611 |
| walker |  | 6131 | 72 | Markdown::ReadmeHeadline { file: guide/src/guide/README.md } |  |  | 0.611 |
| walker |  | 6150 | 19 | Code::CodeKey { rung: ModuleDoc, file: crates/mdbook-html/src/html/serialize.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.611 |
| walker |  | 6171 | 21 | Markdown::HeadingsOutline { file: guide/src/format/theme/editor.md } |  |  | 0.611 |
| walker |  | 6216 | 45 | Code::CodeKey { rung: Body, file: crates/mdbook-markdown/src/lib.rs, decl: 3, sub: 0, line: 34 } |  |  | 0.611 |
| ns | 6284 |  | 228 | `[output.html]` keys, first half | 6.5 |  | 0.601 |
| walker |  | 6334 | 118 | Markdown::ReadmeHeadline { file: guide/src/for_developers/README.md } |  |  | 0.601 |
| walker |  | 6361 | 27 | Markdown::HeadingsOutline { file: guide/src/for_developers/README.md } |  |  | 0.601 |
| walker |  | 6416 | 55 | Markdown::HeadingsOutline { file: guide/src/for_developers/preprocessors.md } |  |  | 0.601 |
| ns | 6441 |  | 157 | `[output.html]` keys, second half | 6.6 | 6.5 | 0.595 |
| walker |  | 6532 | 116 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.595 |
| walker |  | 6560 | 28 | Markdown::HeadingsOutline { file: guide/src/format/summary.md } |  |  | 0.595 |
| walker |  | 6603 | 43 | Toml::Dependencies { file: crates/mdbook-markdown/Cargo.toml } |  |  | 0.595 |
| walker |  | 6641 | 38 | Markdown::Section { file: crates/mdbook-core/README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.595 |
| walker |  | 6679 | 38 | Markdown::Section { file: crates/mdbook-driver/README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.595 |
| walker |  | 6717 | 38 | Markdown::Section { file: crates/mdbook-html/README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.595 |
| walker |  | 6755 | 38 | Markdown::Section { file: crates/mdbook-markdown/README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.595 |
| walker |  | 6793 | 38 | Markdown::Section { file: crates/mdbook-preprocessor/README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.595 |
| ns | 6803 |  | 362 | `[output.html.*]` sub-tables | 6.7 |  | 0.581 |
| walker |  | 6831 | 38 | Markdown::Section { file: crates/mdbook-renderer/README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.581 |
| walker |  | 6869 | 38 | Markdown::Section { file: crates/mdbook-summary/README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.581 |
| ns | 6912 |  | 109 | `MDBook` struct | 7.1 |  | 0.578 |
| walker |  | 6932 | 63 | Markdown::ReadmeHeadline { file: guide/src/format/theme/README.md } |  |  | 0.578 |
| walker |  | 7022 | 90 | Code::CodeKey { rung: Doc, file: crates/mdbook-renderer/src/lib.rs, decl: 1, sub: 0, line: 28 } |  |  | 0.578 |
| walker |  | 7054 | 32 | Markdown::HeadingsOutline { file: guide/src/format/mathjax.md } |  |  | 0.578 |
| walker |  | 7086 | 32 | Code::CodeKey { rung: Names, file: src/cmd/watch/poller.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.578 |
| walker |  | 7131 | 45 | Code::CodeKey { rung: Decl, file: src/cmd/watch/poller.rs, decl: 1, sub: 0, line: 18 } |  |  | 0.578 |
| ns | 7134 |  | 222 | `MDBook` method roster | 7.2 | 7.1 | 0.571 |
| walker |  | 7207 | 76 | Markdown::ReadmeHeadline { file: guide/src/format/README.md } |  |  | 0.571 |
| walker |  | 7411 | 204 | Code::CodeKey { rung: Names, file: crates/mdbook-summary/src/lib.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.572 |
| walker |  | 7429 | 18 | Code::CodeKey { rung: Decl, file: crates/mdbook-summary/src/lib.rs, decl: 6, sub: 0, line: 108 } |  |  | 0.572 |
| walker |  | 7451 | 22 | Code::CodeKey { rung: Decl, file: crates/mdbook-summary/src/lib.rs, decl: 9, sub: 0, line: 140 } |  |  | 0.572 |
| ns | 7480 |  | 346 | Which plugins run | 7.3 |  | 0.561 |
| walker |  | 7488 | 37 | Code::CodeKey { rung: Decl, file: crates/mdbook-summary/src/lib.rs, decl: 4, sub: 0, line: 96 } |  |  | 0.561 |
| walker |  | 7499 | 11 | Code::CodeKey { rung: Body, file: crates/mdbook-summary/src/lib.rs, decl: 10, sub: 0, line: 141 } |  |  | 0.561 |
| walker |  | 7512 | 13 | Code::CodeKey { rung: Doc, file: crates/mdbook-summary/src/lib.rs, decl: 5, sub: 0, line: 98 } |  |  | 0.561 |
| walker |  | 7602 | 90 | Code::CodeKey { rung: Decl, file: crates/mdbook-summary/src/lib.rs, decl: 8, sub: 0, line: 120 } |  |  | 0.566 |
| walker |  | 7612 | 10 | Code::CodeKey { rung: Doc, file: crates/mdbook-summary/src/lib.rs, decl: 8, sub: 0, line: 120 } |  |  | 0.567 |
| ns | 7726 |  | 246 | Loading a book from disk, and `BookBuilder` | 7.4 |  | 0.561 |
| walker |  | 7769 | 157 | Code::CodeKey { rung: Decl, file: crates/mdbook-summary/src/lib.rs, decl: 3, sub: 0, line: 82 } |  |  | 0.567 |
| walker |  | 7939 | 170 | Code::CodeKey { rung: Decl, file: crates/mdbook-summary/src/lib.rs, decl: 2, sub: 0, line: 65 } |  |  | 0.575 |
| walker |  | 7959 | 20 | Code::CodeKey { rung: Doc, file: crates/mdbook-summary/src/lib.rs, decl: 2, sub: 0, line: 65 } |  |  | 0.575 |
| ns | 7970 |  | 244 | The `links` preprocessor's helper syntax | 7.5 |  | 0.569 |
| walker |  | 8011 | 52 | Code::CodeKey { rung: Doc, file: crates/mdbook-summary/src/lib.rs, decl: 3, sub: 0, line: 82 } |  |  | 0.569 |
| walker |  | 8119 | 108 | Code::CodeKey { rung: Decl, file: crates/mdbook-summary/src/lib.rs, decl: 11, sub: 0, line: 173 } |  |  | 0.569 |
| walker |  | 8146 | 27 | Code::CodeKey { rung: Doc, file: crates/mdbook-summary/src/lib.rs, decl: 15, sub: 0, line: 634 } |  |  | 0.569 |
| ns | 8271 |  | 301 | The other built-in plugins | 7.6 |  | 0.562 |
| walker |  | 8351 | 205 | Code::CodeKey { rung: Names, file: crates/xtask/src/main.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.562 |
| walker |  | 8374 | 23 | Code::CodeKey { rung: Body, file: crates/xtask/src/main.rs, decl: 8, sub: 0, line: 106 } |  |  | 0.562 |
| walker |  | 8398 | 24 | Code::CodeKey { rung: Body, file: crates/xtask/src/main.rs, decl: 10, sub: 0, line: 116 } |  |  | 0.562 |
| walker |  | 8425 | 27 | Code::CodeKey { rung: Body, file: crates/xtask/src/main.rs, decl: 9, sub: 0, line: 111 } |  |  | 0.562 |
| walker |  | 8506 | 81 | Markdown::HeadingsOutline { file: guide/src/continuous-integration.md } |  |  | 0.562 |
| ns | 8521 |  | 250 | The markdown-to-HTML pipeline | 8.1 |  | 0.554 |
| walker |  | 8557 | 51 | Toml::Dependencies { file: crates/mdbook-preprocessor/Cargo.toml } |  |  | 0.554 |
| walker |  | 8608 | 51 | Toml::Dependencies { file: crates/mdbook-renderer/Cargo.toml } |  |  | 0.554 |
| walker |  | 8627 | 19 | Code::CodeKey { rung: Doc, file: src/cmd/watch/poller.rs, decl: 1, sub: 0, line: 18 } |  |  | 0.554 |
| ns | 8640 |  | 119 | HTML pipeline entry points | 8.2 | 8.1 | 0.550 |
| walker |  | 8676 | 49 | Markdown::HeadingsOutline { file: guide/src/guide/reading.md } |  |  | 0.550 |
| ns | 8749 |  | 109 | `HtmlHandlebars` and its render steps | 8.3 |  | 0.546 |
| walker |  | 8753 | 77 | Markdown::HeadingsOutline { file: guide/src/for_developers/backends.md } |  |  | 0.546 |
| walker |  | 8785 | 32 | Markdown::Section { file: guide/src/404.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.546 |
| ns | 8893 |  | 144 | Theme resolution | 8.4 |  | 0.542 |
| walker |  | 8943 | 158 | Fs::DirListing { dir: tests/gui } |  |  | 0.565 |
| walker |  | 8973 | 30 | Fs::DirListing { dir: tests/gui/books } |  |  | 0.565 |
| walker |  | 8981 | 8 | Fs::DirListing { dir: tests/gui/books/heading-nav-folded } |  |  | 0.565 |
| walker |  | 8989 | 8 | Fs::DirListing { dir: tests/gui/books/sidebar-scroll } |  |  | 0.565 |
| walker |  | 9001 | 12 | Fs::DirListing { dir: tests/gui/books/all-summary } |  |  | 0.565 |
| walker |  | 9013 | 12 | Fs::DirListing { dir: tests/gui/books/heading-nav } |  |  | 0.565 |
| ns | 9021 |  | 128 | Search index, static files and handlebars helpers | 8.5 |  | 0.561 |
| walker |  | 9025 | 12 | Fs::DirListing { dir: tests/gui/books/highlighting } |  |  | 0.561 |
| walker |  | 9033 | 8 | Fs::DirListing { dir: tests/gui/books/highlighting/src } |  |  | 0.561 |
| walker |  | 9045 | 12 | Fs::DirListing { dir: tests/gui/books/redirect } |  |  | 0.561 |
| walker |  | 9057 | 12 | Fs::DirListing { dir: tests/gui/books/search } |  |  | 0.561 |
| walker |  | 9070 | 13 | Fs::DirListing { dir: tests/gui/books/search/src } |  |  | 0.561 |
| walker |  | 9086 | 16 | Fs::DirListing { dir: tests/gui/books/heading-nav-folded/src } |  |  | 0.561 |
| walker |  | 9097 | 11 | Fs::DirListing { dir: tests/gui/books/heading-nav-folded/src/sub } |  |  | 0.561 |
| walker |  | 9119 | 22 | Fs::DirListing { dir: tests/gui/books/redirect/src } |  |  | 0.561 |
| walker |  | 9161 | 42 | Fs::DirListing { dir: tests/gui/books/all-summary/src } |  |  | 0.561 |
| walker |  | 9208 | 47 | Fs::DirListing { dir: tests/gui/books/heading-nav/src } |  |  | 0.561 |
| ns | 9219 |  | 198 | Testsuite module map and harness convention | 9.1 |  | 0.554 |
| ns | 9335 |  | 116 | Snapshot testing with snapbox | 9.2 | 9.1 | 0.553 |
| walker |  | 9559 | 351 | Code::CodeKey { rung: ModuleDoc, file: crates/mdbook-driver/src/lib.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.580 |
| walker |  | 9592 | 33 | Code::CodeKey { rung: Doc, file: crates/mdbook-summary/src/lib.rs, decl: 14, sub: 0, line: 618 } |  |  | 0.580 |
| ns | 9669 |  | 334 | `BookTest` harness API | 9.3 | 9.1 | 0.570 |
| walker |  | 9705 | 113 | Code::CodeKey { rung: Doc, file: crates/mdbook-preprocessor/src/lib.rs, decl: 1, sub: 0, line: 30 } |  |  | 0.570 |
| walker |  | 9790 | 85 | Markdown::Section { file: guide/src/for_developers/README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.570 |
| ns | 9820 |  | 151 | Contributor workflow: CONTRIBUTING section map | 9.4 |  | 0.566 |
| walker |  | 9857 | 67 | Code::CodeKey { rung: Names, file: src/cmd/command_prelude.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.566 |
| walker |  | 9879 | 22 | Code::CodeKey { rung: Decl, file: src/cmd/command_prelude.rs, decl: 7, sub: 0, line: 56 } |  |  | 0.566 |
| ns | 9884 |  | 64 | CI and repository automation listing | 9.6 |  | 0.568 |
| walker |  | 9888 | 9 | Code::CodeKey { rung: Body, file: src/cmd/command_prelude.rs, decl: 8, sub: 0, line: 57 } |  |  | 0.568 |
| ns | 9923 |  | 39 | README tail: licence | 9.7 | 1.1 | 0.568 |
| walker |  | 9976 | 88 | Code::CodeKey { rung: Decl, file: src/cmd/command_prelude.rs, decl: 1, sub: 0, line: 7 } |  |  | 0.569 |
| walker |  | 9995 | 19 | Code::CodeKey { rung: Decl, file: src/cmd/command_prelude.rs, decl: 6, sub: 0, line: 41 } |  |  | 0.569 |
