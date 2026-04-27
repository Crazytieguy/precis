scores: Sim=0.342 Reached=11/40 Early=3 Late=7 Partial=9 Missing=20 Used=9938/10000

## Tier rollup

| tier | batches | reached | partial | missing | avg_credit |
|-----:|--------:|--------:|--------:|--------:|-----------:|
| 1 | 13 | 7 | 2 | 4 | 0.68 |
| 2 | 21 | 4 | 7 | 10 | 0.51 |
| 3 | 3 | 0 | 0 | 3 | 0.00 |
| 4 | 3 | 0 | 0 | 3 | 0.00 |

## Arrival ledger (non-aligned or partial-credit NS batches)

| id | exp_t | reached_t | delta_t | credit | status | descriptor | nearby walker batch |
|----|------:|----------:|--------:|-------:|:-------|:-----------|:--------------------|
| 1.2 | 100 | — | — | 0.00 | missing | Project name + description (pyproject lede) |  |
| 1.3 | 168 | — | — | 0.60 | partial | README H1 + tagline | README headline in README.md (t=59, 2 atoms) |
| 1.4 | 238 | 4061 | +3823 | 1.00 | late | Public exports — renderer + decorator | python imports in htmy/__init__.py (t=4061, 5 atoms) |
| 1.5 | 253 | 4061 | +3808 | 1.00 | late | Project version | python imports in htmy/__init__.py (t=4061, 1 atoms) |
| 1.6 | 394 | — | — | 0.25 | missing | README key features (top half) | headings outline in README.md (t=1089, 2 atoms) |
| 1.7 | 516 | 4061 | +3545 | 1.00 | late | Public exports — core utility classes | python imports in htmy/__init__.py (t=4061, 9 atoms) |
| 1.8 | 671 | — | — | 0.00 | missing | README key features (bottom half) |  |
| 1.9 | 739 | 4061 | +3322 | 1.00 | late | Public exports — Snippet/Tag | python imports in htmy/__init__.py (t=4061, 5 atoms) |
| 1.10 | 967 | 4061 | +3094 | 1.00 | late | Public exports — typing aliases | python imports in htmy/__init__.py (t=4061, 16 atoms) |
| 1.11 | 1115 | — | — | 0.75 | partial | Public exports — utility helpers + HTMY alias | python imports in htmy/__init__.py (t=4061, 7 atoms) |
| 1.12 | 1216 | 564 | -652 | 1.00 | early | htmy/ package directory listing |  |
| 1.13 | 1519 | — | — | 0.29 | missing | docs/ + tests/ + examples/ directory listings |  |
| 2.1 | 1694 | — | — | 0.65 | partial | Component protocol heart (SyncComponent / AsyncComponent) | python decl names surface in htmy/typing.py (t=8245, 5 atoms) |
| 2.2 | 1871 | — | — | 0.29 | missing | Component / ComponentType / ComponentSequence type aliases | python decl names surface in htmy/typing.py (t=8245, 4 atoms) |
| 2.3 | 2190 | — | — | 0.30 | missing | Context / Properties / PropertyValue type aliases | python decl names surface in htmy/typing.py (t=8245, 8 atoms) |
| 2.4 | 2429 | — | — | 0.42 | missing | Context provider protocols | python decl names surface in htmy/typing.py (t=8245, 5 atoms) |
| 2.5 | 3034 | — | — | 0.20 | missing | RendererType + StreamingRendererType protocols | python decl names surface in htmy/renderer/typing.py (t=2498, 8 atoms) |
| 2.6 | 3169 | — | — | 0.54 | partial | Default renderer wiring (renderer/__init__.py) | python imports in htmy/renderer/__init__.py (t=656, 4 atoms) |
| 2.7 | 3325 | — | — | 0.43 | missing | Renderer.__init__ + render signatures | python decl doc at htmy/renderer/default.py:228 (t=5970, 5 atoms) |
| 2.8 | 3558 | — | — | 0.54 | partial | BaselineRenderer (streaming) signatures | python method at htmy/renderer/baseline.py:30 (t=9903, 6 atoms) |
| 2.9 | 4044 | 9440 | +5396 | 0.83 | late | Tag / TagWithProps / wildcard_tag signatures | python decl doc at htmy/tag.py:84 (t=9440, 9 atoms) |
| 2.10 | 4419 | — | — | 0.21 | missing | Fragment / WithContext signatures | python decl names surface in htmy/core.py (t=1411, 4 atoms) |
| 2.11 | 4603 | — | — | 0.07 | missing | ContextAware base class — public methods | python decl names surface in htmy/core.py (t=1411, 2 atoms) |
| 2.12 | 5046 | — | — | 0.72 | partial | @component decorator — props+context (function/method) | python method sigs in htmy/function_component.py (t=8690, 30 atoms) |
| 2.13 | 5399 | 9119 | +3720 | 0.86 | late | @component decorator — context-only (function/method) | python method sigs in htmy/function_component.py (t=8690, 18 atoms) |
| 2.14 | 5669 | — | — | 0.70 | partial | Snippet / Slots class signatures | python method sigs in htmy/snippet.py (t=5171, 9 atoms) |
| 2.15 | 5899 | — | — | 0.49 | missing | MD / MarkdownParser class signatures | python method sigs in htmy/md/core.py (t=7702, 10 atoms) |
| 2.16 | 6141 | — | — | 0.70 | partial | I18n class signature | python decl names surface in htmy/i18n.py (t=1908, 9 atoms) |
| 2.17 | 6275 | 3150 | -3125 | 0.92 | early | ErrorBoundary class signature | python method at htmy/error_boundary.py:25 (t=3150, 6 atoms) |
| 2.18 | 6542 | — | — | 0.29 | missing | ETreeConverter class signature | python class body at htmy/etree.py:23 (t=3366, 6 atoms) |
| 2.19 | 6703 | — | — | 0.10 | missing | Formatter class signature | python decl names surface in htmy/core.py (t=1411, 2 atoms) |
| 2.20 | 7214 | — | — | 0.56 | partial | SafeStr / Text / XBool / SkipProperty + xml_format_string | python decl names surface in htmy/core.py (t=1411, 10 atoms) |
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
| 6 | 546 | README.md section #<n> |

## Walker waste (off-NS token spend ≥ 50)

| off_tokens | off_ratio | cost | first_t | batch |
|-----------:|----------:|-----:|--------:|:------|
| 215 | 1.00 | 215 | 4551 | headings outline in docs/index.md |
| 204 | 0.95 | 214 | 1089 | headings outline in README.md |
| 187 | 0.90 | 208 | 4958 | python decl names surface in htmy/function_component.py |
| 120 | 1.00 | 120 | 9661 | README.md section #5 |
| 118 | 1.00 | 118 | 7488 | README.md section #4 |
| 106 | 1.00 | 106 | 7594 | python decl doc at htmy/i18n.py:147 |
| 100 | 1.00 | 100 | 7939 | python imports in htmy/etree.py |
| 96 | 1.00 | 96 | 7312 | python imports in htmy/function_component.py |
| 94 | 1.00 | 94 | 6575 | python decl doc at htmy/i18n.py:115 |
| 94 | 1.00 | 94 | 7127 | python imports in htmy/error_boundary.py |
| 89 | 1.00 | 89 | 7216 | README.md section #16 |
| 88 | 1.00 | 88 | 6454 | README.md section #14 |
| 88 | 1.00 | 88 | 462 | python imports in htmy/md/__init__.py |
| 86 | 1.00 | 86 | 3366 | python class body at htmy/etree.py:23 |
| 82 | 1.00 | 82 | 6336 | python imports in htmy/utils.py |
| 79 | 1.00 | 79 | 5839 | python decl doc at htmy/utils.py:12 |
| 79 | 1.00 | 79 | 6254 | python imports in htmy/tag.py |
| 79 | 0.58 | 136 | 5171 | python method sigs in htmy/snippet.py |
| 76 | 1.00 | 76 | 6046 | python imports in htmy/i18n.py |
| 76 | 0.55 | 137 | 5527 | python method sigs in htmy/etree.py |
| 74 | 0.42 | 175 | 6813 | python method sigs in htmy/tag.py |
| 70 | 1.00 | 70 | 2231 | python decl names surface in htmy/md/typing.py |
| 69 | 1.00 | 69 | 2391 | README.md section #3 |
| 68 | 1.00 | 68 | 4750 | python imports in htmy/html.py |
| 68 | 1.00 | 68 | 9729 | python method doc at htmy/typing.py:108 |
| 68 | 0.34 | 200 | 8690 | python method sigs in htmy/function_component.py |
| 66 | 1.00 | 66 | 2118 | python class body at htmy/i18n.py:24 |
| 64 | 1.00 | 64 | 9504 | python method doc at htmy/i18n.py:34 |
| 62 | 1.00 | 62 | 4662 | README.md section #19 |
| 59 | 1.00 | 59 | 217 | headings outline in docs/function-components.md |
| 59 | 1.00 | 59 | 7819 | python decl body at htmy/i18n.py:147 |
| 58 | 1.00 | 58 | 7370 | python decl body at htmy/utils.py:73 |
| 58 | 0.48 | 121 | 9850 | python method sigs in htmy/renderer/baseline.py |
| 55 | 1.00 | 55 | 4232 | python imports in htmy/io.py |
| 53 | 1.00 | 53 | 158 | headings outline in docs/components-guide.md |
| 53 | 1.00 | 53 | 4285 | python class body at htmy/md/core.py:20 |
| 53 | 1.00 | 53 | 6924 | python method at htmy/tag.py:27 |
| 52 | 0.85 | 61 | 2006 | python decl names surface in htmy/renderer/default.py |
| 51 | 1.00 | 51 | 9006 | python method at htmy/function_component.py:147 |
