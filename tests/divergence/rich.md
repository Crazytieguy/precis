Score(3000)=0.674 I=0.837 C=0.543 ns_rows≤3K=15/51 grid(1000/1442/2080/3000/4327/6240/9000)=0.549/0.522/0.707/0.674/0.621/0.537/0.527

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| ns | 85 |  | 85 | README lede — what Rich is | 1.1 |  | 0.000 |
| ns | 129 |  | 44 | rich/__init__.py docstring + __all__ | 1.2 |  | 0.000 |
| walker |  | 191 | 191 | Fs::DirListing { dir: . } |  |  | 0.000 |
| walker |  | 203 | 12 | Fs::DirListing { dir: assets } |  |  | 0.000 |
| walker |  | 222 | 19 | Fs::DirListing { dir: docs } |  |  | 0.000 |
| walker |  | 231 | 9 | Fs::DirListing { dir: docs/images } |  |  | 0.000 |
| walker |  | 239 | 8 | Fs::DirListing { dir: .faq } |  |  | 0.000 |
| ns | 263 |  | 134 | pyproject identity block | 1.3 |  | 0.000 |
| ns | 363 |  | 100 | README section headings (all H1/H2 locations) | 1.4 |  | 0.000 |
| walker |  | 393 | 154 | Plaintext::Whole { file: Makefile } |  |  | 0.000 |
| walker |  | 476 | 83 | Toml::Identity { file: pyproject.toml } |  |  | 0.200 |
| ns | 514 |  | 151 | Runtime dependencies, extras and build backend | 1.5 |  | 0.158 |
| walker |  | 549 | 73 | Fs::DirListing { dir: questions } |  |  | 0.159 |
| walker |  | 647 | 98 | Fs::DirListing { dir: imgs } |  |  | 0.159 |
| ns | 661 |  | 147 | README renderable gallery — all `<summary>` labels | 1.6 |  | 0.134 |
| walker |  | 728 | 81 | Toml::Dependencies { file: pyproject.toml } |  |  | 0.187 |
| ns | 758 |  | 97 | Compatibility + install + `python -m rich` | 1.7 |  | 0.174 |
| walker |  | 875 | 147 | Markdown::ReadmeHeadline { file: README.md } |  |  | 0.316 |
| walker |  | 898 | 23 | Fs::DirListing { dir: .github } |  |  | 0.316 |
| walker |  | 928 | 30 | Fs::DirListing { dir: .github/workflows } |  |  | 0.316 |
| ns | 949 |  | 191 | Repository root listing (complete) | 1.8 |  | 0.548 |
| walker |  | 951 | 23 | Fs::DirListing { dir: benchmarks } |  |  | 0.549 |
| walker |  | 1032 | 81 | Markdown::Prelude { file: README.md } |  |  | 0.549 |
| walker |  | 1123 | 91 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.647 |
| walker |  | 1145 | 22 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.656 |
| walker |  | 1196 | 51 | Toml::PackageMetadata { file: pyproject.toml } |  |  | 0.750 |
| walker |  | 1239 | 43 | Fs::DirListing { dir: tools } |  |  | 0.751 |
| ns | 1312 |  | 363 | rich/ package module roster (complete) | 1.9 |  | 0.535 |
| walker |  | 1374 | 135 | Fs::DirListing { dir: docs/source } |  |  | 0.537 |
| walker |  | 1384 | 10 | Fs::DirListing { dir: docs/source/appendix } |  |  | 0.538 |
| ns | 1424 |  | 112 | rich.* top-level function signatures: get_console, reconfigure, print | 2.1 |  | 0.522 |
| walker |  | 1550 | 166 | Fs::DirListing { dir: examples } |  |  | 0.525 |
| ns | 1729 |  | 305 | rich.* top-level function signatures: print_json, inspect | 2.2 |  | 0.486 |
| walker |  | 1913 | 363 | Fs::DirListing { dir: rich } |  |  | 0.743 |
| walker |  | 1929 | 16 | Code::CodeKey { rung: ModuleDoc, file: rich/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.756 |
| walker |  | 1955 | 26 | Code::CodeKey { rung: Names, file: rich/__main__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.756 |
| walker |  | 1977 | 22 | Code::CodeKey { rung: Decl, file: rich/__main__.py, decl: 1, sub: 0, line: 18 } |  |  | 0.756 |
| walker |  | 1988 | 11 | Code::CodeKey { rung: Names, file: rich/_emoji_codes.py, decl: 0, sub: 0, line: 0 } |  |  | 0.756 |
| walker |  | 1999 | 11 | Code::CodeKey { rung: Names, file: rich/json.py, decl: 0, sub: 0, line: 0 } |  |  | 0.756 |
| walker |  | 2010 | 11 | Code::CodeKey { rung: Names, file: rich/palette.py, decl: 0, sub: 0, line: 0 } |  |  | 0.756 |
| ns | 2019 |  | 290 | The console protocol: RichCast, ConsoleRenderable, RenderableType, RenderResult | 2.3 |  | 0.707 |
| walker |  | 2021 | 11 | Code::CodeKey { rung: Names, file: rich/screen.py, decl: 0, sub: 0, line: 0 } |  |  | 0.707 |
| walker |  | 2032 | 11 | Code::CodeKey { rung: Names, file: rich/spinner.py, decl: 0, sub: 0, line: 0 } |  |  | 0.707 |
| walker |  | 2043 | 11 | Code::CodeKey { rung: Names, file: rich/styled.py, decl: 0, sub: 0, line: 0 } |  |  | 0.707 |
| walker |  | 2069 | 26 | Code::CodeKey { rung: Decl, file: rich/__main__.py, decl: 2, sub: 0, line: 19 } |  |  | 0.707 |
| walker |  | 2095 | 26 | Code::CodeKey { rung: Decl, file: rich/__main__.py, decl: 3, sub: 0, line: 33 } |  |  | 0.707 |
| walker |  | 2107 | 12 | Code::CodeKey { rung: Names, file: rich/scope.py, decl: 0, sub: 0, line: 0 } |  |  | 0.707 |
| walker |  | 2120 | 13 | Code::CodeKey { rung: Names, file: rich/_spinners.py, decl: 0, sub: 0, line: 0 } |  |  | 0.707 |
| walker |  | 2133 | 13 | Code::CodeKey { rung: Names, file: rich/_windows.py, decl: 0, sub: 0, line: 0 } |  |  | 0.707 |
| walker |  | 2147 | 14 | Code::CodeKey { rung: Names, file: rich/abc.py, decl: 0, sub: 0, line: 0 } |  |  | 0.707 |
| walker |  | 2161 | 14 | Code::CodeKey { rung: Names, file: rich/columns.py, decl: 0, sub: 0, line: 0 } |  |  | 0.707 |
| walker |  | 2175 | 14 | Code::CodeKey { rung: Names, file: rich/logging.py, decl: 0, sub: 0, line: 0 } |  |  | 0.708 |
| walker |  | 2189 | 14 | Code::CodeKey { rung: Names, file: rich/panel.py, decl: 0, sub: 0, line: 0 } |  |  | 0.708 |
| walker |  | 2203 | 14 | Code::CodeKey { rung: Names, file: rich/region.py, decl: 0, sub: 0, line: 0 } |  |  | 0.708 |
| walker |  | 2217 | 14 | Code::CodeKey { rung: Names, file: rich/rule.py, decl: 0, sub: 0, line: 0 } |  |  | 0.708 |
| walker |  | 2231 | 14 | Code::CodeKey { rung: Names, file: rich/status.py, decl: 0, sub: 0, line: 0 } |  |  | 0.708 |
| walker |  | 2245 | 14 | Code::CodeKey { rung: Names, file: rich/themes.py, decl: 0, sub: 0, line: 0 } |  |  | 0.708 |
| walker |  | 2260 | 15 | Code::CodeKey { rung: Names, file: rich/constrain.py, decl: 0, sub: 0, line: 0 } |  |  | 0.708 |
| walker |  | 2276 | 16 | Code::CodeKey { rung: Names, file: rich/color_triplet.py, decl: 0, sub: 0, line: 0 } |  |  | 0.708 |
| ns | 2281 |  | 262 | console.py module-level symbol roster | 2.4 |  | 0.672 |
| walker |  | 2292 | 16 | Code::CodeKey { rung: Names, file: rich/file_proxy.py, decl: 0, sub: 0, line: 0 } |  |  | 0.672 |
| walker |  | 2312 | 20 | Code::CodeKey { rung: Names, file: rich/default_styles.py, decl: 0, sub: 0, line: 0 } |  |  | 0.672 |
| walker |  | 2333 | 21 | Code::CodeKey { rung: Names, file: rich/_pick.py, decl: 0, sub: 0, line: 0 } |  |  | 0.672 |
| walker |  | 2354 | 21 | Code::CodeKey { rung: Names, file: rich/diagnose.py, decl: 0, sub: 0, line: 0 } |  |  | 0.672 |
| walker |  | 2377 | 23 | Code::CodeKey { rung: Names, file: rich/_fileno.py, decl: 0, sub: 0, line: 0 } |  |  | 0.672 |
| walker |  | 2399 | 22 | Code::CodeKey { rung: Decl, file: rich/abc.py, decl: 1, sub: 0, line: 4 } |  |  | 0.672 |
| walker |  | 2407 | 8 | Code::CodeKey { rung: Decl, file: rich/abc.py, decl: 2, sub: 0, line: 15 } |  |  | 0.672 |
| walker |  | 2424 | 17 | Code::CodeKey { rung: Doc, file: rich/__main__.py, decl: 4, sub: 0, line: 39 } |  |  | 0.672 |
| walker |  | 2448 | 24 | Code::CodeKey { rung: Names, file: rich/_null_file.py, decl: 0, sub: 0, line: 0 } |  |  | 0.672 |
| walker |  | 2473 | 25 | Code::CodeKey { rung: Names, file: rich/measure.py, decl: 0, sub: 0, line: 0 } |  |  | 0.672 |
| walker |  | 2498 | 25 | Code::CodeKey { rung: Names, file: rich/pager.py, decl: 0, sub: 0, line: 0 } |  |  | 0.672 |
| walker |  | 2516 | 18 | Code::CodeKey { rung: Decl, file: rich/pager.py, decl: 1, sub: 0, line: 5 } |  |  | 0.672 |
| walker |  | 2525 | 9 | Code::CodeKey { rung: Decl, file: rich/pager.py, decl: 2, sub: 0, line: 8 } |  |  | 0.642 |
| ns | 2525 |  | 244 | Console method roster — rendering and output (lines 1092–1652) | 2.5 |  | 0.642 |
| walker |  | 2551 | 26 | Code::CodeKey { rung: Names, file: rich/_timer.py, decl: 0, sub: 0, line: 0 } |  |  | 0.642 |
| walker |  | 2561 | 10 | Code::CodeKey { rung: Decl, file: rich/_timer.py, decl: 1, sub: 0, line: 12 } |  |  | 0.642 |
| walker |  | 2588 | 27 | Code::CodeKey { rung: Names, file: rich/_extension.py, decl: 0, sub: 0, line: 0 } |  |  | 0.642 |
| walker |  | 2615 | 27 | Code::CodeKey { rung: Names, file: rich/_windows_renderer.py, decl: 0, sub: 0, line: 0 } |  |  | 0.642 |
| walker |  | 2644 | 29 | Code::CodeKey { rung: Names, file: rich/live.py, decl: 0, sub: 0, line: 0 } |  |  | 0.642 |
| walker |  | 2677 | 33 | Code::CodeKey { rung: Names, file: rich/theme.py, decl: 0, sub: 0, line: 0 } |  |  | 0.642 |
| walker |  | 2711 | 34 | Code::CodeKey { rung: Names, file: rich/_stack.py, decl: 0, sub: 0, line: 0 } |  |  | 0.642 |
| ns | 2735 |  | 210 | Console method roster — JSON, screen updates, exceptions, logging, export (1758–2606) | 2.6 |  | 0.621 |
| walker |  | 2743 | 32 | Code::CodeKey { rung: Decl, file: rich/_stack.py, decl: 2, sub: 0, line: 6 } |  |  | 0.621 |
| walker |  | 2751 | 8 | Code::CodeKey { rung: Decl, file: rich/_stack.py, decl: 3, sub: 0, line: 9 } |  |  | 0.621 |
| walker |  | 2910 | 159 | Code::CodeKey { rung: Names, file: rich/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.656 |
| walker |  | 2977 | 67 | Code::CodeKey { rung: Decl, file: rich/__init__.py, decl: 4, sub: 0, line: 53 } |  |  | 0.674 |
| walker |  | 3116 | 139 | Code::CodeKey { rung: Decl, file: rich/__init__.py, decl: 6, sub: 0, line: 120 } |  |  | 0.691 |
| ns | 3126 |  | 391 | Console method roster — construction, context management, properties (617–1084) | 2.7 |  | 0.649 |
| walker |  | 3263 | 147 | Code::CodeKey { rung: Decl, file: rich/__init__.py, decl: 5, sub: 0, line: 77 } |  |  | 0.691 |
| walker |  | 3299 | 36 | Code::CodeKey { rung: Names, file: rich/_log_render.py, decl: 0, sub: 0, line: 0 } |  |  | 0.691 |
| walker |  | 3319 | 20 | Code::CodeKey { rung: Decl, file: rich/_log_render.py, decl: 2, sub: 0, line: 14 } |  |  | 0.691 |
| walker |  | 3355 | 36 | Code::CodeKey { rung: Names, file: rich/progress_bar.py, decl: 0, sub: 0, line: 0 } |  |  | 0.691 |
| walker |  | 3391 | 36 | Code::CodeKey { rung: Decl, file: rich/region.py, decl: 1, sub: 0, line: 4 } |  |  | 0.691 |
| walker |  | 3429 | 38 | Code::CodeKey { rung: Decl, file: rich/json.py, decl: 1, sub: 0, line: 9 } |  |  | 0.691 |
| walker |  | 3469 | 40 | Code::CodeKey { rung: Names, file: rich/live_render.py, decl: 0, sub: 0, line: 0 } |  |  | 0.691 |
| walker |  | 3509 | 40 | Code::CodeKey { rung: Names, file: rich/tree.py, decl: 0, sub: 0, line: 0 } |  |  | 0.691 |
| walker |  | 3551 | 42 | Code::CodeKey { rung: Names, file: rich/containers.py, decl: 0, sub: 0, line: 0 } |  |  | 0.692 |
| ns | 3573 |  | 447 | Console.__init__ full keyword surface | 2.8 | 2.7 | 0.653 |
| walker |  | 3591 | 40 | Code::CodeKey { rung: Decl, file: rich/screen.py, decl: 1, sub: 0, line: 18 } |  |  | 0.653 |
| walker |  | 3635 | 44 | Code::CodeKey { rung: Names, file: rich/_palettes.py, decl: 0, sub: 0, line: 0 } |  |  | 0.653 |
| walker |  | 3680 | 45 | Code::CodeKey { rung: Names, file: rich/table.py, decl: 0, sub: 0, line: 0 } |  |  | 0.654 |
| walker |  | 3723 | 43 | Code::CodeKey { rung: Decl, file: rich/measure.py, decl: 8, sub: 0, line: 125 } |  |  | 0.654 |
| walker |  | 3769 | 46 | Code::CodeKey { rung: Decl, file: rich/columns.py, decl: 1, sub: 0, line: 16 } |  |  | 0.654 |
| ns | 3780 |  | 207 | ConsoleOptions dataclass — the per-render context | 2.9 | 2.4 | 0.637 |
| walker |  | 3815 | 46 | Code::CodeKey { rung: Decl, file: rich/pager.py, decl: 3, sub: 0, line: 17 } |  |  | 0.637 |
| walker |  | 3827 | 12 | Code::CodeKey { rung: Body, file: rich/__main__.py, decl: 3, sub: 0, line: 33 } |  |  | 0.637 |
| walker |  | 3876 | 49 | Code::CodeKey { rung: Names, file: rich/emoji.py, decl: 0, sub: 0, line: 0 } |  |  | 0.637 |
| walker |  | 3927 | 51 | Code::CodeKey { rung: Names, file: rich/padding.py, decl: 0, sub: 0, line: 0 } |  |  | 0.637 |
| walker |  | 3953 | 26 | Code::CodeKey { rung: Decl, file: rich/columns.py, decl: 4, sub: 0, line: 62 } |  |  | 0.637 |
| walker |  | 4006 | 53 | Code::CodeKey { rung: Decl, file: rich/styled.py, decl: 1, sub: 0, line: 11 } |  |  | 0.637 |
| ns | 4016 |  | 236 | Console.print full keyword surface | 2.10 | 2.5 | 0.620 |
| walker |  | 4033 | 27 | Code::CodeKey { rung: Decl, file: rich/styled.py, decl: 4, sub: 0, line: 31 } |  |  | 0.620 |
| walker |  | 4089 | 56 | Code::CodeKey { rung: Names, file: rich/_export_format.py, decl: 0, sub: 0, line: 0 } |  |  | 0.620 |
| walker |  | 4146 | 57 | Code::CodeKey { rung: Names, file: rich/segment.py, decl: 0, sub: 0, line: 0 } |  |  | 0.620 |
| ns | 4154 |  | 138 | Table, Panel, Columns, Align, Padding, Rule, Constrain, Styled, Box classes | 3.1 |  | 0.621 |
| walker |  | 4186 | 40 | Code::CodeKey { rung: Decl, file: rich/segment.py, decl: 31, sub: 0, line: 724 } |  |  | 0.621 |
| walker |  | 4227 | 41 | Code::CodeKey { rung: Decl, file: rich/segment.py, decl: 28, sub: 0, line: 699 } |  |  | 0.621 |
| walker |  | 4271 | 44 | Code::CodeKey { rung: Decl, file: rich/segment.py, decl: 2, sub: 0, line: 53 } |  |  | 0.621 |
| walker |  | 4300 | 29 | Code::CodeKey { rung: Decl, file: rich/screen.py, decl: 3, sub: 0, line: 40 } |  |  | 0.621 |
| walker |  | 4329 | 29 | Code::CodeKey { rung: Decl, file: rich/segment.py, decl: 30, sub: 0, line: 712 } |  |  | 0.621 |
| walker |  | 4358 | 29 | Code::CodeKey { rung: Decl, file: rich/segment.py, decl: 33, sub: 0, line: 736 } |  |  | 0.621 |
| walker |  | 4387 | 29 | Code::CodeKey { rung: Decl, file: rich/styled.py, decl: 3, sub: 0, line: 23 } |  |  | 0.621 |
| walker |  | 4445 | 58 | Code::CodeKey { rung: Names, file: rich/_ratio.py, decl: 0, sub: 0, line: 0 } |  |  | 0.621 |
| ns | 4449 |  | 295 | Every predefined Box border style | 3.2 |  | 0.604 |
| walker |  | 4478 | 33 | Code::CodeKey { rung: Decl, file: rich/_ratio.py, decl: 4, sub: 0, line: 107 } |  |  | 0.604 |
| walker |  | 4512 | 34 | Code::CodeKey { rung: Decl, file: rich/_ratio.py, decl: 3, sub: 0, line: 75 } |  |  | 0.604 |
| walker |  | 4550 | 38 | Code::CodeKey { rung: Decl, file: rich/_ratio.py, decl: 1, sub: 0, line: 6 } |  |  | 0.604 |
| ns | 4578 |  | 129 | Text, markup and container primitives | 3.3 |  | 0.596 |
| walker |  | 4606 | 56 | Code::CodeKey { rung: Decl, file: rich/constrain.py, decl: 1, sub: 0, line: 10 } |  |  | 0.596 |
| ns | 4618 |  | 40 | Style module symbols | 3.4 |  | 0.593 |
| walker |  | 4634 | 28 | Code::CodeKey { rung: Decl, file: rich/constrain.py, decl: 4, sub: 0, line: 31 } |  |  | 0.593 |
| walker |  | 4663 | 29 | Code::CodeKey { rung: Decl, file: rich/constrain.py, decl: 3, sub: 0, line: 22 } |  |  | 0.593 |
| walker |  | 4863 | 200 | Fs::DirListing { dir: rich/_unicode_data } |  |  | 0.595 |
| walker |  | 4873 | 10 | Code::CodeKey { rung: Names, file: rich/_unicode_data/_versions.py, decl: 0, sub: 0, line: 0 } |  |  | 0.595 |
| walker |  | 4887 | 14 | Code::CodeKey { rung: Names, file: rich/_unicode_data/unicode10-0-0.py, decl: 0, sub: 0, line: 0 } |  |  | 0.595 |
| walker |  | 4901 | 14 | Code::CodeKey { rung: Names, file: rich/_unicode_data/unicode11-0-0.py, decl: 0, sub: 0, line: 0 } |  |  | 0.595 |
| walker |  | 4915 | 14 | Code::CodeKey { rung: Names, file: rich/_unicode_data/unicode12-0-0.py, decl: 0, sub: 0, line: 0 } |  |  | 0.595 |
| walker |  | 4929 | 14 | Code::CodeKey { rung: Names, file: rich/_unicode_data/unicode12-1-0.py, decl: 0, sub: 0, line: 0 } |  |  | 0.595 |
| walker |  | 4943 | 14 | Code::CodeKey { rung: Names, file: rich/_unicode_data/unicode13-0-0.py, decl: 0, sub: 0, line: 0 } |  |  | 0.595 |
| walker |  | 4957 | 14 | Code::CodeKey { rung: Names, file: rich/_unicode_data/unicode14-0-0.py, decl: 0, sub: 0, line: 0 } |  |  | 0.595 |
| walker |  | 4971 | 14 | Code::CodeKey { rung: Names, file: rich/_unicode_data/unicode15-0-0.py, decl: 0, sub: 0, line: 0 } |  |  | 0.595 |
| walker |  | 4985 | 14 | Code::CodeKey { rung: Names, file: rich/_unicode_data/unicode15-1-0.py, decl: 0, sub: 0, line: 0 } |  |  | 0.595 |
| ns | 4986 |  | 368 | Style method roster | 3.5 | 3.4 | 0.571 |
| walker |  | 4999 | 14 | Code::CodeKey { rung: Names, file: rich/_unicode_data/unicode16-0-0.py, decl: 0, sub: 0, line: 0 } |  |  | 0.571 |
| walker |  | 5013 | 14 | Code::CodeKey { rung: Names, file: rich/_unicode_data/unicode17-0-0.py, decl: 0, sub: 0, line: 0 } |  |  | 0.571 |
| walker |  | 5027 | 14 | Code::CodeKey { rung: Names, file: rich/_unicode_data/unicode4-1-0.py, decl: 0, sub: 0, line: 0 } |  |  | 0.571 |
| walker |  | 5041 | 14 | Code::CodeKey { rung: Names, file: rich/_unicode_data/unicode5-0-0.py, decl: 0, sub: 0, line: 0 } |  |  | 0.571 |
| walker |  | 5055 | 14 | Code::CodeKey { rung: Names, file: rich/_unicode_data/unicode5-1-0.py, decl: 0, sub: 0, line: 0 } |  |  | 0.571 |
| walker |  | 5069 | 14 | Code::CodeKey { rung: Names, file: rich/_unicode_data/unicode5-2-0.py, decl: 0, sub: 0, line: 0 } |  |  | 0.571 |
| walker |  | 5083 | 14 | Code::CodeKey { rung: Names, file: rich/_unicode_data/unicode6-0-0.py, decl: 0, sub: 0, line: 0 } |  |  | 0.571 |
| walker |  | 5097 | 14 | Code::CodeKey { rung: Names, file: rich/_unicode_data/unicode6-1-0.py, decl: 0, sub: 0, line: 0 } |  |  | 0.571 |
| walker |  | 5111 | 14 | Code::CodeKey { rung: Names, file: rich/_unicode_data/unicode6-2-0.py, decl: 0, sub: 0, line: 0 } |  |  | 0.571 |
| walker |  | 5125 | 14 | Code::CodeKey { rung: Names, file: rich/_unicode_data/unicode6-3-0.py, decl: 0, sub: 0, line: 0 } |  |  | 0.571 |
| walker |  | 5139 | 14 | Code::CodeKey { rung: Names, file: rich/_unicode_data/unicode7-0-0.py, decl: 0, sub: 0, line: 0 } |  |  | 0.571 |
| walker |  | 5153 | 14 | Code::CodeKey { rung: Names, file: rich/_unicode_data/unicode8-0-0.py, decl: 0, sub: 0, line: 0 } |  |  | 0.571 |
| walker |  | 5167 | 14 | Code::CodeKey { rung: Names, file: rich/_unicode_data/unicode9-0-0.py, decl: 0, sub: 0, line: 0 } |  |  | 0.571 |
| ns | 5233 |  | 247 | Colour system, palettes and themes | 3.6 |  | 0.562 |
| ns | 5276 |  | 43 | Segment — the atomic unit of rendered output | 3.7 |  | 0.565 |
| walker |  | 5294 | 127 | Code::CodeKey { rung: Names, file: rich/_unicode_data/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.565 |
| walker |  | 5301 | 7 | Code::CodeKey { rung: Decl, file: rich/_unicode_data/__init__.py, decl: 4, sub: 0, line: 58 } |  |  | 0.565 |
| walker |  | 5353 | 52 | Code::CodeKey { rung: Decl, file: rich/_unicode_data/__init__.py, decl: 1, sub: 0, line: 20 } |  |  | 0.565 |
| walker |  | 5410 | 57 | Code::CodeKey { rung: Decl, file: rich/_windows.py, decl: 1, sub: 0, line: 5 } |  |  | 0.565 |
| walker |  | 5418 | 8 | Code::CodeKey { rung: Doc, file: rich/_windows.py, decl: 1, sub: 0, line: 5 } |  |  | 0.565 |
| walker |  | 5428 | 10 | Code::CodeKey { rung: Doc, file: rich/emoji.py, decl: 2, sub: 0, line: 16 } |  |  | 0.565 |
| walker |  | 5438 | 10 | Code::CodeKey { rung: Doc, file: rich/pager.py, decl: 1, sub: 0, line: 5 } |  |  | 0.565 |
| walker |  | 5498 | 60 | Code::CodeKey { rung: Decl, file: rich/theme.py, decl: 7, sub: 0, line: 81 } |  |  | 0.565 |
| ns | 5513 |  | 237 | Progress bars — functions, columns and classes | 3.8 |  | 0.551 |
| walker |  | 5564 | 66 | Code::CodeKey { rung: Names, file: rich/filesize.py, decl: 0, sub: 0, line: 0 } |  |  | 0.552 |
| walker |  | 5614 | 50 | Code::CodeKey { rung: Decl, file: rich/filesize.py, decl: 3, sub: 0, line: 52 } |  |  | 0.552 |
| walker |  | 5677 | 63 | Code::CodeKey { rung: Decl, file: rich/theme.py, decl: 1, sub: 0, line: 7 } |  |  | 0.552 |
| walker |  | 5685 | 8 | Code::CodeKey { rung: Decl, file: rich/theme.py, decl: 3, sub: 0, line: 29 } |  |  | 0.552 |
| walker |  | 5717 | 32 | Code::CodeKey { rung: Decl, file: rich/theme.py, decl: 2, sub: 0, line: 17 } |  |  | 0.552 |
| ns | 5723 |  | 210 | Live display, status, spinners, bars, screen | 3.9 |  | 0.550 |
| walker |  | 5728 | 11 | Code::CodeKey { rung: Doc, file: rich/_stack.py, decl: 2, sub: 0, line: 6 } |  |  | 0.550 |
| walker |  | 5793 | 65 | Code::CodeKey { rung: Decl, file: rich/spinner.py, decl: 1, sub: 0, line: 13 } |  |  | 0.550 |
| walker |  | 5820 | 27 | Code::CodeKey { rung: Decl, file: rich/spinner.py, decl: 4, sub: 0, line: 55 } |  |  | 0.550 |
| walker |  | 5849 | 29 | Code::CodeKey { rung: Decl, file: rich/spinner.py, decl: 3, sub: 0, line: 50 } |  |  | 0.550 |
| ns | 5906 |  | 183 | Syntax highlighting | 3.10 |  | 0.543 |
| walker |  | 5921 | 72 | Code::CodeKey { rung: Names, file: rich/align.py, decl: 0, sub: 0, line: 0 } |  |  | 0.548 |
| walker |  | 5972 | 51 | Code::CodeKey { rung: Decl, file: rich/align.py, decl: 11, sub: 0, line: 242 } |  |  | 0.548 |
| walker |  | 5999 | 27 | Code::CodeKey { rung: Decl, file: rich/align.py, decl: 15, sub: 0, line: 292 } |  |  | 0.548 |
| walker |  | 6028 | 29 | Code::CodeKey { rung: Decl, file: rich/align.py, decl: 14, sub: 0, line: 265 } |  |  | 0.548 |
| walker |  | 6040 | 12 | Code::CodeKey { rung: Doc, file: rich/region.py, decl: 1, sub: 0, line: 4 } |  |  | 0.548 |
| ns | 6108 |  | 202 | Markdown element hierarchy | 3.11 |  | 0.537 |
| walker |  | 6115 | 75 | Code::CodeKey { rung: Names, file: rich/ansi.py, decl: 0, sub: 0, line: 0 } |  |  | 0.537 |
| walker |  | 6171 | 56 | Code::CodeKey { rung: Decl, file: rich/ansi.py, decl: 5, sub: 0, line: 120 } |  |  | 0.537 |
| walker |  | 6183 | 12 | Code::CodeKey { rung: Doc, file: rich/ansi.py, decl: 5, sub: 0, line: 120 } |  |  | 0.537 |
| ns | 6241 |  | 133 | Tracebacks and the logging handler | 3.12 |  | 0.532 |
| walker |  | 6256 | 73 | Code::CodeKey { rung: Decl, file: rich/table.py, decl: 5, sub: 0, line: 131 } |  |  | 0.532 |
| walker |  | 6265 | 9 | Code::CodeKey { rung: Doc, file: rich/table.py, decl: 5, sub: 0, line: 131 } |  |  | 0.532 |
| walker |  | 6340 | 75 | Code::CodeKey { rung: Decl, file: rich/containers.py, decl: 2, sub: 0, line: 30 } |  |  | 0.532 |
| walker |  | 6368 | 28 | Code::CodeKey { rung: Decl, file: rich/containers.py, decl: 5, sub: 0, line: 46 } |  |  | 0.532 |
| walker |  | 6397 | 29 | Code::CodeKey { rung: Decl, file: rich/containers.py, decl: 3, sub: 0, line: 33 } |  |  | 0.532 |
| walker |  | 6426 | 29 | Code::CodeKey { rung: Decl, file: rich/containers.py, decl: 4, sub: 0, line: 40 } |  |  | 0.532 |
| ns | 6483 |  | 242 | Pretty printing, the repr protocol, inspect, Jupyter | 3.13 |  | 0.521 |
| walker |  | 6505 | 79 | Code::CodeKey { rung: Names, file: rich/_wrap.py, decl: 0, sub: 0, line: 0 } |  |  | 0.521 |
| walker |  | 6560 | 55 | Code::CodeKey { rung: Body, file: rich/__init__.py, decl: 4, sub: 0, line: 53 } |  |  | 0.521 |
| walker |  | 6636 | 76 | Code::CodeKey { rung: Decl, file: rich/rule.py, decl: 1, sub: 0, line: 12 } |  |  | 0.521 |
| walker |  | 6661 | 25 | Code::CodeKey { rung: Decl, file: rich/rule.py, decl: 6, sub: 0, line: 111 } |  |  | 0.521 |
| ns | 6679 |  | 196 | Tree, JSON, emoji, highlighters, prompts | 3.14 |  | 0.514 |
| walker |  | 6687 | 26 | Code::CodeKey { rung: Decl, file: rich/rule.py, decl: 4, sub: 0, line: 49 } |  |  | 0.514 |
| ns | 6765 |  | 86 | Layout engine | 3.15 |  | 0.510 |
| walker |  | 6938 | 251 | Code::CodeKey { rung: Names, file: rich/console.py, decl: 0, sub: 0, line: 0 } |  |  | 0.515 |
| ns | 6939 |  | 174 | Width measurement, cell arithmetic, wrapping, ratios | 3.16 |  | 0.512 |
| walker |  | 6943 | 5 | Code::CodeKey { rung: Decl, file: rich/console.py, decl: 7, sub: 0, line: 73 } |  |  | 0.512 |
| walker |  | 6963 | 20 | Code::CodeKey { rung: Decl, file: rich/console.py, decl: 21, sub: 0, line: 246 } |  |  | 0.512 |
| walker |  | 7013 | 50 | Code::CodeKey { rung: Decl, file: rich/console.py, decl: 12, sub: 0, line: 103 } |  |  | 0.512 |
| walker |  | 7022 | 9 | Code::CodeKey { rung: Doc, file: rich/console.py, decl: 12, sub: 0, line: 103 } |  |  | 0.512 |
| ns | 7034 |  | 95 | Complete exception hierarchy | 3.17 |  | 0.508 |
| walker |  | 7054 | 32 | Code::CodeKey { rung: Decl, file: rich/console.py, decl: 22, sub: 0, line: 250 } |  |  | 0.509 |
| ns | 7112 |  | 78 | Control codes | 3.18 |  | 0.507 |
| walker |  | 7113 | 59 | Code::CodeKey { rung: Doc, file: rich/_unicode_data/__init__.py, decl: 4, sub: 0, line: 58 } |  |  | 0.507 |
| walker |  | 7196 | 83 | Code::CodeKey { rung: Names, file: rich/protocol.py, decl: 0, sub: 0, line: 0 } |  |  | 0.507 |
| walker |  | 7275 | 79 | Code::CodeKey { rung: Decl, file: rich/panel.py, decl: 1, sub: 0, line: 17 } |  |  | 0.507 |
| walker |  | 7283 | 8 | Code::CodeKey { rung: Decl, file: rich/panel.py, decl: 4, sub: 0, line: 109 } |  |  | 0.507 |
| walker |  | 7291 | 8 | Code::CodeKey { rung: Decl, file: rich/panel.py, decl: 5, sub: 0, line: 125 } |  |  | 0.507 |
| walker |  | 7319 | 28 | Code::CodeKey { rung: Decl, file: rich/panel.py, decl: 7, sub: 0, line: 277 } |  |  | 0.507 |
| walker |  | 7348 | 29 | Code::CodeKey { rung: Decl, file: rich/panel.py, decl: 6, sub: 0, line: 141 } |  |  | 0.507 |
| ns | 7375 |  | 263 | Protocol helpers, file plumbing and small utilities | 3.19 |  | 0.513 |
| walker |  | 7429 | 81 | Code::CodeKey { rung: Decl, file: rich/align.py, decl: 3, sub: 0, line: 17 } |  |  | 0.513 |
| walker |  | 7456 | 27 | Code::CodeKey { rung: Decl, file: rich/align.py, decl: 10, sub: 0, line: 235 } |  |  | 0.513 |
| walker |  | 7485 | 29 | Code::CodeKey { rung: Decl, file: rich/align.py, decl: 9, sub: 0, line: 143 } |  |  | 0.513 |
| walker |  | 7528 | 43 | Code::CodeKey { rung: Decl, file: rich/align.py, decl: 12, sub: 0, line: 254 } |  |  | 0.513 |
| walker |  | 7571 | 43 | Code::CodeKey { rung: Decl, file: rich/theme.py, decl: 5, sub: 0, line: 59 } |  |  | 0.513 |
| ns | 7612 |  | 237 | Legacy Windows console support | 3.20 |  | 0.505 |
| walker |  | 7616 | 45 | Code::CodeKey { rung: Decl, file: rich/theme.py, decl: 4, sub: 0, line: 37 } |  |  | 0.505 |
| walker |  | 7703 | 87 | Code::CodeKey { rung: Decl, file: rich/palette.py, decl: 1, sub: 0, line: 11 } |  |  | 0.505 |
| walker |  | 7718 | 15 | Code::CodeKey { rung: Decl, file: rich/palette.py, decl: 5, sub: 0, line: 44 } |  |  | 0.505 |
| walker |  | 7728 | 10 | Code::CodeKey { rung: Doc, file: rich/palette.py, decl: 1, sub: 0, line: 11 } |  |  | 0.505 |
| ns | 7766 |  | 154 | Makefile — the complete set of dev commands | 4.1 |  | 0.515 |
| walker |  | 7822 | 94 | Code::CodeKey { rung: Names, file: rich/_inspect.py, decl: 0, sub: 0, line: 0 } |  |  | 0.516 |
| walker |  | 7848 | 26 | Code::CodeKey { rung: Decl, file: rich/_inspect.py, decl: 11, sub: 0, line: 262 } |  |  | 0.516 |
| walker |  | 7942 | 94 | Code::CodeKey { rung: Names, file: rich/control.py, decl: 0, sub: 0, line: 0 } |  |  | 0.523 |
| walker |  | 7976 | 34 | Code::CodeKey { rung: Decl, file: rich/control.py, decl: 18, sub: 0, line: 181 } |  |  | 0.523 |
| walker |  | 8011 | 35 | Code::CodeKey { rung: Decl, file: rich/control.py, decl: 19, sub: 0, line: 195 } |  |  | 0.523 |
| walker |  | 8069 | 58 | Code::CodeKey { rung: Decl, file: rich/control.py, decl: 3, sub: 0, line: 20 } |  |  | 0.523 |
| walker |  | 8137 | 68 | Code::CodeKey { rung: Decl, file: rich/control.py, decl: 1, sub: 0, line: 9 } |  |  | 0.523 |
| ns | 8144 |  | 378 | tests/ directory listing (complete) | 4.2 |  | 0.497 |
| walker |  | 8160 | 23 | Code::CodeKey { rung: Decl, file: rich/control.py, decl: 2, sub: 0, line: 16 } |  |  | 0.497 |
| walker |  | 8250 | 90 | Code::CodeKey { rung: Decl, file: rich/ansi.py, decl: 1, sub: 0, line: 10 } |  |  | 0.497 |
| ns | 8310 |  | 166 | examples/ directory listing (complete) | 4.3 |  | 0.518 |
| walker |  | 8345 | 95 | Code::CodeKey { rung: Names, file: rich/_loop.py, decl: 0, sub: 0, line: 0 } |  |  | 0.522 |
| walker |  | 8441 | 96 | Code::CodeKey { rung: Names, file: rich/_emoji_replace.py, decl: 0, sub: 0, line: 0 } |  |  | 0.524 |
| walker |  | 8533 | 92 | Code::CodeKey { rung: Decl, file: rich/live_render.py, decl: 2, sub: 0, line: 13 } |  |  | 0.524 |
| walker |  | 8541 | 8 | Code::CodeKey { rung: Decl, file: rich/live_render.py, decl: 4, sub: 0, line: 32 } |  |  | 0.524 |
| ns | 8550 |  | 240 | Type-check and test configuration, and the global test fixture | 4.4 |  | 0.516 |
| walker |  | 8567 | 26 | Code::CodeKey { rung: Decl, file: rich/live_render.py, decl: 8, sub: 0, line: 86 } |  |  | 0.516 |
| walker |  | 8583 | 16 | Code::CodeKey { rung: Doc, file: rich/_ratio.py, decl: 1, sub: 0, line: 6 } |  |  | 0.516 |
| walker |  | 8599 | 16 | Code::CodeKey { rung: Doc, file: rich/diagnose.py, decl: 1, sub: 0, line: 10 } |  |  | 0.516 |
| walker |  | 8615 | 16 | Code::CodeKey { rung: Doc, file: rich/filesize.py, decl: 2, sub: 0, line: 43 } |  |  | 0.516 |
| walker |  | 8631 | 16 | Code::CodeKey { rung: Doc, file: rich/protocol.py, decl: 2, sub: 0, line: 9 } |  |  | 0.516 |
| ns | 8685 |  | 135 | docs/source narrative pages (complete listing) | 4.5 |  | 0.530 |
| walker |  | 8733 | 102 | Code::CodeKey { rung: Names, file: rich/jupyter.py, decl: 0, sub: 0, line: 0 } |  |  | 0.532 |
| walker |  | 8763 | 30 | Code::CodeKey { rung: Decl, file: rich/jupyter.py, decl: 5, sub: 0, line: 36 } |  |  | 0.532 |
| walker |  | 8802 | 39 | Code::CodeKey { rung: Decl, file: rich/jupyter.py, decl: 2, sub: 0, line: 18 } |  |  | 0.532 |
| walker |  | 8853 | 51 | Code::CodeKey { rung: Decl, file: rich/jupyter.py, decl: 1, sub: 0, line: 13 } |  |  | 0.532 |
| walker |  | 8864 | 11 | Code::CodeKey { rung: Doc, file: rich/jupyter.py, decl: 9, sub: 0, line: 98 } |  |  | 0.532 |
| ns | 8896 |  | 211 | docs/source/reference and appendix listings (complete) | 4.6 |  | 0.519 |
| walker |  | 8972 | 108 | Code::CodeKey { rung: Names, file: rich/bar.py, decl: 0, sub: 0, line: 0 } |  |  | 0.527 |
| ns | 9012 |  | 116 | CONTRIBUTING.md section headings (complete) | 4.7 |  | 0.523 |
| walker |  | 9023 | 51 | Code::CodeKey { rung: Decl, file: rich/bar.py, decl: 4, sub: 0, line: 17 } |  |  | 0.523 |
| walker |  | 9048 | 25 | Code::CodeKey { rung: Decl, file: rich/bar.py, decl: 8, sub: 0, line: 86 } |  |  | 0.523 |
| walker |  | 9074 | 26 | Code::CodeKey { rung: Decl, file: rich/bar.py, decl: 7, sub: 0, line: 48 } |  |  | 0.523 |
| walker |  | 9182 | 108 | Code::CodeKey { rung: Names, file: rich/terminal_theme.py, decl: 0, sub: 0, line: 0 } |  |  | 0.529 |
| ns | 9183 |  | 171 | tox.ini — supported interpreters and the lint/docs environments | 4.8 |  | 0.524 |
| walker |  | 9193 | 11 | Code::CodeKey { rung: Decl, file: rich/terminal_theme.py, decl: 2, sub: 0, line: 9 } |  |  | 0.524 |
| walker |  | 9296 | 103 | Code::CodeKey { rung: Decl, file: rich/padding.py, decl: 2, sub: 0, line: 19 } |  |  | 0.524 |
| walker |  | 9304 | 8 | Code::CodeKey { rung: Decl, file: rich/padding.py, decl: 4, sub: 0, line: 46 } |  |  | 0.524 |
| walker |  | 9312 | 8 | Code::CodeKey { rung: Decl, file: rich/padding.py, decl: 5, sub: 0, line: 60 } |  |  | 0.524 |
| walker |  | 9340 | 28 | Code::CodeKey { rung: Decl, file: rich/padding.py, decl: 8, sub: 0, line: 125 } |  |  | 0.524 |
| walker |  | 9369 | 29 | Code::CodeKey { rung: Decl, file: rich/padding.py, decl: 7, sub: 0, line: 79 } |  |  | 0.524 |
| ns | 9473 |  | 290 | CHANGELOG — format convention and the most recent releases | 4.9 |  | 0.518 |
| walker |  | 9478 | 109 | Code::CodeKey { rung: Names, file: rich/highlighter.py, decl: 0, sub: 0, line: 0 } |  |  | 0.523 |
| walker |  | 9496 | 18 | Code::CodeKey { rung: Decl, file: rich/highlighter.py, decl: 5, sub: 0, line: 50 } |  |  | 0.523 |
| walker |  | 9537 | 41 | Code::CodeKey { rung: Decl, file: rich/highlighter.py, decl: 2, sub: 0, line: 17 } |  |  | 0.523 |
| walker |  | 9546 | 9 | Code::CodeKey { rung: Decl, file: rich/highlighter.py, decl: 4, sub: 0, line: 41 } |  |  | 0.523 |
| walker |  | 9598 | 52 | Code::CodeKey { rung: Decl, file: rich/highlighter.py, decl: 7, sub: 0, line: 61 } |  |  | 0.523 |
| walker |  | 9607 | 9 | Code::CodeKey { rung: Body, file: rich/_stack.py, decl: 4, sub: 0, line: 14 } |  |  | 0.523 |
| walker |  | 9616 | 9 | Code::CodeKey { rung: Body, file: rich/ansi.py, decl: 6, sub: 0, line: 123 } |  |  | 0.523 |
| walker |  | 9625 | 9 | Code::CodeKey { rung: Body, file: rich/json.py, decl: 4, sub: 0, line: 101 } |  |  | 0.523 |
| walker |  | 9634 | 9 | Code::CodeKey { rung: Body, file: rich/palette.py, decl: 2, sub: 0, line: 14 } |  |  | 0.520 |
| ns | 9634 |  | 161 | FAQ.md — every question heading | 4.10 |  | 0.520 |
| walker |  | 9689 | 55 | Code::CodeKey { rung: Decl, file: rich/live_render.py, decl: 3, sub: 0, line: 21 } |  |  | 0.520 |
| walker |  | 9701 | 12 | Code::CodeKey { rung: Doc, file: rich/highlighter.py, decl: 2, sub: 0, line: 17 } |  |  | 0.520 |
| walker |  | 9713 | 12 | Code::CodeKey { rung: Doc, file: rich/jupyter.py, decl: 8, sub: 0, line: 84 } |  |  | 0.520 |
| walker |  | 9725 | 12 | Code::CodeKey { rung: Doc, file: rich/pager.py, decl: 3, sub: 0, line: 17 } |  |  | 0.520 |
| walker |  | 9743 | 18 | Code::CodeKey { rung: Doc, file: rich/_timer.py, decl: 1, sub: 0, line: 12 } |  |  | 0.520 |
| ns | 9781 |  | 147 | benchmarks/, tools/, questions/ and .faq/ listings | 4.11 |  | 0.531 |
| walker |  | 9799 | 56 | Code::CodeKey { rung: Decl, file: rich/screen.py, decl: 2, sub: 0, line: 28 } |  |  | 0.531 |
| walker |  | 9911 | 112 | Code::CodeKey { rung: Names, file: rich/errors.py, decl: 0, sub: 0, line: 0 } |  |  | 0.538 |
| walker |  | 9919 | 8 | Code::CodeKey { rung: Doc, file: rich/errors.py, decl: 4, sub: 0, line: 13 } |  |  | 0.538 |
| walker |  | 9928 | 9 | Code::CodeKey { rung: Doc, file: rich/errors.py, decl: 2, sub: 0, line: 5 } |  |  | 0.538 |
| walker |  | 9937 | 9 | Code::CodeKey { rung: Doc, file: rich/errors.py, decl: 3, sub: 0, line: 9 } |  |  | 0.538 |
| walker |  | 9946 | 9 | Code::CodeKey { rung: Doc, file: rich/errors.py, decl: 5, sub: 0, line: 17 } |  |  | 0.538 |
| walker |  | 9955 | 9 | Code::CodeKey { rung: Doc, file: rich/errors.py, decl: 7, sub: 0, line: 25 } |  |  | 0.538 |
| ns | 9981 |  | 200 | rich/_unicode_data listing (complete) | 4.12 |  | 0.546 |
