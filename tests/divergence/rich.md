Score(3000)=0.691 I=0.762 C=0.626 ns_rows≤3K=15/51 grid(1000/1442/2080/3000/4327/6240/9000)=0.635/0.458/0.643/0.691/0.609/0.556/0.591

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
| walker |  | 800 | 23 | Fs::DirListing { dir: .github } |  |  | 0.259 |
| walker |  | 830 | 30 | Fs::DirListing { dir: .github/workflows } |  |  | 0.259 |
| walker |  | 853 | 23 | Fs::DirListing { dir: benchmarks } |  |  | 0.259 |
| walker |  | 863 | 10 | Fs::DirListing { dir: .github/ISSUE_TEMPLATE } |  |  | 0.259 |
| ns | 949 |  | 191 | Repository root listing (complete) | 1.8 |  | 0.527 |
| walker |  | 954 | 91 | Markdown::HeadingsOutline { file: README.md } |  |  | 0.625 |
| walker |  | 976 | 22 | Markdown::Section { file: README.md, section_index: 0, keeps_default_concavity: false } |  |  | 0.634 |
| walker |  | 1057 | 81 | Toml::Dependencies { file: pyproject.toml } |  |  | 0.656 |
| walker |  | 1147 | 90 | Markdown::CommandBlock { file: CONTRIBUTING.md, row: 56 } |  |  | 0.656 |
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
| walker |  | 3774 | 269 | Code::CodeKey { rung: Names, file: rich/console.py, decl: 0, sub: 1, line: 0 } |  |  | 0.644 |
| ns | 3780 |  | 207 | ConsoleOptions dataclass — the per-render context | 2.9 | 2.4 | 0.627 |
| walker |  | 3806 | 32 | Code::CodeKey { rung: Decl, file: rich/console.py, decl: 23, sub: 0, line: 256 } |  |  | 0.629 |
| walker |  | 3822 | 16 | Code::CodeKey { rung: Decl, file: rich/console.py, decl: 24, sub: 0, line: 260 } |  |  | 0.631 |
| walker |  | 3860 | 38 | Code::CodeKey { rung: Decl, file: rich/console.py, decl: 21, sub: 0, line: 246 } |  |  | 0.635 |
| walker |  | 3865 | 5 | Code::CodeKey { rung: Decl, file: rich/console.py, decl: 22, sub: 0, line: 250 } |  |  | 0.637 |
| walker |  | 3911 | 46 | Code::CodeKey { rung: Decl, file: rich/console.py, decl: 11, sub: 0, line: 96 } |  |  | 0.637 |
| walker |  | 3959 | 48 | Code::CodeKey { rung: Decl, file: rich/console.py, decl: 12, sub: 0, line: 103 } |  |  | 0.637 |
| walker |  | 4007 | 48 | Code::CodeKey { rung: Decl, file: rich/console.py, decl: 29, sub: 0, line: 280 } |  |  | 0.637 |
| ns | 4016 |  | 236 | Console.print full keyword surface | 2.10 | 2.5 | 0.620 |
| walker |  | 4023 | 16 | Code::CodeKey { rung: Decl, file: rich/console.py, decl: 31, sub: 0, line: 286 } |  |  | 0.620 |
| walker |  | 4078 | 55 | Code::CodeKey { rung: Decl, file: rich/console.py, decl: 32, sub: 0, line: 292 } |  |  | 0.620 |
| walker |  | 4093 | 15 | Code::CodeKey { rung: Decl, file: rich/console.py, decl: 34, sub: 0, line: 300 } |  |  | 0.620 |
| ns | 4154 |  | 138 | Table, Panel, Columns, Align, Padding, Rule, Constrain, Styled, Box classes | 3.1 |  | 0.609 |
| walker |  | 4155 | 62 | Code::CodeKey { rung: Decl, file: rich/console.py, decl: 44, sub: 0, line: 364 } |  |  | 0.609 |
| walker |  | 4202 | 47 | Code::CodeKey { rung: Decl, file: rich/console.py, decl: 47, sub: 0, line: 383 } |  |  | 0.609 |
| walker |  | 4253 | 51 | Code::CodeKey { rung: Decl, file: rich/console.py, decl: 45, sub: 0, line: 367 } |  |  | 0.609 |
| walker |  | 4324 | 71 | Code::CodeKey { rung: Decl, file: rich/console.py, decl: 40, sub: 0, line: 343 } |  |  | 0.609 |
| walker |  | 4371 | 47 | Code::CodeKey { rung: Decl, file: rich/console.py, decl: 43, sub: 0, line: 355 } |  |  | 0.609 |
| walker |  | 4445 | 74 | Code::CodeKey { rung: Decl, file: rich/console.py, decl: 35, sub: 0, line: 310 } |  |  | 0.609 |
| ns | 4449 |  | 295 | Every predefined Box border style | 3.2 |  | 0.592 |
| walker |  | 4492 | 47 | Code::CodeKey { rung: Decl, file: rich/console.py, decl: 38, sub: 0, line: 326 } |  |  | 0.592 |
| walker |  | 4575 | 83 | Code::CodeKey { rung: Decl, file: rich/console.py, decl: 48, sub: 0, line: 403 } |  |  | 0.592 |
| ns | 4578 |  | 129 | Text, markup and container primitives | 3.3 |  | 0.584 |
| walker |  | 4596 | 21 | Code::CodeKey { rung: Decl, file: rich/console.py, decl: 49, sub: 0, line: 406 } |  |  | 0.584 |
| ns | 4618 |  | 40 | Style module symbols | 3.4 |  | 0.581 |
| walker |  | 4619 | 23 | Code::CodeKey { rung: Decl, file: rich/console.py, decl: 50, sub: 0, line: 414 } |  |  | 0.581 |
| walker |  | 4666 | 47 | Code::CodeKey { rung: Decl, file: rich/console.py, decl: 52, sub: 0, line: 438 } |  |  | 0.581 |
| walker |  | 4770 | 104 | Code::CodeKey { rung: Decl, file: rich/console.py, decl: 53, sub: 0, line: 450 } |  |  | 0.581 |
| walker |  | 4786 | 16 | Code::CodeKey { rung: Decl, file: rich/console.py, decl: 56, sub: 0, line: 469 } |  |  | 0.581 |
| walker |  | 4802 | 16 | Code::CodeKey { rung: Decl, file: rich/console.py, decl: 57, sub: 0, line: 477 } |  |  | 0.581 |
| ns | 4986 |  | 368 | Style method roster | 3.5 | 3.4 | 0.558 |
| ns | 5233 |  | 247 | Colour system, palettes and themes | 3.6 |  | 0.546 |
| walker |  | 5261 | 459 | Code::CodeKey { rung: Decl, file: rich/console.py, decl: 13, sub: 0, line: 112 } |  |  | 0.566 |
| ns | 5276 |  | 43 | Segment — the atomic unit of rendered output | 3.7 |  | 0.563 |
| walker |  | 5448 | 187 | Code::CodeKey { rung: Decl, file: rich/console.py, decl: 16, sub: 0, line: 157 } |  |  | 0.563 |
| walker |  | 5457 | 9 | Code::CodeKey { rung: Doc, file: rich/console.py, decl: 12, sub: 0, line: 103 } |  |  | 0.563 |
| walker |  | 5469 | 12 | Code::CodeKey { rung: Doc, file: rich/console.py, decl: 13, sub: 0, line: 112 } |  |  | 0.567 |
| walker |  | 5481 | 12 | Code::CodeKey { rung: Doc, file: rich/console.py, decl: 23, sub: 0, line: 256 } |  |  | 0.569 |
| walker |  | 5493 | 12 | Code::CodeKey { rung: Doc, file: rich/console.py, decl: 28, sub: 0, line: 276 } |  |  | 0.569 |
| walker |  | 5506 | 13 | Code::CodeKey { rung: Doc, file: rich/console.py, decl: 16, sub: 0, line: 157 } |  |  | 0.569 |
| ns | 5513 |  | 237 | Progress bars — functions, columns and classes | 3.8 |  | 0.556 |
| walker |  | 5519 | 13 | Code::CodeKey { rung: Doc, file: rich/console.py, decl: 39, sub: 0, line: 334 } |  |  | 0.556 |
| walker |  | 5533 | 14 | Code::CodeKey { rung: Doc, file: rich/console.py, decl: 29, sub: 0, line: 280 } |  |  | 0.556 |
| walker |  | 5547 | 14 | Code::CodeKey { rung: Doc, file: rich/console.py, decl: 32, sub: 0, line: 292 } |  |  | 0.556 |
| walker |  | 5562 | 15 | Code::CodeKey { rung: Doc, file: rich/console.py, decl: 14, sub: 0, line: 142 } |  |  | 0.556 |
| walker |  | 5578 | 16 | Code::CodeKey { rung: Doc, file: rich/console.py, decl: 59, sub: 0, line: 505 } |  |  | 0.556 |
| ns | 5723 |  | 210 | Live display, status, spinners, bars, screen | 3.9 |  | 0.548 |
| walker |  | 5735 | 157 | Code::CodeKey { rung: Names, file: rich/console.py, decl: 0, sub: 2, line: 0 } |  |  | 0.561 |
| walker |  | 5770 | 35 | Code::CodeKey { rung: Decl, file: rich/console.py, decl: 63, sub: 0, line: 544 } |  |  | 0.561 |
| walker |  | 5783 | 13 | Code::CodeKey { rung: Decl, file: rich/console.py, decl: 64, sub: 0, line: 547 } |  |  | 0.561 |
| walker |  | 5824 | 41 | Code::CodeKey { rung: Decl, file: rich/console.py, decl: 62, sub: 0, line: 535 } |  |  | 0.561 |
| walker |  | 5882 | 58 | Code::CodeKey { rung: Decl, file: rich/console.py, decl: 60, sub: 0, line: 525 } |  |  | 0.561 |
| ns | 5906 |  | 183 | Syntax highlighting | 3.10 |  | 0.554 |
| walker |  | 6096 | 214 | Code::CodeKey { rung: Decl, file: rich/console.py, decl: 68, sub: 0, line: 581 } |  |  | 0.558 |
| ns | 6108 |  | 202 | Markdown element hierarchy | 3.11 |  | 0.547 |
| ns | 6241 |  | 133 | Tracebacks and the logging handler | 3.12 |  | 0.541 |
| walker |  | 6282 | 186 | Code::CodeKey { rung: Decl, file: rich/console.py, decl: 68, sub: 1, line: 581 } |  |  | 0.552 |
| walker |  | 6482 | 200 | Code::CodeKey { rung: Decl, file: rich/console.py, decl: 68, sub: 2, line: 581 } |  |  | 0.568 |
| ns | 6483 |  | 242 | Pretty printing, the repr protocol, inspect, Jupyter | 3.13 |  | 0.557 |
| ns | 6679 |  | 196 | Tree, JSON, emoji, highlighters, prompts | 3.14 |  | 0.548 |
| walker |  | 6722 | 240 | Code::CodeKey { rung: Decl, file: rich/console.py, decl: 68, sub: 3, line: 581 } |  |  | 0.560 |
| walker |  | 6749 | 27 | Code::CodeKey { rung: Decl, file: rich/console.py, decl: 104, sub: 0, line: 1113 } |  |  | 0.560 |
| ns | 6765 |  | 86 | Layout engine | 3.15 |  | 0.556 |
| ns | 6939 |  | 174 | Width measurement, cell arithmetic, wrapping, ratios | 3.16 |  | 0.549 |
| walker |  | 6986 | 237 | Code::CodeKey { rung: Decl, file: rich/console.py, decl: 68, sub: 4, line: 581 } |  |  | 0.559 |
| walker |  | 7009 | 23 | Code::CodeKey { rung: Decl, file: rich/console.py, decl: 112, sub: 0, line: 1263 } |  |  | 0.559 |
| walker |  | 7032 | 23 | Code::CodeKey { rung: Decl, file: rich/console.py, decl: 114, sub: 0, line: 1294 } |  |  | 0.559 |
| ns | 7034 |  | 95 | Complete exception hierarchy | 3.17 |  | 0.555 |
| walker |  | 7056 | 24 | Code::CodeKey { rung: Decl, file: rich/console.py, decl: 113, sub: 0, line: 1277 } |  |  | 0.555 |
| ns | 7112 |  | 78 | Control codes | 3.18 |  | 0.552 |
| walker |  | 7138 | 82 | Code::CodeKey { rung: Decl, file: rich/console.py, decl: 115, sub: 0, line: 1345 } |  |  | 0.552 |
| walker |  | 7224 | 86 | Code::CodeKey { rung: Decl, file: rich/console.py, decl: 107, sub: 0, line: 1157 } |  |  | 0.552 |
| walker |  | 7234 | 10 | Code::CodeKey { rung: Doc, file: rich/console.py, decl: 67, sub: 0, line: 576 } |  |  | 0.552 |
| walker |  | 7244 | 10 | Code::CodeKey { rung: Doc, file: rich/console.py, decl: 85, sub: 0, line: 862 } |  |  | 0.552 |
| walker |  | 7255 | 11 | Code::CodeKey { rung: Doc, file: rich/console.py, decl: 62, sub: 0, line: 535 } |  |  | 0.552 |
| walker |  | 7266 | 11 | Code::CodeKey { rung: Doc, file: rich/console.py, decl: 95, sub: 0, line: 990 } |  |  | 0.552 |
| ns | 7375 |  | 263 | Protocol helpers, file plumbing and small utilities | 3.19 |  | 0.542 |
| walker |  | 7573 | 307 | Code::CodeKey { rung: Decl, file: rich/console.py, decl: 68, sub: 5, line: 581 } |  |  | 0.557 |
| walker |  | 7597 | 24 | Code::CodeKey { rung: Decl, file: rich/console.py, decl: 127, sub: 0, line: 1908 } |  |  | 0.557 |
| ns | 7612 |  | 237 | Legacy Windows console support | 3.20 |  | 0.549 |
| walker |  | 7624 | 27 | Code::CodeKey { rung: Decl, file: rich/console.py, decl: 117, sub: 0, line: 1470 } |  |  | 0.549 |
| walker |  | 7651 | 27 | Code::CodeKey { rung: Decl, file: rich/console.py, decl: 125, sub: 0, line: 1853 } |  |  | 0.549 |
| walker |  | 7708 | 57 | Code::CodeKey { rung: Decl, file: rich/console.py, decl: 124, sub: 0, line: 1819 } |  |  | 0.549 |
| ns | 7766 |  | 154 | Makefile — the complete set of dev commands | 4.1 |  | 0.556 |
| walker |  | 7778 | 70 | Code::CodeKey { rung: Decl, file: rich/console.py, decl: 119, sub: 0, line: 1589 } |  |  | 0.556 |
| walker |  | 7853 | 75 | Code::CodeKey { rung: Decl, file: rich/console.py, decl: 121, sub: 0, line: 1620 } |  |  | 0.556 |
| walker |  | 7959 | 106 | Code::CodeKey { rung: Decl, file: rich/console.py, decl: 118, sub: 0, line: 1500 } |  |  | 0.556 |
| walker |  | 8074 | 115 | Code::CodeKey { rung: Decl, file: rich/console.py, decl: 126, sub: 0, line: 1873 } |  |  | 0.556 |
| ns | 8144 |  | 378 | tests/ directory listing (complete) | 4.2 |  | 0.529 |
| walker |  | 8208 | 134 | Code::CodeKey { rung: Decl, file: rich/console.py, decl: 116, sub: 0, line: 1409 } |  |  | 0.529 |
| ns | 8310 |  | 166 | examples/ directory listing (complete) | 4.3 |  | 0.545 |
| walker |  | 8364 | 156 | Code::CodeKey { rung: Decl, file: rich/console.py, decl: 123, sub: 0, line: 1758 } |  |  | 0.545 |
| ns | 8550 |  | 240 | Type-check and test configuration, and the global test fixture | 4.4 |  | 0.537 |
| walker |  | 8588 | 224 | Code::CodeKey { rung: Decl, file: rich/console.py, decl: 122, sub: 0, line: 1652 } |  |  | 0.554 |
| ns | 8685 |  | 135 | docs/source narrative pages (complete listing) | 4.5 |  | 0.564 |
| walker |  | 8858 | 270 | Code::CodeKey { rung: Decl, file: rich/console.py, decl: 68, sub: 6, line: 581 } |  |  | 0.578 |
| ns | 8896 |  | 211 | docs/source/reference and appendix listings (complete) | 4.6 |  | 0.591 |
| walker |  | 8910 | 52 | Code::CodeKey { rung: Decl, file: rich/console.py, decl: 135, sub: 0, line: 2224 } |  |  | 0.591 |
| walker |  | 8979 | 69 | Code::CodeKey { rung: Decl, file: rich/console.py, decl: 136, sub: 0, line: 2244 } |  |  | 0.591 |
| ns | 9012 |  | 116 | CONTRIBUTING.md section headings (complete) | 4.7 |  | 0.587 |
| walker |  | 9056 | 77 | Code::CodeKey { rung: Decl, file: rich/console.py, decl: 133, sub: 0, line: 2156 } |  |  | 0.587 |
| walker |  | 9141 | 85 | Code::CodeKey { rung: Decl, file: rich/console.py, decl: 137, sub: 0, line: 2321 } |  |  | 0.587 |
| ns | 9183 |  | 171 | tox.ini — supported interpreters and the lint/docs environments | 4.8 |  | 0.581 |
| walker |  | 9245 | 104 | Code::CodeKey { rung: Decl, file: rich/console.py, decl: 138, sub: 0, line: 2352 } |  |  | 0.581 |
| walker |  | 9364 | 119 | Code::CodeKey { rung: Decl, file: rich/console.py, decl: 139, sub: 0, line: 2606 } |  |  | 0.581 |
| ns | 9473 |  | 290 | CHANGELOG — format convention and the most recent releases | 4.9 |  | 0.574 |
| walker |  | 9515 | 151 | Code::CodeKey { rung: Decl, file: rich/console.py, decl: 128, sub: 0, line: 1947 } |  |  | 0.574 |
| walker |  | 9527 | 12 | Code::CodeKey { rung: Doc, file: rich/console.py, decl: 63, sub: 0, line: 544 } |  |  | 0.574 |
| walker |  | 9539 | 12 | Code::CodeKey { rung: Doc, file: rich/console.py, decl: 72, sub: 0, line: 765 } |  |  | 0.574 |
| walker |  | 9551 | 12 | Code::CodeKey { rung: Doc, file: rich/console.py, decl: 73, sub: 0, line: 770 } |  |  | 0.574 |
| walker |  | 9563 | 12 | Code::CodeKey { rung: Doc, file: rich/console.py, decl: 74, sub: 0, line: 775 } |  |  | 0.574 |
| walker |  | 9576 | 13 | Code::CodeKey { rung: Doc, file: rich/console.py, decl: 76, sub: 0, line: 784 } |  |  | 0.574 |
| walker |  | 9589 | 13 | Code::CodeKey { rung: Doc, file: rich/console.py, decl: 77, sub: 0, line: 789 } |  |  | 0.574 |
| walker |  | 9603 | 14 | Code::CodeKey { rung: Doc, file: rich/console.py, decl: 71, sub: 0, line: 756 } |  |  | 0.574 |
| walker |  | 9617 | 14 | Code::CodeKey { rung: Doc, file: rich/console.py, decl: 84, sub: 0, line: 857 } |  |  | 0.574 |
| walker |  | 9632 | 15 | Code::CodeKey { rung: Doc, file: rich/console.py, decl: 83, sub: 0, line: 852 } |  |  | 0.574 |
| ns | 9634 |  | 161 | FAQ.md — every question heading | 4.10 |  | 0.571 |
| walker |  | 9647 | 15 | Code::CodeKey { rung: Doc, file: rich/console.py, decl: 131, sub: 0, line: 2059 } |  |  | 0.571 |
| walker |  | 9662 | 15 | Code::CodeKey { rung: Doc, file: rich/console.py, decl: 132, sub: 0, line: 2132 } |  |  | 0.571 |
| walker |  | 9678 | 16 | Code::CodeKey { rung: Doc, file: rich/console.py, decl: 79, sub: 0, line: 817 } |  |  | 0.571 |
| ns | 9781 |  | 147 | benchmarks/, tools/, questions/ and .faq/ listings | 4.11 |  | 0.579 |
| ns | 9981 |  | 200 | rich/_unicode_data listing (complete) | 4.12 |  | 0.585 |
