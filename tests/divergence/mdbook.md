Score(3000)=0.661 I=0.753 C=0.580 ns_rows≤3K=25/63 grid(1000/1442/2080/3000/4327/6240/9000)=0.673/0.605/0.611/0.661/0.609/0.571/0.561

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
| walker |  | 169 | 7 | Fs::DirListing { dir: tests } |  |  | 0.621 |
| ns | 201 |  | 48 | The nine workspace crates | 1.3 |  | 0.519 |
| walker |  | 217 | 48 | Fs::DirListing { dir: crates } |  |  | 0.662 |
| walker |  | 229 | 12 | Fs::DirListing { dir: crates/mdbook-compare } |  |  | 0.662 |
| walker |  | 233 | 4 | Fs::DirListing { dir: crates/mdbook-compare/src } |  |  | 0.662 |
| walker |  | 245 | 12 | Fs::DirListing { dir: crates/mdbook-core } |  |  | 0.662 |
| walker |  | 257 | 12 | Fs::DirListing { dir: crates/mdbook-driver } |  |  | 0.662 |
| walker |  | 269 | 12 | Fs::DirListing { dir: crates/mdbook-markdown } |  |  | 0.662 |
| walker |  | 273 | 4 | Fs::DirListing { dir: crates/mdbook-markdown/src } |  |  | 0.662 |
| walker |  | 285 | 12 | Fs::DirListing { dir: crates/mdbook-preprocessor } |  |  | 0.662 |
| walker |  | 289 | 4 | Fs::DirListing { dir: crates/mdbook-preprocessor/src } |  |  | 0.663 |
| walker |  | 301 | 12 | Fs::DirListing { dir: crates/mdbook-renderer } |  |  | 0.663 |
| walker |  | 305 | 4 | Fs::DirListing { dir: crates/mdbook-renderer/src } |  |  | 0.664 |
| walker |  | 317 | 12 | Fs::DirListing { dir: crates/mdbook-summary } |  |  | 0.664 |
| walker |  | 321 | 4 | Fs::DirListing { dir: crates/mdbook-summary/src } |  |  | 0.666 |
| walker |  | 337 | 16 | Fs::DirListing { dir: crates/mdbook-html } |  |  | 0.666 |
| ns | 349 |  | 148 | Crate-purpose map (driver crate docs), first half | 1.4 |  | 0.565 |
| walker |  | 357 | 20 | Code::CodeKey { rung: ModuleDoc, file: crates/mdbook-compare/src/main.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.565 |
| walker |  | 368 | 11 | Fs::DirListing { dir: examples } |  |  | 0.565 |
| walker |  | 386 | 18 | Fs::DirListing { dir: crates/mdbook-core/src } |  |  | 0.569 |
| walker |  | 390 | 4 | Fs::DirListing { dir: crates/mdbook-core/src/book } |  |  | 0.571 |
| ns | 403 |  | 54 | Crate-purpose map, `mdbook_core` entry | 1.5 | 1.4 | 0.550 |
| walker |  | 408 | 18 | Fs::DirListing { dir: crates/mdbook-core/src/utils } |  |  | 0.559 |
| walker |  | 429 | 21 | Code::CodeKey { rung: ModuleDoc, file: crates/mdbook-core/src/lib.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.559 |
| ns | 457 |  | 54 | Binary crate source listing (`src/`, `src/cmd/`, `src/cmd/watch/`) | 1.6 |  | 0.579 |
| walker |  | 508 | 79 | Markdown::ReadmeHeadline { file: README.md } |  |  | 0.807 |
| walker |  | 532 | 24 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.807 |
| walker |  | 551 | 19 | Fs::DirListing { dir: crates/mdbook-html/src } |  |  | 0.808 |
| ns | 565 |  | 108 | Workspace members and root package identity | 1.7 |  | 0.740 |
| walker |  | 569 | 18 | Fs::DirListing { dir: crates/mdbook-html/src/theme } |  |  | 0.741 |
| walker |  | 581 | 12 | Code::CodeKey { rung: ModuleDoc, file: crates/mdbook-html/src/lib.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.741 |
| walker |  | 603 | 22 | Fs::DirListing { dir: crates/mdbook-html/src/html_handlebars } |  |  | 0.745 |
| walker |  | 637 | 34 | Fs::DirListing { dir: crates/mdbook-html/src/html } |  |  | 0.753 |
| ns | 667 |  | 102 | Cargo features | 1.8 |  | 0.724 |
| walker |  | 674 | 37 | Fs::DirListing { dir: guide/src } |  |  | 0.725 |
| walker |  | 678 | 4 | Fs::DirListing { dir: guide/src/misc } |  |  | 0.725 |
| walker |  | 694 | 16 | Fs::DirListing { dir: guide/src/guide } |  |  | 0.726 |
| ns | 707 |  | 40 | mdbook-core source roster | 2.1 |  | 0.743 |
| ns | 775 |  | 68 | mdbook-driver source roster | 2.2 |  | 0.672 |
| walker |  | 808 | 114 | Code::CodeKey { rung: Names, file: src/main.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.672 |
| walker |  | 820 | 12 | Fs::DirListing { dir: .github } |  |  | 0.672 |
| walker |  | 834 | 14 | Fs::DirListing { dir: .github/workflows } |  |  | 0.673 |
| walker |  | 854 | 20 | Fs::DirListing { dir: guide/src/for_developers } |  |  | 0.674 |
| walker |  | 862 | 8 | Fs::DirListing { dir: guide/src/for_developers/mdbook-wordcount } |  |  | 0.674 |
| walker |  | 866 | 4 | Fs::DirListing { dir: guide/src/for_developers/mdbook-wordcount/src } |  |  | 0.674 |
| ns | 885 |  | 110 | mdbook-html source roster | 2.3 |  | 0.678 |
| walker |  | 907 | 41 | Code::CodeKey { rung: Names, file: guide/src/for_developers/mdbook-wordcount/src/main.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.678 |
| ns | 914 |  | 29 | Single-file crates and dev-tool crates | 2.4 |  | 0.673 |
| walker |  | 963 | 56 | Code::CodeKey { rung: Decl, file: guide/src/for_developers/mdbook-wordcount/src/main.rs, decl: 3, sub: 0, line: 44 } |  |  | 0.673 |
| walker |  | 980 | 17 | Code::CodeKey { rung: ModuleDoc, file: src/cmd/command_prelude.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.673 |
| walker |  | 1003 | 23 | Fs::DirListing { dir: crates/mdbook-html/front-end } |  |  | 0.673 |
| walker |  | 1011 | 8 | Fs::DirListing { dir: crates/mdbook-html/front-end/images } |  |  | 0.673 |
| walker |  | 1024 | 13 | Fs::DirListing { dir: crates/mdbook-html/front-end/js } |  |  | 0.674 |
| ns | 1055 |  | 141 | Integration testsuite listing | 2.5 |  | 0.582 |
| walker |  | 1075 | 51 | Code::CodeKey { rung: Names, file: crates/mdbook-html/src/lib.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.583 |
| walker |  | 1127 | 52 | Code::CodeKey { rung: Names, file: crates/mdbook-core/src/lib.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.583 |
| walker |  | 1145 | 18 | Code::CodeKey { rung: ModuleDoc, file: src/cmd/mod.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.583 |
| walker |  | 1176 | 31 | Fs::DirListing { dir: crates/mdbook-driver/src } |  |  | 0.597 |
| walker |  | 1180 | 4 | Fs::DirListing { dir: crates/mdbook-driver/src/mdbook } |  |  | 0.601 |
| walker |  | 1189 | 9 | Fs::DirListing { dir: crates/mdbook-driver/src/builtin_renderers } |  |  | 0.610 |
| walker |  | 1208 | 19 | Fs::DirListing { dir: crates/mdbook-driver/src/builtin_preprocessors } |  |  | 0.643 |
| walker |  | 1213 | 5 | Fs::DirListing { dir: crates/mdbook-driver/src/builtin_preprocessors/links } |  |  | 0.610 |
| ns | 1213 |  | 158 | Browser GUI test listing | 2.6 |  | 0.610 |
| walker |  | 1230 | 17 | Fs::DirListing { dir: crates/mdbook-html/front-end/searcher } |  |  | 0.610 |
| walker |  | 1306 | 76 | Code::CodeKey { rung: ModuleDoc, file: crates/mdbook-summary/src/lib.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.610 |
| walker |  | 1341 | 35 | Fs::DirListing { dir: guide/src/format } |  |  | 0.613 |
| walker |  | 1348 | 7 | Fs::DirListing { dir: guide/src/format/images } |  |  | 0.613 |
| walker |  | 1358 | 10 | Code::CodeKey { rung: Names, file: guide/src/format/example.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.613 |
| walker |  | 1378 | 20 | Fs::DirListing { dir: guide/src/format/theme } |  |  | 0.615 |
| walker |  | 1394 | 16 | Code::CodeKey { rung: ModuleDoc, file: src/cmd/watch/native.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.615 |
| ns | 1420 |  | 207 | Guide (user documentation) tree listing | 2.7 |  | 0.605 |
| walker |  | 1475 | 81 | Code::CodeKey { rung: Names, file: crates/mdbook-markdown/src/lib.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.605 |
| walker |  | 1494 | 19 | Code::CodeKey { rung: Decl, file: crates/mdbook-markdown/src/lib.rs, decl: 2, sub: 0, line: 33 } |  |  | 0.605 |
| walker |  | 1518 | 24 | Fs::DirListing { dir: ci } |  |  | 0.606 |
| ns | 1573 |  | 153 | Bundled front-end asset listing | 2.8 |  | 0.579 |
| walker |  | 1603 | 85 | Code::CodeKey { rung: Names, file: crates/mdbook-renderer/src/lib.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.579 |
| ns | 1626 |  | 53 | Examples tree listing | 2.9 |  | 0.567 |
| walker |  | 1647 | 44 | Code::CodeKey { rung: Decl, file: crates/mdbook-renderer/src/lib.rs, decl: 1, sub: 0, line: 28 } |  |  | 0.568 |
| walker |  | 1686 | 39 | Fs::DirListing { dir: guide/src/cli } |  |  | 0.603 |
| ns | 1736 |  | 110 | mdbook-core crate root | 3.1 |  | 0.599 |
| walker |  | 1769 | 83 | Code::CodeKey { rung: Decl, file: crates/mdbook-renderer/src/lib.rs, decl: 5, sub: 0, line: 63 } |  |  | 0.599 |
| walker |  | 1804 | 35 | Code::CodeKey { rung: Decl, file: crates/mdbook-renderer/src/lib.rs, decl: 6, sub: 0, line: 65 } |  |  | 0.599 |
| walker |  | 1828 | 24 | Fs::DirListing { dir: guide/src/format/configuration } |  |  | 0.621 |
| ns | 1867 |  | 131 | mdbook-html crate roots (whole files) | 3.2 |  | 0.610 |
| walker |  | 1926 | 98 | Code::CodeKey { rung: ModuleDoc, file: crates/mdbook-renderer/src/lib.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.610 |
| walker |  | 1938 | 12 | Fs::DirListing { dir: crates/xtask } |  |  | 0.610 |
| ns | 1942 |  | 75 | mdbook-driver modules and re-exports | 3.3 | 1.5 | 0.603 |
| walker |  | 1947 | 9 | Fs::DirListing { dir: crates/xtask/src } |  |  | 0.611 |
| walker |  | 2050 | 103 | Code::CodeKey { rung: ModuleDoc, file: crates/mdbook-markdown/src/lib.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.611 |
| ns | 2107 |  | 165 | The `Preprocessor` trait | 3.4 |  | 0.598 |
| walker |  | 2153 | 103 | Code::CodeKey { rung: ModuleDoc, file: crates/mdbook-preprocessor/src/lib.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.598 |
| walker |  | 2170 | 17 | Fs::DirListing { dir: crates/mdbook-html/src/html_handlebars/helpers } |  |  | 0.615 |
| walker |  | 2183 | 13 | Code::CodeKey { rung: Body, file: guide/src/for_developers/mdbook-wordcount/src/main.rs, decl: 2, sub: 0, line: 40 } |  |  | 0.615 |
| walker |  | 2200 | 17 | Code::CodeKey { rung: Doc, file: crates/mdbook-markdown/src/lib.rs, decl: 4, sub: 0, line: 44 } |  |  | 0.615 |
| walker |  | 2229 | 29 | Fs::DirListing { dir: crates/mdbook-html/front-end/playground_editor } |  |  | 0.626 |
| ns | 2259 |  | 152 | `PreprocessorContext` and `parse_input` | 3.5 |  | 0.616 |
| walker |  | 2280 | 51 | Markdown::ReadmeHeadline { file: crates/mdbook-compare/README.md } |  |  | 0.616 |
| walker |  | 2316 | 36 | Code::CodeKey { rung: Names, file: src/cmd/build.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.616 |
| walker |  | 2352 | 36 | Code::CodeKey { rung: Names, file: src/cmd/test.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.616 |
| ns | 2367 |  | 108 | The `Renderer` trait | 3.6 |  | 0.621 |
| walker |  | 2465 | 113 | Code::CodeKey { rung: Names, file: crates/mdbook-driver/src/lib.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.635 |
| ns | 2506 |  | 139 | `RenderContext` fields and methods | 3.7 |  | 0.626 |
| walker |  | 2579 | 114 | Code::CodeKey { rung: Names, file: crates/mdbook-preprocessor/src/lib.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.630 |
| walker |  | 2612 | 33 | Code::CodeKey { rung: Decl, file: crates/mdbook-preprocessor/src/lib.rs, decl: 6, sub: 0, line: 68 } |  |  | 0.630 |
| walker |  | 2624 | 12 | Code::CodeKey { rung: Doc, file: crates/mdbook-preprocessor/src/lib.rs, decl: 8, sub: 0, line: 82 } |  |  | 0.631 |
| walker |  | 2698 | 74 | Code::CodeKey { rung: Decl, file: crates/mdbook-preprocessor/src/lib.rs, decl: 1, sub: 0, line: 30 } |  |  | 0.642 |
| walker |  | 2706 | 8 | Code::CodeKey { rung: Body, file: crates/mdbook-preprocessor/src/lib.rs, decl: 4, sub: 0, line: 42 } |  |  | 0.644 |
| walker |  | 2737 | 31 | Fs::DirListing { dir: crates/mdbook-html/front-end/css } |  |  | 0.662 |
| ns | 2784 |  | 278 | Which markdown extensions mdBook enables | 3.8 |  | 0.642 |
| walker |  | 2854 | 117 | Code::CodeKey { rung: Names, file: crates/mdbook-compare/src/main.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.642 |
| walker |  | 2886 | 32 | Fs::DirListing { dir: crates/mdbook-html/front-end/templates } |  |  | 0.661 |
| walker |  | 2902 | 16 | Code::CodeKey { rung: Doc, file: src/main.rs, decl: 3, sub: 0, line: 58 } |  |  | 0.661 |
| ns | 3061 |  | 277 | mdbook-summary public types | 3.9 |  | 0.640 |
| walker |  | 3257 | 355 | Toml::Identity { file: Cargo.toml } |  |  | 0.664 |
| walker |  | 3268 | 11 | Code::CodeKey { rung: Doc, file: crates/mdbook-renderer/src/lib.rs, decl: 6, sub: 0, line: 65 } |  |  | 0.664 |
| ns | 3303 |  | 242 | Subcommand roster with descriptions | 4.1 |  | 0.651 |
| walker |  | 3329 | 61 | Markdown::ReadmeHeadline { file: crates/mdbook-renderer/README.md } |  |  | 0.651 |
| walker |  | 3390 | 61 | Markdown::ReadmeHeadline { file: crates/mdbook-summary/README.md } |  |  | 0.651 |
| walker |  | 3401 | 11 | Toml::Dependencies { file: crates/mdbook-compare/Cargo.toml } |  |  | 0.651 |
| walker |  | 3463 | 62 | Markdown::ReadmeHeadline { file: crates/mdbook-markdown/README.md } |  |  | 0.651 |
| ns | 3518 |  | 215 | `main()` dispatch | 4.2 |  | 0.639 |
| walker |  | 3525 | 62 | Markdown::ReadmeHeadline { file: crates/mdbook-preprocessor/README.md } |  |  | 0.639 |
| walker |  | 3588 | 63 | Markdown::ReadmeHeadline { file: crates/mdbook-core/README.md } |  |  | 0.639 |
| walker |  | 3660 | 72 | Markdown::ReadmeHeadline { file: crates/mdbook-html/README.md } |  |  | 0.639 |
| walker |  | 3681 | 21 | Toml::Operational { file: crates/mdbook-driver/Cargo.toml } |  |  | 0.639 |
| walker |  | 3693 | 12 | Code::CodeKey { rung: ModuleDoc, file: crates/mdbook-html/src/utils.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.639 |
| ns | 3791 |  | 273 | Shared CLI argument builders | 4.3 |  | 0.622 |
| walker |  | 3795 | 102 | Toml::Operational { file: Cargo.toml } |  |  | 0.632 |
| walker |  | 3999 | 204 | Code::CodeKey { rung: Names, file: crates/mdbook-summary/src/lib.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.633 |
| walker |  | 4017 | 18 | Code::CodeKey { rung: Decl, file: crates/mdbook-summary/src/lib.rs, decl: 6, sub: 0, line: 108 } |  |  | 0.633 |
| walker |  | 4039 | 22 | Code::CodeKey { rung: Decl, file: crates/mdbook-summary/src/lib.rs, decl: 9, sub: 0, line: 140 } |  |  | 0.633 |
| ns | 4064 |  | 273 | `init` and `serve` subcommand flags | 4.4 |  | 0.617 |
| walker |  | 4076 | 37 | Code::CodeKey { rung: Decl, file: crates/mdbook-summary/src/lib.rs, decl: 4, sub: 0, line: 96 } |  |  | 0.617 |
| walker |  | 4166 | 90 | Code::CodeKey { rung: Decl, file: crates/mdbook-summary/src/lib.rs, decl: 8, sub: 0, line: 120 } |  |  | 0.625 |
| walker |  | 4176 | 10 | Code::CodeKey { rung: Doc, file: crates/mdbook-summary/src/lib.rs, decl: 8, sub: 0, line: 120 } |  |  | 0.626 |
| ns | 4296 |  | 232 | `test`, `build`, `clean` and `watch` flags | 4.5 |  | 0.609 |
| walker |  | 4333 | 157 | Code::CodeKey { rung: Decl, file: crates/mdbook-summary/src/lib.rs, decl: 3, sub: 0, line: 82 } |  |  | 0.617 |
| ns | 4483 |  | 187 | CLI function roster | 4.6 |  | 0.605 |
| walker |  | 4503 | 170 | Code::CodeKey { rung: Decl, file: crates/mdbook-summary/src/lib.rs, decl: 2, sub: 0, line: 65 } |  |  | 0.616 |
| walker |  | 4537 | 34 | Code::CodeKey { rung: Names, file: src/cmd/watch/poller.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.616 |
| walker |  | 4561 | 24 | Fs::DirListing { dir: examples/remove-emphasis } |  |  | 0.622 |
| walker |  | 4571 | 10 | Fs::DirListing { dir: examples/remove-emphasis/src } |  |  | 0.626 |
| walker |  | 4595 | 24 | Toml::Operational { file: crates/mdbook-html/Cargo.toml } |  |  | 0.626 |
| walker |  | 4609 | 14 | Code::CodeKey { rung: ModuleDoc, file: crates/mdbook-core/src/book.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.626 |
| walker |  | 4623 | 14 | Code::CodeKey { rung: ModuleDoc, file: crates/mdbook-driver/src/init.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.626 |
| ns | 4742 |  | 259 | Clap app assembly and `MDBOOK_LOG` | 4.7 |  | 0.615 |
| walker |  | 4809 | 186 | Code::CodeKey { rung: Decl, file: crates/mdbook-markdown/src/lib.rs, decl: 1, sub: 0, line: 14 } |  |  | 0.617 |
| ns | 4811 |  | 69 | `Book` — the tree type | 5.1 |  | 0.613 |
| walker |  | 4817 | 8 | Code::CodeKey { rung: Doc, file: crates/mdbook-markdown/src/lib.rs, decl: 1, sub: 0, line: 14 } |  |  | 0.613 |
| walker |  | 4829 | 12 | Code::CodeKey { rung: Doc, file: crates/mdbook-preprocessor/src/lib.rs, decl: 7, sub: 0, line: 70 } |  |  | 0.613 |
| walker |  | 4841 | 12 | Code::CodeKey { rung: Doc, file: crates/mdbook-renderer/src/lib.rs, decl: 2, sub: 0, line: 30 } |  |  | 0.613 |
| walker |  | 4908 | 67 | Code::CodeKey { rung: Names, file: src/cmd/command_prelude.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.614 |
| walker |  | 4930 | 22 | Code::CodeKey { rung: Decl, file: src/cmd/command_prelude.rs, decl: 7, sub: 0, line: 56 } |  |  | 0.614 |
| ns | 4938 |  | 127 | `Book` method roster | 5.2 | 5.1 | 0.609 |
| walker |  | 4959 | 29 | Markdown::HeadingsOutline { file: guide/src/SUMMARY.md } |  |  | 0.609 |
| ns | 5009 |  | 71 | `BookItem` enum | 5.3 |  | 0.604 |
| walker |  | 5061 | 102 | Markdown::ReadmeHeadline { file: crates/mdbook-driver/README.md } |  |  | 0.604 |
| ns | 5150 |  | 141 | `Chapter` — first fields | 5.4 |  | 0.597 |
| walker |  | 5279 | 218 | Code::CodeKey { rung: Decl, file: crates/mdbook-preprocessor/src/lib.rs, decl: 5, sub: 0, line: 49 } |  |  | 0.604 |
| walker |  | 5333 | 54 | Toml::Identity { file: guide/src/for_developers/mdbook-wordcount/Cargo.toml } |  |  | 0.604 |
| ns | 5335 |  | 185 | `Chapter` — `path` vs `source_path`, `parent_names` | 5.5 | 5.4 | 0.598 |
| walker |  | 5350 | 17 | Code::CodeKey { rung: ModuleDoc, file: crates/mdbook-driver/src/mdbook.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.598 |
| walker |  | 5398 | 48 | Markdown::Section { file: crates/mdbook-driver/README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.598 |
| walker |  | 5446 | 48 | Markdown::Section { file: crates/mdbook-markdown/README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.598 |
| ns | 5450 |  | 115 | `Chapter` methods, `SectionNumber`, `BookItems` | 5.6 |  | 0.594 |
| walker |  | 5494 | 48 | Markdown::Section { file: crates/mdbook-preprocessor/README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.594 |
| walker |  | 5542 | 48 | Markdown::Section { file: crates/mdbook-renderer/README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.594 |
| walker |  | 5590 | 48 | Markdown::Section { file: crates/mdbook-summary/README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.594 |
| ns | 5593 |  | 143 | `Config` — the `book.toml` root | 6.1 |  | 0.588 |
| walker |  | 5656 | 66 | Code::CodeKey { rung: Decl, file: crates/mdbook-compare/src/main.rs, decl: 1, sub: 0, line: 6 } |  |  | 0.588 |
| walker |  | 5706 | 50 | Markdown::Section { file: crates/mdbook-core/README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.588 |
| ns | 5731 |  | 138 | `Config` method roster | 6.2 | 6.1 | 0.583 |
| walker |  | 5756 | 50 | Markdown::Section { file: crates/mdbook-html/README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.583 |
| walker |  | 5835 | 79 | Code::CodeKey { rung: Names, file: src/cmd/mod.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.583 |
| walker |  | 5880 | 45 | Code::CodeKey { rung: Decl, file: src/cmd/watch/poller.rs, decl: 1, sub: 0, line: 18 } |  |  | 0.583 |
| ns | 5944 |  | 213 | `[book]`, `[build]` and `[rust]` keys | 6.3 |  | 0.574 |
| walker |  | 5961 | 81 | Code::CodeKey { rung: Names, file: src/cmd/init.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.575 |
| walker |  | 6032 | 71 | Code::CodeKey { rung: Decl, file: crates/mdbook-driver/src/lib.rs, decl: 2, sub: 0, line: 107 } |  |  | 0.575 |
| walker |  | 6044 | 12 | Code::CodeKey { rung: ModuleDoc, file: crates/mdbook-core/src/utils/mod.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.575 |
| walker |  | 6056 | 12 | Code::CodeKey { rung: ModuleDoc, file: crates/mdbook-driver/src/builtin_preprocessors/mod.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.569 |
| ns | 6056 |  | 112 | `RustEdition` variants | 6.4 |  | 0.569 |
| walker |  | 6068 | 12 | Code::CodeKey { rung: ModuleDoc, file: crates/mdbook-html/src/theme/mod.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.569 |
| walker |  | 6154 | 86 | Code::CodeKey { rung: Names, file: src/cmd/clean.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.571 |
| walker |  | 6184 | 30 | Code::CodeKey { rung: Decl, file: src/cmd/clean.rs, decl: 5, sub: 0, line: 91 } |  |  | 0.571 |
| walker |  | 6233 | 49 | Code::CodeKey { rung: Decl, file: src/cmd/clean.rs, decl: 4, sub: 0, line: 45 } |  |  | 0.571 |
| walker |  | 6246 | 13 | Markdown::Section { file: guide/src/SUMMARY.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.571 |
| ns | 6284 |  | 228 | `[output.html]` keys, first half | 6.5 |  | 0.562 |
| walker |  | 6291 | 45 | Code::CodeKey { rung: Doc, file: crates/mdbook-core/src/lib.rs, decl: 1, sub: 0, line: 7 } |  |  | 0.562 |
| walker |  | 6304 | 13 | Code::CodeKey { rung: ModuleDoc, file: crates/mdbook-core/src/utils/fs.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.562 |
| walker |  | 6317 | 13 | Code::CodeKey { rung: ModuleDoc, file: crates/mdbook-core/src/utils/html.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.562 |
| walker |  | 6330 | 13 | Code::CodeKey { rung: ModuleDoc, file: crates/mdbook-html/src/html/hide_lines.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.562 |
| walker |  | 6343 | 13 | Code::CodeKey { rung: ModuleDoc, file: crates/mdbook-html/src/html_handlebars/static_files.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.562 |
| walker |  | 6433 | 90 | Code::CodeKey { rung: Names, file: src/cmd/serve.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.563 |
| ns | 6441 |  | 157 | `[output.html]` keys, second half | 6.6 | 6.5 | 0.557 |
| walker |  | 6521 | 88 | Code::CodeKey { rung: Decl, file: src/cmd/command_prelude.rs, decl: 1, sub: 0, line: 7 } |  |  | 0.559 |
| walker |  | 6540 | 19 | Code::CodeKey { rung: Decl, file: src/cmd/command_prelude.rs, decl: 6, sub: 0, line: 41 } |  |  | 0.559 |
| walker |  | 6554 | 14 | Code::CodeKey { rung: ModuleDoc, file: crates/mdbook-html/src/theme/playground_editor.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.559 |
| walker |  | 6688 | 134 | Fs::DirListing { dir: tests/testsuite } |  |  | 0.611 |
| walker |  | 6709 | 21 | Code::CodeKey { rung: Names, file: crates/mdbook-driver/src/init.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.611 |
| walker |  | 6772 | 63 | Code::CodeKey { rung: ModuleDoc, file: src/cmd/watch/poller.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.611 |
| walker |  | 6787 | 15 | Code::CodeKey { rung: Doc, file: crates/mdbook-driver/src/lib.rs, decl: 2, sub: 0, line: 107 } |  |  | 0.611 |
| walker |  | 6802 | 15 | Code::CodeKey { rung: ModuleDoc, file: crates/mdbook-core/src/utils/toml_ext.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.611 |
| ns | 6803 |  | 362 | `[output.html.*]` sub-tables | 6.7 |  | 0.596 |
| ns | 6912 |  | 109 | `MDBook` struct | 7.1 |  | 0.592 |
| walker |  | 7119 | 317 | Code::CodeKey { rung: Decl, file: crates/mdbook-renderer/src/lib.rs, decl: 4, sub: 0, line: 38 } |  |  | 0.603 |
| walker |  | 7130 | 11 | Code::CodeKey { rung: Doc, file: crates/mdbook-renderer/src/lib.rs, decl: 4, sub: 0, line: 38 } |  |  | 0.603 |
| ns | 7134 |  | 222 | `MDBook` method roster | 7.2 | 7.1 | 0.595 |
| walker |  | 7139 | 9 | Code::CodeKey { rung: Body, file: src/cmd/command_prelude.rs, decl: 8, sub: 0, line: 57 } |  |  | 0.595 |
| walker |  | 7249 | 110 | Code::CodeKey { rung: Names, file: src/cmd/watch.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.600 |
| walker |  | 7266 | 17 | Code::CodeKey { rung: Decl, file: src/cmd/watch.rs, decl: 2, sub: 0, line: 21 } |  |  | 0.603 |
| walker |  | 7292 | 26 | Code::CodeKey { rung: Decl, file: src/cmd/watch.rs, decl: 3, sub: 0, line: 26 } |  |  | 0.607 |
| walker |  | 7348 | 56 | Code::CodeKey { rung: Decl, file: src/cmd/watch.rs, decl: 6, sub: 0, line: 62 } |  |  | 0.607 |
| walker |  | 7417 | 69 | Code::CodeKey { rung: Names, file: src/cmd/watch/native.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.609 |
| walker |  | 7462 | 45 | Code::CodeKey { rung: Decl, file: src/cmd/watch/native.rs, decl: 1, sub: 0, line: 11 } |  |  | 0.609 |
| walker |  | 7480 | 18 | Code::CodeKey { rung: Doc, file: crates/mdbook-driver/src/lib.rs, decl: 1, sub: 0, line: 82 } |  |  | 0.597 |
| ns | 7480 |  | 346 | Which plugins run | 7.3 |  | 0.597 |
| walker |  | 7529 | 49 | Code::CodeKey { rung: Body, file: src/main.rs, decl: 6, sub: 0, line: 139 } |  |  | 0.597 |
| walker |  | 7637 | 108 | Code::CodeKey { rung: Decl, file: crates/mdbook-summary/src/lib.rs, decl: 11, sub: 0, line: 173 } |  |  | 0.597 |
| walker |  | 7709 | 72 | Markdown::ReadmeHeadline { file: guide/src/guide/README.md } |  |  | 0.597 |
| ns | 7726 |  | 246 | Loading a book from disk, and `BookBuilder` | 7.4 |  | 0.591 |
| walker |  | 7728 | 19 | Code::CodeKey { rung: ModuleDoc, file: crates/mdbook-html/src/html/serialize.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.591 |
| walker |  | 7750 | 22 | Code::CodeKey { rung: Body, file: crates/mdbook-preprocessor/src/lib.rs, decl: 8, sub: 0, line: 82 } |  |  | 0.593 |
| walker |  | 7771 | 21 | Markdown::HeadingsOutline { file: guide/src/format/theme/editor.md } |  |  | 0.593 |
| walker |  | 7889 | 118 | Markdown::ReadmeHeadline { file: guide/src/for_developers/README.md } |  |  | 0.593 |
| walker |  | 7916 | 27 | Markdown::HeadingsOutline { file: guide/src/for_developers/README.md } |  |  | 0.593 |
| ns | 7970 |  | 244 | The `links` preprocessor's helper syntax | 7.5 |  | 0.587 |
| walker |  | 7971 | 55 | Markdown::HeadingsOutline { file: guide/src/for_developers/preprocessors.md } |  |  | 0.587 |
| walker |  | 8087 | 116 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.587 |
| walker |  | 8115 | 28 | Markdown::HeadingsOutline { file: guide/src/format/summary.md } |  |  | 0.587 |
| walker |  | 8130 | 15 | Code::CodeKey { rung: Names, file: crates/mdbook-html/src/html_handlebars/helpers/fontawesome.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.587 |
| walker |  | 8173 | 43 | Toml::Dependencies { file: crates/mdbook-markdown/Cargo.toml } |  |  | 0.587 |
| walker |  | 8211 | 38 | Markdown::Section { file: crates/mdbook-core/README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.587 |
| walker |  | 8249 | 38 | Markdown::Section { file: crates/mdbook-driver/README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.587 |
| ns | 8271 |  | 301 | The other built-in plugins | 7.6 |  | 0.580 |
| walker |  | 8287 | 38 | Markdown::Section { file: crates/mdbook-html/README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.580 |
| walker |  | 8325 | 38 | Markdown::Section { file: crates/mdbook-markdown/README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.580 |
| walker |  | 8363 | 38 | Markdown::Section { file: crates/mdbook-preprocessor/README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.580 |
| walker |  | 8401 | 38 | Markdown::Section { file: crates/mdbook-renderer/README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.580 |
| walker |  | 8439 | 38 | Markdown::Section { file: crates/mdbook-summary/README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.580 |
| walker |  | 8502 | 63 | Markdown::ReadmeHeadline { file: guide/src/format/theme/README.md } |  |  | 0.580 |
| ns | 8521 |  | 250 | The markdown-to-HTML pipeline | 8.1 |  | 0.571 |
| walker |  | 8534 | 32 | Markdown::HeadingsOutline { file: guide/src/format/mathjax.md } |  |  | 0.571 |
| walker |  | 8610 | 76 | Markdown::ReadmeHeadline { file: guide/src/format/README.md } |  |  | 0.571 |
| walker |  | 8636 | 26 | Code::CodeKey { rung: Names, file: crates/mdbook-driver/src/mdbook/tests.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.571 |
| ns | 8640 |  | 119 | HTML pipeline entry points | 8.2 | 8.1 | 0.567 |
| walker |  | 8656 | 20 | Code::CodeKey { rung: Doc, file: crates/mdbook-summary/src/lib.rs, decl: 2, sub: 0, line: 65 } |  |  | 0.567 |
| walker |  | 8737 | 81 | Markdown::HeadingsOutline { file: guide/src/continuous-integration.md } |  |  | 0.567 |
| ns | 8749 |  | 109 | `HtmlHandlebars` and its render steps | 8.3 |  | 0.563 |
| walker |  | 8788 | 51 | Toml::Dependencies { file: crates/mdbook-preprocessor/Cargo.toml } |  |  | 0.563 |
| walker |  | 8839 | 51 | Toml::Dependencies { file: crates/mdbook-renderer/Cargo.toml } |  |  | 0.563 |
| walker |  | 8858 | 19 | Code::CodeKey { rung: Doc, file: src/cmd/watch/poller.rs, decl: 1, sub: 0, line: 18 } |  |  | 0.563 |
| walker |  | 8872 | 14 | Code::CodeKey { rung: Doc, file: crates/mdbook-preprocessor/src/lib.rs, decl: 2, sub: 0, line: 32 } |  |  | 0.565 |
| ns | 8893 |  | 144 | Theme resolution | 8.4 |  | 0.561 |
| walker |  | 8921 | 49 | Markdown::HeadingsOutline { file: guide/src/guide/reading.md } |  |  | 0.561 |
| ns | 9021 |  | 128 | Search index, static files and handlebars helpers | 8.5 |  | 0.557 |
| walker |  | 9079 | 158 | Fs::DirListing { dir: tests/gui } |  |  | 0.580 |
| walker |  | 9109 | 30 | Fs::DirListing { dir: tests/gui/books } |  |  | 0.580 |
| walker |  | 9117 | 8 | Fs::DirListing { dir: tests/gui/books/heading-nav-folded } |  |  | 0.580 |
| walker |  | 9125 | 8 | Fs::DirListing { dir: tests/gui/books/sidebar-scroll } |  |  | 0.580 |
| walker |  | 9137 | 12 | Fs::DirListing { dir: tests/gui/books/all-summary } |  |  | 0.580 |
| walker |  | 9149 | 12 | Fs::DirListing { dir: tests/gui/books/heading-nav } |  |  | 0.580 |
| walker |  | 9161 | 12 | Fs::DirListing { dir: tests/gui/books/highlighting } |  |  | 0.580 |
| walker |  | 9169 | 8 | Fs::DirListing { dir: tests/gui/books/highlighting/src } |  |  | 0.580 |
| walker |  | 9181 | 12 | Fs::DirListing { dir: tests/gui/books/redirect } |  |  | 0.580 |
| walker |  | 9193 | 12 | Fs::DirListing { dir: tests/gui/books/search } |  |  | 0.580 |
| walker |  | 9206 | 13 | Fs::DirListing { dir: tests/gui/books/search/src } |  |  | 0.580 |
| ns | 9219 |  | 198 | Testsuite module map and harness convention | 9.1 |  | 0.572 |
| walker |  | 9222 | 16 | Fs::DirListing { dir: tests/gui/books/heading-nav-folded/src } |  |  | 0.572 |
| walker |  | 9233 | 11 | Fs::DirListing { dir: tests/gui/books/heading-nav-folded/src/sub } |  |  | 0.572 |
| walker |  | 9255 | 22 | Fs::DirListing { dir: tests/gui/books/redirect/src } |  |  | 0.572 |
| walker |  | 9297 | 42 | Fs::DirListing { dir: tests/gui/books/all-summary/src } |  |  | 0.572 |
| ns | 9335 |  | 116 | Snapshot testing with snapbox | 9.2 | 9.1 | 0.571 |
| walker |  | 9344 | 47 | Fs::DirListing { dir: tests/gui/books/heading-nav/src } |  |  | 0.571 |
| walker |  | 9378 | 34 | Code::CodeKey { rung: Doc, file: src/cmd/clean.rs, decl: 3, sub: 0, line: 38 } |  |  | 0.571 |
| walker |  | 9392 | 14 | Code::CodeKey { rung: Body, file: crates/mdbook-renderer/src/lib.rs, decl: 7, sub: 0, line: 81 } |  |  | 0.571 |
| walker |  | 9469 | 77 | Markdown::HeadingsOutline { file: guide/src/for_developers/backends.md } |  |  | 0.571 |
| walker |  | 9501 | 32 | Markdown::Section { file: guide/src/404.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.571 |
| ns | 9669 |  | 334 | `BookTest` harness API | 9.3 | 9.1 | 0.562 |
| ns | 9820 |  | 151 | Contributor workflow: CONTRIBUTING section map | 9.4 |  | 0.557 |
| walker |  | 9852 | 351 | Code::CodeKey { rung: ModuleDoc, file: crates/mdbook-driver/src/lib.rs, decl: 0, sub: 0, line: 0 } |  |  | 0.584 |
| ns | 9884 |  | 64 | CI and repository automation listing | 9.6 |  | 0.585 |
| walker |  | 9914 | 62 | Code::CodeKey { rung: Decl, file: src/cmd/serve.rs, decl: 4, sub: 0, line: 108 } |  |  | 0.585 |
| ns | 9923 |  | 39 | README tail: licence | 9.7 | 1.1 | 0.585 |
| walker |  | 9999 | 85 | Markdown::Section { file: guide/src/for_developers/README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.585 |
