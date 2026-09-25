Score(3000)=0.677 I=0.781 C=0.587 ns_rows≤3K=25/63 grid(1000/1442/2080/3000/4327/6240/9000)=0.725/0.595/0.633/0.677/0.616/0.583/0.582

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| ns | 74 |  | 74 | README identity lede | 1.1 |  | 0.000 |
| walker |  | 79 | 79 | Fs::DirListing { dir: . } |  |  | 0.000 |
| walker |  | 86 | 7 | Fs::DirListing { dir: src } |  |  | 0.000 |
| walker |  | 98 | 12 | Fs::DirListing { dir: guide } |  |  | 0.000 |
| walker |  | 110 | 12 | Code::CodeKey { rung: ModuleDoc, file: src/main.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.000 |
| walker |  | 118 | 8 | Fs::DirListing { dir: guide/guide-helper } |  |  | 0.000 |
| walker |  | 123 | 5 | Fs::DirListing { dir: .cargo } |  |  | 0.000 |
| walker |  | 131 | 8 | Fs::DirListing { dir: guide/guide-helper/src } |  |  | 0.000 |
| walker |  | 146 | 15 | Code::CodeKey { rung: ModuleDoc, file: guide/guide-helper/src/lib.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.000 |
| ns | 153 |  | 79 | Repository root listing | 1.2 |  | 0.582 |
| walker |  | 161 | 15 | Code::CodeKey { rung: ModuleDoc, file: guide/guide-helper/src/main.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.582 |
| walker |  | 199 | 38 | Fs::DirListing { dir: src/cmd } |  |  | 0.610 |
| ns | 201 |  | 48 | The nine workspace crates | 1.3 |  | 0.510 |
| walker |  | 208 | 9 | Fs::DirListing { dir: src/cmd/watch } |  |  | 0.519 |
| walker |  | 256 | 48 | Fs::DirListing { dir: crates } |  |  | 0.661 |
| walker |  | 268 | 12 | Fs::DirListing { dir: crates/mdbook-compare } |  |  | 0.661 |
| walker |  | 272 | 4 | Fs::DirListing { dir: crates/mdbook-compare/src } |  |  | 0.662 |
| walker |  | 292 | 20 | Code::CodeKey { rung: ModuleDoc, file: crates/mdbook-compare/src/main.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.662 |
| walker |  | 304 | 12 | Fs::DirListing { dir: crates/mdbook-core } |  |  | 0.662 |
| walker |  | 316 | 12 | Fs::DirListing { dir: crates/mdbook-driver } |  |  | 0.662 |
| walker |  | 328 | 12 | Fs::DirListing { dir: crates/mdbook-markdown } |  |  | 0.662 |
| walker |  | 332 | 4 | Fs::DirListing { dir: crates/mdbook-markdown/src } |  |  | 0.662 |
| walker |  | 344 | 12 | Fs::DirListing { dir: crates/mdbook-preprocessor } |  |  | 0.662 |
| walker |  | 348 | 4 | Fs::DirListing { dir: crates/mdbook-preprocessor/src } |  |  | 0.663 |
| ns | 349 |  | 148 | Crate-purpose map (driver crate docs), first half | 1.4 |  | 0.563 |
| walker |  | 360 | 12 | Fs::DirListing { dir: crates/mdbook-renderer } |  |  | 0.563 |
| walker |  | 364 | 4 | Fs::DirListing { dir: crates/mdbook-renderer/src } |  |  | 0.564 |
| walker |  | 376 | 12 | Fs::DirListing { dir: crates/mdbook-summary } |  |  | 0.564 |
| walker |  | 380 | 4 | Fs::DirListing { dir: crates/mdbook-summary/src } |  |  | 0.565 |
| walker |  | 392 | 12 | Fs::DirListing { dir: crates/xtask } |  |  | 0.565 |
| ns | 403 |  | 54 | Crate-purpose map, `mdbook_core` entry | 1.5 | 1.4 | 0.544 |
| walker |  | 408 | 16 | Fs::DirListing { dir: crates/mdbook-html } |  |  | 0.544 |
| walker |  | 417 | 9 | Fs::DirListing { dir: crates/xtask/src } |  |  | 0.547 |
| walker |  | 429 | 12 | Code::CodeKey { rung: ModuleDoc, file: crates/xtask/src/main.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.547 |
| walker |  | 436 | 7 | Fs::DirListing { dir: tests } |  |  | 0.547 |
| ns | 457 |  | 54 | Binary crate source listing (`src/`, `src/cmd/`, `src/cmd/watch/`) | 1.6 |  | 0.567 |
| walker |  | 515 | 79 | Markdown::ReadmeHeadline { file: README.md } |  |  | 0.798 |
| walker |  | 539 | 24 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.798 |
| walker |  | 550 | 11 | Fs::DirListing { dir: examples } |  |  | 0.798 |
| ns | 565 |  | 108 | Workspace members and root package identity | 1.7 |  | 0.730 |
| walker |  | 568 | 18 | Fs::DirListing { dir: crates/mdbook-core/src } |  |  | 0.733 |
| walker |  | 572 | 4 | Fs::DirListing { dir: crates/mdbook-core/src/book } |  |  | 0.734 |
| walker |  | 590 | 18 | Fs::DirListing { dir: crates/mdbook-core/src/utils } |  |  | 0.741 |
| walker |  | 611 | 21 | Code::CodeKey { rung: ModuleDoc, file: crates/mdbook-core/src/lib.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.741 |
| walker |  | 630 | 19 | Fs::DirListing { dir: crates/mdbook-html/src } |  |  | 0.742 |
| walker |  | 642 | 12 | Code::CodeKey { rung: ModuleDoc, file: crates/mdbook-html/src/lib.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.742 |
| walker |  | 660 | 18 | Fs::DirListing { dir: crates/mdbook-html/src/theme } |  |  | 0.744 |
| ns | 667 |  | 102 | Cargo features | 1.8 |  | 0.715 |
| walker |  | 682 | 22 | Fs::DirListing { dir: crates/mdbook-html/src/html_handlebars } |  |  | 0.718 |
| walker |  | 699 | 17 | Fs::DirListing { dir: crates/mdbook-html/src/html_handlebars/helpers } |  |  | 0.721 |
| ns | 707 |  | 40 | mdbook-core source roster | 2.1 |  | 0.738 |
| walker |  | 733 | 34 | Fs::DirListing { dir: crates/mdbook-html/src/html } |  |  | 0.748 |
| walker |  | 745 | 12 | Fs::DirListing { dir: .github } |  |  | 0.748 |
| walker |  | 759 | 14 | Fs::DirListing { dir: .github/workflows } |  |  | 0.748 |
| ns | 775 |  | 68 | mdbook-driver source roster | 2.2 |  | 0.677 |
| walker |  | 796 | 37 | Fs::DirListing { dir: guide/src } |  |  | 0.678 |
| walker |  | 800 | 4 | Fs::DirListing { dir: guide/src/misc } |  |  | 0.678 |
| walker |  | 816 | 16 | Fs::DirListing { dir: guide/src/guide } |  |  | 0.679 |
| walker |  | 836 | 20 | Fs::DirListing { dir: guide/src/for_developers } |  |  | 0.680 |
| walker |  | 844 | 8 | Fs::DirListing { dir: guide/src/for_developers/mdbook-wordcount } |  |  | 0.680 |
| walker |  | 848 | 4 | Fs::DirListing { dir: guide/src/for_developers/mdbook-wordcount/src } |  |  | 0.680 |
| ns | 885 |  | 110 | mdbook-html source roster | 2.3 |  | 0.717 |
| ns | 914 |  | 29 | Single-file crates and dev-tool crates | 2.4 |  | 0.724 |
| walker |  | 924 | 76 | Code::CodeKey { rung: ModuleDoc, file: crates/mdbook-summary/src/lib.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.724 |
| walker |  | 941 | 17 | Code::CodeKey { rung: ModuleDoc, file: src/cmd/command_prelude.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.724 |
| walker |  | 959 | 18 | Code::CodeKey { rung: ModuleDoc, file: src/cmd/mod.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.724 |
| walker |  | 982 | 23 | Fs::DirListing { dir: crates/mdbook-html/front-end } |  |  | 0.724 |
| walker |  | 990 | 8 | Fs::DirListing { dir: crates/mdbook-html/front-end/images } |  |  | 0.725 |
| walker |  | 1003 | 13 | Fs::DirListing { dir: crates/mdbook-html/front-end/js } |  |  | 0.725 |
| walker |  | 1015 | 12 | Code::CodeKey { rung: ModuleDoc, file: crates/mdbook-html/src/utils.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.725 |
| ns | 1055 |  | 141 | Integration testsuite listing | 2.5 |  | 0.626 |
| walker |  | 1115 | 100 | Code::CodeKey { rung: ModuleDoc, file: crates/mdbook-renderer/src/lib.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.626 |
| ns | 1213 |  | 158 | Browser GUI test listing | 2.6 |  | 0.586 |
| walker |  | 1218 | 103 | Code::CodeKey { rung: ModuleDoc, file: crates/mdbook-preprocessor/src/lib.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.586 |
| walker |  | 1235 | 17 | Fs::DirListing { dir: crates/mdbook-html/front-end/searcher } |  |  | 0.587 |
| walker |  | 1340 | 105 | Code::CodeKey { rung: ModuleDoc, file: crates/mdbook-markdown/src/lib.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.587 |
| walker |  | 1354 | 14 | Code::CodeKey { rung: ModuleDoc, file: crates/mdbook-core/src/book.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.587 |
| walker |  | 1385 | 31 | Fs::DirListing { dir: crates/mdbook-driver/src } |  |  | 0.599 |
| walker |  | 1389 | 4 | Fs::DirListing { dir: crates/mdbook-driver/src/mdbook } |  |  | 0.603 |
| walker |  | 1398 | 9 | Fs::DirListing { dir: crates/mdbook-driver/src/builtin_renderers } |  |  | 0.611 |
| walker |  | 1417 | 19 | Fs::DirListing { dir: crates/mdbook-driver/src/builtin_preprocessors } |  |  | 0.641 |
| ns | 1420 |  | 207 | Guide (user documentation) tree listing | 2.7 |  | 0.589 |
| walker |  | 1422 | 5 | Fs::DirListing { dir: crates/mdbook-driver/src/builtin_preprocessors/links } |  |  | 0.595 |
| walker |  | 1436 | 14 | Code::CodeKey { rung: ModuleDoc, file: crates/mdbook-driver/src/init.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.595 |
| walker |  | 1487 | 51 | Json::Dependencies { file: package.json } |  |  | 0.595 |
| walker |  | 1503 | 16 | Code::CodeKey { rung: ModuleDoc, file: src/cmd/watch/native.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.595 |
| walker |  | 1538 | 35 | Fs::DirListing { dir: guide/src/format } |  |  | 0.622 |
| walker |  | 1545 | 7 | Fs::DirListing { dir: guide/src/format/images } |  |  | 0.622 |
| walker |  | 1565 | 20 | Fs::DirListing { dir: guide/src/format/theme } |  |  | 0.636 |
| ns | 1573 |  | 153 | Bundled front-end asset listing | 2.8 |  | 0.606 |
| walker |  | 1582 | 17 | Code::CodeKey { rung: ModuleDoc, file: crates/mdbook-driver/src/mdbook.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.606 |
| walker |  | 1606 | 24 | Fs::DirListing { dir: ci } |  |  | 0.607 |
| ns | 1626 |  | 53 | Examples tree listing | 2.9 |  | 0.594 |
| walker |  | 1645 | 39 | Fs::DirListing { dir: guide/src/cli } |  |  | 0.629 |
| walker |  | 1663 | 18 | Code::CodeKey { rung: ModuleDoc, file: crates/xtask/src/changelog.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.629 |
| walker |  | 1675 | 12 | Code::CodeKey { rung: ModuleDoc, file: crates/mdbook-core/src/utils/mod.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.629 |
| walker |  | 1687 | 12 | Code::CodeKey { rung: ModuleDoc, file: crates/mdbook-driver/src/builtin_preprocessors/mod.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.629 |
| walker |  | 1699 | 12 | Code::CodeKey { rung: ModuleDoc, file: crates/mdbook-html/src/theme/mod.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.629 |
| walker |  | 1723 | 24 | Fs::DirListing { dir: guide/src/format/configuration } |  |  | 0.651 |
| walker |  | 1736 | 13 | Code::CodeKey { rung: ModuleDoc, file: crates/mdbook-core/src/utils/fs.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.639 |
| ns | 1736 |  | 110 | mdbook-core crate root | 3.1 |  | 0.639 |
| walker |  | 1749 | 13 | Code::CodeKey { rung: ModuleDoc, file: crates/mdbook-core/src/utils/html.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.639 |
| walker |  | 1762 | 13 | Code::CodeKey { rung: ModuleDoc, file: crates/mdbook-html/src/html/hide_lines.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.639 |
| walker |  | 1775 | 13 | Code::CodeKey { rung: ModuleDoc, file: crates/mdbook-html/src/html_handlebars/static_files.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.639 |
| walker |  | 1789 | 14 | Code::CodeKey { rung: ModuleDoc, file: crates/mdbook-html/src/theme/playground_editor.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.639 |
| ns | 1867 |  | 131 | mdbook-html crate roots (whole files) | 3.2 |  | 0.621 |
| ns | 1942 |  | 75 | mdbook-driver modules and re-exports | 3.3 | 1.5 | 0.613 |
| ns | 2107 |  | 165 | The `Preprocessor` trait | 3.4 |  | 0.600 |
| walker |  | 2144 | 355 | Toml::Identity { file: Cargo.toml } |  |  | 0.628 |
| walker |  | 2173 | 29 | Fs::DirListing { dir: crates/mdbook-html/front-end/playground_editor } |  |  | 0.639 |
| walker |  | 2188 | 15 | Code::CodeKey { rung: ModuleDoc, file: crates/mdbook-core/src/utils/toml_ext.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.639 |
| walker |  | 2219 | 31 | Fs::DirListing { dir: crates/mdbook-html/front-end/css } |  |  | 0.659 |
| walker |  | 2251 | 32 | Fs::DirListing { dir: crates/mdbook-html/front-end/templates } |  |  | 0.681 |
| ns | 2259 |  | 152 | `PreprocessorContext` and `parse_input` | 3.5 |  | 0.669 |
| walker |  | 2340 | 89 | Json::Scripts { file: package.json } |  |  | 0.669 |
| walker |  | 2361 | 21 | Toml::Operational { file: crates/mdbook-driver/Cargo.toml } |  |  | 0.669 |
| ns | 2367 |  | 108 | The `Renderer` trait | 3.6 |  | 0.661 |
| walker |  | 2369 | 8 | Code::CodeKey { rung: Names, file: guide/guide-helper/src/main.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.661 |
| ns | 2506 |  | 139 | `RenderContext` fields and methods | 3.7 |  | 0.649 |
| walker |  | 2551 | 182 | Code::CodeKey { rung: Decl, file: guide/guide-helper/src/main.rs, decl: 1, sub: 0, line: 3 } |  |  | 0.649 |
| walker |  | 2570 | 19 | Code::CodeKey { rung: ModuleDoc, file: crates/mdbook-html/src/html/serialize.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.649 |
| walker |  | 2594 | 24 | Toml::Operational { file: crates/mdbook-html/Cargo.toml } |  |  | 0.649 |
| walker |  | 2645 | 51 | Code::CodeKey { rung: Names, file: crates/mdbook-html/src/lib.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.655 |
| walker |  | 2777 | 132 | Toml::Operational { file: Cargo.toml } |  |  | 0.668 |
| ns | 2784 |  | 278 | Which markdown extensions mdBook enables | 3.8 |  | 0.646 |
| walker |  | 2801 | 24 | Fs::DirListing { dir: examples/remove-emphasis } |  |  | 0.653 |
| walker |  | 2809 | 8 | Fs::DirListing { dir: examples/remove-emphasis/mdbook-remove-emphasis } |  |  | 0.659 |
| walker |  | 2813 | 4 | Fs::DirListing { dir: examples/remove-emphasis/mdbook-remove-emphasis/src } |  |  | 0.659 |
| walker |  | 2823 | 10 | Fs::DirListing { dir: examples/remove-emphasis/src } |  |  | 0.665 |
| walker |  | 2877 | 54 | Toml::Identity { file: guide/src/for_developers/mdbook-wordcount/Cargo.toml } |  |  | 0.665 |
| walker |  | 2960 | 83 | Code::CodeKey { rung: Names, file: crates/mdbook-renderer/src/lib.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.671 |
| walker |  | 3004 | 44 | Code::CodeKey { rung: Decl, file: crates/mdbook-renderer/src/lib.rs, decl: 1, sub: 0, line: 28 } |  |  | 0.679 |
| ns | 3061 |  | 277 | mdbook-summary public types | 3.9 |  | 0.657 |
| walker |  | 3087 | 83 | Code::CodeKey { rung: Decl, file: crates/mdbook-renderer/src/lib.rs, decl: 5, sub: 0, line: 63 } |  |  | 0.660 |
| walker |  | 3122 | 35 | Code::CodeKey { rung: Decl, file: crates/mdbook-renderer/src/lib.rs, decl: 6, sub: 0, line: 65 } |  |  | 0.660 |
| walker |  | 3133 | 11 | Code::CodeKey { rung: Doc, file: crates/mdbook-renderer/src/lib.rs, decl: 6, sub: 0, line: 65 } |  |  | 0.660 |
| walker |  | 3145 | 12 | Code::CodeKey { rung: Doc, file: crates/mdbook-renderer/src/lib.rs, decl: 2, sub: 0, line: 30 } |  |  | 0.660 |
| ns | 3303 |  | 242 | Subcommand roster with descriptions | 4.1 |  | 0.647 |
| walker |  | 3462 | 317 | Code::CodeKey { rung: Decl, file: crates/mdbook-renderer/src/lib.rs, decl: 4, sub: 0, line: 38 } |  |  | 0.662 |
| walker |  | 3473 | 11 | Code::CodeKey { rung: Doc, file: crates/mdbook-renderer/src/lib.rs, decl: 4, sub: 0, line: 38 } |  |  | 0.662 |
| walker |  | 3484 | 11 | Toml::Dependencies { file: crates/mdbook-compare/Cargo.toml } |  |  | 0.662 |
| walker |  | 3495 | 11 | Toml::Dependencies { file: crates/xtask/Cargo.toml } |  |  | 0.662 |
| ns | 3518 |  | 215 | `main()` dispatch | 4.2 |  | 0.650 |
| walker |  | 3608 | 113 | Code::CodeKey { rung: Names, file: crates/mdbook-driver/src/lib.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.660 |
| walker |  | 3679 | 71 | Code::CodeKey { rung: Decl, file: crates/mdbook-driver/src/lib.rs, decl: 2, sub: 0, line: 107 } |  |  | 0.660 |
| walker |  | 3694 | 15 | Code::CodeKey { rung: Doc, file: crates/mdbook-driver/src/lib.rs, decl: 2, sub: 0, line: 107 } |  |  | 0.660 |
| walker |  | 3746 | 52 | Code::CodeKey { rung: Names, file: crates/mdbook-core/src/lib.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.665 |
| walker |  | 3763 | 17 | Code::CodeKey { rung: Doc, file: crates/mdbook-renderer/src/lib.rs, decl: 8, sub: 0, line: 86 } |  |  | 0.665 |
| ns | 3791 |  | 273 | Shared CLI argument builders | 4.3 |  | 0.647 |
| walker |  | 3804 | 41 | Code::CodeKey { rung: Names, file: guide/src/for_developers/mdbook-wordcount/src/main.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.647 |
| walker |  | 3860 | 56 | Code::CodeKey { rung: Decl, file: guide/src/for_developers/mdbook-wordcount/src/main.rs, decl: 3, sub: 0, line: 44 } |  |  | 0.647 |
| walker |  | 3873 | 13 | Code::CodeKey { rung: Body, file: guide/src/for_developers/mdbook-wordcount/src/main.rs, decl: 2, sub: 0, line: 40 } |  |  | 0.647 |
| ns | 4064 |  | 273 | `init` and `serve` subcommand flags | 4.4 |  | 0.631 |
| walker |  | 4185 | 312 | Code::CodeKey { rung: Decl, file: guide/src/for_developers/mdbook-wordcount/src/main.rs, decl: 1, sub: 0, line: 12 } |  |  | 0.631 |
| walker |  | 4203 | 18 | Code::CodeKey { rung: Doc, file: crates/mdbook-driver/src/lib.rs, decl: 1, sub: 0, line: 82 } |  |  | 0.631 |
| walker |  | 4221 | 18 | Code::CodeKey { rung: Doc, file: crates/mdbook-renderer/src/lib.rs, decl: 7, sub: 0, line: 81 } |  |  | 0.631 |
| ns | 4296 |  | 232 | `test`, `build`, `clean` and `watch` flags | 4.5 |  | 0.613 |
| walker |  | 4335 | 114 | Code::CodeKey { rung: Names, file: crates/mdbook-preprocessor/src/lib.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.617 |
| walker |  | 4368 | 33 | Code::CodeKey { rung: Decl, file: crates/mdbook-preprocessor/src/lib.rs, decl: 6, sub: 0, line: 68 } |  |  | 0.617 |
| walker |  | 4442 | 74 | Code::CodeKey { rung: Decl, file: crates/mdbook-preprocessor/src/lib.rs, decl: 1, sub: 0, line: 30 } |  |  | 0.625 |
| walker |  | 4450 | 8 | Code::CodeKey { rung: Body, file: crates/mdbook-preprocessor/src/lib.rs, decl: 4, sub: 0, line: 42 } |  |  | 0.626 |
| ns | 4483 |  | 187 | CLI function roster | 4.6 |  | 0.614 |
| walker |  | 4668 | 218 | Code::CodeKey { rung: Decl, file: crates/mdbook-preprocessor/src/lib.rs, decl: 5, sub: 0, line: 49 } |  |  | 0.621 |
| walker |  | 4680 | 12 | Code::CodeKey { rung: Doc, file: crates/mdbook-preprocessor/src/lib.rs, decl: 7, sub: 0, line: 70 } |  |  | 0.621 |
| walker |  | 4692 | 12 | Code::CodeKey { rung: Doc, file: crates/mdbook-preprocessor/src/lib.rs, decl: 8, sub: 0, line: 82 } |  |  | 0.623 |
| walker |  | 4706 | 14 | Code::CodeKey { rung: Doc, file: crates/mdbook-preprocessor/src/lib.rs, decl: 2, sub: 0, line: 32 } |  |  | 0.626 |
| ns | 4742 |  | 259 | Clap app assembly and `MDBOOK_LOG` | 4.7 |  | 0.614 |
| walker |  | 4764 | 58 | Code::CodeKey { rung: Names, file: guide/guide-helper/src/lib.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.614 |
| ns | 4811 |  | 69 | `Book` — the tree type | 5.1 |  | 0.611 |
| walker |  | 4813 | 49 | Code::CodeKey { rung: Decl, file: guide/guide-helper/src/lib.rs, decl: 3, sub: 0, line: 35 } |  |  | 0.611 |
| walker |  | 4822 | 9 | Code::CodeKey { rung: Body, file: guide/guide-helper/src/lib.rs, decl: 4, sub: 0, line: 36 } |  |  | 0.611 |
| walker |  | 4832 | 10 | Code::CodeKey { rung: Doc, file: guide/guide-helper/src/lib.rs, decl: 1, sub: 0, line: 10 } |  |  | 0.611 |
| ns | 4938 |  | 127 | `Book` method roster | 5.2 | 5.1 | 0.606 |
| walker |  | 4949 | 117 | Code::CodeKey { rung: Names, file: crates/mdbook-compare/src/main.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.606 |
| ns | 5009 |  | 71 | `BookItem` enum | 5.3 |  | 0.601 |
| walker |  | 5015 | 66 | Code::CodeKey { rung: Decl, file: crates/mdbook-compare/src/main.rs, decl: 1, sub: 0, line: 6 } |  |  | 0.601 |
| ns | 5150 |  | 141 | `Chapter` — first fields | 5.4 |  | 0.593 |
| ns | 5335 |  | 185 | `Chapter` — `path` vs `source_path`, `parent_names` | 5.5 | 5.4 | 0.588 |
| walker |  | 5361 | 346 | Code::CodeKey { rung: Decl, file: crates/mdbook-compare/src/main.rs, decl: 2, sub: 0, line: 14 } |  |  | 0.588 |
| walker |  | 5396 | 35 | Code::CodeKey { rung: ModuleDoc, file: crates/mdbook-html/src/theme/searcher.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.588 |
| walker |  | 5415 | 19 | Code::CodeKey { rung: Body, file: guide/guide-helper/src/lib.rs, decl: 5, sub: 0, line: 40 } |  |  | 0.588 |
| ns | 5450 |  | 115 | `Chapter` methods, `SectionNumber`, `BookItems` | 5.6 |  | 0.584 |
| ns | 5593 |  | 143 | `Config` — the `book.toml` root | 6.1 |  | 0.577 |
| walker |  | 5619 | 204 | Code::CodeKey { rung: Names, file: crates/mdbook-summary/src/lib.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.578 |
| walker |  | 5637 | 18 | Code::CodeKey { rung: Decl, file: crates/mdbook-summary/src/lib.rs, decl: 6, sub: 0, line: 108 } |  |  | 0.578 |
| walker |  | 5659 | 22 | Code::CodeKey { rung: Decl, file: crates/mdbook-summary/src/lib.rs, decl: 9, sub: 0, line: 140 } |  |  | 0.578 |
| walker |  | 5696 | 37 | Code::CodeKey { rung: Decl, file: crates/mdbook-summary/src/lib.rs, decl: 4, sub: 0, line: 96 } |  |  | 0.578 |
| ns | 5731 |  | 138 | `Config` method roster | 6.2 | 6.1 | 0.574 |
| walker |  | 5786 | 90 | Code::CodeKey { rung: Decl, file: crates/mdbook-summary/src/lib.rs, decl: 8, sub: 0, line: 120 } |  |  | 0.580 |
| walker |  | 5894 | 108 | Code::CodeKey { rung: Decl, file: crates/mdbook-summary/src/lib.rs, decl: 11, sub: 0, line: 173 } |  |  | 0.580 |
| ns | 5944 |  | 213 | `[book]`, `[build]` and `[rust]` keys | 6.3 |  | 0.571 |
| walker |  | 6051 | 157 | Code::CodeKey { rung: Decl, file: crates/mdbook-summary/src/lib.rs, decl: 3, sub: 0, line: 82 } |  |  | 0.577 |
| ns | 6056 |  | 112 | `RustEdition` variants | 6.4 |  | 0.572 |
| walker |  | 6221 | 170 | Code::CodeKey { rung: Decl, file: crates/mdbook-summary/src/lib.rs, decl: 2, sub: 0, line: 65 } |  |  | 0.581 |
| walker |  | 6231 | 10 | Code::CodeKey { rung: Doc, file: crates/mdbook-summary/src/lib.rs, decl: 8, sub: 0, line: 120 } |  |  | 0.583 |
| ns | 6284 |  | 228 | `[output.html]` keys, first half | 6.5 |  | 0.574 |
| ns | 6441 |  | 157 | `[output.html]` keys, second half | 6.6 | 6.5 | 0.569 |
| walker |  | 6523 | 292 | Code::CodeKey { rung: Decl, file: crates/mdbook-summary/src/lib.rs, decl: 12, sub: 0, line: 190 } |  |  | 0.569 |
| walker |  | 6536 | 13 | Code::CodeKey { rung: Doc, file: crates/mdbook-summary/src/lib.rs, decl: 5, sub: 0, line: 98 } |  |  | 0.569 |
| walker |  | 6741 | 205 | Code::CodeKey { rung: Names, file: crates/xtask/src/main.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.569 |
| ns | 6803 |  | 362 | `[output.html.*]` sub-tables | 6.7 |  | 0.555 |
| walker |  | 6820 | 79 | Code::CodeKey { rung: Names, file: crates/mdbook-markdown/src/lib.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.556 |
| walker |  | 6839 | 19 | Code::CodeKey { rung: Decl, file: crates/mdbook-markdown/src/lib.rs, decl: 2, sub: 0, line: 33 } |  |  | 0.556 |
| ns | 6912 |  | 109 | `MDBook` struct | 7.1 |  | 0.552 |
| walker |  | 7025 | 186 | Code::CodeKey { rung: Decl, file: crates/mdbook-markdown/src/lib.rs, decl: 1, sub: 0, line: 14 } |  |  | 0.554 |
| walker |  | 7033 | 8 | Code::CodeKey { rung: Doc, file: crates/mdbook-markdown/src/lib.rs, decl: 1, sub: 0, line: 14 } |  |  | 0.554 |
| walker |  | 7050 | 17 | Code::CodeKey { rung: Doc, file: crates/mdbook-markdown/src/lib.rs, decl: 4, sub: 0, line: 44 } |  |  | 0.554 |
| walker |  | 7070 | 20 | Code::CodeKey { rung: Doc, file: crates/mdbook-summary/src/lib.rs, decl: 2, sub: 0, line: 65 } |  |  | 0.554 |
| walker |  | 7108 | 38 | Code::CodeKey { rung: ModuleDoc, file: crates/mdbook-driver/src/builtin_renderers/mod.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.554 |
| ns | 7134 |  | 222 | `MDBook` method roster | 7.2 | 7.1 | 0.547 |
| walker |  | 7222 | 114 | Code::CodeKey { rung: Names, file: src/main.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.547 |
| walker |  | 7238 | 16 | Code::CodeKey { rung: Doc, file: src/main.rs, decl: 3, sub: 0, line: 58 } |  |  | 0.547 |
| ns | 7480 |  | 346 | Which plugins run | 7.3 |  | 0.537 |
| walker |  | 7676 | 438 | Code::CodeKey { rung: Decl, file: src/main.rs, decl: 2, sub: 0, line: 18 } |  |  | 0.551 |
| ns | 7726 |  | 246 | Loading a book from disk, and `BookBuilder` | 7.4 |  | 0.545 |
| walker |  | 7741 | 65 | Code::CodeKey { rung: ModuleDoc, file: src/cmd/watch/poller.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.545 |
| walker |  | 7764 | 23 | Code::CodeKey { rung: Body, file: crates/xtask/src/main.rs, decl: 8, sub: 0, line: 106 } |  |  | 0.545 |
| walker |  | 7898 | 134 | Fs::DirListing { dir: tests/testsuite } |  |  | 0.593 |
| walker |  | 7922 | 24 | Code::CodeKey { rung: Body, file: crates/xtask/src/main.rs, decl: 10, sub: 0, line: 116 } |  |  | 0.593 |
| ns | 7970 |  | 244 | The `links` preprocessor's helper syntax | 7.5 |  | 0.587 |
| walker |  | 8163 | 241 | Code::CodeKey { rung: Decl, file: crates/xtask/src/main.rs, decl: 2, sub: 0, line: 13 } |  |  | 0.587 |
| ns | 8271 |  | 301 | The other built-in plugins | 7.6 |  | 0.579 |
| walker |  | 8481 | 318 | Code::CodeKey { rung: Decl, file: crates/xtask/src/main.rs, decl: 2, sub: 1, line: 13 } |  |  | 0.579 |
| walker |  | 8508 | 27 | Code::CodeKey { rung: Doc, file: crates/mdbook-preprocessor/src/lib.rs, decl: 5, sub: 0, line: 49 } |  |  | 0.579 |
| ns | 8521 |  | 250 | The markdown-to-HTML pipeline | 8.1 |  | 0.571 |
| walker |  | 8535 | 27 | Code::CodeKey { rung: Doc, file: crates/mdbook-renderer/src/lib.rs, decl: 3, sub: 0, line: 34 } |  |  | 0.571 |
| walker |  | 8562 | 27 | Code::CodeKey { rung: Doc, file: crates/mdbook-summary/src/lib.rs, decl: 15, sub: 0, line: 634 } |  |  | 0.571 |
| walker |  | 8612 | 50 | Code::CodeKey { rung: ModuleDoc, file: crates/mdbook-html/src/html/tokenizer.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.571 |
| ns | 8640 |  | 119 | HTML pipeline entry points | 8.2 | 8.1 | 0.567 |
| walker |  | 8643 | 31 | Code::CodeKey { rung: Doc, file: crates/mdbook-preprocessor/src/lib.rs, decl: 3, sub: 0, line: 36 } |  |  | 0.567 |
| walker |  | 8676 | 33 | Code::CodeKey { rung: Doc, file: crates/mdbook-summary/src/lib.rs, decl: 14, sub: 0, line: 618 } |  |  | 0.567 |
| ns | 8749 |  | 109 | `HtmlHandlebars` and its render steps | 8.3 |  | 0.563 |
| walker |  | 8876 | 200 | Code::CodeKey { rung: ModuleDoc, file: crates/mdbook-driver/src/lib.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.567 |
| ns | 8893 |  | 144 | Theme resolution | 8.4 |  | 0.562 |
| ns | 9021 |  | 128 | Search index, static files and handlebars helpers | 8.5 |  | 0.558 |
| walker |  | 9060 | 184 | Code::CodeKey { rung: ModuleDoc, file: crates/mdbook-driver/src/lib.rs, decl: 0, sub: 1, line: 0 } |  |  | 0.582 |
| ns | 9219 |  | 198 | Testsuite module map and harness convention | 9.1 |  | 0.575 |
| walker |  | 9281 | 221 | Code::CodeKey { rung: ModuleDoc, file: crates/mdbook-driver/src/lib.rs, decl: 0, sub: 2, line: 0 } |  |  | 0.575 |
| ns | 9335 |  | 116 | Snapshot testing with snapbox | 9.2 | 9.1 | 0.574 |
| walker |  | 9470 | 189 | Code::CodeKey { rung: ModuleDoc, file: crates/mdbook-driver/src/lib.rs, decl: 0, sub: 3, line: 0 } |  |  | 0.574 |
| walker |  | 9497 | 27 | Code::CodeKey { rung: Body, file: crates/xtask/src/main.rs, decl: 9, sub: 0, line: 111 } |  |  | 0.574 |
| walker |  | 9574 | 77 | Code::CodeKey { rung: ModuleDoc, file: crates/mdbook-html/src/html/print.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.574 |
| walker |  | 9619 | 45 | Code::CodeKey { rung: Doc, file: crates/mdbook-core/src/lib.rs, decl: 1, sub: 0, line: 7 } |  |  | 0.574 |
| walker |  | 9664 | 45 | Code::CodeKey { rung: Body, file: crates/mdbook-compare/src/main.rs, decl: 3, sub: 0, line: 39 } |  |  | 0.574 |
| ns | 9669 |  | 334 | `BookTest` harness API | 9.3 | 9.1 | 0.564 |
| walker |  | 9713 | 49 | Code::CodeKey { rung: Doc, file: crates/mdbook-preprocessor/src/lib.rs, decl: 4, sub: 0, line: 42 } |  |  | 0.564 |
| walker |  | 9762 | 49 | Code::CodeKey { rung: Body, file: src/main.rs, decl: 6, sub: 0, line: 139 } |  |  | 0.564 |
| ns | 9820 |  | 151 | Contributor workflow: CONTRIBUTING section map | 9.4 |  | 0.560 |
| ns | 9884 |  | 64 | CI and repository automation listing | 9.6 |  | 0.561 |
| walker |  | 9920 | 158 | Fs::DirListing { dir: tests/gui } |  |  | 0.583 |
| ns | 9923 |  | 39 | README tail: licence | 9.7 | 1.1 | 0.582 |
| walker |  | 9950 | 30 | Fs::DirListing { dir: tests/gui/books } |  |  | 0.582 |
| walker |  | 9958 | 8 | Fs::DirListing { dir: tests/gui/books/basic } |  |  | 0.582 |
| walker |  | 9966 | 8 | Fs::DirListing { dir: tests/gui/books/heading-nav-folded } |  |  | 0.582 |
| walker |  | 9974 | 8 | Fs::DirListing { dir: tests/gui/books/sidebar-scroll } |  |  | 0.582 |
| walker |  | 9984 | 10 | Fs::DirListing { dir: tests/gui/books/basic/src } |  |  | 0.582 |
| walker |  | 9996 | 12 | Fs::DirListing { dir: tests/gui/books/all-summary } |  |  | 0.582 |
