Score(3000)=0.739 I=0.866 C=0.631 ns_rows≤3K=15/51 grid(1000/1442/2080/3000/4327/6240/9000)=0.549/0.522/0.707/0.739/0.647/0.542/0.544

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
| ns | 2019 |  | 290 | The console protocol: RichCast, ConsoleRenderable, RenderableType, RenderResult | 2.3 |  | 0.707 |
| walker |  | 2129 | 200 | Fs::DirListing { dir: rich/_unicode_data } |  |  | 0.709 |
| walker |  | 2174 | 45 | Markdown::ReadmeHeadline { file: questions/README.md } |  |  | 0.709 |
| walker |  | 2200 | 26 | Code::CodeKey { rung: Names, file: rich/__main__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.709 |
| walker |  | 2222 | 22 | Code::CodeKey { rung: Decl, file: rich/__main__.py, decl: 1, sub: 0, line: 18 } |  |  | 0.709 |
| walker |  | 2248 | 26 | Code::CodeKey { rung: Decl, file: rich/__main__.py, decl: 2, sub: 0, line: 19 } |  |  | 0.709 |
| walker |  | 2274 | 26 | Code::CodeKey { rung: Decl, file: rich/__main__.py, decl: 3, sub: 0, line: 33 } |  |  | 0.709 |
| ns | 2281 |  | 262 | console.py module-level symbol roster | 2.4 |  | 0.673 |
| walker |  | 2286 | 12 | Code::CodeKey { rung: Body, file: rich/__main__.py, decl: 3, sub: 0, line: 33 } |  |  | 0.673 |
| walker |  | 2303 | 17 | Code::CodeKey { rung: Doc, file: rich/__main__.py, decl: 4, sub: 0, line: 39 } |  |  | 0.673 |
| walker |  | 2485 | 182 | Fs::DirListing { dir: docs/source/reference } |  |  | 0.676 |
| ns | 2525 |  | 244 | Console method roster — rendering and output (lines 1092–1652) | 2.5 |  | 0.645 |
| walker |  | 2644 | 159 | Code::CodeKey { rung: Names, file: rich/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.681 |
| walker |  | 2711 | 67 | Code::CodeKey { rung: Decl, file: rich/__init__.py, decl: 4, sub: 0, line: 53 } |  |  | 0.700 |
| ns | 2735 |  | 210 | Console method roster — JSON, screen updates, exceptions, logging, export (1758–2606) | 2.6 |  | 0.677 |
| walker |  | 2850 | 139 | Code::CodeKey { rung: Decl, file: rich/__init__.py, decl: 6, sub: 0, line: 120 } |  |  | 0.694 |
| walker |  | 2997 | 147 | Code::CodeKey { rung: Decl, file: rich/__init__.py, decl: 5, sub: 0, line: 77 } |  |  | 0.739 |
| walker |  | 3052 | 55 | Code::CodeKey { rung: Body, file: rich/__init__.py, decl: 4, sub: 0, line: 53 } |  |  | 0.739 |
| walker |  | 3122 | 70 | Code::CodeKey { rung: Doc, file: rich/__init__.py, decl: 2, sub: 0, line: 23 } |  |  | 0.739 |
| ns | 3126 |  | 391 | Console method roster — construction, context management, properties (617–1084) | 2.7 |  | 0.694 |
| walker |  | 3205 | 83 | Code::CodeKey { rung: Doc, file: rich/__init__.py, decl: 3, sub: 0, line: 39 } |  |  | 0.694 |
| walker |  | 3263 | 58 | Plaintext::Whole { file: docs/requirements.txt } |  |  | 0.694 |
| walker |  | 3390 | 127 | Code::CodeKey { rung: Names, file: rich/_unicode_data/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.694 |
| walker |  | 3397 | 7 | Code::CodeKey { rung: Decl, file: rich/_unicode_data/__init__.py, decl: 4, sub: 0, line: 58 } |  |  | 0.694 |
| walker |  | 3449 | 52 | Code::CodeKey { rung: Decl, file: rich/_unicode_data/__init__.py, decl: 1, sub: 0, line: 20 } |  |  | 0.694 |
| walker |  | 3508 | 59 | Code::CodeKey { rung: Doc, file: rich/_unicode_data/__init__.py, decl: 4, sub: 0, line: 58 } |  |  | 0.694 |
| ns | 3573 |  | 447 | Console.__init__ full keyword surface | 2.8 | 2.7 | 0.656 |
| walker |  | 3604 | 96 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.668 |
| ns | 3780 |  | 207 | ConsoleOptions dataclass — the per-render context | 2.9 | 2.4 | 0.650 |
| walker |  | 3847 | 243 | Toml::Config { file: pyproject.toml } |  |  | 0.677 |
| walker |  | 3973 | 126 | Markdown::Section { file: AI_POLICY.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.677 |
| walker |  | 3984 | 11 | Code::CodeKey { rung: Names, file: rich/_emoji_codes.py, decl: 0, sub: 0, line: 0 } |  |  | 0.677 |
| walker |  | 3995 | 11 | Code::CodeKey { rung: Names, file: rich/json.py, decl: 0, sub: 0, line: 0 } |  |  | 0.677 |
| ns | 4016 |  | 236 | Console.print full keyword surface | 2.10 | 2.5 | 0.659 |
| walker |  | 4033 | 38 | Code::CodeKey { rung: Decl, file: rich/json.py, decl: 1, sub: 0, line: 9 } |  |  | 0.659 |
| walker |  | 4042 | 9 | Code::CodeKey { rung: Body, file: rich/json.py, decl: 4, sub: 0, line: 101 } |  |  | 0.659 |
| ns | 4154 |  | 138 | Table, Panel, Columns, Align, Padding, Rule, Constrain, Styled, Box classes | 3.1 |  | 0.647 |
| walker |  | 4176 | 134 | Code::CodeKey { rung: Decl, file: rich/json.py, decl: 2, sub: 0, line: 25 } |  |  | 0.647 |
| walker |  | 4319 | 143 | Code::CodeKey { rung: Decl, file: rich/json.py, decl: 3, sub: 0, line: 53 } |  |  | 0.647 |
| walker |  | 4330 | 11 | Code::CodeKey { rung: Names, file: rich/palette.py, decl: 0, sub: 0, line: 0 } |  |  | 0.647 |
| walker |  | 4417 | 87 | Code::CodeKey { rung: Decl, file: rich/palette.py, decl: 1, sub: 0, line: 11 } |  |  | 0.647 |
| walker |  | 4432 | 15 | Code::CodeKey { rung: Decl, file: rich/palette.py, decl: 5, sub: 0, line: 44 } |  |  | 0.647 |
| walker |  | 4441 | 9 | Code::CodeKey { rung: Body, file: rich/palette.py, decl: 2, sub: 0, line: 14 } |  |  | 0.647 |
| ns | 4449 |  | 295 | Every predefined Box border style | 3.2 |  | 0.629 |
| walker |  | 4451 | 10 | Code::CodeKey { rung: Doc, file: rich/palette.py, decl: 1, sub: 0, line: 11 } |  |  | 0.629 |
| walker |  | 4464 | 13 | Code::CodeKey { rung: Body, file: rich/palette.py, decl: 3, sub: 0, line: 17 } |  |  | 0.629 |
| walker |  | 4475 | 11 | Code::CodeKey { rung: Names, file: rich/screen.py, decl: 0, sub: 0, line: 0 } |  |  | 0.629 |
| walker |  | 4515 | 40 | Code::CodeKey { rung: Decl, file: rich/screen.py, decl: 1, sub: 0, line: 18 } |  |  | 0.629 |
| walker |  | 4544 | 29 | Code::CodeKey { rung: Decl, file: rich/screen.py, decl: 3, sub: 0, line: 40 } |  |  | 0.629 |
| ns | 4578 |  | 129 | Text, markup and container primitives | 3.3 |  | 0.621 |
| walker |  | 4600 | 56 | Code::CodeKey { rung: Decl, file: rich/screen.py, decl: 2, sub: 0, line: 28 } |  |  | 0.621 |
| ns | 4618 |  | 40 | Style module symbols | 3.4 |  | 0.617 |
| walker |  | 4650 | 50 | Code::CodeKey { rung: Body, file: rich/screen.py, decl: 2, sub: 0, line: 28 } |  |  | 0.617 |
| walker |  | 4661 | 11 | Code::CodeKey { rung: Names, file: rich/spinner.py, decl: 0, sub: 0, line: 0 } |  |  | 0.617 |
| walker |  | 4726 | 65 | Code::CodeKey { rung: Decl, file: rich/spinner.py, decl: 1, sub: 0, line: 13 } |  |  | 0.617 |
| walker |  | 4753 | 27 | Code::CodeKey { rung: Decl, file: rich/spinner.py, decl: 4, sub: 0, line: 55 } |  |  | 0.617 |
| walker |  | 4782 | 29 | Code::CodeKey { rung: Decl, file: rich/spinner.py, decl: 3, sub: 0, line: 50 } |  |  | 0.617 |
| walker |  | 4792 | 10 | Code::CodeKey { rung: Body, file: rich/spinner.py, decl: 3, sub: 0, line: 50 } |  |  | 0.617 |
| walker |  | 4856 | 64 | Code::CodeKey { rung: Decl, file: rich/spinner.py, decl: 6, sub: 0, line: 95 } |  |  | 0.617 |
| walker |  | 4929 | 73 | Code::CodeKey { rung: Decl, file: rich/spinner.py, decl: 2, sub: 0, line: 26 } |  |  | 0.617 |
| walker |  | 4940 | 11 | Code::CodeKey { rung: Names, file: rich/styled.py, decl: 0, sub: 0, line: 0 } |  |  | 0.618 |
| ns | 4986 |  | 368 | Style method roster | 3.5 | 3.4 | 0.594 |
| walker |  | 4993 | 53 | Code::CodeKey { rung: Decl, file: rich/styled.py, decl: 1, sub: 0, line: 11 } |  |  | 0.594 |
| walker |  | 5020 | 27 | Code::CodeKey { rung: Decl, file: rich/styled.py, decl: 4, sub: 0, line: 31 } |  |  | 0.594 |
| walker |  | 5049 | 29 | Code::CodeKey { rung: Decl, file: rich/styled.py, decl: 3, sub: 0, line: 23 } |  |  | 0.594 |
| walker |  | 5065 | 16 | Code::CodeKey { rung: Body, file: rich/styled.py, decl: 4, sub: 0, line: 31 } |  |  | 0.594 |
| walker |  | 5085 | 20 | Code::CodeKey { rung: Body, file: rich/styled.py, decl: 2, sub: 0, line: 19 } |  |  | 0.594 |
| walker |  | 5150 | 65 | Code::CodeKey { rung: Doc, file: rich/styled.py, decl: 1, sub: 0, line: 11 } |  |  | 0.594 |
| walker |  | 5183 | 33 | Code::CodeKey { rung: Names, file: rich/theme.py, decl: 0, sub: 0, line: 0 } |  |  | 0.594 |
| ns | 5233 |  | 247 | Colour system, palettes and themes | 3.6 |  | 0.581 |
| walker |  | 5243 | 60 | Code::CodeKey { rung: Decl, file: rich/theme.py, decl: 7, sub: 0, line: 81 } |  |  | 0.581 |
| ns | 5276 |  | 43 | Segment — the atomic unit of rendered output | 3.7 |  | 0.579 |
| walker |  | 5306 | 63 | Code::CodeKey { rung: Decl, file: rich/theme.py, decl: 1, sub: 0, line: 7 } |  |  | 0.579 |
| walker |  | 5314 | 8 | Code::CodeKey { rung: Decl, file: rich/theme.py, decl: 3, sub: 0, line: 29 } |  |  | 0.579 |
| walker |  | 5346 | 32 | Code::CodeKey { rung: Decl, file: rich/theme.py, decl: 2, sub: 0, line: 17 } |  |  | 0.579 |
| walker |  | 5389 | 43 | Code::CodeKey { rung: Decl, file: rich/theme.py, decl: 5, sub: 0, line: 59 } |  |  | 0.579 |
| walker |  | 5434 | 45 | Code::CodeKey { rung: Decl, file: rich/theme.py, decl: 4, sub: 0, line: 37 } |  |  | 0.579 |
| walker |  | 5448 | 14 | Code::CodeKey { rung: Doc, file: rich/theme.py, decl: 6, sub: 0, line: 77 } |  |  | 0.579 |
| walker |  | 5464 | 16 | Code::CodeKey { rung: Doc, file: rich/theme.py, decl: 3, sub: 0, line: 29 } |  |  | 0.579 |
| walker |  | 5480 | 16 | Code::CodeKey { rung: Doc, file: rich/theme.py, decl: 10, sub: 0, line: 106 } |  |  | 0.579 |
| ns | 5513 |  | 237 | Progress bars — functions, columns and classes | 3.8 |  | 0.565 |
| walker |  | 5520 | 40 | Code::CodeKey { rung: Doc, file: rich/theme.py, decl: 7, sub: 0, line: 81 } |  |  | 0.565 |
| walker |  | 5587 | 67 | Code::CodeKey { rung: Doc, file: rich/theme.py, decl: 9, sub: 0, line: 92 } |  |  | 0.565 |
| walker |  | 5632 | 45 | Code::CodeKey { rung: Names, file: rich/table.py, decl: 0, sub: 0, line: 0 } |  |  | 0.568 |
| walker |  | 5705 | 73 | Code::CodeKey { rung: Decl, file: rich/table.py, decl: 5, sub: 0, line: 131 } |  |  | 0.568 |
| walker |  | 5714 | 9 | Code::CodeKey { rung: Doc, file: rich/table.py, decl: 5, sub: 0, line: 131 } |  |  | 0.568 |
| ns | 5723 |  | 210 | Live display, status, spinners, bars, screen | 3.9 |  | 0.560 |
| walker |  | 5779 | 65 | Code::CodeKey { rung: Decl, file: rich/table.py, decl: 6, sub: 0, line: 142 } |  |  | 0.560 |
| walker |  | 5790 | 11 | Code::CodeKey { rung: Doc, file: rich/table.py, decl: 6, sub: 0, line: 142 } |  |  | 0.560 |
| ns | 5906 |  | 183 | Syntax highlighting | 3.10 |  | 0.553 |
| ns | 6108 |  | 202 | Markdown element hierarchy | 3.11 |  | 0.542 |
| walker |  | 6113 | 323 | Code::CodeKey { rung: Decl, file: rich/table.py, decl: 7, sub: 0, line: 153 } |  |  | 0.542 |
| walker |  | 6121 | 8 | Code::CodeKey { rung: Decl, file: rich/table.py, decl: 10, sub: 0, line: 285 } |  |  | 0.542 |
| walker |  | 6129 | 8 | Code::CodeKey { rung: Decl, file: rich/table.py, decl: 13, sub: 0, line: 305 } |  |  | 0.542 |
| walker |  | 6137 | 8 | Code::CodeKey { rung: Decl, file: rich/table.py, decl: 16, sub: 0, line: 353 } |  |  | 0.542 |
| walker |  | 6146 | 9 | Code::CodeKey { rung: Decl, file: rich/table.py, decl: 11, sub: 0, line: 290 } |  |  | 0.542 |
| walker |  | 6155 | 9 | Code::CodeKey { rung: Decl, file: rich/table.py, decl: 17, sub: 0, line: 358 } |  |  | 0.542 |
| walker |  | 6182 | 27 | Code::CodeKey { rung: Decl, file: rich/table.py, decl: 15, sub: 0, line: 320 } |  |  | 0.542 |
| walker |  | 6190 | 8 | Code::CodeKey { rung: Decl, file: rich/table.py, decl: 12, sub: 0, line: 295 } |  |  | 0.542 |
| walker |  | 6219 | 29 | Code::CodeKey { rung: Decl, file: rich/table.py, decl: 21, sub: 0, line: 475 } |  |  | 0.542 |
| walker |  | 6228 | 9 | Code::CodeKey { rung: Doc, file: rich/table.py, decl: 11, sub: 0, line: 290 } |  |  | 0.542 |
| ns | 6241 |  | 133 | Tracebacks and the logging handler | 3.12 |  | 0.536 |
| walker |  | 6285 | 57 | Code::CodeKey { rung: Decl, file: rich/table.py, decl: 19, sub: 0, line: 422 } |  |  | 0.536 |
| walker |  | 6295 | 10 | Code::CodeKey { rung: Doc, file: rich/table.py, decl: 16, sub: 0, line: 353 } |  |  | 0.536 |
| walker |  | 6305 | 10 | Code::CodeKey { rung: Doc, file: rich/table.py, decl: 17, sub: 0, line: 358 } |  |  | 0.536 |
| walker |  | 6317 | 12 | Code::CodeKey { rung: Doc, file: rich/table.py, decl: 14, sub: 0, line: 310 } |  |  | 0.536 |
| walker |  | 6330 | 13 | Code::CodeKey { rung: Doc, file: rich/table.py, decl: 13, sub: 0, line: 305 } |  |  | 0.536 |
| walker |  | 6417 | 87 | Code::CodeKey { rung: Decl, file: rich/table.py, decl: 9, sub: 0, line: 252 } |  |  | 0.536 |
| walker |  | 6433 | 16 | Code::CodeKey { rung: Doc, file: rich/table.py, decl: 10, sub: 0, line: 285 } |  |  | 0.536 |
| walker |  | 6461 | 28 | Code::CodeKey { rung: Decl, file: rich/table.py, decl: 22, sub: 0, line: 523 } |  |  | 0.536 |
| ns | 6483 |  | 242 | Pretty printing, the repr protocol, inspect, Jupyter | 3.13 |  | 0.525 |
| walker |  | 6493 | 32 | Code::CodeKey { rung: Decl, file: rich/table.py, decl: 24, sub: 0, line: 627 } |  |  | 0.525 |
| walker |  | 6512 | 19 | Code::CodeKey { rung: Doc, file: rich/table.py, decl: 20, sub: 0, line: 469 } |  |  | 0.525 |
| walker |  | 6546 | 34 | Code::CodeKey { rung: Decl, file: rich/table.py, decl: 27, sub: 0, line: 755 } |  |  | 0.525 |
| walker |  | 6588 | 42 | Code::CodeKey { rung: Decl, file: rich/table.py, decl: 23, sub: 0, line: 588 } |  |  | 0.525 |
| walker |  | 6634 | 46 | Code::CodeKey { rung: Decl, file: rich/table.py, decl: 26, sub: 0, line: 716 } |  |  | 0.525 |
| ns | 6679 |  | 196 | Tree, JSON, emoji, highlighters, prompts | 3.14 |  | 0.517 |
| ns | 6765 |  | 86 | Layout engine | 3.15 |  | 0.513 |
| walker |  | 6856 | 222 | Code::CodeKey { rung: Decl, file: rich/table.py, decl: 18, sub: 0, line: 364 } |  |  | 0.513 |
| walker |  | 6868 | 12 | Code::CodeKey { rung: Doc, file: rich/table.py, decl: 25, sub: 0, line: 700 } |  |  | 0.513 |
| walker |  | 6883 | 15 | Code::CodeKey { rung: Doc, file: rich/table.py, decl: 12, sub: 0, line: 295 } |  |  | 0.513 |
| walker |  | 6899 | 16 | Code::CodeKey { rung: Doc, file: rich/table.py, decl: 24, sub: 0, line: 627 } |  |  | 0.513 |
| walker |  | 6915 | 16 | Code::CodeKey { rung: Doc, file: rich/table.py, decl: 26, sub: 0, line: 716 } |  |  | 0.513 |
| ns | 6939 |  | 174 | Width measurement, cell arithmetic, wrapping, ratios | 3.16 |  | 0.506 |
| ns | 7034 |  | 95 | Complete exception hierarchy | 3.17 |  | 0.502 |
| ns | 7112 |  | 78 | Control codes | 3.18 |  | 0.499 |
| walker |  | 7310 | 395 | Code::CodeKey { rung: Decl, file: rich/table.py, decl: 8, sub: 0, line: 188 } |  |  | 0.499 |
| ns | 7375 |  | 263 | Protocol helpers, file plumbing and small utilities | 3.19 |  | 0.491 |
| walker |  | 7405 | 95 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.491 |
| walker |  | 7425 | 20 | Code::CodeKey { rung: Doc, file: rich/table.py, decl: 22, sub: 0, line: 523 } |  |  | 0.491 |
| walker |  | 7482 | 57 | Code::CodeKey { rung: Names, file: rich/segment.py, decl: 0, sub: 0, line: 0 } |  |  | 0.496 |
| walker |  | 7522 | 40 | Code::CodeKey { rung: Decl, file: rich/segment.py, decl: 31, sub: 0, line: 724 } |  |  | 0.496 |
| walker |  | 7551 | 29 | Code::CodeKey { rung: Decl, file: rich/segment.py, decl: 33, sub: 0, line: 736 } |  |  | 0.496 |
| walker |  | 7592 | 41 | Code::CodeKey { rung: Decl, file: rich/segment.py, decl: 28, sub: 0, line: 699 } |  |  | 0.496 |
| ns | 7612 |  | 237 | Legacy Windows console support | 3.20 |  | 0.488 |
| walker |  | 7621 | 29 | Code::CodeKey { rung: Decl, file: rich/segment.py, decl: 30, sub: 0, line: 712 } |  |  | 0.488 |
| walker |  | 7665 | 44 | Code::CodeKey { rung: Decl, file: rich/segment.py, decl: 2, sub: 0, line: 53 } |  |  | 0.488 |
| ns | 7766 |  | 154 | Makefile — the complete set of dev commands | 4.1 |  | 0.499 |
| walker |  | 7859 | 194 | Code::CodeKey { rung: Decl, file: rich/segment.py, decl: 1, sub: 0, line: 32 } |  |  | 0.499 |
| walker |  | 7875 | 16 | Code::CodeKey { rung: Doc, file: rich/segment.py, decl: 1, sub: 0, line: 32 } |  |  | 0.499 |
| walker |  | 7944 | 69 | Code::CodeKey { rung: Doc, file: rich/screen.py, decl: 1, sub: 0, line: 18 } |  |  | 0.499 |
| walker |  | 8000 | 56 | Code::CodeKey { rung: Body, file: rich/__init__.py, decl: 3, sub: 0, line: 39 } |  |  | 0.499 |
| walker |  | 8024 | 24 | Code::CodeKey { rung: Names, file: rich/_null_file.py, decl: 0, sub: 0, line: 0 } |  |  | 0.500 |
| ns | 8144 |  | 378 | tests/ directory listing (complete) | 4.2 |  | 0.475 |
| ns | 8310 |  | 166 | examples/ directory listing (complete) | 4.3 |  | 0.500 |
| walker |  | 8358 | 334 | Code::CodeKey { rung: Decl, file: rich/_null_file.py, decl: 1, sub: 0, line: 5 } |  |  | 0.500 |
| walker |  | 8363 | 5 | Code::CodeKey { rung: Body, file: rich/_null_file.py, decl: 2, sub: 0, line: 6 } |  |  | 0.500 |
| walker |  | 8368 | 5 | Code::CodeKey { rung: Body, file: rich/_null_file.py, decl: 4, sub: 0, line: 12 } |  |  | 0.500 |
| walker |  | 8373 | 5 | Code::CodeKey { rung: Body, file: rich/_null_file.py, decl: 6, sub: 0, line: 18 } |  |  | 0.500 |
| walker |  | 8378 | 5 | Code::CodeKey { rung: Body, file: rich/_null_file.py, decl: 7, sub: 0, line: 21 } |  |  | 0.500 |
| walker |  | 8437 | 59 | Code::CodeKey { rung: Decl, file: rich/_null_file.py, decl: 17, sub: 0, line: 51 } |  |  | 0.500 |
| walker |  | 8442 | 5 | Code::CodeKey { rung: Body, file: rich/_null_file.py, decl: 13, sub: 0, line: 39 } |  |  | 0.500 |
| walker |  | 8447 | 5 | Code::CodeKey { rung: Body, file: rich/_null_file.py, decl: 14, sub: 0, line: 42 } |  |  | 0.500 |
| walker |  | 8452 | 5 | Code::CodeKey { rung: Body, file: rich/_null_file.py, decl: 17, sub: 0, line: 51 } |  |  | 0.500 |
| walker |  | 8457 | 5 | Code::CodeKey { rung: Body, file: rich/_null_file.py, decl: 19, sub: 0, line: 62 } |  |  | 0.500 |
| walker |  | 8463 | 6 | Code::CodeKey { rung: Body, file: rich/_null_file.py, decl: 3, sub: 0, line: 9 } |  |  | 0.500 |
| walker |  | 8469 | 6 | Code::CodeKey { rung: Body, file: rich/_null_file.py, decl: 5, sub: 0, line: 15 } |  |  | 0.500 |
| walker |  | 8475 | 6 | Code::CodeKey { rung: Body, file: rich/_null_file.py, decl: 9, sub: 0, line: 27 } |  |  | 0.500 |
| walker |  | 8487 | 12 | Code::CodeKey { rung: Names, file: rich/scope.py, decl: 0, sub: 0, line: 0 } |  |  | 0.500 |
| ns | 8550 |  | 240 | Type-check and test configuration, and the global test fixture | 4.4 |  | 0.499 |
| walker |  | 8613 | 126 | Code::CodeKey { rung: Decl, file: rich/scope.py, decl: 1, sub: 0, line: 14 } |  |  | 0.499 |
| ns | 8685 |  | 135 | docs/source narrative pages (complete listing) | 4.5 |  | 0.515 |
| walker |  | 8784 | 171 | Code::CodeKey { rung: Doc, file: rich/__init__.py, decl: 4, sub: 0, line: 53 } |  |  | 0.515 |
| walker |  | 8808 | 24 | Code::CodeKey { rung: Body, file: rich/spinner.py, decl: 4, sub: 0, line: 55 } |  |  | 0.515 |
| walker |  | 8881 | 73 | Code::CodeKey { rung: Doc, file: rich/spinner.py, decl: 5, sub: 0, line: 61 } |  |  | 0.515 |
| ns | 8896 |  | 211 | docs/source/reference and appendix listings (complete) | 4.6 |  | 0.536 |
| walker |  | 8993 | 112 | Code::CodeKey { rung: Names, file: rich/errors.py, decl: 0, sub: 0, line: 0 } |  |  | 0.544 |
| walker |  | 9001 | 8 | Code::CodeKey { rung: Doc, file: rich/errors.py, decl: 4, sub: 0, line: 13 } |  |  | 0.544 |
| walker |  | 9010 | 9 | Code::CodeKey { rung: Doc, file: rich/errors.py, decl: 2, sub: 0, line: 5 } |  |  | 0.544 |
| ns | 9012 |  | 116 | CONTRIBUTING.md section headings (complete) | 4.7 |  | 0.540 |
| walker |  | 9019 | 9 | Code::CodeKey { rung: Doc, file: rich/errors.py, decl: 3, sub: 0, line: 9 } |  |  | 0.540 |
| walker |  | 9028 | 9 | Code::CodeKey { rung: Doc, file: rich/errors.py, decl: 5, sub: 0, line: 17 } |  |  | 0.540 |
| walker |  | 9037 | 9 | Code::CodeKey { rung: Doc, file: rich/errors.py, decl: 7, sub: 0, line: 25 } |  |  | 0.540 |
| walker |  | 9047 | 10 | Code::CodeKey { rung: Doc, file: rich/errors.py, decl: 1, sub: 0, line: 1 } |  |  | 0.540 |
| walker |  | 9057 | 10 | Code::CodeKey { rung: Doc, file: rich/errors.py, decl: 6, sub: 0, line: 21 } |  |  | 0.540 |
| walker |  | 9067 | 10 | Code::CodeKey { rung: Doc, file: rich/errors.py, decl: 8, sub: 0, line: 29 } |  |  | 0.540 |
| walker |  | 9077 | 10 | Code::CodeKey { rung: Doc, file: rich/errors.py, decl: 9, sub: 0, line: 33 } |  |  | 0.540 |
| walker |  | 9102 | 25 | Code::CodeKey { rung: Names, file: rich/measure.py, decl: 0, sub: 0, line: 0 } |  |  | 0.541 |
| walker |  | 9145 | 43 | Code::CodeKey { rung: Decl, file: rich/measure.py, decl: 8, sub: 0, line: 125 } |  |  | 0.541 |
| ns | 9183 |  | 171 | tox.ini — supported interpreters and the lint/docs environments | 4.8 |  | 0.535 |
| walker |  | 9284 | 139 | Code::CodeKey { rung: Decl, file: rich/measure.py, decl: 1, sub: 0, line: 11 } |  |  | 0.535 |
| walker |  | 9290 | 6 | Code::CodeKey { rung: Decl, file: rich/measure.py, decl: 2, sub: 0, line: 19 } |  |  | 0.535 |
| walker |  | 9325 | 35 | Code::CodeKey { rung: Decl, file: rich/measure.py, decl: 6, sub: 0, line: 59 } |  |  | 0.535 |
| walker |  | 9368 | 43 | Code::CodeKey { rung: Decl, file: rich/measure.py, decl: 7, sub: 0, line: 78 } |  |  | 0.535 |
| walker |  | 9381 | 13 | Code::CodeKey { rung: Doc, file: rich/measure.py, decl: 2, sub: 0, line: 19 } |  |  | 0.535 |
| walker |  | 9401 | 20 | Code::CodeKey { rung: Doc, file: rich/measure.py, decl: 1, sub: 0, line: 11 } |  |  | 0.535 |
| walker |  | 9411 | 10 | Code::CodeKey { rung: Body, file: rich/measure.py, decl: 2, sub: 0, line: 19 } |  |  | 0.535 |
| walker |  | 9460 | 49 | Code::CodeKey { rung: Doc, file: rich/measure.py, decl: 3, sub: 0, line: 24 } |  |  | 0.535 |
| ns | 9473 |  | 290 | CHANGELOG — format convention and the most recent releases | 4.9 |  | 0.529 |
| walker |  | 9532 | 72 | Code::CodeKey { rung: Doc, file: rich/measure.py, decl: 4, sub: 0, line: 34 } |  |  | 0.529 |
| walker |  | 9604 | 72 | Code::CodeKey { rung: Doc, file: rich/measure.py, decl: 5, sub: 0, line: 46 } |  |  | 0.529 |
| walker |  | 9629 | 25 | Code::CodeKey { rung: Names, file: rich/pager.py, decl: 0, sub: 0, line: 0 } |  |  | 0.530 |
| ns | 9634 |  | 161 | FAQ.md — every question heading | 4.10 |  | 0.527 |
| walker |  | 9647 | 18 | Code::CodeKey { rung: Decl, file: rich/pager.py, decl: 1, sub: 0, line: 5 } |  |  | 0.527 |
| walker |  | 9656 | 9 | Code::CodeKey { rung: Decl, file: rich/pager.py, decl: 2, sub: 0, line: 8 } |  |  | 0.527 |
| walker |  | 9702 | 46 | Code::CodeKey { rung: Decl, file: rich/pager.py, decl: 3, sub: 0, line: 17 } |  |  | 0.527 |
| walker |  | 9712 | 10 | Code::CodeKey { rung: Doc, file: rich/pager.py, decl: 1, sub: 0, line: 5 } |  |  | 0.527 |
| walker |  | 9724 | 12 | Code::CodeKey { rung: Doc, file: rich/pager.py, decl: 3, sub: 0, line: 17 } |  |  | 0.527 |
| walker |  | 9740 | 16 | Code::CodeKey { rung: Doc, file: rich/pager.py, decl: 5, sub: 0, line: 23 } |  |  | 0.527 |
| walker |  | 9750 | 10 | Code::CodeKey { rung: Body, file: rich/pager.py, decl: 5, sub: 0, line: 23 } |  |  | 0.527 |
| ns | 9781 |  | 147 | benchmarks/, tools/, questions/ and .faq/ listings | 4.11 |  | 0.538 |
| walker |  | 9791 | 41 | Code::CodeKey { rung: Doc, file: rich/pager.py, decl: 2, sub: 0, line: 8 } |  |  | 0.538 |
| ns | 9981 |  | 200 | rich/_unicode_data listing (complete) | 4.12 |  | 0.547 |
