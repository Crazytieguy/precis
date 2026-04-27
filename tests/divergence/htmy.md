scores: Sim=0.328 Reached=5/40 Early=1 Late=3 Partial=2 Missing=33 Used=9689/10000

## Tier rollup

| tier | batches | reached | partial | missing | avg_credit |
|-----:|--------:|--------:|--------:|--------:|-----------:|
| 1 | 13 | 5 | 1 | 7 | 0.43 |
| 2 | 21 | 0 | 0 | 21 | 0.00 |
| 3 | 3 | 0 | 0 | 3 | 0.00 |
| 4 | 3 | 0 | 1 | 2 | 0.26 |

## Arrival ledger (non-aligned or partial-credit NS batches)

| id | exp_t | reached_t | delta_t | credit | status | descriptor | nearby walker batch |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.2 | 100 | — | — | 0.00 | missing | Project name + description (pyproject lede) |  |
| 1.3 | 168 | — | — | 0.60 | partial | README H1 + tagline | README headline in README.md (t=59, 2 atoms) |
| 1.4 | 238 | — | — | 0.00 | missing | Public exports — renderer + decorator |  |
| 1.5 | 253 | — | — | 0.00 | missing | Project version |  |
| 1.6 | 394 | 2428 | +2034 | 1.00 | late | README key features (top half) | README.md section #1 (t=2428, 7 atoms) |
| 1.7 | 516 | — | — | 0.00 | missing | Public exports — core utility classes |  |
| 1.8 | 671 | 2428 | +1757 | 1.00 | late | README key features (bottom half) | README.md section #1 (t=2428, 8 atoms) |
| 1.9 | 739 | — | — | 0.00 | missing | Public exports — Snippet/Tag |  |
| 1.10 | 967 | — | — | 0.00 | missing | Public exports — typing aliases |  |
| 1.11 | 1115 | — | — | 0.00 | missing | Public exports — utility helpers + HTMY alias |  |
| 1.12 | 1216 | 318 | -898 | 1.00 | early | htmy/ package directory listing |  |
| 1.13 | 1519 | 6586 | +5067 | 1.00 | late | docs/ + tests/ + examples/ directory listings |  |
| 2.1 | 1694 | — | — | 0.00 | missing | Component protocol heart (SyncComponent / AsyncComponent) |  |
| 2.2 | 1871 | — | — | 0.00 | missing | Component / ComponentType / ComponentSequence type aliases |  |
| 2.3 | 2190 | — | — | 0.00 | missing | Context / Properties / PropertyValue type aliases |  |
| 2.4 | 2429 | — | — | 0.00 | missing | Context provider protocols |  |
| 2.5 | 3034 | — | — | 0.00 | missing | RendererType + StreamingRendererType protocols |  |
| 2.6 | 3169 | — | — | 0.00 | missing | Default renderer wiring (renderer/__init__.py) |  |
| 2.7 | 3325 | — | — | 0.00 | missing | Renderer.__init__ + render signatures |  |
| 2.8 | 3558 | — | — | 0.00 | missing | BaselineRenderer (streaming) signatures |  |
| 2.9 | 4044 | — | — | 0.00 | missing | Tag / TagWithProps / wildcard_tag signatures |  |
| 2.10 | 4419 | — | — | 0.00 | missing | Fragment / WithContext signatures |  |
| 2.11 | 4603 | — | — | 0.00 | missing | ContextAware base class — public methods |  |
| 2.12 | 5046 | — | — | 0.00 | missing | @component decorator — props+context (function/method) |  |
| 2.13 | 5399 | — | — | 0.00 | missing | @component decorator — context-only (function/method) |  |
| 2.14 | 5669 | — | — | 0.00 | missing | Snippet / Slots class signatures |  |
| 2.15 | 5899 | — | — | 0.00 | missing | MD / MarkdownParser class signatures |  |
| 2.16 | 6141 | — | — | 0.00 | missing | I18n class signature |  |
| 2.17 | 6275 | — | — | 0.00 | missing | ErrorBoundary class signature |  |
| 2.18 | 6542 | — | — | 0.00 | missing | ETreeConverter class signature |  |
| 2.19 | 6703 | — | — | 0.00 | missing | Formatter class signature |  |
| 2.20 | 7214 | — | — | 0.00 | missing | SafeStr / Text / XBool / SkipProperty + xml_format_string |  |
| 2.21 | 7383 | — | — | 0.00 | missing | Utility helpers — public function signatures |  |
| 3.1 | 7838 | — | — | 0.00 | missing | html.py — top of file (DOCTYPE + first tags + Link) |  |
| 3.2 | 8131 | — | — | 0.00 | missing | html.py — block + form tag inventory (locations) |  |
| 3.3 | 8784 | — | — | 0.00 | missing | html.py — inline + table + heading tag inventory (locations) |  |
| 4.1 | 9299 | — | — | 0.00 | missing | Snippet docstring — the 4-step pipeline + warning |  |
| 4.2 | 9652 | — | — | 0.78 | partial | README — default attribute-formatting rules | README.md section #10 (t=8601, 7 atoms) |
| 4.3 | 9808 | — | — | 0.00 | missing | tests/conftest.py — renderer fixtures |  |

## Walker waste rollup (by descriptor pattern)

| n | off_tokens_total | pattern |
|--:|-----------------:|:--------|
| 15 | 2719 | README.md section #<n> |
| 14 | 2463 | docs/index.md section #<n> |
| 4 | 1162 | docs/function-components.md section #<n> |
| 2 | 715 | docs/components-guide.md section #<n> |

## Walker waste (off-NS token spend ≥ 50)

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 655 | 1.00 | 655 | 9256 | docs/components-guide.md section #2 |
| 547 | 1.00 | 547 | 4973 | README.md section #18 |
| 547 | 1.00 | 547 | 8258 | docs/index.md section #17 |
| 345 | 1.00 | 345 | 7089 | docs/function-components.md section #3 |
| 321 | 1.00 | 321 | 6514 | docs/function-components.md section #1 |
| 314 | 1.00 | 314 | 6193 | docs/function-components.md section #2 |
| 307 | 1.00 | 307 | 3273 | README.md section #2 |
| 307 | 1.00 | 307 | 5879 | docs/index.md section #2 |
| 296 | 1.00 | 296 | 7711 | README.md section #8 |
| 283 | 1.00 | 283 | 5414 | docs/index.md section #1 |
| 243 | 1.00 | 243 | 7415 | README.md section #7 |
| 215 | 1.00 | 215 | 1299 | headings outline in docs/index.md |
| 204 | 0.95 | 214 | 532 | headings outline in README.md |
| 185 | 1.00 | 185 | 3458 | README.md section #21 |
| 185 | 1.00 | 185 | 4379 | docs/index.md section #20 |
| 182 | 1.00 | 182 | 2610 | docs/function-components.md section #0 |
| 165 | 1.00 | 165 | 2775 | README.md section #15 |
| 165 | 1.00 | 165 | 4194 | docs/index.md section #15 |
| 158 | 1.00 | 158 | 5572 | README.md section #12 |
| 158 | 1.00 | 158 | 9552 | docs/index.md section #12 |
| 138 | 1.00 | 138 | 5131 | README.md section #11 |
| 138 | 1.00 | 138 | 9394 | docs/index.md section #11 |
| 134 | 1.00 | 134 | 2096 | README.md section #13 |
| 134 | 1.00 | 134 | 4007 | docs/index.md section #13 |
| 120 | 1.00 | 120 | 1962 | README.md section #5 |
| 120 | 1.00 | 120 | 3873 | docs/index.md section #5 |
| 118 | 1.00 | 118 | 1842 | README.md section #4 |
| 118 | 1.00 | 118 | 3753 | docs/index.md section #4 |
| 89 | 1.00 | 89 | 1724 | README.md section #16 |
| 89 | 1.00 | 89 | 3635 | docs/index.md section #16 |
| 88 | 1.00 | 88 | 1546 | README.md section #14 |
| 88 | 1.00 | 88 | 3546 | docs/index.md section #14 |
| 69 | 1.00 | 69 | 805 | README.md section #3 |
| 69 | 1.00 | 69 | 2966 | docs/index.md section #3 |
| 62 | 1.00 | 62 | 1410 | README.md section #19 |
| 62 | 1.00 | 62 | 2897 | docs/index.md section #18 |
| 60 | 1.00 | 60 | 2835 | docs/components-guide.md section #3 |
| 59 | 1.00 | 59 | 217 | headings outline in docs/function-components.md |
| 58 | 1.00 | 58 | 9610 | examples/markdown_essentials/post.md section #0 |
| 58 | 1.00 | 58 | 9668 | tests/data/blog-post.md section #0 |
| 53 | 1.00 | 53 | 158 | headings outline in docs/components-guide.md |
