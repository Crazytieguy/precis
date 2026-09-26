Score(3000)=0.669 I=0.778 C=0.574 ns_rows≤3K=25/63 grid(1000/1442/2080/3000/4327/6240/9000)=0.724/0.559/0.623/0.669/0.630/0.585/0.588

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
| ns | 565 |  | 108 | Workspace members and root package identity | 1.7 |  | 0.730 |
| walker |  | 577 | 45 | Markdown::CommandBlock { file: CONTRIBUTING.md, row: 51 } |  |  | 0.730 |
| walker |  | 584 | 7 | Fs::DirListing { dir: tests } |  |  | 0.730 |
| walker |  | 595 | 11 | Fs::DirListing { dir: examples } |  |  | 0.730 |
| walker |  | 613 | 18 | Fs::DirListing { dir: crates/mdbook-core/src } |  |  | 0.733 |
| walker |  | 617 | 4 | Fs::DirListing { dir: crates/mdbook-core/src/book } |  |  | 0.734 |
| walker |  | 635 | 18 | Fs::DirListing { dir: crates/mdbook-core/src/utils } |  |  | 0.741 |
| walker |  | 656 | 21 | Code::CodeKey { rung: ModuleDoc, file: crates/mdbook-core/src/lib.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.741 |
| ns | 667 |  | 102 | Cargo features | 1.8 |  | 0.712 |
| walker |  | 675 | 19 | Fs::DirListing { dir: crates/mdbook-html/src } |  |  | 0.713 |
| walker |  | 687 | 12 | Code::CodeKey { rung: ModuleDoc, file: crates/mdbook-html/src/lib.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.713 |
| walker |  | 705 | 18 | Fs::DirListing { dir: crates/mdbook-html/src/theme } |  |  | 0.715 |
| ns | 707 |  | 40 | mdbook-core source roster | 2.1 |  | 0.731 |
| walker |  | 727 | 22 | Fs::DirListing { dir: crates/mdbook-html/src/html_handlebars } |  |  | 0.734 |
| walker |  | 744 | 17 | Fs::DirListing { dir: crates/mdbook-html/src/html_handlebars/helpers } |  |  | 0.738 |
| ns | 775 |  | 68 | mdbook-driver source roster | 2.2 |  | 0.668 |
| walker |  | 778 | 34 | Fs::DirListing { dir: crates/mdbook-html/src/html } |  |  | 0.677 |
| walker |  | 790 | 12 | Fs::DirListing { dir: .github } |  |  | 0.677 |
| walker |  | 804 | 14 | Fs::DirListing { dir: .github/workflows } |  |  | 0.677 |
| walker |  | 841 | 37 | Fs::DirListing { dir: guide/src } |  |  | 0.678 |
| walker |  | 845 | 4 | Fs::DirListing { dir: guide/src/misc } |  |  | 0.678 |
| walker |  | 861 | 16 | Fs::DirListing { dir: guide/src/guide } |  |  | 0.679 |
| walker |  | 881 | 20 | Fs::DirListing { dir: guide/src/for_developers } |  |  | 0.680 |
| ns | 885 |  | 110 | mdbook-html source roster | 2.3 |  | 0.717 |
| walker |  | 889 | 8 | Fs::DirListing { dir: guide/src/for_developers/mdbook-wordcount } |  |  | 0.717 |
| walker |  | 893 | 4 | Fs::DirListing { dir: guide/src/for_developers/mdbook-wordcount/src } |  |  | 0.717 |
| ns | 914 |  | 29 | Single-file crates and dev-tool crates | 2.4 |  | 0.724 |
| walker |  | 969 | 76 | Code::CodeKey { rung: ModuleDoc, file: crates/mdbook-summary/src/lib.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.724 |
| walker |  | 986 | 17 | Code::CodeKey { rung: ModuleDoc, file: src/cmd/command_prelude.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.724 |
| walker |  | 1004 | 18 | Code::CodeKey { rung: ModuleDoc, file: src/cmd/mod.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.724 |
| walker |  | 1027 | 23 | Fs::DirListing { dir: crates/mdbook-html/front-end } |  |  | 0.724 |
| walker |  | 1035 | 8 | Fs::DirListing { dir: crates/mdbook-html/front-end/images } |  |  | 0.725 |
| walker |  | 1048 | 13 | Fs::DirListing { dir: crates/mdbook-html/front-end/js } |  |  | 0.725 |
| ns | 1055 |  | 141 | Integration testsuite listing | 2.5 |  | 0.626 |
| walker |  | 1060 | 12 | Code::CodeKey { rung: ModuleDoc, file: crates/mdbook-html/src/utils.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.626 |
| walker |  | 1160 | 100 | Code::CodeKey { rung: ModuleDoc, file: crates/mdbook-renderer/src/lib.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.626 |
| ns | 1213 |  | 158 | Browser GUI test listing | 2.6 |  | 0.586 |
| walker |  | 1263 | 103 | Code::CodeKey { rung: ModuleDoc, file: crates/mdbook-preprocessor/src/lib.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.586 |
| walker |  | 1280 | 17 | Fs::DirListing { dir: crates/mdbook-html/front-end/searcher } |  |  | 0.587 |
| walker |  | 1385 | 105 | Code::CodeKey { rung: ModuleDoc, file: crates/mdbook-markdown/src/lib.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.587 |
| walker |  | 1399 | 14 | Code::CodeKey { rung: ModuleDoc, file: crates/mdbook-core/src/book.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.587 |
| ns | 1420 |  | 207 | Guide (user documentation) tree listing | 2.7 |  | 0.542 |
| walker |  | 1430 | 31 | Fs::DirListing { dir: crates/mdbook-driver/src } |  |  | 0.552 |
| walker |  | 1434 | 4 | Fs::DirListing { dir: crates/mdbook-driver/src/mdbook } |  |  | 0.556 |
| walker |  | 1443 | 9 | Fs::DirListing { dir: crates/mdbook-driver/src/builtin_renderers } |  |  | 0.563 |
| walker |  | 1462 | 19 | Fs::DirListing { dir: crates/mdbook-driver/src/builtin_preprocessors } |  |  | 0.589 |
| walker |  | 1467 | 5 | Fs::DirListing { dir: crates/mdbook-driver/src/builtin_preprocessors/links } |  |  | 0.595 |
| walker |  | 1481 | 14 | Code::CodeKey { rung: ModuleDoc, file: crates/mdbook-driver/src/init.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.595 |
| walker |  | 1492 | 11 | Toml::Dependencies { file: crates/mdbook-compare/Cargo.toml } |  |  | 0.595 |
| walker |  | 1503 | 11 | Toml::Dependencies { file: crates/xtask/Cargo.toml } |  |  | 0.595 |
| walker |  | 1554 | 51 | Json::Dependencies { file: package.json } |  |  | 0.595 |
| ns | 1573 |  | 153 | Bundled front-end asset listing | 2.8 |  | 0.569 |
| walker |  | 1606 | 52 | Json::Scripts { file: package.json } |  |  | 0.569 |
| ns | 1626 |  | 53 | Examples tree listing | 2.9 |  | 0.557 |
| walker |  | 1643 | 37 | Json::ScriptsTail { file: package.json } |  |  | 0.557 |
| walker |  | 1659 | 16 | Code::CodeKey { rung: ModuleDoc, file: src/cmd/watch/native.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.557 |
| walker |  | 1694 | 35 | Fs::DirListing { dir: guide/src/format } |  |  | 0.581 |
| walker |  | 1701 | 7 | Fs::DirListing { dir: guide/src/format/images } |  |  | 0.581 |
| walker |  | 1721 | 20 | Fs::DirListing { dir: guide/src/format/theme } |  |  | 0.594 |
| ns | 1736 |  | 110 | mdbook-core crate root | 3.1 |  | 0.583 |
| walker |  | 1738 | 17 | Code::CodeKey { rung: ModuleDoc, file: crates/mdbook-driver/src/mdbook.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.583 |
| walker |  | 1762 | 24 | Fs::DirListing { dir: ci } |  |  | 0.583 |
| walker |  | 1801 | 39 | Fs::DirListing { dir: guide/src/cli } |  |  | 0.617 |
| walker |  | 1819 | 18 | Code::CodeKey { rung: ModuleDoc, file: crates/xtask/src/changelog.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.617 |
| walker |  | 1831 | 12 | Code::CodeKey { rung: ModuleDoc, file: crates/mdbook-core/src/utils/mod.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.617 |
| walker |  | 1843 | 12 | Code::CodeKey { rung: ModuleDoc, file: crates/mdbook-driver/src/builtin_preprocessors/mod.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.617 |
| walker |  | 1855 | 12 | Code::CodeKey { rung: ModuleDoc, file: crates/mdbook-html/src/theme/mod.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.617 |
| ns | 1867 |  | 131 | mdbook-html crate roots (whole files) | 3.2 |  | 0.600 |
| walker |  | 1879 | 24 | Fs::DirListing { dir: guide/src/format/configuration } |  |  | 0.621 |
| walker |  | 1892 | 13 | Code::CodeKey { rung: ModuleDoc, file: crates/mdbook-core/src/utils/fs.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.621 |
| walker |  | 1905 | 13 | Code::CodeKey { rung: ModuleDoc, file: crates/mdbook-core/src/utils/html.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.621 |
| walker |  | 1918 | 13 | Code::CodeKey { rung: ModuleDoc, file: crates/mdbook-html/src/html/hide_lines.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.621 |
| walker |  | 1931 | 13 | Code::CodeKey { rung: ModuleDoc, file: crates/mdbook-html/src/html_handlebars/static_files.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.621 |
| ns | 1942 |  | 75 | mdbook-driver modules and re-exports | 3.3 | 1.5 | 0.613 |
| walker |  | 1945 | 14 | Code::CodeKey { rung: ModuleDoc, file: crates/mdbook-html/src/theme/playground_editor.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.613 |
| ns | 2107 |  | 165 | The `Preprocessor` trait | 3.4 |  | 0.600 |
| ns | 2259 |  | 152 | `PreprocessorContext` and `parse_input` | 3.5 |  | 0.590 |
| walker |  | 2300 | 355 | Toml::Identity { file: Cargo.toml } |  |  | 0.617 |
| walker |  | 2329 | 29 | Fs::DirListing { dir: crates/mdbook-html/front-end/playground_editor } |  |  | 0.628 |
| walker |  | 2344 | 15 | Code::CodeKey { rung: ModuleDoc, file: crates/mdbook-core/src/utils/toml_ext.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.628 |
| ns | 2367 |  | 108 | The `Renderer` trait | 3.6 |  | 0.620 |
| walker |  | 2375 | 31 | Fs::DirListing { dir: crates/mdbook-html/front-end/css } |  |  | 0.639 |
| walker |  | 2407 | 32 | Fs::DirListing { dir: crates/mdbook-html/front-end/templates } |  |  | 0.661 |
| walker |  | 2428 | 21 | Toml::Operational { file: crates/mdbook-driver/Cargo.toml } |  |  | 0.661 |
| walker |  | 2436 | 8 | Code::CodeKey { rung: Names, file: guide/guide-helper/src/main.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.661 |
| ns | 2506 |  | 139 | `RenderContext` fields and methods | 3.7 |  | 0.649 |
| walker |  | 2618 | 182 | Code::CodeKey { rung: Decl, file: guide/guide-helper/src/main.rs, decl: 1, sub: 0, line: 3 } |  |  | 0.649 |
| walker |  | 2637 | 19 | Code::CodeKey { rung: ModuleDoc, file: crates/mdbook-html/src/html/serialize.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.649 |
| walker |  | 2661 | 24 | Toml::Operational { file: crates/mdbook-html/Cargo.toml } |  |  | 0.649 |
| walker |  | 2712 | 51 | Code::CodeKey { rung: Names, file: crates/mdbook-html/src/lib.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.655 |
| ns | 2784 |  | 278 | Which markdown extensions mdBook enables | 3.8 |  | 0.634 |
| walker |  | 2844 | 132 | Toml::Operational { file: Cargo.toml } |  |  | 0.646 |
| walker |  | 2868 | 24 | Fs::DirListing { dir: examples/remove-emphasis } |  |  | 0.653 |
| walker |  | 2876 | 8 | Fs::DirListing { dir: examples/remove-emphasis/mdbook-remove-emphasis } |  |  | 0.659 |
| walker |  | 2880 | 4 | Fs::DirListing { dir: examples/remove-emphasis/mdbook-remove-emphasis/src } |  |  | 0.659 |
| walker |  | 2890 | 10 | Fs::DirListing { dir: examples/remove-emphasis/src } |  |  | 0.665 |
| walker |  | 2944 | 54 | Toml::Identity { file: guide/src/for_developers/mdbook-wordcount/Cargo.toml } |  |  | 0.665 |
| walker |  | 3027 | 83 | Code::CodeKey { rung: Names, file: crates/mdbook-renderer/src/lib.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.671 |
| ns | 3061 |  | 277 | mdbook-summary public types | 3.9 |  | 0.649 |
| walker |  | 3071 | 44 | Code::CodeKey { rung: Decl, file: crates/mdbook-renderer/src/lib.rs, decl: 1, sub: 0, line: 28 } |  |  | 0.657 |
| walker |  | 3154 | 83 | Code::CodeKey { rung: Decl, file: crates/mdbook-renderer/src/lib.rs, decl: 5, sub: 0, line: 63 } |  |  | 0.660 |
| walker |  | 3189 | 35 | Code::CodeKey { rung: Decl, file: crates/mdbook-renderer/src/lib.rs, decl: 6, sub: 0, line: 65 } |  |  | 0.660 |
| walker |  | 3200 | 11 | Code::CodeKey { rung: Doc, file: crates/mdbook-renderer/src/lib.rs, decl: 6, sub: 0, line: 65 } |  |  | 0.660 |
| walker |  | 3212 | 12 | Code::CodeKey { rung: Doc, file: crates/mdbook-renderer/src/lib.rs, decl: 2, sub: 0, line: 30 } |  |  | 0.660 |
| ns | 3303 |  | 242 | Subcommand roster with descriptions | 4.1 |  | 0.647 |
| ns | 3518 |  | 215 | `main()` dispatch | 4.2 |  | 0.635 |
| walker |  | 3529 | 317 | Code::CodeKey { rung: Decl, file: crates/mdbook-renderer/src/lib.rs, decl: 4, sub: 0, line: 38 } |  |  | 0.650 |
| walker |  | 3540 | 11 | Code::CodeKey { rung: Doc, file: crates/mdbook-renderer/src/lib.rs, decl: 4, sub: 0, line: 38 } |  |  | 0.650 |
| walker |  | 3615 | 75 | Code::CodeKey { rung: Names, file: crates/mdbook-core/src/lib.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.659 |
| walker |  | 3728 | 113 | Code::CodeKey { rung: Names, file: crates/mdbook-driver/src/lib.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.669 |
| ns | 3791 |  | 273 | Shared CLI argument builders | 4.3 |  | 0.651 |
| walker |  | 3799 | 71 | Code::CodeKey { rung: Decl, file: crates/mdbook-driver/src/lib.rs, decl: 2, sub: 0, line: 107 } |  |  | 0.651 |
| walker |  | 3915 | 116 | Code::CodeKey { rung: Names, file: crates/mdbook-summary/src/lib.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.653 |
| walker |  | 3933 | 18 | Code::CodeKey { rung: Decl, file: crates/mdbook-summary/src/lib.rs, decl: 6, sub: 0, line: 108 } |  |  | 0.653 |
| walker |  | 3955 | 22 | Code::CodeKey { rung: Decl, file: crates/mdbook-summary/src/lib.rs, decl: 9, sub: 0, line: 140 } |  |  | 0.653 |
| walker |  | 3992 | 37 | Code::CodeKey { rung: Decl, file: crates/mdbook-summary/src/lib.rs, decl: 4, sub: 0, line: 96 } |  |  | 0.653 |
| ns | 4064 |  | 273 | `init` and `serve` subcommand flags | 4.4 |  | 0.636 |
| walker |  | 4082 | 90 | Code::CodeKey { rung: Decl, file: crates/mdbook-summary/src/lib.rs, decl: 8, sub: 0, line: 120 } |  |  | 0.643 |
| walker |  | 4190 | 108 | Code::CodeKey { rung: Decl, file: crates/mdbook-summary/src/lib.rs, decl: 11, sub: 0, line: 173 } |  |  | 0.643 |
| ns | 4296 |  | 232 | `test`, `build`, `clean` and `watch` flags | 4.5 |  | 0.625 |
| walker |  | 4347 | 157 | Code::CodeKey { rung: Decl, file: crates/mdbook-summary/src/lib.rs, decl: 3, sub: 0, line: 82 } |  |  | 0.633 |
| ns | 4483 |  | 187 | CLI function roster | 4.6 |  | 0.620 |
| walker |  | 4517 | 170 | Code::CodeKey { rung: Decl, file: crates/mdbook-summary/src/lib.rs, decl: 2, sub: 0, line: 65 } |  |  | 0.630 |
| walker |  | 4527 | 10 | Code::CodeKey { rung: Doc, file: crates/mdbook-summary/src/lib.rs, decl: 8, sub: 0, line: 120 } |  |  | 0.633 |
| walker |  | 4540 | 13 | Code::CodeKey { rung: Doc, file: crates/mdbook-summary/src/lib.rs, decl: 5, sub: 0, line: 98 } |  |  | 0.633 |
| walker |  | 4581 | 41 | Code::CodeKey { rung: Names, file: guide/src/for_developers/mdbook-wordcount/src/main.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.633 |
| walker |  | 4637 | 56 | Code::CodeKey { rung: Decl, file: guide/src/for_developers/mdbook-wordcount/src/main.rs, decl: 3, sub: 0, line: 44 } |  |  | 0.633 |
| ns | 4742 |  | 259 | Clap app assembly and `MDBOOK_LOG` | 4.7 |  | 0.621 |
| ns | 4811 |  | 69 | `Book` — the tree type | 5.1 |  | 0.618 |
| ns | 4938 |  | 127 | `Book` method roster | 5.2 | 5.1 | 0.613 |
| walker |  | 4949 | 312 | Code::CodeKey { rung: Decl, file: guide/src/for_developers/mdbook-wordcount/src/main.rs, decl: 1, sub: 0, line: 12 } |  |  | 0.613 |
| walker |  | 4964 | 15 | Code::CodeKey { rung: Doc, file: crates/mdbook-driver/src/lib.rs, decl: 2, sub: 0, line: 107 } |  |  | 0.613 |
| walker |  | 5006 | 42 | Code::CodeKey { rung: Names, file: guide/guide-helper/src/lib.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.613 |
| ns | 5009 |  | 71 | `BookItem` enum | 5.3 |  | 0.608 |
| walker |  | 5057 | 51 | Code::CodeKey { rung: Decl, file: guide/guide-helper/src/lib.rs, decl: 3, sub: 0, line: 35 } |  |  | 0.608 |
| walker |  | 5067 | 10 | Code::CodeKey { rung: Doc, file: guide/guide-helper/src/lib.rs, decl: 1, sub: 0, line: 10 } |  |  | 0.608 |
| walker |  | 5076 | 9 | Code::CodeKey { rung: Body, file: guide/guide-helper/src/lib.rs, decl: 4, sub: 0, line: 36 } |  |  | 0.608 |
| ns | 5150 |  | 141 | `Chapter` — first fields | 5.4 |  | 0.600 |
| walker |  | 5190 | 114 | Code::CodeKey { rung: Names, file: crates/mdbook-preprocessor/src/lib.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.603 |
| walker |  | 5223 | 33 | Code::CodeKey { rung: Decl, file: crates/mdbook-preprocessor/src/lib.rs, decl: 6, sub: 0, line: 68 } |  |  | 0.603 |
| walker |  | 5297 | 74 | Code::CodeKey { rung: Decl, file: crates/mdbook-preprocessor/src/lib.rs, decl: 1, sub: 0, line: 30 } |  |  | 0.611 |
| ns | 5335 |  | 185 | `Chapter` — `path` vs `source_path`, `parent_names` | 5.5 | 5.4 | 0.605 |
| ns | 5450 |  | 115 | `Chapter` methods, `SectionNumber`, `BookItems` | 5.6 |  | 0.600 |
| walker |  | 5515 | 218 | Code::CodeKey { rung: Decl, file: crates/mdbook-preprocessor/src/lib.rs, decl: 5, sub: 0, line: 49 } |  |  | 0.606 |
| walker |  | 5527 | 12 | Code::CodeKey { rung: Doc, file: crates/mdbook-preprocessor/src/lib.rs, decl: 7, sub: 0, line: 70 } |  |  | 0.606 |
| walker |  | 5539 | 12 | Code::CodeKey { rung: Doc, file: crates/mdbook-preprocessor/src/lib.rs, decl: 8, sub: 0, line: 82 } |  |  | 0.609 |
| walker |  | 5553 | 14 | Code::CodeKey { rung: Doc, file: crates/mdbook-preprocessor/src/lib.rs, decl: 2, sub: 0, line: 32 } |  |  | 0.611 |
| ns | 5593 |  | 143 | `Config` — the `book.toml` root | 6.1 |  | 0.604 |
| walker |  | 5670 | 117 | Code::CodeKey { rung: Names, file: crates/mdbook-compare/src/main.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.604 |
| ns | 5731 |  | 138 | `Config` method roster | 6.2 | 6.1 | 0.600 |
| walker |  | 5736 | 66 | Code::CodeKey { rung: Decl, file: crates/mdbook-compare/src/main.rs, decl: 1, sub: 0, line: 6 } |  |  | 0.600 |
| ns | 5944 |  | 213 | `[book]`, `[build]` and `[rust]` keys | 6.3 |  | 0.590 |
| ns | 6056 |  | 112 | `RustEdition` variants | 6.4 |  | 0.585 |
| walker |  | 6082 | 346 | Code::CodeKey { rung: Decl, file: crates/mdbook-compare/src/main.rs, decl: 2, sub: 0, line: 14 } |  |  | 0.585 |
| walker |  | 6117 | 35 | Code::CodeKey { rung: ModuleDoc, file: crates/mdbook-html/src/theme/searcher.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.585 |
| walker |  | 6160 | 43 | Toml::Dependencies { file: crates/mdbook-markdown/Cargo.toml } |  |  | 0.585 |
| walker |  | 6177 | 17 | Code::CodeKey { rung: Doc, file: crates/mdbook-renderer/src/lib.rs, decl: 8, sub: 0, line: 86 } |  |  | 0.585 |
| ns | 6284 |  | 228 | `[output.html]` keys, first half | 6.5 |  | 0.576 |
| walker |  | 6382 | 205 | Code::CodeKey { rung: Names, file: crates/xtask/src/main.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.576 |
| ns | 6441 |  | 157 | `[output.html]` keys, second half | 6.6 | 6.5 | 0.570 |
| walker |  | 6461 | 79 | Code::CodeKey { rung: Names, file: crates/mdbook-markdown/src/lib.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.571 |
| walker |  | 6480 | 19 | Code::CodeKey { rung: Decl, file: crates/mdbook-markdown/src/lib.rs, decl: 2, sub: 0, line: 33 } |  |  | 0.571 |
| walker |  | 6666 | 186 | Code::CodeKey { rung: Decl, file: crates/mdbook-markdown/src/lib.rs, decl: 1, sub: 0, line: 14 } |  |  | 0.572 |
| walker |  | 6674 | 8 | Code::CodeKey { rung: Doc, file: crates/mdbook-markdown/src/lib.rs, decl: 1, sub: 0, line: 14 } |  |  | 0.572 |
| walker |  | 6691 | 17 | Code::CodeKey { rung: Doc, file: crates/mdbook-markdown/src/lib.rs, decl: 4, sub: 0, line: 44 } |  |  | 0.572 |
| walker |  | 6729 | 38 | Code::CodeKey { rung: ModuleDoc, file: crates/mdbook-driver/src/builtin_renderers/mod.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.572 |
| ns | 6803 |  | 362 | `[output.html.*]` sub-tables | 6.7 |  | 0.559 |
| walker |  | 6843 | 114 | Code::CodeKey { rung: Names, file: src/main.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.559 |
| walker |  | 6859 | 16 | Code::CodeKey { rung: Doc, file: src/main.rs, decl: 3, sub: 0, line: 58 } |  |  | 0.559 |
| ns | 6912 |  | 109 | `MDBook` struct | 7.1 |  | 0.556 |
| ns | 7134 |  | 222 | `MDBook` method roster | 7.2 | 7.1 | 0.549 |
| walker |  | 7297 | 438 | Code::CodeKey { rung: Decl, file: src/main.rs, decl: 2, sub: 0, line: 18 } |  |  | 0.564 |
| walker |  | 7315 | 18 | Code::CodeKey { rung: Doc, file: crates/mdbook-driver/src/lib.rs, decl: 1, sub: 0, line: 82 } |  |  | 0.564 |
| walker |  | 7333 | 18 | Code::CodeKey { rung: Doc, file: crates/mdbook-renderer/src/lib.rs, decl: 7, sub: 0, line: 81 } |  |  | 0.564 |
| walker |  | 7398 | 65 | Code::CodeKey { rung: ModuleDoc, file: src/cmd/watch/poller.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.564 |
| walker |  | 7449 | 51 | Toml::Dependencies { file: crates/mdbook-preprocessor/Cargo.toml } |  |  | 0.564 |
| ns | 7480 |  | 346 | Which plugins run | 7.3 |  | 0.553 |
| walker |  | 7500 | 51 | Toml::Dependencies { file: crates/mdbook-renderer/Cargo.toml } |  |  | 0.553 |
| walker |  | 7520 | 20 | Code::CodeKey { rung: Doc, file: crates/mdbook-summary/src/lib.rs, decl: 2, sub: 0, line: 65 } |  |  | 0.553 |
| walker |  | 7654 | 134 | Fs::DirListing { dir: tests/testsuite } |  |  | 0.601 |
| walker |  | 7707 | 53 | Toml::Dependencies { file: guide/guide-helper/Cargo.toml } |  |  | 0.601 |
| ns | 7726 |  | 246 | Loading a book from disk, and `BookBuilder` | 7.4 |  | 0.594 |
| walker |  | 7943 | 236 | Code::CodeKey { rung: Decl, file: crates/xtask/src/main.rs, decl: 2, sub: 0, line: 13 } |  |  | 0.594 |
| ns | 7970 |  | 244 | The `links` preprocessor's helper syntax | 7.5 |  | 0.588 |
| walker |  | 8266 | 323 | Code::CodeKey { rung: Decl, file: crates/xtask/src/main.rs, decl: 2, sub: 1, line: 13 } |  |  | 0.588 |
| ns | 8271 |  | 301 | The other built-in plugins | 7.6 |  | 0.581 |
| walker |  | 8316 | 50 | Code::CodeKey { rung: ModuleDoc, file: crates/mdbook-html/src/html/tokenizer.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.581 |
| walker |  | 8385 | 69 | Toml::Dependencies { file: crates/mdbook-core/Cargo.toml } |  |  | 0.581 |
| walker |  | 8404 | 19 | Code::CodeKey { rung: Body, file: guide/guide-helper/src/lib.rs, decl: 5, sub: 0, line: 40 } |  |  | 0.581 |
| walker |  | 8431 | 27 | Code::CodeKey { rung: Doc, file: crates/mdbook-preprocessor/src/lib.rs, decl: 5, sub: 0, line: 49 } |  |  | 0.581 |
| walker |  | 8458 | 27 | Code::CodeKey { rung: Doc, file: crates/mdbook-renderer/src/lib.rs, decl: 3, sub: 0, line: 34 } |  |  | 0.581 |
| ns | 8521 |  | 250 | The markdown-to-HTML pipeline | 8.1 |  | 0.572 |
| walker |  | 8532 | 74 | Toml::Dependencies { file: crates/mdbook-summary/Cargo.toml } |  |  | 0.572 |
| ns | 8640 |  | 119 | HTML pipeline entry points | 8.2 | 8.1 | 0.568 |
| walker |  | 8732 | 200 | Code::CodeKey { rung: ModuleDoc, file: crates/mdbook-driver/src/lib.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.571 |
| ns | 8749 |  | 109 | `HtmlHandlebars` and its render steps | 8.3 |  | 0.568 |
| ns | 8893 |  | 144 | Theme resolution | 8.4 |  | 0.564 |
| walker |  | 8916 | 184 | Code::CodeKey { rung: ModuleDoc, file: crates/mdbook-driver/src/lib.rs, decl: 0, sub: 1, line: 0 } |  |  | 0.588 |
| ns | 9021 |  | 128 | Search index, static files and handlebars helpers | 8.5 |  | 0.584 |
| walker |  | 9137 | 221 | Code::CodeKey { rung: ModuleDoc, file: crates/mdbook-driver/src/lib.rs, decl: 0, sub: 2, line: 0 } |  |  | 0.584 |
| walker |  | 9168 | 31 | Code::CodeKey { rung: Doc, file: crates/mdbook-preprocessor/src/lib.rs, decl: 3, sub: 0, line: 36 } |  |  | 0.584 |
| ns | 9219 |  | 198 | Testsuite module map and harness convention | 9.1 |  | 0.576 |
| ns | 9335 |  | 116 | Snapshot testing with snapbox | 9.2 | 9.1 | 0.575 |
| walker |  | 9357 | 189 | Code::CodeKey { rung: ModuleDoc, file: crates/mdbook-driver/src/lib.rs, decl: 0, sub: 3, line: 0 } |  |  | 0.575 |
| walker |  | 9365 | 8 | Code::CodeKey { rung: Body, file: crates/mdbook-preprocessor/src/lib.rs, decl: 4, sub: 0, line: 42 } |  |  | 0.577 |
| walker |  | 9378 | 13 | Code::CodeKey { rung: Body, file: guide/src/for_developers/mdbook-wordcount/src/main.rs, decl: 2, sub: 0, line: 40 } |  |  | 0.577 |
| walker |  | 9455 | 77 | Code::CodeKey { rung: ModuleDoc, file: crates/mdbook-html/src/html/print.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.577 |
| walker |  | 9613 | 158 | Fs::DirListing { dir: tests/gui } |  |  | 0.599 |
| walker |  | 9643 | 30 | Fs::DirListing { dir: tests/gui/books } |  |  | 0.599 |
| walker |  | 9651 | 8 | Fs::DirListing { dir: tests/gui/books/basic } |  |  | 0.599 |
| walker |  | 9659 | 8 | Fs::DirListing { dir: tests/gui/books/heading-nav-folded } |  |  | 0.599 |
| walker |  | 9667 | 8 | Fs::DirListing { dir: tests/gui/books/sidebar-scroll } |  |  | 0.599 |
| ns | 9669 |  | 334 | `BookTest` harness API | 9.3 | 9.1 | 0.589 |
| walker |  | 9677 | 10 | Fs::DirListing { dir: tests/gui/books/basic/src } |  |  | 0.589 |
| walker |  | 9689 | 12 | Fs::DirListing { dir: tests/gui/books/all-summary } |  |  | 0.589 |
| walker |  | 9701 | 12 | Fs::DirListing { dir: tests/gui/books/heading-nav } |  |  | 0.589 |
| walker |  | 9713 | 12 | Fs::DirListing { dir: tests/gui/books/highlighting } |  |  | 0.589 |
| walker |  | 9721 | 8 | Fs::DirListing { dir: tests/gui/books/highlighting/src } |  |  | 0.589 |
| walker |  | 9733 | 12 | Fs::DirListing { dir: tests/gui/books/redirect } |  |  | 0.589 |
| walker |  | 9745 | 12 | Fs::DirListing { dir: tests/gui/books/search } |  |  | 0.589 |
| walker |  | 9758 | 13 | Fs::DirListing { dir: tests/gui/books/search/src } |  |  | 0.589 |
| walker |  | 9764 | 6 | Fs::DirListing { dir: tests/gui/books/search/src/inner } |  |  | 0.589 |
| walker |  | 9780 | 16 | Fs::DirListing { dir: tests/gui/books/heading-nav-folded/src } |  |  | 0.589 |
| walker |  | 9791 | 11 | Fs::DirListing { dir: tests/gui/books/heading-nav-folded/src/sub } |  |  | 0.589 |
| walker |  | 9795 | 4 | Fs::DirListing { dir: tests/gui/books/heading-nav-folded/src/sub/inner } |  |  | 0.589 |
| walker |  | 9817 | 22 | Fs::DirListing { dir: tests/gui/books/redirect/src } |  |  | 0.589 |
| ns | 9820 |  | 151 | Contributor workflow: CONTRIBUTING section map | 9.4 |  | 0.584 |
| walker |  | 9859 | 42 | Fs::DirListing { dir: tests/gui/books/all-summary/src } |  |  | 0.584 |
| walker |  | 9865 | 6 | Fs::DirListing { dir: tests/gui/books/all-summary/src/part-1 } |  |  | 0.584 |
| walker |  | 9871 | 6 | Fs::DirListing { dir: tests/gui/books/all-summary/src/part-2 } |  |  | 0.584 |
| ns | 9884 |  | 64 | CI and repository automation listing | 9.6 |  | 0.585 |
| walker |  | 9918 | 47 | Fs::DirListing { dir: tests/gui/books/heading-nav/src } |  |  | 0.585 |
| ns | 9923 |  | 39 | README tail: licence | 9.7 | 1.1 | 0.585 |
| walker |  | 9963 | 45 | Code::CodeKey { rung: Doc, file: crates/mdbook-core/src/lib.rs, decl: 1, sub: 0, line: 7 } |  |  | 0.585 |
| walker |  | 9998 | 35 | Toml::Dependencies { file: guide/src/for_developers/mdbook-wordcount/Cargo.toml } |  |  | 0.585 |
