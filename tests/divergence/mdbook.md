Score(3000)=0.672 I=0.779 C=0.579 ns_rows≤3K=25/63 grid(1000/1442/2080/3000/4327/6240/9000)=0.725/0.595/0.633/0.672/0.633/0.585/0.589

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| ns | 74 |  | 74 | README identity lede | 1.1 |  | 0.000 |
| walker |  | 79 | 79 | Fs::DirListing { dir: . } |  |  | 0.000 |
| walker |  | 86 | 7 | Fs::DirListing { dir: src } |  |  | 0.000 |
| ns | 153 |  | 79 | Repository root listing | 1.2 |  | 0.581 |
| walker |  | 165 | 79 | Markdown::ReadmeHeadline { file: README.md } |  |  | 0.916 |
| walker |  | 177 | 12 | Fs::DirListing { dir: guide } |  |  | 0.916 |
| walker |  | 189 | 12 | Code::CodeKey { rung: ModuleDoc, file: src/main.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.916 |
| walker |  | 197 | 8 | Fs::DirListing { dir: guide/guide-helper } |  |  | 0.916 |
| ns | 201 |  | 48 | The nine workspace crates | 1.3 |  | 0.766 |
| walker |  | 221 | 24 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.766 |
| walker |  | 226 | 5 | Fs::DirListing { dir: .cargo } |  |  | 0.766 |
| walker |  | 234 | 8 | Fs::DirListing { dir: guide/guide-helper/src } |  |  | 0.766 |
| walker |  | 249 | 15 | Code::CodeKey { rung: ModuleDoc, file: guide/guide-helper/src/lib.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.766 |
| walker |  | 264 | 15 | Code::CodeKey { rung: ModuleDoc, file: guide/guide-helper/src/main.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.766 |
| walker |  | 302 | 38 | Fs::DirListing { dir: src/cmd } |  |  | 0.784 |
| walker |  | 311 | 9 | Fs::DirListing { dir: src/cmd/watch } |  |  | 0.791 |
| ns | 349 |  | 148 | Crate-purpose map (driver crate docs), first half | 1.4 |  | 0.671 |
| walker |  | 359 | 48 | Fs::DirListing { dir: crates } |  |  | 0.809 |
| walker |  | 371 | 12 | Fs::DirListing { dir: crates/mdbook-compare } |  |  | 0.809 |
| walker |  | 375 | 4 | Fs::DirListing { dir: crates/mdbook-compare/src } |  |  | 0.809 |
| walker |  | 395 | 20 | Code::CodeKey { rung: ModuleDoc, file: crates/mdbook-compare/src/main.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.809 |
| ns | 403 |  | 54 | Crate-purpose map, `mdbook_core` entry | 1.5 | 1.4 | 0.779 |
| walker |  | 407 | 12 | Fs::DirListing { dir: crates/mdbook-core } |  |  | 0.779 |
| walker |  | 419 | 12 | Fs::DirListing { dir: crates/mdbook-driver } |  |  | 0.779 |
| walker |  | 431 | 12 | Fs::DirListing { dir: crates/mdbook-markdown } |  |  | 0.779 |
| walker |  | 435 | 4 | Fs::DirListing { dir: crates/mdbook-markdown/src } |  |  | 0.779 |
| walker |  | 447 | 12 | Fs::DirListing { dir: crates/mdbook-preprocessor } |  |  | 0.779 |
| walker |  | 451 | 4 | Fs::DirListing { dir: crates/mdbook-preprocessor/src } |  |  | 0.779 |
| ns | 457 |  | 54 | Binary crate source listing (`src/`, `src/cmd/`, `src/cmd/watch/`) | 1.6 |  | 0.793 |
| walker |  | 463 | 12 | Fs::DirListing { dir: crates/mdbook-renderer } |  |  | 0.793 |
| walker |  | 467 | 4 | Fs::DirListing { dir: crates/mdbook-renderer/src } |  |  | 0.794 |
| walker |  | 479 | 12 | Fs::DirListing { dir: crates/mdbook-summary } |  |  | 0.794 |
| walker |  | 483 | 4 | Fs::DirListing { dir: crates/mdbook-summary/src } |  |  | 0.795 |
| walker |  | 495 | 12 | Fs::DirListing { dir: crates/xtask } |  |  | 0.795 |
| walker |  | 511 | 16 | Fs::DirListing { dir: crates/mdbook-html } |  |  | 0.795 |
| walker |  | 520 | 9 | Fs::DirListing { dir: crates/xtask/src } |  |  | 0.798 |
| walker |  | 532 | 12 | Code::CodeKey { rung: ModuleDoc, file: crates/xtask/src/main.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.798 |
| walker |  | 539 | 7 | Fs::DirListing { dir: tests } |  |  | 0.798 |
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
| walker |  | 1447 | 11 | Toml::Dependencies { file: crates/mdbook-compare/Cargo.toml } |  |  | 0.595 |
| walker |  | 1458 | 11 | Toml::Dependencies { file: crates/xtask/Cargo.toml } |  |  | 0.595 |
| walker |  | 1509 | 51 | Json::Dependencies { file: package.json } |  |  | 0.595 |
| walker |  | 1525 | 16 | Code::CodeKey { rung: ModuleDoc, file: src/cmd/watch/native.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.595 |
| walker |  | 1560 | 35 | Fs::DirListing { dir: guide/src/format } |  |  | 0.622 |
| walker |  | 1567 | 7 | Fs::DirListing { dir: guide/src/format/images } |  |  | 0.622 |
| ns | 1573 |  | 153 | Bundled front-end asset listing | 2.8 |  | 0.593 |
| walker |  | 1587 | 20 | Fs::DirListing { dir: guide/src/format/theme } |  |  | 0.606 |
| walker |  | 1604 | 17 | Code::CodeKey { rung: ModuleDoc, file: crates/mdbook-driver/src/mdbook.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.606 |
| ns | 1626 |  | 53 | Examples tree listing | 2.9 |  | 0.594 |
| walker |  | 1628 | 24 | Fs::DirListing { dir: ci } |  |  | 0.594 |
| walker |  | 1667 | 39 | Fs::DirListing { dir: guide/src/cli } |  |  | 0.629 |
| walker |  | 1685 | 18 | Code::CodeKey { rung: ModuleDoc, file: crates/xtask/src/changelog.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.629 |
| walker |  | 1697 | 12 | Code::CodeKey { rung: ModuleDoc, file: crates/mdbook-core/src/utils/mod.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.629 |
| walker |  | 1709 | 12 | Code::CodeKey { rung: ModuleDoc, file: crates/mdbook-driver/src/builtin_preprocessors/mod.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.629 |
| walker |  | 1721 | 12 | Code::CodeKey { rung: ModuleDoc, file: crates/mdbook-html/src/theme/mod.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.629 |
| ns | 1736 |  | 110 | mdbook-core crate root | 3.1 |  | 0.617 |
| walker |  | 1745 | 24 | Fs::DirListing { dir: guide/src/format/configuration } |  |  | 0.639 |
| walker |  | 1758 | 13 | Code::CodeKey { rung: ModuleDoc, file: crates/mdbook-core/src/utils/fs.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.639 |
| walker |  | 1771 | 13 | Code::CodeKey { rung: ModuleDoc, file: crates/mdbook-core/src/utils/html.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.639 |
| walker |  | 1784 | 13 | Code::CodeKey { rung: ModuleDoc, file: crates/mdbook-html/src/html/hide_lines.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.639 |
| walker |  | 1797 | 13 | Code::CodeKey { rung: ModuleDoc, file: crates/mdbook-html/src/html_handlebars/static_files.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.639 |
| walker |  | 1811 | 14 | Code::CodeKey { rung: ModuleDoc, file: crates/mdbook-html/src/theme/playground_editor.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.639 |
| ns | 1867 |  | 131 | mdbook-html crate roots (whole files) | 3.2 |  | 0.621 |
| ns | 1942 |  | 75 | mdbook-driver modules and re-exports | 3.3 | 1.5 | 0.613 |
| ns | 2107 |  | 165 | The `Preprocessor` trait | 3.4 |  | 0.600 |
| walker |  | 2166 | 355 | Toml::Identity { file: Cargo.toml } |  |  | 0.628 |
| walker |  | 2195 | 29 | Fs::DirListing { dir: crates/mdbook-html/front-end/playground_editor } |  |  | 0.639 |
| walker |  | 2210 | 15 | Code::CodeKey { rung: ModuleDoc, file: crates/mdbook-core/src/utils/toml_ext.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.639 |
| walker |  | 2241 | 31 | Fs::DirListing { dir: crates/mdbook-html/front-end/css } |  |  | 0.659 |
| ns | 2259 |  | 152 | `PreprocessorContext` and `parse_input` | 3.5 |  | 0.648 |
| walker |  | 2273 | 32 | Fs::DirListing { dir: crates/mdbook-html/front-end/templates } |  |  | 0.669 |
| walker |  | 2362 | 89 | Json::Scripts { file: package.json } |  |  | 0.669 |
| ns | 2367 |  | 108 | The `Renderer` trait | 3.6 |  | 0.661 |
| walker |  | 2383 | 21 | Toml::Operational { file: crates/mdbook-driver/Cargo.toml } |  |  | 0.661 |
| walker |  | 2391 | 8 | Code::CodeKey { rung: Names, file: guide/guide-helper/src/main.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.661 |
| ns | 2506 |  | 139 | `RenderContext` fields and methods | 3.7 |  | 0.649 |
| walker |  | 2573 | 182 | Code::CodeKey { rung: Decl, file: guide/guide-helper/src/main.rs, decl: 1, sub: 0, line: 3 } |  |  | 0.649 |
| walker |  | 2592 | 19 | Code::CodeKey { rung: ModuleDoc, file: crates/mdbook-html/src/html/serialize.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.649 |
| walker |  | 2616 | 24 | Toml::Operational { file: crates/mdbook-html/Cargo.toml } |  |  | 0.649 |
| walker |  | 2667 | 51 | Code::CodeKey { rung: Names, file: crates/mdbook-html/src/lib.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.655 |
| ns | 2784 |  | 278 | Which markdown extensions mdBook enables | 3.8 |  | 0.634 |
| walker |  | 2799 | 132 | Toml::Operational { file: Cargo.toml } |  |  | 0.646 |
| walker |  | 2823 | 24 | Fs::DirListing { dir: examples/remove-emphasis } |  |  | 0.653 |
| walker |  | 2831 | 8 | Fs::DirListing { dir: examples/remove-emphasis/mdbook-remove-emphasis } |  |  | 0.659 |
| walker |  | 2835 | 4 | Fs::DirListing { dir: examples/remove-emphasis/mdbook-remove-emphasis/src } |  |  | 0.659 |
| walker |  | 2845 | 10 | Fs::DirListing { dir: examples/remove-emphasis/src } |  |  | 0.665 |
| walker |  | 2899 | 54 | Toml::Identity { file: guide/src/for_developers/mdbook-wordcount/Cargo.toml } |  |  | 0.665 |
| walker |  | 2982 | 83 | Code::CodeKey { rung: Names, file: crates/mdbook-renderer/src/lib.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.671 |
| walker |  | 3026 | 44 | Code::CodeKey { rung: Decl, file: crates/mdbook-renderer/src/lib.rs, decl: 1, sub: 0, line: 28 } |  |  | 0.679 |
| ns | 3061 |  | 277 | mdbook-summary public types | 3.9 |  | 0.657 |
| walker |  | 3109 | 83 | Code::CodeKey { rung: Decl, file: crates/mdbook-renderer/src/lib.rs, decl: 5, sub: 0, line: 63 } |  |  | 0.660 |
| walker |  | 3144 | 35 | Code::CodeKey { rung: Decl, file: crates/mdbook-renderer/src/lib.rs, decl: 6, sub: 0, line: 65 } |  |  | 0.660 |
| walker |  | 3155 | 11 | Code::CodeKey { rung: Doc, file: crates/mdbook-renderer/src/lib.rs, decl: 6, sub: 0, line: 65 } |  |  | 0.660 |
| walker |  | 3167 | 12 | Code::CodeKey { rung: Doc, file: crates/mdbook-renderer/src/lib.rs, decl: 2, sub: 0, line: 30 } |  |  | 0.660 |
| ns | 3303 |  | 242 | Subcommand roster with descriptions | 4.1 |  | 0.647 |
| walker |  | 3484 | 317 | Code::CodeKey { rung: Decl, file: crates/mdbook-renderer/src/lib.rs, decl: 4, sub: 0, line: 38 } |  |  | 0.662 |
| walker |  | 3495 | 11 | Code::CodeKey { rung: Doc, file: crates/mdbook-renderer/src/lib.rs, decl: 4, sub: 0, line: 38 } |  |  | 0.662 |
| ns | 3518 |  | 215 | `main()` dispatch | 4.2 |  | 0.650 |
| walker |  | 3570 | 75 | Code::CodeKey { rung: Names, file: crates/mdbook-core/src/lib.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.659 |
| walker |  | 3683 | 113 | Code::CodeKey { rung: Names, file: crates/mdbook-driver/src/lib.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.669 |
| walker |  | 3754 | 71 | Code::CodeKey { rung: Decl, file: crates/mdbook-driver/src/lib.rs, decl: 2, sub: 0, line: 107 } |  |  | 0.669 |
| walker |  | 3769 | 15 | Code::CodeKey { rung: Doc, file: crates/mdbook-driver/src/lib.rs, decl: 2, sub: 0, line: 107 } |  |  | 0.669 |
| ns | 3791 |  | 273 | Shared CLI argument builders | 4.3 |  | 0.651 |
| walker |  | 3885 | 116 | Code::CodeKey { rung: Names, file: crates/mdbook-summary/src/lib.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.653 |
| walker |  | 3903 | 18 | Code::CodeKey { rung: Decl, file: crates/mdbook-summary/src/lib.rs, decl: 6, sub: 0, line: 108 } |  |  | 0.653 |
| walker |  | 3925 | 22 | Code::CodeKey { rung: Decl, file: crates/mdbook-summary/src/lib.rs, decl: 9, sub: 0, line: 140 } |  |  | 0.653 |
| walker |  | 3962 | 37 | Code::CodeKey { rung: Decl, file: crates/mdbook-summary/src/lib.rs, decl: 4, sub: 0, line: 96 } |  |  | 0.653 |
| walker |  | 4052 | 90 | Code::CodeKey { rung: Decl, file: crates/mdbook-summary/src/lib.rs, decl: 8, sub: 0, line: 120 } |  |  | 0.660 |
| ns | 4064 |  | 273 | `init` and `serve` subcommand flags | 4.4 |  | 0.643 |
| walker |  | 4160 | 108 | Code::CodeKey { rung: Decl, file: crates/mdbook-summary/src/lib.rs, decl: 11, sub: 0, line: 173 } |  |  | 0.643 |
| ns | 4296 |  | 232 | `test`, `build`, `clean` and `watch` flags | 4.5 |  | 0.625 |
| walker |  | 4317 | 157 | Code::CodeKey { rung: Decl, file: crates/mdbook-summary/src/lib.rs, decl: 3, sub: 0, line: 82 } |  |  | 0.633 |
| ns | 4483 |  | 187 | CLI function roster | 4.6 |  | 0.620 |
| walker |  | 4487 | 170 | Code::CodeKey { rung: Decl, file: crates/mdbook-summary/src/lib.rs, decl: 2, sub: 0, line: 65 } |  |  | 0.630 |
| walker |  | 4497 | 10 | Code::CodeKey { rung: Doc, file: crates/mdbook-summary/src/lib.rs, decl: 8, sub: 0, line: 120 } |  |  | 0.633 |
| walker |  | 4510 | 13 | Code::CodeKey { rung: Doc, file: crates/mdbook-summary/src/lib.rs, decl: 5, sub: 0, line: 98 } |  |  | 0.633 |
| walker |  | 4527 | 17 | Code::CodeKey { rung: Doc, file: crates/mdbook-renderer/src/lib.rs, decl: 8, sub: 0, line: 86 } |  |  | 0.633 |
| walker |  | 4568 | 41 | Code::CodeKey { rung: Names, file: guide/src/for_developers/mdbook-wordcount/src/main.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.633 |
| walker |  | 4624 | 56 | Code::CodeKey { rung: Decl, file: guide/src/for_developers/mdbook-wordcount/src/main.rs, decl: 3, sub: 0, line: 44 } |  |  | 0.633 |
| ns | 4742 |  | 259 | Clap app assembly and `MDBOOK_LOG` | 4.7 |  | 0.621 |
| ns | 4811 |  | 69 | `Book` — the tree type | 5.1 |  | 0.618 |
| walker |  | 4936 | 312 | Code::CodeKey { rung: Decl, file: guide/src/for_developers/mdbook-wordcount/src/main.rs, decl: 1, sub: 0, line: 12 } |  |  | 0.618 |
| ns | 4938 |  | 127 | `Book` method roster | 5.2 | 5.1 | 0.613 |
| walker |  | 4978 | 42 | Code::CodeKey { rung: Names, file: guide/guide-helper/src/lib.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.613 |
| ns | 5009 |  | 71 | `BookItem` enum | 5.3 |  | 0.608 |
| walker |  | 5029 | 51 | Code::CodeKey { rung: Decl, file: guide/guide-helper/src/lib.rs, decl: 3, sub: 0, line: 35 } |  |  | 0.608 |
| walker |  | 5039 | 10 | Code::CodeKey { rung: Doc, file: guide/guide-helper/src/lib.rs, decl: 1, sub: 0, line: 10 } |  |  | 0.608 |
| walker |  | 5048 | 9 | Code::CodeKey { rung: Body, file: guide/guide-helper/src/lib.rs, decl: 4, sub: 0, line: 36 } |  |  | 0.608 |
| walker |  | 5066 | 18 | Code::CodeKey { rung: Doc, file: crates/mdbook-driver/src/lib.rs, decl: 1, sub: 0, line: 82 } |  |  | 0.608 |
| walker |  | 5084 | 18 | Code::CodeKey { rung: Doc, file: crates/mdbook-renderer/src/lib.rs, decl: 7, sub: 0, line: 81 } |  |  | 0.608 |
| ns | 5150 |  | 141 | `Chapter` — first fields | 5.4 |  | 0.600 |
| walker |  | 5198 | 114 | Code::CodeKey { rung: Names, file: crates/mdbook-preprocessor/src/lib.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.603 |
| walker |  | 5231 | 33 | Code::CodeKey { rung: Decl, file: crates/mdbook-preprocessor/src/lib.rs, decl: 6, sub: 0, line: 68 } |  |  | 0.603 |
| walker |  | 5305 | 74 | Code::CodeKey { rung: Decl, file: crates/mdbook-preprocessor/src/lib.rs, decl: 1, sub: 0, line: 30 } |  |  | 0.611 |
| ns | 5335 |  | 185 | `Chapter` — `path` vs `source_path`, `parent_names` | 5.5 | 5.4 | 0.605 |
| ns | 5450 |  | 115 | `Chapter` methods, `SectionNumber`, `BookItems` | 5.6 |  | 0.600 |
| walker |  | 5523 | 218 | Code::CodeKey { rung: Decl, file: crates/mdbook-preprocessor/src/lib.rs, decl: 5, sub: 0, line: 49 } |  |  | 0.606 |
| walker |  | 5535 | 12 | Code::CodeKey { rung: Doc, file: crates/mdbook-preprocessor/src/lib.rs, decl: 7, sub: 0, line: 70 } |  |  | 0.606 |
| walker |  | 5547 | 12 | Code::CodeKey { rung: Doc, file: crates/mdbook-preprocessor/src/lib.rs, decl: 8, sub: 0, line: 82 } |  |  | 0.609 |
| walker |  | 5561 | 14 | Code::CodeKey { rung: Doc, file: crates/mdbook-preprocessor/src/lib.rs, decl: 2, sub: 0, line: 32 } |  |  | 0.611 |
| ns | 5593 |  | 143 | `Config` — the `book.toml` root | 6.1 |  | 0.604 |
| walker |  | 5678 | 117 | Code::CodeKey { rung: Names, file: crates/mdbook-compare/src/main.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.604 |
| ns | 5731 |  | 138 | `Config` method roster | 6.2 | 6.1 | 0.600 |
| walker |  | 5744 | 66 | Code::CodeKey { rung: Decl, file: crates/mdbook-compare/src/main.rs, decl: 1, sub: 0, line: 6 } |  |  | 0.600 |
| ns | 5944 |  | 213 | `[book]`, `[build]` and `[rust]` keys | 6.3 |  | 0.590 |
| ns | 6056 |  | 112 | `RustEdition` variants | 6.4 |  | 0.585 |
| walker |  | 6090 | 346 | Code::CodeKey { rung: Decl, file: crates/mdbook-compare/src/main.rs, decl: 2, sub: 0, line: 14 } |  |  | 0.585 |
| walker |  | 6125 | 35 | Code::CodeKey { rung: ModuleDoc, file: crates/mdbook-html/src/theme/searcher.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.585 |
| walker |  | 6168 | 43 | Toml::Dependencies { file: crates/mdbook-markdown/Cargo.toml } |  |  | 0.585 |
| ns | 6284 |  | 228 | `[output.html]` keys, first half | 6.5 |  | 0.576 |
| walker |  | 6373 | 205 | Code::CodeKey { rung: Names, file: crates/xtask/src/main.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.576 |
| ns | 6441 |  | 157 | `[output.html]` keys, second half | 6.6 | 6.5 | 0.570 |
| walker |  | 6452 | 79 | Code::CodeKey { rung: Names, file: crates/mdbook-markdown/src/lib.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.571 |
| walker |  | 6471 | 19 | Code::CodeKey { rung: Decl, file: crates/mdbook-markdown/src/lib.rs, decl: 2, sub: 0, line: 33 } |  |  | 0.571 |
| walker |  | 6657 | 186 | Code::CodeKey { rung: Decl, file: crates/mdbook-markdown/src/lib.rs, decl: 1, sub: 0, line: 14 } |  |  | 0.572 |
| walker |  | 6665 | 8 | Code::CodeKey { rung: Doc, file: crates/mdbook-markdown/src/lib.rs, decl: 1, sub: 0, line: 14 } |  |  | 0.572 |
| walker |  | 6682 | 17 | Code::CodeKey { rung: Doc, file: crates/mdbook-markdown/src/lib.rs, decl: 4, sub: 0, line: 44 } |  |  | 0.572 |
| walker |  | 6702 | 20 | Code::CodeKey { rung: Doc, file: crates/mdbook-summary/src/lib.rs, decl: 2, sub: 0, line: 65 } |  |  | 0.572 |
| walker |  | 6740 | 38 | Code::CodeKey { rung: ModuleDoc, file: crates/mdbook-driver/src/builtin_renderers/mod.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.572 |
| ns | 6803 |  | 362 | `[output.html.*]` sub-tables | 6.7 |  | 0.559 |
| walker |  | 6854 | 114 | Code::CodeKey { rung: Names, file: src/main.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.559 |
| walker |  | 6870 | 16 | Code::CodeKey { rung: Doc, file: src/main.rs, decl: 3, sub: 0, line: 58 } |  |  | 0.559 |
| ns | 6912 |  | 109 | `MDBook` struct | 7.1 |  | 0.556 |
| ns | 7134 |  | 222 | `MDBook` method roster | 7.2 | 7.1 | 0.549 |
| walker |  | 7308 | 438 | Code::CodeKey { rung: Decl, file: src/main.rs, decl: 2, sub: 0, line: 18 } |  |  | 0.564 |
| walker |  | 7373 | 65 | Code::CodeKey { rung: ModuleDoc, file: src/cmd/watch/poller.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.564 |
| walker |  | 7424 | 51 | Toml::Dependencies { file: crates/mdbook-preprocessor/Cargo.toml } |  |  | 0.564 |
| walker |  | 7475 | 51 | Toml::Dependencies { file: crates/mdbook-renderer/Cargo.toml } |  |  | 0.564 |
| ns | 7480 |  | 346 | Which plugins run | 7.3 |  | 0.553 |
| walker |  | 7609 | 134 | Fs::DirListing { dir: tests/testsuite } |  |  | 0.601 |
| walker |  | 7662 | 53 | Toml::Dependencies { file: guide/guide-helper/Cargo.toml } |  |  | 0.601 |
| ns | 7726 |  | 246 | Loading a book from disk, and `BookBuilder` | 7.4 |  | 0.594 |
| walker |  | 7903 | 241 | Code::CodeKey { rung: Decl, file: crates/xtask/src/main.rs, decl: 2, sub: 0, line: 13 } |  |  | 0.594 |
| ns | 7970 |  | 244 | The `links` preprocessor's helper syntax | 7.5 |  | 0.588 |
| walker |  | 8221 | 318 | Code::CodeKey { rung: Decl, file: crates/xtask/src/main.rs, decl: 2, sub: 1, line: 13 } |  |  | 0.588 |
| walker |  | 8240 | 19 | Code::CodeKey { rung: Body, file: guide/guide-helper/src/lib.rs, decl: 5, sub: 0, line: 40 } |  |  | 0.588 |
| walker |  | 8267 | 27 | Code::CodeKey { rung: Doc, file: crates/mdbook-preprocessor/src/lib.rs, decl: 5, sub: 0, line: 49 } |  |  | 0.588 |
| ns | 8271 |  | 301 | The other built-in plugins | 7.6 |  | 0.581 |
| walker |  | 8294 | 27 | Code::CodeKey { rung: Doc, file: crates/mdbook-renderer/src/lib.rs, decl: 3, sub: 0, line: 34 } |  |  | 0.581 |
| walker |  | 8344 | 50 | Code::CodeKey { rung: ModuleDoc, file: crates/mdbook-html/src/html/tokenizer.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.581 |
| walker |  | 8375 | 31 | Code::CodeKey { rung: Doc, file: crates/mdbook-preprocessor/src/lib.rs, decl: 3, sub: 0, line: 36 } |  |  | 0.581 |
| walker |  | 8444 | 69 | Toml::Dependencies { file: crates/mdbook-core/Cargo.toml } |  |  | 0.581 |
| walker |  | 8452 | 8 | Code::CodeKey { rung: Body, file: crates/mdbook-preprocessor/src/lib.rs, decl: 4, sub: 0, line: 42 } |  |  | 0.582 |
| ns | 8521 |  | 250 | The markdown-to-HTML pipeline | 8.1 |  | 0.574 |
| walker |  | 8526 | 74 | Toml::Dependencies { file: crates/mdbook-summary/Cargo.toml } |  |  | 0.574 |
| walker |  | 8539 | 13 | Code::CodeKey { rung: Body, file: guide/src/for_developers/mdbook-wordcount/src/main.rs, decl: 2, sub: 0, line: 40 } |  |  | 0.574 |
| ns | 8640 |  | 119 | HTML pipeline entry points | 8.2 | 8.1 | 0.569 |
| walker |  | 8739 | 200 | Code::CodeKey { rung: ModuleDoc, file: crates/mdbook-driver/src/lib.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.573 |
| ns | 8749 |  | 109 | `HtmlHandlebars` and its render steps | 8.3 |  | 0.569 |
| ns | 8893 |  | 144 | Theme resolution | 8.4 |  | 0.565 |
| walker |  | 8923 | 184 | Code::CodeKey { rung: ModuleDoc, file: crates/mdbook-driver/src/lib.rs, decl: 0, sub: 1, line: 0 } |  |  | 0.589 |
| ns | 9021 |  | 128 | Search index, static files and handlebars helpers | 8.5 |  | 0.585 |
| walker |  | 9144 | 221 | Code::CodeKey { rung: ModuleDoc, file: crates/mdbook-driver/src/lib.rs, decl: 0, sub: 2, line: 0 } |  |  | 0.585 |
| ns | 9219 |  | 198 | Testsuite module map and harness convention | 9.1 |  | 0.577 |
| walker |  | 9333 | 189 | Code::CodeKey { rung: ModuleDoc, file: crates/mdbook-driver/src/lib.rs, decl: 0, sub: 3, line: 0 } |  |  | 0.577 |
| ns | 9335 |  | 116 | Snapshot testing with snapbox | 9.2 | 9.1 | 0.577 |
| walker |  | 9410 | 77 | Code::CodeKey { rung: ModuleDoc, file: crates/mdbook-html/src/html/print.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.577 |
| walker |  | 9455 | 45 | Code::CodeKey { rung: Doc, file: crates/mdbook-core/src/lib.rs, decl: 1, sub: 0, line: 7 } |  |  | 0.577 |
| walker |  | 9504 | 49 | Code::CodeKey { rung: Doc, file: crates/mdbook-preprocessor/src/lib.rs, decl: 4, sub: 0, line: 42 } |  |  | 0.577 |
| walker |  | 9662 | 158 | Fs::DirListing { dir: tests/gui } |  |  | 0.599 |
| ns | 9669 |  | 334 | `BookTest` harness API | 9.3 | 9.1 | 0.589 |
| walker |  | 9692 | 30 | Fs::DirListing { dir: tests/gui/books } |  |  | 0.589 |
| walker |  | 9700 | 8 | Fs::DirListing { dir: tests/gui/books/basic } |  |  | 0.589 |
| walker |  | 9708 | 8 | Fs::DirListing { dir: tests/gui/books/heading-nav-folded } |  |  | 0.589 |
| walker |  | 9716 | 8 | Fs::DirListing { dir: tests/gui/books/sidebar-scroll } |  |  | 0.589 |
| walker |  | 9726 | 10 | Fs::DirListing { dir: tests/gui/books/basic/src } |  |  | 0.589 |
| walker |  | 9738 | 12 | Fs::DirListing { dir: tests/gui/books/all-summary } |  |  | 0.589 |
| walker |  | 9750 | 12 | Fs::DirListing { dir: tests/gui/books/heading-nav } |  |  | 0.589 |
| walker |  | 9762 | 12 | Fs::DirListing { dir: tests/gui/books/highlighting } |  |  | 0.589 |
| walker |  | 9770 | 8 | Fs::DirListing { dir: tests/gui/books/highlighting/src } |  |  | 0.589 |
| walker |  | 9782 | 12 | Fs::DirListing { dir: tests/gui/books/redirect } |  |  | 0.589 |
| walker |  | 9794 | 12 | Fs::DirListing { dir: tests/gui/books/search } |  |  | 0.589 |
| walker |  | 9807 | 13 | Fs::DirListing { dir: tests/gui/books/search/src } |  |  | 0.589 |
| walker |  | 9813 | 6 | Fs::DirListing { dir: tests/gui/books/search/src/inner } |  |  | 0.589 |
| ns | 9820 |  | 151 | Contributor workflow: CONTRIBUTING section map | 9.4 |  | 0.584 |
| walker |  | 9829 | 16 | Fs::DirListing { dir: tests/gui/books/heading-nav-folded/src } |  |  | 0.584 |
| walker |  | 9840 | 11 | Fs::DirListing { dir: tests/gui/books/heading-nav-folded/src/sub } |  |  | 0.584 |
| walker |  | 9844 | 4 | Fs::DirListing { dir: tests/gui/books/heading-nav-folded/src/sub/inner } |  |  | 0.584 |
| walker |  | 9866 | 22 | Fs::DirListing { dir: tests/gui/books/redirect/src } |  |  | 0.584 |
| ns | 9884 |  | 64 | CI and repository automation listing | 9.6 |  | 0.585 |
| walker |  | 9908 | 42 | Fs::DirListing { dir: tests/gui/books/all-summary/src } |  |  | 0.585 |
| walker |  | 9914 | 6 | Fs::DirListing { dir: tests/gui/books/all-summary/src/part-1 } |  |  | 0.585 |
| walker |  | 9920 | 6 | Fs::DirListing { dir: tests/gui/books/all-summary/src/part-2 } |  |  | 0.585 |
| ns | 9923 |  | 39 | README tail: licence | 9.7 | 1.1 | 0.585 |
| walker |  | 9967 | 47 | Fs::DirListing { dir: tests/gui/books/heading-nav/src } |  |  | 0.585 |
| walker |  | 9991 | 24 | Fs::DirListing { dir: crates/mdbook-html/front-end/fonts } |  |  | 0.585 |
