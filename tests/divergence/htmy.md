scores: Sim=0.334 Reached=6/40 Early=3 Late=2 Partial=8 Missing=26 Used=9966/10000

## Tier rollup

| tier | batches | reached | partial | missing | avg_credit |
|-----:|--------:|--------:|--------:|--------:|-----------:|
| 1 | 13 | 2 | 1 | 10 | 0.25 |
| 2 | 21 | 4 | 7 | 10 | 0.52 |
| 3 | 3 | 0 | 0 | 3 | 0.00 |
| 4 | 3 | 0 | 0 | 3 | 0.00 |

## Arrival ledger (non-aligned or partial-credit NS batches)

| id | exp_t | reached_t | delta_t | credit | status | descriptor | nearby walker batch |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.2 | 100 | — | — | 0.00 | missing | Project name + description (pyproject lede) |  |
| 1.3 | 168 | — | — | 0.60 | partial | README H1 + tagline | README headline in README.md (t=59, 2 atoms) |
| 1.4 | 238 | — | — | 0.00 | missing | Public exports — renderer + decorator |  |
| 1.5 | 253 | — | — | 0.00 | missing | Project version |  |
| 1.6 | 394 | — | — | 0.25 | missing | README key features (top half) | headings outline in README.md (t=941, 2 atoms) |
| 1.7 | 516 | — | — | 0.00 | missing | Public exports — core utility classes |  |
| 1.8 | 671 | — | — | 0.00 | missing | README key features (bottom half) |  |
| 1.9 | 739 | — | — | 0.00 | missing | Public exports — Snippet/Tag |  |
| 1.10 | 967 | — | — | 0.00 | missing | Public exports — typing aliases |  |
| 1.11 | 1115 | — | — | 0.17 | missing | Public exports — utility helpers + HTMY alias | python decl names surface in htmy/__init__.py (t=300, 2 atoms) |
| 1.12 | 1216 | 476 | -740 | 1.00 | early | htmy/ package directory listing |  |
| 1.13 | 1519 | — | — | 0.29 | missing | docs/ + tests/ + examples/ directory listings |  |
| 2.1 | 1694 | — | — | 0.65 | partial | Component protocol heart (SyncComponent / AsyncComponent) | python decl names surface in htmy/typing.py (t=7634, 5 atoms) |
| 2.2 | 1871 | — | — | 0.29 | missing | Component / ComponentType / ComponentSequence type aliases | python decl names surface in htmy/typing.py (t=7634, 4 atoms) |
| 2.3 | 2190 | — | — | 0.30 | missing | Context / Properties / PropertyValue type aliases | python decl names surface in htmy/typing.py (t=7634, 8 atoms) |
| 2.4 | 2429 | — | — | 0.42 | missing | Context provider protocols | python decl names surface in htmy/typing.py (t=7634, 5 atoms) |
| 2.5 | 3034 | — | — | 0.35 | missing | RendererType + StreamingRendererType protocols | python decl names surface in htmy/renderer/typing.py (t=2498, 8 atoms) |
| 2.6 | 3169 | — | — | 0.54 | partial | Default renderer wiring (renderer/__init__.py) | python imports in htmy/renderer/__init__.py (t=1063, 4 atoms) |
| 2.7 | 3325 | — | — | 0.43 | missing | Renderer.__init__ + render signatures | python decl doc at htmy/renderer/default.py:228 (t=5359, 5 atoms) |
| 2.8 | 3558 | — | — | 0.54 | partial | BaselineRenderer (streaming) signatures | python method at htmy/renderer/baseline.py:30 (t=9292, 6 atoms) |
| 2.9 | 4044 | 8829 | +4785 | 0.83 | late | Tag / TagWithProps / wildcard_tag signatures | python decl doc at htmy/tag.py:84 (t=8829, 9 atoms) |
| 2.10 | 4419 | — | — | 0.21 | missing | Fragment / WithContext signatures | python decl names surface in htmy/core.py (t=1323, 4 atoms) |
| 2.11 | 4603 | — | — | 0.07 | missing | ContextAware base class — public methods | python decl names surface in htmy/core.py (t=1323, 2 atoms) |
| 2.12 | 5046 | — | — | 0.72 | partial | @component decorator — props+context (function/method) | python method sigs in htmy/function_component.py (t=8079, 30 atoms) |
| 2.13 | 5399 | 8508 | +3109 | 0.86 | late | @component decorator — context-only (function/method) | python method sigs in htmy/function_component.py (t=8079, 18 atoms) |
| 2.14 | 5669 | — | — | 0.70 | partial | Snippet / Slots class signatures | python method sigs in htmy/snippet.py (t=4560, 9 atoms) |
| 2.15 | 5899 | — | — | 0.49 | missing | MD / MarkdownParser class signatures | python method sigs in htmy/md/core.py (t=7091, 10 atoms) |
| 2.16 | 6141 | — | — | 0.70 | partial | I18n class signature | python decl names surface in htmy/i18n.py (t=1820, 9 atoms) |
| 2.17 | 6275 | 3150 | -3125 | 0.92 | early | ErrorBoundary class signature | python method at htmy/error_boundary.py:25 (t=3150, 6 atoms) |
| 2.18 | 6542 | — | — | 0.29 | missing | ETreeConverter class signature | python class body at htmy/etree.py:23 (t=3366, 6 atoms) |
| 2.19 | 6703 | — | — | 0.10 | missing | Formatter class signature | python decl names surface in htmy/core.py (t=1323, 2 atoms) |
| 2.20 | 7214 | — | — | 0.56 | partial | SafeStr / Text / XBool / SkipProperty + xml_format_string | python decl names surface in htmy/core.py (t=1323, 10 atoms) |
| 2.21 | 7383 | 2829 | -4554 | 1.00 | early | Utility helpers — public function signatures | python decl names surface in htmy/utils.py (t=2689, 13 atoms) |
| 3.1 | 7838 | — | — | 0.00 | missing | html.py — top of file (DOCTYPE + first tags + Link) |  |
| 3.2 | 8131 | — | — | 0.00 | missing | html.py — block + form tag inventory (locations) |  |
| 3.3 | 8784 | — | — | 0.00 | missing | html.py — inline + table + heading tag inventory (locations) |  |
| 4.1 | 9299 | — | — | 0.00 | missing | Snippet docstring — the 4-step pipeline + warning | python decl names surface in htmy/snippet.py (t=340, 1 atoms) |
| 4.2 | 9652 | — | — | 0.00 | missing | README — default attribute-formatting rules |  |
| 4.3 | 9808 | — | — | 0.00 | missing | tests/conftest.py — renderer fixtures |  |

## Walker waste rollup (by descriptor pattern)

| n | off_tokens_total | pattern |
|--:|-----------------:|:--------|
| 7 | 680 | README.md section #<n> |

## Walker waste (off-NS token spend ≥ 50)

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 215 | 1.00 | 215 | 3940 | headings outline in docs/index.md |
| 204 | 0.95 | 214 | 941 | headings outline in README.md |
| 187 | 0.90 | 208 | 4347 | python decl names surface in htmy/function_component.py |
| 134 | 1.00 | 134 | 9966 | README.md section #13 |
| 133 | 1.00 | 133 | 9832 | python imports in htmy/core.py |
| 120 | 1.00 | 120 | 9050 | README.md section #5 |
| 118 | 1.00 | 118 | 6877 | README.md section #4 |
| 106 | 1.00 | 106 | 6983 | python decl doc at htmy/i18n.py:147 |
| 100 | 1.00 | 100 | 7328 | python imports in htmy/etree.py |
| 96 | 1.00 | 96 | 6701 | python imports in htmy/function_component.py |
| 94 | 1.00 | 94 | 5964 | python decl doc at htmy/i18n.py:115 |
| 94 | 1.00 | 94 | 6516 | python imports in htmy/error_boundary.py |
| 89 | 1.00 | 89 | 6605 | README.md section #16 |
| 88 | 1.00 | 88 | 5843 | README.md section #14 |
| 88 | 1.00 | 88 | 2267 | python imports in htmy/md/__init__.py |
| 86 | 1.00 | 86 | 3366 | python class body at htmy/etree.py:23 |
| 82 | 1.00 | 82 | 5725 | python imports in htmy/utils.py |
| 79 | 1.00 | 79 | 5228 | python decl doc at htmy/utils.py:12 |
| 79 | 1.00 | 79 | 5643 | python imports in htmy/tag.py |
| 79 | 0.58 | 136 | 4560 | python method sigs in htmy/snippet.py |
| 76 | 1.00 | 76 | 5435 | python imports in htmy/i18n.py |
| 76 | 0.55 | 137 | 4916 | python method sigs in htmy/etree.py |
| 74 | 0.42 | 175 | 6202 | python method sigs in htmy/tag.py |
| 70 | 1.00 | 70 | 2143 | python decl names surface in htmy/md/typing.py |
| 69 | 1.00 | 69 | 2391 | README.md section #3 |
| 68 | 1.00 | 68 | 4139 | python imports in htmy/html.py |
| 68 | 1.00 | 68 | 9118 | python method doc at htmy/typing.py:108 |
| 68 | 0.34 | 200 | 8079 | python method sigs in htmy/function_component.py |
| 66 | 1.00 | 66 | 2030 | python class body at htmy/i18n.py:24 |
| 66 | 1.00 | 66 | 9393 | python decl body at htmy/utils.py:55 |
| 64 | 1.00 | 64 | 8893 | python method doc at htmy/i18n.py:34 |
| 62 | 1.00 | 62 | 4051 | README.md section #19 |
| 59 | 1.00 | 59 | 217 | headings outline in docs/function-components.md |
| 59 | 1.00 | 59 | 7208 | python decl body at htmy/i18n.py:147 |
| 58 | 1.00 | 58 | 6759 | python decl body at htmy/utils.py:73 |
| 58 | 0.48 | 121 | 9239 | python method sigs in htmy/renderer/baseline.py |
| 55 | 1.00 | 55 | 3621 | python imports in htmy/io.py |
| 53 | 1.00 | 53 | 158 | headings outline in docs/components-guide.md |
| 53 | 1.00 | 53 | 3674 | python class body at htmy/md/core.py:20 |
| 53 | 1.00 | 53 | 6313 | python method at htmy/tag.py:27 |
| 52 | 0.85 | 61 | 1918 | python decl names surface in htmy/renderer/default.py |
| 51 | 1.00 | 51 | 8395 | python method at htmy/function_component.py:147 |
