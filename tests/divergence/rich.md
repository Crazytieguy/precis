Score(3000)=0.691 I=0.762 C=0.626 ns_rows≤3K=15/51 grid(1000/1442/2080/3000/4327/6240/9000)=0.556/0.458/0.643/0.691/0.608/0.554/0.590

| source | ns_cum | walker_cum | marginal | descriptor | id | predecessor | Score(B=cum) |
|:-------|-------:|-----------:|---------:|:-----------|:---|:------------|-------------:|
| ns | 85 |  | 85 | README lede — what Rich is | 1.1 |  | 0.000 |
| ns | 129 |  | 44 | rich/__init__.py docstring + __all__ | 1.2 |  | 0.000 |
| walker |  | 191 | 191 | Fs::DirListing { dir: . } |  |  | 0.000 |
| walker |  | 203 | 12 | Fs::DirListing { dir: assets } |  |  | 0.000 |
| ns | 263 |  | 134 | pyproject identity block | 1.3 |  | 0.000 |
| walker |  | 350 | 147 | Markdown::ReadmeHeadline { file: README.md } |  |  | 0.310 |
| ns | 363 |  | 100 | README section headings (all H1/H2 locations) | 1.4 |  | 0.227 |
| walker |  | 369 | 19 | Fs::DirListing { dir: docs } |  |  | 0.227 |
| walker |  | 378 | 9 | Fs::DirListing { dir: docs/images } |  |  | 0.227 |
| ns | 514 |  | 151 | Runtime dependencies, extras and build backend | 1.5 |  | 0.179 |
| walker |  | 532 | 154 | Plaintext::Whole { file: Makefile } |  |  | 0.180 |
| walker |  | 615 | 83 | Toml::Identity { file: pyproject.toml } |  |  | 0.313 |
| walker |  | 623 | 8 | Fs::DirListing { dir: .faq } |  |  | 0.313 |
| ns | 661 |  | 147 | README renderable gallery — all `<summary>` labels | 1.6 |  | 0.265 |
| walker |  | 696 | 73 | Fs::DirListing { dir: questions } |  |  | 0.265 |
| ns | 758 |  | 97 | Compatibility + install + `python -m rich` | 1.7 |  | 0.259 |
| walker |  | 777 | 81 | Markdown::Prelude { file: README.md } |  |  | 0.259 |
| walker |  | 867 | 90 | Markdown::CommandBlock { file: CONTRIBUTING.md, row: 56 } |  |  | 0.259 |
| walker |  | 890 | 23 | Fs::DirListing { dir: .github } |  |  | 0.259 |
| walker |  | 920 | 30 | Fs::DirListing { dir: .github/workflows } |  |  | 0.259 |
| walker |  | 943 | 23 | Fs::DirListing { dir: benchmarks } |  |  | 0.259 |
| ns | 949 |  | 191 | Repository root listing (complete) | 1.8 |  | 0.527 |
| walker |  | 953 | 10 | Fs::DirListing { dir: .github/ISSUE_TEMPLATE } |  |  | 0.527 |
| walker |  | 1044 | 91 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.625 |
| walker |  | 1066 | 22 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.634 |
| walker |  | 1147 | 81 | Toml::Dependencies { file: pyproject.toml } |  |  | 0.656 |
| walker |  | 1190 | 43 | Fs::DirListing { dir: tools } |  |  | 0.657 |
| ns | 1312 |  | 363 | rich/ package module roster (complete) | 1.9 |  | 0.468 |
| walker |  | 1325 | 135 | Fs::DirListing { dir: docs/source } |  |  | 0.471 |
| walker |  | 1335 | 10 | Fs::DirListing { dir: docs/source/appendix } |  |  | 0.471 |
| walker |  | 1376 | 41 | Markdown::Section { file: README.md, section_index: 17, keeps_default_concavity: false } |  |  | 0.471 |
| ns | 1424 |  | 112 | rich.* top-level function signatures: get_console, reconfigure, print | 2.1 |  | 0.457 |
| walker |  | 1542 | 166 | Fs::DirListing { dir: examples } |  |  | 0.461 |
| walker |  | 1585 | 43 | Markdown::Section { file: README.md, section_index: 16, keeps_default_concavity: false } |  |  | 0.461 |
| ns | 1729 |  | 305 | rich.* top-level function signatures: print_json, inspect | 2.2 |  | 0.426 |
| walker |  | 1948 | 363 | Fs::DirListing { dir: rich } |  |  | 0.673 |
| walker |  | 1964 | 16 | Code::CodeKey { rung: ModuleDoc, file: rich/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.686 |
| ns | 2019 |  | 290 | The console protocol: RichCast, ConsoleRenderable, RenderableType, RenderResult | 2.3 |  | 0.642 |
| walker |  | 2164 | 200 | Fs::DirListing { dir: rich/_unicode_data } |  |  | 0.644 |
| walker |  | 2231 | 67 | Markdown::Section { file: README.md, section_index: 3, keeps_default_concavity: false } |  |  | 0.644 |
| walker |  | 2257 | 26 | Code::CodeKey { rung: Names, file: rich/__main__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.644 |
| ns | 2281 |  | 262 | console.py module-level symbol roster | 2.4 |  | 0.611 |
| walker |  | 2302 | 45 | Code::CodeKey { rung: Decl, file: rich/__main__.py, decl: 1, sub: 0, line: 18 } |  |  | 0.611 |
| walker |  | 2316 | 14 | Code::CodeKey { rung: Decl, file: rich/__main__.py, decl: 2, sub: 0, line: 19 } |  |  | 0.611 |
| walker |  | 2331 | 15 | Code::CodeKey { rung: Decl, file: rich/__main__.py, decl: 3, sub: 0, line: 33 } |  |  | 0.611 |
| walker |  | 2343 | 12 | Code::CodeKey { rung: Body, file: rich/__main__.py, decl: 3, sub: 0, line: 33 } |  |  | 0.611 |
| walker |  | 2439 | 96 | Markdown::Section { file: README.md, section_index: 1, keeps_default_concavity: false } |  |  | 0.626 |
| ns | 2525 |  | 244 | Console method roster — rendering and output (lines 1092–1652) | 2.5 |  | 0.597 |
| walker |  | 2602 | 163 | Code::CodeKey { rung: Names, file: rich/__init__.py, decl: 0, sub: 0, line: 0 } |  |  | 0.636 |
| walker |  | 2659 | 57 | Code::CodeKey { rung: Decl, file: rich/__init__.py, decl: 4, sub: 0, line: 53 } |  |  | 0.653 |
| ns | 2735 |  | 210 | Console method roster — JSON, screen updates, exceptions, logging, export (1758–2606) | 2.6 |  | 0.632 |
| walker |  | 2788 | 129 | Code::CodeKey { rung: Decl, file: rich/__init__.py, decl: 6, sub: 0, line: 120 } |  |  | 0.649 |
| walker |  | 2925 | 137 | Code::CodeKey { rung: Decl, file: rich/__init__.py, decl: 5, sub: 0, line: 77 } |  |  | 0.691 |
| walker |  | 2987 | 62 | Markdown::Section { file: README.md, section_index: 15, keeps_default_concavity: false } |  |  | 0.691 |
| walker |  | 3004 | 17 | Code::CodeKey { rung: Doc, file: rich/__main__.py, decl: 4, sub: 0, line: 39 } |  |  | 0.691 |
| ns | 3126 |  | 391 | Console method roster — construction, context management, properties (617–1084) | 2.7 |  | 0.649 |
| walker |  | 3200 | 196 | Code::CodeKey { rung: Names, file: rich/console.py, decl: 0, sub: 0, line: 0 } |  |  | 0.652 |
| walker |  | 3205 | 5 | Code::CodeKey { rung: Decl, file: rich/console.py, decl: 7, sub: 0, line: 73 } |  |  | 0.652 |
| walker |  | 3387 | 182 | Fs::DirListing { dir: docs/source/reference } |  |  | 0.656 |
| walker |  | 3505 | 118 | Markdown::Section { file: README.md, section_index: 2, keeps_default_concavity: false } |  |  | 0.656 |
| ns | 3573 |  | 447 | Console.__init__ full keyword surface | 2.8 | 2.7 | 0.619 |
| walker |  | 3748 | 243 | Code::CodeKey { rung: Names, file: rich/console.py, decl: 0, sub: 1, line: 0 } |  |  | 0.642 |
| ns | 3780 |  | 207 | ConsoleOptions dataclass — the per-render context | 2.9 | 2.4 | 0.625 |
| walker |  | 3789 | 41 | Code::CodeKey { rung: Decl, file: rich/console.py, decl: 23, sub: 0, line: 256 } |  |  | 0.628 |
| walker |  | 3805 | 16 | Code::CodeKey { rung: Decl, file: rich/console.py, decl: 24, sub: 0, line: 260 } |  |  | 0.630 |
| walker |  | 3851 | 46 | Code::CodeKey { rung: Decl, file: rich/console.py, decl: 11, sub: 0, line: 96 } |  |  | 0.630 |
| walker |  | 3898 | 47 | Code::CodeKey { rung: Decl, file: rich/console.py, decl: 21, sub: 0, line: 246 } |  |  | 0.635 |
| walker |  | 3903 | 5 | Code::CodeKey { rung: Decl, file: rich/console.py, decl: 22, sub: 0, line: 250 } |  |  | 0.637 |
| walker |  | 3951 | 48 | Code::CodeKey { rung: Decl, file: rich/console.py, decl: 29, sub: 0, line: 280 } |  |  | 0.637 |
| walker |  | 3967 | 16 | Code::CodeKey { rung: Decl, file: rich/console.py, decl: 31, sub: 0, line: 286 } |  |  | 0.637 |
| ns | 4016 |  | 236 | Console.print full keyword surface | 2.10 | 2.5 | 0.619 |
| walker |  | 4017 | 50 | Code::CodeKey { rung: Decl, file: rich/console.py, decl: 12, sub: 0, line: 103 } |  |  | 0.619 |
| walker |  | 4072 | 55 | Code::CodeKey { rung: Decl, file: rich/console.py, decl: 32, sub: 0, line: 292 } |  |  | 0.619 |
| walker |  | 4087 | 15 | Code::CodeKey { rung: Decl, file: rich/console.py, decl: 34, sub: 0, line: 300 } |  |  | 0.619 |
| walker |  | 4149 | 62 | Code::CodeKey { rung: Decl, file: rich/console.py, decl: 44, sub: 0, line: 364 } |  |  | 0.619 |
| ns | 4154 |  | 138 | Table, Panel, Columns, Align, Padding, Rule, Constrain, Styled, Box classes | 3.1 |  | 0.608 |
| walker |  | 4196 | 47 | Code::CodeKey { rung: Decl, file: rich/console.py, decl: 47, sub: 0, line: 383 } |  |  | 0.608 |
| walker |  | 4247 | 51 | Code::CodeKey { rung: Decl, file: rich/console.py, decl: 45, sub: 0, line: 367 } |  |  | 0.608 |
| walker |  | 4318 | 71 | Code::CodeKey { rung: Decl, file: rich/console.py, decl: 40, sub: 0, line: 343 } |  |  | 0.608 |
| walker |  | 4365 | 47 | Code::CodeKey { rung: Decl, file: rich/console.py, decl: 43, sub: 0, line: 355 } |  |  | 0.608 |
| walker |  | 4439 | 74 | Code::CodeKey { rung: Decl, file: rich/console.py, decl: 35, sub: 0, line: 310 } |  |  | 0.608 |
| ns | 4449 |  | 295 | Every predefined Box border style | 3.2 |  | 0.592 |
| walker |  | 4486 | 47 | Code::CodeKey { rung: Decl, file: rich/console.py, decl: 38, sub: 0, line: 326 } |  |  | 0.592 |
| walker |  | 4569 | 83 | Code::CodeKey { rung: Decl, file: rich/console.py, decl: 48, sub: 0, line: 403 } |  |  | 0.592 |
| ns | 4578 |  | 129 | Text, markup and container primitives | 3.3 |  | 0.584 |
| walker |  | 4590 | 21 | Code::CodeKey { rung: Decl, file: rich/console.py, decl: 49, sub: 0, line: 406 } |  |  | 0.584 |
| walker |  | 4613 | 23 | Code::CodeKey { rung: Decl, file: rich/console.py, decl: 50, sub: 0, line: 414 } |  |  | 0.584 |
| ns | 4618 |  | 40 | Style module symbols | 3.4 |  | 0.581 |
| walker |  | 4660 | 47 | Code::CodeKey { rung: Decl, file: rich/console.py, decl: 52, sub: 0, line: 438 } |  |  | 0.581 |
| walker |  | 4756 | 96 | Code::CodeKey { rung: Decl, file: rich/console.py, decl: 53, sub: 0, line: 450 } |  |  | 0.581 |
| walker |  | 4764 | 8 | Code::CodeKey { rung: Decl, file: rich/console.py, decl: 55, sub: 0, line: 463 } |  |  | 0.581 |
| walker |  | 4780 | 16 | Code::CodeKey { rung: Decl, file: rich/console.py, decl: 56, sub: 0, line: 469 } |  |  | 0.581 |
| walker |  | 4796 | 16 | Code::CodeKey { rung: Decl, file: rich/console.py, decl: 57, sub: 0, line: 477 } |  |  | 0.581 |
| ns | 4986 |  | 368 | Style method roster | 3.5 | 3.4 | 0.558 |
| ns | 5233 |  | 247 | Colour system, palettes and themes | 3.6 |  | 0.546 |
| walker |  | 5250 | 454 | Code::CodeKey { rung: Decl, file: rich/console.py, decl: 13, sub: 0, line: 112 } |  |  | 0.564 |
| walker |  | 5256 | 6 | Code::CodeKey { rung: Decl, file: rich/console.py, decl: 14, sub: 0, line: 142 } |  |  | 0.564 |
| ns | 5276 |  | 43 | Segment — the atomic unit of rendered output | 3.7 |  | 0.562 |
| walker |  | 5443 | 187 | Code::CodeKey { rung: Decl, file: rich/console.py, decl: 16, sub: 0, line: 157 } |  |  | 0.562 |
| walker |  | 5452 | 9 | Code::CodeKey { rung: Doc, file: rich/console.py, decl: 12, sub: 0, line: 103 } |  |  | 0.562 |
| walker |  | 5464 | 12 | Code::CodeKey { rung: Doc, file: rich/console.py, decl: 13, sub: 0, line: 112 } |  |  | 0.565 |
| walker |  | 5476 | 12 | Code::CodeKey { rung: Doc, file: rich/console.py, decl: 23, sub: 0, line: 256 } |  |  | 0.568 |
| walker |  | 5488 | 12 | Code::CodeKey { rung: Doc, file: rich/console.py, decl: 28, sub: 0, line: 276 } |  |  | 0.568 |
| walker |  | 5501 | 13 | Code::CodeKey { rung: Doc, file: rich/console.py, decl: 16, sub: 0, line: 157 } |  |  | 0.568 |
| ns | 5513 |  | 237 | Progress bars — functions, columns and classes | 3.8 |  | 0.554 |
| walker |  | 5514 | 13 | Code::CodeKey { rung: Doc, file: rich/console.py, decl: 39, sub: 0, line: 334 } |  |  | 0.554 |
| walker |  | 5528 | 14 | Code::CodeKey { rung: Doc, file: rich/console.py, decl: 29, sub: 0, line: 280 } |  |  | 0.554 |
| walker |  | 5542 | 14 | Code::CodeKey { rung: Doc, file: rich/console.py, decl: 32, sub: 0, line: 292 } |  |  | 0.554 |
| walker |  | 5557 | 15 | Code::CodeKey { rung: Doc, file: rich/console.py, decl: 14, sub: 0, line: 142 } |  |  | 0.554 |
| walker |  | 5573 | 16 | Code::CodeKey { rung: Doc, file: rich/console.py, decl: 59, sub: 0, line: 505 } |  |  | 0.554 |
| walker |  | 5714 | 141 | Code::CodeKey { rung: Names, file: rich/console.py, decl: 0, sub: 2, line: 0 } |  |  | 0.568 |
| ns | 5723 |  | 210 | Live display, status, spinners, bars, screen | 3.9 |  | 0.560 |
| walker |  | 5740 | 26 | Code::CodeKey { rung: Decl, file: rich/console.py, decl: 63, sub: 0, line: 544 } |  |  | 0.560 |
| walker |  | 5762 | 22 | Code::CodeKey { rung: Decl, file: rich/console.py, decl: 64, sub: 0, line: 547 } |  |  | 0.560 |
| walker |  | 5809 | 47 | Code::CodeKey { rung: Decl, file: rich/console.py, decl: 62, sub: 0, line: 535 } |  |  | 0.560 |
| walker |  | 5867 | 58 | Code::CodeKey { rung: Decl, file: rich/console.py, decl: 60, sub: 0, line: 525 } |  |  | 0.560 |
| ns | 5906 |  | 183 | Syntax highlighting | 3.10 |  | 0.553 |
| walker |  | 6066 | 199 | Code::CodeKey { rung: Decl, file: rich/console.py, decl: 68, sub: 0, line: 581 } |  |  | 0.558 |
| walker |  | 6074 | 8 | Code::CodeKey { rung: Decl, file: rich/console.py, decl: 71, sub: 0, line: 756 } |  |  | 0.558 |
| walker |  | 6082 | 8 | Code::CodeKey { rung: Decl, file: rich/console.py, decl: 73, sub: 0, line: 770 } |  |  | 0.558 |
| walker |  | 6090 | 8 | Code::CodeKey { rung: Decl, file: rich/console.py, decl: 74, sub: 0, line: 775 } |  |  | 0.558 |
| walker |  | 6098 | 8 | Code::CodeKey { rung: Decl, file: rich/console.py, decl: 76, sub: 0, line: 784 } |  |  | 0.558 |
| walker |  | 6107 | 9 | Code::CodeKey { rung: Decl, file: rich/console.py, decl: 72, sub: 0, line: 765 } |  |  | 0.558 |
| ns | 6108 |  | 202 | Markdown element hierarchy | 3.11 |  | 0.547 |
| walker |  | 6117 | 10 | Code::CodeKey { rung: Decl, file: rich/console.py, decl: 75, sub: 0, line: 780 } |  |  | 0.547 |
| ns | 6241 |  | 133 | Tracebacks and the logging handler | 3.12 |  | 0.541 |
| walker |  | 6308 | 191 | Code::CodeKey { rung: Decl, file: rich/console.py, decl: 68, sub: 1, line: 581 } |  |  | 0.554 |
| ns | 6483 |  | 242 | Pretty printing, the repr protocol, inspect, Jupyter | 3.13 |  | 0.543 |
| walker |  | 6500 | 192 | Code::CodeKey { rung: Decl, file: rich/console.py, decl: 68, sub: 2, line: 581 } |  |  | 0.563 |
| walker |  | 6508 | 8 | Code::CodeKey { rung: Decl, file: rich/console.py, decl: 91, sub: 0, line: 908 } |  |  | 0.563 |
| walker |  | 6516 | 8 | Code::CodeKey { rung: Decl, file: rich/console.py, decl: 92, sub: 0, line: 921 } |  |  | 0.563 |
| walker |  | 6524 | 8 | Code::CodeKey { rung: Decl, file: rich/console.py, decl: 93, sub: 0, line: 930 } |  |  | 0.563 |
| walker |  | 6532 | 8 | Code::CodeKey { rung: Decl, file: rich/console.py, decl: 94, sub: 0, line: 978 } |  |  | 0.563 |
| walker |  | 6540 | 8 | Code::CodeKey { rung: Decl, file: rich/console.py, decl: 95, sub: 0, line: 990 } |  |  | 0.563 |
| walker |  | 6549 | 9 | Code::CodeKey { rung: Decl, file: rich/console.py, decl: 96, sub: 0, line: 1004 } |  |  | 0.563 |
| walker |  | 6558 | 9 | Code::CodeKey { rung: Decl, file: rich/console.py, decl: 98, sub: 0, line: 1056 } |  |  | 0.563 |
| walker |  | 6568 | 10 | Code::CodeKey { rung: Decl, file: rich/console.py, decl: 97, sub: 0, line: 1045 } |  |  | 0.563 |
| walker |  | 6578 | 10 | Code::CodeKey { rung: Decl, file: rich/console.py, decl: 99, sub: 0, line: 1065 } |  |  | 0.563 |
| ns | 6679 |  | 196 | Tree, JSON, emoji, highlighters, prompts | 3.14 |  | 0.554 |
| ns | 6765 |  | 86 | Layout engine | 3.15 |  | 0.550 |
| walker |  | 6795 | 217 | Code::CodeKey { rung: Decl, file: rich/console.py, decl: 68, sub: 3, line: 581 } |  |  | 0.559 |
| walker |  | 6804 | 9 | Code::CodeKey { rung: Decl, file: rich/console.py, decl: 100, sub: 0, line: 1074 } |  |  | 0.559 |
| walker |  | 6813 | 9 | Code::CodeKey { rung: Decl, file: rich/console.py, decl: 110, sub: 0, line: 1222 } |  |  | 0.559 |
| walker |  | 6823 | 10 | Code::CodeKey { rung: Decl, file: rich/console.py, decl: 101, sub: 0, line: 1083 } |  |  | 0.559 |
| walker |  | 6850 | 27 | Code::CodeKey { rung: Decl, file: rich/console.py, decl: 104, sub: 0, line: 1113 } |  |  | 0.559 |
| walker |  | 6936 | 86 | Code::CodeKey { rung: Decl, file: rich/console.py, decl: 107, sub: 0, line: 1157 } |  |  | 0.559 |
| ns | 6939 |  | 174 | Width measurement, cell arithmetic, wrapping, ratios | 3.16 |  | 0.551 |
| walker |  | 6946 | 10 | Code::CodeKey { rung: Doc, file: rich/console.py, decl: 67, sub: 0, line: 576 } |  |  | 0.551 |
| walker |  | 6956 | 10 | Code::CodeKey { rung: Doc, file: rich/console.py, decl: 85, sub: 0, line: 862 } |  |  | 0.551 |
| ns | 7034 |  | 95 | Complete exception hierarchy | 3.17 |  | 0.547 |
| ns | 7112 |  | 78 | Control codes | 3.18 |  | 0.544 |
| walker |  | 7221 | 265 | Code::CodeKey { rung: Decl, file: rich/console.py, decl: 68, sub: 4, line: 581 } |  |  | 0.562 |
| walker |  | 7244 | 23 | Code::CodeKey { rung: Decl, file: rich/console.py, decl: 112, sub: 0, line: 1263 } |  |  | 0.562 |
| walker |  | 7267 | 23 | Code::CodeKey { rung: Decl, file: rich/console.py, decl: 114, sub: 0, line: 1294 } |  |  | 0.562 |
| walker |  | 7291 | 24 | Code::CodeKey { rung: Decl, file: rich/console.py, decl: 113, sub: 0, line: 1277 } |  |  | 0.562 |
| walker |  | 7318 | 27 | Code::CodeKey { rung: Decl, file: rich/console.py, decl: 117, sub: 0, line: 1470 } |  |  | 0.562 |
| ns | 7375 |  | 263 | Protocol helpers, file plumbing and small utilities | 3.19 |  | 0.552 |
| walker |  | 7388 | 70 | Code::CodeKey { rung: Decl, file: rich/console.py, decl: 119, sub: 0, line: 1589 } |  |  | 0.552 |
| walker |  | 7463 | 75 | Code::CodeKey { rung: Decl, file: rich/console.py, decl: 121, sub: 0, line: 1620 } |  |  | 0.552 |
| walker |  | 7545 | 82 | Code::CodeKey { rung: Decl, file: rich/console.py, decl: 115, sub: 0, line: 1345 } |  |  | 0.552 |
| ns | 7612 |  | 237 | Legacy Windows console support | 3.20 |  | 0.544 |
| walker |  | 7651 | 106 | Code::CodeKey { rung: Decl, file: rich/console.py, decl: 118, sub: 0, line: 1500 } |  |  | 0.544 |
| ns | 7766 |  | 154 | Makefile — the complete set of dev commands | 4.1 |  | 0.551 |
| walker |  | 7785 | 134 | Code::CodeKey { rung: Decl, file: rich/console.py, decl: 116, sub: 0, line: 1409 } |  |  | 0.551 |
| walker |  | 8037 | 252 | Code::CodeKey { rung: Decl, file: rich/console.py, decl: 68, sub: 5, line: 581 } |  |  | 0.560 |
| walker |  | 8064 | 27 | Code::CodeKey { rung: Decl, file: rich/console.py, decl: 125, sub: 0, line: 1853 } |  |  | 0.560 |
| walker |  | 8097 | 33 | Code::CodeKey { rung: Decl, file: rich/console.py, decl: 127, sub: 0, line: 1908 } |  |  | 0.560 |
| ns | 8144 |  | 378 | tests/ directory listing (complete) | 4.2 |  | 0.532 |
| walker |  | 8154 | 57 | Code::CodeKey { rung: Decl, file: rich/console.py, decl: 124, sub: 0, line: 1819 } |  |  | 0.532 |
| walker |  | 8269 | 115 | Code::CodeKey { rung: Decl, file: rich/console.py, decl: 126, sub: 0, line: 1873 } |  |  | 0.532 |
| ns | 8310 |  | 166 | examples/ directory listing (complete) | 4.3 |  | 0.548 |
| walker |  | 8420 | 151 | Code::CodeKey { rung: Decl, file: rich/console.py, decl: 128, sub: 0, line: 1947 } |  |  | 0.548 |
| ns | 8550 |  | 240 | Type-check and test configuration, and the global test fixture | 4.4 |  | 0.540 |
| walker |  | 8576 | 156 | Code::CodeKey { rung: Decl, file: rich/console.py, decl: 123, sub: 0, line: 1758 } |  |  | 0.540 |
| ns | 8685 |  | 135 | docs/source narrative pages (complete listing) | 4.5 |  | 0.551 |
| walker |  | 8800 | 224 | Code::CodeKey { rung: Decl, file: rich/console.py, decl: 122, sub: 0, line: 1652 } |  |  | 0.567 |
| ns | 8896 |  | 211 | docs/source/reference and appendix listings (complete) | 4.6 |  | 0.581 |
| walker |  | 8972 | 172 | Code::CodeKey { rung: Decl, file: rich/console.py, decl: 68, sub: 6, line: 581 } |  |  | 0.590 |
| ns | 9012 |  | 116 | CONTRIBUTING.md section headings (complete) | 4.7 |  | 0.586 |
| walker |  | 9024 | 52 | Code::CodeKey { rung: Decl, file: rich/console.py, decl: 135, sub: 0, line: 2224 } |  |  | 0.586 |
| walker |  | 9093 | 69 | Code::CodeKey { rung: Decl, file: rich/console.py, decl: 136, sub: 0, line: 2244 } |  |  | 0.586 |
| walker |  | 9170 | 77 | Code::CodeKey { rung: Decl, file: rich/console.py, decl: 133, sub: 0, line: 2156 } |  |  | 0.586 |
| ns | 9183 |  | 171 | tox.ini — supported interpreters and the lint/docs environments | 4.8 |  | 0.580 |
| walker |  | 9255 | 85 | Code::CodeKey { rung: Decl, file: rich/console.py, decl: 137, sub: 0, line: 2321 } |  |  | 0.580 |
| walker |  | 9359 | 104 | Code::CodeKey { rung: Decl, file: rich/console.py, decl: 138, sub: 0, line: 2352 } |  |  | 0.580 |
| ns | 9473 |  | 290 | CHANGELOG — format convention and the most recent releases | 4.9 |  | 0.574 |
| walker |  | 9478 | 119 | Code::CodeKey { rung: Decl, file: rich/console.py, decl: 139, sub: 0, line: 2606 } |  |  | 0.574 |
| walker |  | 9489 | 11 | Code::CodeKey { rung: Doc, file: rich/console.py, decl: 62, sub: 0, line: 535 } |  |  | 0.574 |
| walker |  | 9500 | 11 | Code::CodeKey { rung: Doc, file: rich/console.py, decl: 95, sub: 0, line: 990 } |  |  | 0.574 |
| walker |  | 9512 | 12 | Code::CodeKey { rung: Doc, file: rich/console.py, decl: 63, sub: 0, line: 544 } |  |  | 0.574 |
| walker |  | 9524 | 12 | Code::CodeKey { rung: Doc, file: rich/console.py, decl: 72, sub: 0, line: 765 } |  |  | 0.574 |
| walker |  | 9536 | 12 | Code::CodeKey { rung: Doc, file: rich/console.py, decl: 73, sub: 0, line: 770 } |  |  | 0.574 |
| walker |  | 9548 | 12 | Code::CodeKey { rung: Doc, file: rich/console.py, decl: 74, sub: 0, line: 775 } |  |  | 0.574 |
| walker |  | 9561 | 13 | Code::CodeKey { rung: Doc, file: rich/console.py, decl: 76, sub: 0, line: 784 } |  |  | 0.574 |
| walker |  | 9574 | 13 | Code::CodeKey { rung: Doc, file: rich/console.py, decl: 77, sub: 0, line: 789 } |  |  | 0.574 |
| walker |  | 9588 | 14 | Code::CodeKey { rung: Doc, file: rich/console.py, decl: 71, sub: 0, line: 756 } |  |  | 0.574 |
| walker |  | 9602 | 14 | Code::CodeKey { rung: Doc, file: rich/console.py, decl: 84, sub: 0, line: 857 } |  |  | 0.574 |
| walker |  | 9617 | 15 | Code::CodeKey { rung: Doc, file: rich/console.py, decl: 83, sub: 0, line: 852 } |  |  | 0.574 |
| walker |  | 9632 | 15 | Code::CodeKey { rung: Doc, file: rich/console.py, decl: 131, sub: 0, line: 2059 } |  |  | 0.574 |
| ns | 9634 |  | 161 | FAQ.md — every question heading | 4.10 |  | 0.570 |
| walker |  | 9647 | 15 | Code::CodeKey { rung: Doc, file: rich/console.py, decl: 132, sub: 0, line: 2132 } |  |  | 0.570 |
| walker |  | 9663 | 16 | Code::CodeKey { rung: Doc, file: rich/console.py, decl: 79, sub: 0, line: 817 } |  |  | 0.570 |
| walker |  | 9680 | 17 | Code::CodeKey { rung: Doc, file: rich/console.py, decl: 89, sub: 0, line: 892 } |  |  | 0.570 |
| ns | 9781 |  | 147 | benchmarks/, tools/, questions/ and .faq/ listings | 4.11 |  | 0.578 |
| walker |  | 9858 | 178 | Code::CodeKey { rung: Decl, file: rich/console.py, decl: 69, sub: 0, line: 619 } |  |  | 0.583 |
| ns | 9981 |  | 200 | rich/_unicode_data listing (complete) | 4.12 |  | 0.589 |
| walker |  | 9998 | 140 | Code::CodeKey { rung: Decl, file: rich/console.py, decl: 69, sub: 1, line: 619 } |  |  | 0.598 |
